//! Government Energy Weapons and Signal Detection Module
//!
//! ╔══════════════════════════════════════════════════════════════════════════════╗
//! ║                        IMPORTANT LEGAL DISCLAIMER                             ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
//! ║                                                                              ║
//! ║  THIS SOFTWARE IS PROVIDED FOR LAWFUL, DEFENSIVE, AND EDUCATIONAL           ║
//! ║  PURPOSES ONLY.                                                              ║
//! ║                                                                              ║
//! ║  BY USING THIS SOFTWARE, YOU ACKNOWLEDGE AND AGREE TO THE FOLLOWING:        ║
//! ║                                                                              ║
//! ║  1. COMPLIANCE WITH ALL APPLICABLE LAWS                                      ║
//! ║     You shall comply with all applicable federal, state, local, and         ║
//! ║     international laws and regulations, including but not limited to:       ║
//! ║     - The Communications Act of 1934 (47 U.S.C. § 151 et seq.)             ║
//! ║     - FCC Rules and Regulations (47 C.F.R.)                                 ║
//! ║     - The Electronic Communications Privacy Act (18 U.S.C. § 2510-2522)    ║
//! ║     - The Computer Fraud and Abuse Act (18 U.S.C. § 1030)                  ║
//! ║     - Export Administration Regulations (EAR) (15 C.F.R. § 730-774)        ║
//! ║     - International Traffic in Arms Regulations (ITAR) (22 C.F.R. § 120)   ║
//! ║     - All applicable laws in your jurisdiction                              ║
//! ║                                                                              ║
//! ║  2. PROHIBITED ACTIVITIES                                                    ║
//! ║     You shall NOT use this software to:                                      ║
//! ║     - Intercept, decode, or decrypt any communications without proper       ║
//! ║       authorization from all parties involved                                ║
//! ║     - Interfere with, jam, or disrupt any radio communications              ║
//! ║     - Transmit on any frequency without proper licensing                     ║
//! ║     - Conduct surveillance on any person without lawful authority           ║
//! ║     - Violate any person's reasonable expectation of privacy                ║
//! ║     - Access, intercept, or monitor government/military communications      ║
//! ║     - Engage in any activity that violates federal, state, or local law    ║
//! ║                                                                              ║
//! ║  3. INTENDED USE                                                             ║
//! ║     This module is intended SOLELY for:                                      ║
//! ║     - Personal safety and situational awareness                              ║
//! ║     - Detection of potentially harmful directed energy                       ║
//! ║     - Academic research and education                                        ║
//! ║     - Authorized security assessments                                        ║
//! ║     - Amateur radio operations within licensed privileges                    ║
//! ║     - Emergency preparedness and civil defense                               ║
//! ║                                                                              ║
//! ║  4. NO WARRANTY; LIMITATION OF LIABILITY                                     ║
//! ║     This software is provided "AS IS" without warranty of any kind.         ║
//! ║     The authors and contributors shall not be liable for any damages        ║
//! ║     arising from the use or misuse of this software.                        ║
//! ║                                                                              ║
//! ║  5. USER RESPONSIBILITY                                                      ║
//! ║     You are solely responsible for ensuring that your use of this           ║
//! ║     software complies with all applicable laws. The authors do not          ║
//! ║     endorse, encourage, or condone any illegal activity.                    ║
//! ║                                                                              ║
//! ║  6. DATA SOURCES                                                             ║
//! ║     All information in this module is derived from publicly available       ║
//! ║     sources including government publications, academic research,           ║
//! ║     manufacturer specifications, and open-source intelligence.              ║
//! ║     No classified or restricted information is included.                    ║
//! ║                                                                              ║
//! ║  IF YOU DO NOT AGREE TO THESE TERMS, DO NOT USE THIS SOFTWARE.              ║
//! ║                                                                              ║
//! ╚══════════════════════════════════════════════════════════════════════════════╝

use serde::{Deserialize, Serialize};

// ============================================================================
// DIRECTED ENERGY WEAPONS DATABASE
// ============================================================================

