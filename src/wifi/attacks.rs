use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::parser::{FrameType, WifiDevice};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEvent {
    pub attack_type: AttackType,
    pub source_mac: String,
    pub target_mac: Option<String>,
    pub bssid: Option<String>,
    pub severity: AttackSeverity,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub evidence: AttackEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackType {
    DeauthFlood,
    DisassocFlood,
    BeaconFlood,
    EvilTwin,
    KarmaAttack,
    AuthDoS,
    ProbeFlood,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackEvidence {
    pub frame_count: u32,
    pub time_window_seconds: u32,
    pub unique_targets: u32,
    pub channels_affected: Vec<u8>,
}

pub struct AttackDetector {
    deauth_tracker: HashMap<String, Vec<DateTime<Utc>>>,
    beacon_tracker: HashMap<String, BeaconInfo>,
    probe_response_tracker: HashMap<String, Vec<DateTime<Utc>>>,
    window_duration: Duration,
    deauth_threshold: u32,
    beacon_flood_threshold: u32,
}

#[derive(Debug, Clone)]
struct BeaconInfo {
    ssid: String,
    bssid: String,
    channel: u8,
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    beacon_count: u32,
}

impl AttackDetector {
    pub fn new() -> Self {
        Self {
            deauth_tracker: HashMap::new(),
            beacon_tracker: HashMap::new(),
            probe_response_tracker: HashMap::new(),
            window_duration: Duration::seconds(60),
            deauth_threshold: 10,
            beacon_flood_threshold: 100,
        }
    }

    pub fn analyze(&mut self, device: &WifiDevice) -> Option<AttackEvent> {
        let now = Utc::now();

        // Clean old entries
        self.cleanup_old_entries(now);

        // Check for different attack types
        if let Some(attack) = self.detect_deauth_flood(device, now) {
            return Some(attack);
        }

        if let Some(attack) = self.detect_evil_twin(device, now) {
            return Some(attack);
        }

        if let Some(attack) = self.detect_beacon_flood(device, now) {
            return Some(attack);
        }

        if let Some(attack) = self.detect_karma_attack(device, now) {
            return Some(attack);
        }

        None
    }

    fn detect_deauth_flood(&mut self, device: &WifiDevice, now: DateTime<Utc>) -> Option<AttackEvent> {
        // Check frame type - deauth frames are management subtype 12
        // For simplicity, we track all management frames from aggressive sources
        if device.frame_type != FrameType::Management {
            return None;
        }

        let entry = self.deauth_tracker
            .entry(device.mac_address.clone())
            .or_insert_with(Vec::new);
        
        entry.push(now);

        // Count frames in window
        let count = entry.iter()
            .filter(|t| now.signed_duration_since(**t) < self.window_duration)
            .count() as u32;

        if count >= self.deauth_threshold {
            return Some(AttackEvent {
                attack_type: AttackType::DeauthFlood,
                source_mac: device.mac_address.clone(),
                target_mac: device.bssid.clone(),
                bssid: device.bssid.clone(),
                severity: AttackSeverity::High,
                description: format!(
                    "Deauthentication flood detected: {} frames in {} seconds from {}",
                    count, self.window_duration.num_seconds(), device.mac_address
                ),
                timestamp: now,
                evidence: AttackEvidence {
                    frame_count: count,
                    time_window_seconds: self.window_duration.num_seconds() as u32,
                    unique_targets: 1,
                    channels_affected: vec![device.channel],
                },
            });
        }

        None
    }

    fn detect_evil_twin(&mut self, device: &WifiDevice, now: DateTime<Utc>) -> Option<AttackEvent> {
        // Evil twin: same SSID, different BSSID
        if !device.is_ap {
            return None;
        }

        let ssid = device.ssid.as_ref()?;
        let bssid = device.bssid.as_ref()?;

        // Check if we've seen this SSID with a different BSSID
        let existing = self.beacon_tracker
            .values()
            .find(|b| &b.ssid == ssid && &b.bssid != bssid);

        if let Some(original) = existing {
            // Same SSID, different BSSID on different channel = likely evil twin
            if original.channel != device.channel {
                return Some(AttackEvent {
                    attack_type: AttackType::EvilTwin,
                    source_mac: device.mac_address.clone(),
                    target_mac: Some(original.bssid.clone()),
                    bssid: Some(bssid.clone()),
                    severity: AttackSeverity::Critical,
                    description: format!(
                        "Possible Evil Twin detected: SSID '{}' on channel {} (original on channel {})",
                        ssid, device.channel, original.channel
                    ),
                    timestamp: now,
                    evidence: AttackEvidence {
                        frame_count: 1,
                        time_window_seconds: 0,
                        unique_targets: 1,
                        channels_affected: vec![device.channel, original.channel],
                    },
                });
            }
        }

        // Track this beacon
        self.beacon_tracker.insert(
            bssid.clone(),
            BeaconInfo {
                ssid: ssid.clone(),
                bssid: bssid.clone(),
                channel: device.channel,
                first_seen: now,
                last_seen: now,
                beacon_count: 1,
            },
        );

        None
    }

    fn detect_beacon_flood(&mut self, device: &WifiDevice, now: DateTime<Utc>) -> Option<AttackEvent> {
        if !device.is_ap {
            return None;
        }

        let bssid = device.bssid.as_ref()?;

        if let Some(info) = self.beacon_tracker.get_mut(bssid) {
            info.beacon_count += 1;
            info.last_seen = now;

            let duration = now.signed_duration_since(info.first_seen);
            if duration.num_seconds() > 0 {
                let rate = info.beacon_count as f64 / duration.num_seconds() as f64;
                
                // Normal beacon rate is ~10/sec, > 50/sec is suspicious
                if rate > 50.0 && info.beacon_count > self.beacon_flood_threshold {
                    return Some(AttackEvent {
                        attack_type: AttackType::BeaconFlood,
                        source_mac: device.mac_address.clone(),
                        target_mac: None,
                        bssid: Some(bssid.clone()),
                        severity: AttackSeverity::Medium,
                        description: format!(
                            "Beacon flood detected: {} beacons/sec from {}",
                            rate as u32, bssid
                        ),
                        timestamp: now,
                        evidence: AttackEvidence {
                            frame_count: info.beacon_count,
                            time_window_seconds: duration.num_seconds() as u32,
                            unique_targets: 0,
                            channels_affected: vec![device.channel],
                        },
                    });
                }
            }
        }

        None
    }

    fn detect_karma_attack(&mut self, device: &WifiDevice, now: DateTime<Utc>) -> Option<AttackEvent> {
        // KARMA/MANA attack: AP responds to all probe requests
        // We detect by tracking probe responses from a single MAC to many different SSIDs
        
        // This would require tracking probe responses, which needs frame subtype parsing
        // Simplified version: if we see an AP advertising multiple SSIDs rapidly
        
        None // TODO: Implement full KARMA detection
    }

    fn cleanup_old_entries(&mut self, now: DateTime<Utc>) {
        // Remove old deauth tracking entries
        for entries in self.deauth_tracker.values_mut() {
            entries.retain(|t| now.signed_duration_since(*t) < self.window_duration);
        }
        self.deauth_tracker.retain(|_, v| !v.is_empty());

        // Remove old beacon tracking (keep for 5 minutes)
        let beacon_retention = Duration::minutes(5);
        self.beacon_tracker.retain(|_, v| {
            now.signed_duration_since(v.last_seen) < beacon_retention
        });
    }
}

impl Default for AttackDetector {
    fn default() -> Self {
        Self::new()
    }
}
