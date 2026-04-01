//! Technical Surveillance Countermeasures (TSCM) Module
//! 
//! Comprehensive database of surveillance device frequencies, signatures, and detection methods.
//! Based on documented TSCM research, intelligence reports, and known device specifications.
//!
//! WARNING: This module is for DEFENSIVE counter-surveillance use only.
//! Unauthorized interception of communications is illegal.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// THREAT CATEGORIES
// ============================================================================

/// Categories of surveillance threats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThreatCategory {
    // Audio Surveillance
    AudioBug,
    WirelessMicrophone,
    BodyWire,
    CarrierCurrent,     // Uses power lines
    UltrasonicBug,
    InfraredBug,
    
    // Video Surveillance
    VideoBug,
    CovertCamera,
    PinholeCamera,
    
    // Tracking
    GpsTracker,
    BumperBeeper,
    CellularTracker,
    BluetoothTracker,
    
    // Cellular Interception
    ImsiCatcher,
    Stingray,
    DirtBox,
    
    // Government/Intel
    FederalSurveillance,
    LawEnforcement,
    MilitaryTactical,
    IntelligenceService,
    
    // Covert Communications
    NumbersStation,
    SpySatellite,
    MilitaryComms,
    
    // Other
    SpreadSpectrum,
    FrequencyHopping,
    Unknown,
}

/// Threat severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// SURVEILLANCE FREQUENCY BANDS
// ============================================================================

/// Known surveillance frequency band
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveillanceBand {
    pub name: String,
    pub start_hz: u64,
    pub end_hz: u64,
    pub category: ThreatCategory,
    pub description: String,
    pub source: String,
    pub severity: ThreatSeverity,
    pub typical_power_mw: Option<f64>,
    pub modulation: Vec<String>,
}

impl SurveillanceBand {
    /// Comprehensive database of known surveillance frequencies
    /// Sources: TSCM.com, Granite Island Group, intelligence documents
    pub fn threat_database() -> Vec<Self> {
        vec![
            // ================================================================
            // CARRIER CURRENT / POWER LINE BUGS (Very common)
            // ================================================================
            Self {
                name: "Carrier Current VLF".to_string(),
                start_hz: 50_000,
                end_hz: 750_000,
                category: ThreatCategory::CarrierCurrent,
                description: "Power line carrier bugs, extremely common".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(250.0),
                modulation: vec!["AM".to_string(), "FM".to_string()],
            },
            Self {
                name: "Carrier Current HF".to_string(),
                start_hz: 300_000,
                end_hz: 50_000_000,
                category: ThreatCategory::CarrierCurrent,
                description: "AC mains antenna transmission".to_string(),
                source: "47 CFR 15.207".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(30.0),
                modulation: vec!["AM".to_string(), "FM".to_string(), "SSB".to_string()],
            },
            
            // ================================================================
            // VHF AUDIO BUGS (Most popular range)
            // ================================================================
            Self {
                name: "Ultra Low Power VHF".to_string(),
                start_hz: 25_000_000,
                end_hz: 80_000_000,
                category: ThreatCategory::AudioBug,
                description: "Ultra low power devices (micro watt)".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Medium,
                typical_power_mw: Some(0.01),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "FM Broadcast Band Bugs".to_string(),
                start_hz: 65_000_000,
                end_hz: 130_000_000,
                category: ThreatCategory::AudioBug,
                description: "Micro power Part 15 devices hiding in FM band".to_string(),
                source: "47 CFR 15.219".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(0.5),
                modulation: vec!["WFM".to_string(), "NFM".to_string()],
            },
            Self {
                name: "Body Wire Band I".to_string(),
                start_hz: 130_000_000,
                end_hz: 150_000_000,
                category: ThreatCategory::BodyWire,
                description: "Body wires and wireless microphones".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string(), "WFM".to_string()],
            },
            Self {
                name: "Body Wire Band II".to_string(),
                start_hz: 150_000_000,
                end_hz: 174_000_000,
                category: ThreatCategory::BodyWire,
                description: "Common body wire band".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "Body Wire Band III".to_string(),
                start_hz: 174_000_000,
                end_hz: 225_000_000,
                category: ThreatCategory::BodyWire,
                description: "In-band wireless microphones".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string(), "WFM".to_string()],
            },
            
