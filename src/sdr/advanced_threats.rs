//! Advanced Threat Detection Module
//!
//! Detection capabilities for:
//! - UAV/Drone motor EMI signatures
//! - Fiber-optic drone detection methods
//! - Military autonomous systems (Anduril, etc.)
//! - Iranian/Israeli drone frequencies
//! - Directed energy weapons (Havana Syndrome, ADS, LRAD)
//! - HAARP and ionospheric heaters
//! - Covert surveillance methods
//! - Microwave link vulnerabilities
//!
//! DISCLAIMER: This module is for DEFENSIVE detection only.
//! Unauthorized interference with any communications is illegal.

use serde::{Deserialize, Serialize};

// ============================================================================
// UAV/DRONE DETECTION
// ============================================================================

/// UAV communication frequency bands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UavFrequencyBand {
    pub name: String,
    pub freq_start_mhz: f64,
    pub freq_end_mhz: f64,
    pub use_type: UavLinkType,
    pub common_drones: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UavLinkType {
    Control,        // Remote control link
    Telemetry,      // Drone-to-controller data
    Video,          // FPV video downlink
    Combined,       // All-in-one (like DJI)
    Gps,            // GPS/GNSS
    Collision,      // Collision avoidance
}

/// Standard UAV frequency database
pub fn uav_frequency_database() -> Vec<UavFrequencyBand> {
    vec![
        // Control frequencies
        UavFrequencyBand {
            name: "27 MHz RC Band".to_string(),
            freq_start_mhz: 26.995,
            freq_end_mhz: 27.255,
            use_type: UavLinkType::Control,
            common_drones: vec!["Toy drones".to_string(), "Legacy RC".to_string()],
            notes: "Old RC band, rarely used now".to_string(),
        },
        UavFrequencyBand {
            name: "72 MHz RC Band".to_string(),
            freq_start_mhz: 72.0,
            freq_end_mhz: 72.99,
            use_type: UavLinkType::Control,
            common_drones: vec!["RC aircraft".to_string()],
            notes: "US only, dedicated RC aircraft".to_string(),
        },
        UavFrequencyBand {
            name: "433 MHz ISM".to_string(),
            freq_start_mhz: 433.05,
            freq_end_mhz: 434.79,
            use_type: UavLinkType::Control,
            common_drones: vec!["Long range FPV".to_string(), "Crossfire".to_string(), "ExpressLRS".to_string()],
            notes: "EU: 10mW limit, Good penetration".to_string(),
        },
        UavFrequencyBand {
            name: "868 MHz ISM (EU)".to_string(),
            freq_start_mhz: 868.0,
            freq_end_mhz: 870.0,
            use_type: UavLinkType::Control,
            common_drones: vec!["ExpressLRS".to_string(), "Long range systems".to_string()],
            notes: "EU ISM band".to_string(),
        },
        UavFrequencyBand {
            name: "900 MHz ISM".to_string(),
            freq_start_mhz: 902.0,
            freq_end_mhz: 928.0,
            use_type: UavLinkType::Control,
            common_drones: vec!["Crossfire 900".to_string(), "TBS Tracer".to_string(), "ExpressLRS 900".to_string()],
            notes: "US ISM, excellent range 10-30km+".to_string(),
        },
        UavFrequencyBand {
            name: "2.4 GHz ISM Control".to_string(),
            freq_start_mhz: 2400.0,
            freq_end_mhz: 2483.5,
            use_type: UavLinkType::Control,
            common_drones: vec!["Most consumer drones".to_string(), "DJI".to_string(), "FrSky".to_string()],
            notes: "Most common, WiFi interference".to_string(),
        },
        
        // Video frequencies
        UavFrequencyBand {
            name: "1.2 GHz Video".to_string(),
            freq_start_mhz: 1240.0,
            freq_end_mhz: 1300.0,
            use_type: UavLinkType::Video,
            common_drones: vec!["Long range FPV".to_string(), "Analog systems".to_string()],
            notes: "Good penetration, ham license required in US".to_string(),
        },
        UavFrequencyBand {
            name: "2.4 GHz Video".to_string(),
            freq_start_mhz: 2400.0,
            freq_end_mhz: 2500.0,
            use_type: UavLinkType::Video,
            common_drones: vec!["DJI HD".to_string(), "HDZero".to_string()],
            notes: "Can conflict with 2.4GHz control".to_string(),
        },
        UavFrequencyBand {
            name: "5.8 GHz Video".to_string(),
            freq_start_mhz: 5650.0,
            freq_end_mhz: 5925.0,
            use_type: UavLinkType::Video,
            common_drones: vec!["Most FPV drones".to_string(), "DJI Digital".to_string(), "Analog VTX".to_string()],
            notes: "MOST COMMON FPV VIDEO, 40+ channels".to_string(),
        },
        
        // DJI Specific
        UavFrequencyBand {
            name: "DJI OcuSync 2.0/3.0".to_string(),
            freq_start_mhz: 2400.0,
            freq_end_mhz: 2483.5,
            use_type: UavLinkType::Combined,
            common_drones: vec!["DJI Mavic".to_string(), "DJI Air".to_string(), "DJI Mini".to_string()],
            notes: "FHSS, identifiable by DroneID".to_string(),
        },
        UavFrequencyBand {
            name: "DJI 5.8 GHz Mode".to_string(),
            freq_start_mhz: 5725.0,
            freq_end_mhz: 5850.0,
            use_type: UavLinkType::Combined,
            common_drones: vec!["DJI (FCC mode)".to_string()],
            notes: "Higher power mode in FCC regions".to_string(),
        },
    ]
}

