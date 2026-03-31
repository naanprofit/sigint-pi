//! Spectrum Monitoring and Anomaly Detection
//! 
//! Uses rtl_power or hackrf_sweep to monitor spectrum and detect anomalies:
//! - Unexpected strong signals
//! - New transmitters in monitored bands
//! - Jamming detection
//! - Signal pattern changes

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::sync::broadcast;
use tracing::{info, warn, error, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Spectrum measurement point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumPoint {
    pub frequency_hz: u64,
    pub power_db: f64,
    pub timestamp: u64,
}

/// Spectrum scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumScan {
    pub start_freq_hz: u64,
    pub end_freq_hz: u64,
    pub step_hz: u64,
    pub points: Vec<SpectrumPoint>,
    pub timestamp: u64,
    pub duration_ms: u64,
}

/// Detected spectrum anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumAnomaly {
    pub id: String,
    pub anomaly_type: AnomalyType,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub power_db: f64,
    pub baseline_power_db: f64,
    pub deviation_db: f64,
    pub timestamp: u64,
    pub description: String,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalyType {
    NewTransmitter,      // New signal where none existed
    PowerIncrease,       // Significant power increase
    PowerDecrease,       // Signal disappeared (possible jamming victim)
    Jamming,             // Wideband high power (jamming)
    Intermittent,        // Signal appearing/disappearing rapidly
    FrequencyDrift,      // Signal drifting in frequency
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Frequency band definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyBand {
    pub name: String,
    pub start_hz: u64,
    pub end_hz: u64,
    pub description: String,
    pub sensitivity: f64, // dB threshold for anomaly
}

impl FrequencyBand {
    pub fn common_bands() -> Vec<Self> {
        vec![
            Self {
                name: "ISM 315".to_string(),
                start_hz: 314_000_000,
                end_hz: 316_000_000,
                description: "US garage doors, keyfobs".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "ISM 433".to_string(),
                start_hz: 432_000_000,
                end_hz: 435_000_000,
                description: "EU/Worldwide ISM, sensors".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "ISM 868".to_string(),
                start_hz: 867_000_000,
                end_hz: 869_000_000,
                description: "EU IoT devices".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "ISM 915".to_string(),
                start_hz: 914_000_000,
                end_hz: 916_000_000,
                description: "US IoT, LoRa".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "WiFi 2.4GHz".to_string(),
                start_hz: 2_400_000_000,
                end_hz: 2_500_000_000,
                description: "WiFi, Bluetooth, Zigbee".to_string(),
                sensitivity: 15.0,
            },
            Self {
                name: "Cellular 700".to_string(),
                start_hz: 698_000_000,
                end_hz: 756_000_000,
                description: "LTE Band 12/13/17".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "Cellular 850".to_string(),
                start_hz: 824_000_000,
                end_hz: 894_000_000,
                description: "Cellular Band 5/26".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "Cellular 1900".to_string(),
                start_hz: 1_850_000_000,
                end_hz: 1_990_000_000,
                description: "PCS Band 2/25".to_string(),
                sensitivity: 10.0,
            },
            Self {
                name: "GPS L1".to_string(),
                start_hz: 1_575_000_000,
                end_hz: 1_576_000_000,
                description: "GPS L1 signal".to_string(),
                sensitivity: 5.0, // GPS jamming is serious
            },
            Self {
                name: "Drone 5.8GHz".to_string(),
                start_hz: 5_725_000_000,
                end_hz: 5_875_000_000,
                description: "FPV drone video".to_string(),
                sensitivity: 15.0,
            },
        ]
    }
}

/// Spectrum scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectrumConfig {
    pub enabled: bool,
    pub bands: Vec<FrequencyBand>,
    pub scan_interval_secs: u64,
    pub baseline_samples: u32,
    pub anomaly_threshold_db: f64,
    pub device: SpectrumDevice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectrumDevice {
    RtlSdr { device_index: u32, gain: Option<i32> },
    HackRf,
    LimeSdr,
}

impl Default for SpectrumConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bands: FrequencyBand::common_bands(),
            scan_interval_secs: 60,
            baseline_samples: 10,
            anomaly_threshold_db: 10.0,
            device: SpectrumDevice::RtlSdr { device_index: 0, gain: None },
        }
    }
}

