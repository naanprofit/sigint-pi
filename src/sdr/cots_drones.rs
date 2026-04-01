//! Commercial Off-The-Shelf (COTS) Drone Detection Module
//!
//! Comprehensive database of:
//! - Chinese drone manufacturers and frequencies
//! - Drone motor/ESC EMI signatures
//! - Consumer radio services (FRS, GMRS, PMR446)
//! - Tactical military radios
//! - Loitering munitions / suicide drones
//!
//! For DEFENSIVE detection purposes only.

use serde::{Deserialize, Serialize};

// ============================================================================
// COTS DRONE DATABASE
// ============================================================================

/// Commercial drone manufacturer entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CotsDroneManufacturer {
    pub name: String,
    pub country: String,
    pub common_models: Vec<String>,
    pub frequency_bands: Vec<FrequencyBand>,
    pub transmission_system: String,
    pub remote_id: bool,
    pub detection_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyBand {
    pub name: String,
    pub freq_mhz: (f64, f64),
    pub power_mw: f64,
    pub modulation: String,
}

/// Comprehensive COTS drone database
pub fn cots_drone_database() -> Vec<CotsDroneManufacturer> {
    vec![
        // ===== CHINESE MANUFACTURERS =====
        CotsDroneManufacturer {
            name: "DJI".to_string(),
            country: "China".to_string(),
            common_models: vec![
                "Mavic 3/Air/Mini series".to_string(),
                "Phantom 4 Pro".to_string(),
                "Inspire 3".to_string(),
                "Matrice 300/350".to_string(),
                "Avata 2".to_string(),
                "FPV".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "OcuSync 2.4 GHz".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "FHSS OFDM".to_string(),
                },
                FrequencyBand {
                    name: "OcuSync 5.8 GHz (FCC)".to_string(),
                    freq_mhz: (5725.0, 5850.0),
                    power_mw: 400.0,
                    modulation: "FHSS OFDM".to_string(),
                },
            ],
            transmission_system: "OcuSync 3.0/4.0, O4".to_string(),
            remote_id: true,
            detection_notes: "DroneID broadcast on all models, easily identifiable via AeroScope or RF analysis. Uses proprietary FHSS, unique signal pattern.".to_string(),
        },
        CotsDroneManufacturer {
            name: "Autel Robotics".to_string(),
            country: "China (Shenzhen)".to_string(),
            common_models: vec![
                "EVO II Pro/Dual".to_string(),
                "EVO Lite+".to_string(),
                "EVO Max 4T".to_string(),
                "EVO Nano+".to_string(),
                "Dragonfish".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "SkyLink 2.4 GHz".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "FHSS".to_string(),
                },
                FrequencyBand {
                    name: "SkyLink 5.8 GHz".to_string(),
                    freq_mhz: (5725.0, 5850.0),
                    power_mw: 500.0,
                    modulation: "FHSS".to_string(),
                },
            ],
            transmission_system: "Autel SkyLink".to_string(),
            remote_id: true,
            detection_notes: "Similar to DJI frequencies but different protocol signature".to_string(),
        },
        CotsDroneManufacturer {
            name: "FIMI (Xiaomi)".to_string(),
            country: "China (Beijing)".to_string(),
            common_models: vec![
                "FIMI X8 SE 2022".to_string(),
                "FIMI X8 Mini".to_string(),
                "FIMI Mini 3".to_string(),
                "FIMI A3".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "2.4 GHz WiFi".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "WiFi 802.11n".to_string(),
                },
                FrequencyBand {
                    name: "5.8 GHz WiFi".to_string(),
                    freq_mhz: (5150.0, 5850.0),
                    power_mw: 200.0,
                    modulation: "WiFi 802.11ac".to_string(),
                },
            ],
            transmission_system: "Enhanced WiFi".to_string(),
            remote_id: false,
            detection_notes: "Uses standard WiFi protocols, easier to detect with WiFi scanners".to_string(),
        },
        CotsDroneManufacturer {
            name: "Hubsan".to_string(),
            country: "China (Shenzhen)".to_string(),
            common_models: vec![
                "Zino Mini Pro".to_string(),
                "Zino Mini SE".to_string(),
                "ACE Pro".to_string(),
                "Blackhawk 2".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "Syncleas 2.4 GHz".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "Proprietary".to_string(),
                },
                FrequencyBand {
                    name: "5.8 GHz".to_string(),
                    freq_mhz: (5725.0, 5850.0),
                    power_mw: 200.0,
                    modulation: "Proprietary".to_string(),
                },
            ],
            transmission_system: "Syncleas".to_string(),
            remote_id: false,
            detection_notes: "Lower-end consumer, often WiFi-based".to_string(),
        },
        CotsDroneManufacturer {
            name: "Potensic".to_string(),
            country: "China".to_string(),
            common_models: vec![
                "Atom SE".to_string(),
                "Dreamer Pro".to_string(),
                "Dreamer 4K".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "2.4 GHz WiFi".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "WiFi".to_string(),
                },
            ],
            transmission_system: "WiFi".to_string(),
            remote_id: false,
            detection_notes: "Budget drones, standard WiFi".to_string(),
        },
        CotsDroneManufacturer {
            name: "Holy Stone".to_string(),
            country: "China".to_string(),
            common_models: vec![
                "HS720E".to_string(),
                "HS175D".to_string(),
                "HS700D".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "2.4 GHz WiFi".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "WiFi".to_string(),
                },
            ],
            transmission_system: "WiFi".to_string(),
            remote_id: false,
            detection_notes: "Consumer toy grade, WiFi FPV".to_string(),
        },
        
        // ===== NON-CHINESE MANUFACTURERS =====
        CotsDroneManufacturer {
            name: "Parrot".to_string(),
            country: "France".to_string(),
            common_models: vec![
                "ANAFI USA".to_string(),
                "ANAFI Ai".to_string(),
                "ANAFI Thermal".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "WiFi 2.4 GHz".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "WiFi 802.11".to_string(),
                },
                FrequencyBand {
                    name: "WiFi 5 GHz".to_string(),
                    freq_mhz: (5150.0, 5850.0),
                    power_mw: 200.0,
                    modulation: "WiFi 802.11ac".to_string(),
                },
                FrequencyBand {
                    name: "4G LTE".to_string(),
                    freq_mhz: (700.0, 2600.0),
                    power_mw: 200.0,
                    modulation: "LTE".to_string(),
                },
            ],
            transmission_system: "WiFi / 4G LTE".to_string(),
            remote_id: true,
            detection_notes: "US government approved, uses standard WiFi + optional 4G".to_string(),
        },
        CotsDroneManufacturer {
            name: "Skydio".to_string(),
            country: "USA".to_string(),
            common_models: vec![
                "Skydio 2+".to_string(),
                "Skydio X2".to_string(),
                "Skydio X10".to_string(),
            ],
            frequency_bands: vec![
                FrequencyBand {
                    name: "Skydio Link 2.4 GHz".to_string(),
                    freq_mhz: (2400.0, 2483.5),
                    power_mw: 100.0,
                    modulation: "Proprietary".to_string(),
                },
                FrequencyBand {
                    name: "Skydio Link 5.2 GHz".to_string(),
                    freq_mhz: (5150.0, 5350.0),
                    power_mw: 200.0,
                    modulation: "Proprietary".to_string(),
                },
            ],
            transmission_system: "Skydio Link".to_string(),
            remote_id: true,
            detection_notes: "US-made, used by DOD, different RF fingerprint from Chinese drones".to_string(),
        },
    ]
}