            // ================================================================
            // UHF AUDIO/VIDEO BUGS
            // ================================================================
            Self {
                name: "Tactical Bug Band".to_string(),
                start_hz: 225_000_000,
                end_hz: 400_000_000,
                category: ThreatCategory::AudioBug,
                description: "Throw away bugs, beer can bugs, tactical".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(300.0),
                modulation: vec!["NFM".to_string(), "AM".to_string()],
            },
            Self {
                name: "Micro-Powered Bug Band".to_string(),
                start_hz: 290_000_000,
                end_hz: 330_000_000,
                category: ThreatCategory::AudioBug,
                description: "Cigarette butt bugs, wafer bugs".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(0.01),
                modulation: vec!["NFM".to_string(), "Spread Spectrum".to_string()],
            },
            Self {
                name: "SpyShop Popular Band".to_string(),
                start_hz: 330_000_000,
                end_hz: 440_000_000,
                category: ThreatCategory::AudioBug,
                description: "398.605, 399.030 MHz very popular spy shop frequencies".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(15.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "ISM 433 Bug Band".to_string(),
                start_hz: 430_000_000,
                end_hz: 550_000_000,
                category: ThreatCategory::AudioBug,
                description: "433.920 and 418 MHz popular for audio/video bugs".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(10.0),
                modulation: vec!["NFM".to_string(), "AM".to_string()],
            },
            
            // ================================================================
            // FEDERAL SURVEILLANCE FREQUENCIES (from TSCM.com)
            // ================================================================
            Self {
                name: "Federal Primary Band I".to_string(),
                start_hz: 25_000_000,
                end_hz: 75_000_000,
                category: ThreatCategory::FederalSurveillance,
                description: "Federal agency surveillance primary".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string(), "AM".to_string()],
            },
            Self {
                name: "Federal Primary Band II".to_string(),
                start_hz: 135_000_000,
                end_hz: 175_000_000,
                category: ThreatCategory::FederalSurveillance,
                description: "Federal agency surveillance primary".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "Federal Primary Band III".to_string(),
                start_hz: 225_000_000,
                end_hz: 440_000_000,
                category: ThreatCategory::FederalSurveillance,
                description: "Federal agency surveillance primary".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string(), "Spread Spectrum".to_string()],
            },
            Self {
                name: "Federal Microwave Band".to_string(),
                start_hz: 630_000_000,
                end_hz: 890_000_000,
                category: ThreatCategory::FederalSurveillance,
                description: "Federal microwave surveillance".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(100.0),
                modulation: vec!["Various".to_string()],
            },
            
            // ================================================================
            // BUMPER BEEPERS / GPS TRACKERS
            // ================================================================
            Self {
                name: "Bumper Beeper VHF".to_string(),
                start_hz: 25_000_000,
                end_hz: 50_000_000,
                category: ThreatCategory::BumperBeeper,
                description: "Mobile locator transmitters 38-47 MHz popular".to_string(),
                source: "47 CFR 90.19".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(100.0),
                modulation: vec!["NFM".to_string(), "Pulse".to_string()],
            },
            Self {
                name: "Bumper Beeper UHF".to_string(),
                start_hz: 135_000_000,
                end_hz: 170_000_000,
                category: ThreatCategory::BumperBeeper,
                description: "150-170 MHz very popular tracking band".to_string(),
                source: "47 CFR 90.19".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(100.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "ISM Bumper Beeper".to_string(),
                start_hz: 903_000_000,
                end_hz: 927_000_000,
                category: ThreatCategory::BumperBeeper,
                description: "ISM band vehicle trackers".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(50.0),
                modulation: vec!["Various".to_string()],
            },
            
            // ================================================================
            // VIDEO SURVEILLANCE
            // ================================================================
            Self {
                name: "Analog Video 900 MHz".to_string(),
                start_hz: 800_000_000,
                end_hz: 990_000_000,
                category: ThreatCategory::VideoBug,
                description: "902-985 MHz ISM band popular for video".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: Some(100.0),
                modulation: vec!["NTSC".to_string(), "PAL".to_string()],
            },
            Self {
                name: "Video Bug 1.2 GHz".to_string(),
                start_hz: 1_100_000_000,
                end_hz: 1_300_000_000,
                category: ThreatCategory::VideoBug,
                description: "Very popular covert video band".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(100.0),
                modulation: vec!["FM Video".to_string()],
            },
            Self {
                name: "Video Bug 2.4 GHz".to_string(),
                start_hz: 2_400_000_000,
                end_hz: 2_500_000_000,
                category: ThreatCategory::VideoBug,
                description: "EXTREMELY popular for covert video".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(200.0),
                modulation: vec!["FM Video".to_string(), "Digital".to_string()],
            },
            Self {
                name: "Video Bug 5.8 GHz".to_string(),
                start_hz: 5_600_000_000,
                end_hz: 7_500_000_000,
                category: ThreatCategory::VideoBug,
                description: "5.8-6.2 GHz becoming very popular".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(100.0),
                modulation: vec!["FM Video".to_string(), "Digital".to_string()],
            },
            
            // ================================================================
            // IMSI CATCHERS / STINGRAYS
            // ================================================================
            Self {
                name: "Cellular 700 MHz".to_string(),
                start_hz: 698_000_000,
                end_hz: 756_000_000,
                category: ThreatCategory::ImsiCatcher,
                description: "LTE Band 12/13/17 - IMSI catcher target".to_string(),
                source: "3GPP".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["LTE".to_string()],
            },
            Self {
                name: "Cellular 850 MHz".to_string(),
                start_hz: 824_000_000,
                end_hz: 894_000_000,
                category: ThreatCategory::ImsiCatcher,
                description: "Cellular Band 5/26 - common Stingray band".to_string(),
                source: "3GPP".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["GSM".to_string(), "CDMA".to_string(), "LTE".to_string()],
            },
            Self {
                name: "Cellular 1900 MHz".to_string(),
                start_hz: 1_850_000_000,
                end_hz: 1_990_000_000,
                category: ThreatCategory::ImsiCatcher,
                description: "PCS Band 2/25 - IMSI catcher target".to_string(),
                source: "3GPP".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["GSM".to_string(), "LTE".to_string()],
            },
            
            // ================================================================
            // SPECIFIC FEDERAL FREQUENCIES (from TSCM.com)
            // ================================================================
            Self {
                name: "FBI Surveillance".to_string(),
                start_hz: 164_912_500,
                end_hz: 164_912_500,
                category: ThreatCategory::FederalSurveillance,
                description: "FBI 164.9125 MHz surveillance".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(5.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "ATF Surveillance".to_string(),
                start_hz: 165_912_500,
                end_hz: 165_912_500,
                category: ThreatCategory::FederalSurveillance,
                description: "ATF F5 Surveillance 165.9125 MHz".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(5.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "DEA Surveillance".to_string(),
                start_hz: 418_000_000,
                end_hz: 419_000_000,
                category: ThreatCategory::FederalSurveillance,
                description: "DEA low power surveillance 418 MHz band".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(5.0),
                modulation: vec!["NFM".to_string()],
            },
            Self {
                name: "Secret Service/CIA".to_string(),
                start_hz: 407_800_000,
                end_hz: 407_800_000,
                category: ThreatCategory::FederalSurveillance,
                description: "Secret Service, CIA, State Department".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: Some(5.0),
                modulation: vec!["NFM".to_string()],
            },
            
            // ================================================================
            // MILITARY TACTICAL
            // ================================================================
            Self {
                name: "SINCGARS".to_string(),
                start_hz: 30_000_000,
                end_hz: 88_000_000,
                category: ThreatCategory::MilitaryTactical,
                description: "US Military tactical FM combat net radio".to_string(),
                source: "SINCGARS".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["FM".to_string(), "Frequency Hopping".to_string()],
            },
            Self {
                name: "HAVEQUICK".to_string(),
                start_hz: 225_000_000,
                end_hz: 400_000_000,
                category: ThreatCategory::MilitaryTactical,
                description: "Military air-ground frequency hopping".to_string(),
                source: "MIL-STD".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["AM".to_string(), "Frequency Hopping".to_string()],
            },
            Self {
                name: "Link 16".to_string(),
                start_hz: 969_000_000,
                end_hz: 1_206_000_000,
                category: ThreatCategory::MilitaryTactical,
                description: "Military tactical data link".to_string(),
                source: "MIL-STD-6016".to_string(),
                severity: ThreatSeverity::Critical,
                typical_power_mw: None,
                modulation: vec!["TDMA".to_string(), "Frequency Hopping".to_string()],
            },
            
            // ================================================================
            // INFRARED / OPTICAL BUGS
            // ================================================================
            Self {
                name: "Infrared Audio Bug".to_string(),
                start_hz: 850_000_000_000_000, // 850nm in Hz
                end_hz: 950_000_000_000_000,   // 950nm in Hz  
                category: ThreatCategory::InfraredBug,
                description: "880-950nm IR audio transmitters very common".to_string(),
                source: "TSCM.com".to_string(),
                severity: ThreatSeverity::High,
                typical_power_mw: None,
                modulation: vec!["IR Modulated".to_string()],
            },
        ]
    }
}

