//! Drone/UAV Detection
//! 
//! Detects drones by monitoring:
//! - 2.4 GHz control signals (WiFi-based drones, DJI, etc.)
//! - 5.8 GHz FPV video transmitters
//! - 915 MHz (some long-range systems)
//! - Known drone protocol signatures

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Detected drone signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneSignal {
    pub id: String,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub power_db: f64,
    pub signal_type: DroneSignalType,
    pub drone_type: Option<DroneType>,
    pub protocol: Option<DroneProtocol>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub duration_secs: u64,
    pub direction: Option<f64>,  // Bearing if directional antenna available
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneSignalType {
    Control,      // Remote control link
    Telemetry,    // Downlink telemetry
    Video,        // FPV video feed
    Gps,          // GPS spoofing/jamming
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneType {
    DjiMavic,
    DjiPhantom,
    DjiMini,
    DjiInspire,
    DjiMatrice,
    Parrot,
    Autel,
    Skydio,
    FpvRacing,
    FixedWing,
    Custom,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneProtocol {
    DjiOcusync,     // DJI OcuSync 1/2/3
    DjiLightbridge, // DJI Lightbridge
    Wifi,           // Standard WiFi
    Analog5_8,      // Analog 5.8GHz video
    Crossfire,      // TBS Crossfire (868/915 MHz)
    Expresslrs,     // ExpressLRS
    FrSky,          // FrSky protocols
    Spektrum,       // Spektrum DSMX
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    None,       // Normal recreational drone
    Low,        // Unknown drone nearby
    Medium,     // Drone loitering, possible surveillance
    High,       // Drone in restricted area or following
    Critical,   // GPS interference detected
}

/// Drone frequency signatures
#[derive(Debug, Clone)]
pub struct DroneSignature {
    pub center_freq_hz: u64,
    pub bandwidth_hz: u64,
    pub protocol: DroneProtocol,
    pub signal_type: DroneSignalType,
    pub description: String,
}

impl DroneSignature {
    pub fn known_signatures() -> Vec<Self> {
        vec![
            // DJI OcuSync 2.4 GHz
            Self {
                center_freq_hz: 2_400_000_000,
                bandwidth_hz: 40_000_000,
                protocol: DroneProtocol::DjiOcusync,
                signal_type: DroneSignalType::Control,
                description: "DJI OcuSync 2.4GHz control".to_string(),
            },
            // DJI OcuSync 5.8 GHz
            Self {
                center_freq_hz: 5_800_000_000,
                bandwidth_hz: 40_000_000,
                protocol: DroneProtocol::DjiOcusync,
                signal_type: DroneSignalType::Video,
                description: "DJI OcuSync 5.8GHz video".to_string(),
            },
            // Analog FPV 5.8 GHz
            Self {
                center_freq_hz: 5_800_000_000,
                bandwidth_hz: 20_000_000,
                protocol: DroneProtocol::Analog5_8,
                signal_type: DroneSignalType::Video,
                description: "Analog FPV video".to_string(),
            },
            // TBS Crossfire 868 MHz (EU)
            Self {
                center_freq_hz: 868_000_000,
                bandwidth_hz: 500_000,
                protocol: DroneProtocol::Crossfire,
                signal_type: DroneSignalType::Control,
                description: "TBS Crossfire 868MHz".to_string(),
            },
            // TBS Crossfire 915 MHz (US)
            Self {
                center_freq_hz: 915_000_000,
                bandwidth_hz: 500_000,
                protocol: DroneProtocol::Crossfire,
                signal_type: DroneSignalType::Control,
                description: "TBS Crossfire 915MHz".to_string(),
            },
            // ExpressLRS 2.4 GHz
            Self {
                center_freq_hz: 2_400_000_000,
                bandwidth_hz: 1_000_000,
                protocol: DroneProtocol::Expresslrs,
                signal_type: DroneSignalType::Control,
                description: "ExpressLRS 2.4GHz".to_string(),
            },
            // WiFi-based drones
            Self {
                center_freq_hz: 2_437_000_000, // Channel 6
                bandwidth_hz: 20_000_000,
                protocol: DroneProtocol::Wifi,
                signal_type: DroneSignalType::Control,
                description: "WiFi drone control".to_string(),
            },
        ]
    }
}

/// Drone detector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneDetectorConfig {
    pub enabled: bool,
    pub monitor_2_4ghz: bool,
    pub monitor_5_8ghz: bool,
    pub monitor_sub_ghz: bool,
    pub scan_interval_secs: u64,
    pub min_signal_db: f64,
    pub device_index: u32,
}

impl Default for DroneDetectorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitor_2_4ghz: true,
            monitor_5_8ghz: true,
            monitor_sub_ghz: true,
            scan_interval_secs: 10,
            min_signal_db: -70.0,
            device_index: 0,
        }
    }
}

/// Drone signal detector
pub struct DroneDetector {
    config: DroneDetectorConfig,
    signatures: Vec<DroneSignature>,
    detected: HashMap<String, DroneSignal>,
}

impl DroneDetector {
    pub fn new(config: DroneDetectorConfig) -> Self {
        Self {
            config,
            signatures: DroneSignature::known_signatures(),
            detected: HashMap::new(),
        }
    }
    
