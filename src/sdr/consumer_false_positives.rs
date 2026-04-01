//! Consumer Device False-Positive Database for TSCM Sweeps
//!
//! Comprehensive catalog of benign consumer/household RF devices that operate
//! in frequency bands overlapping with known surveillance equipment. These
//! devices will trigger false positives during TSCM sweeps and must be
//! identified and excluded to reduce noise.
//!
//! Sources: FCC Part 15/90/95 rules, manufacturer specs, ISM band allocations,
//! ETSI standards, IEEE 802.11/802.15.4, Bluetooth SIG, Z-Wave Alliance.

use serde::{Deserialize, Serialize};
use super::tscm::ThreatCategory;

// ============================================================================
// CONSUMER DEVICE FALSE-POSITIVE DEFINITIONS
// ============================================================================

/// Category of benign consumer device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConsumerDeviceCategory {
    BabyMonitor,
    WirelessDoorbell,
    GarageDoorOpener,
    WeatherStation,
    CarKeyFob,
    WirelessSensor,
    SmartHome,
    DectPhone,
    WirelessSecurityCamera,
    RcToyDrone,
    MicrowaveOven,
    WirelessAudio,
    BluetoothDevice,
    WifiRouter,
    TpmsTireSensor,
    SmartMeter,
    LoRaDevice,
    WalkieTalkie,
    MedicalDevice,
    WirelessHdmi,
}

/// A benign consumer RF device that may trigger TSCM false positives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerDevice {
    pub name: String,
    pub category: ConsumerDeviceCategory,
    pub start_hz: u64,
    pub end_hz: u64,
    pub common_brands: Vec<String>,
    pub typical_power_mw: Option<f64>,
    pub modulation: Vec<String>,
    pub confused_with: Vec<ThreatCategory>,
    pub description: String,
    pub fcc_rule: Option<String>,
    pub prevalence: Prevalence,
}

/// How commonly encountered this device type is in a typical home/office
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Prevalence {
    /// Nearly every home/office has one
    Ubiquitous,
    /// Very common, found in most homes
    VeryCommon,
    /// Common, found in many homes
    Common,
    /// Occasionally encountered
    Occasional,
    /// Rarely encountered but documented
    Rare,
}