/// Directed energy weapon system entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedEnergySystem {
    pub name: String,
    pub designation: Option<String>,
    pub country: String,
    pub weapon_type: DewType,
    pub frequency_or_wavelength: FrequencySpec,
    pub power_output: PowerSpec,
    pub range_m: f64,
    pub effects: Vec<String>,
    pub platform: String,
    pub status: String,
    pub detection_methods: Vec<DetectionMethod>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DewType {
    HighEnergyLaser,
    HighPowerMicrowave,
    MillimeterWave,
    Acoustic,
    PulsedRf,
    ParticleBeam,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencySpec {
    pub value: f64,
    pub unit: String,
    pub band_name: Option<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSpec {
    pub value: f64,
    pub unit: String,
    pub peak_or_continuous: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMethod {
    pub method: String,
    pub equipment_needed: String,
    pub detection_signature: String,
    pub difficulty: String,
}

/// Comprehensive directed energy weapons database
/// Sources: Congressional Research Service, GAO, DTIC, manufacturer press releases
pub fn directed_energy_weapons_database() -> Vec<DirectedEnergySystem> {
    vec![
        // ===== HIGH ENERGY LASER SYSTEMS =====
        DirectedEnergySystem {
            name: "High Energy Laser Weapon System".to_string(),
            designation: Some("HELWS".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::HighEnergyLaser,
            frequency_or_wavelength: FrequencySpec {
                value: 1064.0,
                unit: "nm".to_string(),
                band_name: Some("Near-IR".to_string()),
                notes: "Nd:YAG or fiber laser".to_string(),
            },
            power_output: PowerSpec {
                value: 50.0,
                unit: "kW".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 1500.0,
            effects: vec!["C-UAS defeat".to_string(), "Target destruction".to_string()],
            platform: "Vehicle-mounted".to_string(),
            status: "Operational".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Laser warning receiver".to_string(),
                    equipment_needed: "LWR sensor (1064nm)".to_string(),
                    detection_signature: "Coherent IR pulse/CW".to_string(),
                    difficulty: "Medium".to_string(),
                },
                DetectionMethod {
                    method: "Thermal bloom".to_string(),
                    equipment_needed: "Thermal camera".to_string(),
                    detection_signature: "Atmospheric heating along beam path".to_string(),
                    difficulty: "Hard".to_string(),
                },
            ],
            source: "Raytheon press releases, CRS R46925".to_string(),
        },
        DirectedEnergySystem {
            name: "Laser Weapon System".to_string(),
            designation: Some("AN/SEQ-3 LaWS".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::HighEnergyLaser,
            frequency_or_wavelength: FrequencySpec {
                value: 1070.0,
                unit: "nm".to_string(),
                band_name: Some("Near-IR".to_string()),
                notes: "Fiber laser array".to_string(),
            },
            power_output: PowerSpec {
                value: 33.0,
                unit: "kW".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 2000.0,
            effects: vec!["UAV defeat".to_string(), "Small boat neutralization".to_string()],
            platform: "USS Ponce (naval)".to_string(),
            status: "Tested 2014".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "IR detector".to_string(),
                    equipment_needed: "Broadband IR sensor".to_string(),
                    detection_signature: "High-intensity 1070nm".to_string(),
                    difficulty: "Medium".to_string(),
                },
            ],
            source: "US Navy, Wikipedia (public)".to_string(),
        },
        DirectedEnergySystem {
            name: "HELIOS".to_string(),
            designation: Some("Mk 5 Mod 0".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::HighEnergyLaser,
            frequency_or_wavelength: FrequencySpec {
                value: 1000.0,
                unit: "nm".to_string(),
                band_name: Some("Near-IR".to_string()),
                notes: "Solid-state laser".to_string(),
            },
            power_output: PowerSpec {
                value: 60.0,
                unit: "kW".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 3000.0,
            effects: vec!["C-UAS".to_string(), "Anti-surface".to_string(), "ISR dazzle".to_string()],
            platform: "Arleigh Burke-class destroyers".to_string(),
            status: "Deployed 2022+".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Laser warning receiver".to_string(),
                    equipment_needed: "Military-grade LWR".to_string(),
                    detection_signature: "60kW coherent beam".to_string(),
                    difficulty: "Medium".to_string(),
                },
            ],
            source: "Lockheed Martin, CRS".to_string(),
        },
        DirectedEnergySystem {
            name: "ATHENA".to_string(),
            designation: None,
            country: "USA".to_string(),
            weapon_type: DewType::HighEnergyLaser,
            frequency_or_wavelength: FrequencySpec {
                value: 1064.0,
                unit: "nm".to_string(),
                band_name: Some("Near-IR".to_string()),
                notes: "ALADIN laser".to_string(),
            },
            power_output: PowerSpec {
                value: 30.0,
                unit: "kW".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 1500.0,
            effects: vec!["UAV defeat".to_string(), "Vehicle disabling".to_string()],
            platform: "Ground vehicle".to_string(),
            status: "Demonstrated".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "IR sensor".to_string(),
                    equipment_needed: "1064nm photodetector".to_string(),
                    detection_signature: "High-power CW IR".to_string(),
                    difficulty: "Medium".to_string(),
                },
            ],
            source: "Lockheed Martin press".to_string(),
        },
        
        // ===== HIGH POWER MICROWAVE SYSTEMS =====
        DirectedEnergySystem {
            name: "Active Denial System".to_string(),
            designation: Some("ADS".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::MillimeterWave,
            frequency_or_wavelength: FrequencySpec {
                value: 95.0,
                unit: "GHz".to_string(),
                band_name: Some("W-band / Millimeter wave".to_string()),
                notes: "3.2mm wavelength".to_string(),
            },
            power_output: PowerSpec {
                value: 100.0,
                unit: "kW".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 500.0,
            effects: vec![
                "Intense burning sensation".to_string(),
                "Pain compliance".to_string(),
                "Involuntary retreat".to_string(),
            ],
            platform: "Vehicle (Humvee/truck)".to_string(),
            status: "Operational".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Millimeter wave detector".to_string(),
                    equipment_needed: "95 GHz RF detector/sensor".to_string(),
                    detection_signature: "High-power 95 GHz continuous wave".to_string(),
                    difficulty: "Medium".to_string(),
                },
                DetectionMethod {
                    method: "Thermal sensation".to_string(),
                    equipment_needed: "Human perception".to_string(),
                    detection_signature: "Sudden skin heating".to_string(),
                    difficulty: "Easy".to_string(),
                },
            ],
            source: "JNLWP, DOD Fact Sheet".to_string(),
        },
        DirectedEnergySystem {
            name: "THOR (Tactical High-power Operational Responder)".to_string(),
            designation: Some("THOR".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::HighPowerMicrowave,
            frequency_or_wavelength: FrequencySpec {
                value: 2.0,
                unit: "GHz".to_string(),
                band_name: Some("S-band (estimated)".to_string()),
                notes: "Classified exact frequency".to_string(),
            },
            power_output: PowerSpec {
                value: 1000.0,  // Estimated MW-class peak
                unit: "MW (peak)".to_string(),
                peak_or_continuous: "Pulsed".to_string(),
            },
            range_m: 1000.0,
            effects: vec!["Drone swarm defeat".to_string(), "Electronics disruption".to_string()],
            platform: "Containerized".to_string(),
            status: "Operational 2023".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Wideband RF detector".to_string(),
                    equipment_needed: "Spectrum analyzer 1-6 GHz".to_string(),
                    detection_signature: "High-power RF pulse burst".to_string(),
                    difficulty: "Medium".to_string(),
                },
                DetectionMethod {
                    method: "Electronics malfunction".to_string(),
                    equipment_needed: "Observation".to_string(),
                    detection_signature: "Sudden equipment failure".to_string(),
                    difficulty: "Easy".to_string(),
                },
            ],
            source: "AFRL press releases".to_string(),
        },
        DirectedEnergySystem {
            name: "CHAMP".to_string(),
            designation: Some("Counter-electronics High Power Microwave Advanced Missile Project".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::HighPowerMicrowave,
            frequency_or_wavelength: FrequencySpec {
                value: 10.0,
                unit: "GHz".to_string(),
                band_name: Some("X-band (estimated)".to_string()),
                notes: "Narrowband HPM".to_string(),
            },
            power_output: PowerSpec {
                value: 100.0,
                unit: "MW (peak)".to_string(),
                peak_or_continuous: "Pulsed".to_string(),
            },
            range_m: 100.0,  // Stand-off cruise missile
            effects: vec!["Permanent electronics damage".to_string(), "EMP effect".to_string()],
            platform: "AGM-86 ALCM (cruise missile)".to_string(),
            status: "Tested 2012".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Extremely difficult".to_string(),
                    equipment_needed: "Fast RF detection + warning".to_string(),
                    detection_signature: "Brief high-power pulse".to_string(),
                    difficulty: "Very Hard".to_string(),
                },
            ],
            source: "Boeing press, USAF".to_string(),
        },
        DirectedEnergySystem {
            name: "Leonidas".to_string(),
            designation: None,
            country: "USA".to_string(),
            weapon_type: DewType::HighPowerMicrowave,
            frequency_or_wavelength: FrequencySpec {
                value: 3.0,
                unit: "GHz".to_string(),
                band_name: Some("S-band".to_string()),
                notes: "Solid-state phased array".to_string(),
            },
            power_output: PowerSpec {
                value: 500.0,
                unit: "kW (peak)".to_string(),
                peak_or_continuous: "Long pulse".to_string(),
            },
            range_m: 500.0,
            effects: vec!["C-UAS".to_string(), "Drone swarm defeat".to_string()],
            platform: "Various (trailer, vehicle)".to_string(),
            status: "Production".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "RF detector".to_string(),
                    equipment_needed: "S-band spectrum analyzer".to_string(),
                    detection_signature: "High-power pulse".to_string(),
                    difficulty: "Medium".to_string(),
                },
            ],
            source: "Epirus Inc".to_string(),
        },
        DirectedEnergySystem {
            name: "Bofors HPM Blackout".to_string(),
            designation: None,
            country: "Sweden".to_string(),
            weapon_type: DewType::HighPowerMicrowave,
            frequency_or_wavelength: FrequencySpec {
                value: 1.0,
                unit: "GHz".to_string(),
                band_name: Some("L-band (estimated)".to_string()),
                notes: "Wideband HPM".to_string(),
            },
            power_output: PowerSpec {
                value: 100.0,
                unit: "MW (peak)".to_string(),
                peak_or_continuous: "Pulsed".to_string(),
            },
            range_m: 100.0,
            effects: vec!["COTS electronics defeat".to_string(), "Vehicle stopping".to_string()],
            platform: "Vehicle".to_string(),
            status: "Demonstrated".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Wideband detector".to_string(),
                    equipment_needed: "Broadband RF monitor".to_string(),
                    detection_signature: "Wideband pulse".to_string(),
                    difficulty: "Medium".to_string(),
                },
            ],
            source: "BAE Systems Bofors".to_string(),
        },
        
        // ===== ACOUSTIC WEAPONS =====
        DirectedEnergySystem {
            name: "LRAD (Long Range Acoustic Device)".to_string(),
            designation: Some("LRAD 500X / 1000X / 2000X".to_string()),
            country: "USA".to_string(),
            weapon_type: DewType::Acoustic,
            frequency_or_wavelength: FrequencySpec {
                value: 2500.0,
                unit: "Hz".to_string(),
                band_name: Some("Audio frequency".to_string()),
                notes: "2.5 kHz primary tone, range 1-3 kHz".to_string(),
            },
            power_output: PowerSpec {
                value: 162.0,
                unit: "dB SPL".to_string(),
                peak_or_continuous: "Continuous".to_string(),
            },
            range_m: 3000.0,  // Communication mode
            effects: vec![
                "Pain at 140+ dB".to_string(),
                "Disorientation".to_string(),
                "Nausea".to_string(),
                "Hearing damage".to_string(),
            ],
            platform: "Portable/vehicle/naval".to_string(),
            status: "Widely deployed".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Sound level meter".to_string(),
                    equipment_needed: "SPL meter, directional mic".to_string(),
                    detection_signature: "High-intensity 2-3 kHz tone".to_string(),
                    difficulty: "Easy".to_string(),
                },
            ],
            source: "Genasys Inc".to_string(),
        },
        
        // ===== ALLEGED/SUSPECTED COVERT WEAPONS =====
        DirectedEnergySystem {
            name: "Havana Syndrome Device (Alleged)".to_string(),
            designation: None,
            country: "Unknown (attributed to Russia)".to_string(),
            weapon_type: DewType::PulsedRf,
            frequency_or_wavelength: FrequencySpec {
                value: 3.0,
                unit: "GHz (estimated)".to_string(),
                band_name: Some("S-band / microwave".to_string()),
                notes: "Pulsed microwave 1-10 GHz range suspected".to_string(),
            },
            power_output: PowerSpec {
                value: 1.0,
                unit: "kW (estimated)".to_string(),
                peak_or_continuous: "Pulsed".to_string(),
            },
            range_m: 100.0,
            effects: vec![
                "Severe headaches".to_string(),
                "Vertigo".to_string(),
                "Cognitive impairment".to_string(),
                "Brain lesions (MRI-confirmed)".to_string(),
                "Tinnitus".to_string(),
            ],
            platform: "Portable/concealable".to_string(),
            status: "Under investigation".to_string(),
            detection_methods: vec![
                DetectionMethod {
                    method: "Broadband RF monitoring".to_string(),
                    equipment_needed: "Spectrum analyzer 1-10 GHz, real-time capture".to_string(),
                    detection_signature: "Pulsed RF bursts, unknown repetition rate".to_string(),
                    difficulty: "Very Hard".to_string(),
                },
                DetectionMethod {
                    method: "Personnel symptoms".to_string(),
                    equipment_needed: "Medical observation".to_string(),
                    detection_signature: "Sudden onset headache, vertigo".to_string(),
                    difficulty: "Easy (after exposure)".to_string(),
                },
            ],
            source: "National Academy of Sciences report 2020, CBS 60 Minutes 2024-2026".to_string(),
        },
    ]
}

