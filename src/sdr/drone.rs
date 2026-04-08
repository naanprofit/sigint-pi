//! Drone/UAV Detection
//! 
//! Detects drones by monitoring:
//! - 2.4 GHz control signals (WiFi-based drones, DJI, etc.)
//! - 5.8 GHz FPV video transmitters
//! - 915 MHz (some long-range systems)
//! - Known drone protocol signatures

use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn, debug};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Detected drone signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneSignal {
    pub id: String,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub power_db: f64,
    pub signal_type: DroneSignalType,
    pub drone_type: Option<DroneType>,
    pub protocol: Option<DroneProtocol>,
    pub first_seen: u64,
    pub last_seen: u64,
    pub duration_secs: u64,
    pub direction: Option<f64>,
    pub threat_level: ThreatLevel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectral_features: Option<crate::ml::features::SpectralFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_classification: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneSignalType {
    Control,      // Remote control link
    Telemetry,    // Downlink telemetry
    Video,        // FPV video feed
    Gps,          // GPS spoofing/jamming
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneType {
    // Consumer/Commercial
    DjiMavic,
    DjiPhantom,
    DjiMini,
    DjiInspire,
    DjiMatrice,
    Parrot,
    Autel,
    Skydio,
    FpvRacing,
    FixedWing,
    Custom,
    // Military - Russia
    Orlan10,        // Orlan-10 recon (Russia)
    Lancet,         // ZALA Lancet loitering munition
    ZalaUav,        // ZALA recon variants
    Orion,          // Kronshtadt Orion MALE
    Supercam,       // Supercam-350
    // Military - Iran
    ShahedOwa,      // Shahed-136/Geran-2 one-way attack
    Mohajer,        // Mohajer-6/10 ISTAR
    // Military - China
    ChUav,          // CH-3A/CH-4B/CH-5 series
    // Military - Israel
    Hermes,         // Elbit Hermes 450/900
    Heron,          // IAI Heron
    Harop,          // IAI Harop loitering munition
    // Military - USA
    Predator,       // MQ-1/MQ-9 Reaper
    GlobalHawk,     // RQ-4
    ScanEagle,      // Insitu ScanEagle
    AndurilGhost,   // Anduril Ghost-X
    Switchblade,    // AeroVironment Switchblade
    // Military - Turkey
    Bayraktar,      // Bayraktar TB2/TB3/Akinci
    // Military - Other
    MilitaryGeneric,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneProtocol {
    DjiOcusync,     // DJI OcuSync 1/2/3/4
    DjiLightbridge, // DJI Lightbridge
    Wifi,           // Standard WiFi
    Analog5_8,      // Analog 5.8GHz video
    Crossfire,      // TBS Crossfire (868/915 MHz)
    Expresslrs,     // ExpressLRS
    FrSky,          // FrSky protocols
    Spektrum,       // Spektrum DSMX
    MilitaryUhf,    // Military UHF datalink
    MilitarySBand,  // Military S-band (2-4 GHz)
    MilitaryCBand,  // Military C-band (4-6 GHz)
    MilitaryLBand,  // Military L-band (1-2 GHz)
    SatcomKuBand,   // Satellite comm Ku-band
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThreatLevel {
    None,       // Normal recreational drone
    Low,        // Unknown drone nearby
    Medium,     // Drone loitering, possible surveillance
    High,       // Drone in restricted area or following
    Critical,   // GPS interference detected
}

/// Drone frequency signatures
#[derive(Debug, Clone)]
pub struct DroneSignature {
    pub center_freq_hz: u64,
    pub bandwidth_hz: u64,
    pub protocol: DroneProtocol,
    pub signal_type: DroneSignalType,
    pub description: String,
}

impl DroneSignature {
    pub fn known_signatures() -> Vec<Self> {
        vec![
            // DJI OcuSync 2.4 GHz
            Self {
                center_freq_hz: 2_400_000_000,
                bandwidth_hz: 40_000_000,
                protocol: DroneProtocol::DjiOcusync,
                signal_type: DroneSignalType::Control,
                description: "DJI OcuSync 2.4GHz control".to_string(),
            },
            // DJI OcuSync 5.8 GHz
            Self {
                center_freq_hz: 5_800_000_000,
                bandwidth_hz: 40_000_000,
                protocol: DroneProtocol::DjiOcusync,
                signal_type: DroneSignalType::Video,
                description: "DJI OcuSync 5.8GHz video".to_string(),
            },
            // Analog FPV 5.8 GHz
            Self {
                center_freq_hz: 5_800_000_000,
                bandwidth_hz: 20_000_000,
                protocol: DroneProtocol::Analog5_8,
                signal_type: DroneSignalType::Video,
                description: "Analog FPV video".to_string(),
            },
            // TBS Crossfire 868 MHz (EU)
            Self {
                center_freq_hz: 868_000_000,
                bandwidth_hz: 500_000,
                protocol: DroneProtocol::Crossfire,
                signal_type: DroneSignalType::Control,
                description: "TBS Crossfire 868MHz".to_string(),
            },
            // TBS Crossfire 915 MHz (US)
            Self {
                center_freq_hz: 915_000_000,
                bandwidth_hz: 500_000,
                protocol: DroneProtocol::Crossfire,
                signal_type: DroneSignalType::Control,
                description: "TBS Crossfire 915MHz".to_string(),
            },
            // ExpressLRS 2.4 GHz
            Self {
                center_freq_hz: 2_400_000_000,
                bandwidth_hz: 1_000_000,
                protocol: DroneProtocol::Expresslrs,
                signal_type: DroneSignalType::Control,
                description: "ExpressLRS 2.4GHz".to_string(),
            },
            // WiFi-based drones
            Self {
                center_freq_hz: 2_437_000_000,
                bandwidth_hz: 20_000_000,
                protocol: DroneProtocol::Wifi,
                signal_type: DroneSignalType::Control,
                description: "WiFi drone control".to_string(),
            },
            // ExpressLRS 868 MHz (EU)
            Self {
                center_freq_hz: 868_000_000,
                bandwidth_hz: 500_000,
                protocol: DroneProtocol::Expresslrs,
                signal_type: DroneSignalType::Control,
                description: "ExpressLRS 868MHz EU".to_string(),
            },
            // ExpressLRS 915 MHz (US)
            Self {
                center_freq_hz: 915_000_000,
                bandwidth_hz: 26_000_000,
                protocol: DroneProtocol::Expresslrs,
                signal_type: DroneSignalType::Control,
                description: "ExpressLRS 915MHz US (902-928)".to_string(),
            },
            // FrSky R9 868/915 MHz
            Self {
                center_freq_hz: 868_000_000,
                bandwidth_hz: 5_000_000,
                protocol: DroneProtocol::FrSky,
                signal_type: DroneSignalType::Control,
                description: "FrSky R9 868MHz".to_string(),
            },
            
            // =========================================================
            // MILITARY DRONE SIGNATURES
            // Sources: Armada International, TRADOC, open-source intel
            // =========================================================
            
            // --- RUSSIA ---
            
            // Orlan-10 (most deployed Russian recon UAV)
            // Frequencies: 200-450 MHz, 867-872 MHz, 915-920 MHz, 1.08-1.3 GHz, 2.2-2.7 GHz
            Self {
                center_freq_hz: 325_000_000,
                bandwidth_hz: 250_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Orlan-10 UHF datalink (200-450 MHz)".to_string(),
            },
            Self {
                center_freq_hz: 870_000_000,
                bandwidth_hz: 5_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Telemetry,
                description: "Orlan-10 / Russian UAV 867-872 MHz".to_string(),
            },
            Self {
                center_freq_hz: 917_000_000,
                bandwidth_hz: 5_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Telemetry,
                description: "Orlan-10 / Russian UAV 915-920 MHz".to_string(),
            },
            Self {
                center_freq_hz: 1_190_000_000,
                bandwidth_hz: 220_000_000,
                protocol: DroneProtocol::MilitaryLBand,
                signal_type: DroneSignalType::Control,
                description: "Orlan-10 L-band (1.08-1.3 GHz)".to_string(),
            },
            Self {
                center_freq_hz: 2_450_000_000,
                bandwidth_hz: 500_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "Orlan-10 S-band datalink (2.2-2.7 GHz)".to_string(),
            },
            
            // Lancet / ZALA / Kub (loitering munitions)
            // 867-872 MHz, 902-928 MHz, 1.561-1.616 GHz (GLONASS), 2.2-2.4 GHz
            Self {
                center_freq_hz: 915_000_000,
                bandwidth_hz: 26_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Lancet/ZALA/Kub 902-928 MHz".to_string(),
            },
            Self {
                center_freq_hz: 2_300_000_000,
                bandwidth_hz: 200_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "Lancet/ZALA S-band (2.2-2.4 GHz)".to_string(),
            },
            
            // Orion MALE UAV
            Self {
                center_freq_hz: 905_000_000,
                bandwidth_hz: 30_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Orion UAV 890-920 MHz".to_string(),
            },
            Self {
                center_freq_hz: 4_950_000_000,
                bandwidth_hz: 1_500_000_000,
                protocol: DroneProtocol::MilitaryCBand,
                signal_type: DroneSignalType::Control,
                description: "Orion C-band SATCOM (4.2-5.7 GHz)".to_string(),
            },
            
            // Granat-1/2/3/4
            Self {
                center_freq_hz: 1_180_000_000,
                bandwidth_hz: 200_000_000,
                protocol: DroneProtocol::MilitaryLBand,
                signal_type: DroneSignalType::Control,
                description: "Granat L-band (1.08-1.28 GHz)".to_string(),
            },
            
            // --- IRAN ---
            
            // Shahed-136/Geran-2 (one-way attack, uses GPS/GLONASS + inertial)
            // Primarily autonomous but may use cellular/900 MHz for RTK
            Self {
                center_freq_hz: 900_000_000,
                bandwidth_hz: 50_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Shahed-136/Geran-2 UHF control".to_string(),
            },
            
            // Mohajer-6 (L-band datalink)
            Self {
                center_freq_hz: 1_400_000_000,
                bandwidth_hz: 400_000_000,
                protocol: DroneProtocol::MilitaryLBand,
                signal_type: DroneSignalType::Control,
                description: "Mohajer-6 L-band (1.2-1.6 GHz)".to_string(),
            },
            
            // --- CHINA ---
            
            // CH-3A
            Self {
                center_freq_hz: 357_000_000,
                bandwidth_hz: 65_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "CH-3A UHF (325-390 MHz)".to_string(),
            },
            Self {
                center_freq_hz: 655_000_000,
                bandwidth_hz: 270_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "CH-3A UHF (520-790 MHz)".to_string(),
            },
            Self {
                center_freq_hz: 2_475_000_000,
                bandwidth_hz: 350_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "CH-3A S-band (2.3-2.65 GHz)".to_string(),
            },
            
            // CH-4B (Chinese MALE, exported widely)
            Self {
                center_freq_hz: 850_000_000,
                bandwidth_hz: 100_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "CH-4B UHF (800-900 MHz)".to_string(),
            },
            Self {
                center_freq_hz: 5_150_000_000,
                bandwidth_hz: 1_900_000_000,
                protocol: DroneProtocol::MilitaryCBand,
                signal_type: DroneSignalType::Control,
                description: "CH-4B C-band (4.2-6.1 GHz)".to_string(),
            },
            Self {
                center_freq_hz: 10_000_000_000,
                bandwidth_hz: 4_000_000_000,
                protocol: DroneProtocol::SatcomKuBand,
                signal_type: DroneSignalType::Control,
                description: "CH-4B X-band SATCOM (8-12 GHz)".to_string(),
            },
            
            // --- TURKEY ---
            
            // Bayraktar TB2/TB3 (uses both LOS and SATCOM)
            Self {
                center_freq_hz: 900_000_000,
                bandwidth_hz: 50_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Bayraktar UHF LOS control".to_string(),
            },
            Self {
                center_freq_hz: 1_800_000_000,
                bandwidth_hz: 400_000_000,
                protocol: DroneProtocol::MilitaryLBand,
                signal_type: DroneSignalType::Control,
                description: "Bayraktar L-band datalink".to_string(),
            },
            Self {
                center_freq_hz: 5_000_000_000,
                bandwidth_hz: 1_000_000_000,
                protocol: DroneProtocol::MilitaryCBand,
                signal_type: DroneSignalType::Control,
                description: "Bayraktar C-band SATCOM".to_string(),
            },
            
            // --- USA ---
            
            // MQ-9 Reaper / MQ-1 Predator (C-band LOS + Ku-band SATCOM)
            Self {
                center_freq_hz: 5_000_000_000,
                bandwidth_hz: 1_000_000_000,
                protocol: DroneProtocol::MilitaryCBand,
                signal_type: DroneSignalType::Control,
                description: "Predator/Reaper C-band LOS".to_string(),
            },
            Self {
                center_freq_hz: 14_500_000_000,
                bandwidth_hz: 2_000_000_000,
                protocol: DroneProtocol::SatcomKuBand,
                signal_type: DroneSignalType::Control,
                description: "Predator/Reaper Ku-band SATCOM".to_string(),
            },
            
            // Anduril Ghost-X (resilient mesh, freq-agile)
            Self {
                center_freq_hz: 900_000_000,
                bandwidth_hz: 28_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Anduril Ghost 902-928 MHz mesh".to_string(),
            },
            Self {
                center_freq_hz: 2_440_000_000,
                bandwidth_hz: 83_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "Anduril Ghost 2.4 GHz mesh".to_string(),
            },
            
            // AeroVironment Switchblade
            Self {
                center_freq_hz: 900_000_000,
                bandwidth_hz: 26_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Switchblade UHF (902-928 MHz)".to_string(),
            },
            
            // ScanEagle
            Self {
                center_freq_hz: 2_300_000_000,
                bandwidth_hz: 200_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "ScanEagle S-band datalink".to_string(),
            },
            
            // --- ISRAEL ---
            
            // Elbit Hermes 450/900 (S-band LOS)
            Self {
                center_freq_hz: 2_300_000_000,
                bandwidth_hz: 200_000_000,
                protocol: DroneProtocol::MilitarySBand,
                signal_type: DroneSignalType::Control,
                description: "Hermes 450/900 S-band LOS".to_string(),
            },
            
            // IAI Heron (SATCOM Ku-band + LOS)
            Self {
                center_freq_hz: 1_300_000_000,
                bandwidth_hz: 200_000_000,
                protocol: DroneProtocol::MilitaryLBand,
                signal_type: DroneSignalType::Control,
                description: "Heron L-band LOS datalink".to_string(),
            },
            Self {
                center_freq_hz: 14_500_000_000,
                bandwidth_hz: 2_000_000_000,
                protocol: DroneProtocol::SatcomKuBand,
                signal_type: DroneSignalType::Control,
                description: "Heron Ku-band SATCOM".to_string(),
            },
            
            // IAI Harop loitering munition
            Self {
                center_freq_hz: 900_000_000,
                bandwidth_hz: 50_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "Harop UHF datalink".to_string(),
            },
            
            // --- GENERAL MILITARY BANDS ---
            
            // NATO STANAG 4586 common datalink bands
            Self {
                center_freq_hz: 350_000_000,
                bandwidth_hz: 100_000_000,
                protocol: DroneProtocol::MilitaryUhf,
                signal_type: DroneSignalType::Control,
                description: "NATO UHF tactical drone band (300-400 MHz)".to_string(),
            },
            Self {
                center_freq_hz: 1_575_420_000,
                bandwidth_hz: 15_000_000,
                protocol: DroneProtocol::Unknown,
                signal_type: DroneSignalType::Gps,
                description: "GPS L1 (all GPS-guided drones)".to_string(),
            },
            Self {
                center_freq_hz: 1_602_000_000,
                bandwidth_hz: 20_000_000,
                protocol: DroneProtocol::Unknown,
                signal_type: DroneSignalType::Gps,
                description: "GLONASS L1 (Russian/Iranian drones)".to_string(),
            },
        ]
    }
}

/// Drone detector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneDetectorConfig {
    pub enabled: bool,
    pub monitor_2_4ghz: bool,
    pub monitor_5_8ghz: bool,
    pub monitor_sub_ghz: bool,
    pub scan_interval_secs: u64,
    pub min_signal_db: f64,
    pub device_index: u32,
}

impl Default for DroneDetectorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitor_2_4ghz: true,
            monitor_5_8ghz: true,
            monitor_sub_ghz: true,
            scan_interval_secs: 10,
            min_signal_db: -70.0,
            device_index: 0,
        }
    }
}

