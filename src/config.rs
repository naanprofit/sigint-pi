use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub device: DeviceConfig,
    pub wifi: WifiConfig,
    pub bluetooth: BluetoothConfig,
    pub gps: GpsConfig,
    pub database: DatabaseConfig,
    pub alerts: AlertsConfig,
    pub web: WebConfig,
    pub learning: LearningConfig,
    pub power: PowerConfig,
    pub influxdb: InfluxConfig,
    #[serde(default)]
    pub llm: Option<LlmConfig>,
}

/// LLM/AI Provider Configuration
/// Supports OpenAI-compatible APIs (OpenAI, llama.cpp, Ollama, LMStudio, etc.)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmConfig {
    pub enabled: bool,
    /// Provider type: "openai", "llamacpp", "ollama", "lmstudio", "custom"
    pub provider: String,
    /// API endpoint URL (e.g., "https://api.openai.com/v1" or "http://192.168.1.100:8080")
    pub endpoint: String,
    /// API key (loaded from config, never hardcoded)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Model name (e.g., "gpt-4", "llama3", "mistral")
    pub model: String,
    /// Max tokens for response
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_max_tokens() -> u32 { 200 }
fn default_timeout() -> u64 { 30 }

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "llamacpp".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            api_key: None,
            model: "default".to_string(),
            max_tokens: 200,
            timeout_secs: 30,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeviceConfig {
    pub name: String,
    pub location_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WifiConfig {
    pub enabled: bool,
    pub interface: String,
    pub scan_interval_ms: u64,
    pub rssi_threshold: i32,
    pub attack_detection: bool,
    pub pcap_enabled: bool,
    pub pcap_path: PathBuf,
    pub pcap_rotate_mb: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BluetoothConfig {
    pub enabled: bool,
    pub scan_interval_ms: u64,
    pub rssi_threshold: i32,
    pub detect_airtags: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GpsConfig {
    pub enabled: bool,
    pub gpsd_host: String,
    pub gpsd_port: u16,
    pub update_interval_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub path: PathBuf,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlertsConfig {
    pub telegram: TelegramConfig,
    pub twilio: TwilioConfig,
    pub email: EmailConfig,
    pub mqtt: MqttConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelegramConfig {
    pub enabled: bool,
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TwilioConfig {
    pub enabled: bool,
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub to_number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailConfig {
    pub enabled: bool,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_address: String,
    pub to_addresses: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MqttConfig {
    pub enabled: bool,
    pub broker_host: String,
    pub broker_port: u16,
    pub client_id: String,
    pub topic_prefix: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InfluxConfig {
    pub enabled: bool,
    pub url: String,
    pub token: String,
    pub org: String,
    pub bucket: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LearningConfig {
    pub enabled: bool,
    pub training_hours: u32,
    pub anomaly_threshold: f64,
    pub geofence_radius_meters: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PowerConfig {
    pub low_power_mode: bool,
    pub battery_scan_interval_ms: u64,
    pub ac_scan_interval_ms: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_paths = vec![
            PathBuf::from("/etc/sigint-pi/config.toml"),
            PathBuf::from("./config.toml"),
            dirs::config_dir()
                .map(|p| p.join("sigint-pi/config.toml"))
                .unwrap_or_default(),
        ];

        for path in &config_paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read config from {:?}", path))?;
                let config: Config = toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config from {:?}", path))?;
                return Ok(config);
            }
        }

        Ok(Self::default())
    }

    pub fn default() -> Self {
        Self {
            device: DeviceConfig {
                name: "sigint-pi-01".to_string(),
                location_name: "default".to_string(),
            },
            wifi: WifiConfig {
                enabled: true,
                interface: "wlan1".to_string(),
                scan_interval_ms: 5000,
                rssi_threshold: -80,
                attack_detection: true,
                pcap_enabled: true,
                pcap_path: PathBuf::from("/var/lib/sigint-pi/pcap"),
                pcap_rotate_mb: 100,
            },
            bluetooth: BluetoothConfig {
                enabled: true,
                scan_interval_ms: 10000,
                rssi_threshold: -90,
                detect_airtags: true,
            },
            gps: GpsConfig {
                enabled: true,
                gpsd_host: "127.0.0.1".to_string(),
                gpsd_port: 2947,
                update_interval_ms: 1000,
            },
            database: DatabaseConfig {
                path: PathBuf::from("/var/lib/sigint-pi/sigint.db"),
                retention_days: 30,
            },
            alerts: AlertsConfig {
                telegram: TelegramConfig {
                    enabled: false,
                    bot_token: String::new(),
                    chat_id: String::new(),
                },
                twilio: TwilioConfig {
                    enabled: false,
                    account_sid: String::new(),
                    auth_token: String::new(),
                    from_number: String::new(),
                    to_number: String::new(),
                },
                email: EmailConfig {
                    enabled: false,
                    smtp_host: String::new(),
                    smtp_port: 587,
                    smtp_user: String::new(),
                    smtp_password: String::new(),
                    from_address: String::new(),
                    to_addresses: vec![],
                },
                mqtt: MqttConfig {
                    enabled: false,
                    broker_host: "localhost".to_string(),
                    broker_port: 1883,
                    client_id: "sigint-pi".to_string(),
                    topic_prefix: "sigint".to_string(),
                    username: None,
                    password: None,
                },
            },
            web: WebConfig {
                enabled: true,
                bind_address: "0.0.0.0".to_string(),
                port: 8080,
            },
            learning: LearningConfig {
                enabled: true,
                training_hours: 24,
                anomaly_threshold: 0.7,
                geofence_radius_meters: 100.0,
            },
            power: PowerConfig {
                low_power_mode: false,
                battery_scan_interval_ms: 15000,
                ac_scan_interval_ms: 5000,
            },
            influxdb: InfluxConfig {
                enabled: false,
                url: "http://localhost:8086".to_string(),
                token: String::new(),
                org: "sigint".to_string(),
                bucket: "sigint".to_string(),
            },
            llm: Some(LlmConfig::default()),
        }
    }
}