    /// Scan 2.4 GHz band for drone signals
    pub async fn scan_2_4ghz(&mut self) -> anyhow::Result<Vec<DroneSignal>> {
        if !self.config.monitor_2_4ghz {
            return Ok(vec![]);
        }
        
        // Use hackrf_sweep for wideband scanning
        let output = Command::new("hackrf_sweep")
            .args(&[
                "-f", "2400:2500",
                "-w", "500000", // 500 kHz bins
                "-1", // Single sweep
            ])
            .output()
            .await;
        
        let mut signals = Vec::new();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            signals.extend(self.parse_sweep_for_drones(&stdout, 2_400_000_000, 2_500_000_000));
        }
        
        Ok(signals)
    }
    
    /// Scan 5.8 GHz band for FPV video
    pub async fn scan_5_8ghz(&mut self) -> anyhow::Result<Vec<DroneSignal>> {
        if !self.config.monitor_5_8ghz {
            return Ok(vec![]);
        }
        
        let output = Command::new("hackrf_sweep")
            .args(&[
                "-f", "5650:5950",
                "-w", "1000000", // 1 MHz bins
                "-1",
            ])
            .output()
            .await;
        
        let mut signals = Vec::new();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            signals.extend(self.parse_sweep_for_drones(&stdout, 5_650_000_000, 5_950_000_000));
        }
        
        Ok(signals)
    }
    
    /// Parse spectrum sweep data for drone signatures
    fn parse_sweep_for_drones(&mut self, output: &str, start_hz: u64, end_hz: u64) -> Vec<DroneSignal> {
        let mut signals = Vec::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Parse hackrf_sweep output and look for strong signals
        let mut power_by_freq: HashMap<u64, f64> = HashMap::new();
        
        for line in output.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 7 {
                continue;
            }
            
            if let (Ok(hz_low), Ok(hz_bin)) = (
                parts[2].trim().parse::<u64>(),
                parts[4].trim().parse::<u64>(),
            ) {
                for (i, db_str) in parts[6..].iter().enumerate() {
                    if let Ok(power_db) = db_str.trim().parse::<f64>() {
                        let freq = hz_low + (i as u64 * hz_bin);
                        power_by_freq.insert(freq, power_db);
                    }
                }
            }
        }
        
        // Look for signals matching known drone signatures
        for sig in &self.signatures {
            if sig.center_freq_hz < start_hz || sig.center_freq_hz > end_hz {
                continue;
            }
            
            // Check if there's significant power in this frequency range
            let sig_start = sig.center_freq_hz.saturating_sub(sig.bandwidth_hz / 2);
            let sig_end = sig.center_freq_hz + sig.bandwidth_hz / 2;
            
            let matching_powers: Vec<f64> = power_by_freq.iter()
                .filter(|(&freq, _)| freq >= sig_start && freq <= sig_end)
                .map(|(_, &power)| power)
                .collect();
            
            if matching_powers.is_empty() {
                continue;
            }
            
            let max_power = matching_powers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let avg_power = matching_powers.iter().sum::<f64>() / matching_powers.len() as f64;
            
            // If signal is above threshold, consider it a detection
            if max_power > self.config.min_signal_db {
                let id = format!("{:?}_{}", sig.protocol, sig.center_freq_hz);
                
                let drone_signal = DroneSignal {
                    id: id.clone(),
                    frequency_hz: sig.center_freq_hz,
                    bandwidth_hz: sig.bandwidth_hz,
                    power_db: max_power,
                    signal_type: sig.signal_type.clone(),
                    drone_type: guess_drone_type(&sig.protocol),
                    protocol: Some(sig.protocol.clone()),
                    first_seen: self.detected.get(&id).map(|d| d.first_seen).unwrap_or(now),
                    last_seen: now,
                    duration_secs: 0,
                    direction: None,
                    threat_level: assess_drone_threat(max_power, &sig.signal_type),
                };
                
                self.detected.insert(id, drone_signal.clone());
                signals.push(drone_signal);
            }
        }
        
        signals
    }
    
    /// Get all detected drone signals
    pub fn get_detected(&self) -> Vec<&DroneSignal> {
        self.detected.values().collect()
    }
    
    /// Clean up old detections
    pub fn cleanup_old(&mut self, max_age_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.detected.retain(|_, signal| {
            now - signal.last_seen < max_age_secs
        });
    }
}

fn guess_drone_type(protocol: &DroneProtocol) -> Option<DroneType> {
    match protocol {
        DroneProtocol::DjiOcusync | DroneProtocol::DjiLightbridge => Some(DroneType::DjiMavic),
        DroneProtocol::Analog5_8 | DroneProtocol::Expresslrs | DroneProtocol::Crossfire => Some(DroneType::FpvRacing),
        _ => None,
    }
}

fn assess_drone_threat(power_db: f64, signal_type: &DroneSignalType) -> ThreatLevel {
    // Very strong signal = drone is very close
    if power_db > -30.0 {
        return ThreatLevel::High;
    }
    
    // Strong signal = drone is nearby
    if power_db > -50.0 {
        return ThreatLevel::Medium;
    }
    
    // Video signal = active surveillance possible
    if *signal_type == DroneSignalType::Video && power_db > -60.0 {
        return ThreatLevel::Medium;
    }
    
    ThreatLevel::Low
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_drone_signatures() {
        let sigs = DroneSignature::known_signatures();
        assert!(sigs.len() >= 5);
        
        // Check we have both 2.4 and 5.8 GHz signatures
        assert!(sigs.iter().any(|s| s.center_freq_hz >= 2_400_000_000 && s.center_freq_hz <= 2_500_000_000));
        assert!(sigs.iter().any(|s| s.center_freq_hz >= 5_700_000_000 && s.center_freq_hz <= 5_900_000_000));
    }
}
