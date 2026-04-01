//! rtl_433 Integration - ISM Band Device Detection
//! 
//! Detects wireless devices in ISM bands:
//! - 315 MHz (US garage doors, car keyfobs)
//! - 433.92 MHz (EU/worldwide - weather stations, sensors, keyfobs)
//! - 868 MHz (EU IoT devices)
//! - 915 MHz (US IoT, LoRa)

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::broadcast;
use tracing::{info, warn, error, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// RF Device detected by rtl_433
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfDevice {
    pub id: String,
    pub time: String,
    pub model: String,
    pub device_type: RfDeviceType,
    pub frequency: f64,
    pub rssi: Option<f64>,
    pub snr: Option<f64>,
    pub noise: Option<f64>,
    pub protocol: Option<u32>,
    pub raw_data: HashMap<String, serde_json::Value>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub count: u32,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RfDeviceType {
    WeatherStation,
    TemperatureSensor,
    DoorSensor,
    MotionSensor,
    CarKeyfob,
    GarageDoor,
    TirePressure,
    SmartMeter,
    RemoteControl,
    SecuritySystem,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    None,
    Low,      // Normal household device
    Medium,   // Unknown device, monitor
    High,     // Suspicious (replay attack equipment, etc.)
}

impl RfDevice {
    pub fn from_json(json: &serde_json::Value) -> Option<Self> {
        let model = json.get("model")?.as_str()?.to_string();
        let time = json.get("time").and_then(|t| t.as_str()).unwrap_or("").to_string();
        
        // Generate unique ID from model + any available identifiers
        let id = generate_device_id(json, &model);
        
        let device_type = classify_device(&model, json);
        let threat_level = assess_threat(&model, &device_type, json);
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Some(Self {
            id,
            time,
            model,
            device_type,
            frequency: json.get("freq").and_then(|f| f.as_f64()).unwrap_or(433.92),
            rssi: json.get("rssi").and_then(|r| r.as_f64()),
            snr: json.get("snr").and_then(|s| s.as_f64()),
            noise: json.get("noise").and_then(|n| n.as_f64()),
            protocol: json.get("protocol").and_then(|p| p.as_u64()).map(|p| p as u32),
            raw_data: json.as_object()
                .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
            first_seen: now,
            last_seen: now,
            count: 1,
            threat_level,
        })
    }
}

fn generate_device_id(json: &serde_json::Value, model: &str) -> String {
    // Try various ID fields
    if let Some(id) = json.get("id").and_then(|i| i.as_u64()) {
        return format!("{}_{}", model, id);
    }
    if let Some(id) = json.get("id").and_then(|i| i.as_str()) {
        return format!("{}_{}", model, id);
    }
    if let Some(channel) = json.get("channel").and_then(|c| c.as_u64()) {
        return format!("{}_ch{}", model, channel);
    }
    // Fallback to model + protocol
    if let Some(proto) = json.get("protocol").and_then(|p| p.as_u64()) {
        return format!("{}_p{}", model, proto);
    }
    model.to_string()
}

fn classify_device(model: &str, json: &serde_json::Value) -> RfDeviceType {
    let model_lower = model.to_lowercase();
    
    // Weather stations
    if model_lower.contains("weather") || model_lower.contains("acurite") 
        || model_lower.contains("lacrosse") || model_lower.contains("oregon") 
        || model_lower.contains("ws-") || model_lower.contains("fineoffset") {
        return RfDeviceType::WeatherStation;
    }
    
    // Temperature sensors
    if model_lower.contains("temp") || model_lower.contains("thermo")
        || json.get("temperature_C").is_some() || json.get("temperature_F").is_some() {
        return RfDeviceType::TemperatureSensor;
    }
    
    // Door/window sensors
    if model_lower.contains("door") || model_lower.contains("window")
        || model_lower.contains("contact") || model_lower.contains("open")
        || json.get("contact").is_some() || json.get("opened").is_some() {
        return RfDeviceType::DoorSensor;
    }
    
    // Motion sensors
    if model_lower.contains("motion") || model_lower.contains("pir")
        || json.get("motion").is_some() {
        return RfDeviceType::MotionSensor;
    }
    
    // Car keyfobs
    if model_lower.contains("keyfob") || model_lower.contains("carkey")
        || model_lower.contains("car-") || model_lower.contains("toyota")
        || model_lower.contains("honda") || model_lower.contains("ford") {
        return RfDeviceType::CarKeyfob;
    }
    
    // Garage doors
    if model_lower.contains("garage") || model_lower.contains("liftmaster")
        || model_lower.contains("chamberlain") || model_lower.contains("genie") {
        return RfDeviceType::GarageDoor;
    }
    
    // TPMS
    if model_lower.contains("tpms") || model_lower.contains("tire")
        || json.get("pressure_kPa").is_some() || json.get("pressure_PSI").is_some() {
        return RfDeviceType::TirePressure;
    }
    
    // Smart meters
    if model_lower.contains("meter") || model_lower.contains("idm")
        || model_lower.contains("scm") || model_lower.contains("ert") {
        return RfDeviceType::SmartMeter;
    }
    
    // Security systems
    if model_lower.contains("security") || model_lower.contains("alarm")
        || model_lower.contains("honeywell") || model_lower.contains("adt") {
        return RfDeviceType::SecuritySystem;
    }
    
    // Remote controls
    if model_lower.contains("remote") || model_lower.contains("rc-")
        || model_lower.contains("button") {
        return RfDeviceType::RemoteControl;
    }
    
    RfDeviceType::Unknown
}

fn assess_threat(model: &str, device_type: &RfDeviceType, json: &serde_json::Value) -> ThreatLevel {
    let model_lower = model.to_lowercase();
    
    // Known replay attack tools
    if model_lower.contains("flipper") || model_lower.contains("yardstick")
        || model_lower.contains("hackrf") || model_lower.contains("rf-cat") {
        return ThreatLevel::High;
    }
    
    // Unknown devices warrant monitoring
    if *device_type == RfDeviceType::Unknown {
        return ThreatLevel::Medium;
    }
    
    // Car keyfobs in unexpected locations
    if *device_type == RfDeviceType::CarKeyfob {
        return ThreatLevel::Medium;
    }
    
    ThreatLevel::Low
}

/// rtl_433 scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rtl433Config {
    pub enabled: bool,
    pub frequencies: Vec<f64>,
    pub hop_interval: u32,
    pub gain: Option<i32>,
    pub device_index: u32,
    pub protocols: Vec<u32>,
}

impl Default for Rtl433Config {
    fn default() -> Self {
        Self {
            enabled: true,
            frequencies: vec![433.92, 315.0, 868.0, 915.0],
            hop_interval: 30,
            gain: None,
            device_index: 0,
            protocols: vec![], // Empty = all protocols
        }
    }
}

/// rtl_433 Scanner
pub struct Rtl433Scanner {
    config: Rtl433Config,
    devices: HashMap<String, RfDevice>,
    tx: broadcast::Sender<RfDevice>,
}

impl Rtl433Scanner {
    pub fn new(config: Rtl433Config, tx: broadcast::Sender<RfDevice>) -> Self {
        Self {
            config,
            devices: HashMap::new(),
            tx,
        }
    }
    
    /// Start scanning in background
    pub async fn start(&mut self) -> anyhow::Result<()> {
        if !self.config.enabled {
            info!("rtl_433 scanning disabled");
            return Ok(());
        }
        
        info!("Starting rtl_433 scanner on {:?} MHz", self.config.frequencies);
        
        // Build command
        let mut args = vec![
            "-F".to_string(), "json".to_string(),
            "-M".to_string(), "time:utc".to_string(),
            "-M".to_string(), "level".to_string(),
            "-M".to_string(), "noise".to_string(),
            "-M".to_string(), "protocol".to_string(),
        ];
        
        // Add frequencies
        for freq in &self.config.frequencies {
            args.push("-f".to_string());
            args.push(format!("{}M", freq));
        }
        
        // Add hop interval
        args.push("-H".to_string());
        args.push(self.config.hop_interval.to_string());
        
        // Add gain if specified
        if let Some(gain) = self.config.gain {
            args.push("-g".to_string());
            args.push(gain.to_string());
        }
        
        // Add device index
        args.push("-d".to_string());
        args.push(self.config.device_index.to_string());
        
        // Add specific protocols if configured
        for proto in &self.config.protocols {
            args.push("-R".to_string());
            args.push(proto.to_string());
        }
        
        debug!("rtl_433 command: rtl_433 {}", args.join(" "));
        
        let cmd = crate::sdr::resolve_sdr_command("rtl_433");
        let mut child = Command::new(&cmd)
            .args(&args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        
        let tx = self.tx.clone();
        
        tokio::spawn(async move {
            while let Ok(Some(line)) = lines.next_line().await {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                    if let Some(device) = RfDevice::from_json(&json) {
                        debug!("RF device: {} - {}", device.model, device.id);
                        let _ = tx.send(device);
                    }
                }
            }
            warn!("rtl_433 process ended");
        });
        
        Ok(())
    }
    
    /// Get all detected devices
    pub fn get_devices(&self) -> Vec<&RfDevice> {
        self.devices.values().collect()
    }
    
    /// Update device tracking
    pub fn update_device(&mut self, device: RfDevice) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        if let Some(existing) = self.devices.get_mut(&device.id) {
            existing.last_seen = now;
            existing.count += 1;
            existing.rssi = device.rssi;
            existing.raw_data = device.raw_data;
        } else {
            self.devices.insert(device.id.clone(), device);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_classification() {
        let json: serde_json::Value = serde_json::json!({
            "model": "Acurite-Tower",
            "id": 12345,
            "temperature_C": 22.5,
            "humidity": 65
        });
        
        let device = RfDevice::from_json(&json).unwrap();
        assert_eq!(device.device_type, RfDeviceType::WeatherStation);
        assert_eq!(device.threat_level, ThreatLevel::Low);
    }
    
    #[test]
    fn test_keyfob_detection() {
        let json: serde_json::Value = serde_json::json!({
            "model": "Toyota-Keyfob",
            "id": "ABC123"
        });
        
        let device = RfDevice::from_json(&json).unwrap();
        assert_eq!(device.device_type, RfDeviceType::CarKeyfob);
        assert_eq!(device.threat_level, ThreatLevel::Medium);
    }
}