// ============================================================================
// FPV RACING / HOBBY DRONES
// ============================================================================

/// FPV system frequency channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FpvSystem {
    pub name: String,
    pub frequency_band: String,
    pub freq_mhz: Vec<f64>,
    pub power_mw: (f64, f64),
    pub modulation: String,
    pub notes: String,
}

pub fn fpv_system_database() -> Vec<FpvSystem> {
    vec![
        FpvSystem {
            name: "5.8 GHz Analog (Raceband)".to_string(),
            frequency_band: "5.8 GHz ISM".to_string(),
            freq_mhz: vec![5658.0, 5695.0, 5732.0, 5769.0, 5806.0, 5843.0, 5880.0, 5917.0],
            power_mw: (25.0, 600.0),
            modulation: "FM Analog Video".to_string(),
            notes: "Most common FPV racing frequencies, R1-R8 channels".to_string(),
        },
        FpvSystem {
            name: "5.8 GHz Analog (Fatshark)".to_string(),
            frequency_band: "5.8 GHz ISM".to_string(),
            freq_mhz: vec![5740.0, 5760.0, 5780.0, 5800.0, 5820.0, 5840.0, 5860.0, 5880.0],
            power_mw: (25.0, 600.0),
            modulation: "FM Analog Video".to_string(),
            notes: "Fatshark band F1-F8".to_string(),
        },
        FpvSystem {
            name: "DJI Digital FPV".to_string(),
            frequency_band: "5.8 GHz".to_string(),
            freq_mhz: vec![5660.0, 5700.0, 5745.0, 5785.0, 5825.0, 5865.0],
            power_mw: (25.0, 700.0),
            modulation: "OFDM Digital".to_string(),
            notes: "DJI FPV system, 50 Mbps, 720p/1080p".to_string(),
        },
        FpvSystem {
            name: "HDZero".to_string(),
            frequency_band: "5.8 GHz".to_string(),
            freq_mhz: vec![5658.0, 5695.0, 5732.0, 5769.0, 5806.0, 5843.0, 5880.0, 5917.0],
            power_mw: (25.0, 400.0),
            modulation: "Digital (low latency)".to_string(),
            notes: "Open-source digital FPV, <4ms latency".to_string(),
        },
        FpvSystem {
            name: "Walksnail Avatar".to_string(),
            frequency_band: "5.8 GHz".to_string(),
            freq_mhz: vec![5660.0, 5700.0, 5745.0, 5785.0, 5825.0, 5865.0],
            power_mw: (25.0, 700.0),
            modulation: "OFDM Digital".to_string(),
            notes: "Caddx/Walksnail, 1080p digital FPV".to_string(),
        },
        
        // Long range control links
        FpvSystem {
            name: "TBS Crossfire".to_string(),
            frequency_band: "900 MHz ISM".to_string(),
            freq_mhz: vec![868.0, 915.0],  // EU/US centers
            power_mw: (10.0, 2000.0),
            modulation: "LoRa FHSS".to_string(),
            notes: "Long range 30-40km+, used for control link".to_string(),
        },
        FpvSystem {
            name: "ExpressLRS 900".to_string(),
            frequency_band: "900 MHz ISM".to_string(),
            freq_mhz: vec![868.0, 915.0],
            power_mw: (10.0, 1000.0),
            modulation: "LoRa FHSS".to_string(),
            notes: "Open-source long range, 150Hz+ update rate".to_string(),
        },
        FpvSystem {
            name: "ExpressLRS 2.4G".to_string(),
            frequency_band: "2.4 GHz ISM".to_string(),
            freq_mhz: vec![2400.0, 2483.0],
            power_mw: (10.0, 250.0),
            modulation: "LoRa FHSS".to_string(),
            notes: "Open-source, 500Hz update rate possible".to_string(),
        },
        FpvSystem {
            name: "1.2 GHz Video".to_string(),
            frequency_band: "1.2-1.3 GHz".to_string(),
            freq_mhz: vec![1080.0, 1120.0, 1160.0, 1200.0, 1240.0, 1280.0, 1320.0, 1360.0],
            power_mw: (200.0, 1500.0),
            modulation: "FM Analog Video".to_string(),
            notes: "Long range video, requires ham license in US".to_string(),
        },
    ]
}