/// UAV motor EMI detection signatures
/// Based on research: Motors create EMI in specific frequency patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorEmiSignature {
    pub motor_type: String,
    pub primary_freq_hz: f64,
    pub harmonics: Vec<f64>,
    pub bandwidth_hz: f64,
    pub detection_range_m: f64,
    pub notes: String,
}

/// Known motor EMI signatures for drone detection
pub fn motor_emi_signatures() -> Vec<MotorEmiSignature> {
    vec![
        MotorEmiSignature {
            motor_type: "Brushless DC (small)".to_string(),
            primary_freq_hz: 20_000.0,  // PWM switching frequency
            harmonics: vec![40_000.0, 60_000.0, 80_000.0],
            bandwidth_hz: 5_000.0,
            detection_range_m: 50.0,
            notes: "Consumer drone motors, PWM artifacts".to_string(),
        },
        MotorEmiSignature {
            motor_type: "Brushless DC (racing)".to_string(),
            primary_freq_hz: 48_000.0,  // Higher PWM
            harmonics: vec![96_000.0, 144_000.0],
            bandwidth_hz: 10_000.0,
            detection_range_m: 100.0,
            notes: "High-performance FPV racing motors".to_string(),
        },
        MotorEmiSignature {
            motor_type: "Large multirotor".to_string(),
            primary_freq_hz: 8_000.0,
            harmonics: vec![16_000.0, 24_000.0, 32_000.0],
            bandwidth_hz: 2_000.0,
            detection_range_m: 200.0,
            notes: "Industrial/agricultural drones".to_string(),
        },
    ]
}

// ============================================================================
// FIBER-OPTIC UAV SYSTEMS
// ============================================================================

/// Fiber-optic drone characteristics
/// These drones are JAM-PROOF but have other detection vectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiberOpticDrone {
    pub name: String,
    pub origin: String,
    pub fiber_length_km: f64,
    pub spool_weight_g: f64,
    pub detection_methods: Vec<String>,
    pub notes: String,
}

pub fn fiber_optic_drone_database() -> Vec<FiberOpticDrone> {
    vec![
        FiberOpticDrone {
            name: "HCX FPV (Ukraine)".to_string(),
            origin: "Germany/Ukraine".to_string(),
            fiber_length_km: 10.0,
            spool_weight_g: 500.0,
            detection_methods: vec![
                "Acoustic (motor noise)".to_string(),
                "Visual/thermal imaging".to_string(),
                "Radar".to_string(),
                "Motor EMI".to_string(),
            ],
            notes: "First widely deployed fiber FPV, jam-proof".to_string(),
        },
        FiberOpticDrone {
            name: "KVS (Russia)".to_string(),
            origin: "Russia".to_string(),
            fiber_length_km: 8.0,
            spool_weight_g: 600.0,
            detection_methods: vec![
                "Acoustic signature".to_string(),
                "Thermal imaging".to_string(),
                "Motor EMI".to_string(),
            ],
            notes: "Russian response to Ukrainian jamming".to_string(),
        },
        FiberOpticDrone {
            name: "Generic FOG-D".to_string(),
            origin: "Various".to_string(),
            fiber_length_km: 5.0,
            spool_weight_g: 300.0,
            detection_methods: vec![
                "Visual detection".to_string(),
                "Acoustic detection".to_string(),
                "Thermal signature".to_string(),
                "Motor PWM EMI (20-50 kHz harmonics)".to_string(),
            ],
            notes: "Cannot be jammed, RF silent, trailing fiber visible".to_string(),
        },
    ]
}

// ============================================================================
// MILITARY AUTONOMOUS SYSTEMS
// ============================================================================

/// Anduril system specifications (from public sources)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndurilSystem {
    pub name: String,
    pub designation: Option<String>,
    pub system_type: String,
    pub status: String,
    pub known_specs: Vec<String>,
    pub detection_vectors: Vec<String>,
    pub notes: String,
}