/// Drone signal detector
pub struct DroneDetector {
    config: DroneDetectorConfig,
    signatures: Vec<DroneSignature>,
    detected: HashMap<String, DroneSignal>,
}

impl DroneDetector {
    pub fn new(config: DroneDetectorConfig) -> Self {
        Self {
            config,
            signatures: DroneSignature::known_signatures(),
            detected: HashMap::new(),
        }
    }
    
    /// Scan 2.4 GHz band for drone signals
    pub async fn scan_2_4ghz(&mut self) -> anyhow::Result<Vec<DroneSignal>> {
        if !self.config.monitor_2_4ghz {
            return Ok(vec![]);
        }
        
        // Use hackrf_sweep for wideband scanning
        let output = Command::new("hackrf_sweep")
            .args(&[
                "-f", "2400:2500",
                "-w", "500000", // 500 kHz bins
                "-1", // Single sweep
            ])
            .output()
            .await;
        
        let mut signals = Vec::new();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            signals.extend(self.parse_sweep_for_drones(&stdout, 2_400_000_000, 2_500_000_000));
        }
        
        Ok(signals)
    }
    
    /// Scan 5.8 GHz band for FPV video
    pub async fn scan_5_8ghz(&mut self) -> anyhow::Result<Vec<DroneSignal>> {
        if !self.config.monitor_5_8ghz {
            return Ok(vec![]);
        }
        
        let output = Command::new("hackrf_sweep")
            .args(&[
                "-f", "5650:5950",
                "-w", "1000000", // 1 MHz bins
                "-1",
            ])
            .output()
            .await;
        
        let mut signals = Vec::new();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            signals.extend(self.parse_sweep_for_drones(&stdout, 5_650_000_000, 5_950_000_000));
        }
        
        Ok(signals)
    }
    
    /// Parse spectrum sweep data for drone signatures
    fn parse_sweep_for_drones(&mut self, output: &str, start_hz: u64, end_hz: u64) -> Vec<DroneSignal> {
        let mut signals = Vec::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Parse hackrf_sweep output and look for strong signals
        let mut power_by_freq: HashMap<u64, f64> = HashMap::new();
        
        for line in output.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 7 {
                continue;
            }
            
            if let (Ok(hz_low), Ok(hz_bin)) = (
                parts[2].trim().parse::<u64>(),
                parts[4].trim().parse::<u64>(),
            ) {
                for (i, db_str) in parts[6..].iter().enumerate() {
                    if let Ok(power_db) = db_str.trim().parse::<f64>() {
                        let freq = hz_low + (i as u64 * hz_bin);
                        power_by_freq.insert(freq, power_db);
                    }
                }
            }
        }
        
        // Look for signals matching known drone signatures
        for sig in &self.signatures {
            if sig.center_freq_hz < start_hz || sig.center_freq_hz > end_hz {
                continue;
            }
            
            // Check if there's significant power in this frequency range
            let sig_start = sig.center_freq_hz.saturating_sub(sig.bandwidth_hz / 2);
            let sig_end = sig.center_freq_hz + sig.bandwidth_hz / 2;
            
            let matching_powers: Vec<f64> = power_by_freq.iter()
                .filter(|(&freq, _)| freq >= sig_start && freq <= sig_end)
                .map(|(_, &power)| power)
                .collect();
            
            if matching_powers.is_empty() {
                continue;
            }
            
            let max_power = matching_powers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let avg_power = matching_powers.iter().sum::<f64>() / matching_powers.len() as f64;
            
            // If signal is above threshold, consider it a detection
            if max_power > self.config.min_signal_db {
                let id = format!("{:?}_{}", sig.protocol, sig.center_freq_hz);
                
                // Extract spectral features from the signal's frequency range
                let sig_powers: Vec<f32> = power_by_freq.iter()
                    .filter(|(&freq, _)| freq >= sig_start && freq <= sig_end)
                    .map(|(_, &p)| p as f32)
                    .collect();
                let ml_features = if sig_powers.len() >= 4 {
                    Some(crate::ml::features::extract_spectral_features(&sig_powers))
                } else { None };

                // Classify based on spectral features
                let ml_class = ml_features.as_ref().map(|f| {
                    if f.flatness > 0.5 { "FHSS/Spread-Spectrum" }
                    else if f.num_peaks <= 2 && f.bandwidth < 3.0 { "Narrowband Control" }
                    else if f.num_peaks > 5 { "OFDM/Wideband" }
                    else if f.peak_to_avg > 10.0 { "Burst/TDMA" }
                    else { "Unknown Modulation" }
                }.to_string());

                let drone_signal = DroneSignal {
                    id: id.clone(),
                    frequency_hz: sig.center_freq_hz,
                    bandwidth_hz: sig.bandwidth_hz,
                    power_db: max_power,
                    signal_type: sig.signal_type.clone(),
                    drone_type: guess_drone_type(&sig.protocol),
                    protocol: Some(sig.protocol.clone()),
                    first_seen: self.detected.get(&id).map(|d| d.first_seen).unwrap_or(now),
                    last_seen: now,
                    duration_secs: 0,
                    direction: None,
                    threat_level: assess_drone_threat(max_power, &sig.signal_type),
                    spectral_features: ml_features,
                    ml_classification: ml_class,
                };
                
                self.detected.insert(id, drone_signal.clone());
                signals.push(drone_signal);
            }
        }
        
        signals
    }
    
    /// Get all detected drone signals
    pub fn get_detected(&self) -> Vec<&DroneSignal> {
        self.detected.values().collect()
    }
    
    /// Clean up old detections
    pub fn cleanup_old(&mut self, max_age_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.detected.retain(|_, signal| {
            now - signal.last_seen < max_age_secs
        });
    }
}