// ============================================================================
// SIGNAL CHARACTERISTICS - PREAMBLES, HANDSHAKES, BURST PATTERNS
// ============================================================================

/// Radio signal characteristic patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCharacteristic {
    pub protocol_name: String,
    pub signal_type: String,
    pub preamble_pattern: Option<String>,
    pub sync_word: Option<String>,
    pub burst_duration_ms: Option<f64>,
    pub repetition_rate_hz: Option<f64>,
    pub modulation: String,
    pub bandwidth_khz: f64,
    pub frequency_range: String,
    pub detection_notes: String,
}

/// Known signal preamble and synchronization patterns
pub fn signal_characteristics_database() -> Vec<SignalCharacteristic> {
    vec![
        // ===== DRONE PROTOCOLS =====
        SignalCharacteristic {
            protocol_name: "DJI OcuSync".to_string(),
            signal_type: "Control/video link".to_string(),
            preamble_pattern: Some("Proprietary FHSS sync".to_string()),
            sync_word: Some("DJI-specific (encrypted)".to_string()),
            burst_duration_ms: Some(2.0),
            repetition_rate_hz: Some(400.0),
            modulation: "OFDM FHSS".to_string(),
            bandwidth_khz: 20000.0,
            frequency_range: "2.4 GHz / 5.8 GHz".to_string(),
            detection_notes: "DroneID beacon broadcasts in clear, identifiable by bandwidth and hopping pattern".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "ExpressLRS".to_string(),
            signal_type: "RC control link".to_string(),
            preamble_pattern: Some("LoRa preamble (configurable symbols)".to_string()),
            sync_word: Some("0x12 (default)".to_string()),
            burst_duration_ms: Some(4.0),
            repetition_rate_hz: Some(500.0),
            modulation: "LoRa FHSS".to_string(),
            bandwidth_khz: 500.0,
            frequency_range: "868/915/2400 MHz".to_string(),
            detection_notes: "Distinctive LoRa chirp spread spectrum signature".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "TBS Crossfire".to_string(),
            signal_type: "Long-range RC link".to_string(),
            preamble_pattern: Some("LoRa preamble".to_string()),
            sync_word: Some("Proprietary".to_string()),
            burst_duration_ms: Some(5.0),
            repetition_rate_hz: Some(150.0),
            modulation: "LoRa".to_string(),
            bandwidth_khz: 500.0,
            frequency_range: "868/915 MHz".to_string(),
            detection_notes: "Long-range, identifiable chirp pattern".to_string(),
        },
        
        // ===== MILITARY PROTOCOLS =====
        SignalCharacteristic {
            protocol_name: "SINCGARS".to_string(),
            signal_type: "Tactical VHF".to_string(),
            preamble_pattern: Some("TSK (Time Slot Key) sync".to_string()),
            sync_word: Some("Encrypted COMSEC".to_string()),
            burst_duration_ms: Some(13.33),  // 75 hops/sec
            repetition_rate_hz: Some(75.0),
            modulation: "FM + FHSS".to_string(),
            bandwidth_khz: 25.0,
            frequency_range: "30-88 MHz".to_string(),
            detection_notes: "Rapid hopping across 2320 channels, can detect energy but not decode".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "HAVEQUICK".to_string(),
            signal_type: "Air-ground anti-jam".to_string(),
            preamble_pattern: Some("Net sync burst".to_string()),
            sync_word: Some("Encrypted TOD".to_string()),
            burst_duration_ms: Some(5.0),
            repetition_rate_hz: Some(200.0),
            modulation: "AM + FHSS".to_string(),
            bandwidth_khz: 25.0,
            frequency_range: "225-400 MHz UHF".to_string(),
            detection_notes: "Military UHF, rapid hopping".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "Link 16".to_string(),
            signal_type: "Tactical data link".to_string(),
            preamble_pattern: Some("TDMA slot synchronization".to_string()),
            sync_word: Some("NPG header".to_string()),
            burst_duration_ms: Some(7.8125),
            repetition_rate_hz: Some(128.0),  // Time slots
            modulation: "MSK + FHSS".to_string(),
            bandwidth_khz: 3000.0,
            frequency_range: "969-1206 MHz".to_string(),
            detection_notes: "51 frequency channels, 128 time slots/12.8 sec frame".to_string(),
        },
        
        // ===== CONSUMER/AMATEUR PROTOCOLS =====
        SignalCharacteristic {
            protocol_name: "FRS/GMRS".to_string(),
            signal_type: "Consumer voice".to_string(),
            preamble_pattern: None,
            sync_word: None,
            burst_duration_ms: None,
            repetition_rate_hz: None,
            modulation: "FM".to_string(),
            bandwidth_khz: 12.5,
            frequency_range: "462-467 MHz".to_string(),
            detection_notes: "Simple FM, CTCSS/DCS tones for squelch".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "PMR446".to_string(),
            signal_type: "Consumer voice (EU)".to_string(),
            preamble_pattern: None,
            sync_word: None,
            burst_duration_ms: None,
            repetition_rate_hz: None,
            modulation: "FM / DMR".to_string(),
            bandwidth_khz: 12.5,
            frequency_range: "446.0-446.2 MHz".to_string(),
            detection_notes: "European equivalent of FRS".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "Bluetooth".to_string(),
            signal_type: "Short-range data".to_string(),
            preamble_pattern: Some("01010101... (alternating)".to_string()),
            sync_word: Some("LAP-derived (24-bit)".to_string()),
            burst_duration_ms: Some(0.625),
            repetition_rate_hz: Some(1600.0),
            modulation: "GFSK FHSS".to_string(),
            bandwidth_khz: 1000.0,
            frequency_range: "2402-2480 MHz".to_string(),
            detection_notes: "79 channels, 1600 hops/sec, distinctive pattern".to_string(),
        },
        SignalCharacteristic {
            protocol_name: "WiFi 802.11".to_string(),
            signal_type: "WLAN".to_string(),
            preamble_pattern: Some("Short/Long training sequence".to_string()),
            sync_word: Some("PLCP header".to_string()),
            burst_duration_ms: Some(5.0),  // Variable
            repetition_rate_hz: None,
            modulation: "OFDM / DSSS".to_string(),
            bandwidth_khz: 20000.0,
            frequency_range: "2.4 GHz / 5 GHz".to_string(),
            detection_notes: "Ubiquitous, easy to detect".to_string(),
        },
    ]
}