pub fn anduril_systems_database() -> Vec<AndurilSystem> {
    vec![
        AndurilSystem {
            name: "Fury".to_string(),
            designation: Some("YFQ-44A".to_string()),
            system_type: "Unmanned Combat Aerial Vehicle (UCAV)".to_string(),
            status: "Production started March 2026".to_string(),
            known_specs: vec![
                "Near-fighter speed (subsonic)".to_string(),
                "Jet propulsion".to_string(),
                "AI-driven via Lattice OS".to_string(),
                "Modular payload bays".to_string(),
                "Collaborative Combat Aircraft (CCA) program".to_string(),
            ],
            detection_vectors: vec![
                "Radar (low RCS design)".to_string(),
                "IR signature from jet engine".to_string(),
                "Acoustic signature".to_string(),
                "RF data links (encrypted)".to_string(),
            ],
            notes: "Designed to operate alongside manned F-35/F-22".to_string(),
        },
        AndurilSystem {
            name: "Roadrunner".to_string(),
            designation: None,
            system_type: "Autonomous Air Defense Interceptor".to_string(),
            status: "Operational".to_string(),
            known_specs: vec![
                "Twin turbojet engines".to_string(),
                "VTOL capable".to_string(),
                "High-subsonic speed".to_string(),
                "Reusable if no intercept".to_string(),
                "AI-autonomous intercept".to_string(),
                "Modular warhead options".to_string(),
            ],
            detection_vectors: vec![
                "Jet noise signature".to_string(),
                "Radar (small target)".to_string(),
                "IR from turbojets".to_string(),
            ],
            notes: "Anti-drone, anti-cruise missile interceptor".to_string(),
        },
        AndurilSystem {
            name: "Dive-XL".to_string(),
            designation: None,
            system_type: "Extra Large Autonomous Underwater Vehicle (XL-AUV)".to_string(),
            status: "Testing/Prototype 2026".to_string(),
            known_specs: vec![
                "Months-long mission duration".to_string(),
                "All-electric propulsion".to_string(),
                "100+ hour single voyage demonstrated".to_string(),
                "Lattice OS integration".to_string(),
                "Large payload capacity".to_string(),
                "Australia $58M investment".to_string(),
            ],
            detection_vectors: vec![
                "Sonar (difficult, quiet propulsion)".to_string(),
                "Magnetic anomaly detection".to_string(),
                "Wake detection".to_string(),
            ],
            notes: "Seabed warfare, mine deployment, ISR capable".to_string(),
        },
        AndurilSystem {
            name: "EagleEye".to_string(),
            designation: None,
            system_type: "AR/VR Soldier Augmentation System".to_string(),
            status: "100 units to Army 2026".to_string(),
            known_specs: vec![
                "360-degree situational awareness".to_string(),
                "Lattice AI integration".to_string(),
                "Transparent AR and passthrough MR modes".to_string(),
                "Digital vision enhancement".to_string(),
                "Mesh networking between units".to_string(),
                "Threat detection and marking".to_string(),
                "Control robotic assets".to_string(),
            ],
            detection_vectors: vec![
                "RF mesh networking emissions".to_string(),
                "IR from electronics".to_string(),
                "Potential NVG-like emissions".to_string(),
            ],
            notes: "Qualcomm/Oakley/Gentex partnerships".to_string(),
        },
        AndurilSystem {
            name: "Lattice OS".to_string(),
            designation: None,
            system_type: "AI Command and Control Platform".to_string(),
            status: "Deployed".to_string(),
            known_specs: vec![
                "Sensor fusion from all Anduril systems".to_string(),
                "Autonomous decision making".to_string(),
                "Human-on-the-loop controls".to_string(),
                "Real-time threat assessment".to_string(),
            ],
            detection_vectors: vec![
                "RF data links between nodes".to_string(),
            ],
            notes: "Core AI platform for all Anduril systems".to_string(),
        },
    ]
}

// ============================================================================
// IRANIAN/ISRAELI DRONE FREQUENCIES
// ============================================================================

/// Drone system frequency characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilitaryDroneSystem {
    pub name: String,
    pub country: String,
    pub drone_type: String,
    pub navigation: Vec<String>,
    pub communication: Vec<String>,
    pub detection_methods: Vec<String>,
    pub notes: String,
}