fn guess_drone_type(protocol: &DroneProtocol) -> Option<DroneType> {
    match protocol {
        DroneProtocol::DjiOcusync | DroneProtocol::DjiLightbridge => Some(DroneType::DjiMavic),
        DroneProtocol::Analog5_8 | DroneProtocol::Expresslrs | DroneProtocol::Crossfire | DroneProtocol::FrSky => Some(DroneType::FpvRacing),
        DroneProtocol::MilitaryUhf | DroneProtocol::MilitaryLBand | DroneProtocol::MilitarySBand | DroneProtocol::MilitaryCBand | DroneProtocol::SatcomKuBand => Some(DroneType::MilitaryGeneric),
        _ => None,
    }
}

fn assess_drone_threat(power_db: f64, signal_type: &DroneSignalType) -> ThreatLevel {
    // Very strong signal = drone is very close
    if power_db > -30.0 {
        return ThreatLevel::High;
    }
    
    // Strong signal = drone is nearby
    if power_db > -50.0 {
        return ThreatLevel::Medium;
    }
    
    // Video signal = active surveillance possible
    if *signal_type == DroneSignalType::Video && power_db > -60.0 {
        return ThreatLevel::Medium;
    }
    
    ThreatLevel::Low
}

// ============================================================================
// EMI / MOTOR HARMONIC DETECTION
// ============================================================================

