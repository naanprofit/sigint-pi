use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};

/// Attack signature definitions for enhanced detection
#[derive(Debug, Clone)]
pub struct AttackSignature {
    pub name: &'static str,
    pub description: &'static str,
    pub frame_types: Vec<u8>,
    pub threshold_count: u32,
    pub window_seconds: u32,
    pub severity: SignatureSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum SignatureSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Enhanced signature-based attack detector
pub struct SignatureDetector {
    signatures: Vec<AttackSignature>,
    frame_counts: HashMap<String, Vec<FrameRecord>>,
    detected_attacks: Vec<DetectedAttack>,
}

#[derive(Debug, Clone)]
struct FrameRecord {
    frame_type: u8,
    frame_subtype: u8,
    source_mac: String,
    dest_mac: String,
    bssid: Option<String>,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DetectedAttack {
    pub signature_name: String,
    pub source_mac: String,
    pub target_mac: Option<String>,
    pub bssid: Option<String>,
    pub frame_count: u32,
    pub severity: SignatureSeverity,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub description: String,
}

impl SignatureDetector {
    pub fn new() -> Self {
        Self {
            signatures: Self::default_signatures(),
            frame_counts: HashMap::new(),
            detected_attacks: Vec::new(),
        }
    }

    fn default_signatures() -> Vec<AttackSignature> {
        vec![
            // Deauthentication flood
            AttackSignature {
                name: "DEAUTH_FLOOD",
                description: "Mass deauthentication attack - devices being forcibly disconnected",
                frame_types: vec![0x0C], // Deauth subtype
                threshold_count: 10,
                window_seconds: 10,
                severity: SignatureSeverity::High,
            },
            // Disassociation flood
            AttackSignature {
                name: "DISASSOC_FLOOD",
                description: "Mass disassociation attack - similar to deauth but different method",
                frame_types: vec![0x0A], // Disassoc subtype
                threshold_count: 10,
                window_seconds: 10,
                severity: SignatureSeverity::High,
            },
            // Authentication flood (DoS)
            AttackSignature {
                name: "AUTH_FLOOD",
                description: "Authentication request flood - attempting to overwhelm AP",
                frame_types: vec![0x0B], // Auth subtype
                threshold_count: 50,
                window_seconds: 10,
                severity: SignatureSeverity::Medium,
            },
            // Beacon flood (fake APs)
            AttackSignature {
                name: "BEACON_FLOOD",
                description: "Beacon frame flood - creating fake access points",
                frame_types: vec![0x08], // Beacon subtype
                threshold_count: 100,
                window_seconds: 5,
                severity: SignatureSeverity::Medium,
            },
            // Probe response flood (KARMA-style)
            AttackSignature {
                name: "PROBE_RESP_FLOOD",
                description: "Probe response flood - possible KARMA/MANA attack",
                frame_types: vec![0x05], // Probe response subtype
                threshold_count: 30,
                window_seconds: 10,
                severity: SignatureSeverity::Critical,
            },
            // RTS flood
            AttackSignature {
                name: "RTS_FLOOD",
                description: "RTS frame flood - attempting to reserve channel",
                frame_types: vec![0x1B], // RTS
                threshold_count: 100,
                window_seconds: 5,
                severity: SignatureSeverity::Low,
            },
            // CTS flood
            AttackSignature {
                name: "CTS_FLOOD",
                description: "CTS frame flood - virtual carrier sense attack",
                frame_types: vec![0x1C], // CTS
                threshold_count: 100,
                window_seconds: 5,
                severity: SignatureSeverity::Medium,
            },
            // Association flood
            AttackSignature {
                name: "ASSOC_FLOOD",
                description: "Association request flood - attempting to exhaust AP resources",
                frame_types: vec![0x00], // Assoc request subtype
                threshold_count: 20,
                window_seconds: 10,
                severity: SignatureSeverity::Medium,
            },
            // Broadcast deauth (mass disconnect)
            AttackSignature {
                name: "BROADCAST_DEAUTH",
                description: "Broadcast deauthentication - disconnecting ALL clients from AP",
                frame_types: vec![0x0C],
                threshold_count: 3,
                window_seconds: 5,
                severity: SignatureSeverity::Critical,
            },
        ]
    }