pub fn military_drone_database() -> Vec<MilitaryDroneSystem> {
    vec![
        // Iranian systems
        MilitaryDroneSystem {
            name: "Shahed-136 / Geran-2".to_string(),
            country: "Iran/Russia".to_string(),
            drone_type: "Loitering munition (kamikaze)".to_string(),
            navigation: vec![
                "GPS (civilian)".to_string(),
                "GLONASS".to_string(),
                "16-element CRPA anti-jam antenna".to_string(),
                "RTK via cellular modem".to_string(),
                "Inertial Navigation System (INS) backup".to_string(),
            ],
            communication: vec![
                "Minimal RF emission (autonomous)".to_string(),
                "Pre-programmed waypoints".to_string(),
            ],
            detection_methods: vec![
                "Radar (small RCS but detectable)".to_string(),
                "Acoustic (distinctive lawn mower engine)".to_string(),
                "IR signature from engine".to_string(),
                "GPS jamming can divert course".to_string(),
            ],
            notes: "Low cost ($20-50k), mass produced, 1500-2000km range".to_string(),
        },
        MilitaryDroneSystem {
            name: "Shahed-101".to_string(),
            country: "Iran".to_string(),
            drone_type: "Electric loitering munition".to_string(),
            navigation: vec![
                "GPS/GLONASS".to_string(),
                "INS backup".to_string(),
            ],
            communication: vec![
                "Silent approach (electric)".to_string(),
                "Minimal RF".to_string(),
            ],
            detection_methods: vec![
                "Radar".to_string(),
                "Visual (quiet, harder acoustic detection)".to_string(),
                "IR (lower than combustion)".to_string(),
            ],
            notes: "Electric propulsion for silent approach".to_string(),
        },
        
        // Israeli systems
        MilitaryDroneSystem {
            name: "IAI Heron".to_string(),
            country: "Israel".to_string(),
            drone_type: "MALE UAV (reconnaissance/strike)".to_string(),
            navigation: vec![
                "GPS/INS".to_string(),
                "SATCOM".to_string(),
            ],
            communication: vec![
                "Line-of-sight data link".to_string(),
                "SATCOM for BLOS".to_string(),
                "Encrypted military frequencies".to_string(),
            ],
            detection_methods: vec![
                "Radar".to_string(),
                "RF emission detection".to_string(),
                "IR from engine".to_string(),
            ],
            notes: "52 hour endurance, used by multiple nations".to_string(),
        },
        MilitaryDroneSystem {
            name: "IAI Harop".to_string(),
            country: "Israel".to_string(),
            drone_type: "Loitering munition (SEAD)".to_string(),
            navigation: vec![
                "GPS/INS".to_string(),
                "Anti-radiation homing".to_string(),
            ],
            communication: vec![
                "Encrypted data link".to_string(),
                "Man-in-the-loop abort capable".to_string(),
            ],
            detection_methods: vec![
                "Radar".to_string(),
                "IR".to_string(),
                "RF data link intercept".to_string(),
            ],
            notes: "Homes on radar emissions, 23kg warhead".to_string(),
        },
        MilitaryDroneSystem {
            name: "Elbit Hermes 450/900".to_string(),
            country: "Israel".to_string(),
            drone_type: "Tactical UAV".to_string(),
            navigation: vec![
                "GPS/INS".to_string(),
                "Automatic takeoff/landing".to_string(),
            ],
            communication: vec![
                "C-band data link".to_string(),
                "Ku-band SATCOM option".to_string(),
            ],
            detection_methods: vec![
                "Radar".to_string(),
                "RF intercept".to_string(),
                "Acoustic".to_string(),
            ],
            notes: "Widely exported, 20+ hour endurance".to_string(),
        },
    ]
}

// ============================================================================
// DIRECTED ENERGY WEAPONS (DEW) DETECTION
// ============================================================================

/// Directed energy weapon specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectedEnergyWeapon {
    pub name: String,
    pub dew_type: DewType,
    pub frequency_ghz: Option<f64>,
    pub wavelength: Option<String>,
    pub range_m: f64,
    pub power_kw: f64,
    pub effects: Vec<String>,
    pub detection_methods: Vec<String>,
    pub countermeasures: Vec<String>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DewType {
    Microwave,
    MillimeterWave,
    Acoustic,
    Laser,
    PulsedRf,
    Unknown,
}