// ============================================================================
// ENERGY SPIKE AND TRANSIENT DETECTION
// ============================================================================

/// RF transient / energy spike characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyTransient {
    pub event_type: String,
    pub typical_duration: String,
    pub rise_time: String,
    pub frequency_content: String,
    pub amplitude: String,
    pub detection_method: String,
    pub sources: Vec<String>,
}

pub fn energy_transient_database() -> Vec<EnergyTransient> {
    vec![
        EnergyTransient {
            event_type: "HPM Weapon Pulse".to_string(),
            typical_duration: "100 ns - 10 µs".to_string(),
            rise_time: "< 1 ns".to_string(),
            frequency_content: "Narrowband 1-10 GHz or wideband".to_string(),
            amplitude: "MV/m at source, kV/m at range".to_string(),
            detection_method: "Fast RF detector, oscilloscope with high bandwidth".to_string(),
            sources: vec!["Military HPM weapons".to_string(), "CHAMP-type missiles".to_string()],
        },
        EnergyTransient {
            event_type: "EMP (Nuclear)".to_string(),
            typical_duration: "E1: <1µs, E2: 1µs-1s, E3: 1s-minutes".to_string(),
            rise_time: "E1: ~5 ns".to_string(),
            frequency_content: "DC to 200 MHz (E1 dominant)".to_string(),
            amplitude: "50 kV/m (E1), varies with altitude".to_string(),
            detection_method: "HEMP sensors, D-dot sensors".to_string(),
            sources: vec!["Nuclear detonation (high altitude)".to_string()],
        },
        EnergyTransient {
            event_type: "Lightning-induced".to_string(),
            typical_duration: "100 µs - 1 ms".to_string(),
            rise_time: "1-10 µs".to_string(),
            frequency_content: "DC to 10 MHz".to_string(),
            amplitude: "Highly variable, kV/m nearby".to_string(),
            detection_method: "Lightning detector, sferics receiver".to_string(),
            sources: vec!["Natural lightning".to_string()],
        },
        EnergyTransient {
            event_type: "Radar pulse".to_string(),
            typical_duration: "0.1 - 100 µs".to_string(),
            rise_time: "ns to µs".to_string(),
            frequency_content: "Narrowband at radar frequency".to_string(),
            amplitude: "MW peak, W average".to_string(),
            detection_method: "RWR (Radar Warning Receiver)".to_string(),
            sources: vec!["Military/civilian radar".to_string()],
        },
        EnergyTransient {
            event_type: "ADS pulse".to_string(),
            typical_duration: "Continuous (not pulsed)".to_string(),
            rise_time: "ms ramp-up".to_string(),
            frequency_content: "95 GHz narrowband".to_string(),
            amplitude: "100 kW continuous".to_string(),
            detection_method: "95 GHz detector".to_string(),
            sources: vec!["Active Denial System".to_string()],
        },
        EnergyTransient {
            event_type: "Directed microwave (suspected Havana-type)".to_string(),
            typical_duration: "Unknown, possibly ms to seconds".to_string(),
            rise_time: "Unknown".to_string(),
            frequency_content: "1-10 GHz (suspected)".to_string(),
            amplitude: "Unknown, sufficient for biological effect".to_string(),
            detection_method: "Broadband RF monitor with logging".to_string(),
            sources: vec!["Alleged covert weapons".to_string()],
        },
    ]
}

