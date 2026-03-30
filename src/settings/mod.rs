//! Settings management module
//!
//! Provides runtime settings management, persistence, and API for settings UI.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// All application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub wifi: WifiSettings,
    pub bluetooth: BluetoothSettings,
    pub gps: GpsSettings,
    pub alerts: AlertSettings,
    pub power: PowerSettings,
    pub privacy: PrivacySettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            wifi: WifiSettings::default(),
            bluetooth: BluetoothSettings::default(),
            gps: GpsSettings::default(),
            alerts: AlertSettings::default(),
            power: PowerSettings::default(),
            privacy: PrivacySettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub device_name: String,
    pub device_id: String,
    pub data_dir: PathBuf,
    pub log_level: String,
    pub web_port: u16,
    pub auto_start_scanning: bool,
    pub simulation_mode: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            device_name: "SIGINT-Pi".to_string(),
            device_id: uuid::Uuid::new_v4().to_string(),
            data_dir: PathBuf::from("/data"),
            log_level: "info".to_string(),
            web_port: 8080,
            auto_start_scanning: true,
            simulation_mode: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiSettings {
    pub enabled: bool,
    pub interface: String,
    pub auto_monitor_mode: bool,
    pub channels: Vec<u8>,
    pub channel_hop: bool,
    pub channel_hop_interval_ms: u64,
    pub capture_pcap: bool,
    pub pcap_dir: PathBuf,
    pub pcap_rotate_mb: u32,
    pub attack_detection: bool,
}

impl Default for WifiSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interface: "wlan1".to_string(),
            auto_monitor_mode: false,
            channels: vec![1, 6, 11],
            channel_hop: true,
            channel_hop_interval_ms: 500,
            capture_pcap: false,
            pcap_dir: PathBuf::from("/data/pcap"),
            pcap_rotate_mb: 100,
            attack_detection: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothSettings {
    pub enabled: bool,
    pub scan_interval_ms: u64,
    pub detect_trackers: bool,
    pub tracker_alert: bool,
    pub classify_devices: bool,
}

impl Default for BluetoothSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_interval_ms: 5000,
            detect_trackers: true,
            tracker_alert: true,
            classify_devices: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsSettings {
    pub enabled: bool,
    pub gpsd_host: String,
    pub gpsd_port: u16,
    pub geofencing: bool,
    pub home_lat: Option<f64>,
    pub home_lon: Option<f64>,
    pub home_radius_m: f64,
    pub log_tracks: bool,
}

impl Default for GpsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            gpsd_host: "localhost".to_string(),
            gpsd_port: 2947,
            geofencing: false,
            home_lat: None,
            home_lon: None,
            home_radius_m: 100.0,
            log_tracks: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub sound: SoundAlertSettings,
    pub telegram: TelegramAlertSettings,
    pub signal: SignalAlertSettings,
    pub email: EmailAlertSettings,
    pub mqtt: MqttAlertSettings,
    pub webhook: WebhookAlertSettings,
    pub openclaw: OpenClawAlertSettings,
}