/// Spectrum monitor
pub struct SpectrumMonitor {
    config: SpectrumConfig,
    baseline: HashMap<u64, f64>, // frequency -> average power
    baseline_count: u32,
    tx: broadcast::Sender<SpectrumAnomaly>,
}

impl SpectrumMonitor {
    pub fn new(config: SpectrumConfig, tx: broadcast::Sender<SpectrumAnomaly>) -> Self {
        Self {
            config,
            baseline: HashMap::new(),
            baseline_count: 0,
            tx,
        }
    }
    
    /// Perform a single spectrum scan using rtl_power
    pub async fn scan_rtl_power(&self, band: &FrequencyBand) -> anyhow::Result<SpectrumScan> {
        let start = SystemTime::now();
        
        let (device_index, gain) = match &self.config.device {
            SpectrumDevice::RtlSdr { device_index, gain } => (*device_index, *gain),
            _ => (0, None),
        };
        
        let freq_range = format!(
            "{}:{}:{}",
            band.start_hz / 1_000_000,
            band.end_hz / 1_000_000,
            100_000 // 100 kHz bins
        );
        
        let mut args = vec![
            "-f".to_string(), freq_range,
            "-i".to_string(), "1".to_string(), // 1 second integration
            "-1".to_string(), // Single sweep
            "-d".to_string(), device_index.to_string(),
        ];
        
        if let Some(g) = gain {
            args.push("-g".to_string());
            args.push(g.to_string());
        }
        
        let output = Command::new("rtl_power")
            .args(&args)
            .output()
            .await?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let points = parse_rtl_power_output(&stdout, band);
        
        let duration = start.elapsed().unwrap_or_default();
        
        Ok(SpectrumScan {
            start_freq_hz: band.start_hz,
            end_freq_hz: band.end_hz,
            step_hz: 100_000,
            points,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            duration_ms: duration.as_millis() as u64,
        })
    }
    
    /// Perform a wideband sweep using hackrf_sweep
    pub async fn scan_hackrf_sweep(&self, start_mhz: u32, end_mhz: u32) -> anyhow::Result<SpectrumScan> {
        let start = SystemTime::now();
        
        let output = Command::new("hackrf_sweep")
            .args(&[
                "-f", &format!("{}:{}", start_mhz, end_mhz),
                "-w", "100000", // 100 kHz bin width
                "-1", // Single sweep
            ])
            .output()
            .await?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let points = parse_hackrf_sweep_output(&stdout);
        
        let duration = start.elapsed().unwrap_or_default();
        
        Ok(SpectrumScan {
            start_freq_hz: (start_mhz as u64) * 1_000_000,
            end_freq_hz: (end_mhz as u64) * 1_000_000,
            step_hz: 100_000,
            points,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            duration_ms: duration.as_millis() as u64,
        })
    }
    
    /// Update baseline with new scan
    pub fn update_baseline(&mut self, scan: &SpectrumScan) {
        for point in &scan.points {
            let entry = self.baseline.entry(point.frequency_hz).or_insert(point.power_db);
            // Exponential moving average
            *entry = (*entry * 0.9) + (point.power_db * 0.1);
        }
        self.baseline_count += 1;
    }
    
