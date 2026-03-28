use chrono::{DateTime, Duration, Utc, Timelike, Weekday, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Device fingerprinting based on behavioral patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    pub mac_address: String,
    pub fingerprint_id: String,
    
    // Probe behavior
    pub probe_ssids: Vec<String>,
    pub probe_sequence_hash: Option<String>,
    pub probe_interval_avg_ms: Option<f64>,
    
    // Timing patterns
    pub typical_hours: Vec<u8>,
    pub typical_days: Vec<Weekday>,
    pub avg_visit_duration_mins: f64,
    pub visit_frequency_per_day: f64,
    
    // Signal characteristics
    pub rssi_mean: f64,
    pub rssi_stddev: f64,
    pub rssi_min: i32,
    pub rssi_max: i32,
    
    // Network associations
    pub associated_bssids: Vec<String>,
    pub preferred_channels: Vec<u8>,
    
    // Device classification
    pub device_class: DeviceClass,
    pub mobility_score: f64,  // 0 = stationary, 1 = highly mobile
    pub confidence: f64,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceClass {
    Smartphone,
    Laptop,
    Tablet,
    IoTDevice,
    AccessPoint,
    Wearable,
    Vehicle,
    Unknown,
}

pub struct FingerprintEngine {
    fingerprints: HashMap<String, DeviceFingerprint>,
    probe_history: HashMap<String, Vec<ProbeEvent>>,
    sighting_history: HashMap<String, Vec<SightingEvent>>,
}

#[derive(Debug, Clone)]
struct ProbeEvent {
    ssid: String,
    timestamp: DateTime<Utc>,
    rssi: i32,
}

#[derive(Debug, Clone)]
struct SightingEvent {
    timestamp: DateTime<Utc>,
    rssi: i32,
    channel: u8,
    bssid: Option<String>,
}

impl FingerprintEngine {
    pub fn new() -> Self {
        Self {
            fingerprints: HashMap::new(),
            probe_history: HashMap::new(),
            sighting_history: HashMap::new(),
        }
    }

    pub fn record_probe(&mut self, mac: &str, ssid: &str, rssi: i32) {
        let events = self.probe_history.entry(mac.to_string()).or_default();
        events.push(ProbeEvent {
            ssid: ssid.to_string(),
            timestamp: Utc::now(),
            rssi,
        });

        // Keep last 1000 probes per device
        if events.len() > 1000 {
            events.drain(0..100);
        }
    }

    pub fn record_sighting(&mut self, mac: &str, rssi: i32, channel: u8, bssid: Option<&str>) {
        let events = self.sighting_history.entry(mac.to_string()).or_default();
        events.push(SightingEvent {
            timestamp: Utc::now(),
            rssi,
            channel,
            bssid: bssid.map(String::from),
        });

        // Keep last 10000 sightings per device
        if events.len() > 10000 {
            events.drain(0..1000);
        }
    }

    pub fn compute_fingerprint(&mut self, mac: &str) -> Option<DeviceFingerprint> {
        let probes = self.probe_history.get(mac)?;
        let sightings = self.sighting_history.get(mac)?;

        if sightings.len() < 10 {
            return None; // Not enough data
        }

        let now = Utc::now();

        // Analyze probe patterns
        let probe_ssids: Vec<String> = probes
            .iter()
            .map(|p| p.ssid.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let probe_sequence_hash = self.compute_probe_sequence_hash(probes);
        let probe_interval_avg_ms = self.compute_probe_interval(probes);

        // Analyze timing patterns
        let typical_hours = self.compute_typical_hours(sightings);
        let typical_days = self.compute_typical_days(sightings);
        let (avg_visit_duration, visit_frequency) = self.compute_visit_patterns(sightings);

        // Analyze signal characteristics
        let rssi_values: Vec<i32> = sightings.iter().map(|s| s.rssi).collect();
        let rssi_mean = rssi_values.iter().map(|&r| r as f64).sum::<f64>() / rssi_values.len() as f64;
        let rssi_variance = rssi_values.iter()
            .map(|&r| (r as f64 - rssi_mean).powi(2))
            .sum::<f64>() / rssi_values.len() as f64;
        let rssi_stddev = rssi_variance.sqrt();
        let rssi_min = *rssi_values.iter().min().unwrap_or(&-100);
        let rssi_max = *rssi_values.iter().max().unwrap_or(&-100);

        // Analyze network associations
        let associated_bssids: Vec<String> = sightings
            .iter()
            .filter_map(|s| s.bssid.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let preferred_channels: Vec<u8> = {
            let mut channel_counts: HashMap<u8, usize> = HashMap::new();
            for s in sightings {
                *channel_counts.entry(s.channel).or_default() += 1;
            }
            let mut channels: Vec<_> = channel_counts.into_iter().collect();
            channels.sort_by(|a, b| b.1.cmp(&a.1));
            channels.into_iter().take(3).map(|(c, _)| c).collect()
        };

        // Classify device
        let device_class = self.classify_device(&probe_ssids, &typical_hours, rssi_stddev, &associated_bssids);
        let mobility_score = self.compute_mobility_score(rssi_stddev, &associated_bssids);

        // Compute confidence based on data quantity
        let confidence = (sightings.len() as f64 / 1000.0).min(1.0);

        // Generate fingerprint ID (hash of key characteristics)
        let fingerprint_id = self.generate_fingerprint_id(mac, &probe_ssids, &typical_hours);

        let fingerprint = DeviceFingerprint {
            mac_address: mac.to_string(),
            fingerprint_id,
            probe_ssids,
            probe_sequence_hash,
            probe_interval_avg_ms,
            typical_hours,
            typical_days,
            avg_visit_duration_mins: avg_visit_duration,
            visit_frequency_per_day: visit_frequency,
            rssi_mean,
            rssi_stddev,
            rssi_min,
            rssi_max,
            associated_bssids,
            preferred_channels,
            device_class,
            mobility_score,
            confidence,
            created_at: now,
            updated_at: now,
        };

        self.fingerprints.insert(mac.to_string(), fingerprint.clone());
        Some(fingerprint)
    }

    fn compute_probe_sequence_hash(&self, probes: &[ProbeEvent]) -> Option<String> {
        if probes.len() < 3 {
            return None;
        }

        // Create hash from SSID sequence pattern
        let sequence: Vec<&str> = probes.iter()
            .take(10)
            .map(|p| p.ssid.as_str())
            .collect();
        
        let hash_input = sequence.join(",");
        Some(format!("{:x}", md5_hash(&hash_input)))
    }

    fn compute_probe_interval(&self, probes: &[ProbeEvent]) -> Option<f64> {
        if probes.len() < 2 {
            return None;
        }

        let mut intervals: Vec<i64> = Vec::new();
        for window in probes.windows(2) {
            let diff = window[1].timestamp.signed_duration_since(window[0].timestamp);
            if diff.num_milliseconds() > 0 && diff.num_milliseconds() < 60000 {
                intervals.push(diff.num_milliseconds());
            }
        }

        if intervals.is_empty() {
            return None;
        }

        Some(intervals.iter().sum::<i64>() as f64 / intervals.len() as f64)
    }

    fn compute_typical_hours(&self, sightings: &[SightingEvent]) -> Vec<u8> {
        let mut hour_counts: HashMap<u8, usize> = HashMap::new();
        for s in sightings {
            *hour_counts.entry(s.timestamp.hour() as u8).or_default() += 1;
        }

        let total: usize = hour_counts.values().sum();
        let threshold = total / 24 / 2; // Below average = not typical

        let mut typical: Vec<u8> = hour_counts
            .into_iter()
            .filter(|(_, count)| *count > threshold)
            .map(|(hour, _)| hour)
            .collect();
        
        typical.sort();
        typical
    }

    fn compute_typical_days(&self, sightings: &[SightingEvent]) -> Vec<Weekday> {
        let mut day_counts: HashMap<Weekday, usize> = HashMap::new();
        for s in sightings {
            *day_counts.entry(s.timestamp.weekday()).or_default() += 1;
        }

        let total: usize = day_counts.values().sum();
        let threshold = total / 7 / 2;

        day_counts
            .into_iter()
            .filter(|(_, count)| *count > threshold)
            .map(|(day, _)| day)
            .collect()
    }

    fn compute_visit_patterns(&self, sightings: &[SightingEvent]) -> (f64, f64) {
        if sightings.len() < 2 {
            return (0.0, 0.0);
        }

        // Group sightings into visits (gap > 30 min = new visit)
        let mut visits: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();
        let mut visit_start = sightings[0].timestamp;
        let mut visit_end = sightings[0].timestamp;

        for s in sightings.iter().skip(1) {
            if s.timestamp.signed_duration_since(visit_end) > Duration::minutes(30) {
                visits.push((visit_start, visit_end));
                visit_start = s.timestamp;
            }
            visit_end = s.timestamp;
        }
        visits.push((visit_start, visit_end));

        // Calculate average visit duration
        let total_duration: i64 = visits.iter()
            .map(|(start, end)| end.signed_duration_since(*start).num_minutes())
            .sum();
        let avg_duration = total_duration as f64 / visits.len() as f64;

        // Calculate visit frequency
        let first_seen = sightings.first().map(|s| s.timestamp).unwrap();
        let last_seen = sightings.last().map(|s| s.timestamp).unwrap();
        let days_observed = last_seen.signed_duration_since(first_seen).num_days().max(1) as f64;
        let visit_frequency = visits.len() as f64 / days_observed;

        (avg_duration, visit_frequency)
    }

    fn classify_device(
        &self,
        probe_ssids: &[String],
        typical_hours: &[u8],
        rssi_stddev: f64,
        associated_bssids: &[String],
    ) -> DeviceClass {
        // IoT devices: low RSSI variance, few/no probes, always on
        if rssi_stddev < 3.0 && probe_ssids.is_empty() && typical_hours.len() > 20 {
            return DeviceClass::IoTDevice;
        }

        // Access points: very low variance, no probes, always present
        if rssi_stddev < 2.0 && probe_ssids.is_empty() && typical_hours.len() == 24 {
            return DeviceClass::AccessPoint;
        }

        // Smartphones: many probes, variable hours, moderate variance
        if probe_ssids.len() > 5 && rssi_stddev > 5.0 {
            return DeviceClass::Smartphone;
        }

        // Laptops: fewer probes than phones, business hours pattern
        let business_hours: Vec<u8> = (9..18).collect();
        let business_overlap = typical_hours.iter()
            .filter(|h| business_hours.contains(h))
            .count();
        if business_overlap > 5 && probe_ssids.len() > 2 && probe_ssids.len() < 10 {
            return DeviceClass::Laptop;
        }

        // Wearables: follows smartphone patterns but fewer associations
        if associated_bssids.len() <= 2 && rssi_stddev > 8.0 {
            return DeviceClass::Wearable;
        }

        DeviceClass::Unknown
    }

    fn compute_mobility_score(&self, rssi_stddev: f64, associated_bssids: &[String]) -> f64 {
        // High RSSI variance + many BSSIDs = mobile
        let rssi_factor = (rssi_stddev / 20.0).min(1.0);
        let bssid_factor = (associated_bssids.len() as f64 / 10.0).min(1.0);
        
        (rssi_factor * 0.6 + bssid_factor * 0.4).min(1.0)
    }

    fn generate_fingerprint_id(&self, mac: &str, probe_ssids: &[String], typical_hours: &[u8]) -> String {
        let mut input = mac.to_string();
        input.push_str(&probe_ssids.join(","));
        input.push_str(&typical_hours.iter().map(|h| h.to_string()).collect::<Vec<_>>().join(","));
        
        format!("fp_{:x}", md5_hash(&input))
    }

    pub fn get_fingerprint(&self, mac: &str) -> Option<&DeviceFingerprint> {
        self.fingerprints.get(mac)
    }

    pub fn match_fingerprint(&self, mac: &str) -> Option<(String, f64)> {
        let target = self.fingerprints.get(mac)?;
        
        let mut best_match: Option<(String, f64)> = None;
        
        for (other_mac, other_fp) in &self.fingerprints {
            if other_mac == mac {
                continue;
            }

            let similarity = self.compute_similarity(target, other_fp);
            if similarity > 0.8 {
                if best_match.is_none() || similarity > best_match.as_ref().unwrap().1 {
                    best_match = Some((other_mac.clone(), similarity));
                }
            }
        }

        best_match
    }

    fn compute_similarity(&self, a: &DeviceFingerprint, b: &DeviceFingerprint) -> f64 {
        let mut score = 0.0;
        let mut weights = 0.0;

        // Probe SSID overlap (high weight)
        let ssid_overlap = a.probe_ssids.iter()
            .filter(|s| b.probe_ssids.contains(s))
            .count() as f64;
        let ssid_total = (a.probe_ssids.len() + b.probe_ssids.len()) as f64 / 2.0;
        if ssid_total > 0.0 {
            score += (ssid_overlap / ssid_total) * 0.4;
            weights += 0.4;
        }

        // Timing pattern overlap
        let hour_overlap = a.typical_hours.iter()
            .filter(|h| b.typical_hours.contains(h))
            .count() as f64;
        let hour_total = (a.typical_hours.len() + b.typical_hours.len()) as f64 / 2.0;
        if hour_total > 0.0 {
            score += (hour_overlap / hour_total) * 0.2;
            weights += 0.2;
        }

        // RSSI similarity
        let rssi_diff = (a.rssi_mean - b.rssi_mean).abs();
        let rssi_sim = (1.0 - rssi_diff / 50.0).max(0.0);
        score += rssi_sim * 0.2;
        weights += 0.2;

        // Device class match
        if a.device_class == b.device_class {
            score += 0.2;
        }
        weights += 0.2;

        if weights > 0.0 {
            score / weights
        } else {
            0.0
        }
    }
}

impl Default for FingerprintEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Simple hash function for fingerprint IDs
fn md5_hash(input: &str) -> u64 {
    let mut hash: u64 = 0;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_engine() {
        let mut engine = FingerprintEngine::new();
        
        // Simulate a smartphone
        for i in 0..100 {
            engine.record_probe("AA:BB:CC:DD:EE:FF", "HomeWiFi", -65 + (i % 10) as i32);
            engine.record_probe("AA:BB:CC:DD:EE:FF", "WorkWiFi", -70 + (i % 10) as i32);
            engine.record_sighting("AA:BB:CC:DD:EE:FF", -65 + (i % 15) as i32, 6, Some("00:11:22:33:44:55"));
        }

        let fp = engine.compute_fingerprint("AA:BB:CC:DD:EE:FF");
        assert!(fp.is_some());
        
        let fp = fp.unwrap();
        assert_eq!(fp.probe_ssids.len(), 2);
        assert!(fp.rssi_stddev > 0.0);
    }
}