impl Default for AlertSettings {
    fn default() -> Self {
        Self {
            sound: SoundAlertSettings::default(),
            telegram: TelegramAlertSettings::default(),
            signal: SignalAlertSettings::default(),
            email: EmailAlertSettings::default(),
            mqtt: MqttAlertSettings::default(),
            webhook: WebhookAlertSettings::default(),
            openclaw: OpenClawAlertSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundAlertSettings {
    pub enabled: bool,
    pub ninja_mode: bool,
    pub volume: u8,
    pub new_device: bool,
    pub tracker: bool,
    pub attack: bool,
    pub geofence: bool,
}

impl Default for SoundAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            ninja_mode: false,
            volume: 70,
            new_device: true,
            tracker: true,
            attack: true,
            geofence: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramAlertSettings {
    pub enabled: bool,
    pub bot_token: String,
    pub chat_id: String,
    pub min_priority: String,
}

impl Default for TelegramAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            bot_token: String::new(),
            chat_id: String::new(),
            min_priority: "medium".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalAlertSettings {
    pub enabled: bool,
    pub sender_number: String,
    pub recipients: Vec<String>,
    pub signal_cli_path: String,
    pub min_priority: String,
}

impl Default for SignalAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            sender_number: String::new(),
            recipients: Vec::new(),
            signal_cli_path: "signal-cli".to_string(),
            min_priority: "high".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAlertSettings {
    pub enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub from_addr: String,
    pub to_addrs: Vec<String>,
    pub min_priority: String,
}

impl Default for EmailAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_host: String::new(),
            smtp_port: 587,
            smtp_user: String::new(),
            smtp_pass: String::new(),
            from_addr: String::new(),
            to_addrs: Vec::new(),
            min_priority: "high".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttAlertSettings {
    pub enabled: bool,
    pub broker_url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub topic_prefix: String,
    pub client_id: String,
}

impl Default for MqttAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            broker_url: "mqtt://localhost:1883".to_string(),
            username: None,
            password: None,
            topic_prefix: "sigint-pi".to_string(),
            client_id: "sigint-pi".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAlertSettings {
    pub enabled: bool,
    pub endpoints: Vec<WebhookEndpointConfig>,
}

impl Default for WebhookAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpointConfig {
    pub name: String,
    pub url: String,
    pub auth_header: Option<String>,
    pub auth_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawAlertSettings {
    pub enabled: bool,
    pub api_url: String,
    pub api_key: String,
    pub device_id: String,
    pub device_name: String,
    pub tags: Vec<String>,
    pub min_priority: String,
}

impl Default for OpenClawAlertSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            api_url: "https://api.openclaw.io/v1/alerts".to_string(),
            api_key: String::new(),
            device_id: String::new(),
            device_name: "SIGINT-Pi".to_string(),
            tags: vec!["sigint".to_string()],
            min_priority: "medium".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSettings {
    pub mode: String,
    pub auto_low_power_percent: Option<u8>,
    pub sleep_on_ac: bool,
}

impl Default for PowerSettings {
    fn default() -> Self {
        Self {
            mode: "balanced".to_string(),
            auto_low_power_percent: Some(20),
            sleep_on_ac: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    pub anonymize_macs: bool,
    pub retention_days: u32,
    pub exclude_macs: Vec<String>,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            anonymize_macs: false,
            retention_days: 30,
            exclude_macs: Vec::new(),
        }
    }
}

/// Settings manager for runtime access and persistence
pub struct SettingsManager {
    settings: Arc<RwLock<AppSettings>>,
    settings_file: PathBuf,
}

impl SettingsManager {
    /// Create a new settings manager
    pub fn new(settings_file: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(AppSettings::default())),
            settings_file,
        }
    }

    /// Load settings from file
    pub async fn load(&self) -> Result<(), String> {
        if !self.settings_file.exists() {
            info!("No settings file found, using defaults");
            return Ok(());
        }

        let content = tokio::fs::read_to_string(&self.settings_file)
            .await
            .map_err(|e| format!("Failed to read settings: {}", e))?;

        let loaded: AppSettings = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse settings: {}", e))?;

        *self.settings.write().await = loaded;
        info!("Settings loaded from {:?}", self.settings_file);
        Ok(())
    }

    /// Save settings to file
    pub async fn save(&self) -> Result<(), String> {
        let settings = self.settings.read().await;
        let content = toml::to_string_pretty(&*settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;

        if let Some(parent) = self.settings_file.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create settings dir: {}", e))?;
        }

        tokio::fs::write(&self.settings_file, content)
            .await
            .map_err(|e| format!("Failed to write settings: {}", e))?;

        info!("Settings saved to {:?}", self.settings_file);
        Ok(())
    }

    /// Get all settings
    pub async fn get_all(&self) -> AppSettings {
        self.settings.read().await.clone()
    }

    /// Update settings
    pub async fn update(&self, settings: AppSettings) -> Result<(), String> {
        *self.settings.write().await = settings;
        self.save().await
    }

    /// Get a specific setting section as JSON
    pub async fn get_section(&self, section: &str) -> Option<serde_json::Value> {
        let settings = self.settings.read().await;
        match section {
            "general" => serde_json::to_value(&settings.general).ok(),
            "wifi" => serde_json::to_value(&settings.wifi).ok(),
            "bluetooth" => serde_json::to_value(&settings.bluetooth).ok(),
            "gps" => serde_json::to_value(&settings.gps).ok(),
            "alerts" => serde_json::to_value(&settings.alerts).ok(),
            "power" => serde_json::to_value(&settings.power).ok(),
            "privacy" => serde_json::to_value(&settings.privacy).ok(),
            _ => None,
        }
    }

    /// Toggle ninja mode
    pub async fn set_ninja_mode(&self, enabled: bool) {
        self.settings.write().await.alerts.sound.ninja_mode = enabled;
    }

    /// Check if ninja mode is active
    pub async fn is_ninja_mode(&self) -> bool {
        self.settings.read().await.alerts.sound.ninja_mode
    }
}

/// About/system information for the about page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub build_date: String,
    pub rust_version: String,
    pub platform: String,
    pub device_id: String,
    pub device_name: String,
    pub uptime_secs: u64,
    pub settings_summary: SettingsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSummary {
    pub wifi_enabled: bool,
    pub wifi_interface: String,
    pub bluetooth_enabled: bool,
    pub gps_enabled: bool,
    pub attack_detection: bool,
    pub pcap_capture: bool,
    pub sound_alerts: bool,
    pub ninja_mode: bool,
    pub telegram_enabled: bool,
    pub signal_enabled: bool,
    pub email_enabled: bool,
    pub mqtt_enabled: bool,
    pub openclaw_enabled: bool,
    pub webhook_count: usize,
    pub power_mode: String,
    pub simulation_mode: bool,
}

impl SettingsSummary {
    pub fn from_settings(settings: &AppSettings) -> Self {
        Self {
            wifi_enabled: settings.wifi.enabled,
            wifi_interface: settings.wifi.interface.clone(),
            bluetooth_enabled: settings.bluetooth.enabled,
            gps_enabled: settings.gps.enabled,
            attack_detection: settings.wifi.attack_detection,
            pcap_capture: settings.wifi.capture_pcap,
            sound_alerts: settings.alerts.sound.enabled,
            ninja_mode: settings.alerts.sound.ninja_mode,
            telegram_enabled: settings.alerts.telegram.enabled,
            signal_enabled: settings.alerts.signal.enabled,
            email_enabled: settings.alerts.email.enabled,
            mqtt_enabled: settings.alerts.mqtt.enabled,
            openclaw_enabled: settings.alerts.openclaw.enabled,
            webhook_count: settings.alerts.webhook.endpoints.len(),
            power_mode: settings.power.mode.clone(),
            simulation_mode: settings.general.simulation_mode,
        }
    }
}

/// Legal disclaimer
pub const LEGAL_DISCLAIMER: &str = r#"
================================================================================
                            LEGAL DISCLAIMER
================================================================================

SIGINT-Pi is a security research and educational tool. By using this software,
you acknowledge and agree to the following:

1. LEGAL COMPLIANCE
   - You are solely responsible for ensuring your use of this tool complies
     with all applicable local, state, federal, and international laws.
   - Monitoring wireless communications without authorization may be illegal
     in your jurisdiction.
   - The legality of passive WiFi monitoring varies by country and region.

2. AUTHORIZED USE ONLY
   - Only use this tool on networks and devices you own or have explicit
     written permission to monitor.
   - Unauthorized interception of communications is illegal in most
     jurisdictions and may result in criminal prosecution.

3. PROHIBITED USES
   - Do NOT use this tool to intercept, capture, or analyze communications
     of third parties without their consent.
   - Do NOT use this tool for stalking, harassment, or invasion of privacy.
   - Do NOT use this tool to conduct attacks on networks or devices.
   - Do NOT use this tool for any illegal or unethical purposes.

4. HARDWARE REQUIREMENTS
   - WiFi monitor mode requires an external USB WiFi adapter that supports
     monitor mode (e.g., Alfa AWUS036ACHM).
   - The Steam Deck's internal WiFi adapter does NOT support monitor mode.
   - GPS functionality requires an external USB GPS receiver.
   - For portable operation, use a powered USB hub to ensure stable power
     delivery to all connected devices.

5. NO WARRANTY
   - This software is provided "AS IS" without warranty of any kind.
   - The authors are not responsible for any damages or legal issues arising
     from the use of this software.

6. EDUCATIONAL PURPOSE
   - This tool is intended for security research, penetration testing with
     authorization, and educational purposes only.

By proceeding, you confirm that you have read, understood, and agree to
comply with all applicable laws and this disclaimer.

================================================================================
"#;
