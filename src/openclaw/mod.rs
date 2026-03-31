use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, error, info, warn};

use crate::config::OpenClawConfig;
use crate::rayhunter::RayHunterAlert;
use crate::ScanEvent;

/// Threat levels for relay filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl ThreatLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => ThreatLevel::Critical,
            "high" => ThreatLevel::High,
            "medium" => ThreatLevel::Medium,
            _ => ThreatLevel::Low,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ThreatLevel::Critical => "critical",
            ThreatLevel::High => "high",
            ThreatLevel::Medium => "medium",
            ThreatLevel::Low => "low",
        }
    }
}

/// Message format for OpenClaw relay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawMessage {
    /// Message type: "threat", "device", "rayhunter", "status"
    pub msg_type: String,
    /// Source device identifier
    pub source_device: String,
    /// Threat level
    pub threat_level: String,
    /// Threat category (e.g., "surveillance", "chinese_state", "tracker")
    pub category: Option<String>,
    /// Device MAC address (if applicable)
    pub mac_address: Option<String>,
    /// Vendor name
    pub vendor: Option<String>,
    /// GPS coordinates (if enabled)
    pub location: Option<Location>,
    /// Human-readable message
    pub message: String,
    /// Additional details
    pub details: HashMap<String, serde_json::Value>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Message ID for deduplication
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>,
}

/// Received threat from another OpenClaw node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivedThreat {
    pub source: String,
    pub message: OpenClawMessage,
    pub received_at: DateTime<Utc>,
}

pub struct OpenClawClient {
    config: OpenClawConfig,
    http_client: reqwest::Client,
    device_name: String,
    /// Recently relayed messages (for deduplication)
    recent_messages: Arc<RwLock<Vec<String>>>,
    /// Threats received from peers
    received_threats: Arc<RwLock<Vec<ReceivedThreat>>>,
}

impl OpenClawClient {
    pub fn new(config: OpenClawConfig, device_name: String) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
            device_name,
            recent_messages: Arc::new(RwLock::new(Vec::new())),
            received_threats: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Send a threat alert to OpenClaw webhook
    pub async fn send_alert(&self, message: &OpenClawMessage) -> Result<()> {
        if !self.config.enabled || self.config.webhook_url.is_empty() {
            return Ok(());
        }

        // Check threat level filter
        let msg_level = ThreatLevel::from_str(&message.threat_level);
        let min_level = ThreatLevel::from_str(&self.config.relay_min_level);
        
        if msg_level < min_level {
            debug!("Skipping relay: {} < {}", message.threat_level, self.config.relay_min_level);
            return Ok(());
        }

        // Check category filter
        if !self.config.relay_categories.is_empty() {
            if let Some(cat) = &message.category {
                if !self.config.relay_categories.contains(cat) {
                    debug!("Skipping relay: category {} not in filter", cat);
                    return Ok(());
                }
            }
        }

        // Deduplication check
        {
            let recent = self.recent_messages.read().await;
            if recent.contains(&message.message_id) {
                debug!("Skipping duplicate message: {}", message.message_id);
                return Ok(());
            }
        }

        // Add to recent messages
        {
            let mut recent = self.recent_messages.write().await;
            recent.push(message.message_id.clone());
            // Keep last 100 messages
            if recent.len() > 100 {
                recent.remove(0);
            }
        }

        // Build request
        let mut request = self.http_client
            .post(&self.config.webhook_url)
            .json(message);

        // Add API key if configured
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        // Send
        let response = request.send().await?;
        
        if response.status().is_success() {
            info!("OpenClaw alert sent: {} - {}", message.threat_level, message.msg_type);
        } else {
            warn!("OpenClaw webhook returned {}: {}", 
                  response.status(), 
                  response.text().await.unwrap_or_default());
        }

        Ok(())
    }

