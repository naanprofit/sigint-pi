use std::collections::HashMap;
use std::sync::Arc;
use chrono::{DateTime, Duration, Utc, Timelike};
use tokio::sync::broadcast;
use tracing::{debug, info, warn};

use crate::config::Config;
use crate::storage::Database;
use crate::gps::GpsPosition;
use crate::wifi::WifiDevice;
use crate::bluetooth::BleDevice;
use crate::ScanEvent;

use super::anomaly::AnomalyDetector;

pub struct DeviceLearner {
    db: Arc<Database>,
    config: Arc<Config>,
    anomaly_detector: AnomalyDetector,
    device_stats: HashMap<String, DeviceStats>,
    current_position: Option<GpsPosition>,
    current_location_id: i64,
    training_start: Option<DateTime<Utc>>,
    is_training: bool,
}

#[derive(Debug, Clone)]
pub struct DeviceStats {
    pub mac_address: String,
    pub rssi_samples: Vec<i32>,
    pub hours_seen: Vec<u8>,
    pub visit_count: u32,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub total_time_seen: Duration,
    pub probed_ssids: Vec<String>,
}

impl DeviceLearner {
    pub fn new(db: Arc<Database>, config: Arc<Config>) -> Self {
        Self {
            db,
            config,
            anomaly_detector: AnomalyDetector::new(),
            device_stats: HashMap::new(),
            current_position: None,
            current_location_id: 1, // Default location
            training_start: None,
            is_training: false,
        }
    }

    pub async fn run(&self, rx: &mut broadcast::Receiver<ScanEvent>) {
        let mut learner = self.clone_state();
        
        loop {
            match rx.recv().await {
                Ok(event) => {
                    learner.process_event(event).await;
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!("Learner lagged {} events", n);
                }
                Err(broadcast::error::RecvError::Closed) => {
                    info!("Event channel closed, stopping learner");
                    break;
                }
            }
        }
    }

    fn clone_state(&self) -> DeviceLearnerState {
        DeviceLearnerState {
            db: self.db.clone(),
            config: self.config.clone(),
            anomaly_detector: AnomalyDetector::new(),
            device_stats: HashMap::new(),
            current_position: None,
            current_location_id: 1,
            training_start: None,
            is_training: self.config.learning.enabled,
        }
    }
}

struct DeviceLearnerState {
    db: Arc<Database>,
    config: Arc<Config>,
    anomaly_detector: AnomalyDetector,
    device_stats: HashMap<String, DeviceStats>,
    current_position: Option<GpsPosition>,
    current_location_id: i64,
    training_start: Option<DateTime<Utc>>,
    is_training: bool,
}

impl DeviceLearnerState {
    async fn process_event(&mut self, event: ScanEvent) {
        match event {
            ScanEvent::WifiDevice(device) => {
                self.process_wifi_device(&device).await;
            }
            ScanEvent::BleDevice(device) => {
                self.process_ble_device(&device).await;
            }
            ScanEvent::GpsUpdate(position) => {
                self.update_position(position).await;
            }
            ScanEvent::Attack(_) => {
                // Attacks are handled by alert manager
            }
        }
    }

    async fn process_wifi_device(&mut self, device: &WifiDevice) {
        let mac = &device.mac_address;
        let now = Utc::now();
        let hour = now.hour() as u8;

        // Update or create device stats
        let stats = self.device_stats.entry(mac.clone()).or_insert_with(|| {
            DeviceStats {
                mac_address: mac.clone(),
                rssi_samples: Vec::new(),
                hours_seen: Vec::new(),
                visit_count: 1,
                first_seen: now,
                last_seen: now,
                total_time_seen: Duration::zero(),
                probed_ssids: Vec::new(),
            }
        });

        stats.rssi_samples.push(device.rssi);
        if !stats.hours_seen.contains(&hour) {
            stats.hours_seen.push(hour);
        }
        
        // Update visit count if this is a new visit (gap > 30 min)
        if now.signed_duration_since(stats.last_seen) > Duration::minutes(30) {
            stats.visit_count += 1;
        }
        
        stats.last_seen = now;

        // Track probe requests
        if let Some(ref ssid) = device.ssid {
            if !stats.probed_ssids.contains(ssid) {
                stats.probed_ssids.push(ssid.clone());
            }
        }

        // Check if device should be marked as baseline
        if self.is_training {
            self.check_training_complete();
            
            // During training, collect data but don't flag anomalies
            debug!("Training: observed device {} (RSSI: {})", mac, device.rssi);
        } else {
            // After training, compute anomaly score
            let score = self.anomaly_detector.score_device(mac, device.rssi, stats);
            
            if score.is_anomalous() {
                info!(
                    "Anomaly detected: {} (score: {:.2}, reason: {})",
                    mac, score.total_score, score.reason
                );
                // Alert manager will handle this via the event broadcast
            }
        }

        // Store sighting in database
        if let Ok(device_id) = self.db.upsert_wifi_device(device, self.current_location_id).await {
            let _ = self.db.record_sighting(
                device_id,
                "wifi",
                device.rssi,
                Some(device.channel),
                device.ssid.as_deref(),
                self.current_position.as_ref(),
            ).await;
        }
    }

