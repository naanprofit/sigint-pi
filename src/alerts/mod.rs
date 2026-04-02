mod telegram;
mod twilio;
mod email;
mod mqtt;
pub mod sound;
pub mod signal;
pub mod webhook;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Globally silenced device MACs - alerts suppressed for these
pub static SILENCED_DEVICES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn is_device_silenced(mac: &str) -> bool {
    SILENCED_DEVICES.lock().unwrap().contains(&mac.to_uppercase())
}

pub fn silence_device(mac: &str) {
    SILENCED_DEVICES.lock().unwrap().insert(mac.to_uppercase());
}

pub fn unsilence_device(mac: &str) {
    SILENCED_DEVICES.lock().unwrap().remove(&mac.to_uppercase());
}

pub fn get_silenced_devices() -> Vec<String> {
    SILENCED_DEVICES.lock().unwrap().iter().cloned().collect()
}

use crate::config::Config;
use crate::storage::Database;
use crate::wifi::{AttackEvent, WifiDevice};
use crate::bluetooth::BleDevice;
use crate::learning::AnomalyScore;
use crate::ScanEvent;

pub use telegram::TelegramAlert;
pub use twilio::TwilioAlert;
pub use email::EmailAlert;
pub use mqtt::MqttAlert;
pub use sound::{SoundPlayer, SoundEffect, SoundConfig};
pub use signal::{SignalClient, SignalConfig};
pub use webhook::{WebhookClient, WebhookConfig, OpenClawConfig, AlertData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub priority: AlertPriority,
    pub alert_type: AlertType,
    pub title: String,
    pub message: String,
    pub device_mac: Option<String>,
    pub device_vendor: Option<String>,
    pub rssi: Option<i32>,
    pub location: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    NewDevice,
    StrongSignalDevice,
    AttackDetected,
    TrackerDetected,
    AnomalousBehavior,
    SystemAlert,
    ImsiCatcher,
    DroneDetected,
    TscmThreat,
    RfAnomaly,
    SurveillanceDevice,
    GeofenceBreach,
}

pub struct AlertManager {
    db: Arc<Database>,
    config: Arc<Config>,
    telegram: Option<TelegramAlert>,
    twilio: Option<TwilioAlert>,
    email: Option<EmailAlert>,
    mqtt: Option<MqttAlert>,
    recent_alerts: std::collections::HashMap<String, DateTime<Utc>>,
    cooldown_seconds: i64,
}

impl AlertManager {
    pub async fn new(db: Arc<Database>, config: Arc<Config>) -> Self {
        let telegram = if config.alerts.telegram.enabled {
            Some(TelegramAlert::new(&config.alerts.telegram))
        } else {
            None
        };

        let twilio = if config.alerts.twilio.enabled {
            Some(TwilioAlert::new(&config.alerts.twilio))
        } else {
            None
        };

        let email = if config.alerts.email.enabled {
            EmailAlert::new(&config.alerts.email).ok()
        } else {
            None
        };

        let mqtt = if config.alerts.mqtt.enabled {
            MqttAlert::new(&config.alerts.mqtt).await.ok()
        } else {
            None
        };

        Self {
            db,
            config,
            telegram,
            twilio,
            email,
            mqtt,
            recent_alerts: std::collections::HashMap::new(),
            cooldown_seconds: 300, // 5 minute cooldown per device
        }
    }

