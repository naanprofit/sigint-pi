//! Trunked Radio Monitoring
//! 
//! Monitors trunked radio systems:
//! - P25 (Project 25) - US public safety
//! - DMR (Digital Mobile Radio)
//! - NXDN
//! - TETRA (EU emergency services)

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Trunked radio system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrunkedSystem {
    pub id: String,
    pub name: Option<String>,
    pub system_type: TrunkedSystemType,
    pub control_channel_hz: u64,
    pub frequencies: Vec<u64>,
    pub nac: Option<u16>,  // P25 Network Access Code
    pub color_code: Option<u8>,  // DMR Color Code
    pub first_seen: u64,
    pub last_seen: u64,
}

/// Trunked radio traffic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrunkedTraffic {
    pub system_id: String,
    pub frequency_hz: u64,
    pub talkgroup: Option<u32>,
    pub source_id: Option<u32>,
    pub traffic_type: TrafficType,
    pub encrypted: bool,
    pub emergency: bool,
    pub timestamp: u64,
    pub duration_secs: Option<u64>,
    pub audio_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrunkedSystemType {
    P25Phase1,
    P25Phase2,
    DmrTier2,
    DmrTier3,
    Nxdn,
    Tetra,
    AnalogConventional,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrafficType {
    Voice,
    Data,
    Control,
    Paging,
    Emergency,
    Encrypted,
}

/// Known trunked radio frequency ranges
#[derive(Debug, Clone)]
pub struct TrunkedBand {
    pub name: String,
    pub start_hz: u64,
    pub end_hz: u64,
    pub typical_systems: Vec<TrunkedSystemType>,
}

impl TrunkedBand {
    pub fn common_bands() -> Vec<Self> {
        vec![
            Self {
                name: "VHF High".to_string(),
                start_hz: 148_000_000,
                end_hz: 174_000_000,
                typical_systems: vec![TrunkedSystemType::P25Phase1, TrunkedSystemType::AnalogConventional],
            },
            Self {
                name: "UHF".to_string(),
                start_hz: 450_000_000,
                end_hz: 470_000_000,
                typical_systems: vec![TrunkedSystemType::P25Phase1, TrunkedSystemType::DmrTier2],
            },
            Self {
                name: "700 MHz Public Safety".to_string(),
                start_hz: 764_000_000,
                end_hz: 776_000_000,
                typical_systems: vec![TrunkedSystemType::P25Phase2],
            },
            Self {
                name: "800 MHz Public Safety".to_string(),
                start_hz: 851_000_000,
                end_hz: 869_000_000,
                typical_systems: vec![TrunkedSystemType::P25Phase1, TrunkedSystemType::P25Phase2],
            },
            Self {
                name: "900 MHz".to_string(),
                start_hz: 935_000_000,
                end_hz: 941_000_000,
                typical_systems: vec![TrunkedSystemType::P25Phase1],
            },
        ]
    }
}

/// Trunked radio monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrunkedConfig {
    pub enabled: bool,
    pub bands: Vec<String>,  // Band names to monitor
    pub scan_interval_secs: u64,
    pub record_audio: bool,
    pub decode_p25: bool,
    pub decode_dmr: bool,
    pub device_index: u32,
    pub gain: Option<i32>,
}

impl Default for TrunkedConfig {
    fn default() -> Self {
        Self {
            enabled: false,  // Disabled by default (requires additional setup)
            bands: vec!["800 MHz Public Safety".to_string(), "UHF".to_string()],
            scan_interval_secs: 30,
            record_audio: false,
            decode_p25: true,
            decode_dmr: true,
            device_index: 0,
            gain: None,
        }
    }
}

/// Trunked radio scanner
pub struct TrunkedScanner {
    config: TrunkedConfig,
    systems: HashMap<String, TrunkedSystem>,
    traffic: Vec<TrunkedTraffic>,
    all_bands: Vec<TrunkedBand>,
}

impl TrunkedScanner {
    pub fn new(config: TrunkedConfig) -> Self {
        Self {
            config,
            systems: HashMap::new(),
            traffic: Vec::new(),
            all_bands: TrunkedBand::common_bands(),
        }
    }
    
