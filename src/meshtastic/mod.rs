use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::config::MeshtasticConfig;
use crate::openclaw::{CompactThreatMessage, OpenClawMessage, ThreatLevel};
use crate::ScanEvent;

/// Meshtastic message for sending over LoRa mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshtasticMessage {
    /// Channel index (0-7)
    pub channel: u8,
    /// Message payload (text or binary)
    pub payload: String,
    /// Want acknowledgment
    pub want_ack: bool,
}

/// Connection types for Meshtastic
pub enum MeshtasticConnection {
    /// Serial/USB connection to Meshtastic device
    Serial(String),
    /// TCP connection to Meshtastic device
    Tcp(String, u16),
    /// MQTT bridge (most common for SIGINT use)
    Mqtt(MqttBridge),
}

/// MQTT bridge for Meshtastic
pub struct MqttBridge {
    broker: String,
    topic_prefix: String,
    client: Option<rumqttc::AsyncClient>,
}

pub struct MeshtasticClient {
    config: MeshtasticConfig,
    mqtt_client: Option<rumqttc::AsyncClient>,
}

impl MeshtasticClient {
    pub fn new(config: MeshtasticConfig) -> Self {
        Self {
            config,
            mqtt_client: None,
        }
    }

    /// Connect to Meshtastic via configured method
    pub async fn connect(&mut self) -> Result<()> {
        match self.config.connection_type.as_str() {
            "mqtt" => self.connect_mqtt().await,
            "serial" => self.connect_serial().await,
            "tcp" => self.connect_tcp().await,
            _ => {
                warn!("Unknown Meshtastic connection type: {}", self.config.connection_type);
                Ok(())
            }
        }
    }

    async fn connect_mqtt(&mut self) -> Result<()> {
        let broker = self.config.mqtt_broker.as_ref()
            .ok_or_else(|| anyhow::anyhow!("MQTT broker not configured"))?;

        info!("Connecting to Meshtastic MQTT broker: {}", broker);

        // Parse broker URL
        let (host, port) = if broker.contains(':') {
            let parts: Vec<&str> = broker.split(':').collect();
            (parts[0].to_string(), parts[1].parse().unwrap_or(1883))
        } else {
            (broker.clone(), 1883)
        };

        // Create MQTT client
        let mut mqtt_options = rumqttc::MqttOptions::new(
            format!("sigint-{}", std::process::id()),
            host,
            port,
        );
        mqtt_options.set_keep_alive(Duration::from_secs(30));

        let (client, mut eventloop) = rumqttc::AsyncClient::new(mqtt_options, 10);
        
        // Subscribe to receive messages
        let topic = format!("{}/+/json/rx", self.config.mqtt_topic);
        client.subscribe(&topic, rumqttc::QoS::AtLeastOnce).await?;
        
        self.mqtt_client = Some(client);
        
        info!("Connected to Meshtastic MQTT, subscribed to: {}", topic);
        
        // Start event loop in background
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(event) => {
                        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(msg)) = event {
                            debug!("Meshtastic MQTT message: {:?}", msg.topic);
                        }
                    }
                    Err(e) => {
                        error!("Meshtastic MQTT error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        Ok(())
    }

    async fn connect_serial(&mut self) -> Result<()> {
        let port = self.config.serial_port.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Serial port not configured"))?;

        info!("Connecting to Meshtastic serial: {}", port);
        
        // For serial connection, we'd use the meshtastic-rs crate
        // For now, just log that it's not yet implemented
        warn!("Meshtastic serial connection not yet implemented");
        
        Ok(())
    }

    async fn connect_tcp(&mut self) -> Result<()> {
        let host = self.config.tcp_host.as_ref()
            .ok_or_else(|| anyhow::anyhow!("TCP host not configured"))?;

        info!("Connecting to Meshtastic TCP: {}:{}", host, self.config.tcp_port);
        
        // For TCP connection, we'd use the meshtastic protocol
        // For now, just log that it's not yet implemented
        warn!("Meshtastic TCP connection not yet implemented");
        
        Ok(())
    }

    /// Send a threat message over Meshtastic
    pub async fn send_threat(&self, message: &OpenClawMessage) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Check threat level
        let msg_level = ThreatLevel::from_str(&message.threat_level);
        let min_level = ThreatLevel::from_str(&self.config.min_threat_level);
        
        if msg_level < min_level {
            debug!("Skipping Meshtastic: {} < {}", message.threat_level, self.config.min_threat_level);
            return Ok(());
        }

        // Create message (compact or full)
        let payload = if self.config.compact_messages {
            let compact = CompactThreatMessage::from_openclaw(message);
            serde_json::to_string(&compact)?
        } else {
            // Shortened full message for LoRa
            format!(
                "🚨 {} {} {}",
                message.threat_level.to_uppercase(),
                message.category.as_deref().unwrap_or("UNKNOWN"),
                message.mac_address.as_deref().unwrap_or("")
            )
        };

        self.send_message(&payload).await
    }