/// EMI detection for drone motors/ESCs
/// Uses RTL-SDR direct sampling mode to detect PWM switching harmonics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmiSignature {
    pub fundamental_khz: f64,
    pub harmonics_detected: Vec<f64>,
    pub power_db: f64,
    pub estimated_motor_count: u8,
    pub esc_type: EscType,
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectral_features: Option<crate::ml::features::SpectralFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_z_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EscType {
    BLHeliS,      // 24/48 kHz switching
    BLHeli32,     // 24/48/96 kHz
    Industrial,   // 8/16 kHz (T-Motor, Hobbywing)
    DjiStock,     // 16 kHz (consumer DJI)
    Unknown,
}

/// Known ESC PWM frequencies and their harmonic patterns
#[derive(Debug, Clone)]
pub struct EscPwmSignature {
    pub esc_type: EscType,
    pub fundamental_khz: f64,
    pub expected_harmonics: Vec<f64>,
    pub description: String,
}

impl EscPwmSignature {
    pub fn known_signatures() -> Vec<Self> {
        vec![
            // BLHeli_S - Most common hobby ESCs
            Self {
                esc_type: EscType::BLHeliS,
                fundamental_khz: 24.0,
                expected_harmonics: vec![24.0, 48.0, 72.0, 96.0, 120.0, 144.0],
                description: "BLHeli_S 24kHz PWM".to_string(),
            },
            Self {
                esc_type: EscType::BLHeliS,
                fundamental_khz: 48.0,
                expected_harmonics: vec![48.0, 96.0, 144.0, 192.0, 240.0],
                description: "BLHeli_S 48kHz PWM (DShot)".to_string(),
            },
            // BLHeli_32 - Higher performance
            Self {
                esc_type: EscType::BLHeli32,
                fundamental_khz: 48.0,
                expected_harmonics: vec![48.0, 96.0, 144.0, 192.0, 240.0, 288.0],
                description: "BLHeli_32 48kHz".to_string(),
            },
            Self {
                esc_type: EscType::BLHeli32,
                fundamental_khz: 96.0,
                expected_harmonics: vec![96.0, 192.0, 288.0, 384.0],
                description: "BLHeli_32 96kHz (high frequency)".to_string(),
            },
            // Industrial / Cinema drones
            Self {
                esc_type: EscType::Industrial,
                fundamental_khz: 8.0,
                expected_harmonics: vec![8.0, 16.0, 24.0, 32.0, 40.0, 48.0],
                description: "Industrial 8kHz (T-Motor, large drones)".to_string(),
            },
            Self {
                esc_type: EscType::Industrial,
                fundamental_khz: 16.0,
                expected_harmonics: vec![16.0, 32.0, 48.0, 64.0, 80.0],
                description: "Industrial 16kHz (Hobbywing XRotor)".to_string(),
            },
            // Consumer DJI
            Self {
                esc_type: EscType::DjiStock,
                fundamental_khz: 16.0,
                expected_harmonics: vec![16.0, 32.0, 48.0, 64.0],
                description: "DJI stock ESC 16kHz".to_string(),
            },
        ]
    }
}

