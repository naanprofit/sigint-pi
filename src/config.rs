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
    #[serde(default)]
    pub openclaw: Option<OpenClawConfig>,
    #[serde(default)]
    pub meshtastic: Option<MeshtasticConfig>,
    #[serde(default)]
    pub rayhunter: Option<RayHunterConfig>,
    #[serde(default)]
    pub sdr: Option<SdrConfig>,
}

/// OpenClaw mesh networking for threat sharing
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OpenClawConfig {
    pub enabled: bool,
    /// Webhook URL for sending alerts
    pub webhook_url: String,
    /// API key for authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Enable relay mode - share threats with other nodes
    #[serde(default)]
    pub relay_enabled: bool,
    /// Minimum threat level to relay: "low", "medium", "high", "critical"
    #[serde(default = "default_relay_level")]
    pub relay_min_level: String,
    /// Relay endpoint for receiving threats from other nodes
    #[serde(default)]
    pub relay_listen_port: Option<u16>,
    /// List of peer nodes to sync with
    #[serde(default)]
    pub peers: Vec<String>,
    /// Include GPS coordinates in relayed messages
    #[serde(default)]
    pub include_location: bool,
    /// Device categories to relay (empty = all)
    #[serde(default)]
    pub relay_categories: Vec<String>,
}

fn default_relay_level() -> String { "high".to_string() }

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            webhook_url: String::new(),
            api_key: None,
            relay_enabled: false,
            relay_min_level: "high".to_string(),
            relay_listen_port: Some(8081),
            peers: Vec::new(),
            include_location: false,
            relay_categories: Vec::new(),
        }
    }
}

/// Meshtastic LoRa mesh network integration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MeshtasticConfig {
    pub enabled: bool,
    /// Connection type: "serial", "tcp", "mqtt"
    pub connection_type: String,
    /// Serial port for USB connection (e.g., "/dev/ttyUSB0")
    #[serde(default)]
    pub serial_port: Option<String>,
    /// TCP host for network connection
    #[serde(default)]
    pub tcp_host: Option<String>,
    /// TCP port (default 4403)
    #[serde(default = "default_meshtastic_port")]
    pub tcp_port: u16,
    /// MQTT broker for Meshtastic MQTT mode
    #[serde(default)]
    pub mqtt_broker: Option<String>,
    /// MQTT topic prefix
    #[serde(default = "default_meshtastic_topic")]
    pub mqtt_topic: String,
    /// Minimum threat level to send over mesh
    #[serde(default = "default_relay_level")]
    pub min_threat_level: String,
    /// Use compact message format (saves bandwidth)
    #[serde(default = "default_true")]
    pub compact_messages: bool,
    /// Channel to send on (0-7)
    #[serde(default)]
    pub channel: u8,
}

fn default_meshtastic_port() -> u16 { 4403 }
fn default_meshtastic_topic() -> String { "msh/US/sigint".to_string() }
fn default_true() -> bool { true }

impl Default for MeshtasticConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            connection_type: "mqtt".to_string(),
            serial_port: None,
            tcp_host: None,
            tcp_port: 4403,
            mqtt_broker: Some("mqtt.meshtastic.org".to_string()),
            mqtt_topic: "msh/US/sigint".to_string(),
            min_threat_level: "high".to_string(),
            compact_messages: true,
            channel: 0,
        }
    }
}

/// EFF RayHunter IMSI catcher detection
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RayHunterConfig {
    pub enabled: bool,
    /// RayHunter API endpoint (usually via ADB forward)
    #[serde(default = "default_rayhunter_url")]
    pub api_url: String,
    /// Poll interval in seconds
    #[serde(default = "default_rayhunter_interval")]
    pub poll_interval_secs: u64,
    /// Alert on any suspicious activity
    #[serde(default = "default_true")]
    pub alert_on_suspicious: bool,
    /// Relay RayHunter alerts to OpenClaw
    #[serde(default = "default_true")]
    pub relay_to_openclaw: bool,
    /// Relay RayHunter alerts to Meshtastic
    #[serde(default)]
    pub relay_to_meshtastic: bool,
    /// Custom alert message template
    #[serde(default)]
    pub alert_template: Option<String>,
}

fn default_rayhunter_url() -> String { "http://localhost:8081".to_string() }
fn default_rayhunter_interval() -> u64 { 5 }

impl Default for RayHunterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_url: default_rayhunter_url(),
            poll_interval_secs: 5,
            alert_on_suspicious: true,
            relay_to_openclaw: true,
            relay_to_meshtastic: false,
            alert_template: None,
        }
    }
}

/// SDR (Software Defined Radio) configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SdrConfig {
    pub enabled: bool,
    /// SDR device type: "rtlsdr", "hackrf", "limesdr"
    pub device_type: String,
    /// Device index or serial number
    #[serde(default)]
    pub device_index: u32,
    /// Frequencies to monitor (Hz)
    #[serde(default)]
    pub monitor_frequencies: Vec<u64>,
    /// Enable automatic frequency scanning
    #[serde(default)]
    pub auto_scan: bool,
    /// Frequency range for auto scan (start_hz, end_hz, step_hz)
    #[serde(default)]
    pub scan_range: Option<(u64, u64, u64)>,
    /// Signal threshold for detection (dB)
    #[serde(default = "default_sdr_threshold")]
    pub signal_threshold_db: f64,
    /// Sample rate
    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,
    /// Gain (0 = auto)
    #[serde(default)]
    pub gain: u32,
}

fn default_sdr_threshold() -> f64 { -50.0 }
fn default_sample_rate() -> u32 { 2_400_000 }

impl Default for SdrConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device_type: "rtlsdr".to_string(),
            device_index: 0,
            monitor_frequencies: vec![
                462_562_500,  // FRS/GMRS Ch 1
                462_587_500,  // FRS/GMRS Ch 2
                467_562_500,  // FRS Ch 8
                154_570_000,  // Police common
                155_475_000,  // Police common
                460_000_000,  // Public safety
            ],
            auto_scan: false,
            scan_range: None,
            signal_threshold_db: -50.0,
            sample_rate: 2_400_000,
            gain: 0,
        }
    }
}

/// LLM/AI Provider Configuration
/// Supports OpenAI-compatible APIs (OpenAI, llama.cpp, Ollama, LMStudio, etc.)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmConfig {
    pub enabled: bool,
    /// Provider type: "openai", "llamacpp", "ollama", "lmstudio", "custom"
    pub provider: String,
    /// API endpoint URL (e.g., "https://api.openai.com/v1" or "http://localhost:8080")
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
            openclaw: Some(OpenClawConfig::default()),
            meshtastic: Some(MeshtasticConfig::default()),
            rayhunter: Some(RayHunterConfig::default()),
            sdr: Some(SdrConfig::default()),
        }
    }
}
