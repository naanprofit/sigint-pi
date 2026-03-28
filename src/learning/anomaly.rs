use std::collections::HashMap;
use chrono::{Duration, Utc, Timelike};

pub use super::baseline::DeviceStats;

#[derive(Debug, Clone)]
pub struct AnomalyScore {
    pub total_score: f64,
    pub is_new_device: bool,
    pub rssi_anomaly: f64,
    pub time_anomaly: f64,
    pub behavior_anomaly: f64,
    pub reason: String,
}

impl AnomalyScore {
    pub fn is_anomalous(&self) -> bool {
        self.total_score > 0.7
    }

    pub fn priority(&self) -> AlertPriority {
        if self.total_score > 0.9 {
            AlertPriority::Critical
        } else if self.total_score > 0.7 {
            AlertPriority::High
        } else if self.total_score > 0.5 {
            AlertPriority::Medium
        } else {
            AlertPriority::Low
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertPriority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct AnomalyDetector {
    baseline_devices: HashMap<String, BaselineProfile>,
    threshold: f64,
}

#[derive(Debug, Clone)]
struct BaselineProfile {
    mac_address: String,
    avg_rssi: f64,
    rssi_stddev: f64,
    typical_hours: Vec<u8>,
    visit_frequency: f64,
    probed_ssids: Vec<String>,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            baseline_devices: HashMap::new(),
            threshold: 0.7,
        }
    }

    pub fn add_baseline_device(&mut self, mac: String, stats: DeviceStats) {
        let days_observed = stats.last_seen.signed_duration_since(stats.first_seen).num_days().max(1) as f64;
        
        let profile = BaselineProfile {
            mac_address: mac.clone(),
            avg_rssi: stats.avg_rssi(),
            rssi_stddev: stats.rssi_stddev().max(3.0), // Minimum 3dB stddev
            typical_hours: stats.hours_seen.clone(),
            visit_frequency: stats.visit_count as f64 / days_observed,
            probed_ssids: stats.probed_ssids.clone(),
        };

        self.baseline_devices.insert(mac, profile);
    }

    pub fn score_device(&self, mac: &str, rssi: i32, stats: &DeviceStats) -> AnomalyScore {
        // Check if device is in baseline
        if let Some(profile) = self.baseline_devices.get(mac) {
            // Known device - check for anomalous behavior
            let rssi_anomaly = self.score_rssi_anomaly(rssi, profile);
            let time_anomaly = self.score_time_anomaly(profile);
            let behavior_anomaly = self.score_behavior_anomaly(stats, profile);

            let total_score = (rssi_anomaly * 0.3 + time_anomaly * 0.3 + behavior_anomaly * 0.4).min(1.0);

            let reason = if rssi_anomaly > 0.7 {
                format!("Unusual signal strength: {} dBm (expected ~{:.0} dBm)", rssi, profile.avg_rssi)
            } else if time_anomaly > 0.7 {
                format!("Unusual time of appearance: hour {}", Utc::now().hour())
            } else if behavior_anomaly > 0.7 {
                "Unusual behavior pattern".to_string()
            } else {
                "Normal known device".to_string()
            };

            AnomalyScore {
                total_score,
                is_new_device: false,
                rssi_anomaly,
                time_anomaly,
                behavior_anomaly,
                reason,
            }
        } else {
            // New/unknown device
            let rssi_score = self.score_new_device_rssi(rssi);
            let time_score = self.score_new_device_time();

            // New device with strong signal = high anomaly
            let total_score = 0.5 + (rssi_score * 0.3) + (time_score * 0.2);

            let reason = if rssi > -50 {
                format!("New device with very strong signal: {} dBm (likely nearby)", rssi)
            } else if rssi > -60 {
                format!("New device with strong signal: {} dBm", rssi)
            } else if rssi > -70 {
                format!("New device detected: {} dBm", rssi)
            } else {
                format!("New device with weak signal: {} dBm (likely passing by)", rssi)
            };

            AnomalyScore {
                total_score,
                is_new_device: true,
                rssi_anomaly: rssi_score,
                time_anomaly: time_score,
                behavior_anomaly: 0.0,
                reason,
            }
        }
    }

    fn score_rssi_anomaly(&self, rssi: i32, profile: &BaselineProfile) -> f64 {
        // How many standard deviations from mean?
        let z_score = ((rssi as f64 - profile.avg_rssi) / profile.rssi_stddev).abs();
        
        // Convert to 0-1 score (3+ stddev = max anomaly)
        (z_score / 3.0).min(1.0)
    }

    fn score_time_anomaly(&self, profile: &BaselineProfile) -> f64 {
        let current_hour = Utc::now().hour() as u8;
        
        if profile.typical_hours.contains(&current_hour) {
            0.0
        } else {
            // Find distance to nearest typical hour
            let min_distance = profile.typical_hours.iter()
                .map(|&h| {
                    let diff = (h as i32 - current_hour as i32).abs();
                    diff.min(24 - diff) as f64
                })
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(12.0);
            
            // 6+ hours away = max anomaly
            (min_distance / 6.0).min(1.0)
        }
    }

    fn score_behavior_anomaly(&self, stats: &DeviceStats, profile: &BaselineProfile) -> f64 {
        let mut anomaly = 0.0;
        let mut factors = 0;

        // Check if probing unusual SSIDs
        let new_ssids: Vec<_> = stats.probed_ssids.iter()
            .filter(|ssid| !profile.probed_ssids.contains(ssid))
            .collect();
        
        if !new_ssids.is_empty() {
            anomaly += 0.5;
            factors += 1;
        }

        // Check visit frequency anomaly
        let days_since_first = Utc::now()
            .signed_duration_since(stats.first_seen)
            .num_days()
            .max(1) as f64;
        let current_frequency = stats.visit_count as f64 / days_since_first;
        
        if current_frequency > profile.visit_frequency * 2.0 {
            anomaly += 0.5;
            factors += 1;
        }

        if factors > 0 {
            anomaly / factors as f64
        } else {
            0.0
        }
    }

    fn score_new_device_rssi(&self, rssi: i32) -> f64 {
        // Strong signal = more concerning for new device
        // -40 dBm = 1.0 (very close)
        // -60 dBm = 0.5 (moderate)
        // -80 dBm = 0.0 (far away)
        ((-rssi as f64 - 80.0) / -40.0).clamp(0.0, 1.0)
    }

    fn score_new_device_time(&self) -> f64 {
        // Late night = more suspicious
        let hour = Utc::now().hour();
        match hour {
            0..=5 => 0.8,   // Midnight to 5am
            6..=8 => 0.3,   // Early morning
            9..=17 => 0.0,  // Business hours
            18..=21 => 0.1, // Evening
            22..=23 => 0.5, // Late evening
            _ => 0.0,
        }
    }

    pub fn is_device_known(&self, mac: &str) -> bool {
        self.baseline_devices.contains_key(mac)
    }

    pub fn get_baseline_count(&self) -> usize {
        self.baseline_devices.len()
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_device_scoring() {
        let detector = AnomalyDetector::new();
        let stats = DeviceStats {
            mac_address: "00:11:22:33:44:55".to_string(),
            rssi_samples: vec![-45],
            hours_seen: vec![14],
            visit_count: 1,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            total_time_seen: chrono::Duration::zero(),
            probed_ssids: vec![],
        };

        let score = detector.score_device("00:11:22:33:44:55", -45, &stats);
        
        assert!(score.is_new_device);
        assert!(score.total_score > 0.5); // New + strong signal
    }

    #[test]
    fn test_rssi_anomaly() {
        let mut detector = AnomalyDetector::new();
        
        // Add baseline device
        let stats = DeviceStats {
            mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
            rssi_samples: vec![-70, -72, -68, -71, -69],
            hours_seen: vec![9, 10, 11, 14, 15, 16],
            visit_count: 5,
            first_seen: Utc::now() - chrono::Duration::days(7),
            last_seen: Utc::now(),
            total_time_seen: chrono::Duration::hours(10),
            probed_ssids: vec!["HomeWiFi".to_string()],
        };
        
        detector.add_baseline_device("AA:BB:CC:DD:EE:FF".to_string(), stats.clone());

        // Test with normal RSSI
        let score = detector.score_device("AA:BB:CC:DD:EE:FF", -70, &stats);
        assert!(!score.is_new_device);
        assert!(score.rssi_anomaly < 0.3);

        // Test with anomalous RSSI
        let score = detector.score_device("AA:BB:CC:DD:EE:FF", -40, &stats);
        assert!(score.rssi_anomaly > 0.5);
    }
}