// ============================================================================
// SPECIFIC DEVICE SIGNATURES
// ============================================================================

/// Known surveillance device signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSignature {
    pub name: String,
    pub manufacturer: Option<String>,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub category: ThreatCategory,
    pub modulation: String,
    pub preamble_pattern: Option<String>,
    pub timing_ms: Option<f64>,
    pub power_mw: Option<f64>,
    pub description: String,
    pub source: String,
}

impl DeviceSignature {
    /// Database of specific known device signatures
    pub fn device_database() -> Vec<Self> {
        vec![
            // Popular SpyShop frequencies
            Self {
                name: "SpyShop Bug Type A".to_string(),
                manufacturer: None,
                frequency_hz: 398_605_000,
                bandwidth_hz: 25_000,
                category: ThreatCategory::AudioBug,
                modulation: "NFM".to_string(),
                preamble_pattern: None,
                timing_ms: None,
                power_mw: Some(15.0),
                description: "Very popular spy shop frequency".to_string(),
                source: "TSCM.com".to_string(),
            },
            Self {
                name: "SpyShop Bug Type B".to_string(),
                manufacturer: None,
                frequency_hz: 300_455_000,
                bandwidth_hz: 25_000,
                category: ThreatCategory::AudioBug,
                modulation: "NFM".to_string(),
                preamble_pattern: None,
                timing_ms: None,
                power_mw: Some(15.0),
                description: "Popular spy shop frequency".to_string(),
                source: "TSCM.com".to_string(),
            },
            Self {
                name: "SpyShop Bug Type C".to_string(),
                manufacturer: None,
                frequency_hz: 399_030_000,
                bandwidth_hz: 25_000,
                category: ThreatCategory::AudioBug,
                modulation: "NFM".to_string(),
                preamble_pattern: None,
                timing_ms: None,
                power_mw: Some(15.0),
                description: "Popular spy shop frequency".to_string(),
                source: "TSCM.com".to_string(),
            },
            // ISM Band devices
            Self {
                name: "Generic 433 MHz Bug".to_string(),
                manufacturer: None,
                frequency_hz: 433_920_000,
                bandwidth_hz: 25_000,
                category: ThreatCategory::AudioBug,
                modulation: "NFM".to_string(),
                preamble_pattern: None,
                timing_ms: None,
                power_mw: Some(10.0),
                description: "ISM band audio bug".to_string(),
                source: "Common".to_string(),
            },
            Self {
                name: "Generic 418 MHz Bug".to_string(),
                manufacturer: None,
                frequency_hz: 418_000_000,
                bandwidth_hz: 25_000,
                category: ThreatCategory::AudioBug,
                modulation: "NFM".to_string(),
                preamble_pattern: None,
                timing_ms: None,
                power_mw: Some(10.0),
                description: "Common audio bug frequency".to_string(),
                source: "Common".to_string(),
            },
        ]
    }
}

