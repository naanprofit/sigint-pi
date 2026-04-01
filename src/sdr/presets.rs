//! SDR Frequency Presets and Station Management
//! 
//! Allows users to save and load frequency presets for:
//! - Radio stations (FM, AM, shortwave)
//! - Scanner channels
//! - Numbers stations
//! - Emergency frequencies
//! - Custom frequency lists

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A single frequency preset/station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyPreset {
    pub id: String,
    pub name: String,
    pub frequency_hz: u64,
    pub modulation: Modulation,
    pub bandwidth_hz: Option<u32>,
    pub category: PresetCategory,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub squelch: Option<i32>,
    pub gain: Option<i32>,
    pub favorite: bool,
    pub last_used: Option<u64>,
    pub notes: Option<String>,
}

/// Radio modulation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Modulation {
    AM,
    FM,
    WFM,       // Wideband FM (broadcast)
    NFM,       // Narrowband FM
    USB,       // Upper sideband
    LSB,       // Lower sideband
    CW,        // Morse code
    RAW,       // Raw IQ
    Unknown,
}

impl Modulation {
    pub fn to_rtl_fm_arg(&self) -> &'static str {
        match self {
            Modulation::AM => "am",
            Modulation::FM | Modulation::NFM => "fm",
            Modulation::WFM => "wbfm",
            Modulation::USB => "usb",
            Modulation::LSB => "lsb",
            Modulation::CW => "cw",
            Modulation::RAW => "raw",
            Modulation::Unknown => "fm",
        }
    }
}

/// Preset categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresetCategory {
    // Broadcast
    FmBroadcast,
    AmBroadcast,
    Shortwave,
    
    // Emergency
    NoaaWeather,
    EmergencyServices,
    Ems,
    Fire,
    Police,
    
    // Aviation
    AirBand,
    Atis,
    
    // Maritime
    MarineVhf,
    
    // Amateur Radio
    HamHf,
    HamVhf,
    HamUhf,
    
    // Intelligence
    NumbersStation,
    MilitaryHf,
    
    // ISM/IoT
    IsmBand,
    
    // Cellular
    Cellular,
    
    // Custom
    Custom,
}

impl PresetCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::FmBroadcast => "FM Radio",
            Self::AmBroadcast => "AM Radio",
            Self::Shortwave => "Shortwave",
            Self::NoaaWeather => "NOAA Weather",
            Self::EmergencyServices => "Emergency",
            Self::Ems => "EMS",
            Self::Fire => "Fire",
            Self::Police => "Police",
            Self::AirBand => "Aviation",
            Self::Atis => "ATIS",
            Self::MarineVhf => "Marine VHF",
            Self::HamHf => "Ham HF",
            Self::HamVhf => "Ham VHF",
            Self::HamUhf => "Ham UHF",
            Self::NumbersStation => "Numbers Station",
            Self::MilitaryHf => "Military HF",
            Self::IsmBand => "ISM Band",
            Self::Cellular => "Cellular",
            Self::Custom => "Custom",
        }
    }
}

/// A preset list (collection of presets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetList {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub presets: Vec<FrequencyPreset>,
    pub created: u64,
    pub modified: u64,
    pub is_builtin: bool,
}

/// Preset manager handles loading/saving presets
pub struct PresetManager {
    presets_dir: String,
    lists: HashMap<String, PresetList>,
}

impl PresetManager {
    pub fn new(presets_dir: &str) -> Self {
        let mut manager = Self {
            presets_dir: presets_dir.to_string(),
            lists: HashMap::new(),
        };
        manager.load_builtin_presets();
        manager.load_user_presets();
        manager
    }
    