// ============================================================================
// MOTOR / ESC EMI SIGNATURES
// ============================================================================

/// Drone motor EMI characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorEscSignature {
    pub motor_type: String,
    pub kv_rating: String,
    pub pwm_freq_khz: f64,
    pub emi_harmonics_khz: Vec<f64>,
    pub current_draw_amps: (f64, f64),
    pub detection_range_m: f64,
    pub notes: String,
}

/// Common ESC brands and their PWM frequencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscBrand {
    pub name: String,
    pub pwm_freq_khz: Vec<f64>,
    pub protocol: String,
    pub common_current_ratings: Vec<u32>,
    pub notes: String,
}

pub fn esc_brand_database() -> Vec<EscBrand> {
    vec![
        EscBrand {
            name: "BLHeli_S".to_string(),
            pwm_freq_khz: vec![24.0, 48.0],
            protocol: "DShot150/300/600".to_string(),
            common_current_ratings: vec![20, 30, 35, 45],
            notes: "Most common hobby ESC firmware, 24/48kHz switching".to_string(),
        },
        EscBrand {
            name: "BLHeli_32".to_string(),
            pwm_freq_khz: vec![24.0, 48.0, 96.0],
            protocol: "DShot300/600/1200, Bi-directional".to_string(),
            common_current_ratings: vec![35, 45, 55, 65],
            notes: "32-bit ESCs, higher PWM frequencies possible".to_string(),
        },
        EscBrand {
            name: "AM32".to_string(),
            pwm_freq_khz: vec![24.0, 48.0, 96.0],
            protocol: "DShot, Bidirectional".to_string(),
            common_current_ratings: vec![35, 45, 55],
            notes: "Open-source 32-bit alternative to BLHeli_32".to_string(),
        },
        EscBrand {
            name: "KISS".to_string(),
            pwm_freq_khz: vec![24.0, 32.0],
            protocol: "OneShot, DShot".to_string(),
            common_current_ratings: vec![25, 32],
            notes: "High-end racing ESCs".to_string(),
        },
        EscBrand {
            name: "T-Motor".to_string(),
            pwm_freq_khz: vec![8.0, 16.0, 24.0],
            protocol: "PWM, DShot".to_string(),
            common_current_ratings: vec![40, 55, 80],
            notes: "Industrial/cinematic drone ESCs".to_string(),
        },
        EscBrand {
            name: "Hobbywing XRotor".to_string(),
            pwm_freq_khz: vec![8.0, 16.0],
            protocol: "PWM".to_string(),
            common_current_ratings: vec![40, 60, 80],
            notes: "Industrial drone ESCs, lower switching frequency".to_string(),
        },
    ]
}

