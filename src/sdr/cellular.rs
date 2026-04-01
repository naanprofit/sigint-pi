//! Cellular Tower Mapping
//! 
//! Uses kalibrate-rtl or similar tools to:
//! - Detect nearby cell towers
//! - Map tower frequencies and power levels
//! - Identify potential rogue base stations
//! - Complement RayHunter IMSI catcher detection

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Cellular tower information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellTower {
    pub id: String,
    pub frequency_hz: u64,
    pub arfcn: u32,            // Absolute Radio Frequency Channel Number
    pub band: CellularBand,
    pub power_db: f64,
    pub technology: CellTechnology,
    pub first_seen: u64,
    pub last_seen: u64,
    pub location: Option<(f64, f64)>, // lat, lon if known
    pub suspicious: bool,
    pub suspicion_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellularBand {
    // US Bands
    Band2,   // 1900 MHz PCS
    Band4,   // 1700/2100 MHz AWS
    Band5,   // 850 MHz Cellular
    Band12,  // 700 MHz Lower
    Band13,  // 700 MHz Upper (Verizon)
    Band14,  // FirstNet
    Band17,  // 700 MHz Lower
    Band25,  // 1900 MHz Extended PCS
    Band26,  // 850 MHz Extended
    Band66,  // AWS-3
    Band71,  // 600 MHz
    // GSM Bands
    Gsm850,
    Gsm900,
    Gsm1800,
    Gsm1900,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CellTechnology {
    Gsm2G,
    Umts3G,
    Lte4G,
    Nr5G,
    Unknown,
}

impl CellTower {
    pub fn from_kalibrate_line(line: &str, band: CellularBand) -> Option<Self> {
        // kalibrate-rtl output format: "chan: 123 (934.4MHz +   45Hz) power: 123456.78"
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() < 6 || !line.contains("chan:") {
            return None;
        }
        
        let arfcn = parts.get(1)?.parse::<u32>().ok()?;
        
        // Extract frequency from parentheses
        let freq_str = line.split('(').nth(1)?.split(')').next()?;
        let freq_mhz_str = freq_str.split("MHz").next()?.trim();
        let freq_mhz = freq_mhz_str.parse::<f64>().ok()?;
        let frequency_hz = (freq_mhz * 1_000_000.0) as u64;
        
        // Extract power
        let power_str = line.split("power:").nth(1)?.trim();
        let power_db = power_str.parse::<f64>().ok()?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Some(Self {
            id: format!("{:?}_{}", band, arfcn),
            frequency_hz,
            arfcn,
            band: band.clone(),
            power_db,
            technology: band_to_technology(&band),
            first_seen: now,
            last_seen: now,
            location: None,
            suspicious: false,
            suspicion_reason: None,
        })
    }
}

fn band_to_technology(band: &CellularBand) -> CellTechnology {
    match band {
        CellularBand::Gsm850 | CellularBand::Gsm900 |
        CellularBand::Gsm1800 | CellularBand::Gsm1900 => CellTechnology::Gsm2G,
        CellularBand::Band5 | CellularBand::Band2 => CellTechnology::Umts3G, // Could be either
        _ => CellTechnology::Lte4G,
    }
}

/// Cellular scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularConfig {
    pub enabled: bool,
    pub scan_bands: Vec<CellularBand>,
    pub scan_interval_secs: u64,
    pub device_index: u32,
    pub gain: Option<i32>,
    pub rogue_detection: bool,
}

impl Default for CellularConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_bands: vec![
                CellularBand::Gsm850,
                CellularBand::Gsm1900,
                CellularBand::Band12,
                CellularBand::Band13,
            ],
            scan_interval_secs: 300, // 5 minutes
            device_index: 0,
            gain: None,
            rogue_detection: true,
        }
    }
}

/// Cellular tower scanner
pub struct CellularScanner {
    config: CellularConfig,
    towers: HashMap<String, CellTower>,
    baseline_towers: HashMap<String, CellTower>,
}

impl CellularScanner {
    pub fn new(config: CellularConfig) -> Self {
        Self {
            config,
            towers: HashMap::new(),
            baseline_towers: HashMap::new(),
        }
    }
    