    /// Load built-in preset lists
    fn load_builtin_presets(&mut self) {
        // NOAA Weather Radio
        self.lists.insert("noaa".to_string(), PresetList {
            id: "noaa".to_string(),
            name: "NOAA Weather Radio".to_string(),
            description: Some("US NOAA Weather Radio frequencies".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "noaa_wx1".to_string(),
                    name: "WX1 - 162.550".to_string(),
                    frequency_hz: 162_550_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 1".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx2".to_string(),
                    name: "WX2 - 162.400".to_string(),
                    frequency_hz: 162_400_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 2".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx3".to_string(),
                    name: "WX3 - 162.475".to_string(),
                    frequency_hz: 162_475_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 3".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx4".to_string(),
                    name: "WX4 - 162.425".to_string(),
                    frequency_hz: 162_425_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 4".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx5".to_string(),
                    name: "WX5 - 162.450".to_string(),
                    frequency_hz: 162_450_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 5".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx6".to_string(),
                    name: "WX6 - 162.500".to_string(),
                    frequency_hz: 162_500_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 6".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "noaa_wx7".to_string(),
                    name: "WX7 - 162.525".to_string(),
                    frequency_hz: 162_525_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::NoaaWeather,
                    description: Some("NOAA Weather Channel 7".to_string()),
                    tags: vec!["weather".to_string(), "noaa".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
        
        // Marine VHF
        self.lists.insert("marine".to_string(), PresetList {
            id: "marine".to_string(),
            name: "Marine VHF".to_string(),
            description: Some("International Marine VHF channels".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "marine_16".to_string(),
                    name: "Ch 16 - Distress".to_string(),
                    frequency_hz: 156_800_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::MarineVhf,
                    description: Some("International distress, safety, calling".to_string()),
                    tags: vec!["marine".to_string(), "distress".to_string(), "emergency".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "marine_13".to_string(),
                    name: "Ch 13 - Bridge to Bridge".to_string(),
                    frequency_hz: 156_650_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::MarineVhf,
                    description: Some("Navigation safety".to_string()),
                    tags: vec!["marine".to_string(), "navigation".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "marine_22a".to_string(),
                    name: "Ch 22A - Coast Guard".to_string(),
                    frequency_hz: 157_100_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::MarineVhf,
                    description: Some("US Coast Guard liaison".to_string()),
                    tags: vec!["marine".to_string(), "coast_guard".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
        
        // Air Band
        self.lists.insert("airband".to_string(), PresetList {
            id: "airband".to_string(),
            name: "Aviation".to_string(),
            description: Some("Aviation VHF frequencies".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "air_guard".to_string(),
                    name: "121.5 - Guard/Emergency".to_string(),
                    frequency_hz: 121_500_000,
                    modulation: Modulation::AM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::AirBand,
                    description: Some("International aviation emergency frequency".to_string()),
                    tags: vec!["aviation".to_string(), "emergency".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "air_unicom".to_string(),
                    name: "122.8 - UNICOM".to_string(),
                    frequency_hz: 122_800_000,
                    modulation: Modulation::AM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::AirBand,
                    description: Some("Common UNICOM/CTAF".to_string()),
                    tags: vec!["aviation".to_string(), "unicom".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "air_multicom".to_string(),
                    name: "122.9 - Multicom".to_string(),
                    frequency_hz: 122_900_000,
                    modulation: Modulation::AM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::AirBand,
                    description: Some("Self-announce at uncontrolled airports".to_string()),
                    tags: vec!["aviation".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
        
        // Numbers Stations (from TSCM module)
        self.lists.insert("numbers".to_string(), PresetList {
            id: "numbers".to_string(),
            name: "Numbers Stations".to_string(),
            description: Some("Known numbers station frequencies (shortwave)".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "uvb76".to_string(),
                    name: "UVB-76 The Buzzer".to_string(),
                    frequency_hz: 4_625_000,
                    modulation: Modulation::USB,
                    bandwidth_hz: Some(3_000),
                    category: PresetCategory::NumbersStation,
                    description: Some("Russian military buzzer station".to_string()),
                    tags: vec!["numbers".to_string(), "russian".to_string(), "buzzer".to_string()],
                    squelch: None,
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: Some("Active 24/7, occasional voice messages".to_string()),
                },
                FrequencyPreset {
                    id: "pip".to_string(),
                    name: "The Pip".to_string(),
                    frequency_hz: 5_448_000,
                    modulation: Modulation::USB,
                    bandwidth_hz: Some(3_000),
                    category: PresetCategory::NumbersStation,
                    description: Some("Russian military pip station".to_string()),
                    tags: vec!["numbers".to_string(), "russian".to_string()],
                    squelch: None,
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "hm01_1".to_string(),
                    name: "HM01 Cuban".to_string(),
                    frequency_hz: 7_375_000,
                    modulation: Modulation::USB,
                    bandwidth_hz: Some(3_000),
                    category: PresetCategory::NumbersStation,
                    description: Some("Cuban intelligence hybrid mode".to_string()),
                    tags: vec!["numbers".to_string(), "cuban".to_string()],
                    squelch: None,
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
        
        // Ham Radio Bands
        self.lists.insert("ham".to_string(), PresetList {
            id: "ham".to_string(),
            name: "Amateur Radio".to_string(),
            description: Some("Common ham radio frequencies and calling frequencies".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "ham_2m_call".to_string(),
                    name: "2m Calling - 146.520".to_string(),
                    frequency_hz: 146_520_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(15_000),
                    category: PresetCategory::HamVhf,
                    description: Some("2m FM calling frequency".to_string()),
                    tags: vec!["ham".to_string(), "2m".to_string(), "calling".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "ham_70cm_call".to_string(),
                    name: "70cm Calling - 446.000".to_string(),
                    frequency_hz: 446_000_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(15_000),
                    category: PresetCategory::HamUhf,
                    description: Some("70cm FM calling frequency".to_string()),
                    tags: vec!["ham".to_string(), "70cm".to_string(), "calling".to_string()],
                    squelch: Some(-40),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "ham_20m".to_string(),
                    name: "20m SSB - 14.300".to_string(),
                    frequency_hz: 14_300_000,
                    modulation: Modulation::USB,
                    bandwidth_hz: Some(3_000),
                    category: PresetCategory::HamHf,
                    description: Some("20m DX/calling area".to_string()),
                    tags: vec!["ham".to_string(), "20m".to_string(), "hf".to_string()],
                    squelch: None,
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "ham_40m".to_string(),
                    name: "40m SSB - 7.200".to_string(),
                    frequency_hz: 7_200_000,
                    modulation: Modulation::LSB,
                    bandwidth_hz: Some(3_000),
                    category: PresetCategory::HamHf,
                    description: Some("40m phone band".to_string()),
                    tags: vec!["ham".to_string(), "40m".to_string(), "hf".to_string()],
                    squelch: None,
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
        
        // International Emergency Frequencies
        self.lists.insert("emergency".to_string(), PresetList {
            id: "emergency".to_string(),
            name: "Emergency Services".to_string(),
            description: Some("International emergency and distress frequencies".to_string()),
            presets: vec![
                FrequencyPreset {
                    id: "em_406".to_string(),
                    name: "406 MHz EPIRB/PLB".to_string(),
                    frequency_hz: 406_025_000,
                    modulation: Modulation::NFM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::EmergencyServices,
                    description: Some("Satellite distress beacon frequency".to_string()),
                    tags: vec!["emergency".to_string(), "epirb".to_string(), "plb".to_string()],
                    squelch: Some(-50),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: Some("COSPAS-SARSAT distress beacons".to_string()),
                },
                FrequencyPreset {
                    id: "em_121_5".to_string(),
                    name: "121.5 MHz ELT".to_string(),
                    frequency_hz: 121_500_000,
                    modulation: Modulation::AM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::EmergencyServices,
                    description: Some("Aviation emergency/ELT beacon".to_string()),
                    tags: vec!["emergency".to_string(), "aviation".to_string(), "elt".to_string()],
                    squelch: Some(-50),
                    gain: None,
                    favorite: true,
                    last_used: None,
                    notes: None,
                },
                FrequencyPreset {
                    id: "em_243".to_string(),
                    name: "243 MHz Military Guard".to_string(),
                    frequency_hz: 243_000_000,
                    modulation: Modulation::AM,
                    bandwidth_hz: Some(25_000),
                    category: PresetCategory::EmergencyServices,
                    description: Some("Military aviation emergency".to_string()),
                    tags: vec!["emergency".to_string(), "military".to_string()],
                    squelch: Some(-50),
                    gain: None,
                    favorite: false,
                    last_used: None,
                    notes: None,
                },
            ],
            created: 0,
            modified: 0,
            is_builtin: true,
        });
    }
    
    /// Load user presets from disk
    fn load_user_presets(&mut self) {
        let path = Path::new(&self.presets_dir);
        if !path.exists() {
            let _ = fs::create_dir_all(path);
            return;
        }
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().map_or(false, |e| e == "json") {
                    if let Ok(content) = fs::read_to_string(&file_path) {
                        if let Ok(list) = serde_json::from_str::<PresetList>(&content) {
                            if !list.is_builtin {
                                self.lists.insert(list.id.clone(), list);
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Get all preset lists
    pub fn get_all_lists(&self) -> Vec<&PresetList> {
        self.lists.values().collect()
    }
    
    /// Get a specific preset list
    pub fn get_list(&self, id: &str) -> Option<&PresetList> {
        self.lists.get(id)
    }
    
    /// Create a new preset list
    pub fn create_list(&mut self, name: &str, description: Option<String>) -> PresetList {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let id = format!("user_{}", now);
        let list = PresetList {
            id: id.clone(),
            name: name.to_string(),
            description,
            presets: Vec::new(),
            created: now,
            modified: now,
            is_builtin: false,
        };
        
        self.lists.insert(id.clone(), list.clone());
        self.save_list(&id);
        list
    }
    
    /// Add a preset to a list
    pub fn add_preset(&mut self, list_id: &str, preset: FrequencyPreset) -> Result<(), String> {
        let list = self.lists.get_mut(list_id).ok_or("List not found")?;
        if list.is_builtin {
            return Err("Cannot modify built-in list".to_string());
        }
        
        list.presets.push(preset);
        list.modified = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.save_list(list_id);
        Ok(())
    }
    
    /// Remove a preset from a list
    pub fn remove_preset(&mut self, list_id: &str, preset_id: &str) -> Result<(), String> {
        let list = self.lists.get_mut(list_id).ok_or("List not found")?;
        if list.is_builtin {
            return Err("Cannot modify built-in list".to_string());
        }
        
        list.presets.retain(|p| p.id != preset_id);
        list.modified = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.save_list(list_id);
        Ok(())
    }
    
    /// Delete a preset list
    pub fn delete_list(&mut self, list_id: &str) -> Result<(), String> {
        let list = self.lists.get(list_id).ok_or("List not found")?;
        if list.is_builtin {
            return Err("Cannot delete built-in list".to_string());
        }
        
        self.lists.remove(list_id);
        
        let file_path = Path::new(&self.presets_dir).join(format!("{}.json", list_id));
        let _ = fs::remove_file(file_path);
        
        Ok(())
    }
    
    /// Save a list to disk
    fn save_list(&self, list_id: &str) {
        if let Some(list) = self.lists.get(list_id) {
            if !list.is_builtin {
                let file_path = Path::new(&self.presets_dir).join(format!("{}.json", list_id));
                if let Ok(content) = serde_json::to_string_pretty(list) {
                    let _ = fs::write(file_path, content);
                }
            }
        }
    }
    
    /// Search presets by name or tag
    pub fn search(&self, query: &str) -> Vec<&FrequencyPreset> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for list in self.lists.values() {
            for preset in &list.presets {
                if preset.name.to_lowercase().contains(&query_lower)
                    || preset.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
                    || preset.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
                {
                    results.push(preset);
                }
            }
        }
        
        results
    }
    
    /// Get presets by category
    pub fn get_by_category(&self, category: PresetCategory) -> Vec<&FrequencyPreset> {
        let mut results = Vec::new();
        
        for list in self.lists.values() {
            for preset in &list.presets {
                if preset.category == category {
                    results.push(preset);
                }
            }
        }
        
        results
    }
    
    /// Get favorite presets
    pub fn get_favorites(&self) -> Vec<&FrequencyPreset> {
        let mut results = Vec::new();
        
        for list in self.lists.values() {
            for preset in &list.presets {
                if preset.favorite {
                    results.push(preset);
                }
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preset_manager() {
        let manager = PresetManager::new("/tmp/sigint-presets-test");
        
        // Check built-in lists are loaded
        assert!(manager.get_list("noaa").is_some());
        assert!(manager.get_list("marine").is_some());
        assert!(manager.get_list("numbers").is_some());
        
        // Check NOAA presets
        let noaa = manager.get_list("noaa").unwrap();
        assert_eq!(noaa.presets.len(), 7);
        
        // Test search
        let results = manager.search("weather");
        assert!(!results.is_empty());
    }
    
    #[test]
    fn test_modulation() {
        assert_eq!(Modulation::WFM.to_rtl_fm_arg(), "wbfm");
        assert_eq!(Modulation::USB.to_rtl_fm_arg(), "usb");
    }
}