/// Motor EMI detection signatures
pub fn motor_emi_database() -> Vec<MotorEscSignature> {
    vec![
        MotorEscSignature {
            motor_type: "2205-2300KV Racing".to_string(),
            kv_rating: "2300 KV".to_string(),
            pwm_freq_khz: 48.0,
            emi_harmonics_khz: vec![48.0, 96.0, 144.0, 192.0, 240.0],
            current_draw_amps: (0.5, 25.0),
            detection_range_m: 50.0,
            notes: "Common 5-inch FPV racing motor".to_string(),
        },
        MotorEscSignature {
            motor_type: "2806.5-1300KV Long Range".to_string(),
            kv_rating: "1300 KV".to_string(),
            pwm_freq_khz: 24.0,
            emi_harmonics_khz: vec![24.0, 48.0, 72.0, 96.0],
            current_draw_amps: (0.3, 20.0),
            detection_range_m: 75.0,
            notes: "7-inch long range FPV motor".to_string(),
        },
        MotorEscSignature {
            motor_type: "DJI Mavic (proprietary)".to_string(),
            kv_rating: "1600 KV".to_string(),
            pwm_freq_khz: 16.0,
            emi_harmonics_khz: vec![16.0, 32.0, 48.0, 64.0],
            current_draw_amps: (0.5, 15.0),
            detection_range_m: 30.0,
            notes: "Consumer DJI motors, well shielded".to_string(),
        },
        MotorEscSignature {
            motor_type: "Industrial U8/U13".to_string(),
            kv_rating: "100-170 KV".to_string(),
            pwm_freq_khz: 8.0,
            emi_harmonics_khz: vec![8.0, 16.0, 24.0, 32.0],
            current_draw_amps: (1.0, 60.0),
            detection_range_m: 150.0,
            notes: "Heavy lift/industrial motors, high current draw".to_string(),
        },
    ]
}

