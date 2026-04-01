//! Environmental Sensors Module
//! 
//! Support for:
//! - USB Geiger counters (radiation detection)
//! - Air quality monitors (PM2.5, CO2, VOC)
//! - CBRN detection devices
//!
//! Designed to integrate with Linux-compatible USB sensors

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error, debug};

// ============================================================================
// GEIGER COUNTER / RADIATION DETECTION
// ============================================================================

/// Supported Geiger counter models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeigerModel {
    GqGmc300,       // GQ GMC-300
    GqGmc320,       // GQ GMC-320
    GqGmc500,       // GQ GMC-500
    GqGmc600,       // GQ GMC-600
    RadiationD,     // RadiationD v1.1
    MightyOhm,      // MightyOhm Geiger Kit
    Generic,        // Generic serial Geiger counter
}

/// Radiation measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationReading {
    pub timestamp: u64,
    pub cpm: f64,              // Counts per minute
    pub usv_h: f64,            // Microsieverts per hour
    pub cumulative_dose: f64,  // Cumulative dose in uSv
    pub alarm: bool,
    pub model: GeigerModel,
    pub battery_percent: Option<u8>,
}

/// Geiger counter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeigerConfig {
    pub enabled: bool,
    pub device_path: String,    // e.g., /dev/ttyUSB0
    pub baud_rate: u32,
    pub model: GeigerModel,
    pub alert_threshold_usv: f64,  // Alert if above this
    pub poll_interval_ms: u64,
}

impl Default for GeigerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            device_path: "/dev/ttyUSB0".to_string(),
            baud_rate: 57600,
            model: GeigerModel::GqGmc320,
            alert_threshold_usv: 0.5, // Normal background is ~0.1-0.2 uSv/h
            poll_interval_ms: 1000,
        }
    }
}

/// GQ GMC Geiger counter protocol commands
pub mod gq_gmc_protocol {
    /// Get current CPM
    pub const CMD_GET_CPM: &[u8] = b"<GETCPM>>";
    /// Get device version
    pub const CMD_GET_VER: &[u8] = b"<GETVER>>";
    /// Get serial number
    pub const CMD_GET_SERIAL: &[u8] = b"<GETSERIAL>>";
    /// Turn on data output
    pub const CMD_HEARTBEAT_ON: &[u8] = b"<HEARTBEAT1>>";
    /// Turn off data output
    pub const CMD_HEARTBEAT_OFF: &[u8] = b"<HEARTBEAT0>>";
    /// Get battery voltage
    pub const CMD_GET_VOLT: &[u8] = b"<GETVOLT>>";
    /// Get configuration
    pub const CMD_GET_CFG: &[u8] = b"<GETCFG>>";
    /// Power off
    pub const CMD_POWER_OFF: &[u8] = b"<POWEROFF>>";
    /// Reboot
    pub const CMD_REBOOT: &[u8] = b"<REBOOT>>";
    
    /// Convert CPM to uSv/h (approximate, depends on tube)
    /// Default conversion factor for SBM-20 tube
    pub fn cpm_to_usv(cpm: f64) -> f64 {
        cpm / 153.8  // SBM-20 conversion factor
    }
}

/// Radiation alert levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RadiationLevel {
    Normal,      // < 0.2 uSv/h (typical background)
    Elevated,    // 0.2 - 0.5 uSv/h
    High,        // 0.5 - 1.0 uSv/h
    VeryHigh,    // 1.0 - 10.0 uSv/h
    Dangerous,   // > 10.0 uSv/h
}

impl RadiationLevel {
    pub fn from_usv(usv_h: f64) -> Self {
        if usv_h < 0.2 {
            Self::Normal
        } else if usv_h < 0.5 {
            Self::Elevated
        } else if usv_h < 1.0 {
            Self::High
        } else if usv_h < 10.0 {
            Self::VeryHigh
        } else {
            Self::Dangerous
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::Normal => "Normal background radiation",
            Self::Elevated => "Slightly elevated - monitor",
            Self::High => "High - investigate source",
            Self::VeryHigh => "Very high - leave area",
            Self::Dangerous => "DANGEROUS - evacuate immediately",
        }
    }
}

// ============================================================================
// AIR QUALITY MONITORING
// ============================================================================

/// Air quality measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirQualityReading {
    pub timestamp: u64,
    pub pm1_0: Option<f64>,     // PM1.0 in ug/m3
    pub pm2_5: Option<f64>,     // PM2.5 in ug/m3
    pub pm10: Option<f64>,      // PM10 in ug/m3
    pub co2_ppm: Option<f64>,   // CO2 in ppm
    pub voc_ppb: Option<f64>,   // VOC in ppb
    pub voc_index: Option<u32>, // VOC index (1-500)
    pub nox_index: Option<u32>, // NOx index (1-500)
    pub temperature_c: Option<f64>,
    pub humidity_percent: Option<f64>,
    pub aqi: Option<u32>,       // Air Quality Index
}

