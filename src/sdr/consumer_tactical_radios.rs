//! Consumer and Tactical Radio Systems Database
//!
//! Comprehensive database of affordable radio systems for:
//! - Hunting and outdoor recreation
//! - Emergency preparedness
//! - Community safety networks
//! - Field communications
//!
//! ╔══════════════════════════════════════════════════════════════════════════════╗
//! ║                           LEGAL NOTICE                                        ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
//! ║  This information is for LAWFUL radio use only.                              ║
//! ║  - FRS/MURS: No license required in USA                                      ║
//! ║  - GMRS: Requires FCC license ($35, 10 years, covers family)                ║
//! ║  - Amateur (Ham): Requires FCC license (Technician exam)                    ║
//! ║  - PMR446: License-free in Europe                                           ║
//! ║  Always comply with your country's radio regulations.                        ║
//! ╚══════════════════════════════════════════════════════════════════════════════╝

use serde::{Deserialize, Serialize};

// ============================================================================
// CONSUMER RADIO SERVICES (LICENSE-FREE)
// ============================================================================

/// License-free radio service specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseFreeService {
    pub name: String,
    pub abbreviation: String,
    pub region: String,
    pub frequency_band: String,
    pub channels: Vec<RadioChannel>,
    pub max_power_watts: f64,
    pub max_range_km: (f64, f64),  // (typical, max_optimal)
    pub license_required: bool,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub best_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioChannel {
    pub number: u8,
    pub frequency_mhz: f64,
    pub bandwidth_khz: f64,
    pub power_watts: f64,
    pub notes: String,
}