    /// Send raw message over Meshtastic
    pub async fn send_message(&self, payload: &str) -> Result<()> {
        match &self.mqtt_client {
            Some(client) => {
                let topic = format!("{}/tx", self.config.mqtt_topic);
                
                // Meshtastic MQTT format
                let msg = serde_json::json!({
                    "channel": self.config.channel,
                    "payload": payload,
                    "type": "text"
                });

                client.publish(
                    &topic,
                    rumqttc::QoS::AtLeastOnce,
                    false,
                    serde_json::to_vec(&msg)?,
                ).await?;

                info!("Sent Meshtastic message: {} bytes", payload.len());
            }
            None => {
                debug!("Meshtastic not connected, skipping message");
            }
        }

        Ok(())
    }

    /// Send a compact IMSI catcher alert
    pub async fn send_imsi_alert(&self, confidence: f64, cell_id: Option<u32>) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let payload = if self.config.compact_messages {
            format!("!IMSI {:.0}% CID:{}", 
                    confidence * 100.0,
                    cell_id.map(|c| c.to_string()).unwrap_or_else(|| "?".to_string()))
        } else {
            format!("🚨 IMSI CATCHER DETECTED\nConfidence: {:.1}%\nCell ID: {}",
                    confidence * 100.0,
                    cell_id.map(|c| c.to_string()).unwrap_or_else(|| "Unknown".to_string()))
        };

        self.send_message(&payload).await
    }
}

/// Preset configurations for common Meshtastic setups
pub struct MeshtasticPresets;

impl MeshtasticPresets {
    /// Local mesh network (direct serial/TCP to your Meshtastic device)
    pub fn local_device(serial_port: &str) -> MeshtasticConfig {
        MeshtasticConfig {
            enabled: true,
            connection_type: "serial".to_string(),
            serial_port: Some(serial_port.to_string()),
            tcp_host: None,
            tcp_port: 4403,
            mqtt_broker: None,
            mqtt_topic: "msh/US/sigint".to_string(),
            min_threat_level: "high".to_string(),
            compact_messages: true,
            channel: 0,
        }
    }

    /// Public MQTT bridge (uses meshtastic.org MQTT server)
    pub fn public_mqtt() -> MeshtasticConfig {
        MeshtasticConfig {
            enabled: true,
            connection_type: "mqtt".to_string(),
            serial_port: None,
            tcp_host: None,
            tcp_port: 4403,
            mqtt_broker: Some("mqtt.meshtastic.org".to_string()),
            mqtt_topic: "msh/US/2/json".to_string(),
            min_threat_level: "high".to_string(),
            compact_messages: true,
            channel: 0,
        }
    }

    /// Private MQTT broker (your own MQTT server)
    pub fn private_mqtt(broker: &str, topic: &str) -> MeshtasticConfig {
        MeshtasticConfig {
            enabled: true,
            connection_type: "mqtt".to_string(),
            serial_port: None,
            tcp_host: None,
            tcp_port: 4403,
            mqtt_broker: Some(broker.to_string()),
            mqtt_topic: topic.to_string(),
            min_threat_level: "high".to_string(),
            compact_messages: true,
            channel: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_imsi_alert() {
        let payload = format!("!IMSI {:.0}% CID:{}", 85.5, 12345);
        assert!(payload.len() < 50); // Should be short for LoRa
    }
}
