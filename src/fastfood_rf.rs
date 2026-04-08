use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialRfSignal {
    pub frequency_hz: u64,
    pub frequency_mhz: f64,
    pub signal_type: CommercialSignalType,
    pub system_vendor: Option<String>,
    pub system_name: String,
    pub power_dbm: f32,
    pub modulation: String,
    pub license_required: bool,
    pub decode_available: bool,
    pub band_group: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommercialSignalType {
    DriveThruIntercom,
    PagerSystem,
    Frs,
    Murs,
    Gmrs,
    DmrDigital,
    P25Digital,
    BusinessUhf,
    GuestBuzzer,
    Unknown,
}

impl CommercialSignalType {
    pub fn label(&self) -> &str {
        match self {
            Self::DriveThruIntercom => "Drive-Thru Intercom",
            Self::PagerSystem => "Pager System",
            Self::Frs => "FRS Radio",
            Self::Murs => "MURS Radio",
            Self::Gmrs => "GMRS Radio",
            Self::DmrDigital => "DMR Digital Voice",
            Self::P25Digital => "P25 Digital Voice",
            Self::BusinessUhf => "Business UHF (Part 90)",
            Self::GuestBuzzer => "Guest Buzzer/Coaster",
            Self::Unknown => "Unknown Commercial",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommercialRfBand {
    pub name: String,
    pub vendor: Option<String>,
    pub start_hz: u64,
    pub end_hz: u64,
    pub signal_type: CommercialSignalType,
    pub modulation: String,
    pub license_required: bool,
    pub notes: String,
    pub band_group: String,
}

pub fn commercial_rf_database() -> Vec<CommercialRfBand> {
    vec![
        // === Drive-Thru Intercom Systems ===
        CommercialRfBand {
            name: "PAX/3M Legacy Intercom".into(),
            vendor: Some("PAX/3M".into()),
            start_hz: 433_800_000, end_hz: 434_100_000,
            signal_type: CommercialSignalType::DriveThruIntercom,
            modulation: "FM Analog".into(),
            license_required: false,
            notes: "Older installs, ISM band, still common at independent restaurants".into(),
            band_group: "Intercoms".into(),
        },
        CommercialRfBand {
            name: "Panasonic Drive-Thru".into(),
            vendor: Some("Panasonic".into()),
            start_hz: 906_000_000, end_hz: 907_000_000,
            signal_type: CommercialSignalType::DriveThruIntercom,
            modulation: "FM Analog".into(),
            license_required: false,
            notes: "Wendy's, Burger King legacy systems".into(),
            band_group: "Intercoms".into(),
        },
        CommercialRfBand {
            name: "HME Legacy Analog Intercom".into(),
            vendor: Some("HME".into()),
            start_hz: 916_000_000, end_hz: 917_000_000,
            signal_type: CommercialSignalType::DriveThruIntercom,
            modulation: "FM Analog".into(),
            license_required: false,
            notes: "Most pre-2015 US drive-thrus. HME DT800/EOS series".into(),
            band_group: "Intercoms".into(),
        },
        CommercialRfBand {
            name: "TELEX/Bosch FHSS Intercom".into(),
            vendor: Some("TELEX/Bosch".into()),
            start_hz: 902_000_000, end_hz: 928_000_000,
            signal_type: CommercialSignalType::DriveThruIntercom,
            modulation: "FHSS".into(),
            license_required: false,
            notes: "Newer analog/hybrid systems, ISM 902-928 MHz".into(),
            band_group: "Intercoms".into(),
        },
        CommercialRfBand {
            name: "HME NEXEO Digital (DECT)".into(),
            vendor: Some("HME".into()),
            start_hz: 1_920_000_000, end_hz: 1_930_000_000,
            signal_type: CommercialSignalType::DriveThruIntercom,
            modulation: "DECT 6.0".into(),
            license_required: false,
            notes: "McDonald's, Taco Bell primary since ~2018. Requires HackRF (>1766 MHz)".into(),
            band_group: "Intercoms".into(),
        },

        // === Restaurant Paging / Guest Buzzer Systems ===
        CommercialRfBand {
            name: "Generic Coaster Pagers".into(),
            vendor: None,
            start_hz: 417_500_000, end_hz: 418_500_000,
            signal_type: CommercialSignalType::GuestBuzzer,
            modulation: "OOK".into(),
            license_required: false,
            notes: "Cheap coaster pager systems, 418 MHz OOK".into(),
            band_group: "Pagers".into(),
        },
        CommercialRfBand {
            name: "Jtech/KALLPOD Table Buzzers".into(),
            vendor: Some("Jtech".into()),
            start_hz: 433_800_000, end_hz: 434_100_000,
            signal_type: CommercialSignalType::GuestBuzzer,
            modulation: "OOK/FSK".into(),
            license_required: false,
            notes: "Table buzzers, ISM 433.92 MHz".into(),
            band_group: "Pagers".into(),
        },
        CommercialRfBand {
            name: "LRS POCSAG Paging".into(),
            vendor: Some("Long Range Systems".into()),
            start_hz: 467_750_000, end_hz: 467_912_500,
            signal_type: CommercialSignalType::PagerSystem,
            modulation: "POCSAG/FSK".into(),
            license_required: false,
            notes: "Most common US restaurant paging. Decodable via multimon-ng".into(),
            band_group: "Pagers".into(),
        },
        CommercialRfBand {
            name: "Revel/JTECH Digital Paging".into(),
            vendor: Some("Revel/JTECH".into()),
            start_hz: 902_000_000, end_hz: 928_000_000,
            signal_type: CommercialSignalType::PagerSystem,
            modulation: "FHSS".into(),
            license_required: false,
            notes: "Digital guest management, ISM 902-928 MHz".into(),
            band_group: "Pagers".into(),
        },

        // === FRS Channels (462/467 MHz, no license ≤2W) ===
        CommercialRfBand {
            name: "FRS Channel 1".into(), vendor: None,
            start_hz: 462_562_500, end_hz: 462_562_500,
            signal_type: CommercialSignalType::Frs,
            modulation: "NFM".into(), license_required: false,
            notes: "462.5625 MHz — shared FRS/GMRS".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "FRS Channel 2".into(), vendor: None,
            start_hz: 462_587_500, end_hz: 462_587_500,
            signal_type: CommercialSignalType::Frs,
            modulation: "NFM".into(), license_required: false,
            notes: "462.5875 MHz — shared FRS/GMRS".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "FRS Channel 3".into(), vendor: None,
            start_hz: 462_612_500, end_hz: 462_612_500,
            signal_type: CommercialSignalType::Frs,
            modulation: "NFM".into(), license_required: false,
            notes: "462.6125 MHz — shared FRS/GMRS".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "FRS Channels 4-8".into(), vendor: None,
            start_hz: 462_637_500, end_hz: 462_712_500,
            signal_type: CommercialSignalType::Frs,
            modulation: "NFM".into(), license_required: false,
            notes: "462.6375-462.7125 MHz — shared FRS/GMRS ch 4-8".into(), band_group: "FRS/MURS".into(),
        },

        // === MURS Channels (151/154 MHz, no license ≤2W) ===
        CommercialRfBand {
            name: "MURS Channel 1".into(), vendor: None,
            start_hz: 151_820_000, end_hz: 151_820_000,
            signal_type: CommercialSignalType::Murs,
            modulation: "NFM".into(), license_required: false,
            notes: "151.820 MHz — 11.25 kHz bandwidth".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "MURS Channel 2".into(), vendor: None,
            start_hz: 151_880_000, end_hz: 151_880_000,
            signal_type: CommercialSignalType::Murs,
            modulation: "NFM".into(), license_required: false,
            notes: "151.880 MHz".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "MURS Channel 3".into(), vendor: None,
            start_hz: 151_940_000, end_hz: 151_940_000,
            signal_type: CommercialSignalType::Murs,
            modulation: "NFM".into(), license_required: false,
            notes: "151.940 MHz — Syscall/Table Talk paging".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "MURS Channel 4".into(), vendor: None,
            start_hz: 154_570_000, end_hz: 154_570_000,
            signal_type: CommercialSignalType::Murs,
            modulation: "NFM".into(), license_required: false,
            notes: "154.570 MHz — 20 kHz bandwidth".into(), band_group: "FRS/MURS".into(),
        },
        CommercialRfBand {
            name: "MURS Channel 5".into(), vendor: None,
            start_hz: 154_600_000, end_hz: 154_600_000,
            signal_type: CommercialSignalType::Murs,
            modulation: "NFM".into(), license_required: false,
            notes: "154.600 MHz — 20 kHz bandwidth".into(), band_group: "FRS/MURS".into(),
        },

        // === GMRS (licensed) ===
        CommercialRfBand {
            name: "GMRS Channels (licensed)".into(), vendor: None,
            start_hz: 462_550_000, end_hz: 462_725_000,
            signal_type: CommercialSignalType::Gmrs,
            modulation: "NFM".into(), license_required: true,
            notes: "462.550-462.725 MHz + repeater pairs. FCC license required ($35/10yr)".into(),
            band_group: "FRS/MURS".into(),
        },

        // === Business UHF / Digital ===
        CommercialRfBand {
            name: "Business UHF (Part 90)".into(), vendor: None,
            start_hz: 450_000_000, end_hz: 470_000_000,
            signal_type: CommercialSignalType::BusinessUhf,
            modulation: "NFM/DMR/P25".into(), license_required: true,
            notes: "Corporate chains, licensed per site. Chick-fil-A uses DMR Tier II".into(),
            band_group: "Digital".into(),
        },
        CommercialRfBand {
            name: "P25 Digital Voice".into(), vendor: None,
            start_hz: 450_000_000, end_hz: 512_000_000,
            signal_type: CommercialSignalType::P25Digital,
            modulation: "C4FM/CQPSK".into(), license_required: true,
            notes: "Some corporate security teams. Metadata only decode.".into(),
            band_group: "Digital".into(),
        },
    ]
}

pub fn classify_signal(freq_hz: u64, power_db: f64) -> Option<CommercialRfSignal> {
    let db = commercial_rf_database();
    // Find matching band (with 50 kHz tolerance for single-frequency entries)
    for band in &db {
        let tolerance = if band.start_hz == band.end_hz { 50_000 } else { 0 };
        if freq_hz >= band.start_hz.saturating_sub(tolerance)
            && freq_hz <= band.end_hz.saturating_add(tolerance)
        {
            return Some(CommercialRfSignal {
                frequency_hz: freq_hz,
                frequency_mhz: freq_hz as f64 / 1_000_000.0,
                signal_type: band.signal_type.clone(),
                system_vendor: band.vendor.clone(),
                system_name: band.name.clone(),
                power_dbm: power_db as f32,
                modulation: band.modulation.clone(),
                license_required: band.license_required,
                decode_available: matches!(band.signal_type,
                    CommercialSignalType::PagerSystem |
                    CommercialSignalType::Frs |
                    CommercialSignalType::Murs
                ),
                band_group: band.band_group.clone(),
                notes: band.notes.clone(),
            });
        }
    }
    None
}