    /// Scan for trunked systems using rtl_power
    pub async fn scan_for_systems(&mut self) -> anyhow::Result<Vec<TrunkedSystem>> {
        let mut found_systems = Vec::new();
        
        for band_name in &self.config.bands {
            if let Some(band) = self.all_bands.iter().find(|b| &b.name == band_name) {
                info!("Scanning {} for trunked systems", band.name);
                
                // Use rtl_power to find active frequencies
                let freq_range = format!(
                    "{}M:{}M:12.5k",
                    band.start_hz / 1_000_000,
                    band.end_hz / 1_000_000
                );
                
                let mut args = vec![
                    "-f".to_string(), freq_range,
                    "-i".to_string(), "2".to_string(), // 2 second integration
                    "-1".to_string(), // Single sweep
                    "-d".to_string(), self.config.device_index.to_string(),
                ];
                
                if let Some(gain) = self.config.gain {
                    args.push("-g".to_string());
                    args.push(gain.to_string());
                }
                
                let output = Command::new("rtl_power")
                    .args(&args)
                    .output()
                    .await?;
                
                let stdout = String::from_utf8_lossy(&output.stdout);
                let active_freqs = self.parse_active_frequencies(&stdout, -70.0);
                
                // Group nearby frequencies into potential systems
                for freq in active_freqs {
                    let system_id = format!("system_{}", freq / 1_000_000);
                    
                    let system = TrunkedSystem {
                        id: system_id.clone(),
                        name: None,
                        system_type: band.typical_systems.first().cloned().unwrap_or(TrunkedSystemType::Unknown),
                        control_channel_hz: freq,
                        frequencies: vec![freq],
                        nac: None,
                        color_code: None,
                        first_seen: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        last_seen: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                    };
                    
                    self.systems.insert(system_id, system.clone());
                    found_systems.push(system);
                }
            }
        }
        
        Ok(found_systems)
    }
    
    /// Parse rtl_power output for active frequencies
    fn parse_active_frequencies(&self, output: &str, threshold_db: f64) -> Vec<u64> {
        let mut frequencies = Vec::new();
        
        for line in output.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 7 {
                continue;
            }
            
            if let (Ok(hz_low), Ok(hz_step)) = (
                parts[2].trim().parse::<u64>(),
                parts[4].trim().parse::<u64>(),
            ) {
                for (i, db_str) in parts[6..].iter().enumerate() {
                    if let Ok(power_db) = db_str.trim().parse::<f64>() {
                        if power_db > threshold_db {
                            let freq = hz_low + (i as u64 * hz_step);
                            
                            // Check if this is near an existing frequency
                            let near_existing = frequencies.iter()
                                .any(|&f: &u64| (f as i64 - freq as i64).abs() < 25_000);
                            
                            if !near_existing {
                                frequencies.push(freq);
                            }
                        }
                    }
                }
            }
        }
        
        frequencies
    }
    
    /// Check if op25 is available for P25 decoding
    pub fn check_op25_available() -> bool {
        std::process::Command::new("rx.py")
            .arg("--help")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    
    /// Check if dsd+ or dsd is available for DMR decoding
    pub fn check_dsd_available() -> bool {
        std::process::Command::new("dsd")
            .arg("--help")
            .output()
            .map(|_| true)
            .unwrap_or(false)
    }
    
    /// Get discovered systems
    pub fn get_systems(&self) -> Vec<&TrunkedSystem> {
        self.systems.values().collect()
    }
    
    /// Get recent traffic
    pub fn get_traffic(&self, limit: usize) -> Vec<&TrunkedTraffic> {
        self.traffic.iter().rev().take(limit).collect()
    }
}

/// Radio reference database lookup (offline)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioReferenceSystem {
    pub sid: u32,
    pub system_name: String,
    pub system_type: String,
    pub city: String,
    pub state: String,
    pub county: String,
    pub frequencies: Vec<RadioReferenceFrequency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioReferenceFrequency {
    pub frequency_hz: u64,
    pub description: String,
    pub mode: String,
    pub tone: Option<String>,
}

/// Simple offline database for known systems
pub struct TrunkedDatabase {
    systems: HashMap<u64, RadioReferenceSystem>,  // Control freq -> system
}

impl TrunkedDatabase {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
        }
    }
    
    pub fn load_from_file(&mut self, path: &str) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let systems: Vec<RadioReferenceSystem> = serde_json::from_str(&content)?;
        
        for sys in systems {
            for freq in &sys.frequencies {
                self.systems.insert(freq.frequency_hz, sys.clone());
            }
        }
        
        info!("Loaded {} systems from database", self.systems.len());
        Ok(())
    }
    
    pub fn lookup(&self, frequency_hz: u64) -> Option<&RadioReferenceSystem> {
        // Look for exact match or within 12.5 kHz
        self.systems.get(&frequency_hz)
            .or_else(|| {
                self.systems.iter()
                    .find(|(&f, _)| (f as i64 - frequency_hz as i64).abs() < 12_500)
                    .map(|(_, sys)| sys)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trunked_bands() {
        let bands = TrunkedBand::common_bands();
        assert!(bands.len() >= 4);
        
        // Check 800 MHz public safety band
        let band_800 = bands.iter().find(|b| b.name.contains("800"));
        assert!(band_800.is_some());
        let band = band_800.unwrap();
        assert!(band.start_hz >= 851_000_000);
        assert!(band.end_hz <= 870_000_000);
    }
}