/// EMI detector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmiDetectorConfig {
    pub enabled: bool,
    pub scan_range_khz: (f64, f64),  // VLF/LF range to scan
    pub min_signal_db: f64,
    pub min_harmonics_required: u8,  // Minimum harmonics to confirm detection
    pub harmonic_tolerance_hz: f64,  // Tolerance for harmonic frequency matching
    pub device_index: u32,
}

impl Default for EmiDetectorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_range_khz: (5.0, 500.0),  // 5 kHz to 500 kHz
            min_signal_db: -50.0,
            min_harmonics_required: 3,
            harmonic_tolerance_hz: 500.0,  // 500 Hz tolerance
            device_index: 0,
        }
    }
}

/// EMI harmonic detector for drone motors
pub struct EmiDetector {
    config: EmiDetectorConfig,
    signatures: Vec<EscPwmSignature>,
    detected_emi: Vec<EmiSignature>,
    pub anomaly_detector: crate::ml::anomaly::SpectrumAnomalyDetector,
    baseline_collecting: bool,
}

impl EmiDetector {
    pub fn new(config: EmiDetectorConfig) -> Self {
        Self {
            config,
            signatures: EscPwmSignature::known_signatures(),
            detected_emi: Vec::new(),
            anomaly_detector: crate::ml::anomaly::SpectrumAnomalyDetector::new(3.5),
            baseline_collecting: true,
        }
    }

    /// Analyze raw IQ file using Rust FFT (ML module) instead of Python.
    /// Returns detected EMI signatures using ML-enhanced harmonic detection.
    fn analyze_iq_with_ml(&mut self, iq_path: &str, sample_rate: u64) -> Vec<EmiSignature> {
        let raw = match std::fs::read(iq_path) {
            Ok(d) => d,
            Err(_) => return vec![],
        };
        if raw.len() < 512 { return vec![]; }

        // Convert RTL-SDR u8 IQ to f32 (centered at 0, range -1..1)
        let iq_f32: Vec<f32> = raw.iter()
            .map(|&b| (b as f32 - 127.5) / 127.5)
            .collect();

        let window_size = 4096.min(iq_f32.len() / 2);
        let magnitudes = crate::ml::features::iq_to_fft_magnitude(&iq_f32, window_size);

        // Feed into anomaly baseline if still collecting
        if self.baseline_collecting {
            self.anomaly_detector.update_baseline(&magnitudes);
            if self.anomaly_detector.num_baseline_samples() >= 20 {
                self.baseline_collecting = false;
                info!("EMI ML baseline collected ({} samples)", self.anomaly_detector.num_baseline_samples());
            }
        }

        // Check for anomalies against learned baseline
        let anomaly = self.anomaly_detector.check_anomaly(&magnitudes);

        // Extract spectral features for classification
        let features = crate::ml::features::extract_spectral_features(&magnitudes);

        // ML-enhanced harmonic detection (more robust than rule-based)
        let harmonic_series = crate::ml::features::detect_harmonics(&magnitudes, 3, 3);

        let mut detected = Vec::new();
        let bin_hz = sample_rate as f64 / (window_size as f64 * 2.0);

        for series in &harmonic_series {
            let fund_khz = series.fundamental_bin as f64 * bin_hz / 1000.0;
            let harmonics_khz: Vec<f64> = series.harmonics.iter()
                .map(|(bin, _)| *bin as f64 * bin_hz / 1000.0)
                .collect();
            let avg_power: f64 = series.harmonics.iter()
                .map(|(_, mag)| 20.0 * (*mag as f64).max(1e-10).log10())
                .sum::<f64>() / series.harmonics.len() as f64;

            // Try to match to known ESC type
            let mut best_match: Option<(&EscPwmSignature, f64)> = None;
            for sig in &self.signatures {
                let dist = (fund_khz - sig.fundamental_khz).abs();
                if dist < 5.0 {
                    let score = 1.0 - (dist / sig.fundamental_khz).min(1.0);
                    if best_match.map(|(_, s)| score > s).unwrap_or(true) {
                        best_match = Some((sig, score));
                    }
                }
            }

            let (esc_type, confidence) = if let Some((sig, match_score)) = best_match {
                let harmonic_ratio = series.harmonics.len() as f64 / sig.expected_harmonics.len() as f64;
                (sig.esc_type.clone(), (match_score * 0.4 + harmonic_ratio.min(1.0) * 0.4 + if anomaly.is_anomaly { 0.2 } else { 0.0 }).min(1.0))
            } else {
                (EscType::Unknown, 0.3 + if anomaly.is_anomaly { 0.2 } else { 0.0 })
            };

            let motor_count = if series.harmonics.len() >= 6 { 4 }
                else if series.harmonics.len() >= 4 { 2 }
                else { 1 };

            detected.push(EmiSignature {
                fundamental_khz: fund_khz,
                harmonics_detected: harmonics_khz,
                power_db: avg_power,
                estimated_motor_count: motor_count,
                esc_type,
                confidence,
                spectral_features: Some(features.clone()),
                anomaly_detected: Some(anomaly.is_anomaly),
                anomaly_z_score: Some(anomaly.z_score),
            });
        }

        // If anomaly detected but no harmonic series found, still flag it
        if detected.is_empty() && anomaly.is_anomaly && self.anomaly_detector.has_baseline() {
            info!("ML anomaly detected (z={:.1}) but no harmonic series found - possible novel signature", anomaly.z_score);
            detected.push(EmiSignature {
                fundamental_khz: 0.0,
                harmonics_detected: vec![],
                power_db: features.max_val as f64,
                estimated_motor_count: 0,
                esc_type: EscType::Unknown,
                confidence: 0.15 + (anomaly.z_score / 10.0).min(0.3),
                spectral_features: Some(features.clone()),
                anomaly_detected: Some(true),
                anomaly_z_score: Some(anomaly.z_score),
            });
        }

        detected
    }
    