    pub fn record_frame(
        &mut self,
        frame_type: u8,
        frame_subtype: u8,
        source_mac: &str,
        dest_mac: &str,
        bssid: Option<&str>,
    ) -> Option<DetectedAttack> {
        let now = Utc::now();
        
        // Create composite key for tracking
        let key = format!("{}:{}:{}", source_mac, frame_type, frame_subtype);
        
        let records = self.frame_counts.entry(key.clone()).or_default();
        records.push(FrameRecord {
            frame_type,
            frame_subtype,
            source_mac: source_mac.to_string(),
            dest_mac: dest_mac.to_string(),
            bssid: bssid.map(String::from),
            timestamp: now,
        });

        // Check against signatures
        for sig in &self.signatures {
            if !sig.frame_types.contains(&frame_subtype) {
                continue;
            }

            let window_start = now - Duration::seconds(sig.window_seconds as i64);
            let count = records.iter()
                .filter(|r| r.timestamp >= window_start)
                .count() as u32;

            if count >= sig.threshold_count {
                // Check for broadcast deauth specifically
                let is_broadcast = dest_mac == "ff:ff:ff:ff:ff:ff";
                
                let attack = DetectedAttack {
                    signature_name: sig.name.to_string(),
                    source_mac: source_mac.to_string(),
                    target_mac: if is_broadcast { None } else { Some(dest_mac.to_string()) },
                    bssid: bssid.map(String::from),
                    frame_count: count,
                    severity: if is_broadcast && sig.name == "DEAUTH_FLOOD" {
                        SignatureSeverity::Critical
                    } else {
                        sig.severity
                    },
                    first_seen: records.first().map(|r| r.timestamp).unwrap_or(now),
                    last_seen: now,
                    description: if is_broadcast {
                        format!("{} (BROADCAST - all clients targeted)", sig.description)
                    } else {
                        sig.description.to_string()
                    },
                };

                // Don't spam alerts - check if we recently detected this
                let recent_duplicate = self.detected_attacks.iter().any(|a| {
                    a.signature_name == attack.signature_name &&
                    a.source_mac == attack.source_mac &&
                    now.signed_duration_since(a.last_seen) < Duration::seconds(60)
                });

                if !recent_duplicate {
                    self.detected_attacks.push(attack.clone());
                    return Some(attack);
                }
            }
        }

        // Cleanup old records
        self.cleanup_old_records(now);

        None
    }

    fn cleanup_old_records(&mut self, now: DateTime<Utc>) {
        let max_age = Duration::minutes(5);
        
        for records in self.frame_counts.values_mut() {
            records.retain(|r| now.signed_duration_since(r.timestamp) < max_age);
        }
        
        self.frame_counts.retain(|_, v| !v.is_empty());
        
        // Also cleanup old detected attacks
        self.detected_attacks.retain(|a| {
            now.signed_duration_since(a.last_seen) < Duration::hours(1)
        });
    }

    pub fn get_recent_attacks(&self, minutes: i64) -> Vec<&DetectedAttack> {
        let cutoff = Utc::now() - Duration::minutes(minutes);
        self.detected_attacks
            .iter()
            .filter(|a| a.last_seen >= cutoff)
            .collect()
    }

    pub fn add_custom_signature(&mut self, signature: AttackSignature) {
        self.signatures.push(signature);
    }
}

impl Default for SignatureDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Known malicious OUIs and device patterns
pub struct ThreatIntel {
    malicious_ouis: HashMap<String, &'static str>,
    suspicious_ssids: Vec<&'static str>,
    known_attack_tools: Vec<&'static str>,
}

impl ThreatIntel {
    pub fn new() -> Self {
        let mut malicious_ouis = HashMap::new();
        
        // Common WiFi hacking device OUIs
        malicious_ouis.insert("00:C0:CA".to_string(), "Alfa Inc (common pentesting adapter)");
        malicious_ouis.insert("00:0F:00".to_string(), "Qualcomm Atheros (common in hacking tools)");
        
        // Pineapple and similar devices often use random MACs but may have patterns
        
        let suspicious_ssids = vec![
            "Free WiFi",
            "Free Internet",
            "Open WiFi",
            "Hotel WiFi",
            "Airport WiFi",
            "Starbucks WiFi",
            "xfinitywifi",
            "attwifi",
            "Google Starbucks",
        ];

        let known_attack_tools = vec![
            "WiFi-Pineapple",
            "Karma",
            "Mana",
            "Aircrack",
            "MDK3",
            "MDK4",
            "Wifiphisher",
        ];

        Self {
            malicious_ouis,
            suspicious_ssids,
            known_attack_tools,
        }
    }

    pub fn check_oui(&self, mac: &str) -> Option<&'static str> {
        let oui = mac.to_uppercase().chars().take(8).collect::<String>();
        self.malicious_ouis.get(&oui).copied()
    }

    pub fn is_suspicious_ssid(&self, ssid: &str) -> bool {
        let ssid_lower = ssid.to_lowercase();
        self.suspicious_ssids.iter().any(|s| {
            ssid_lower.contains(&s.to_lowercase())
        })
    }

    pub fn check_evil_twin(&self, ssid: &str, known_bssids: &[String], new_bssid: &str) -> bool {
        // If SSID matches a known network but BSSID is different
        !known_bssids.contains(&new_bssid.to_string())
    }
}

impl Default for ThreatIntel {
    fn default() -> Self {
        Self::new()
    }
}