// ============================================================================
// CONSUMER RADIO SERVICES
// ============================================================================

/// FRS/GMRS/PMR446 channel database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerRadioService {
    pub name: String,
    pub region: String,
    pub freq_mhz: Vec<f64>,
    pub power_watts: f64,
    pub channel_spacing_khz: f64,
    pub license_required: bool,
    pub notes: String,
}

pub fn consumer_radio_database() -> Vec<ConsumerRadioService> {
    vec![
        ConsumerRadioService {
            name: "FRS (Family Radio Service)".to_string(),
            region: "USA".to_string(),
            freq_mhz: vec![
                462.5625, 462.5875, 462.6125, 462.6375, 462.6625, 462.6875, 462.7125,
                467.5625, 467.5875, 467.6125, 467.6375, 467.6625, 467.6875, 467.7125,
                462.5500, 462.5750, 462.6000, 462.6250, 462.6500, 462.6750, 462.7000, 462.7250,
            ],
            power_watts: 2.0,
            channel_spacing_khz: 12.5,
            license_required: false,
            notes: "22 channels, Ch 1-7 and 15-22 are 2W, Ch 8-14 are 0.5W".to_string(),
        },
        ConsumerRadioService {
            name: "GMRS (General Mobile Radio Service)".to_string(),
            region: "USA".to_string(),
            freq_mhz: vec![
                462.5500, 462.5750, 462.6000, 462.6250, 462.6500, 462.6750, 462.7000, 462.7250,
                467.5500, 467.5750, 467.6000, 467.6250, 467.6500, 467.6750, 467.7000, 467.7250,
            ],
            power_watts: 50.0,
            channel_spacing_khz: 25.0,
            license_required: true,
            notes: "Up to 50W on main channels, repeater capable".to_string(),
        },
        ConsumerRadioService {
            name: "PMR446".to_string(),
            region: "Europe/UK".to_string(),
            freq_mhz: vec![
                446.00625, 446.01875, 446.03125, 446.04375,
                446.05625, 446.06875, 446.08125, 446.09375,
                446.10625, 446.11875, 446.13125, 446.14375,
                446.15625, 446.16875, 446.18125, 446.19375,
            ],
            power_watts: 0.5,
            channel_spacing_khz: 12.5,
            license_required: false,
            notes: "16 analog + 16 digital channels, 500mW max".to_string(),
        },
        ConsumerRadioService {
            name: "MURS (Multi-Use Radio Service)".to_string(),
            region: "USA".to_string(),
            freq_mhz: vec![151.820, 151.880, 151.940, 154.570, 154.600],
            power_watts: 2.0,
            channel_spacing_khz: 11.25,
            license_required: false,
            notes: "5 VHF channels, better range than UHF FRS".to_string(),
        },
        ConsumerRadioService {
            name: "CB Radio".to_string(),
            region: "USA/Worldwide".to_string(),
            freq_mhz: vec![
                26.965, 26.975, 26.985, 27.005, 27.015, 27.025, 27.035,
                27.055, 27.065, 27.075, 27.085, 27.105, 27.115, 27.125,
                27.135, 27.155, 27.165, 27.175, 27.185, 27.205, 27.215,
                27.225, 27.235, 27.245, 27.255, 27.265, 27.275, 27.285,
                27.295, 27.305, 27.315, 27.325, 27.335, 27.345, 27.355,
                27.365, 27.375, 27.385, 27.395, 27.405,
            ],
            power_watts: 4.0,
            channel_spacing_khz: 10.0,
            license_required: false,
            notes: "40 channels, 27 MHz band, AM/SSB modes".to_string(),
        },
    ]
}