pub fn directed_energy_database() -> Vec<DirectedEnergyWeapon> {
    vec![
        DirectedEnergyWeapon {
            name: "Active Denial System (ADS)".to_string(),
            dew_type: DewType::MillimeterWave,
            frequency_ghz: Some(95.0),
            wavelength: Some("3.2 mm".to_string()),
            range_m: 500.0,
            power_kw: 100.0,
            effects: vec![
                "Intense burning sensation".to_string(),
                "Skin heating (1/64 inch penetration)".to_string(),
                "Pain compliance".to_string(),
                "Involuntary retreat".to_string(),
            ],
            detection_methods: vec![
                "95 GHz RF detector".to_string(),
                "Millimeter wave sensor".to_string(),
                "Thermal anomaly on skin".to_string(),
            ],
            countermeasures: vec![
                "Metallic clothing/shielding".to_string(),
                "Water (absorbs mm-wave)".to_string(),
                "Distance".to_string(),
            ],
            notes: "US military non-lethal system, vehicle mounted".to_string(),
        },
        DirectedEnergyWeapon {
            name: "LRAD (Long Range Acoustic Device)".to_string(),
            dew_type: DewType::Acoustic,
            frequency_ghz: None,
            wavelength: Some("Audible 2.5-15 kHz".to_string()),
            range_m: 3000.0,  // Communication mode
            power_kw: 0.5,
            effects: vec![
                "Pain (140+ dB)".to_string(),
                "Disorientation".to_string(),
                "Nausea".to_string(),
                "Hearing damage".to_string(),
                "Headaches".to_string(),
            ],
            detection_methods: vec![
                "Sound level meter".to_string(),
                "Directional microphone".to_string(),
                "Infrasound detector".to_string(),
            ],
            countermeasures: vec![
                "Hearing protection".to_string(),
                "Distance".to_string(),
                "Barriers".to_string(),
            ],
            notes: "Used for crowd control, hailing, pirate deterrent".to_string(),
        },
        DirectedEnergyWeapon {
            name: "Havana Syndrome Device (alleged)".to_string(),
            dew_type: DewType::PulsedRf,
            frequency_ghz: Some(3.0), // Estimated 1-10 GHz range
            wavelength: Some("cm-wave pulsed".to_string()),
            range_m: 100.0,
            power_kw: 1.0,  // Estimated
            effects: vec![
                "Severe headaches".to_string(),
                "Vertigo/dizziness".to_string(),
                "Cognitive impairment".to_string(),
                "Memory loss".to_string(),
                "Tinnitus".to_string(),
                "Brain lesions".to_string(),
            ],
            detection_methods: vec![
                "Broadband RF detector (1-10 GHz)".to_string(),
                "Pulsed RF analyzer".to_string(),
                "Spectrum analyzer with time-domain capture".to_string(),
                "EMF meter (may not detect pulses)".to_string(),
            ],
            countermeasures: vec![
                "RF shielded room".to_string(),
                "Metal barriers".to_string(),
                "Faraday cage".to_string(),
                "Awareness and rapid evacuation".to_string(),
            ],
            notes: "Linked to Russian GRU Unit 29155, device obtained by US 2024".to_string(),
        },
        DirectedEnergyWeapon {
            name: "Microwave Auditory Effect Device".to_string(),
            dew_type: DewType::Microwave,
            frequency_ghz: Some(2.45), // Common microwave freq
            wavelength: Some("12 cm".to_string()),
            range_m: 300.0,
            power_kw: 10.0,
            effects: vec![
                "Hearing sounds without ears".to_string(),
                "Clicks, buzzing, hissing".to_string(),
                "Disorientation".to_string(),
            ],
            detection_methods: vec![
                "2-3 GHz spectrum analyzer".to_string(),
                "High-power RF detector".to_string(),
            ],
            countermeasures: vec![
                "Faraday cage".to_string(),
                "RF shielding".to_string(),
            ],
            notes: "Frey effect - microwave pulses heard as sounds".to_string(),
        },
    ]
}

// ============================================================================
// HAARP AND IONOSPHERIC HEATERS
// ============================================================================

/// Ionospheric heater facility data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonosphericHeater {
    pub name: String,
    pub location: String,
    pub coordinates: (f64, f64),
    pub freq_range_mhz: (f64, f64),
    pub power_mw: f64,  // Megawatts ERP
    pub status: String,
    pub capabilities: Vec<String>,
    pub detection_notes: String,
}

pub fn ionospheric_heater_database() -> Vec<IonosphericHeater> {
    vec![
        IonosphericHeater {
            name: "HAARP".to_string(),
            location: "Gakona, Alaska, USA".to_string(),
            coordinates: (62.39, -145.15),
            freq_range_mhz: (2.8, 10.0),
            power_mw: 3.6,
            status: "Active (University of Alaska Fairbanks)".to_string(),
            capabilities: vec![
                "Ionospheric modification".to_string(),
                "ELF/VLF generation".to_string(),
                "Artificial aurora creation".to_string(),
                "Radio propagation research".to_string(),
            ],
            detection_notes: "Transmissions announced publicly, 2.8-10 MHz detectable".to_string(),
        },
        IonosphericHeater {
            name: "EISCAT".to_string(),
            location: "Tromso, Norway".to_string(),
            coordinates: (69.58, 19.21),
            freq_range_mhz: (3.85, 8.0),
            power_mw: 1.0,
            status: "Active".to_string(),
            capabilities: vec![
                "Ionospheric research".to_string(),
                "Auroral studies".to_string(),
            ],
            detection_notes: "European ionospheric research facility".to_string(),
        },
        IonosphericHeater {
            name: "Sura".to_string(),
            location: "Vasilsursk, Russia".to_string(),
            coordinates: (56.1, 46.1),
            freq_range_mhz: (4.5, 9.3),
            power_mw: 0.19,
            status: "Active".to_string(),
            capabilities: vec![
                "Ionospheric heating".to_string(),
                "Artificial ionization".to_string(),
            ],
            detection_notes: "Russian facility, less publicly documented".to_string(),
        },
    ]
}