// ============================================================================
// NUMBERS STATIONS DATABASE
// ============================================================================

/// Numbers station entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumbersStation {
    pub designator: String,      // ENIGMA designator (E11, S06, M14, etc.)
    pub name: Option<String>,    // Common name if known
    pub language: String,
    pub operator: Option<String>,
    pub frequencies_khz: Vec<u32>,
    pub schedule: Option<String>,
    pub modulation: String,
    pub status: StationStatus,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StationStatus {
    Active,
    Intermittent,
    Inactive,
    Unknown,
}

impl NumbersStation {
    /// Database of known numbers stations
    /// Source: Priyom.org, ENIGMA 2000, Conet Project
    pub fn numbers_stations_database() -> Vec<Self> {
        vec![
            // English language stations
            Self {
                designator: "E11".to_string(),
                name: Some("Polish 11".to_string()),
                language: "English".to_string(),
                operator: Some("Polish Intelligence".to_string()),
                frequencies_khz: vec![4780, 5422, 6825, 8190],
                schedule: Some("Variable".to_string()),
                modulation: "USB".to_string(),
                status: StationStatus::Active,
                description: "English language mode of Polish 11 operator".to_string(),
            },
            Self {
                designator: "E03".to_string(),
                name: Some("Lincolnshire Poacher".to_string()),
                language: "English".to_string(),
                operator: Some("MI6/GCHQ".to_string()),
                frequencies_khz: vec![],
                schedule: None,
                modulation: "USB".to_string(),
                status: StationStatus::Inactive,
                description: "Famous British numbers station, ceased 2008".to_string(),
            },
            
            // Russian language stations
            Self {
                designator: "S06".to_string(),
                name: Some("Russian Man".to_string()),
                language: "Russian".to_string(),
                operator: Some("Russian Intelligence".to_string()),
                frequencies_khz: vec![4625, 5154, 6853, 7039],
                schedule: Some("Daily".to_string()),
                modulation: "USB".to_string(),
                status: StationStatus::Active,
                description: "Russian numbers station".to_string(),
            },
            
            // The Buzzer (UVB-76)
            Self {
                designator: "S28".to_string(),
                name: Some("The Buzzer (UVB-76)".to_string()),
                language: "Russian".to_string(),
                operator: Some("Russian Military".to_string()),
                frequencies_khz: vec![4625],
                schedule: Some("Continuous".to_string()),
                modulation: "USB".to_string(),
                status: StationStatus::Active,
                description: "Famous Russian buzzing station with occasional voice messages".to_string(),
            },
            
            // The Pip
            Self {
                designator: "S30".to_string(),
                name: Some("The Pip".to_string()),
                language: "Russian".to_string(),
                operator: Some("Russian Military".to_string()),
                frequencies_khz: vec![3756, 5448],
                schedule: Some("Continuous".to_string()),
                modulation: "USB".to_string(),
                status: StationStatus::Active,
                description: "Russian marker station with pip sounds".to_string(),
            },
            
            // Cuban stations
            Self {
                designator: "HM01".to_string(),
                name: Some("Hybrid Mode 01".to_string()),
                language: "Spanish".to_string(),
                operator: Some("Cuban Intelligence (DI)".to_string()),
                frequencies_khz: vec![5855, 7375, 9330, 11435],
                schedule: Some("Variable".to_string()),
                modulation: "USB + Digital".to_string(),
                status: StationStatus::Active,
                description: "Cuban intelligence, hybrid voice/digital".to_string(),
            },
            
            // Chinese stations
            Self {
                designator: "V26".to_string(),
                name: Some("New Star Radio".to_string()),
                language: "Chinese".to_string(),
                operator: Some("PLA/Chinese Intelligence".to_string()),
                frequencies_khz: vec![8300, 9725, 11430, 13750],
                schedule: Some("Daily".to_string()),
                modulation: "AM".to_string(),
                status: StationStatus::Active,
                description: "Chinese propaganda/numbers station".to_string(),
            },
            
            // Morse stations
            Self {
                designator: "M14".to_string(),
                name: None,
                language: "Morse".to_string(),
                operator: Some("Russian Military".to_string()),
                frequencies_khz: vec![5240, 6840, 8120],
                schedule: Some("Variable".to_string()),
                modulation: "CW".to_string(),
                status: StationStatus::Active,
                description: "Russian military morse numbers station".to_string(),
            },
        ]
    }
}