/// FRS (Family Radio Service) - USA
pub fn frs_channels() -> Vec<RadioChannel> {
    vec![
        // Channels 1-7: 2W, shared with GMRS
        RadioChannel { number: 1, frequency_mhz: 462.5625, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 2, frequency_mhz: 462.5875, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 3, frequency_mhz: 462.6125, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 4, frequency_mhz: 462.6375, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 5, frequency_mhz: 462.6625, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 6, frequency_mhz: 462.6875, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 7, frequency_mhz: 462.7125, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        // Channels 8-14: 0.5W, FRS only
        RadioChannel { number: 8, frequency_mhz: 467.5625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 9, frequency_mhz: 467.5875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 10, frequency_mhz: 467.6125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 11, frequency_mhz: 467.6375, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 12, frequency_mhz: 467.6625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 13, frequency_mhz: 467.6875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        RadioChannel { number: 14, frequency_mhz: 467.7125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "FRS only, low power".to_string() },
        // Channels 15-22: 2W, shared with GMRS
        RadioChannel { number: 15, frequency_mhz: 462.5500, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 16, frequency_mhz: 462.5750, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 17, frequency_mhz: 462.6000, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 18, frequency_mhz: 462.6250, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 19, frequency_mhz: 462.6500, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 20, frequency_mhz: 462.6750, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 21, frequency_mhz: 462.7000, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
        RadioChannel { number: 22, frequency_mhz: 462.7250, bandwidth_khz: 12.5, power_watts: 2.0, notes: "Shared with GMRS".to_string() },
    ]
}

/// MURS (Multi-Use Radio Service) - USA
pub fn murs_channels() -> Vec<RadioChannel> {
    vec![
        RadioChannel { number: 1, frequency_mhz: 151.820, bandwidth_khz: 11.25, power_watts: 2.0, notes: "VHF, best penetration".to_string() },
        RadioChannel { number: 2, frequency_mhz: 151.880, bandwidth_khz: 11.25, power_watts: 2.0, notes: "VHF, best penetration".to_string() },
        RadioChannel { number: 3, frequency_mhz: 151.940, bandwidth_khz: 11.25, power_watts: 2.0, notes: "VHF, best penetration".to_string() },
        RadioChannel { number: 4, frequency_mhz: 154.570, bandwidth_khz: 20.0, power_watts: 2.0, notes: "Blue Dot frequency".to_string() },
        RadioChannel { number: 5, frequency_mhz: 154.600, bandwidth_khz: 20.0, power_watts: 2.0, notes: "Green Dot frequency".to_string() },
    ]
}

/// PMR446 (Europe) channels
pub fn pmr446_channels() -> Vec<RadioChannel> {
    vec![
        RadioChannel { number: 1, frequency_mhz: 446.00625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 2, frequency_mhz: 446.01875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 3, frequency_mhz: 446.03125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 4, frequency_mhz: 446.04375, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 5, frequency_mhz: 446.05625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 6, frequency_mhz: 446.06875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 7, frequency_mhz: 446.08125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        RadioChannel { number: 8, frequency_mhz: 446.09375, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Analog".to_string() },
        // Digital channels 9-16
        RadioChannel { number: 9, frequency_mhz: 446.10625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital (dPMR/DMR)".to_string() },
        RadioChannel { number: 10, frequency_mhz: 446.11875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 11, frequency_mhz: 446.13125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 12, frequency_mhz: 446.14375, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 13, frequency_mhz: 446.15625, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 14, frequency_mhz: 446.16875, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 15, frequency_mhz: 446.18125, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
        RadioChannel { number: 16, frequency_mhz: 446.19375, bandwidth_khz: 12.5, power_watts: 0.5, notes: "Digital".to_string() },
    ]
}

/// License-free service comparison
pub fn license_free_services() -> Vec<LicenseFreeService> {
    vec![
        LicenseFreeService {
            name: "Family Radio Service".to_string(),
            abbreviation: "FRS".to_string(),
            region: "USA".to_string(),
            frequency_band: "462-467 MHz (UHF)".to_string(),
            channels: frs_channels(),
            max_power_watts: 2.0,
            max_range_km: (0.8, 3.0),  // 0.5-2 miles typical
            license_required: false,
            pros: vec![
                "No license required".to_string(),
                "Inexpensive radios ($20-50/pair)".to_string(),
                "22 channels".to_string(),
                "Compatible with GMRS on shared channels".to_string(),
            ],
            cons: vec![
                "Limited power (2W max)".to_string(),
                "Short range in terrain".to_string(),
                "No repeater access".to_string(),
                "Crowded in populated areas".to_string(),
            ],
            best_for: vec![
                "Families".to_string(),
                "Short-range communication".to_string(),
                "Camping".to_string(),
                "Events".to_string(),
            ],
        },
        LicenseFreeService {
            name: "Multi-Use Radio Service".to_string(),
            abbreviation: "MURS".to_string(),
            region: "USA".to_string(),
            frequency_band: "151-154 MHz (VHF)".to_string(),
            channels: murs_channels(),
            max_power_watts: 2.0,
            max_range_km: (1.5, 8.0),  // Better than FRS
            license_required: false,
            pros: vec![
                "No license required".to_string(),
                "VHF = better penetration through foliage".to_string(),
                "Less crowded than FRS".to_string(),
                "External antennas allowed".to_string(),
                "Better range in forests".to_string(),
            ],
            cons: vec![
                "Only 5 channels".to_string(),
                "Less radio selection".to_string(),
                "No repeaters".to_string(),
            ],
            best_for: vec![
                "Hunting".to_string(),
                "Farms/ranches".to_string(),
                "Wooded areas".to_string(),
                "Privacy (less users)".to_string(),
            ],
        },
        LicenseFreeService {
            name: "Private Mobile Radio 446".to_string(),
            abbreviation: "PMR446".to_string(),
            region: "Europe/UK".to_string(),
            frequency_band: "446.0-446.2 MHz (UHF)".to_string(),
            channels: pmr446_channels(),
            max_power_watts: 0.5,
            max_range_km: (0.5, 2.0),
            license_required: false,
            pros: vec![
                "License-free across EU".to_string(),
                "16 channels (8 analog + 8 digital)".to_string(),
                "Standardized across Europe".to_string(),
            ],
            cons: vec![
                "Very low power (0.5W)".to_string(),
                "Limited range".to_string(),
                "No external antennas".to_string(),
            ],
            best_for: vec![
                "European travel".to_string(),
                "Short-range comms".to_string(),
                "Business use".to_string(),
            ],
        },
    ]
}

// ============================================================================
// GMRS (LICENSED BUT AFFORDABLE)
// ============================================================================

/// GMRS channel specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmrsChannel {
    pub number: u8,
    pub frequency_mhz: f64,
    pub power_watts: f64,
    pub bandwidth_khz: f64,
    pub channel_type: GmrsChannelType,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GmrsChannelType {
    Main,       // 462 MHz simplex
    Interstitial, // 462 MHz narrow
    Repeater,   // 467 MHz (input)
}

pub fn gmrs_channels() -> Vec<GmrsChannel> {
    vec![
        // Main channels (up to 50W)
        GmrsChannel { number: 15, frequency_mhz: 462.5500, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        GmrsChannel { number: 16, frequency_mhz: 462.5750, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        GmrsChannel { number: 17, frequency_mhz: 462.6000, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        GmrsChannel { number: 18, frequency_mhz: 462.6250, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        GmrsChannel { number: 19, frequency_mhz: 462.6500, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Popular emergency".to_string() },
        GmrsChannel { number: 20, frequency_mhz: 462.6750, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "GMRS calling freq".to_string() },
        GmrsChannel { number: 21, frequency_mhz: 462.7000, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        GmrsChannel { number: 22, frequency_mhz: 462.7250, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Main, notes: "Simplex".to_string() },
        
        // Repeater output frequencies (462 MHz) / input (467 MHz)
        GmrsChannel { number: 23, frequency_mhz: 462.5500, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Repeater, notes: "Rpt 15 output, 467.5500 input".to_string() },
        GmrsChannel { number: 24, frequency_mhz: 462.5750, power_watts: 50.0, bandwidth_khz: 25.0, channel_type: GmrsChannelType::Repeater, notes: "Rpt 16 output, 467.5750 input".to_string() },
        // ... (additional repeater pairs)
    ]
}

/// GMRS radio specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmrsRadio {
    pub model: String,
    pub manufacturer: String,
    pub power_watts: f64,
    pub repeater_capable: bool,
    pub channels: u32,
    pub price_usd: String,
    pub features: Vec<String>,
    pub notes: String,
}

pub fn recommended_gmrs_radios() -> Vec<GmrsRadio> {
    vec![
        GmrsRadio {
            model: "GXT1000VP4".to_string(),
            manufacturer: "Midland".to_string(),
            power_watts: 5.0,
            repeater_capable: false,
            channels: 50,
            price_usd: "$80-120/pair".to_string(),
            features: vec![
                "NOAA weather".to_string(),
                "SOS siren".to_string(),
                "36+ mile range (marketing)".to_string(),
                "Water resistant".to_string(),
            ],
            notes: "Best seller, good build quality".to_string(),
        },
        GmrsRadio {
            model: "GXT67 Pro".to_string(),
            manufacturer: "Midland".to_string(),
            power_watts: 5.0,
            repeater_capable: true,
            channels: 50,
            price_usd: "$100-140".to_string(),
            features: vec![
                "Full 5W GMRS".to_string(),
                "Repeater capable".to_string(),
                "NOAA weather".to_string(),
                "USB-C charging".to_string(),
            ],
            notes: "Newer model with repeater support".to_string(),
        },
        GmrsRadio {
            model: "Expedition Radio".to_string(),
            manufacturer: "Rocky Talkie".to_string(),
            power_watts: 5.0,
            repeater_capable: true,
            channels: 22,
            price_usd: "$190".to_string(),
            features: vec![
                "IP67 waterproof".to_string(),
                "4-6 day battery".to_string(),
                "Rugged design".to_string(),
                "35+ mile line-of-sight".to_string(),
            ],
            notes: "Premium outdoor radio, backcountry focused".to_string(),
        },
        GmrsRadio {
            model: "GMRS-V2".to_string(),
            manufacturer: "BTECH".to_string(),
            power_watts: 5.0,
            repeater_capable: true,
            channels: 30,
            price_usd: "$60-80".to_string(),
            features: vec![
                "Programmable".to_string(),
                "Repeater capable".to_string(),
                "Dual PTT".to_string(),
                "Affordable".to_string(),
            ],
            notes: "Budget repeater-capable option".to_string(),
        },
        GmrsRadio {
            model: "MXT400".to_string(),
            manufacturer: "Midland".to_string(),
            power_watts: 40.0,
            repeater_capable: true,
            channels: 15,
            price_usd: "$200-250".to_string(),
            features: vec![
                "40W mobile".to_string(),
                "Vehicle mount".to_string(),
                "External antenna".to_string(),
                "Maximum legal power".to_string(),
            ],
            notes: "Mobile/base station, maximum range".to_string(),
        },
    ]
}

// ============================================================================
// BUDGET HAM RADIOS (REQUIRES LICENSE)
// ============================================================================

/// Budget amateur (ham) radio specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetHamRadio {
    pub model: String,
    pub manufacturer: String,
    pub frequency_range: Vec<String>,
    pub power_watts: f64,
    pub price_usd: String,
    pub features: Vec<String>,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub fcc_certified: bool,
    pub notes: String,
}

pub fn budget_ham_radios() -> Vec<BudgetHamRadio> {
    vec![
        BudgetHamRadio {
            model: "UV-5R".to_string(),
            manufacturer: "Baofeng".to_string(),
            frequency_range: vec![
                "136-174 MHz (VHF)".to_string(),
                "400-480 MHz (UHF)".to_string(),
            ],
            power_watts: 4.0,
            price_usd: "$25-35".to_string(),
            features: vec![
                "Dual band VHF/UHF".to_string(),
                "128 channels".to_string(),
                "FM radio".to_string(),
                "Flashlight".to_string(),
                "VOX".to_string(),
            ],
            pros: vec![
                "Extremely cheap".to_string(),
                "Huge accessory market".to_string(),
                "CHIRP programmable".to_string(),
                "Proven reliability".to_string(),
            ],
            cons: vec![
                "Poor filtering (spurious emissions)".to_string(),
                "Build quality varies".to_string(),
                "Not FCC Part 90 certified".to_string(),
            ],
            fcc_certified: false,
            notes: "Most popular budget radio worldwide. Legal for ham use only.".to_string(),
        },
        BudgetHamRadio {
            model: "UV-K5".to_string(),
            manufacturer: "Quansheng".to_string(),
            frequency_range: vec![
                "136-174 MHz (VHF)".to_string(),
                "400-470 MHz (UHF)".to_string(),
                "18-620 MHz (RX only)".to_string(),
                "Airband AM (RX)".to_string(),
            ],
            power_watts: 5.0,
            price_usd: "$30-45".to_string(),
            features: vec![
                "Wideband RX".to_string(),
                "Spectrum analyzer".to_string(),
                "Airband receive".to_string(),
                "USB-C charging".to_string(),
                "Open-source firmware".to_string(),
                "SI4732 chip (SDR-like)".to_string(),
            ],
            pros: vec![
                "Excellent receiver".to_string(),
                "Custom firmware support".to_string(),
                "Spectrum display".to_string(),
                "Modern features".to_string(),
                "Better filtering than UV-5R".to_string(),
            ],
            cons: vec![
                "Complex menus".to_string(),
                "Learning curve".to_string(),
                "Not FCC Part 90 certified".to_string(),
            ],
            fcc_certified: false,
            notes: "New favorite among hams. Custom firmware (Egzumer) adds many features.".to_string(),
        },
        BudgetHamRadio {
            model: "UV-5G".to_string(),
            manufacturer: "Baofeng".to_string(),
            frequency_range: vec![
                "462-467 MHz (GMRS)".to_string(),
            ],
            power_watts: 5.0,
            price_usd: "$35-50".to_string(),
            features: vec![
                "GMRS only".to_string(),
                "FCC Part 95E certified".to_string(),
                "22 channels".to_string(),
                "Legal GMRS use".to_string(),
            ],
            pros: vec![
                "FCC certified".to_string(),
                "Legal with GMRS license".to_string(),
                "Simple operation".to_string(),
            ],
            cons: vec![
                "GMRS only (no ham bands)".to_string(),
                "Basic features".to_string(),
            ],
            fcc_certified: true,
            notes: "Legal GMRS radio from Baofeng. Good for licensed GMRS users.".to_string(),
        },
        BudgetHamRadio {
            model: "TD-H3".to_string(),
            manufacturer: "TidRadio".to_string(),
            frequency_range: vec![
                "136-174 MHz (VHF)".to_string(),
                "400-480 MHz (UHF)".to_string(),
            ],
            power_watts: 10.0,
            price_usd: "$40-60".to_string(),
            features: vec![
                "10W output".to_string(),
                "Dual band".to_string(),
                "USB-C".to_string(),
                "App programmable".to_string(),
            ],
            pros: vec![
                "Higher power".to_string(),
                "Modern interface".to_string(),
                "Good build quality".to_string(),
            ],
            cons: vec![
                "Not FCC Part 90".to_string(),
                "Less community support".to_string(),
            ],
            fcc_certified: false,
            notes: "Higher power alternative to UV-5R.".to_string(),
        },
    ]
}

// ============================================================================
// TACTICAL HEADSETS AND PTT SYSTEMS
// ============================================================================

/// Tactical headset specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalHeadset {
    pub model: String,
    pub manufacturer: String,
    pub headset_type: String,
    pub hearing_protection_nrr: u32,
    pub talk_through: bool,
    pub radio_compatible: Vec<String>,
    pub price_usd: String,
    pub features: Vec<String>,
    pub notes: String,
}

pub fn tactical_headsets() -> Vec<TacticalHeadset> {
    vec![
        TacticalHeadset {
            model: "ComTac V".to_string(),
            manufacturer: "3M Peltor".to_string(),
            headset_type: "Over-ear, boom mic".to_string(),
            hearing_protection_nrr: 23,
            talk_through: true,
            radio_compatible: vec![
                "Any with appropriate PTT adapter".to_string(),
                "Baofeng (with FL5035 PTT)".to_string(),
                "Motorola".to_string(),
                "Kenwood".to_string(),
            ],
            price_usd: "$700-900".to_string(),
            features: vec![
                "Environmental listening".to_string(),
                "Gel ear cushions".to_string(),
                "Single/dual comm options".to_string(),
                "Military grade".to_string(),
            ],
            notes: "Gold standard tactical headset. Used by military worldwide.".to_string(),
        },
        TacticalHeadset {
            model: "ComTac VIII".to_string(),
            manufacturer: "3M Peltor".to_string(),
            headset_type: "Over-ear".to_string(),
            hearing_protection_nrr: 23,
            talk_through: true,
            radio_compatible: vec!["Universal with PTT".to_string()],
            price_usd: "$900-1200".to_string(),
            features: vec![
                "Bluetooth".to_string(),
                "Improved audio".to_string(),
                "Next-gen platform".to_string(),
            ],
            notes: "Latest generation ComTac. Premium features.".to_string(),
        },
        TacticalHeadset {
            model: "Liberator HP 2.0".to_string(),
            manufacturer: "Safariland / TEA".to_string(),
            headset_type: "Over-ear".to_string(),
            hearing_protection_nrr: 26,
            talk_through: true,
            radio_compatible: vec!["Universal with PTT".to_string()],
            price_usd: "$400-600".to_string(),
            features: vec![
                "High NRR".to_string(),
                "360 degree audio".to_string(),
                "Comfortable fit".to_string(),
            ],
            notes: "Good mid-range option.".to_string(),
        },
        TacticalHeadset {
            model: "Tactical Sport 500".to_string(),
            manufacturer: "Howard Leight / Honeywell".to_string(),
            headset_type: "Over-ear".to_string(),
            hearing_protection_nrr: 26,
            talk_through: true,
            radio_compatible: vec!["3.5mm aux input".to_string()],
            price_usd: "$80-120".to_string(),
            features: vec![
                "Electronic hearing protection".to_string(),
                "Directional microphones".to_string(),
                "Budget friendly".to_string(),
            ],
            notes: "Budget electronic ear pro with aux input for radio.".to_string(),
        },
        TacticalHeadset {
            model: "Walker Razor Slim".to_string(),
            manufacturer: "Walker's".to_string(),
            headset_type: "Over-ear, low profile".to_string(),
            hearing_protection_nrr: 23,
            talk_through: true,
            radio_compatible: vec!["3.5mm aux".to_string(), "Bluetooth models".to_string()],
            price_usd: "$50-80".to_string(),
            features: vec![
                "Low profile".to_string(),
                "Sound activated compression".to_string(),
                "Budget friendly".to_string(),
                "Bluetooth option".to_string(),
            ],
            notes: "Popular budget hearing protection. Good for hunting.".to_string(),
        },
    ]
}

/// PTT (Push-to-Talk) adapter specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PttAdapter {
    pub model: String,
    pub manufacturer: String,
    pub headset_connection: String,
    pub radio_connection: String,
    pub compatible_radios: Vec<String>,
    pub price_usd: String,
    pub notes: String,
}

pub fn ptt_adapters() -> Vec<PttAdapter> {
    vec![
        PttAdapter {
            model: "FL5035-02 (K1 Plug)".to_string(),
            manufacturer: "3M Peltor".to_string(),
            headset_connection: "Peltor J11/NATO".to_string(),
            radio_connection: "Kenwood 2-pin (K1)".to_string(),
            compatible_radios: vec![
                "Baofeng UV-5R/UV-82".to_string(),
                "Quansheng UV-K5".to_string(),
                "Kenwood TK series".to_string(),
                "Wouxun".to_string(),
            ],
            price_usd: "$60-100".to_string(),
            notes: "Most common PTT for budget radios + Peltor headsets.".to_string(),
        },
        PttAdapter {
            model: "U94 PTT".to_string(),
            manufacturer: "Various (clones)".to_string(),
            headset_connection: "NATO/U174".to_string(),
            radio_connection: "K1/K2 plug options".to_string(),
            compatible_radios: vec![
                "Baofeng".to_string(),
                "Kenwood".to_string(),
                "Various".to_string(),
            ],
            price_usd: "$15-40".to_string(),
            notes: "Budget military-style PTT. Quality varies.".to_string(),
        },
        PttAdapter {
            model: "Nexus PTT".to_string(),
            manufacturer: "TEA / Safariland".to_string(),
            headset_connection: "Nexus TP-120".to_string(),
            radio_connection: "Various".to_string(),
            compatible_radios: vec!["Multiple".to_string()],
            price_usd: "$80-150".to_string(),
            notes: "Quality PTT for civilian tactical setups.".to_string(),
        },
    ]
}

// ============================================================================
// HUNTING AND OUTDOOR RADIO RECOMMENDATIONS
// ============================================================================

/// Hunting/outdoor radio recommendation by scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdoorRadioScenario {
    pub scenario: String,
    pub terrain: String,
    pub range_needed: String,
    pub recommended_service: String,
    pub recommended_radios: Vec<String>,
    pub recommended_channels: Vec<String>,
    pub tips: Vec<String>,
}

pub fn outdoor_scenarios() -> Vec<OutdoorRadioScenario> {
    vec![
        OutdoorRadioScenario {
            scenario: "Dense forest hunting".to_string(),
            terrain: "Heavy tree cover, hills".to_string(),
            range_needed: "1-5 miles".to_string(),
            recommended_service: "MURS (VHF penetrates foliage better)".to_string(),
            recommended_radios: vec![
                "Retevis RT27V (MURS)".to_string(),
                "Dakota Alert M538-HT".to_string(),
            ],
            recommended_channels: vec![
                "MURS 1: 151.820 MHz".to_string(),
                "MURS 2: 151.880 MHz".to_string(),
            ],
            tips: vec![
                "VHF works better through trees than UHF".to_string(),
                "Get to high ground for better range".to_string(),
                "External antenna improves performance".to_string(),
            ],
        },
        OutdoorRadioScenario {
            scenario: "Open terrain / mountains".to_string(),
            terrain: "Line of sight possible".to_string(),
            range_needed: "5-20+ miles".to_string(),
            recommended_service: "GMRS (higher power, repeaters)".to_string(),
            recommended_radios: vec![
                "Rocky Talkie Expedition".to_string(),
                "Midland MXT400 (mobile)".to_string(),
            ],
            recommended_channels: vec![
                "GMRS 20: 462.675 MHz (calling)".to_string(),
                "GMRS 19: 462.650 MHz (popular)".to_string(),
            ],
            tips: vec![
                "Line of sight = great GMRS range".to_string(),
                "Use repeaters for extended range".to_string(),
                "Mobile radios (40W) reach far".to_string(),
            ],
        },
        OutdoorRadioScenario {
            scenario: "Family camping / casual".to_string(),
            terrain: "Campground, trails".to_string(),
            range_needed: "< 1 mile".to_string(),
            recommended_service: "FRS (no license needed)".to_string(),
            recommended_radios: vec![
                "Motorola T100".to_string(),
                "Midland X-Talker T71".to_string(),
            ],
            recommended_channels: vec![
                "FRS 1: 462.5625 MHz".to_string(),
                "Any of 22 FRS channels".to_string(),
            ],
            tips: vec![
                "Use privacy codes (CTCSS/DCS)".to_string(),
                "Keep expectations realistic on range".to_string(),
                "Rechargeable batteries save money".to_string(),
            ],
        },
        OutdoorRadioScenario {
            scenario: "Emergency preparedness".to_string(),
            terrain: "Various".to_string(),
            range_needed: "Maximum possible".to_string(),
            recommended_service: "Ham radio (with license) + GMRS".to_string(),
            recommended_radios: vec![
                "Baofeng UV-5R (ham license)".to_string(),
                "Quansheng UV-K5 (ham license)".to_string(),
                "Midland MXT275 (GMRS)".to_string(),
            ],
            recommended_channels: vec![
                "146.520 MHz (2m calling)".to_string(),
                "446.000 MHz (70cm calling)".to_string(),
                "GMRS 20: 462.675 MHz".to_string(),
            ],
            tips: vec![
                "Get ham license for maximum capability".to_string(),
                "Program NOAA weather channels".to_string(),
                "Have multiple power sources".to_string(),
                "Join local repeater networks".to_string(),
            ],
        },
    ]
}

// ============================================================================
// FREQUENCY QUICK REFERENCE
// ============================================================================

/// Quick reference for common frequencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyQuickRef {
    pub category: String,
    pub frequencies: Vec<QuickRefEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickRefEntry {
    pub name: String,
    pub frequency_mhz: f64,
    pub mode: String,
    pub notes: String,
}

pub fn frequency_quick_reference() -> Vec<FrequencyQuickRef> {
    vec![
        FrequencyQuickRef {
            category: "Emergency / Calling".to_string(),
            frequencies: vec![
                QuickRefEntry { name: "2m National Calling".to_string(), frequency_mhz: 146.520, mode: "FM".to_string(), notes: "Ham license required".to_string() },
                QuickRefEntry { name: "70cm National Calling".to_string(), frequency_mhz: 446.000, mode: "FM".to_string(), notes: "Ham license required".to_string() },
                QuickRefEntry { name: "GMRS Calling".to_string(), frequency_mhz: 462.675, mode: "FM".to_string(), notes: "GMRS license required".to_string() },
                QuickRefEntry { name: "Marine Distress".to_string(), frequency_mhz: 156.800, mode: "FM".to_string(), notes: "Channel 16, emergency only".to_string() },
            ],
        },
        FrequencyQuickRef {
            category: "NOAA Weather".to_string(),
            frequencies: vec![
                QuickRefEntry { name: "WX1".to_string(), frequency_mhz: 162.550, mode: "FM".to_string(), notes: "Most common".to_string() },
                QuickRefEntry { name: "WX2".to_string(), frequency_mhz: 162.400, mode: "FM".to_string(), notes: "Common".to_string() },
                QuickRefEntry { name: "WX3".to_string(), frequency_mhz: 162.475, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "WX4".to_string(), frequency_mhz: 162.425, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "WX5".to_string(), frequency_mhz: 162.450, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "WX6".to_string(), frequency_mhz: 162.500, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "WX7".to_string(), frequency_mhz: 162.525, mode: "FM".to_string(), notes: "".to_string() },
            ],
        },
        FrequencyQuickRef {
            category: "FRS Popular".to_string(),
            frequencies: vec![
                QuickRefEntry { name: "FRS 1".to_string(), frequency_mhz: 462.5625, mode: "FM".to_string(), notes: "Often busy".to_string() },
                QuickRefEntry { name: "FRS 3".to_string(), frequency_mhz: 462.6125, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "FRS 7".to_string(), frequency_mhz: 462.7125, mode: "FM".to_string(), notes: "".to_string() },
            ],
        },
        FrequencyQuickRef {
            category: "MURS".to_string(),
            frequencies: vec![
                QuickRefEntry { name: "MURS 1".to_string(), frequency_mhz: 151.820, mode: "FM".to_string(), notes: "VHF, good forest penetration".to_string() },
                QuickRefEntry { name: "MURS 2".to_string(), frequency_mhz: 151.880, mode: "FM".to_string(), notes: "".to_string() },
                QuickRefEntry { name: "MURS 3".to_string(), frequency_mhz: 151.940, mode: "FM".to_string(), notes: "".to_string() },
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frs_channels() {
        let channels = frs_channels();
        assert_eq!(channels.len(), 22);
        assert_eq!(channels[0].frequency_mhz, 462.5625);
    }
    
    #[test]
    fn test_murs_channels() {
        let channels = murs_channels();
        assert_eq!(channels.len(), 5);
        assert!(channels[0].frequency_mhz > 151.0);
    }
    
    #[test]
    fn test_budget_radios() {
        let radios = budget_ham_radios();
        assert!(radios.iter().any(|r| r.model == "UV-5R"));
        assert!(radios.iter().any(|r| r.model == "UV-K5"));
    }
}