// ============================================================================
// INFRARED DETECTION BANDS
// ============================================================================

/// Infrared band specifications for detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfraredBand {
    pub name: String,
    pub abbreviation: String,
    pub wavelength_um: (f64, f64),
    pub typical_sources: Vec<String>,
    pub detector_types: Vec<String>,
    pub atmospheric_transmission: String,
    pub military_applications: Vec<String>,
}

pub fn infrared_bands_database() -> Vec<InfraredBand> {
    vec![
        InfraredBand {
            name: "Near Infrared".to_string(),
            abbreviation: "NIR".to_string(),
            wavelength_um: (0.75, 1.4),
            typical_sources: vec![
                "Night vision illuminators (850/940nm)".to_string(),
                "IR lasers".to_string(),
                "Hot surfaces".to_string(),
            ],
            detector_types: vec!["Silicon CCD/CMOS".to_string(), "InGaAs".to_string()],
            atmospheric_transmission: "Good".to_string(),
            military_applications: vec![
                "NVG illumination detection".to_string(),
                "Laser rangefinder detection".to_string(),
                "Aiming laser detection".to_string(),
            ],
        },
        InfraredBand {
            name: "Short-Wave Infrared".to_string(),
            abbreviation: "SWIR".to_string(),
            wavelength_um: (1.4, 3.0),
            typical_sources: vec![
                "Hot exhaust".to_string(),
                "Laser designators (1.54µm)".to_string(),
                "Reflected sunlight".to_string(),
            ],
            detector_types: vec!["InGaAs".to_string(), "HgCdTe".to_string()],
            atmospheric_transmission: "Good (except 1.4, 1.9µm water absorption)".to_string(),
            military_applications: vec![
                "Missile plume detection".to_string(),
                "Eye-safe laser detection".to_string(),
                "Camouflage penetration".to_string(),
            ],
        },
        InfraredBand {
            name: "Mid-Wave Infrared".to_string(),
            abbreviation: "MWIR".to_string(),
            wavelength_um: (3.0, 5.0),
            typical_sources: vec![
                "Jet engine exhaust (~700°C)".to_string(),
                "Vehicle engines".to_string(),
                "Missile plumes".to_string(),
                "High-energy laser heating".to_string(),
            ],
            detector_types: vec!["InSb".to_string(), "HgCdTe".to_string(), "QWIP".to_string()],
            atmospheric_transmission: "Excellent (3-5µm window)".to_string(),
            military_applications: vec![
                "IRST (Infrared Search and Track)".to_string(),
                "Missile seekers".to_string(),
                "Aircraft detection".to_string(),
                "Drone detection".to_string(),
            ],
        },
        InfraredBand {
            name: "Long-Wave Infrared".to_string(),
            abbreviation: "LWIR".to_string(),
            wavelength_um: (8.0, 14.0),
            typical_sources: vec![
                "Human body (~37°C)".to_string(),
                "Warm vehicles".to_string(),
                "Buildings".to_string(),
                "Terrain".to_string(),
            ],
            detector_types: vec![
                "Microbolometer (uncooled)".to_string(),
                "HgCdTe (cooled)".to_string(),
            ],
            atmospheric_transmission: "Excellent (8-14µm window)".to_string(),
            military_applications: vec![
                "Personnel detection".to_string(),
                "Vehicle tracking".to_string(),
                "Thermal sights".to_string(),
                "Perimeter security".to_string(),
            ],
        },
        InfraredBand {
            name: "Very Long-Wave Infrared".to_string(),
            abbreviation: "VLWIR".to_string(),
            wavelength_um: (14.0, 30.0),
            typical_sources: vec![
                "Cold objects (space)".to_string(),
                "Low-temperature targets".to_string(),
            ],
            detector_types: vec!["Si:As".to_string(), "Specialized cooled detectors".to_string()],
            atmospheric_transmission: "Poor (CO2 absorption)".to_string(),
            military_applications: vec![
                "Space-based surveillance".to_string(),
                "Cold target detection".to_string(),
            ],
        },
    ]
}