// ============================================================================
// MILITARY / GOVERNMENT RADIO BANDS
// ============================================================================

/// Government/Military radio allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryBand {
    pub name: String,
    pub start_hz: u64,
    pub end_hz: u64,
    pub users: Vec<String>,
    pub modulation: Vec<String>,
    pub encryption: bool,
    pub description: String,
}

impl MilitaryBand {
    pub fn military_bands() -> Vec<Self> {
        vec![
            // HF Military
            Self {
                name: "HF ALE".to_string(),
                start_hz: 2_000_000,
                end_hz: 30_000_000,
                users: vec!["USAF".to_string(), "USN".to_string(), "USCG".to_string()],
                modulation: vec!["USB".to_string(), "ALE".to_string()],
                encryption: true,
                description: "HF Automatic Link Establishment".to_string(),
            },
            
            // VHF Military
            Self {
                name: "SINCGARS VHF".to_string(),
                start_hz: 30_000_000,
                end_hz: 88_000_000,
                users: vec!["US Army".to_string(), "USMC".to_string()],
                modulation: vec!["FM".to_string(), "FHSS".to_string()],
                encryption: true,
                description: "Single Channel Ground-Air Radio System".to_string(),
            },
            
            // UHF Military Air
            Self {
                name: "Military Air UHF".to_string(),
                start_hz: 225_000_000,
                end_hz: 400_000_000,
                users: vec!["USAF".to_string(), "USN".to_string(), "NATO".to_string()],
                modulation: vec!["AM".to_string(), "HAVEQUICK".to_string()],
                encryption: true,
                description: "Military aviation communications".to_string(),
            },
            
            // Federal Law Enforcement
            Self {
                name: "Federal LE VHF".to_string(),
                start_hz: 162_000_000,
                end_hz: 174_000_000,
                users: vec!["FBI".to_string(), "DEA".to_string(), "ATF".to_string(), "USMS".to_string()],
                modulation: vec!["NFM".to_string(), "P25".to_string()],
                encryption: true,
                description: "Federal law enforcement VHF".to_string(),
            },
            
            // Federal LE UHF
            Self {
                name: "Federal LE UHF".to_string(),
                start_hz: 406_000_000,
                end_hz: 420_000_000,
                users: vec!["FBI".to_string(), "Secret Service".to_string(), "DEA".to_string()],
                modulation: vec!["NFM".to_string(), "P25".to_string()],
                encryption: true,
                description: "Federal law enforcement UHF".to_string(),
            },
            
            // 700 MHz Public Safety
            Self {
                name: "700 MHz PSBB".to_string(),
                start_hz: 758_000_000,
                end_hz: 775_000_000,
                users: vec!["FirstNet".to_string(), "Police".to_string(), "Fire".to_string(), "EMS".to_string()],
                modulation: vec!["LTE".to_string(), "P25".to_string()],
                encryption: true,
                description: "700 MHz Public Safety Broadband".to_string(),
            },
            
            // 800 MHz Public Safety
            Self {
                name: "800 MHz PS".to_string(),
                start_hz: 851_000_000,
                end_hz: 869_000_000,
                users: vec!["Police".to_string(), "Fire".to_string(), "EMS".to_string(), "Federal".to_string()],
                modulation: vec!["P25 Phase I".to_string(), "P25 Phase II".to_string()],
                encryption: true,
                description: "800 MHz trunked public safety".to_string(),
            },
        ]
    }
}