// ============================================================================
// MILITARY TACTICAL RADIOS
// ============================================================================

/// Military tactical radio systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalRadio {
    pub designation: String,
    pub name: String,
    pub manufacturer: String,
    pub freq_range_mhz: (f64, f64),
    pub modes: Vec<String>,
    pub encryption: String,
    pub power_watts: f64,
    pub detection_notes: String,
}

pub fn tactical_radio_database() -> Vec<TacticalRadio> {
    vec![
        TacticalRadio {
            designation: "AN/PRC-152".to_string(),
            name: "Falcon III".to_string(),
            manufacturer: "L3Harris".to_string(),
            freq_range_mhz: (30.0, 512.0),
            modes: vec!["VHF/UHF".to_string(), "SINCGARS".to_string(), "HAVEQUICK".to_string(), "SATCOM".to_string()],
            encryption: "AES-256, Type-1".to_string(),
            power_watts: 5.0,
            detection_notes: "Wideband SDR, frequency hopping detectable but not decodable".to_string(),
        },
        TacticalRadio {
            designation: "AN/PRC-148".to_string(),
            name: "MBITR (Multiband Inter/Intra Team Radio)".to_string(),
            manufacturer: "Thales".to_string(),
            freq_range_mhz: (30.0, 512.0),
            modes: vec!["VHF/UHF".to_string(), "SINCGARS".to_string()],
            encryption: "AES-256, Type-1".to_string(),
            power_watts: 5.0,
            detection_notes: "Widely used in Iraq/Afghanistan, frequency hopping".to_string(),
        },
        TacticalRadio {
            designation: "AN/PRC-117G".to_string(),
            name: "Falcon III Manpack".to_string(),
            manufacturer: "L3Harris".to_string(),
            freq_range_mhz: (30.0, 2500.0),
            modes: vec!["VHF/UHF".to_string(), "SATCOM".to_string(), "HAVEQUICK".to_string(), "SINCGARS".to_string()],
            encryption: "AES-256, Type-1".to_string(),
            power_watts: 20.0,
            detection_notes: "Multiband manpack, includes SATCOM uplink".to_string(),
        },
        TacticalRadio {
            designation: "SINCGARS".to_string(),
            name: "Single Channel Ground and Airborne Radio System".to_string(),
            manufacturer: "Various".to_string(),
            freq_range_mhz: (30.0, 88.0),
            modes: vec!["VHF FM".to_string(), "Frequency Hopping".to_string()],
            encryption: "COMSEC".to_string(),
            power_watts: 50.0,
            detection_notes: "Hops 100+ times/sec across 2320 channels".to_string(),
        },
        TacticalRadio {
            designation: "RT-1523".to_string(),
            name: "SINCGARS RT".to_string(),
            manufacturer: "ITT/Harris".to_string(),
            freq_range_mhz: (30.0, 87.975),
            modes: vec!["VHF FM".to_string(), "Single Channel".to_string(), "FH".to_string()],
            encryption: "COMSEC".to_string(),
            power_watts: 50.0,
            detection_notes: "Vehicle/manpack SINCGARS radio".to_string(),
        },
        TacticalRadio {
            designation: "Baofeng UV-5R".to_string(),
            name: "UV-5R (Insurgent commonly used)".to_string(),
            manufacturer: "Baofeng".to_string(),
            freq_range_mhz: (136.0, 174.0),  // VHF
            modes: vec!["VHF FM".to_string(), "UHF FM".to_string()],
            encryption: "None or basic CTCSS/DCS".to_string(),
            power_watts: 5.0,
            detection_notes: "Commonly used by insurgents, civilians. UHF: 400-520 MHz".to_string(),
        },
    ]
}

