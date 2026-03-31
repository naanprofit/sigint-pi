use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::config::GpsConfig;
use crate::ScanEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub speed: Option<f64>,
    pub heading: Option<f64>,
    pub accuracy: Option<f64>,
    pub fix_type: GpsFixType,
    pub satellites: u8,        // Satellites being used for fix
    pub satellites_seen: u8,   // Total satellites visible
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GpsFixType {
    NoFix,
    Fix2D,
    Fix3D,
    DGPS,
}

pub struct GpsClient {
    config: GpsConfig,
}

impl GpsClient {
    pub fn new(config: GpsConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, tx: broadcast::Sender<ScanEvent>) -> Result<()> {
        loop {
            match self.connect_and_read(&tx).await {
                Ok(_) => info!("GPS connection closed normally"),
                Err(e) => {
                    warn!("GPS connection error: {}, reconnecting in 5s", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn connect_and_read(&self, tx: &broadcast::Sender<ScanEvent>) -> Result<()> {
        let addr = format!("{}:{}", self.config.gpsd_host, self.config.gpsd_port);
        
        // Connect to gpsd in a blocking thread
        let (reader_tx, mut reader_rx) = tokio::sync::mpsc::channel::<GpsPosition>(100);
        
        let addr_clone = addr.clone();
        let update_interval = self.config.update_interval_ms;
        
        tokio::task::spawn_blocking(move || {
            if let Err(e) = gpsd_reader(&addr_clone, reader_tx, update_interval) {
                error!("GPS reader error: {}", e);
            }
        });

        info!("Connected to gpsd at {}", addr);

        // Forward GPS updates
        while let Some(position) = reader_rx.recv().await {
            let _ = tx.send(ScanEvent::GpsUpdate(position));
        }

        Ok(())
    }
}

fn gpsd_reader(
    addr: &str,
    tx: tokio::sync::mpsc::Sender<GpsPosition>,
    update_interval_ms: u64,
) -> Result<()> {
    let stream = TcpStream::connect(addr)
        .context("Failed to connect to gpsd")?;
    
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;

    // Send WATCH command to enable JSON output
    use std::io::Write;
    let mut stream = stream;
    stream.write_all(b"?WATCH={\"enable\":true,\"json\":true}\n")?;

    let reader = BufReader::new(stream);
    let mut last_update = std::time::Instant::now();
    let min_interval = Duration::from_millis(update_interval_ms);

    for line in reader.lines() {
        let line = line?;
        
        // Throttle updates
        if last_update.elapsed() < min_interval {
            continue;
        }

        if let Some(position) = parse_gpsd_json(&line) {
            if tx.blocking_send(position).is_err() {
                break; // Channel closed
            }
            last_update = std::time::Instant::now();
        }
    }

    Ok(())
}

fn parse_gpsd_json(line: &str) -> Option<GpsPosition> {
    // Parse gpsd TPV (Time-Position-Velocity) message
    let json: serde_json::Value = serde_json::from_str(line).ok()?;
    
    let class = json.get("class")?.as_str()?;
    
    // Handle TPV (position) messages
    if class == "TPV" {
        let mode = json.get("mode").and_then(|v| v.as_i64()).unwrap_or(0) as u8;
        
        // Get coordinates if available (mode >= 2 means we have a fix)
        let (lat, lon) = if mode >= 2 {
            let lat = json.get("lat").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let lon = json.get("lon").and_then(|v| v.as_f64()).unwrap_or(0.0);
            (lat, lon)
        } else {
            (0.0, 0.0) // No fix yet, but still report status
        };

        return Some(GpsPosition {
            latitude: lat,
            longitude: lon,
            altitude: json.get("alt").and_then(|v| v.as_f64()),
            speed: json.get("speed").and_then(|v| v.as_f64()),
            heading: json.get("track").and_then(|v| v.as_f64()),
            accuracy: json.get("epx").and_then(|v| v.as_f64()),
            fix_type: match mode {
                0 => GpsFixType::NoFix,
                1 => GpsFixType::NoFix, // Searching
                2 => GpsFixType::Fix2D,
                3 => GpsFixType::Fix3D,
                _ => GpsFixType::NoFix,
            },
            satellites: 0, // Will be updated from SKY message
            satellites_seen: 0,
            timestamp: Utc::now(),
        });
    }
    
    // Handle SKY (satellite) messages to get satellite count
    if class == "SKY" {
        let sats_seen = json.get("nSat")
            .and_then(|v| v.as_u64())
            .unwrap_or_else(|| {
                // Fallback: count satellites array
                json.get("satellites")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.len() as u64)
                    .unwrap_or(0)
            }) as u8;
        
        let sats_used = json.get("uSat")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u8;
        
        // Return a no-fix position with satellite info
        return Some(GpsPosition {
            latitude: 0.0,
            longitude: 0.0,
            altitude: None,
            speed: None,
            heading: None,
            accuracy: None,
            fix_type: GpsFixType::NoFix,
            satellites: sats_used,
            satellites_seen: sats_seen,
            timestamp: Utc::now(),
        });
    }
    
    None
}

impl GpsPosition {
    /// Calculate distance in meters between two positions using Haversine formula
    pub fn distance_to(&self, other: &GpsPosition) -> f64 {
        const EARTH_RADIUS: f64 = 6371000.0; // meters

        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS * c
    }

    /// Check if position is within a geofence radius
    pub fn within_radius(&self, center: &GpsPosition, radius_meters: f64) -> bool {
        self.distance_to(center) <= radius_meters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let pos1 = GpsPosition {
            latitude: 40.7128,
            longitude: -74.0060,
            altitude: None,
            speed: None,
            heading: None,
            accuracy: None,
            fix_type: GpsFixType::Fix3D,
            satellites: 8,
            satellites_seen: 12,
            timestamp: Utc::now(),
        };

        let pos2 = GpsPosition {
            latitude: 40.7129,
            longitude: -74.0061,
            altitude: None,
            speed: None,
            heading: None,
            accuracy: None,
            fix_type: GpsFixType::Fix3D,
            satellites: 8,
            satellites_seen: 12,
            timestamp: Utc::now(),
        };

        let distance = pos1.distance_to(&pos2);
        assert!(distance > 0.0 && distance < 100.0); // Should be ~14 meters
    }
}