// ============================================================================
// FREQUENCY HOPPING / SPREAD SPECTRUM DETECTION
// ============================================================================

/// Spread spectrum signal characteristics for detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadSpectrumSignal {
    pub name: String,
    pub spreading_type: String,
    pub hop_rate_hz: Option<f64>,
    pub chip_rate_mcps: Option<f64>,
    pub bandwidth_mhz: f64,
    pub detection_approach: Vec<String>,
    pub identifiable_features: Vec<String>,
}

pub fn spread_spectrum_detection_database() -> Vec<SpreadSpectrumSignal> {
    vec![
        SpreadSpectrumSignal {
            name: "SINCGARS FHSS".to_string(),
            spreading_type: "Frequency Hopping".to_string(),
            hop_rate_hz: Some(75.0),
            chip_rate_mcps: None,
            bandwidth_mhz: 58.0,  // 30-88 MHz
            detection_approach: vec![
                "Wideband energy detection".to_string(),
                "Channelized receiver".to_string(),
                "Real-time spectrum analyzer".to_string(),
            ],
            identifiable_features: vec![
                "75 hops/sec rate".to_string(),
                "25 kHz channel bandwidth".to_string(),
                "2320 possible channels".to_string(),
                "Cannot decode without COMSEC".to_string(),
            ],
        },
        SpreadSpectrumSignal {
            name: "Bluetooth FHSS".to_string(),
            spreading_type: "Frequency Hopping".to_string(),
            hop_rate_hz: Some(1600.0),
            chip_rate_mcps: None,
            bandwidth_mhz: 78.0,
            detection_approach: vec![
                "2.4 GHz spectrum analyzer".to_string(),
                "Bluetooth sniffer".to_string(),
            ],
            identifiable_features: vec![
                "79 channels".to_string(),
                "1 MHz channel spacing".to_string(),
                "1600 hops/sec".to_string(),
                "GFSK modulation".to_string(),
            ],
        },
        SpreadSpectrumSignal {
            name: "GPS DSSS".to_string(),
            spreading_type: "Direct Sequence".to_string(),
            hop_rate_hz: None,
            chip_rate_mcps: Some(1.023),
            bandwidth_mhz: 2.0,
            detection_approach: vec![
                "Correlation receiver".to_string(),
                "Spread spectrum analyzer".to_string(),
            ],
            identifiable_features: vec![
                "1575.42 MHz L1".to_string(),
                "C/A code 1.023 Mcps".to_string(),
                "Below noise floor".to_string(),
            ],
        },
        SpreadSpectrumSignal {
            name: "Military DSSS (generic)".to_string(),
            spreading_type: "Direct Sequence".to_string(),
            hop_rate_hz: None,
            chip_rate_mcps: Some(10.0),  // Varies widely
            bandwidth_mhz: 20.0,
            detection_approach: vec![
                "Energy detection (limited)".to_string(),
                "Autocorrelation analysis".to_string(),
                "Cyclostationary feature detection".to_string(),
            ],
            identifiable_features: vec![
                "Noise-like appearance".to_string(),
                "Spectral flatness".to_string(),
                "Difficult to distinguish from noise".to_string(),
            ],
        },
        SpreadSpectrumSignal {
            name: "DJI OcuSync FHSS".to_string(),
            spreading_type: "Frequency Hopping + OFDM".to_string(),
            hop_rate_hz: Some(400.0),
            chip_rate_mcps: None,
            bandwidth_mhz: 40.0,
            detection_approach: vec![
                "2.4/5.8 GHz spectrum analyzer".to_string(),
                "DroneID decoder".to_string(),
                "Pattern recognition".to_string(),
            ],
            identifiable_features: vec![
                "DroneID beacon in clear".to_string(),
                "Identifiable bandwidth".to_string(),
                "Hopping pattern".to_string(),
            ],
        },
    ]
}