// ============================================================================
// TSCM SWEEP CONFIGURATION
// ============================================================================

/// TSCM sweep mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TscmSweepConfig {
    pub name: String,
    pub bands: Vec<(u64, u64)>,           // (start_hz, end_hz) pairs
    pub dwell_time_ms: u64,
    pub threshold_db: f64,
    pub detect_spread_spectrum: bool,
    pub detect_frequency_hopping: bool,
    pub carrier_current_check: bool,
    pub infrared_check: bool,
}

impl TscmSweepConfig {
    /// Quick sweep - basic check
    pub fn quick_sweep() -> Self {
        Self {
            name: "Quick Sweep".to_string(),
            bands: vec![
                (65_000_000, 130_000_000),     // FM band bugs
                (330_000_000, 550_000_000),    // UHF bugs
                (800_000_000, 1_000_000_000),  // Video/cellular
                (2_400_000_000, 2_500_000_000), // 2.4 GHz video
            ],
            dwell_time_ms: 100,
            threshold_db: -60.0,
            detect_spread_spectrum: false,
            detect_frequency_hopping: false,
            carrier_current_check: false,
            infrared_check: false,
        }
    }
    
    /// Standard sweep - recommended minimum
    pub fn standard_sweep() -> Self {
        Self {
            name: "Standard Sweep".to_string(),
            bands: vec![
                (100_000, 50_000_000),          // Carrier current + VLF
                (50_000_000, 500_000_000),      // VHF/UHF bugs
                (500_000_000, 3_000_000_000),   // Microwave bugs
                (5_000_000_000, 6_500_000_000), // 5-6 GHz video
            ],
            dwell_time_ms: 250,
            threshold_db: -70.0,
            detect_spread_spectrum: true,
            detect_frequency_hopping: false,
            carrier_current_check: true,
            infrared_check: false,
        }
    }
    