    /// Check scan for anomalies
    pub fn detect_anomalies(&self, scan: &SpectrumScan, band: &FrequencyBand) -> Vec<SpectrumAnomaly> {
        let mut anomalies = Vec::new();
        
        if self.baseline_count < self.config.baseline_samples {
            return anomalies; // Still building baseline
        }
        
        for point in &scan.points {
            if let Some(&baseline) = self.baseline.get(&point.frequency_hz) {
                let deviation = point.power_db - baseline;
                
                // Check for significant increase
                if deviation > band.sensitivity {
                    let anomaly_type = if deviation > 30.0 {
                        AnomalyType::Jamming
                    } else if baseline < -90.0 && point.power_db > -70.0 {
                        AnomalyType::NewTransmitter
                    } else {
                        AnomalyType::PowerIncrease
                    };
                    
                    let threat_level = classify_threat(&anomaly_type, deviation, band);
                    
                    anomalies.push(SpectrumAnomaly {
                        id: format!("{}_{}_{}", band.name, point.frequency_hz, scan.timestamp),
                        anomaly_type: anomaly_type.clone(),
                        frequency_hz: point.frequency_hz,
                        bandwidth_hz: scan.step_hz,
                        power_db: point.power_db,
                        baseline_power_db: baseline,
                        deviation_db: deviation,
                        timestamp: scan.timestamp,
                        description: format!(
                            "{:?} at {:.3} MHz: {:.1} dB above baseline",
                            anomaly_type,
                            point.frequency_hz as f64 / 1_000_000.0,
                            deviation
                        ),
                        threat_level,
                    });
                }
                
                // Check for significant decrease (possible jamming victim)
                if deviation < -band.sensitivity && baseline > -80.0 {
                    anomalies.push(SpectrumAnomaly {
                        id: format!("{}_{}_{}", band.name, point.frequency_hz, scan.timestamp),
                        anomaly_type: AnomalyType::PowerDecrease,
                        frequency_hz: point.frequency_hz,
                        bandwidth_hz: scan.step_hz,
                        power_db: point.power_db,
                        baseline_power_db: baseline,
                        deviation_db: deviation,
                        timestamp: scan.timestamp,
                        description: format!(
                            "Signal loss at {:.3} MHz: {:.1} dB below baseline",
                            point.frequency_hz as f64 / 1_000_000.0,
                            deviation.abs()
                        ),
                        threat_level: ThreatLevel::Medium,
                    });
                }
            }
        }
        
        anomalies
    }
}

fn parse_rtl_power_output(output: &str, band: &FrequencyBand) -> Vec<SpectrumPoint> {
    let mut points = Vec::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    for line in output.lines() {
        // rtl_power format: date, time, hz_low, hz_high, hz_step, samples, db values...
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 {
            continue;
        }
        
        if let (Ok(hz_low), Ok(hz_step)) = (
            parts[2].trim().parse::<u64>(),
            parts[4].trim().parse::<u64>(),
        ) {
            for (i, db_str) in parts[6..].iter().enumerate() {
                if let Ok(power_db) = db_str.trim().parse::<f64>() {
                    let freq = hz_low + (i as u64 * hz_step);
                    if freq >= band.start_hz && freq <= band.end_hz {
                        points.push(SpectrumPoint {
                            frequency_hz: freq,
                            power_db,
                            timestamp: now,
                        });
                    }
                }
            }
        }
    }
    
    points
}

fn parse_hackrf_sweep_output(output: &str) -> Vec<SpectrumPoint> {
    let mut points = Vec::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    for line in output.lines() {
        // hackrf_sweep format: date, time, hz_low, hz_high, hz_bin_width, num_samples, dB, dB, ...
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
                    points.push(SpectrumPoint {
                        frequency_hz: hz_low + (i as u64 * hz_bin),
                        power_db,
                        timestamp: now,
                    });
                }
            }
        }
    }
    
    points
}

fn classify_threat(anomaly_type: &AnomalyType, deviation: f64, band: &FrequencyBand) -> ThreatLevel {
    match anomaly_type {
        AnomalyType::Jamming => {
            // GPS jamming is critical
            if band.name.contains("GPS") {
                ThreatLevel::Critical
            } else if band.name.contains("Cellular") {
                ThreatLevel::High
            } else {
                ThreatLevel::Medium
            }
        }
        AnomalyType::NewTransmitter => {
            if deviation > 40.0 {
                ThreatLevel::High
            } else {
                ThreatLevel::Medium
            }
        }
        AnomalyType::PowerIncrease => {
            if deviation > 30.0 {
                ThreatLevel::Medium
            } else {
                ThreatLevel::Low
            }
        }
        _ => ThreatLevel::Info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frequency_bands() {
        let bands = FrequencyBand::common_bands();
        assert!(bands.len() >= 5);
        assert!(bands.iter().any(|b| b.name.contains("433")));
    }
}