// ============================================================================
// LOITERING MUNITIONS / SUICIDE DRONES
// ============================================================================

/// Loitering munition / suicide drone specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoiteringMunition {
    pub name: String,
    pub country: String,
    pub guidance: Vec<String>,
    pub communication: Vec<String>,
    pub propulsion: String,
    pub acoustic_signature: String,
    pub detection_methods: Vec<String>,
    pub notes: String,
}

pub fn loitering_munition_database() -> Vec<LoiteringMunition> {
    vec![
        LoiteringMunition {
            name: "AeroVironment Switchblade 300".to_string(),
            country: "USA".to_string(),
            guidance: vec!["GPS/INS".to_string(), "EO/IR seeker".to_string(), "Operator control".to_string()],
            communication: vec!["Encrypted digital link".to_string(), "Line of sight".to_string()],
            propulsion: "Electric (quiet)".to_string(),
            acoustic_signature: "Very low".to_string(),
            detection_methods: vec![
                "Radar (small RCS)".to_string(),
                "IR (low signature)".to_string(),
                "RF data link detection".to_string(),
            ],
            notes: "Tube-launched, 10km range, anti-personnel".to_string(),
        },
        LoiteringMunition {
            name: "AeroVironment Switchblade 600".to_string(),
            country: "USA".to_string(),
            guidance: vec!["GPS/INS".to_string(), "EO/IR seeker".to_string(), "Javelin warhead".to_string()],
            communication: vec!["Encrypted digital link".to_string()],
            propulsion: "Electric (quiet)".to_string(),
            acoustic_signature: "Low".to_string(),
            detection_methods: vec![
                "Radar".to_string(),
                "IR".to_string(),
                "RF link".to_string(),
            ],
            notes: "Anti-armor, 40km range, top-attack Javelin warhead".to_string(),
        },
        LoiteringMunition {
            name: "ZALA Lancet".to_string(),
            country: "Russia".to_string(),
            guidance: vec!["GPS/GLONASS".to_string(), "EO seeker".to_string(), "AI target recognition".to_string()],
            communication: vec!["Data link to ZALA Orlan spotter".to_string()],
            propulsion: "Electric".to_string(),
            acoustic_signature: "Low".to_string(),
            detection_methods: vec![
                "Radar".to_string(),
                "Acoustic detection".to_string(),
                "RF link to Orlan drone".to_string(),
            ],
            notes: "Works with Orlan-10 spotter drone, used extensively in Ukraine".to_string(),
        },
        LoiteringMunition {
            name: "Shahed-136 / Geran-2".to_string(),
            country: "Iran/Russia".to_string(),
            guidance: vec!["GPS".to_string(), "GLONASS".to_string(), "INS backup".to_string()],
            communication: vec!["Minimal (autonomous)".to_string()],
            propulsion: "Moped engine (loud)".to_string(),
            acoustic_signature: "HIGH - distinctive engine noise".to_string(),
            detection_methods: vec![
                "Acoustic (lawn mower sound)".to_string(),
                "Radar (small but detectable)".to_string(),
                "IR from engine".to_string(),
            ],
            notes: "Very cheap, mass produced, GPS jammable".to_string(),
        },
        LoiteringMunition {
            name: "IAI Harop".to_string(),
            country: "Israel".to_string(),
            guidance: vec!["Anti-radiation homing".to_string(), "EO seeker".to_string(), "Operator control".to_string()],
            communication: vec!["Encrypted SATCOM".to_string(), "LOS data link".to_string()],
            propulsion: "Turbojet".to_string(),
            acoustic_signature: "Medium (jet engine)".to_string(),
            detection_methods: vec![
                "Radar".to_string(),
                "IR (jet signature)".to_string(),
                "Acoustic".to_string(),
            ],
            notes: "Anti-radar SEAD weapon, 6 hour loiter time".to_string(),
        },
        LoiteringMunition {
            name: "Hero-120".to_string(),
            country: "Israel (UVision)".to_string(),
            guidance: vec!["EO/IR".to_string(), "GPS".to_string()],
            communication: vec!["Data link".to_string()],
            propulsion: "Electric".to_string(),
            acoustic_signature: "Low".to_string(),
            detection_methods: vec![
                "Radar".to_string(),
                "IR".to_string(),
            ],
            notes: "Anti-armor loitering munition".to_string(),
        },
    ]
}