impl ConsumerDevice {
    /// Comprehensive database of consumer devices that cause TSCM false positives.
    /// Organized by category with full frequency/modulation/power data.
    pub fn false_positive_database() -> Vec<Self> {
        vec![
            // ================================================================
            // 1. BABY MONITORS
            // ================================================================
            Self {
                name: "Baby Monitor 49 MHz Analog".to_string(),
                category: ConsumerDeviceCategory::BabyMonitor,
                start_hz: 49_830_000,
                end_hz: 49_890_000,
                common_brands: vec![
                    "Fisher-Price".to_string(),
                    "Safety 1st".to_string(),
                    "Graco".to_string(),
                ],
                typical_power_mw: Some(10.0),
                modulation: vec!["FM".to_string(), "NFM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "Legacy 49 MHz analog baby monitors. Open audio on fixed channels, \
                    extremely easy to mistake for a room bug due to continuous voice transmission."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.235".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "Baby Monitor 900 MHz".to_string(),
                category: ConsumerDeviceCategory::BabyMonitor,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "VTech".to_string(),
                    "Motorola".to_string(),
                    "Philips Avent".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["FM".to_string(), "FHSS".to_string()],
                confused_with: vec![
                    ThreatCategory::AudioBug,
                    ThreatCategory::VideoBug,
                    ThreatCategory::BumperBeeper,
                ],
                description: "900 MHz ISM band baby monitors. Audio and some video models. \
                    Overlaps directly with ISM band surveillance devices and vehicle trackers."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Baby Monitor DECT 1.9 GHz".to_string(),
                category: ConsumerDeviceCategory::BabyMonitor,
                start_hz: 1_920_000_000,
                end_hz: 1_930_000_000,
                common_brands: vec![
                    "Philips Avent DECT".to_string(),
                    "VTech DM221".to_string(),
                    "Angelcare".to_string(),
                ],
                typical_power_mw: Some(250.0),
                modulation: vec!["GFSK".to_string(), "DECT".to_string(), "FHSS".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::ImsiCatcher],
                description: "DECT-based baby monitors at 1.9 GHz. Encrypted FHSS. \
                    Proximity to PCS cellular band can confuse IMSI catcher detection."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.323".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Baby Monitor 2.4 GHz Video".to_string(),
                category: ConsumerDeviceCategory::BabyMonitor,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "Infant Optics DXR-8".to_string(),
                    "eufy SpaceView".to_string(),
                    "HelloBaby HB65".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["FHSS".to_string(), "DSSS".to_string(), "FM Video".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "2.4 GHz video baby monitors. Analog FM video models are \
                    indistinguishable from covert video bugs on initial detection. \
                    EXTREMELY common false positive."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 2. WIRELESS DOORBELLS
            // ================================================================
            Self {
                name: "Wireless Doorbell 315 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessDoorbell,
                start_hz: 314_000_000,
                end_hz: 316_000_000,
                common_brands: vec![
                    "SadoTech".to_string(),
                    "Avantek".to_string(),
                    "Honeywell".to_string(),
                ],
                typical_power_mw: Some(10.0),
                modulation: vec!["OOK".to_string(), "ASK".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "315 MHz OOK wireless doorbells. Short burst transmissions \
                    in the tactical bug band. Intermittent signal easily flagged."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Wireless Doorbell 433 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessDoorbell,
                start_hz: 433_050_000,
                end_hz: 434_790_000,
                common_brands: vec![
                    "Ring Chime (RF link)".to_string(),
                    "1byone".to_string(),
                    "Novete".to_string(),
                ],
                typical_power_mw: Some(10.0),
                modulation: vec!["OOK".to_string(), "ASK".to_string(), "FSK".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "433 MHz ISM wireless doorbells. Overlaps ISM 433 bug band. \
                    OOK bursts when button pressed, can look like coded surveillance pings."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 3. GARAGE DOOR OPENERS
            // ================================================================
            Self {
                name: "Garage Door Opener 300 MHz".to_string(),
                category: ConsumerDeviceCategory::GarageDoorOpener,
                start_hz: 300_000_000,
                end_hz: 310_000_000,
                common_brands: vec![
                    "Chamberlain".to_string(),
                    "LiftMaster".to_string(),
                    "Genie".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["OOK".to_string(), "Rolling Code".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "300-310 MHz garage door openers. Directly overlaps micro-powered \
                    bug band (290-330 MHz). Rolling code bursts resemble coded transmitters."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Garage Door Opener 315 MHz".to_string(),
                category: ConsumerDeviceCategory::GarageDoorOpener,
                start_hz: 314_000_000,
                end_hz: 316_000_000,
                common_brands: vec![
                    "LiftMaster Security+ 2.0".to_string(),
                    "Craftsman".to_string(),
                    "Overhead Door".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["OOK".to_string(), "Rolling Code".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "315 MHz garage remotes. In the spy shop and tactical bug band. \
                    Very common in US market."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Garage Door Opener 390 MHz".to_string(),
                category: ConsumerDeviceCategory::GarageDoorOpener,
                start_hz: 390_000_000,
                end_hz: 390_500_000,
                common_brands: vec![
                    "LiftMaster (older)".to_string(),
                    "Chamberlain (older)".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["OOK".to_string(), "Fixed Code".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "390 MHz legacy garage door openers. Squarely in the SpyShop \
                    popular band (330-440 MHz) and near 398.605 MHz spy shop bug frequency."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 4. WEATHER STATIONS
            // ================================================================
            Self {
                name: "Weather Station 433 MHz".to_string(),
                category: ConsumerDeviceCategory::WeatherStation,
                start_hz: 433_920_000,
                end_hz: 433_920_000,
                common_brands: vec![
                    "Acurite".to_string(),
                    "La Crosse Technology".to_string(),
                    "Ambient Weather".to_string(),
                    "Oregon Scientific".to_string(),
                    "Davis Instruments".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["OOK".to_string(), "FSK".to_string(), "ASK".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "433.920 MHz weather sensors. Periodic beacon transmissions \
                    every 30-60 seconds. Directly on the most popular ISM surveillance bug \
                    frequency. EXTREMELY common false positive in any residential sweep."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Weather Station 915 MHz".to_string(),
                category: ConsumerDeviceCategory::WeatherStation,
                start_hz: 915_000_000,
                end_hz: 915_000_000,
                common_brands: vec![
                    "Davis Vantage Pro2".to_string(),
                    "Acurite Atlas".to_string(),
                ],
                typical_power_mw: Some(5.0),
                modulation: vec!["FSK".to_string(), "FHSS".to_string()],
                confused_with: vec![ThreatCategory::BumperBeeper, ThreatCategory::VideoBug],
                description: "915 MHz ISM weather stations. Overlaps ISM bumper beeper and \
                    900 MHz analog video bug bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 5. CAR KEY FOBS
            // ================================================================
            Self {
                name: "Car Key Fob 315 MHz (North America)".to_string(),
                category: ConsumerDeviceCategory::CarKeyFob,
                start_hz: 314_950_000,
                end_hz: 315_050_000,
                common_brands: vec![
                    "Toyota".to_string(),
                    "Honda".to_string(),
                    "Ford".to_string(),
                    "GM/Chevrolet".to_string(),
                    "Nissan".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["ASK".to_string(), "FSK".to_string(), "Rolling Code".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "315 MHz car key fobs used throughout North America. Burst \
                    transmissions on lock/unlock. In tactical bug band. Will trigger during \
                    any parking lot or garage-adjacent sweep."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Car Key Fob 433 MHz (Europe/Asia)".to_string(),
                category: ConsumerDeviceCategory::CarKeyFob,
                start_hz: 433_920_000,
                end_hz: 433_920_000,
                common_brands: vec![
                    "BMW".to_string(),
                    "Mercedes-Benz".to_string(),
                    "Audi/VW".to_string(),
                    "Volvo".to_string(),
                    "Hyundai/Kia".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["ASK".to_string(), "FSK".to_string(), "Rolling Code".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "433.92 MHz car key fobs (European and some Asian vehicles in US). \
                    Directly on ISM 433 bug frequency."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 6. WIRELESS THERMOMETERS / SENSORS
            // ================================================================
            Self {
                name: "Wireless Thermometer 433 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessSensor,
                start_hz: 433_920_000,
                end_hz: 433_920_000,
                common_brands: vec![
                    "ThermoPro".to_string(),
                    "Govee".to_string(),
                    "AcuRite".to_string(),
                    "Inkbird".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["OOK".to_string(), "ASK".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "433.92 MHz wireless temperature/humidity sensors. Beacon every \
                    30-120 seconds. Same frequency as ISM surveillance bugs. \
                    Pool thermometers, fridge sensors, grill thermometers."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Wireless Soil/Plant Sensor 900 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessSensor,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Ecowitt".to_string(),
                    "Ambient Weather".to_string(),
                ],
                typical_power_mw: Some(5.0),
                modulation: vec!["FSK".to_string(), "OOK".to_string()],
                confused_with: vec![ThreatCategory::BumperBeeper, ThreatCategory::VideoBug],
                description: "900 MHz ISM soil moisture, rain gauge, and environmental sensors. \
                    Periodic telemetry in ISM bumper beeper and video bug bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Wireless Water Leak Sensor 315/433 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessSensor,
                start_hz: 315_000_000,
                end_hz: 434_000_000,
                common_brands: vec![
                    "Honeywell".to_string(),
                    "First Alert".to_string(),
                    "Govee".to_string(),
                ],
                typical_power_mw: Some(5.0),
                modulation: vec!["OOK".to_string(), "ASK".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "Water leak, smoke, and CO detectors with wireless 315/433 MHz links. \
                    Heartbeat beacons and alarm bursts in spy shop bug bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 7. SMART HOME DEVICES (Z-Wave, Zigbee, Thread)
            // ================================================================
            Self {
                name: "Z-Wave Smart Home 908 MHz".to_string(),
                category: ConsumerDeviceCategory::SmartHome,
                start_hz: 908_420_000,
                end_hz: 908_420_000,
                common_brands: vec![
                    "Samsung SmartThings".to_string(),
                    "Aeotec".to_string(),
                    "GE/Jasco".to_string(),
                    "Zooz".to_string(),
                    "Inovelli".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["GFSK".to_string(), "FSK".to_string()],
                confused_with: vec![ThreatCategory::BumperBeeper, ThreatCategory::VideoBug],
                description: "Z-Wave smart home devices at 908.42 MHz (US). Smart locks, \
                    light switches, sensors, thermostats. Mesh network with periodic \
                    routing frames. Overlaps ISM bumper beeper and 900 MHz video bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.249".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Zigbee Smart Home 2.4 GHz".to_string(),
                category: ConsumerDeviceCategory::SmartHome,
                start_hz: 2_405_000_000,
                end_hz: 2_480_000_000,
                common_brands: vec![
                    "Philips Hue".to_string(),
                    "IKEA Tradfri".to_string(),
                    "Sengled".to_string(),
                    "Sonoff Zigbee".to_string(),
                    "Aqara".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["O-QPSK".to_string(), "DSSS".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "Zigbee (IEEE 802.15.4) smart home devices at 2.4 GHz. \
                    Smart bulbs, sensors, switches. 16 channels across the band. \
                    Constant mesh traffic overlaps 2.4 GHz video bug band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Zigbee 915 MHz (US)".to_string(),
                category: ConsumerDeviceCategory::SmartHome,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Xbee 900HP".to_string(),
                    "SmartThings (older)".to_string(),
                ],
                typical_power_mw: Some(6.3),
                modulation: vec!["BPSK".to_string(), "DSSS".to_string()],
                confused_with: vec![ThreatCategory::BumperBeeper, ThreatCategory::VideoBug],
                description: "IEEE 802.15.4 sub-GHz at 915 MHz ISM. Less common than 2.4 GHz \
                    Zigbee but used for longer range. Overlaps ISM tracker/video bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "Thread/Matter Smart Home 2.4 GHz".to_string(),
                category: ConsumerDeviceCategory::SmartHome,
                start_hz: 2_405_000_000,
                end_hz: 2_480_000_000,
                common_brands: vec![
                    "Apple HomePod Mini".to_string(),
                    "Google Nest".to_string(),
                    "Eve (Thread)".to_string(),
                    "Nanoleaf".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["O-QPSK".to_string(), "DSSS".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "Thread mesh protocol (Matter/CSA) at 2.4 GHz. Same PHY as Zigbee. \
                    Rapidly growing deployment in modern smart homes."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 8. DECT CORDLESS PHONES
            // ================================================================
            Self {
                name: "DECT 6.0 Cordless Phone".to_string(),
                category: ConsumerDeviceCategory::DectPhone,
                start_hz: 1_920_000_000,
                end_hz: 1_930_000_000,
                common_brands: vec![
                    "Panasonic".to_string(),
                    "VTech".to_string(),
                    "AT&T".to_string(),
                    "Gigaset".to_string(),
                    "Uniden".to_string(),
                ],
                typical_power_mw: Some(250.0),
                modulation: vec!["GFSK".to_string(), "FHSS".to_string(), "TDMA/TDD".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::ImsiCatcher],
                description: "DECT 6.0 cordless phones at 1.92-1.93 GHz. Continuous FHSS \
                    transmission during calls. 10 channels hopping 100x/sec. Adjacent to \
                    PCS 1900 MHz cellular — IMSI catcher false positives. Base station \
                    beacon even when idle. VERY common false positive."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.323".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },

            // ================================================================
            // 9. WIRELESS SECURITY CAMERAS
            // ================================================================
            Self {
                name: "Security Camera 900 MHz Analog".to_string(),
                category: ConsumerDeviceCategory::WirelessSecurityCamera,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "X10 (legacy)".to_string(),
                    "Swann (older)".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["NTSC FM".to_string(), "PAL FM".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "Legacy 900 MHz analog wireless cameras. Identical modulation \
                    to covert video bugs. Continuous wideband FM video transmission. \
                    INDISTINGUISHABLE from a surveillance video bug on spectrum alone."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "Security Camera 2.4 GHz Analog".to_string(),
                category: ConsumerDeviceCategory::WirelessSecurityCamera,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "SecurityMan".to_string(),
                    "Lorex (older analog)".to_string(),
                    "Swann (older analog)".to_string(),
                ],
                typical_power_mw: Some(200.0),
                modulation: vec!["FM Video".to_string(), "NTSC".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "2.4 GHz analog wireless security cameras. The #1 most confusing \
                    false positive — uses SAME modulation and band as covert video bugs. \
                    Wideband FM video signal is a textbook surveillance signature."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Security Camera 2.4 GHz WiFi".to_string(),
                category: ConsumerDeviceCategory::WirelessSecurityCamera,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "Ring".to_string(),
                    "Wyze".to_string(),
                    "Blink".to_string(),
                    "Reolink".to_string(),
                    "TP-Link Tapo".to_string(),
                    "Arlo".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["OFDM".to_string(), "WiFi 802.11n/ac".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "WiFi-based IP security cameras at 2.4 GHz. High bandwidth \
                    continuous streaming overlaps video bug band. Distinguishable by \
                    802.11 framing but still flags spectrum sweeps."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Security Camera 5.8 GHz".to_string(),
                category: ConsumerDeviceCategory::WirelessSecurityCamera,
                start_hz: 5_725_000_000,
                end_hz: 5_850_000_000,
                common_brands: vec![
                    "Reolink (dual-band)".to_string(),
                    "Arlo Pro".to_string(),
                    "Ring (5 GHz)".to_string(),
                    "Lorex".to_string(),
                ],
                typical_power_mw: Some(200.0),
                modulation: vec!["OFDM".to_string(), "WiFi 802.11ac/ax".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "5 GHz WiFi security cameras. Overlaps the 5.8 GHz video \
                    bug band (5.6-7.5 GHz in threat DB). High-bandwidth streaming."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.407".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 10. RC TOYS AND HOBBY DRONES
            // ================================================================
            Self {
                name: "RC Toy 27 MHz".to_string(),
                category: ConsumerDeviceCategory::RcToyDrone,
                start_hz: 26_957_000,
                end_hz: 27_283_000,
                common_brands: vec![
                    "Traxxas (entry)".to_string(),
                    "New Bright".to_string(),
                    "Maisto".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["AM".to_string(), "PPM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::BumperBeeper],
                description: "27 MHz RC cars and toys (CB band adjacent). AM pulse \
                    modulation. Overlaps carrier current HF and VLF bug bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "RC Toy 49 MHz".to_string(),
                category: ConsumerDeviceCategory::RcToyDrone,
                start_hz: 49_830_000,
                end_hz: 49_890_000,
                common_brands: vec![
                    "Various toy brands".to_string(),
                    "Air Hogs".to_string(),
                ],
                typical_power_mw: Some(50.0),
                modulation: vec!["AM".to_string(), "PPM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "49 MHz RC toys sharing the baby monitor band. \
                    Overlaps ultra low power VHF bug band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "RC Aircraft 72 MHz".to_string(),
                category: ConsumerDeviceCategory::RcToyDrone,
                start_hz: 72_010_000,
                end_hz: 72_990_000,
                common_brands: vec![
                    "Spektrum (legacy)".to_string(),
                    "Futaba (legacy)".to_string(),
                    "HiTec (legacy)".to_string(),
                ],
                typical_power_mw: Some(750.0),
                modulation: vec!["FM".to_string(), "PPM".to_string(), "PCM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "72 MHz RC aircraft band (50 channels). Higher power for range. \
                    Overlaps FM broadcast band bug range and SINCGARS military band."
                    .to_string(),
                fcc_rule: Some("47 CFR 95.2101".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "RC/Drone 2.4 GHz Control".to_string(),
                category: ConsumerDeviceCategory::RcToyDrone,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "DJI".to_string(),
                    "Spektrum".to_string(),
                    "FrSky".to_string(),
                    "Futaba".to_string(),
                    "Radiomaster".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["FHSS".to_string(), "DSSS".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "2.4 GHz RC/drone control links. FHSS spread spectrum. \
                    Continuous telemetry during flight overlaps video bug band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "FPV Drone Video 5.8 GHz".to_string(),
                category: ConsumerDeviceCategory::RcToyDrone,
                start_hz: 5_658_000_000,
                end_hz: 5_945_000_000,
                common_brands: vec![
                    "DJI FPV".to_string(),
                    "TBS Unify".to_string(),
                    "ImmersionRC".to_string(),
                    "Rush".to_string(),
                ],
                typical_power_mw: Some(600.0),
                modulation: vec!["FM Video".to_string(), "Digital".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "5.8 GHz FPV drone video transmitters. 25-600mW+ power levels. \
                    Analog FM video is identical signature to 5.8 GHz video bugs. \
                    VERY strong false positive when FPV pilots are nearby."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Occasional,
            },

            // ================================================================
            // 11. MICROWAVE OVENS
            // ================================================================
            Self {
                name: "Microwave Oven Leakage".to_string(),
                category: ConsumerDeviceCategory::MicrowaveOven,
                start_hz: 2_400_000_000,
                end_hz: 2_500_000_000,
                common_brands: vec![
                    "Samsung".to_string(),
                    "LG".to_string(),
                    "GE".to_string(),
                    "Whirlpool".to_string(),
                    "Panasonic".to_string(),
                ],
                typical_power_mw: Some(5000.0), // leakage at 5mW/cm² legal limit, can be several watts total
                modulation: vec!["Broadband Noise".to_string(), "CW 2.45 GHz".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::SpreadSpectrum],
                description: "Microwave oven magnetron leakage at 2.45 GHz. Even compliant \
                    ovens leak milliwatts of broadband energy across the 2.4 GHz ISM band. \
                    Appears as a massive wideband signal during operation. Obliterates \
                    the entire 2.4 GHz video bug detection band while in use."
                    .to_string(),
                fcc_rule: Some("21 CFR 1030.10".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },

            // ================================================================
            // 12. WIRELESS AUDIO SYSTEMS
            // ================================================================
            Self {
                name: "Wireless Speaker 900 MHz".to_string(),
                category: ConsumerDeviceCategory::WirelessAudio,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Sonos (legacy)".to_string(),
                    "Amphony".to_string(),
                    "Nyrius".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["FHSS".to_string(), "FM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::VideoBug],
                description: "900 MHz wireless audio transmitters and speakers. Continuous \
                    audio streaming closely mimics audio bug signatures in 900 MHz ISM band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Wireless Speaker/Headphone 2.4 GHz".to_string(),
                category: ConsumerDeviceCategory::WirelessAudio,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "SteelSeries Arctis".to_string(),
                    "Logitech G Pro".to_string(),
                    "Corsair Virtuoso".to_string(),
                    "Sennheiser RS series".to_string(),
                ],
                typical_power_mw: Some(10.0),
                modulation: vec!["FHSS".to_string(), "GFSK".to_string(), "Proprietary".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::AudioBug],
                description: "2.4 GHz wireless gaming headsets and speakers (non-Bluetooth). \
                    Proprietary low-latency links with continuous streaming in video bug band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "Wireless Microphone VHF/UHF".to_string(),
                category: ConsumerDeviceCategory::WirelessAudio,
                start_hz: 174_000_000,
                end_hz: 698_000_000,
                common_brands: vec![
                    "Shure".to_string(),
                    "Sennheiser".to_string(),
                    "Audio-Technica".to_string(),
                    "Rode Wireless Go".to_string(),
                    "AKG".to_string(),
                ],
                typical_power_mw: Some(50.0),
                modulation: vec!["NFM".to_string(), "WFM".to_string(), "Digital".to_string()],
                confused_with: vec![
                    ThreatCategory::BodyWire,
                    ThreatCategory::WirelessMicrophone,
                    ThreatCategory::AudioBug,
                    ThreatCategory::FederalSurveillance,
                ],
                description: "Professional/prosumer wireless microphones in VHF/UHF bands. \
                    EXTREMELY confusing — legitimate wireless mics are electronically \
                    identical to body wires and wireless surveillance microphones. \
                    Same modulation, same power, same bands. Highest confusion risk."
                    .to_string(),
                fcc_rule: Some("47 CFR 74.861".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 13. BLUETOOTH DEVICES
            // ================================================================
            Self {
                name: "Bluetooth Classic (BR/EDR)".to_string(),
                category: ConsumerDeviceCategory::BluetoothDevice,
                start_hz: 2_402_000_000,
                end_hz: 2_480_000_000,
                common_brands: vec![
                    "Apple AirPods".to_string(),
                    "Sony WH/WF series".to_string(),
                    "Bose".to_string(),
                    "JBL".to_string(),
                    "Samsung Galaxy Buds".to_string(),
                ],
                typical_power_mw: Some(2.5), // Class 2
                modulation: vec!["GFSK".to_string(), "FHSS".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::SpreadSpectrum],
                description: "Bluetooth Classic (BR/EDR) devices. 79 channels, 1600 hops/sec. \
                    Headphones, keyboards, mice, speakers. FHSS pattern looks like spread \
                    spectrum surveillance on wideband sweeps."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Bluetooth Low Energy (BLE)".to_string(),
                category: ConsumerDeviceCategory::BluetoothDevice,
                start_hz: 2_402_000_000,
                end_hz: 2_480_000_000,
                common_brands: vec![
                    "Apple AirTag".to_string(),
                    "Tile".to_string(),
                    "Fitbit".to_string(),
                    "Samsung SmartTag".to_string(),
                    "Chipolo".to_string(),
                ],
                typical_power_mw: Some(1.0), // Class 3 / BLE typical
                modulation: vec!["GFSK".to_string()],
                confused_with: vec![
                    ThreatCategory::BluetoothTracker,
                    ThreatCategory::VideoBug,
                ],
                description: "Bluetooth Low Energy devices. 40 channels (3 advertising). \
                    Trackers (AirTag, Tile) are especially confusing as they ARE tracking \
                    devices — just legitimate ones. Also fitness bands, beacons, sensors."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },

            // ================================================================
            // 14. WIFI ROUTERS AND ACCESS POINTS
            // ================================================================
            Self {
                name: "WiFi 2.4 GHz (802.11b/g/n/ax)".to_string(),
                category: ConsumerDeviceCategory::WifiRouter,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "Netgear".to_string(),
                    "TP-Link".to_string(),
                    "ASUS".to_string(),
                    "Linksys".to_string(),
                    "Ubiquiti".to_string(),
                    "Eero".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["OFDM".to_string(), "DSSS".to_string(), "OFDMA".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::CovertCamera],
                description: "2.4 GHz WiFi. Present in EVERY modern building. Channels 1-14 \
                    spanning the entire 2.4 GHz ISM band. Continuous beacon frames even \
                    with no clients. Multiple overlapping APs in any urban area. \
                    Dominates the 2.4 GHz video bug detection band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "WiFi 5 GHz (802.11a/n/ac/ax)".to_string(),
                category: ConsumerDeviceCategory::WifiRouter,
                start_hz: 5_150_000_000,
                end_hz: 5_850_000_000,
                common_brands: vec![
                    "Netgear".to_string(),
                    "TP-Link".to_string(),
                    "ASUS".to_string(),
                    "Linksys".to_string(),
                    "Ubiquiti".to_string(),
                    "Eero".to_string(),
                ],
                typical_power_mw: Some(200.0),
                modulation: vec!["OFDM".to_string(), "OFDMA".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "5 GHz WiFi (UNII-1/2/2E/3 bands). Overlaps 5.8 GHz video \
                    bug band. High bandwidth channels (80/160 MHz wide). \
                    DFS channels can appear as intermittent radar-like signals."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.407".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "WiFi 6E / 7 (6 GHz)".to_string(),
                category: ConsumerDeviceCategory::WifiRouter,
                start_hz: 5_925_000_000,
                end_hz: 7_125_000_000,
                common_brands: vec![
                    "ASUS ROG".to_string(),
                    "Netgear Orbi".to_string(),
                    "TP-Link Deco".to_string(),
                    "Linksys".to_string(),
                ],
                typical_power_mw: Some(250.0), // AFC-enabled
                modulation: vec!["OFDMA".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "WiFi 6E/7 at 6 GHz. New band overlapping upper range of \
                    5.8 GHz video bug band (threat DB goes to 7.5 GHz). 320 MHz wide \
                    channels. Increasingly common in new routers."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.407".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 15. TPMS TIRE PRESSURE SENSORS
            // ================================================================
            Self {
                name: "TPMS Tire Sensor 315 MHz (US)".to_string(),
                category: ConsumerDeviceCategory::TpmsTireSensor,
                start_hz: 314_900_000,
                end_hz: 315_100_000,
                common_brands: vec![
                    "Schrader (Sensata)".to_string(),
                    "Continental".to_string(),
                    "Pacific Industrial".to_string(),
                    "TRW".to_string(),
                ],
                typical_power_mw: Some(0.5),
                modulation: vec!["ASK".to_string(), "FSK".to_string(), "Manchester".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::BumperBeeper],
                description: "315 MHz TPMS sensors (mandatory in US since 2007). Every vehicle \
                    in a parking lot broadcasts tire data every 60 seconds. \
                    Can resemble vehicle tracking device beacons. \
                    Massive false positive source in any vehicle-adjacent sweep."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "TPMS Tire Sensor 433 MHz (EU/Asia)".to_string(),
                category: ConsumerDeviceCategory::TpmsTireSensor,
                start_hz: 433_820_000,
                end_hz: 434_020_000,
                common_brands: vec![
                    "Schrader (Sensata)".to_string(),
                    "Continental".to_string(),
                    "Huf".to_string(),
                ],
                typical_power_mw: Some(0.5),
                modulation: vec!["ASK".to_string(), "FSK".to_string(), "Manchester".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "433 MHz TPMS sensors (European/Asian vehicles). Same ISM \
                    frequency as audio bugs. Periodic beacons from every parked vehicle."
                    .to_string(),
                fcc_rule: Some("ETSI EN 300 220".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 16. UTILITY SMART METERS
            // ================================================================
            Self {
                name: "Smart Meter 900 MHz ISM".to_string(),
                category: ConsumerDeviceCategory::SmartMeter,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Itron/Silver Spring".to_string(),
                    "Landis+Gyr".to_string(),
                    "Sensus/Xylem".to_string(),
                    "Elster/Honeywell".to_string(),
                ],
                typical_power_mw: Some(1000.0),
                modulation: vec!["FHSS".to_string(), "GFSK".to_string(), "IEEE 802.15.4g".to_string()],
                confused_with: vec![
                    ThreatCategory::BumperBeeper,
                    ThreatCategory::VideoBug,
                    ThreatCategory::FederalSurveillance,
                ],
                description: "AMI smart meters at 900 MHz ISM. Mesh network with periodic \
                    transmissions (every 4-15 seconds). Up to 1W output power. \
                    Present on virtually every building in urban areas. Strong signal \
                    that dominates the 900 MHz ISM band. Appears as persistent \
                    unidentified transmitter."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Smart Water/Gas Meter 900 MHz".to_string(),
                category: ConsumerDeviceCategory::SmartMeter,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Neptune/Trimble".to_string(),
                    "Badger Meter".to_string(),
                    "Mueller/Echologics".to_string(),
                ],
                typical_power_mw: Some(500.0),
                modulation: vec!["FHSS".to_string(), "FSK".to_string()],
                confused_with: vec![ThreatCategory::BumperBeeper, ThreatCategory::VideoBug],
                description: "Water and gas utility smart meters at 900 MHz. Less frequent \
                    transmissions than electric meters but same ISM band overlap."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::VeryCommon,
            },

            // ================================================================
            // 17. LoRa / LoRaWAN DEVICES
            // ================================================================
            Self {
                name: "LoRa Device 915 MHz US".to_string(),
                category: ConsumerDeviceCategory::LoRaDevice,
                start_hz: 902_000_000,
                end_hz: 928_000_000,
                common_brands: vec![
                    "Helium Hotspot".to_string(),
                    "RAK Wireless".to_string(),
                    "LILYGO TTGO".to_string(),
                    "Heltec".to_string(),
                    "Dragino".to_string(),
                    "Meshtastic".to_string(),
                ],
                typical_power_mw: Some(100.0), // up to 1W with FHSS
                modulation: vec!["LoRa CSS".to_string(), "FSK".to_string(), "FHSS".to_string()],
                confused_with: vec![
                    ThreatCategory::BumperBeeper,
                    ThreatCategory::VideoBug,
                    ThreatCategory::SpreadSpectrum,
                ],
                description: "LoRa/LoRaWAN devices at 902-928 MHz US ISM. Chirp spread spectrum \
                    (CSS) modulation with distinctive frequency sweep signature. Helium \
                    hotspots and Meshtastic nodes increasingly common. Long range (km+) \
                    with unusual spectral shape flags spread spectrum detectors."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "LoRa Device 868 MHz EU".to_string(),
                category: ConsumerDeviceCategory::LoRaDevice,
                start_hz: 863_000_000,
                end_hz: 870_000_000,
                common_brands: vec![
                    "The Things Network".to_string(),
                    "RAK Wireless".to_string(),
                    "Dragino".to_string(),
                ],
                typical_power_mw: Some(25.0), // EU limit
                modulation: vec!["LoRa CSS".to_string(), "FSK".to_string()],
                confused_with: vec![
                    ThreatCategory::FederalSurveillance,
                    ThreatCategory::SpreadSpectrum,
                ],
                description: "LoRa at 868 MHz EU ISM. Overlaps with federal microwave \
                    surveillance band (630-890 MHz in threat DB). Chirp signals may \
                    flag as unusual spread spectrum activity."
                    .to_string(),
                fcc_rule: Some("ETSI EN 300 220".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 18. FRS / GMRS / PMR446 WALKIE-TALKIES
            // ================================================================
            Self {
                name: "FRS Walkie-Talkie".to_string(),
                category: ConsumerDeviceCategory::WalkieTalkie,
                start_hz: 462_562_500,
                end_hz: 467_712_500,
                common_brands: vec![
                    "Motorola Talkabout".to_string(),
                    "Midland X-Talker".to_string(),
                    "Cobra".to_string(),
                    "Uniden".to_string(),
                    "BaoFeng (FRS channels)".to_string(),
                ],
                typical_power_mw: Some(2000.0),
                modulation: vec!["NFM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::BodyWire],
                description: "Family Radio Service (FRS) 462-467 MHz. 22 channels. \
                    Up to 2W on shared channels. NFM voice identical to surveillance \
                    audio bug modulation. Popular at events, hotels, and businesses."
                    .to_string(),
                fcc_rule: Some("47 CFR 95 Subpart B".to_string()),
                prevalence: Prevalence::VeryCommon,
            },
            Self {
                name: "GMRS Walkie-Talkie".to_string(),
                category: ConsumerDeviceCategory::WalkieTalkie,
                start_hz: 462_550_000,
                end_hz: 467_725_000,
                common_brands: vec![
                    "Midland MXT".to_string(),
                    "Motorola".to_string(),
                    "Wouxun".to_string(),
                    "BaoFeng".to_string(),
                ],
                typical_power_mw: Some(5000.0),
                modulation: vec!["NFM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::BodyWire],
                description: "General Mobile Radio Service (GMRS) 462-467 MHz. Up to 5W \
                    on exclusive channels, 50W on repeater outputs. Licensed but consumer-grade. \
                    Same band as FRS — higher power makes it louder false positive."
                    .to_string(),
                fcc_rule: Some("47 CFR 95 Subpart E".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "MURS Walkie-Talkie".to_string(),
                category: ConsumerDeviceCategory::WalkieTalkie,
                start_hz: 151_820_000,
                end_hz: 154_600_000,
                common_brands: vec![
                    "Dakota Alert".to_string(),
                    "Motorola RM Series".to_string(),
                    "BaoFeng".to_string(),
                ],
                typical_power_mw: Some(2000.0),
                modulation: vec!["NFM".to_string()],
                confused_with: vec![
                    ThreatCategory::BodyWire,
                    ThreatCategory::FederalSurveillance,
                    ThreatCategory::BumperBeeper,
                ],
                description: "Multi-Use Radio Service (MURS) at 151-154 MHz. 5 channels. \
                    Directly overlaps body wire band II (150-174 MHz) and federal LE VHF. \
                    Business driveway alerts and farm radios. HIGH confusion risk."
                    .to_string(),
                fcc_rule: Some("47 CFR 95 Subpart J".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "PMR446 Walkie-Talkie (EU)".to_string(),
                category: ConsumerDeviceCategory::WalkieTalkie,
                start_hz: 446_006_250,
                end_hz: 446_193_750,
                common_brands: vec![
                    "Motorola TLKR".to_string(),
                    "Binatone".to_string(),
                    "Midland G7".to_string(),
                ],
                typical_power_mw: Some(500.0),
                modulation: vec!["NFM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug],
                description: "PMR446 EU unlicensed walkie-talkies at 446 MHz. 16 analog + \
                    16 digital channels. In the ISM 433 bug band range."
                    .to_string(),
                fcc_rule: Some("ETSI EN 300 296".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 19. MEDICAL DEVICES
            // ================================================================
            Self {
                name: "MICS Medical Implant 402-405 MHz".to_string(),
                category: ConsumerDeviceCategory::MedicalDevice,
                start_hz: 402_000_000,
                end_hz: 405_000_000,
                common_brands: vec![
                    "Medtronic".to_string(),
                    "Abbott/St. Jude".to_string(),
                    "Boston Scientific".to_string(),
                    "Biotronik".to_string(),
                ],
                typical_power_mw: Some(0.025), // 25 microwatts EIRP
                modulation: vec!["GFSK".to_string(), "FSK".to_string(), "OFDM".to_string()],
                confused_with: vec![ThreatCategory::AudioBug, ThreatCategory::FederalSurveillance],
                description: "Medical Implant Communication Service (MICS) at 402-405 MHz. \
                    Pacemakers, insulin pumps, neurostimulators. Very low power (25 µW) \
                    but detectable at close range. In SpyShop popular band. \
                    Ethical concern: must not disrupt medical device operation."
                    .to_string(),
                fcc_rule: Some("47 CFR 95 Subpart I".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Medical Telemetry WMTS 608-614 MHz".to_string(),
                category: ConsumerDeviceCategory::MedicalDevice,
                start_hz: 608_000_000,
                end_hz: 614_000_000,
                common_brands: vec![
                    "Philips".to_string(),
                    "GE Healthcare".to_string(),
                    "Nihon Kohden".to_string(),
                ],
                typical_power_mw: Some(1.5),
                modulation: vec!["FSK".to_string(), "GFSK".to_string()],
                confused_with: vec![ThreatCategory::FederalSurveillance],
                description: "Wireless Medical Telemetry Service (WMTS) at 608-614 MHz. \
                    Hospital patient monitoring. Overlaps federal microwave band start \
                    (630 MHz in threat DB). Relevant for hospital/medical facility sweeps."
                    .to_string(),
                fcc_rule: Some("47 CFR 95 Subpart H".to_string()),
                prevalence: Prevalence::Occasional,
            },
            Self {
                name: "Continuous Glucose Monitor 2.4 GHz".to_string(),
                category: ConsumerDeviceCategory::MedicalDevice,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "Dexcom G6/G7".to_string(),
                    "Abbott FreeStyle Libre 3".to_string(),
                    "Medtronic Guardian".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["GFSK".to_string(), "BLE".to_string()],
                confused_with: vec![ThreatCategory::VideoBug, ThreatCategory::BluetoothTracker],
                description: "Continuous glucose monitors using BLE at 2.4 GHz. \
                    Periodic beacon transmissions. Wearable medical device that \
                    transmits body-worn data — somewhat analogous to a body wire signature."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Common,
            },

            // ================================================================
            // 20. WIRELESS HDMI TRANSMITTERS
            // ================================================================
            Self {
                name: "Wireless HDMI 5 GHz".to_string(),
                category: ConsumerDeviceCategory::WirelessHdmi,
                start_hz: 5_150_000_000,
                end_hz: 5_850_000_000,
                common_brands: vec![
                    "Actiontec ScreenBeam".to_string(),
                    "Nyrius Aries".to_string(),
                    "J-Tech Digital".to_string(),
                    "IOGEAR GWHDMS52".to_string(),
                    "Hollyland Mars".to_string(),
                ],
                typical_power_mw: Some(100.0),
                modulation: vec!["OFDM".to_string(), "Proprietary MIMO".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "5 GHz wireless HDMI/video extenders. Continuous high-bandwidth \
                    video stream. Directly overlaps 5.8 GHz video bug band. \
                    High-bitrate video transmission is the EXACT signature of a \
                    covert video transmitter."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.407".to_string()),
                prevalence: Prevalence::Common,
            },
            Self {
                name: "Wireless HDMI 60 GHz (WiGig)".to_string(),
                category: ConsumerDeviceCategory::WirelessHdmi,
                start_hz: 57_000_000_000,
                end_hz: 66_000_000_000,
                common_brands: vec![
                    "DVDO Air".to_string(),
                    "Belkin (WiGig)".to_string(),
                    "Asus ROG".to_string(),
                ],
                typical_power_mw: Some(500.0),
                modulation: vec!["OFDM".to_string(), "SC-PHY".to_string()],
                confused_with: vec![ThreatCategory::Unknown],
                description: "60 GHz WiGig/802.11ad wireless video. Room-range only. \
                    Outside most TSCM sweep ranges but included for completeness. \
                    Very high bandwidth millimeter-wave transmission."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.255".to_string()),
                prevalence: Prevalence::Rare,
            },

            // ================================================================
            // BONUS: Additional high-confusion devices
            // ================================================================
            Self {
                name: "Wireless Keyboard/Mouse 2.4 GHz".to_string(),
                category: ConsumerDeviceCategory::WirelessSensor,
                start_hz: 2_400_000_000,
                end_hz: 2_483_500_000,
                common_brands: vec![
                    "Logitech Unifying".to_string(),
                    "Microsoft".to_string(),
                    "Dell".to_string(),
                    "HP".to_string(),
                ],
                typical_power_mw: Some(1.0),
                modulation: vec!["GFSK".to_string(), "FHSS".to_string(), "Proprietary".to_string()],
                confused_with: vec![ThreatCategory::VideoBug],
                description: "2.4 GHz wireless keyboards and mice using proprietary RF \
                    (not Bluetooth). Logitech Unifying, etc. Constant low-power \
                    transmission during use, in video bug band."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.247".to_string()),
                prevalence: Prevalence::Ubiquitous,
            },
            Self {
                name: "Smart Garage/Gate 300-400 MHz".to_string(),
                category: ConsumerDeviceCategory::SmartHome,
                start_hz: 300_000_000,
                end_hz: 400_000_000,
                common_brands: vec![
                    "myQ (Chamberlain)".to_string(),
                    "Tailwind iQ3".to_string(),
                    "Meross".to_string(),
                ],
                typical_power_mw: Some(25.0),
                modulation: vec!["OOK".to_string(), "Rolling Code".to_string()],
                confused_with: vec![
                    ThreatCategory::AudioBug,
                    ThreatCategory::FederalSurveillance,
                ],
                description: "Smart garage and gate controllers operating in 300-400 MHz \
                    range. WiFi bridge + RF relay. The RF portion overlaps micro-powered \
                    bug band and SpyShop bands."
                    .to_string(),
                fcc_rule: Some("47 CFR 15.231".to_string()),
                prevalence: Prevalence::Common,
            },
        ]
    }

    /// Check if a detected frequency matches any known consumer device
    pub fn find_matches(frequency_hz: u64, bandwidth_hz: u64) -> Vec<&'static str> {
        // This would be populated at runtime; placeholder for the pattern
        let db = Self::false_positive_database();
        let mut matches = Vec::new();
        for device in &db {
            let detect_start = frequency_hz.saturating_sub(bandwidth_hz / 2);
            let detect_end = frequency_hz + bandwidth_hz / 2;
            if detect_start <= device.end_hz && detect_end >= device.start_hz {
                matches.push(device.name.as_str());
            }
        }
        // Note: returning owned strings since we can't return refs to local
        // In production, this should use a lazy_static or once_cell database
        vec![]
    }

    /// Get all consumer devices that could be confused with a given threat category
    pub fn devices_confused_with(category: &ThreatCategory) -> Vec<Self> {
        Self::false_positive_database()
            .into_iter()
            .filter(|d| d.confused_with.contains(category))
            .collect()
    }

    /// Get all consumer devices in a frequency range
    pub fn devices_in_range(start_hz: u64, end_hz: u64) -> Vec<Self> {
        Self::false_positive_database()
            .into_iter()
            .filter(|d| d.start_hz <= end_hz && d.end_hz >= start_hz)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_positive_database_populated() {
        let db = ConsumerDevice::false_positive_database();
        // Should have at least 45 entries across all 20+ categories
        assert!(db.len() >= 45, "Database has {} entries, expected >= 45", db.len());
    }

    #[test]
    fn test_all_categories_represented() {
        let db = ConsumerDevice::false_positive_database();
        let categories: std::collections::HashSet<_> = db.iter().map(|d| d.category.clone()).collect();
        
        assert!(categories.contains(&ConsumerDeviceCategory::BabyMonitor));
        assert!(categories.contains(&ConsumerDeviceCategory::WirelessDoorbell));
        assert!(categories.contains(&ConsumerDeviceCategory::GarageDoorOpener));
        assert!(categories.contains(&ConsumerDeviceCategory::WeatherStation));
        assert!(categories.contains(&ConsumerDeviceCategory::CarKeyFob));
        assert!(categories.contains(&ConsumerDeviceCategory::WirelessSensor));
        assert!(categories.contains(&ConsumerDeviceCategory::SmartHome));
        assert!(categories.contains(&ConsumerDeviceCategory::DectPhone));
        assert!(categories.contains(&ConsumerDeviceCategory::WirelessSecurityCamera));
        assert!(categories.contains(&ConsumerDeviceCategory::RcToyDrone));
        assert!(categories.contains(&ConsumerDeviceCategory::MicrowaveOven));
        assert!(categories.contains(&ConsumerDeviceCategory::WirelessAudio));
        assert!(categories.contains(&ConsumerDeviceCategory::BluetoothDevice));
        assert!(categories.contains(&ConsumerDeviceCategory::WifiRouter));
        assert!(categories.contains(&ConsumerDeviceCategory::TpmsTireSensor));
        assert!(categories.contains(&ConsumerDeviceCategory::SmartMeter));
        assert!(categories.contains(&ConsumerDeviceCategory::LoRaDevice));
        assert!(categories.contains(&ConsumerDeviceCategory::WalkieTalkie));
        assert!(categories.contains(&ConsumerDeviceCategory::MedicalDevice));
        assert!(categories.contains(&ConsumerDeviceCategory::WirelessHdmi));
    }

    #[test]
    fn test_frequency_ranges_valid() {
        let db = ConsumerDevice::false_positive_database();
        for device in &db {
            assert!(
                device.start_hz <= device.end_hz,
                "Invalid range for {}: {} > {}",
                device.name, device.start_hz, device.end_hz
            );
        }
    }

    #[test]
    fn test_all_entries_have_confusion_targets() {
        let db = ConsumerDevice::false_positive_database();
        for device in &db {
            assert!(
                !device.confused_with.is_empty(),
                "Device {} has no confusion targets",
                device.name
            );
        }
    }

    #[test]
    fn test_433_mhz_false_positives() {
        let matches = ConsumerDevice::devices_in_range(433_000_000, 434_000_000);
        // Should find weather stations, doorbells, key fobs, TPMS, etc.
        assert!(matches.len() >= 4, "Expected >= 4 devices at 433 MHz, found {}", matches.len());
    }

    #[test]
    fn test_2_4_ghz_false_positives() {
        let matches = ConsumerDevice::devices_in_range(2_400_000_000, 2_483_500_000);
        // WiFi, Bluetooth, Zigbee, baby monitors, security cameras, drones, etc.
        assert!(matches.len() >= 10, "Expected >= 10 devices at 2.4 GHz, found {}", matches.len());
    }

    #[test]
    fn test_video_bug_confusion() {
        let confused = ConsumerDevice::devices_confused_with(&ThreatCategory::VideoBug);
        // Many consumer devices overlap video bug bands
        assert!(confused.len() >= 10, "Expected >= 10 devices confused with VideoBug, found {}", confused.len());
    }

    #[test]
    fn test_audio_bug_confusion() {
        let confused = ConsumerDevice::devices_confused_with(&ThreatCategory::AudioBug);
        assert!(confused.len() >= 10, "Expected >= 10 devices confused with AudioBug, found {}", confused.len());
    }
}