    /// Full TSCM sweep - professional grade
    pub fn full_sweep() -> Self {
        Self {
            name: "Full TSCM Sweep".to_string(),
            bands: vec![
                (9_000, 150_000),               // VLF
                (150_000, 50_000_000),          // Carrier current
                (50_000_000, 1_000_000_000),    // VHF/UHF
                (1_000_000_000, 6_000_000_000), // Microwave
                (6_000_000_000, 18_000_000_000), // High microwave
                (18_000_000_000, 26_500_000_000), // K-band
            ],
            dwell_time_ms: 500,
            threshold_db: -80.0,
            detect_spread_spectrum: true,
            detect_frequency_hopping: true,
            carrier_current_check: true,
            infrared_check: true,
        }
    }
    
    /// Federal/Intel threat focused
    pub fn federal_threat_sweep() -> Self {
        Self {
            name: "Federal Threat Sweep".to_string(),
            bands: vec![
                (25_000_000, 75_000_000),       // Federal primary I
                (135_000_000, 220_000_000),     // Federal primary II
                (225_000_000, 525_000_000),     // Federal primary III
                (630_000_000, 1_950_000_000),   // Federal microwave
                (1_950_000_000, 5_500_000_000), // High Federal
                (5_500_000_000, 12_500_000_000), // Very high
            ],
            dwell_time_ms: 500,
            threshold_db: -80.0,
            detect_spread_spectrum: true,
            detect_frequency_hopping: true,
            carrier_current_check: true,
            infrared_check: true,
        }
    }
}

// ============================================================================
// THREAT DETECTION RESULT
// ============================================================================

/// Detected surveillance threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetection {
    pub timestamp: u64,
    pub frequency_hz: u64,
    pub bandwidth_hz: u64,
    pub power_db: f64,
    pub category: ThreatCategory,
    pub severity: ThreatSeverity,
    pub matched_signature: Option<String>,
    pub matched_band: Option<String>,
    pub confidence: f64,
    pub description: String,
    pub recommended_action: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_threat_database() {
        let bands = SurveillanceBand::threat_database();
        assert!(bands.len() > 20);
        
        // Check we have federal surveillance bands
        assert!(bands.iter().any(|b| b.category == ThreatCategory::FederalSurveillance));
        
        // Check we have audio bugs
        assert!(bands.iter().any(|b| b.category == ThreatCategory::AudioBug));
    }
    
    #[test]
    fn test_numbers_stations() {
        let stations = NumbersStation::numbers_stations_database();
        assert!(stations.len() >= 5);
        
        // Check for The Buzzer
        assert!(stations.iter().any(|s| s.name.as_deref() == Some("The Buzzer (UVB-76)")));
    }
    
    #[test]
    fn test_sweep_configs() {
        let quick = TscmSweepConfig::quick_sweep();
        let full = TscmSweepConfig::full_sweep();
        
        // Full sweep should cover more bands
        assert!(full.bands.len() > quick.bands.len());
        
        // Full sweep should have lower threshold (more sensitive)
        assert!(full.threshold_db < quick.threshold_db);
    }
}