    /// Scan for motor EMI harmonics.
    ///
    /// Strategy by available hardware:
    /// 1. RTL-SDR direct sampling mode (-D 2): captures baseband 0-14.4 MHz via ADC
    /// 2. HackRF sweep: can scan from 1 MHz, covers the motor EMI range
    /// 3. RTL-SDR tuner fallback: scans 24-30 MHz for higher-order harmonics only
    pub async fn scan_emi(&mut self) -> anyhow::Result<Vec<EmiSignature>> {
        if !self.config.enabled {
            return Ok(vec![]);
        }
        
        let start_khz = self.config.scan_range_khz.0 as u32; // typically 5
        let end_khz = self.config.scan_range_khz.1 as u32;   // typically 500
        let resolve = crate::sdr::resolve_sdr_command;
        
        let mut detected = Vec::new();
        
        // Strategy 1: RTL-SDR direct sampling mode for true sub-24MHz reception
        // -D 2 enables Q-branch direct sampling, bypassing the tuner
        // This gives ~0 to ~14.4 MHz (half the 28.8 MSPS ADC rate)
        let center_hz: u64 = ((start_khz as u64 + end_khz.min(14400) as u64) / 2) * 1000;
        let sample_rate: u64 = 2_400_000; // 2.4 MSPS
        let iq_path = "/tmp/sigint_emi_direct.bin";
        let num_samples = sample_rate * 2; // 1 second of data
        
        let rtl_direct = Command::new(&resolve("rtl_sdr"))
            .args(&[
                "-D", "2",            // direct sampling Q-branch
                "-f", &center_hz.to_string(),
                "-s", &sample_rate.to_string(),
                "-n", &num_samples.to_string(),
                "-g", "0",            // no tuner gain in direct sampling
                "-d", &self.config.device_index.to_string(),
                iq_path,
            ])
            .output()
            .await;
        
        if let Ok(out) = &rtl_direct {
            if out.status.success() {
                // ML-enhanced analysis: Rust FFT + harmonic detection + anomaly baseline
                let ml_detected = self.analyze_iq_with_ml(iq_path, sample_rate);
                if !ml_detected.is_empty() {
                    detected = ml_detected;
                    info!("EMI ML detected {} signatures from direct sampling IQ", detected.len());
                    self.detected_emi = detected.clone();
                    return Ok(detected);
                }
                // Fallback: Python FFT + rule-based analysis
                if let Some(csv) = crate::web::api::iq_to_csv(iq_path, center_hz, sample_rate, false).await {
                    detected = self.analyze_emi_spectrum(&csv);
                    if !detected.is_empty() {
                        return Ok(detected);
                    }
                }
            }
        }
        
        // Strategy 2: HackRF sweep (supports 1 MHz and up)
        let hrf_end_mhz = (end_khz as f64 / 1000.0).max(1.0).ceil() as u32;
        let hackrf_output = Command::new(&resolve("hackrf_sweep"))
            .args(&[
                "-f", &format!("1:{}", hrf_end_mhz.max(2)),
                "-w", "10000",
                "-1",
            ])
            .output()
            .await;
        
        if let Ok(out) = hackrf_output {
            if out.status.success() && !out.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                detected = self.analyze_emi_spectrum(&stdout);
                if !detected.is_empty() {
                    return Ok(detected);
                }
            }
        }
        