// ============================================================================
// INFRARED / IRDA COMMUNICATION
// ============================================================================

/// IR communication characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrCommunication {
    pub protocol: String,
    pub wavelength_nm: (u32, u32),
    pub modulation: String,
    pub data_rate: String,
    pub range_m: f64,
    pub security_notes: String,
}

pub fn ir_communication_database() -> Vec<IrCommunication> {
    vec![
        IrCommunication {
            protocol: "IrDA SIR".to_string(),
            wavelength_nm: (850, 900),
            modulation: "RZI (Return to Zero Inverted)".to_string(),
            data_rate: "9.6-115.2 kbps".to_string(),
            range_m: 1.0,
            security_notes: "Line of sight only, no spectrum congestion".to_string(),
        },
        IrCommunication {
            protocol: "IrDA FIR".to_string(),
            wavelength_nm: (850, 900),
            modulation: "4PPM".to_string(),
            data_rate: "4 Mbps".to_string(),
            range_m: 1.0,
            security_notes: "Higher speed, still requires alignment".to_string(),
        },
        IrCommunication {
            protocol: "Consumer IR (RC)".to_string(),
            wavelength_nm: (850, 950),
            modulation: "38 kHz carrier, various protocols".to_string(),
            data_rate: "1-2 kbps".to_string(),
            range_m: 10.0,
            security_notes: "NEC, Sony SIRC, RC5 protocols, easily intercepted".to_string(),
        },
        IrCommunication {
            protocol: "Military IRDA".to_string(),
            wavelength_nm: (1000, 1550),
            modulation: "Various encrypted".to_string(),
            data_rate: "Variable".to_string(),
            range_m: 1000.0,
            security_notes: "Line of sight, eye-safe bands, hard to intercept".to_string(),
        },
    ]
}

// ============================================================================
// NVG AND ELECTRONIC EMISSIONS
// ============================================================================

/// Night vision device emission characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvgEmissions {
    pub device_type: String,
    pub emission_type: String,
    pub frequency_or_wavelength: String,
    pub detectability: String,
    pub notes: String,
}

pub fn nvg_emission_database() -> Vec<NvgEmissions> {
    vec![
        NvgEmissions {
            device_type: "Gen 3 Image Intensifier".to_string(),
            emission_type: "Phosphor screen glow".to_string(),
            frequency_or_wavelength: "500-600 nm (green)".to_string(),
            detectability: "Low - but visible if eyes uncovered".to_string(),
            notes: "Green glow from eyepieces can be seen at distance".to_string(),
        },
        NvgEmissions {
            device_type: "Digital NVG".to_string(),
            emission_type: "Display backlight".to_string(),
            frequency_or_wavelength: "Visible + some IR leakage".to_string(),
            detectability: "Low to Medium".to_string(),
            notes: "LCD/OLED screens may leak light".to_string(),
        },
        NvgEmissions {
            device_type: "Active IR Illuminator".to_string(),
            emission_type: "IR flood light".to_string(),
            frequency_or_wavelength: "850 nm or 940 nm".to_string(),
            detectability: "HIGH - visible to other NVGs".to_string(),
            notes: "Active IR illumination easily detectable".to_string(),
        },
        NvgEmissions {
            device_type: "Thermal Imager".to_string(),
            emission_type: "Minimal".to_string(),
            frequency_or_wavelength: "N/A (passive)".to_string(),
            detectability: "Very low".to_string(),
            notes: "Purely passive, no emissions except display".to_string(),
        },
        NvgEmissions {
            device_type: "Anduril EagleEye".to_string(),
            emission_type: "RF mesh networking".to_string(),
            frequency_or_wavelength: "Unknown (Lattice mesh freq)".to_string(),
            detectability: "Medium - RF emissions".to_string(),
            notes: "Mesh networking creates detectable RF signature".to_string(),
        },
    ]
}

// ============================================================================
// MICROWAVE LINK SECURITY
// ============================================================================

/// Microwave link frequency bands and vulnerabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrowaveLinkBand {
    pub band_name: String,
    pub freq_range_ghz: (f64, f64),
    pub typical_use: String,
    pub path_length_km: (f64, f64),
    pub vulnerabilities: Vec<String>,
    pub detection_notes: String,
}