/// Supported air quality sensor models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AirQualitySensor {
    // Sensirion sensors (excellent quality)
    Scd30,          // CO2 + temp/humidity
    Scd40,          // CO2 + temp/humidity (smaller)
    Scd41,          // CO2 + temp/humidity (latest)
    Sen5x,          // PM + VOC + NOx + temp/humidity
    Sen66,          // PM + VOC + NOx + CO2 + temp/humidity (all-in-one)
    Sps30,          // PM sensor
    Sgp30,          // VOC + eCO2
    Sgp40,          // VOC index
    
    // Plantower sensors (common, affordable)
    Pms5003,        // PM1.0/2.5/10
    Pms7003,        // PM1.0/2.5/10 (smaller)
    
    // Winsen sensors
    Mhz19,          // CO2
    Ze03,           // Various gas sensors
    
    // Complete devices
    AirGradient,    // Open-source air quality monitor
    Qingping,       // Xiaomi ecosystem
    
    Generic,
}

/// Air quality configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirQualityConfig {
    pub enabled: bool,
    pub sensor_type: AirQualitySensor,
    pub device_path: Option<String>,      // Serial device
    pub i2c_bus: Option<u8>,              // I2C bus number
    pub i2c_address: Option<u8>,          // I2C address
    pub poll_interval_ms: u64,
    pub pm2_5_alert_threshold: f64,       // Alert if PM2.5 above this
    pub co2_alert_threshold: f64,         // Alert if CO2 above this
}

impl Default for AirQualityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sensor_type: AirQualitySensor::Generic,
            device_path: None,
            i2c_bus: Some(1),
            i2c_address: Some(0x62),  // SCD4x default
            poll_interval_ms: 5000,
            pm2_5_alert_threshold: 35.0,  // WHO guideline
            co2_alert_threshold: 1000.0,  // ASHRAE guideline
        }
    }
}

/// AQI categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AqiCategory {
    Good,             // 0-50
    Moderate,         // 51-100
    UnhealthySensitive, // 101-150
    Unhealthy,        // 151-200
    VeryUnhealthy,    // 201-300
    Hazardous,        // 301+
}

impl AqiCategory {
    pub fn from_aqi(aqi: u32) -> Self {
        if aqi <= 50 {
            Self::Good
        } else if aqi <= 100 {
            Self::Moderate
        } else if aqi <= 150 {
            Self::UnhealthySensitive
        } else if aqi <= 200 {
            Self::Unhealthy
        } else if aqi <= 300 {
            Self::VeryUnhealthy
        } else {
            Self::Hazardous
        }
    }
    
    /// Calculate AQI from PM2.5 concentration
    pub fn aqi_from_pm25(pm25: f64) -> u32 {
        // EPA AQI breakpoints for PM2.5
        let (c_low, c_high, i_low, i_high) = if pm25 < 12.1 {
            (0.0, 12.0, 0, 50)
        } else if pm25 < 35.5 {
            (12.1, 35.4, 51, 100)
        } else if pm25 < 55.5 {
            (35.5, 55.4, 101, 150)
        } else if pm25 < 150.5 {
            (55.5, 150.4, 151, 200)
        } else if pm25 < 250.5 {
            (150.5, 250.4, 201, 300)
        } else if pm25 < 350.5 {
            (250.5, 350.4, 301, 400)
        } else {
            (350.5, 500.4, 401, 500)
        };
        
        let aqi = ((i_high - i_low) as f64 / (c_high - c_low)) * (pm25 - c_low) + i_low as f64;
        aqi.round() as u32
    }
}

// ============================================================================
// CBRN DETECTION
// ============================================================================

/// CBRN threat categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CbrnCategory {
    Chemical,
    Biological,
    Radiological,
    Nuclear,
    Explosive,
}

/// CBRN detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CbrnDetection {
    pub timestamp: u64,
    pub category: CbrnCategory,
    pub agent: Option<String>,
    pub concentration: Option<f64>,
    pub unit: Option<String>,
    pub confidence: f64,
    pub threat_level: CbrnThreatLevel,
    pub sensor: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CbrnThreatLevel {
    None,
    Low,
    Moderate,
    High,
    Extreme,
}

/// Known CBRN detection devices (Linux compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CbrnDevice {
    pub name: String,
    pub manufacturer: String,
    pub category: Vec<CbrnCategory>,
    pub interface: String,          // USB, Serial, Network
    pub linux_support: bool,
    pub description: String,
    pub approximate_cost: String,
}