    /// Relay threat to all configured peers
    pub async fn relay_to_peers(&self, message: &OpenClawMessage) -> Result<()> {
        if !self.config.relay_enabled || self.config.peers.is_empty() {
            return Ok(());
        }

        for peer in &self.config.peers {
            let peer_url = format!("{}/api/openclaw/receive", peer.trim_end_matches('/'));
            
            let mut request = self.http_client
                .post(&peer_url)
                .json(message);

            if let Some(api_key) = &self.config.api_key {
                request = request.header("Authorization", format!("Bearer {}", api_key));
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        debug!("Relayed to peer: {}", peer);
                    } else {
                        debug!("Peer {} returned {}", peer, response.status());
                    }
                }
                Err(e) => {
                    debug!("Failed to relay to peer {}: {}", peer, e);
                }
            }
        }

        Ok(())
    }

    /// Create a threat message from a device detection
    pub fn create_device_threat(
        &self,
        mac: &str,
        vendor: Option<&str>,
        category: Option<&str>,
        threat_level: &str,
        location: Option<(f64, f64)>,
    ) -> OpenClawMessage {
        let loc = if self.config.include_location {
            location.map(|(lat, lon)| Location {
                latitude: lat,
                longitude: lon,
                accuracy: None,
            })
        } else {
            None
        };

        OpenClawMessage {
            msg_type: "device".to_string(),
            source_device: self.device_name.clone(),
            threat_level: threat_level.to_string(),
            category: category.map(String::from),
            mac_address: Some(mac.to_string()),
            vendor: vendor.map(String::from),
            location: loc,
            message: format!(
                "Threat device detected: {} ({})",
                vendor.unwrap_or("Unknown"),
                mac
            ),
            details: HashMap::new(),
            timestamp: Utc::now(),
            message_id: format!("{}-{}-{}", self.device_name, mac, Utc::now().timestamp()),
        }
    }

    /// Create a RayHunter alert message
    pub fn create_rayhunter_alert(
        &self,
        alert: &RayHunterAlert,
        location: Option<(f64, f64)>,
    ) -> OpenClawMessage {
        let loc = if self.config.include_location {
            location.map(|(lat, lon)| Location {
                latitude: lat,
                longitude: lon,
                accuracy: None,
            })
        } else {
            None
        };

        let mut details = HashMap::new();
        details.insert(
            "confidence".to_string(),
            serde_json::json!(alert.analysis.confidence),
        );
        if let Some(cell) = &alert.analysis.cell_info {
            details.insert("cell_info".to_string(), serde_json::json!(cell));
        }

        OpenClawMessage {
            msg_type: "rayhunter".to_string(),
            source_device: self.device_name.clone(),
            threat_level: "critical".to_string(),
            category: Some("imsi_catcher".to_string()),
            mac_address: None,
            vendor: None,
            location: loc,
            message: alert.message.clone(),
            details,
            timestamp: Utc::now(),
            message_id: format!(
                "{}-rayhunter-{}",
                self.device_name,
                Utc::now().timestamp()
            ),
        }
    }

    /// Process received threat from peer
    pub async fn receive_threat(&self, message: OpenClawMessage) -> Result<()> {
        info!(
            "Received threat from {}: {} - {}",
            message.source_device, message.threat_level, message.msg_type
        );

        let threat = ReceivedThreat {
            source: message.source_device.clone(),
            message,
            received_at: Utc::now(),
        };

        let mut threats = self.received_threats.write().await;
        threats.push(threat);

        // Keep last 1000 received threats
        if threats.len() > 1000 {
            threats.remove(0);
        }

        Ok(())
    }

    /// Get recent received threats
    pub async fn get_received_threats(&self) -> Vec<ReceivedThreat> {
        self.received_threats.read().await.clone()
    }

    /// Start the relay listener (for receiving from peers)
    pub async fn start_relay_listener(&self) -> Result<()> {
        if let Some(port) = self.config.relay_listen_port {
            info!("OpenClaw relay listener starting on port {}", port);
            // The actual HTTP server is handled by the web module
            // This just signals that we're ready to receive
        }
        Ok(())
    }
}

/// Compact message format for bandwidth-limited channels (Meshtastic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactThreatMessage {
    /// 2-char type: "DV" (device), "RH" (rayhunter), "ST" (status)
    pub t: String,
    /// Threat level: 1-4 (low to critical)
    pub l: u8,
    /// Category code (first 2 chars)
    pub c: String,
    /// MAC last 4 chars (if device)
    pub m: Option<String>,
    /// Lat/Lon as integers (multiply by 0.00001 to get degrees)
    pub g: Option<(i32, i32)>,
    /// Unix timestamp (seconds)
    pub ts: i64,
}

impl CompactThreatMessage {
    pub fn from_openclaw(msg: &OpenClawMessage) -> Self {
        let type_code = match msg.msg_type.as_str() {
            "device" => "DV",
            "rayhunter" => "RH",
            _ => "ST",
        };

        let level = match ThreatLevel::from_str(&msg.threat_level) {
            ThreatLevel::Critical => 4,
            ThreatLevel::High => 3,
            ThreatLevel::Medium => 2,
            ThreatLevel::Low => 1,
        };

        let category = msg
            .category
            .as_ref()
            .map(|c| c.chars().take(2).collect::<String>().to_uppercase())
            .unwrap_or_else(|| "UN".to_string());

        let mac_short = msg.mac_address.as_ref().map(|m| {
            m.replace(":", "")
                .chars()
                .rev()
                .take(4)
                .collect::<String>()
                .chars()
                .rev()
                .collect()
        });

        let geo = msg.location.as_ref().map(|loc| {
            (
                (loc.latitude * 100000.0) as i32,
                (loc.longitude * 100000.0) as i32,
            )
        });

        Self {
            t: type_code.to_string(),
            l: level,
            c: category,
            m: mac_short,
            g: geo,
            ts: msg.timestamp.timestamp(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // Minimal binary format for LoRa
        // Could be as small as 15-20 bytes
        serde_json::to_vec(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_level_ordering() {
        assert!(ThreatLevel::Critical > ThreatLevel::High);
        assert!(ThreatLevel::High > ThreatLevel::Medium);
        assert!(ThreatLevel::Medium > ThreatLevel::Low);
    }

    #[test]
    fn test_compact_message() {
        let msg = OpenClawMessage {
            msg_type: "device".to_string(),
            source_device: "test".to_string(),
            threat_level: "high".to_string(),
            category: Some("surveillance".to_string()),
            mac_address: Some("AA:BB:CC:DD:EE:FF".to_string()),
            vendor: Some("Hikvision".to_string()),
            location: Some(Location {
                latitude: 37.7749,
                longitude: -122.4194,
                accuracy: None,
            }),
            message: "Test".to_string(),
            details: HashMap::new(),
            timestamp: Utc::now(),
            message_id: "test-1".to_string(),
        };

        let compact = CompactThreatMessage::from_openclaw(&msg);
        assert_eq!(compact.t, "DV");
        assert_eq!(compact.l, 3);
        assert_eq!(compact.c, "SU");
        assert_eq!(compact.m, Some("EEFF".to_string()));
    }
}