    /// Scan for cell towers using kalibrate-rtl
    pub async fn scan(&mut self) -> anyhow::Result<Vec<CellTower>> {
        let mut all_towers = Vec::new();
        
        for band in &self.config.scan_bands {
            let band_arg = band_to_kal_arg(band);
            if band_arg.is_empty() {
                continue;
            }
            
            info!("Scanning cellular band {:?}", band);
            
            let mut args = vec![
                "-s".to_string(), band_arg.to_string(),
                "-d".to_string(), self.config.device_index.to_string(),
            ];
            
            if let Some(gain) = self.config.gain {
                args.push("-g".to_string());
                args.push(gain.to_string());
            }
            
            let kal_cmd = crate::sdr::resolve_sdr_command("kal");
            let output = match Command::new(&kal_cmd).args(&args).output().await {
                Ok(out) => out,
                Err(_) => {
                    let alt_cmd = crate::sdr::resolve_sdr_command("kalibrate-rtl");
                    Command::new(&alt_cmd).args(&args).output().await?
                }
            };
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            for line in stdout.lines() {
                if let Some(tower) = CellTower::from_kalibrate_line(line, band.clone()) {
                    all_towers.push(tower);
                }
            }
        }
        
        // Update tracking
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        for tower in all_towers.iter_mut() {
            if let Some(existing) = self.towers.get(&tower.id) {
                tower.first_seen = existing.first_seen;
            }
            tower.last_seen = now;
            
            // Check for suspicious characteristics
            if self.config.rogue_detection {
                self.check_suspicious(tower);
            }
            
            self.towers.insert(tower.id.clone(), tower.clone());
        }
        
        Ok(all_towers)
    }
    
    /// Check if a tower has suspicious characteristics
    fn check_suspicious(&self, tower: &mut CellTower) {
        // Check if tower is new (not in baseline)
        if !self.baseline_towers.is_empty() && !self.baseline_towers.contains_key(&tower.id) {
            tower.suspicious = true;
            tower.suspicion_reason = Some("New tower not in baseline".to_string());
            return;
        }
        
        // Check for unusually strong signal (possible portable IMSI catcher)
        if tower.power_db > 100_000.0 { // Threshold depends on calibration
            tower.suspicious = true;
            tower.suspicion_reason = Some("Unusually strong signal".to_string());
            return;
        }
        
        // Check for 2G-only in modern area (common IMSI catcher tactic)
        if tower.technology == CellTechnology::Gsm2G {
            // If baseline has LTE but we're seeing GSM, could be downgrade attack
            let has_lte_baseline = self.baseline_towers.values()
                .any(|t| t.technology == CellTechnology::Lte4G);
            if has_lte_baseline {
                tower.suspicious = true;
                tower.suspicion_reason = Some("GSM tower in LTE-covered area".to_string());
            }
        }
    }
    
    /// Save current towers as baseline
    pub fn save_baseline(&mut self) {
        self.baseline_towers = self.towers.clone();
        info!("Saved {} towers as baseline", self.baseline_towers.len());
    }
    
    /// Load baseline from file
    pub fn load_baseline(&mut self, path: &str) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.baseline_towers = serde_json::from_str(&content)?;
        info!("Loaded {} baseline towers", self.baseline_towers.len());
        Ok(())
    }
    
    /// Save baseline to file
    pub fn save_baseline_to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(&self.baseline_towers)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Get all detected towers
    pub fn get_towers(&self) -> Vec<&CellTower> {
        self.towers.values().collect()
    }
    
    /// Get suspicious towers only
    pub fn get_suspicious_towers(&self) -> Vec<&CellTower> {
        self.towers.values().filter(|t| t.suspicious).collect()
    }
}

fn band_to_kal_arg(band: &CellularBand) -> &'static str {
    match band {
        CellularBand::Gsm850 => "GSM850",
        CellularBand::Gsm900 => "GSM900",
        CellularBand::Gsm1800 => "DCS",
        CellularBand::Gsm1900 => "PCS",
        _ => "", // kalibrate-rtl only supports GSM bands
    }
}

/// Rogue base station alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RogueBaseStationAlert {
    pub tower: CellTower,
    pub alert_type: RogueAlertType,
    pub description: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RogueAlertType {
    NewTower,
    UnusualPower,
    ForcedDowngrade,
    LocationMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_kalibrate_output() {
        let line = "chan: 128 (869.2MHz +   45Hz) power: 123456.78";
        let tower = CellTower::from_kalibrate_line(line, CellularBand::Gsm850);
        assert!(tower.is_some());
        let tower = tower.unwrap();
        assert_eq!(tower.arfcn, 128);
        assert_eq!(tower.frequency_hz, 869_200_000);
    }
}