impl CbrnDevice {
    /// Database of known Linux-compatible CBRN sensors
    pub fn available_devices() -> Vec<Self> {
        vec![
            // Radiation
            Self {
                name: "GQ GMC-320+".to_string(),
                manufacturer: "GQ Electronics".to_string(),
                category: vec![CbrnCategory::Radiological],
                interface: "USB Serial".to_string(),
                linux_support: true,
                description: "Consumer Geiger counter with USB data logging".to_string(),
                approximate_cost: "$100-150".to_string(),
            },
            Self {
                name: "GQ GMC-500+".to_string(),
                manufacturer: "GQ Electronics".to_string(),
                category: vec![CbrnCategory::Radiological],
                interface: "USB Serial + WiFi".to_string(),
                linux_support: true,
                description: "Dual-tube Geiger counter for alpha/beta/gamma".to_string(),
                approximate_cost: "$200-250".to_string(),
            },
            Self {
                name: "Radiacode 101/102".to_string(),
                manufacturer: "Radiacode".to_string(),
                category: vec![CbrnCategory::Radiological],
                interface: "Bluetooth + USB".to_string(),
                linux_support: true,
                description: "Scintillation detector with isotope identification".to_string(),
                approximate_cost: "$300-400".to_string(),
            },
            Self {
                name: "MightyOhm Geiger Kit".to_string(),
                manufacturer: "MightyOhm".to_string(),
                category: vec![CbrnCategory::Radiological],
                interface: "Serial TTL".to_string(),
                linux_support: true,
                description: "Open-source DIY Geiger counter kit".to_string(),
                approximate_cost: "$100".to_string(),
            },
            
            // Chemical/Gas
            Self {
                name: "Sensirion SGP40".to_string(),
                manufacturer: "Sensirion".to_string(),
                category: vec![CbrnCategory::Chemical],
                interface: "I2C".to_string(),
                linux_support: true,
                description: "VOC index sensor for air quality".to_string(),
                approximate_cost: "$10-15".to_string(),
            },
            Self {
                name: "Sensirion SEN55".to_string(),
                manufacturer: "Sensirion".to_string(),
                category: vec![CbrnCategory::Chemical],
                interface: "I2C".to_string(),
                linux_support: true,
                description: "PM + VOC + NOx + Temp/Humidity".to_string(),
                approximate_cost: "$30-50".to_string(),
            },
            Self {
                name: "MQ Series Sensors".to_string(),
                manufacturer: "Various".to_string(),
                category: vec![CbrnCategory::Chemical],
                interface: "Analog/Digital GPIO".to_string(),
                linux_support: true,
                description: "MQ-2 (smoke), MQ-7 (CO), MQ-135 (air quality)".to_string(),
                approximate_cost: "$2-10 each".to_string(),
            },
            Self {
                name: "Winsen ZE03 Module".to_string(),
                manufacturer: "Winsen".to_string(),
                category: vec![CbrnCategory::Chemical],
                interface: "UART".to_string(),
                linux_support: true,
                description: "Electrochemical gas sensor (CO, H2S, SO2, etc.)".to_string(),
                approximate_cost: "$20-50".to_string(),
            },
            
            // Biological (limited consumer options)
            Self {
                name: "Particle Counter".to_string(),
                manufacturer: "Various".to_string(),
                category: vec![CbrnCategory::Biological],
                interface: "USB/Serial".to_string(),
                linux_support: true,
                description: "PM counters can detect aerosol anomalies".to_string(),
                approximate_cost: "$50-200".to_string(),
            },
        ]
    }
}

// ============================================================================
// ENVIRONMENTAL SENSOR MANAGER
// ============================================================================

/// Unified environmental sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalReading {
    pub timestamp: u64,
    pub radiation: Option<RadiationReading>,
    pub air_quality: Option<AirQualityReading>,
    pub cbrn_alerts: Vec<CbrnDetection>,
    pub overall_status: EnvironmentalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnvironmentalStatus {
    Normal,
    Caution,
    Warning,
    Danger,
    Evacuate,
}