        // Strategy 3: RTL-SDR tuner mode fallback (24-30 MHz for higher-order harmonics)
        let freq_range = "24M:30M:1k";
        let output = Command::new(&resolve("rtl_power"))
            .args(&[
                "-f", freq_range,
                "-i", "1", "-1", "-g", "40",
                "-d", &self.config.device_index.to_string(),
            ])
            .output()
            .await;
        
        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                detected = self.analyze_emi_spectrum(&stdout);
            }
            Ok(_) => {
                debug!("rtl_power EMI fallback produced no usable output");
            }
            Err(e) => {
                warn!("EMI scan failed: {}", e);
            }
        }
        
        self.detected_emi = detected.clone();
        Ok(detected)
    }
    
    /// Analyze spectrum data for motor EMI harmonics
    fn analyze_emi_spectrum(&self, output: &str) -> Vec<EmiSignature> {
        let mut detected = Vec::new();
        let mut power_by_freq: HashMap<u64, f64> = HashMap::new();
        
        // Parse rtl_power or hackrf_sweep output
        for line in output.lines() {
            // rtl_power format: date, time, hz_low, hz_high, hz_step, samples, db1, db2, ...
            // hackrf_sweep format: date, time, hz_low, hz_high, hz_step, samples, db1, db2, ...
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
                        let freq_hz = hz_low + (i as u64 * hz_step);
                        power_by_freq.insert(freq_hz, power_db);
                    }
                }
            }
        }
        
        if power_by_freq.is_empty() {
            return detected;
        }
        
        // Find peaks in the spectrum
        let peaks = self.find_spectral_peaks(&power_by_freq);
        
        // Try to match harmonic patterns to known ESC signatures
        for sig in &self.signatures {
            if let Some(emi) = self.match_harmonic_pattern(&peaks, sig) {
                detected.push(emi);
            }
        }
        
        // Also look for unknown harmonic series (potential new/unknown drones)
        if let Some(unknown) = self.detect_unknown_harmonics(&peaks) {
            detected.push(unknown);
        }
        
        detected
    }
    
    /// Find peaks in the spectrum that are above threshold
    fn find_spectral_peaks(&self, power_by_freq: &HashMap<u64, f64>) -> Vec<(u64, f64)> {
        let mut peaks = Vec::new();
        let mut sorted_freqs: Vec<_> = power_by_freq.iter().collect();
        sorted_freqs.sort_by_key(|(f, _)| *f);
        
        // Simple peak detection: find local maxima above threshold
        for i in 1..sorted_freqs.len().saturating_sub(1) {
            let (freq, &power) = sorted_freqs[i];
            let (_, &prev_power) = sorted_freqs[i - 1];
            let (_, &next_power) = sorted_freqs[i + 1];
            
            if power > self.config.min_signal_db 
                && power > prev_power 
                && power > next_power 
            {
                peaks.push((*freq, power));
            }
        }
        
        // Sort by power (strongest first)
        peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        peaks
    }
    
    /// Try to match peaks to a known ESC harmonic pattern
    fn match_harmonic_pattern(&self, peaks: &[(u64, f64)], signature: &EscPwmSignature) -> Option<EmiSignature> {
        let fundamental_hz = (signature.fundamental_khz * 1000.0) as u64;
        let tolerance = self.config.harmonic_tolerance_hz as u64;
        
        let mut matched_harmonics = Vec::new();
        let mut total_power = 0.0;
        
        for expected_khz in &signature.expected_harmonics {
            let expected_hz = (*expected_khz * 1000.0) as u64;
            
            // Look for a peak near this harmonic frequency
            for (freq, power) in peaks {
                if (*freq as i64 - expected_hz as i64).unsigned_abs() <= tolerance {
                    matched_harmonics.push(*expected_khz);
                    total_power += power;
                    break;
                }
            }
        }
        
        // Need minimum number of harmonics to confirm detection
        if matched_harmonics.len() >= self.config.min_harmonics_required as usize {
            let confidence = matched_harmonics.len() as f64 / signature.expected_harmonics.len() as f64;
            let avg_power = total_power / matched_harmonics.len() as f64;
            
            // Estimate motor count based on signal strength
            // Stronger signal = closer or more motors
            let estimated_motors = if avg_power > -20.0 {
                4  // Very close, likely quad
            } else if avg_power > -35.0 {
                4
            } else if avg_power > -45.0 {
                2  // Could be bi-copter or distant quad
            } else {
                1
            };
            
            return Some(EmiSignature {
                fundamental_khz: signature.fundamental_khz,
                harmonics_detected: matched_harmonics,
                power_db: avg_power,
                estimated_motor_count: estimated_motors,
                esc_type: signature.esc_type.clone(),
                confidence,
                spectral_features: None,
                anomaly_detected: None,
                anomaly_z_score: None,
            });
        }
        
        None
    }
    
    /// Detect unknown harmonic series (drones with non-standard ESCs)
    fn detect_unknown_harmonics(&self, peaks: &[(u64, f64)]) -> Option<EmiSignature> {
        if peaks.len() < 4 {
            return None;
        }
        
        // Look for evenly-spaced peaks (harmonic series)
        // Take strongest peaks and check for harmonic relationship
        let strong_peaks: Vec<(u64, f64)> = peaks.iter().take(10).cloned().collect();
        
        for i in 0..strong_peaks.len() {
            let (f1, _) = strong_peaks[i];
            
            // Try this as fundamental and look for harmonics
            let mut harmonics = vec![f1 as f64 / 1000.0];
            let tolerance = self.config.harmonic_tolerance_hz as u64;
            
            for n in 2..=8u64 {
                let expected = f1 * n;
                for (freq, _) in &strong_peaks {
                    if (*freq as i64 - expected as i64).unsigned_abs() <= tolerance * n {
                        harmonics.push(*freq as f64 / 1000.0);
                        break;
                    }
                }
            }
            
            if harmonics.len() >= self.config.min_harmonics_required as usize {
                let avg_power: f64 = strong_peaks.iter()
                    .take(harmonics.len())
                    .map(|(_, p)| *p)
                    .sum::<f64>() / harmonics.len() as f64;
                
                return Some(EmiSignature {
                    fundamental_khz: f1 as f64 / 1000.0,
                    harmonics_detected: harmonics,
                    power_db: avg_power,
                    estimated_motor_count: 4,
                    esc_type: EscType::Unknown,
                    confidence: 0.5,
                    spectral_features: None,
                    anomaly_detected: None,
                    anomaly_z_score: None,
                });
            }
        }
        
        None
    }
    
    /// Get detected EMI signatures
    pub fn get_detected(&self) -> &[EmiSignature] {
        &self.detected_emi
    }
}

// ============================================================================
// COMBINED DRONE DETECTOR (RF + EMI)
// ============================================================================

/// Combined drone detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedDroneDetection {
    pub id: String,
    pub rf_signals: Vec<DroneSignal>,
    pub emi_signature: Option<EmiSignature>,
    pub combined_confidence: f64,
    pub estimated_distance_m: Option<f64>,
    pub drone_type: DroneType,
    pub threat_level: ThreatLevel,
    pub first_seen: u64,
    pub last_seen: u64,
    pub detection_method: DetectionMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ml_anomaly_baseline_samples: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetectionMethod {
    RfOnly,           // Only RF signals detected
    EmiOnly,          // Only motor EMI detected  
    RfAndEmi,         // Both RF and EMI confirmed
    Acoustic,         // Future: acoustic detection
}

/// Combined RF + EMI drone detector
pub struct CombinedDroneDetector {
    rf_detector: DroneDetector,
    emi_detector: EmiDetector,
    detections: HashMap<String, CombinedDroneDetection>,
}

impl CombinedDroneDetector {
    pub fn new(rf_config: DroneDetectorConfig, emi_config: EmiDetectorConfig) -> Self {
        Self {
            rf_detector: DroneDetector::new(rf_config),
            emi_detector: EmiDetector::new(emi_config),
            detections: HashMap::new(),
        }
    }
    
    /// Perform full scan (RF + EMI)
    pub async fn full_scan(&mut self) -> anyhow::Result<Vec<CombinedDroneDetection>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Scan RF bands
        let mut rf_signals = Vec::new();
        rf_signals.extend(self.rf_detector.scan_2_4ghz().await?);
        rf_signals.extend(self.rf_detector.scan_5_8ghz().await?);
        
        // Scan EMI
        let emi_signatures = self.emi_detector.scan_emi().await?;
        
        // Correlate detections
        let mut new_detections = Vec::new();
        