pub fn microwave_link_database() -> Vec<MicrowaveLinkBand> {
    vec![
        MicrowaveLinkBand {
            band_name: "6 GHz".to_string(),
            freq_range_ghz: (5.925, 6.875),
            typical_use: "Long-haul backbone".to_string(),
            path_length_km: (30.0, 60.0),
            vulnerabilities: vec![
                "Eavesdropping possible with directional antenna".to_string(),
                "Rain fade exploitation".to_string(),
                "Side-lobe interception".to_string(),
            ],
            detection_notes: "6 GHz links detectable with SDR".to_string(),
        },
        MicrowaveLinkBand {
            band_name: "11 GHz".to_string(),
            freq_range_ghz: (10.7, 11.7),
            typical_use: "Medium distance backbone".to_string(),
            path_length_km: (15.0, 40.0),
            vulnerabilities: vec![
                "Interception requires proximity".to_string(),
                "Encryption often not implemented".to_string(),
            ],
            detection_notes: "Common utility and telco backbone".to_string(),
        },
        MicrowaveLinkBand {
            band_name: "18 GHz".to_string(),
            freq_range_ghz: (17.7, 19.7),
            typical_use: "Cellular backhaul".to_string(),
            path_length_km: (5.0, 15.0),
            vulnerabilities: vec![
                "High density urban deployments".to_string(),
                "Narrow beams harder to intercept".to_string(),
            ],
            detection_notes: "Cellular tower backhaul common".to_string(),
        },
        MicrowaveLinkBand {
            band_name: "23 GHz".to_string(),
            freq_range_ghz: (21.2, 23.6),
            typical_use: "Short distance backhaul".to_string(),
            path_length_km: (2.0, 8.0),
            vulnerabilities: vec![
                "Very narrow beams".to_string(),
                "Atmospheric attenuation limits interception".to_string(),
            ],
            detection_notes: "Higher frequency, shorter range".to_string(),
        },
        MicrowaveLinkBand {
            band_name: "60 GHz (V-Band)".to_string(),
            freq_range_ghz: (57.0, 66.0),
            typical_use: "Building-to-building, 5G backhaul".to_string(),
            path_length_km: (0.5, 2.0),
            vulnerabilities: vec![
                "Oxygen absorption limits range".to_string(),
                "Very hard to intercept due to attenuation".to_string(),
            ],
            detection_notes: "Inherently secure due to atmospheric absorption".to_string(),
        },
        MicrowaveLinkBand {
            band_name: "80 GHz (E-Band)".to_string(),
            freq_range_ghz: (71.0, 86.0),
            typical_use: "High capacity short links".to_string(),
            path_length_km: (1.0, 5.0),
            vulnerabilities: vec![
                "Very directional".to_string(),
                "Requires physical proximity to intercept".to_string(),
            ],
            detection_notes: "10+ Gbps capacity, low intercept probability".to_string(),
        },
    ]
}

// ============================================================================
// COVERT AGENCY DETECTION METHODS
// ============================================================================

/// Known surveillance techniques by intelligence agencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovertTechnique {
    pub technique: String,
    pub agencies: Vec<String>,
    pub detection_method: String,
    pub rf_signature: Option<String>,
    pub notes: String,
}

pub fn covert_technique_database() -> Vec<CovertTechnique> {
    vec![
        CovertTechnique {
            technique: "Directed Energy Attack (AHI/Havana)".to_string(),
            agencies: vec!["GRU Unit 29155".to_string()],
            detection_method: "Broadband RF monitor 1-10 GHz, pulsed signal detection".to_string(),
            rf_signature: Some("Pulsed microwave, exact frequency unknown".to_string()),
            notes: "Brain injuries, cognitive effects documented since 2016".to_string(),
        },
        CovertTechnique {
            technique: "IMSI Catcher / Stingray".to_string(),
            agencies: vec!["FBI".to_string(), "Local police".to_string(), "Various".to_string()],
            detection_method: "Cell tower anomaly detection, forced 2G downgrade".to_string(),
            rf_signature: Some("700/850/1900 MHz cellular bands".to_string()),
            notes: "Forces phone to connect to fake tower".to_string(),
        },
        CovertTechnique {
            technique: "Van Eck Phreaking (TEMPEST)".to_string(),
            agencies: vec!["NSA".to_string(), "GCHQ".to_string(), "Various".to_string()],
            detection_method: "Shielded equipment, TEMPEST compliance".to_string(),
            rf_signature: Some("Unintentional RF from monitors/cables".to_string()),
            notes: "Intercepts EM emanations from electronics".to_string(),
        },
        CovertTechnique {
            technique: "Laser Microphone".to_string(),
            agencies: vec!["Various".to_string()],
            detection_method: "Laser detector, window vibration sensor".to_string(),
            rf_signature: None,
            notes: "Bounces laser off window to detect vibrations".to_string(),
        },
        CovertTechnique {
            technique: "Audio Masking (Dead Drop Spike)".to_string(),
            agencies: vec!["CIA".to_string(), "Various".to_string()],
            detection_method: "RF sweep, physical inspection".to_string(),
            rf_signature: Some("Various UHF/VHF bands".to_string()),
            notes: "Concealed listening device in ground/walls".to_string(),
        },
        CovertTechnique {
            technique: "Acoustic Kitty / Biological Implants".to_string(),
            agencies: vec!["CIA (historical)".to_string()],
            detection_method: "RF sweep of area".to_string(),
            rf_signature: Some("Low power VHF".to_string()),
            notes: "Historical - surgically implanted transmitters".to_string(),
        },
        CovertTechnique {
            technique: "GSM Bugging via SS7".to_string(),
            agencies: vec!["Various state actors".to_string()],
            detection_method: "SS7 network monitoring, encrypted calls".to_string(),
            rf_signature: None,
            notes: "Exploits SS7 protocol weaknesses".to_string(),
        },
        CovertTechnique {
            technique: "Mesh Network Surveillance (Lattice-type)".to_string(),
            agencies: vec!["US Military (Anduril)".to_string()],
            detection_method: "RF spectrum monitoring for mesh protocols".to_string(),
            rf_signature: Some("Unknown frequency, spread spectrum likely".to_string()),
            notes: "Networked sensor systems like EagleEye".to_string(),
        },
    ]
}

