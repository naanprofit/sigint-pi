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
    SecurityCompany,
    MilitaryBase,
    Pmr446,
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
            Self::SecurityCompany => "Security Company",
            Self::MilitaryBase => "Military/Government",
            Self::Pmr446 => "PMR446 (Europe)",
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

fn b(name: &str, vendor: Option<&str>, start: u64, end: u64, sig: CommercialSignalType, modulation: &str, licensed: bool, notes: &str, group: &str) -> CommercialRfBand {
    CommercialRfBand {
        name: name.into(), vendor: vendor.map(|s| s.into()),
        start_hz: start, end_hz: end, signal_type: sig,
        modulation: modulation.into(), license_required: licensed,
        notes: notes.into(), band_group: group.into(),
    }
}

pub fn commercial_rf_database() -> Vec<CommercialRfBand> {
    use CommercialSignalType::*;
    vec![
        // =============================================
        // DRIVE-THRU INTERCOM SYSTEMS (US)
        // =============================================
        b("McDonald's VHF Low (box->inside)", Some("McDonald's"), 30_840_000, 30_840_000, DriveThruIntercom, "FM 2W", false, "Paired w/154.570 or 154.585. Legacy HME system", "Fast Food US"),
        b("McDonald's VHF Low (box->inside)", Some("McDonald's"), 33_140_000, 33_140_000, DriveThruIntercom, "FM 2W", false, "Paired w/151.895. Legacy system", "Fast Food US"),
        b("McDonald's VHF Low (box->inside)", Some("McDonald's"), 33_400_000, 33_400_000, DriveThruIntercom, "FM 1W", false, "Paired w/154.570. Very low power", "Fast Food US"),
        b("McDonald's VHF Low (box->inside)", Some("McDonald's"), 35_020_000, 35_020_000, DriveThruIntercom, "FM 2W", false, "Paired w/154.490 or 154.600. Common legacy freq", "Fast Food US"),
        b("McDonald's VHF High (inside->box)", Some("McDonald's"), 151_895_000, 151_895_000, DriveThruIntercom, "FM", false, "Paired w/33.140", "Fast Food US"),
        b("McDonald's VHF High (inside->box)", Some("McDonald's"), 154_570_000, 154_570_000, DriveThruIntercom, "FM 2W", false, "Paired w/30.840 or 170.245. PL 103.5", "Fast Food US"),
        b("McDonald's VHF High (inside->box)", Some("McDonald's"), 154_600_000, 154_600_000, DriveThruIntercom, "FM 2W", false, "Paired w/35.020 or 171.105", "Fast Food US"),
        b("McDonald's Gov't Band", Some("McDonald's"), 170_245_000, 170_245_000, DriveThruIntercom, "FM", true, "Paired w/154.570", "Fast Food US"),
        b("McDonald's Gov't Band", Some("McDonald's"), 170_305_000, 170_305_000, DriveThruIntercom, "FM", true, "Paired w/31.000", "Fast Food US"),
        b("McDonald's Gov't Band", Some("McDonald's"), 171_105_000, 171_105_000, DriveThruIntercom, "FM", true, "Paired w/154.600", "Fast Food US"),
        b("McDonald's UHF Pair", Some("McDonald's"), 457_550_000, 457_550_000, DriveThruIntercom, "NFM 2W", true, "Customer side. Paired w/467.775", "Fast Food US"),
        b("McDonald's UHF Pair", Some("McDonald's"), 467_775_000, 467_775_000, DriveThruIntercom, "NFM 2W", true, "Attendant side. Paired w/457.550", "Fast Food US"),
        b("McDonald's 900 MHz Digital", Some("McDonald's"), 920_000_000, 921_000_000, DriveThruIntercom, "WBFM Digital", false, "HME NEXEO ISM 900 MHz. Paired w/903-904 MHz", "Fast Food US"),

        b("Burger King UHF", Some("Burger King"), 457_562_500, 457_562_500, DriveThruIntercom, "NFM 2W", true, "Customer window. Paired w/467.7875", "Fast Food US"),
        b("Burger King UHF", Some("Burger King"), 467_787_500, 467_787_500, DriveThruIntercom, "NFM 2W", true, "Order taker. Paired w/457.5625", "Fast Food US"),
        b("Burger King Business", Some("Burger King"), 460_887_500, 460_887_500, DriveThruIntercom, "NFM 2W", true, "Part 90 licensed", "Fast Food US"),

        b("Wendy's / Panasonic", Some("Wendy's"), 906_000_000, 907_000_000, DriveThruIntercom, "FM Analog", false, "Panasonic legacy 906.5 MHz ISM", "Fast Food US"),
        b("Wendy's UHF", Some("Wendy's"), 460_887_500, 460_887_500, DriveThruIntercom, "NFM 2W", true, "Part 90 licensed", "Fast Food US"),

        b("Taco Bell VHF", Some("Taco Bell"), 154_570_000, 154_570_000, DriveThruIntercom, "FM 2W", false, "Paired w/30.840. Shared w/McDonald's freqs", "Fast Food US"),
        b("Taco Bell UHF", Some("Taco Bell"), 460_887_500, 460_887_500, DriveThruIntercom, "NFM 2W", true, "Part 90", "Fast Food US"),

        b("KFC UHF", Some("KFC"), 457_587_500, 457_587_500, DriveThruIntercom, "NFM 2W", true, "Customer window", "Fast Food US"),
        b("KFC UHF", Some("KFC"), 467_812_500, 467_812_500, DriveThruIntercom, "NFM 2W", true, "Order taker. PL 5A", "Fast Food US"),

        b("Hardee's/Carl's Jr VHF", Some("Hardee's"), 154_570_000, 154_570_000, DriveThruIntercom, "FM 2W", false, "Inside->box", "Fast Food US"),
        b("Hardee's/Carl's Jr VHF Low", Some("Hardee's"), 30_840_000, 30_840_000, DriveThruIntercom, "FM 2W", false, "Box->inside", "Fast Food US"),
        b("Hardee's/Carl's Jr UHF", Some("Hardee's"), 457_575_000, 457_575_000, DriveThruIntercom, "NFM 2W", true, "UHF pair", "Fast Food US"),

        b("Chick-fil-A DMR", Some("Chick-fil-A"), 450_000_000, 470_000_000, DmrDigital, "DMR Tier II", true, "Digital voice, licensed UHF. Larger sites", "Fast Food US"),
        b("Subway/Arby's/Popeyes UHF", None, 461_000_000, 462_000_000, DriveThruIntercom, "NFM 2W", true, "Various Part 90 business band freqs", "Fast Food US"),
        b("White Castle UHF", Some("White Castle"), 461_812_500, 461_812_500, DriveThruIntercom, "NFM 2W", true, "Part 90", "Fast Food US"),

        // HME Intercom Systems (vendor-agnostic)
        b("HME Legacy Analog 916 MHz", Some("HME"), 916_000_000, 917_000_000, DriveThruIntercom, "FM Analog", false, "Most pre-2015 US drive-thrus. HME DT800/EOS", "Intercoms"),
        b("HME NEXEO Digital DECT", Some("HME"), 1_920_000_000, 1_930_000_000, DriveThruIntercom, "DECT 6.0", false, "Post-2018 McDonald's/Taco Bell. Needs HackRF", "Intercoms"),
        b("PAX/3M ISM 433 MHz", Some("PAX/3M"), 433_800_000, 434_100_000, DriveThruIntercom, "FM Analog", false, "Older installs, ISM band", "Intercoms"),
        b("TELEX/Bosch FHSS", Some("TELEX/Bosch"), 902_000_000, 928_000_000, DriveThruIntercom, "FHSS", false, "ISM 902-928 MHz. Newer analog/hybrid", "Intercoms"),
        b("Delphi/OES 433/868", Some("Delphi"), 433_920_000, 433_920_000, DriveThruIntercom, "FM", false, "European/some US installs", "Intercoms"),
        b("Delphi/OES 868", Some("Delphi"), 868_000_000, 868_600_000, DriveThruIntercom, "FM", false, "EU ISM 868 MHz band", "Intercoms"),

        // =============================================
        // FAST FOOD - WORLDWIDE
        // =============================================
        // Europe - PMR446 (license-free, used by staff radios)
        b("PMR446 Ch 1", None, 446_006_250, 446_006_250, Pmr446, "NFM", false, "EU license-free. McDonald's/KFC/Burger King staff", "Fast Food EU"),
        b("PMR446 Ch 2", None, 446_018_750, 446_018_750, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 3", None, 446_031_250, 446_031_250, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 4", None, 446_043_750, 446_043_750, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 5", None, 446_056_250, 446_056_250, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 6", None, 446_068_750, 446_068_750, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 7", None, 446_081_250, 446_081_250, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        b("PMR446 Ch 8", None, 446_093_750, 446_093_750, Pmr446, "NFM", false, "EU license-free staff radio", "Fast Food EU"),
        // EU DECT (drive-thru headsets)
        b("EU DECT Drive-Thru", None, 1_880_000_000, 1_900_000_000, DriveThruIntercom, "DECT", false, "EU DECT 1880-1900 MHz. HME/3M headsets", "Fast Food EU"),
        // EU ISM 868 MHz
        b("EU ISM 868 MHz Intercom", None, 868_000_000, 868_600_000, DriveThruIntercom, "FM/FSK", false, "EU ISM short-range devices. Pagers/intercoms", "Fast Food EU"),
        // UK specific
        b("UK Simple Site Licence", None, 449_000_000, 450_000_000, BusinessUhf, "NFM", true, "Ofcom Simple UK Licence. Fast food chains", "Fast Food UK"),
        b("UK Business Radio 453-454", None, 453_000_000, 454_000_000, BusinessUhf, "NFM", true, "UK Ofcom licensed. McDonald's UK/Costa/Greggs", "Fast Food UK"),
        // Japan
        b("Japan Specified Low Power (tokutei)", None, 421_000_000, 422_000_000, DriveThruIntercom, "NFM", false, "Japan tokutei low power radio 421-422 MHz", "Fast Food JP"),
        b("Japan 900 MHz ISM", None, 916_000_000, 928_000_000, DriveThruIntercom, "FHSS", false, "Japan ARIB STD-T108. Drive-thru/staff headsets", "Fast Food JP"),
        // Australia/NZ
        b("AU UHF CB Ch 1-40", None, 476_425_000, 477_412_500, DriveThruIntercom, "NFM", false, "Australian UHF CB (license-free). Some fast food", "Fast Food AU"),
        b("AU ISM 915 MHz", None, 915_000_000, 928_000_000, DriveThruIntercom, "FHSS", false, "AU ISM 915-928. HME/3M intercom systems", "Fast Food AU"),

        // =============================================
        // RESTAURANT PAGING / GUEST BUZZERS
        // =============================================
        b("LRS POCSAG Paging", Some("Long Range Systems"), 467_750_000, 467_912_500, PagerSystem, "POCSAG/FSK", false, "Most common US restaurant paging. multimon-ng decodable", "Pagers"),
        b("Generic Coaster Pagers 418", None, 417_500_000, 418_500_000, GuestBuzzer, "OOK", false, "Cheap coaster pager systems", "Pagers"),
        b("Jtech/KALLPOD 433", Some("Jtech"), 433_800_000, 434_100_000, GuestBuzzer, "OOK/FSK", false, "Table buzzers, ISM 433.92 MHz", "Pagers"),
        b("Revel/JTECH Digital 900", Some("Revel"), 902_000_000, 928_000_000, PagerSystem, "FHSS", false, "Digital guest management, ISM 902-928 MHz", "Pagers"),

        // =============================================
        // FRS CHANNELS (462/467 MHz, no license <=2W)
        // =============================================
        b("FRS Ch 1 / GMRS", None, 462_562_500, 462_562_500, Frs, "NFM", false, "462.5625 MHz. Shared FRS/GMRS", "FRS/MURS"),
        b("FRS Ch 2 / GMRS", None, 462_587_500, 462_587_500, Frs, "NFM", false, "462.5875 MHz", "FRS/MURS"),
        b("FRS Ch 3 / GMRS", None, 462_612_500, 462_612_500, Frs, "NFM", false, "462.6125 MHz", "FRS/MURS"),
        b("FRS Ch 4 / GMRS", None, 462_637_500, 462_637_500, Frs, "NFM", false, "462.6375 MHz", "FRS/MURS"),
        b("FRS Ch 5 / GMRS", None, 462_662_500, 462_662_500, Frs, "NFM", false, "462.6625 MHz", "FRS/MURS"),
        b("FRS Ch 6 / GMRS", None, 462_687_500, 462_687_500, Frs, "NFM", false, "462.6875 MHz", "FRS/MURS"),
        b("FRS Ch 7 / GMRS", None, 462_712_500, 462_712_500, Frs, "NFM", false, "462.7125 MHz", "FRS/MURS"),
        b("FRS Ch 8", None, 467_562_500, 467_562_500, Frs, "NFM", false, "467.5625 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 9", None, 467_587_500, 467_587_500, Frs, "NFM", false, "467.5875 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 10", None, 467_612_500, 467_612_500, Frs, "NFM", false, "467.6125 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 11", None, 467_637_500, 467_637_500, Frs, "NFM", false, "467.6375 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 12", None, 467_662_500, 467_662_500, Frs, "NFM", false, "467.6625 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 13", None, 467_687_500, 467_687_500, Frs, "NFM", false, "467.6875 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 14", None, 467_712_500, 467_712_500, Frs, "NFM", false, "467.7125 MHz. FRS only (0.5W)", "FRS/MURS"),
        b("FRS Ch 15 / GMRS", None, 462_550_000, 462_550_000, Frs, "NFM", false, "462.5500 MHz", "FRS/MURS"),
        b("FRS Ch 16 / GMRS", None, 462_575_000, 462_575_000, Frs, "NFM", false, "462.5750 MHz (White Dot)", "FRS/MURS"),
        b("FRS Ch 17 / GMRS", None, 462_600_000, 462_600_000, Frs, "NFM", false, "462.6000 MHz", "FRS/MURS"),
        b("FRS Ch 18 / GMRS", None, 462_625_000, 462_625_000, Frs, "NFM", false, "462.6250 MHz (Black Dot)", "FRS/MURS"),
        b("FRS Ch 19 / GMRS", None, 462_650_000, 462_650_000, Frs, "NFM", false, "462.6500 MHz", "FRS/MURS"),
        b("FRS Ch 20 / GMRS", None, 462_675_000, 462_675_000, Frs, "NFM", false, "462.6750 MHz (Orange Dot)", "FRS/MURS"),
        b("FRS Ch 21 / GMRS", None, 462_700_000, 462_700_000, Frs, "NFM", false, "462.7000 MHz", "FRS/MURS"),
        b("FRS Ch 22 / GMRS", None, 462_725_000, 462_725_000, Frs, "NFM", false, "462.7250 MHz", "FRS/MURS"),

        // =============================================
        // MURS CHANNELS (151/154 MHz, no license <=2W)
        // =============================================
        b("MURS Ch 1", None, 151_820_000, 151_820_000, Murs, "NFM 11.25kHz", false, "151.820 MHz", "FRS/MURS"),
        b("MURS Ch 2", None, 151_880_000, 151_880_000, Murs, "NFM 11.25kHz", false, "151.880 MHz", "FRS/MURS"),
        b("MURS Ch 3", None, 151_940_000, 151_940_000, Murs, "NFM 11.25kHz", false, "151.940 MHz. Syscall/Table Talk paging", "FRS/MURS"),
        b("MURS Ch 4 (Blue Dot)", None, 154_570_000, 154_570_000, Murs, "NFM 20kHz", false, "154.570 MHz. Also used by drive-thrus!", "FRS/MURS"),
        b("MURS Ch 5 (Green Dot)", None, 154_600_000, 154_600_000, Murs, "NFM 20kHz", false, "154.600 MHz. Also used by drive-thrus!", "FRS/MURS"),

        // =============================================
        // GMRS (licensed)
        // =============================================
        b("GMRS Repeater Input", None, 467_550_000, 467_725_000, Gmrs, "NFM", true, "FCC license required ($35/10yr). Repeater inputs", "FRS/MURS"),

        // =============================================
        // SECURITY COMPANY FREQUENCIES
        // =============================================
        // Wackenhut / G4S (merged 2002, now Allied Universal)
        b("Wackenhut/G4S UHF Business", Some("G4S/Allied Universal"), 461_000_000, 462_000_000, SecurityCompany, "NFM/DMR", true, "Part 90 licensed. Nuclear facilities, govt buildings. Now Allied Universal", "Security"),
        b("Wackenhut/G4S Star Freqs", Some("G4S/Allied Universal"), 467_850_000, 467_850_000, SecurityCompany, "NFM 2W", true, "Silver Star itinerant. Common for security guards", "Security"),
        b("Wackenhut/G4S Star Freqs", Some("G4S/Allied Universal"), 467_875_000, 467_875_000, SecurityCompany, "NFM 2W", true, "Gold Star itinerant. Retail security nationwide", "Security"),
        b("Wackenhut/G4S Star Freqs", Some("G4S/Allied Universal"), 467_900_000, 467_900_000, SecurityCompany, "NFM 2W", true, "Red Star itinerant. Guard-to-dispatch", "Security"),
        b("Wackenhut/G4S Star Freqs", Some("G4S/Allied Universal"), 467_925_000, 467_925_000, SecurityCompany, "NFM 2W", true, "Blue Star itinerant. Patrol check-in", "Security"),

        // Securitas
        b("Securitas UHF Business", Some("Securitas"), 461_000_000, 462_000_000, SecurityCompany, "NFM/DMR", true, "Part 90 licensed. Office buildings, malls, events", "Security"),
        b("Securitas VHF Itinerant", Some("Securitas"), 151_625_000, 151_625_000, SecurityCompany, "NFM", true, "Red Dot VHF itinerant. Guard radios", "Security"),
        b("Securitas VHF Itinerant", Some("Securitas"), 151_955_000, 151_955_000, SecurityCompany, "NFM", true, "Purple Dot VHF itinerant", "Security"),

        // Allied Universal (merged G4S + Wackenhut)
        b("Allied Universal DMR", Some("Allied Universal"), 450_000_000, 470_000_000, SecurityCompany, "DMR Tier II/III", true, "Digital trunked. Large corporate campuses", "Security"),

        // General Security Guard Frequencies (itinerant/business band)
        b("Brown Dot (Security Common)", None, 464_500_000, 464_500_000, SecurityCompany, "NFM", true, "Brown Dot itinerant. Very common for guard companies", "Security"),
        b("Yellow Dot (Security Common)", None, 464_550_000, 464_550_000, SecurityCompany, "NFM", true, "Yellow Dot itinerant. Mall security, events", "Security"),
        b("Silver Star (Security Nationwide)", None, 467_850_000, 467_850_000, SecurityCompany, "NFM 2W", true, "National retail chains. Walmart, Target security", "Security"),
        b("Gold Star (Security Nationwide)", None, 467_875_000, 467_875_000, SecurityCompany, "NFM 2W", true, "National chains security dispatch", "Security"),
        b("Red Star (Security Nationwide)", None, 467_900_000, 467_900_000, SecurityCompany, "NFM 2W", true, "National chains guard-to-guard", "Security"),
        b("Blue Star (Security Nationwide)", None, 467_925_000, 467_925_000, SecurityCompany, "NFM 2W", true, "National chains patrol", "Security"),
        b("J Dot (Business Low Power)", None, 467_762_500, 467_762_500, SecurityCompany, "NFM 2W", true, "Motorola preset ch 3. Security/retail", "Security"),
        b("K Dot (Business Low Power)", None, 467_812_500, 467_812_500, SecurityCompany, "NFM 2W", true, "Motorola preset ch 4. Security/retail", "Security"),

        // =============================================
        // AREA 51 / NTTR / GROOM LAKE
        // =============================================
        b("Area 51 Ops (Coffee Tree)", None, 123_225_000, 123_225_000, MilitaryBase, "AM", true, "Local Airspace Controller. Controls WTR. 'Rainbow' on UHF", "Area 51"),
        b("Area 51 Ops (Rainbow UHF)", None, 265_500_000, 265_500_000, MilitaryBase, "AM", true, "Local Airspace Controller UHF. Controls WTR", "Area 51"),
        b("Area 51 WTR Mission Ctrl", None, 254_900_000, 254_900_000, MilitaryBase, "AM", true, "WTR Mission Controller ('Unclog')", "Area 51"),
        b("Dreamland MOA Primary", None, 126_150_000, 126_150_000, MilitaryBase, "AM", true, "R-4808 MOA Control/Approach. Paired w/261.100 UHF", "Area 51"),
        b("Dreamland MOA UHF Primary", None, 261_100_000, 261_100_000, MilitaryBase, "AM", true, "R-4808 MOA Control/Approach UHF", "Area 51"),
        b("Groom Lake VOR/Weather", None, 117_500_000, 117_500_000, MilitaryBase, "AM", true, "Ident 'MCY'. Weather info", "Area 51"),
        b("Groom Mission Prime UHF", None, 255_800_000, 255_800_000, MilitaryBase, "AM", true, "Dreamland Mission Prime", "Area 51"),
        b("Groom Mission (Groundhog)", None, 139_425_000, 139_425_000, MilitaryBase, "AM", true, "Groom Mission VHF", "Area 51"),
        b("Groom Mission (Gearbox)", None, 299_150_000, 299_150_000, MilitaryBase, "AM", true, "R-4806W Groom Mission prime", "Area 51"),
        b("NTTR Security", None, 167_825_000, 167_825_000, MilitaryBase, "FM", true, "NTTR/NNSS perimeter security", "Area 51"),
        b("Cammo Dudes / Back Gate", None, 150_200_000, 150_200_000, MilitaryBase, "FM", true, "Back Gate radar. Perimeter patrol", "Area 51"),
        b("Cammo Dudes / Back Gate", None, 155_650_000, 155_650_000, MilitaryBase, "FM", true, "Back Gate radar secondary", "Area 51"),
        b("Cammo Dudes / Back Gate", None, 165_900_000, 165_900_000, MilitaryBase, "FM", true, "Back Gate radar", "Area 51"),
        b("NTS Fire/EMS", None, 167_925_000, 167_925_000, MilitaryBase, "FM", true, "NNSS Fire/EMS/Radiation Safety", "Area 51"),
        b("Nellis Range Blackjack VHF", None, 123_550_000, 123_550_000, MilitaryBase, "AM", true, "Range Controller 'Blackjack'. Paired w/377.800 UHF", "Area 51"),
        b("Nellis Range Blackjack", None, 148_500_000, 148_500_000, MilitaryBase, "FM", true, "Freq 'Fox 4' repeater output. Contractors", "Area 51"),
        b("Nellis Security Police", None, 163_375_000, 163_512_500, MilitaryBase, "FM", true, "Nellis AFB Security Forces 'Raymond 22'", "Area 51"),
        b("TTR Base Ops", None, 119_450_000, 119_450_000, MilitaryBase, "AM", true, "Tonopah Test Range 'Ridge Line'. Paired w/233.950", "Area 51"),
        b("AWACS Darkstar", None, 391_800_000, 391_800_000, MilitaryBase, "AM", true, "Groom Lake AWACS 965th AACS", "Area 51"),
        b("Skunkworks Primary", None, 252_400_000, 252_400_000, MilitaryBase, "AM", true, "Lockheed Skunkworks Main", "Area 51"),
        b("Skunkworks Nationwide Test", None, 275_200_000, 275_200_000, MilitaryBase, "AM", true, "Lockheed nationwide test frequency", "Area 51"),
        b("Skunkworks Secondary", None, 349_300_000, 349_300_000, MilitaryBase, "AM", true, "Lockheed Skunkworks", "Area 51"),
        b("Old Area 51 Road Sensors", None, 496_250_000, 497_750_000, MilitaryBase, "Data", true, "Legacy perimeter intrusion sensors", "Area 51"),
        b("DOE Rescue/Medics", None, 173_687_500, 173_687_500, MilitaryBase, "FM", true, "DOE/DOD Rescue 'Rescue 6'", "Area 51"),

        // =============================================
        // BUSINESS UHF / DIGITAL
        // =============================================
        b("Business UHF (Part 90)", None, 450_000_000, 470_000_000, BusinessUhf, "NFM/DMR/P25", true, "Corporate chains, licensed per site", "Digital"),
        b("P25 Digital Voice", None, 450_000_000, 512_000_000, P25Digital, "C4FM/CQPSK", true, "Corporate security. Metadata only decode", "Digital"),

        // =============================================
        // COLOR DOT BUSINESS BAND (Motorola presets)
        // =============================================
        b("Red Dot (Motorola Ch 1)", None, 151_625_000, 151_625_000, BusinessUhf, "NFM", true, "VHF itinerant. Retail/security/events", "Business Band"),
        b("Purple Dot", None, 151_955_000, 151_955_000, BusinessUhf, "NFM", true, "VHF itinerant", "Business Band"),
        b("Pink Dot", None, 42_980_000, 42_980_000, BusinessUhf, "FM 2W", true, "Low band itinerant", "Business Band"),
        b("Motorola CLP Ch 1 (Brown Dot)", None, 464_500_000, 464_500_000, BusinessUhf, "NFM", true, "UHF itinerant. Very common", "Business Band"),
        b("Motorola CLP Ch 2 (Yellow Dot)", None, 464_550_000, 464_550_000, BusinessUhf, "NFM", true, "UHF itinerant", "Business Band"),
    ]
}

pub fn classify_signal(freq_hz: u64, power_db: f64) -> Option<CommercialRfSignal> {
    let db = commercial_rf_database();
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
                    CommercialSignalType::Murs |
                    CommercialSignalType::Pmr446
                ),
                band_group: band.band_group.clone(),
                notes: band.notes.clone(),
            });
        }
    }
    None
}