impl EnvironmentalStatus {
    pub fn calculate(
        radiation: Option<&RadiationReading>,
        air_quality: Option<&AirQualityReading>,
        cbrn_alerts: &[CbrnDetection],
    ) -> Self {
        // Check CBRN alerts first (highest priority)
        for alert in cbrn_alerts {
            match alert.threat_level {
                CbrnThreatLevel::Extreme => return Self::Evacuate,
                CbrnThreatLevel::High => return Self::Danger,
                CbrnThreatLevel::Moderate => return Self::Warning,
                _ => {}
            }
        }
        
        // Check radiation
        if let Some(rad) = radiation {
            let level = RadiationLevel::from_usv(rad.usv_h);
            match level {
                RadiationLevel::Dangerous => return Self::Evacuate,
                RadiationLevel::VeryHigh => return Self::Danger,
                RadiationLevel::High => return Self::Warning,
                RadiationLevel::Elevated => return Self::Caution,
                _ => {}
            }
        }
        
        // Check air quality
        if let Some(aq) = air_quality {
            if let Some(pm25) = aq.pm2_5 {
                if pm25 > 150.0 {
                    return Self::Danger;
                } else if pm25 > 55.0 {
                    return Self::Warning;
                } else if pm25 > 35.0 {
                    return Self::Caution;
                }
            }
            if let Some(co2) = aq.co2_ppm {
                if co2 > 5000.0 {
                    return Self::Danger;
                } else if co2 > 2000.0 {
                    return Self::Warning;
                } else if co2 > 1000.0 {
                    return Self::Caution;
                }
            }
        }
        
        Self::Normal
    }
}

// ============================================================================
// ADS-B / AIRCRAFT TRACKING
// ============================================================================

/// ADS-B aircraft data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AircraftTrack {
    pub icao_hex: String,          // ICAO 24-bit address
    pub callsign: Option<String>,
    pub squawk: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude_ft: Option<i32>,
    pub ground_speed_kt: Option<f64>,
    pub track_deg: Option<f64>,
    pub vertical_rate_fpm: Option<i32>,
    pub aircraft_type: Option<String>,
    pub registration: Option<String>,
    pub category: AircraftCategory,
    pub last_seen: u64,
    pub messages_received: u32,
    pub rssi_db: Option<f64>,
    pub military: bool,
    pub interesting: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AircraftCategory {
    Unknown,
    Light,          // < 15,500 lbs
    Medium,         // 15,500-75,000 lbs
    Heavy,          // > 75,000 lbs
    HighPerformance,
    Rotorcraft,
    Glider,
    Balloon,
    Parachutist,
    Ultralight,
    UAV,
    Space,
    Emergency,
    Military,
    Government,
}

/// ADS-B configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdsbConfig {
    pub enabled: bool,
    pub device_index: u32,
    pub gain: Option<f64>,
    pub frequency_hz: u64,
    pub enable_modeac: bool,
    pub enable_beast: bool,
    pub beast_port: u16,
    pub json_port: u16,
    pub track_military: bool,
    pub track_government: bool,
    pub alert_on_low_altitude: bool,
    pub alert_altitude_ft: i32,
}

impl Default for AdsbConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            device_index: 0,
            gain: None,  // Auto gain
            frequency_hz: 1_090_000_000,
            enable_modeac: false,
            enable_beast: true,
            beast_port: 30005,
            json_port: 30003,
            track_military: true,
            track_government: true,
            alert_on_low_altitude: true,
            alert_altitude_ft: 500,  // Alert if aircraft below this
        }
    }
}

/// Known military/government aircraft ICAO blocks
pub mod military_icao {
    /// US Military ICAO ranges (partial list)
    pub const US_MILITARY: &[(u32, u32)] = &[
        (0xADF000, 0xAFFFFF), // US Military range
    ];
    
    /// Known government/special aircraft
    pub const US_GOVERNMENT: &[&str] = &[
        "AE01", "AE02",  // Air Force One
        "A9CFFF",        // Marine One
        "ADFDF8",        // Various military
    ];
    
    /// Check if ICAO is likely military
    pub fn is_military(icao: u32) -> bool {
        for (start, end) in US_MILITARY {
            if icao >= *start && icao <= *end {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_radiation_levels() {
        assert_eq!(RadiationLevel::from_usv(0.1), RadiationLevel::Normal);
        assert_eq!(RadiationLevel::from_usv(0.3), RadiationLevel::Elevated);
        assert_eq!(RadiationLevel::from_usv(0.7), RadiationLevel::High);
        assert_eq!(RadiationLevel::from_usv(5.0), RadiationLevel::VeryHigh);
        assert_eq!(RadiationLevel::from_usv(15.0), RadiationLevel::Dangerous);
    }
    
    #[test]
    fn test_aqi_calculation() {
        assert!(AqiCategory::aqi_from_pm25(5.0) <= 50);  // Good
        assert!(AqiCategory::aqi_from_pm25(20.0) <= 100); // Moderate
        assert!(AqiCategory::aqi_from_pm25(50.0) <= 150); // Unhealthy for sensitive
    }
    
    #[test]
    fn test_cbrn_devices() {
        let devices = CbrnDevice::available_devices();
        assert!(devices.len() >= 5);
        
        // All should have Linux support
        assert!(devices.iter().all(|d| d.linux_support));
    }
}