        // If we have both RF and EMI, high confidence
        if !rf_signals.is_empty() && !emi_signatures.is_empty() {
            let strongest_rf = rf_signals.iter()
                .max_by(|a, b| a.power_db.partial_cmp(&b.power_db).unwrap())
                .cloned();
            let strongest_emi = emi_signatures.first().cloned();
            
            if let (Some(rf), Some(emi)) = (strongest_rf, strongest_emi) {
                let id = format!("drone_rf_emi_{}", now);
                let detection = CombinedDroneDetection {
                    id: id.clone(),
                    rf_signals: rf_signals.clone(),
                    emi_signature: Some(emi.clone()),
                    combined_confidence: 0.95,  // High confidence with both
                    estimated_distance_m: estimate_distance(rf.power_db, emi.power_db),
                    drone_type: rf.drone_type.unwrap_or(DroneType::Unknown),
                    threat_level: ThreatLevel::High,  // Confirmed drone = elevated threat
                    first_seen: now,
                    last_seen: now,
                    detection_method: DetectionMethod::RfAndEmi,
                    ml_anomaly_baseline_samples: Some(self.emi_detector.anomaly_detector.num_baseline_samples()),
                };
                self.detections.insert(id, detection.clone());
                new_detections.push(detection);
            }
        }
        // RF only
        else if !rf_signals.is_empty() {
            for rf in &rf_signals {
                let id = format!("drone_rf_{}", rf.id);
                let detection = CombinedDroneDetection {
                    id: id.clone(),
                    rf_signals: vec![rf.clone()],
                    emi_signature: None,
                    combined_confidence: 0.7,
                    estimated_distance_m: estimate_distance_rf(rf.power_db),
                    drone_type: rf.drone_type.clone().unwrap_or(DroneType::Unknown),
                    threat_level: rf.threat_level.clone(),
                    first_seen: rf.first_seen,
                    last_seen: now,
                    detection_method: DetectionMethod::RfOnly,
                    ml_anomaly_baseline_samples: None,
                };
                self.detections.insert(id, detection.clone());
                new_detections.push(detection);
            }
        }
        // EMI only (drone may be in autonomous/GPS mode with minimal RF)
        else if !emi_signatures.is_empty() {
            for emi in &emi_signatures {
                let id = format!("drone_emi_{}khz", emi.fundamental_khz as u32);
                
                let drone_type = match emi.esc_type {
                    EscType::DjiStock => DroneType::DjiMavic,
                    EscType::Industrial => DroneType::DjiMatrice,
                    EscType::BLHeliS | EscType::BLHeli32 => DroneType::FpvRacing,
                    EscType::Unknown => DroneType::Unknown,
                };
                
                let detection = CombinedDroneDetection {
                    id: id.clone(),
                    rf_signals: vec![],
                    emi_signature: Some(emi.clone()),
                    combined_confidence: emi.confidence * 0.8,
                    estimated_distance_m: estimate_distance_emi(emi.power_db),
                    drone_type,
                    threat_level: if emi.power_db > -30.0 { ThreatLevel::High } else { ThreatLevel::Medium },
                    first_seen: now,
                    last_seen: now,
                    detection_method: DetectionMethod::EmiOnly,
                    ml_anomaly_baseline_samples: Some(self.emi_detector.anomaly_detector.num_baseline_samples()),
                };
                self.detections.insert(id, detection.clone());
                new_detections.push(detection);
            }
        }
        
        Ok(new_detections)
    }
    
    /// Get all current detections
    pub fn get_all_detections(&self) -> Vec<&CombinedDroneDetection> {
        self.detections.values().collect()
    }
    
    /// Cleanup old detections
    pub fn cleanup(&mut self, max_age_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.detections.retain(|_, d| now - d.last_seen < max_age_secs);
        self.rf_detector.cleanup_old(max_age_secs);
    }
}

/// Estimate distance based on RF signal strength (rough approximation)
fn estimate_distance_rf(power_db: f64) -> Option<f64> {
    // Free space path loss approximation for 2.4 GHz
    // Very rough estimate - actual depends on antenna, obstacles, etc.
    if power_db > -30.0 {
        Some(10.0)   // < 10m
    } else if power_db > -50.0 {
        Some(50.0)   // < 50m
    } else if power_db > -70.0 {
        Some(200.0)  // < 200m
    } else {
        Some(500.0)  // < 500m
    }
}

/// Estimate distance based on EMI signal strength
fn estimate_distance_emi(power_db: f64) -> Option<f64> {
    // EMI falls off rapidly with distance (near-field)
    if power_db > -20.0 {
        Some(5.0)    // Very close
    } else if power_db > -35.0 {
        Some(20.0)   // Close
    } else if power_db > -45.0 {
        Some(50.0)   // Medium
    } else {
        Some(100.0)  // Far (may be large industrial drone)
    }
}

/// Estimate distance using both RF and EMI
fn estimate_distance(rf_power_db: f64, emi_power_db: f64) -> Option<f64> {
    let rf_dist = estimate_distance_rf(rf_power_db)?;
    let emi_dist = estimate_distance_emi(emi_power_db)?;
    
    // EMI is more reliable for close range, RF for far
    // Weight accordingly
    Some((emi_dist * 0.6 + rf_dist * 0.4).min(rf_dist).min(emi_dist))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_drone_signatures() {
        let sigs = DroneSignature::known_signatures();
        assert!(sigs.len() >= 5);
        
        // Check we have both 2.4 and 5.8 GHz signatures
        assert!(sigs.iter().any(|s| s.center_freq_hz >= 2_400_000_000 && s.center_freq_hz <= 2_500_000_000));
        assert!(sigs.iter().any(|s| s.center_freq_hz >= 5_700_000_000 && s.center_freq_hz <= 5_900_000_000));
    }
    
    #[test]
    fn test_esc_signatures() {
        let sigs = EscPwmSignature::known_signatures();
        assert!(sigs.len() >= 5);
        
        // Check we have common ESC types
        assert!(sigs.iter().any(|s| s.esc_type == EscType::BLHeliS));
        assert!(sigs.iter().any(|s| s.esc_type == EscType::DjiStock));
        assert!(sigs.iter().any(|s| s.esc_type == EscType::Industrial));
    }
    
    #[test]
    fn test_emi_detector_config() {
        let config = EmiDetectorConfig::default();
        assert!(config.enabled);
        assert!(config.scan_range_khz.0 < config.scan_range_khz.1);
        assert!(config.min_harmonics_required >= 2);
    }
    
    #[test]
    fn test_distance_estimation() {
        // Strong signal = close
        assert!(estimate_distance_rf(-25.0).unwrap() < 20.0);
        // Weak signal = far
        assert!(estimate_distance_rf(-75.0).unwrap() > 400.0);
        
        // EMI detection
        assert!(estimate_distance_emi(-15.0).unwrap() < 10.0);
        assert!(estimate_distance_emi(-50.0).unwrap() > 50.0);
    }
}