    async fn process_ble_device(&mut self, device: &BleDevice) {
        let mac = &device.mac_address;
        let now = Utc::now();
        let hour = now.hour() as u8;

        let stats = self.device_stats.entry(mac.clone()).or_insert_with(|| {
            DeviceStats {
                mac_address: mac.clone(),
                rssi_samples: Vec::new(),
                hours_seen: Vec::new(),
                visit_count: 1,
                first_seen: now,
                last_seen: now,
                total_time_seen: Duration::zero(),
                probed_ssids: Vec::new(),
            }
        });

        stats.rssi_samples.push(device.rssi);
        if !stats.hours_seen.contains(&hour) {
            stats.hours_seen.push(hour);
        }
        
        if now.signed_duration_since(stats.last_seen) > Duration::minutes(30) {
            stats.visit_count += 1;
        }
        
        stats.last_seen = now;

        // Special handling for trackers
        if device.is_tracker() {
            info!("Tracker device detected: {} (type: {:?})", mac, device.device_type);
        }

        // Store in database
        if let Ok(device_id) = self.db.upsert_ble_device(device, self.current_location_id).await {
            let _ = self.db.record_sighting(
                device_id,
                "ble",
                device.rssi,
                None,
                None,
                self.current_position.as_ref(),
            ).await;
        }
    }

    async fn update_position(&mut self, position: GpsPosition) {
        // Check if we've moved to a new location
        if let Some(ref current) = self.current_position {
            let distance = current.distance_to(&position);
            
            if distance > self.config.learning.geofence_radius_meters {
                info!("Location change detected: moved {:.0}m", distance);
                
                // Get or create new location
                if let Ok(location_id) = self.db.get_or_create_location(
                    &format!("loc_{:.4}_{:.4}", position.latitude, position.longitude),
                    Some(position.latitude),
                    Some(position.longitude),
                ).await {
                    self.current_location_id = location_id;
                    
                    // Reset training for new location
                    if self.config.learning.enabled {
                        self.start_training();
                    }
                }
            }
        }

        self.current_position = Some(position);
    }

    fn start_training(&mut self) {
        self.training_start = Some(Utc::now());
        self.is_training = true;
        self.device_stats.clear();
        info!("Started baseline training for location {}", self.current_location_id);
    }

    fn check_training_complete(&mut self) {
        if let Some(start) = self.training_start {
            let elapsed = Utc::now().signed_duration_since(start);
            let required = Duration::hours(self.config.learning.training_hours as i64);
            
            if elapsed >= required {
                self.complete_training();
            }
        }
    }

    fn complete_training(&mut self) {
        self.is_training = false;
        self.training_start = None;
        
        // Build baseline profiles
        for (mac, stats) in &self.device_stats {
            self.anomaly_detector.add_baseline_device(mac.clone(), stats.clone());
        }
        
        info!(
            "Training complete: {} devices in baseline for location {}",
            self.device_stats.len(),
            self.current_location_id
        );

        // Mark devices as baseline in database
        let db = self.db.clone();
        let location_id = self.current_location_id;
        let macs: Vec<String> = self.device_stats.keys().cloned().collect();
        
        tokio::spawn(async move {
            for mac in macs {
                let _ = db.mark_as_baseline(&mac, location_id).await;
            }
        });
    }
}

impl DeviceStats {
    pub fn avg_rssi(&self) -> f64 {
        if self.rssi_samples.is_empty() {
            return -100.0;
        }
        self.rssi_samples.iter().map(|&r| r as f64).sum::<f64>() / self.rssi_samples.len() as f64
    }

    pub fn rssi_stddev(&self) -> f64 {
        if self.rssi_samples.len() < 2 {
            return 0.0;
        }
        let avg = self.avg_rssi();
        let variance = self.rssi_samples.iter()
            .map(|&r| (r as f64 - avg).powi(2))
            .sum::<f64>() / self.rssi_samples.len() as f64;
        variance.sqrt()
    }
}