// ============================================================================
// SATELLITE COMMUNICATION (UAV SATCOM)
// ============================================================================

/// Satellite communication bands for UAVs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatcomBand {
    pub name: String,
    pub uplink_ghz: (f64, f64),
    pub downlink_ghz: (f64, f64),
    pub typical_use: String,
    pub detection_notes: String,
}

pub fn uav_satcom_bands() -> Vec<SatcomBand> {
    vec![
        SatcomBand {
            name: "UHF SATCOM".to_string(),
            uplink_ghz: (0.292, 0.320),
            downlink_ghz: (0.243, 0.270),
            typical_use: "Military tactical, low data rate".to_string(),
            detection_notes: "Low frequency, wide beams, easier to detect direction".to_string(),
        },
        SatcomBand {
            name: "L-Band".to_string(),
            uplink_ghz: (1.626, 1.660),
            downlink_ghz: (1.525, 1.559),
            typical_use: "Iridium, Inmarsat, low data rate C2".to_string(),
            detection_notes: "Used by commercial and some military systems".to_string(),
        },
        SatcomBand {
            name: "S-Band".to_string(),
            uplink_ghz: (2.0, 2.3),
            downlink_ghz: (1.8, 2.1),
            typical_use: "Telemetry, tracking, command".to_string(),
            detection_notes: "Common for smaller UAVs".to_string(),
        },
        SatcomBand {
            name: "C-Band".to_string(),
            uplink_ghz: (5.925, 6.425),
            downlink_ghz: (3.7, 4.2),
            typical_use: "Larger UAV video/data links".to_string(),
            detection_notes: "Medium dishes required".to_string(),
        },
        SatcomBand {
            name: "Ku-Band".to_string(),
            uplink_ghz: (14.0, 14.5),
            downlink_ghz: (11.7, 12.7),
            typical_use: "MQ-9 Reaper, Global Hawk BLOS".to_string(),
            detection_notes: "PRIMARY MILITARY UAV BAND, high bandwidth".to_string(),
        },
        SatcomBand {
            name: "Ka-Band".to_string(),
            uplink_ghz: (27.5, 31.0),
            downlink_ghz: (17.7, 21.2),
            typical_use: "Next-gen high bandwidth UAV links".to_string(),
            detection_notes: "Higher frequency, smaller antennas, rain fade issues".to_string(),
        },
        SatcomBand {
            name: "X-Band (Military)".to_string(),
            uplink_ghz: (7.9, 8.4),
            downlink_ghz: (7.25, 7.75),
            typical_use: "Military SATCOM, wideband".to_string(),
            detection_notes: "Dedicated military band".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_drone_database() {
        let drones = cots_drone_database();
        assert!(drones.iter().any(|d| d.name == "DJI"));
        assert!(drones.iter().any(|d| d.name == "Autel Robotics"));
        assert!(drones.iter().any(|d| d.name == "Skydio"));
    }
    
    #[test]
    fn test_consumer_radio() {
        let radios = consumer_radio_database();
        assert!(radios.iter().any(|r| r.name.contains("FRS")));
        assert!(radios.iter().any(|r| r.name.contains("PMR446")));
    }
    
    #[test]
    fn test_loitering_munitions() {
        let lm = loitering_munition_database();
        assert!(lm.iter().any(|l| l.name.contains("Switchblade")));
        assert!(lm.iter().any(|l| l.name.contains("Lancet")));
    }
}