// ============================================================================
// DETECTION EQUIPMENT RECOMMENDATIONS
// ============================================================================

/// Equipment recommendations for threat detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionEquipment {
    pub threat_type: String,
    pub equipment_category: String,
    pub specific_products: Vec<String>,
    pub frequency_coverage: String,
    pub estimated_cost: String,
    pub capability_level: String,
}

pub fn detection_equipment_database() -> Vec<DetectionEquipment> {
    vec![
        DetectionEquipment {
            threat_type: "High-Power Microwave".to_string(),
            equipment_category: "Wideband RF Field Meter".to_string(),
            specific_products: vec![
                "Narda NBM-550".to_string(),
                "ETS-Lindgren HI-6105".to_string(),
                "Wavecontrol SMP2".to_string(),
            ],
            frequency_coverage: "100 kHz - 60 GHz".to_string(),
            estimated_cost: "$2,000 - $15,000".to_string(),
            capability_level: "Professional".to_string(),
        },
        DetectionEquipment {
            threat_type: "Millimeter Wave (ADS)".to_string(),
            equipment_category: "95 GHz Detector".to_string(),
            specific_products: vec![
                "Virginia Diodes WR-10 detector".to_string(),
                "Custom W-band horn + detector".to_string(),
            ],
            frequency_coverage: "75-110 GHz".to_string(),
            estimated_cost: "$5,000 - $20,000".to_string(),
            capability_level: "Specialized".to_string(),
        },
        DetectionEquipment {
            threat_type: "Acoustic (LRAD)".to_string(),
            equipment_category: "Sound Level Meter".to_string(),
            specific_products: vec![
                "Larson Davis 831C".to_string(),
                "NTi Audio XL2".to_string(),
                "NIOSH SLM App (smartphone)".to_string(),
            ],
            frequency_coverage: "20 Hz - 20 kHz".to_string(),
            estimated_cost: "$50 - $5,000".to_string(),
            capability_level: "Consumer to Professional".to_string(),
        },
        DetectionEquipment {
            threat_type: "Laser Weapons".to_string(),
            equipment_category: "Laser Warning Receiver".to_string(),
            specific_products: vec![
                "Leonardo AN/AVR-2B".to_string(),
                "Elbit MACS".to_string(),
                "Consumer: IR detector card".to_string(),
            ],
            frequency_coverage: "400nm - 2µm".to_string(),
            estimated_cost: "$50 (card) - $100,000+ (military)".to_string(),
            capability_level: "Consumer to Military".to_string(),
        },
        DetectionEquipment {
            threat_type: "Drones/FPV".to_string(),
            equipment_category: "RF Spectrum Analyzer".to_string(),
            specific_products: vec![
                "RTL-SDR v3/v4".to_string(),
                "HackRF One".to_string(),
                "Airspy R2".to_string(),
                "TinySA Ultra".to_string(),
            ],
            frequency_coverage: "24 MHz - 6 GHz".to_string(),
            estimated_cost: "$30 - $300".to_string(),
            capability_level: "Hobbyist".to_string(),
        },
        DetectionEquipment {
            threat_type: "Thermal/IR Sources".to_string(),
            equipment_category: "Thermal Camera".to_string(),
            specific_products: vec![
                "FLIR Scout TK".to_string(),
                "Seek Thermal Compact".to_string(),
                "Pulsar thermal monoculars".to_string(),
            ],
            frequency_coverage: "7.5-14 µm (LWIR)".to_string(),
            estimated_cost: "$200 - $5,000".to_string(),
            capability_level: "Consumer to Professional".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dew_database() {
        let dews = directed_energy_weapons_database();
        assert!(dews.iter().any(|d| d.name.contains("Active Denial")));
        assert!(dews.iter().any(|d| d.name.contains("LRAD")));
        assert!(dews.iter().any(|d| d.designation.as_deref() == Some("HELWS") || d.name.contains("High Energy Laser")));
    }
    
    #[test]
    fn test_signal_characteristics() {
        let sigs = signal_characteristics_database();
        assert!(sigs.iter().any(|s| s.protocol_name.contains("SINCGARS")));
        assert!(sigs.iter().any(|s| s.protocol_name.contains("DJI")));
    }
    
    #[test]
    fn test_infrared_bands() {
        let bands = infrared_bands_database();
        assert_eq!(bands.len(), 5);
        assert!(bands.iter().any(|b| b.abbreviation == "MWIR"));
        assert!(bands.iter().any(|b| b.abbreviation == "LWIR"));
    }
}