    pub async fn run(&self, rx: &mut broadcast::Receiver<ScanEvent>) -> Result<()> {
        let mut manager = AlertManagerState {
            db: self.db.clone(),
            config: self.config.clone(),
            telegram: self.telegram.clone(),
            twilio: self.twilio.clone(),
            email: self.email.clone(),
            mqtt: self.mqtt.clone(),
            recent_alerts: std::collections::HashMap::new(),
            cooldown_seconds: self.cooldown_seconds,
        };

        loop {
            match rx.recv().await {
                Ok(event) => {
                    if let Err(e) = manager.process_event(event).await {
                        warn!("Alert processing error: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    warn!("Alert manager lagged {} events", n);
                }
                Err(broadcast::error::RecvError::Closed) => {
                    info!("Event channel closed, stopping alert manager");
                    break;
                }
            }
        }

        Ok(())
    }
}

struct AlertManagerState {
    db: Arc<Database>,
    config: Arc<Config>,
    telegram: Option<TelegramAlert>,
    twilio: Option<TwilioAlert>,
    email: Option<EmailAlert>,
    mqtt: Option<MqttAlert>,
    recent_alerts: std::collections::HashMap<String, DateTime<Utc>>,
    cooldown_seconds: i64,
}

impl AlertManagerState {
    async fn process_event(&mut self, event: ScanEvent) -> Result<()> {
        match event {
            ScanEvent::WifiDevice(device) => {
                self.check_wifi_device(&device).await?;
            }
            ScanEvent::BleDevice(device) => {
                self.check_ble_device(&device).await?;
            }
            ScanEvent::Attack(attack) => {
                self.handle_attack(&attack).await?;
            }
            ScanEvent::GpsUpdate(_) => {
                // GPS updates don't generate alerts directly
            }
            ScanEvent::Alert { .. } => {
                // Alert events are generated by this manager, don't process them
            }
            ScanEvent::RayHunterUpdate(_) => {
                // RayHunter updates handled separately
            }
            ScanEvent::RayHunterAlert(alert) => {
                // Forward RayHunter IMSI catcher alerts
                let alert_msg = Alert {
                    id: uuid::Uuid::new_v4().to_string(),
                    priority: AlertPriority::Critical,
                    alert_type: AlertType::ImsiCatcher,
                    title: "IMSI Catcher Detected".to_string(),
                    message: alert.message.clone(),
                    device_mac: None,
                    device_vendor: None,
                    rssi: None,
                    location: None,
                    timestamp: chrono::Utc::now(),
                };
                self.send_alert(&alert_msg).await?;
            }
            ScanEvent::OpenClawReceived(threat) => {
                // Log received threats from other nodes
                tracing::info!("Received threat from {}: {}", threat.source, threat.message.message);
            }
        }
        Ok(())
    }

    async fn check_wifi_device(&mut self, device: &WifiDevice) -> Result<()> {
        // Check if device is silenced
        if is_device_silenced(&device.mac_address) {
            return Ok(());
        }
        // Check if device is known
        let is_known = self.db.is_device_known(&device.mac_address, 1).await?;
        
        if !is_known {
            // New device - determine alert priority based on signal strength
            let priority = if device.rssi > -50 {
                AlertPriority::High
            } else if device.rssi > -60 {
                AlertPriority::Medium
            } else {
                AlertPriority::Low
            };

            let alert = Alert {
                id: uuid::Uuid::new_v4().to_string(),
                priority,
                alert_type: if device.rssi > -50 {
                    AlertType::StrongSignalDevice
                } else {
                    AlertType::NewDevice
                },
                title: format!("New WiFi Device: {}", device.mac_address),
                message: format!(
                    "New device detected\nMAC: {}\nVendor: {}\nSSID: {}\nRSSI: {} dBm\nChannel: {}",
                    device.mac_address,
                    device.vendor.as_deref().unwrap_or("Unknown"),
                    device.ssid.as_deref().unwrap_or("N/A"),
                    device.rssi,
                    device.channel
                ),
                device_mac: Some(device.mac_address.clone()),
                device_vendor: device.vendor.clone(),
                rssi: Some(device.rssi),
                location: Some(self.config.device.location_name.clone()),
                timestamp: Utc::now(),
            };

            self.send_alert(&alert).await?;
        }

        Ok(())
    }

    async fn check_ble_device(&mut self, device: &BleDevice) -> Result<()> {
        // Check if device is silenced
        if is_device_silenced(&device.mac_address) {
            return Ok(());
        }
        // Always alert on trackers
        if device.is_tracker() {
            let alert = Alert {
                id: uuid::Uuid::new_v4().to_string(),
                priority: AlertPriority::High,
                alert_type: AlertType::TrackerDetected,
                title: format!("Tracker Detected: {:?}", device.device_type),
                message: format!(
                    "Tracking device detected!\nType: {:?}\nMAC: {}\nRSSI: {} dBm",
                    device.device_type,
                    device.mac_address,
                    device.rssi
                ),
                device_mac: Some(device.mac_address.clone()),
                device_vendor: device.vendor.clone(),
                rssi: Some(device.rssi),
                location: Some(self.config.device.location_name.clone()),
                timestamp: Utc::now(),
            };

            self.send_alert(&alert).await?;
            return Ok(());
        }

        // Check if device is known
        let is_known = self.db.is_device_known(&device.mac_address, 1).await?;
        
        if !is_known && device.rssi > -60 {
            let alert = Alert {
                id: uuid::Uuid::new_v4().to_string(),
                priority: AlertPriority::Medium,
                alert_type: AlertType::NewDevice,
                title: format!("New BLE Device: {}", device.name.as_deref().unwrap_or(&device.mac_address)),
                message: format!(
                    "New BLE device detected\nMAC: {}\nName: {}\nType: {:?}\nRSSI: {} dBm",
                    device.mac_address,
                    device.name.as_deref().unwrap_or("Unknown"),
                    device.device_type,
                    device.rssi
                ),
                device_mac: Some(device.mac_address.clone()),
                device_vendor: device.vendor.clone(),
                rssi: Some(device.rssi),
                location: Some(self.config.device.location_name.clone()),
                timestamp: Utc::now(),
            };

            self.send_alert(&alert).await?;
        }

        Ok(())
    }

    async fn handle_attack(&mut self, attack: &AttackEvent) -> Result<()> {
        let priority = match attack.severity {
            crate::wifi::AttackSeverity::Critical => AlertPriority::Critical,
            crate::wifi::AttackSeverity::High => AlertPriority::High,
            crate::wifi::AttackSeverity::Medium => AlertPriority::Medium,
            crate::wifi::AttackSeverity::Low => AlertPriority::Low,
        };

        let alert = Alert {
            id: uuid::Uuid::new_v4().to_string(),
            priority,
            alert_type: AlertType::AttackDetected,
            title: format!("ATTACK: {:?}", attack.attack_type),
            message: format!(
                "WiFi Attack Detected!\n\nType: {:?}\nSeverity: {:?}\nSource: {}\nTarget: {}\n\n{}",
                attack.attack_type,
                attack.severity,
                attack.source_mac,
                attack.target_mac.as_deref().unwrap_or("N/A"),
                attack.description
            ),
            device_mac: Some(attack.source_mac.clone()),
            device_vendor: None,
            rssi: None,
            location: Some(self.config.device.location_name.clone()),
            timestamp: attack.timestamp,
        };

        self.send_alert(&alert).await?;
        Ok(())
    }

    async fn send_alert(&mut self, alert: &Alert) -> Result<()> {
        // Check cooldown for this device
        if let Some(mac) = &alert.device_mac {
            if let Some(last_alert) = self.recent_alerts.get(mac) {
                let elapsed = Utc::now().signed_duration_since(*last_alert);
                if elapsed.num_seconds() < self.cooldown_seconds {
                    debug!("Alert cooldown active for {}", mac);
                    return Ok(());
                }
            }
            self.recent_alerts.insert(mac.clone(), Utc::now());
        }

        // Log alert to database
        self.db.log_alert(
            &format!("{:?}", alert.alert_type),
            &format!("{:?}", alert.priority),
            &alert.message,
            alert.device_mac.as_deref(),
        ).await?;

        info!("Sending alert: {} (priority: {:?})", alert.title, alert.priority);

        // Route alert based on type AND priority
        // Critical security events go everywhere; lower-priority items go to appropriate channels
        let channels = match (&alert.alert_type, &alert.priority) {
            // IMSI catchers and surveillance: always all channels
            (AlertType::ImsiCatcher, _) | (AlertType::SurveillanceDevice, _) =>
                vec!["telegram", "twilio", "email", "mqtt"],
            // Drone detection: high urgency channels
            (AlertType::DroneDetected, _) =>
                vec!["telegram", "twilio", "mqtt"],
            // TSCM threats: depends on priority
            (AlertType::TscmThreat, AlertPriority::Critical) =>
                vec!["telegram", "twilio", "email", "mqtt"],
            (AlertType::TscmThreat, _) =>
                vec!["telegram", "mqtt"],
            // WiFi attacks: based on priority
            (AlertType::AttackDetected, AlertPriority::Critical) =>
                vec!["telegram", "twilio", "email", "mqtt"],
            (AlertType::AttackDetected, _) =>
                vec!["telegram", "mqtt"],
            // Tracker detection: high urgency
            (AlertType::TrackerDetected, _) =>
                vec!["telegram", "twilio", "mqtt"],
            // RF anomalies: log only unless critical
            (AlertType::RfAnomaly, AlertPriority::Critical) =>
                vec!["telegram", "mqtt"],
            (AlertType::RfAnomaly, _) =>
                vec!["mqtt"],
            // Geofence: notification channels
            (AlertType::GeofenceBreach, _) =>
                vec!["telegram", "mqtt"],
            // Generic priority-based routing
            (_, AlertPriority::Critical) => vec!["telegram", "twilio", "email", "mqtt"],
            (_, AlertPriority::High) => vec!["telegram", "twilio", "mqtt"],
            (_, AlertPriority::Medium) => vec!["telegram", "mqtt"],
            (_, AlertPriority::Low) => vec!["mqtt"],
        };

        for channel in channels {
            match channel {
                "telegram" => {
                    if let Some(ref tg) = self.telegram {
                        if let Err(e) = tg.send(alert).await {
                            warn!("Telegram alert failed: {}", e);
                        }
                    }
                }
                "twilio" => {
                    if let Some(ref tw) = self.twilio {
                        if let Err(e) = tw.send(alert).await {
                            warn!("Twilio alert failed: {}", e);
                        }
                    }
                }
                "email" => {
                    if let Some(ref em) = self.email {
                        if let Err(e) = em.send(alert).await {
                            warn!("Email alert failed: {}", e);
                        }
                    }
                }
                "mqtt" => {
                    if let Some(ref mq) = self.mqtt {
                        if let Err(e) = mq.publish(alert).await {
                            warn!("MQTT alert failed: {}", e);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