// ============================================================================
// THREAT DETECTION RECOMMENDATIONS
// ============================================================================

/// Recommended detection equipment for various threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionEquipment {
    pub threat_category: String,
    pub recommended_equipment: Vec<String>,
    pub frequency_coverage: String,
    pub estimated_cost: String,
}

pub fn detection_equipment_recommendations() -> Vec<DetectionEquipment> {
    vec![
        DetectionEquipment {
            threat_category: "Consumer Drones".to_string(),
            recommended_equipment: vec![
                "RTL-SDR (2.4/5.8 GHz capable)".to_string(),
                "DroneID decoder software".to_string(),
                "Directional antenna".to_string(),
            ],
            frequency_coverage: "2.4 GHz, 5.8 GHz ISM bands".to_string(),
            estimated_cost: "$50-200".to_string(),
        },
        DetectionEquipment {
            threat_category: "Long Range FPV".to_string(),
            recommended_equipment: vec![
                "SDR with 433/868/900 MHz coverage".to_string(),
                "Yagi or directional antenna".to_string(),
                "Signal analyzer software".to_string(),
            ],
            frequency_coverage: "400-928 MHz".to_string(),
            estimated_cost: "$100-500".to_string(),
        },
        DetectionEquipment {
            threat_category: "Directed Energy Weapons".to_string(),
            recommended_equipment: vec![
                "Broadband RF detector (1-100 GHz)".to_string(),
                "Spectrum analyzer with pulse capture".to_string(),
                "Millimeter wave detector (for ADS)".to_string(),
                "Sound level meter (for LRAD)".to_string(),
            ],
            frequency_coverage: "1 GHz - 95 GHz + acoustic".to_string(),
            estimated_cost: "$1000-10000".to_string(),
        },
        DetectionEquipment {
            threat_category: "Surveillance Bugs".to_string(),
            recommended_equipment: vec![
                "Wideband RF detector (25 MHz - 6 GHz)".to_string(),
                "Non-linear junction detector (NLJD)".to_string(),
                "Thermal camera".to_string(),
                "Power line analyzer (carrier current)".to_string(),
            ],
            frequency_coverage: "25 MHz - 6 GHz".to_string(),
            estimated_cost: "$500-5000".to_string(),
        },
        DetectionEquipment {
            threat_category: "IMSI Catcher".to_string(),
            recommended_equipment: vec![
                "IMSI catcher detector app".to_string(),
                "SDR monitoring cellular bands".to_string(),
                "Raspberry Pi with SIM800 + software".to_string(),
            ],
            frequency_coverage: "700/850/1900 MHz cellular".to_string(),
            estimated_cost: "$100-500".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uav_frequency_database() {
        let db = uav_frequency_database();
        assert!(db.len() >= 10);
        
        // Check 5.8 GHz video band exists
        assert!(db.iter().any(|b| b.name.contains("5.8 GHz")));
    }
    
    #[test]
    fn test_anduril_systems() {
        let systems = anduril_systems_database();
        assert!(systems.iter().any(|s| s.name == "Fury"));
        assert!(systems.iter().any(|s| s.name == "Roadrunner"));
        assert!(systems.iter().any(|s| s.name == "Dive-XL"));
        assert!(systems.iter().any(|s| s.name == "EagleEye"));
    }
    
    #[test]
    fn test_dew_database() {
        let dews = directed_energy_database();
        assert!(dews.iter().any(|d| d.name.contains("Active Denial")));
        assert!(dews.iter().any(|d| d.name.contains("LRAD")));
        assert!(dews.iter().any(|d| d.name.contains("Havana")));
    }
}
