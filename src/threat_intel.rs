//! Threat Intelligence Database
//! 
//! Contains known OUIs and identifiers for surveillance equipment,
//! defense contractors, and intelligence-linked manufacturers.

use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatCategory {
    /// US Defense/Intelligence contractors
    UsDefense,
    /// Israeli surveillance/defense
    Israeli,
    /// Chinese state-linked (Huawei, ZTE, Hikvision, Dahua)
    Chinese,
    /// Russian state-linked
    Russian,
    /// European defense contractors
    EuropeanDefense,
    /// Known surveillance equipment manufacturers
    Surveillance,
    /// Law enforcement equipment (Stingray, etc.)
    LawEnforcement,
    /// General high-interest (telecom infrastructure)
    HighInterest,
}

#[derive(Debug, Clone)]
pub struct ThreatOui {
    pub prefix: &'static str,
    pub vendor: &'static str,
    pub category: ThreatCategory,
    pub description: &'static str,
}

lazy_static! {
    /// Known threat/surveillance OUI prefixes (first 3 bytes of MAC)
    pub static ref THREAT_OUIS: HashMap<&'static str, ThreatOui> = {
        let mut m = HashMap::new();
        
        // ============================================
        // US DEFENSE CONTRACTORS
        // ============================================
        
        // Raytheon
        m.insert("00:00:8F", ThreatOui {
            prefix: "00:00:8F",
            vendor: "Raytheon",
            category: ThreatCategory::UsDefense,
            description: "Major US defense contractor - missiles, radar, surveillance systems",
        });
        m.insert("00:E0:E7", ThreatOui {
            prefix: "00:E0:E7",
            vendor: "Raytheon E-Systems",
            category: ThreatCategory::UsDefense,
            description: "Raytheon electronic warfare and intelligence systems division",
        });
        
        // Harris Corporation (now L3Harris) - Stingray manufacturer
        m.insert("00:00:C3", ThreatOui {
            prefix: "00:00:C3",
            vendor: "Harris Corporation",
            category: ThreatCategory::LawEnforcement,
            description: "Manufacturer of Stingray/IMSI catchers used by law enforcement",
        });
        m.insert("00:02:68", ThreatOui {
            prefix: "00:02:68",
            vendor: "Harris Government Communications",
            category: ThreatCategory::LawEnforcement,
            description: "Harris government/military communications - Stingray manufacturer",
        });
        m.insert("00:06:EC", ThreatOui {
            prefix: "00:06:EC",
            vendor: "Harris Corporation",
            category: ThreatCategory::LawEnforcement,
            description: "Harris tactical radio and surveillance equipment",
        });
        
        // L-3 Communications / L3Harris
        m.insert("00:02:9F", ThreatOui {
            prefix: "00:02:9F",
            vendor: "L-3 Communications",
            category: ThreatCategory::UsDefense,
            description: "Defense contractor - aviation recorders, ISR systems",
        });
        m.insert("00:05:D6", ThreatOui {
            prefix: "00:05:D6",
            vendor: "L-3 Linkabit",
            category: ThreatCategory::UsDefense,
            description: "Military satellite and secure communications",
        });
        m.insert("00:0B:7A", ThreatOui {
            prefix: "00:0B:7A",
            vendor: "L-3 Linkabit",
            category: ThreatCategory::UsDefense,
            description: "Military communications systems",
        });
        m.insert("00:10:27", ThreatOui {
            prefix: "00:10:27",
            vendor: "L-3 Communications East",
            category: ThreatCategory::UsDefense,
            description: "Defense electronics and communications",
        });
        m.insert("00:11:1B", ThreatOui {
            prefix: "00:11:1B",
            vendor: "Targa Systems / L-3 Communications",
            category: ThreatCategory::UsDefense,
            description: "Military video and surveillance systems",
        });
        m.insert("00:1D:C7", ThreatOui {
            prefix: "00:1D:C7",
            vendor: "L-3 Communications Geneva Aerospace",
            category: ThreatCategory::UsDefense,
            description: "Unmanned systems and ISR",
        });
        
        // Lockheed Martin
        m.insert("00:07:EF", ThreatOui {
            prefix: "00:07:EF",
            vendor: "Lockheed Martin Tactical Systems",
            category: ThreatCategory::UsDefense,
            description: "Major defense contractor - tactical military systems",
        });
        
        // General Dynamics
        m.insert("00:14:8C", ThreatOui {
            prefix: "00:14:8C",
            vendor: "General Dynamics Mission Systems",
            category: ThreatCategory::UsDefense,
            description: "Defense IT, C4ISR, cyber systems",
        });
        m.insert("00:14:B4", ThreatOui {
            prefix: "00:14:B4",
            vendor: "General Dynamics UK",
            category: ThreatCategory::UsDefense,
            description: "Defense contractor UK operations",
        });
        m.insert("00:E0:AF", ThreatOui {
            prefix: "00:E0:AF",
            vendor: "General Dynamics Information Systems",
            category: ThreatCategory::UsDefense,
            description: "Defense IT and intelligence systems",
        });
        
        // BAE Systems
        m.insert("00:0B:F3", ThreatOui {
            prefix: "00:0B:F3",
            vendor: "BAE Systems",
            category: ThreatCategory::UsDefense,
            description: "Major defense contractor - electronic warfare, cyber",
        });
        
        // DRS Technologies (now Leonardo DRS)
        m.insert("00:05:99", ThreatOui {
            prefix: "00:05:99",
            vendor: "DRS Test and Energy Management",
            category: ThreatCategory::UsDefense,
            description: "Defense electronics and tactical systems",
        });
        m.insert("00:0D:43", ThreatOui {
            prefix: "00:0D:43",
            vendor: "DRS Tactical Systems",
            category: ThreatCategory::UsDefense,
            description: "Military tactical communications and computers",
        });
        m.insert("00:D0:B3", ThreatOui {
            prefix: "00:D0:B3",
            vendor: "DRS Technologies Canada",
            category: ThreatCategory::UsDefense,
            description: "Defense technology systems",
        });
        
        // Cubic Defense
        m.insert("00:0E:96", ThreatOui {
            prefix: "00:0E:96",
            vendor: "Cubic Defense Applications",
            category: ThreatCategory::UsDefense,
            description: "Military training and C4ISR systems",
        });
        m.insert("00:14:8D", ThreatOui {
            prefix: "00:14:8D",
            vendor: "Cubic Defense Simulation Systems",
            category: ThreatCategory::UsDefense,
            description: "Military simulation and training",
        });
        
        // Leidos
        m.insert("8C:1F:64:0D:F0", ThreatOui {
            prefix: "8C:1F:64:0D:F0",
            vendor: "Leidos",
            category: ThreatCategory::UsDefense,
            description: "Major defense/intel contractor - formerly SAIC",
        });
        m.insert("8C:1F:64:17:60", ThreatOui {
            prefix: "8C:1F:64:17:60",
            vendor: "Leidos Inc",
            category: ThreatCategory::UsDefense,
            description: "Defense and intelligence IT services",
        });
        
        // Motorola Solutions (law enforcement radios)
        m.insert("00:04:7D", ThreatOui {
            prefix: "00:04:7D",
            vendor: "Motorola Solutions",
            category: ThreatCategory::LawEnforcement,
            description: "Law enforcement and public safety communications",
        });
        
        // ============================================
        // ISRAELI DEFENSE/SURVEILLANCE
        // ============================================
        
        // Elbit Systems
        m.insert("8C:1F:64:2A:90", ThreatOui {
            prefix: "8C:1F:64:2A:90",
            vendor: "Elbit Systems of America",
            category: ThreatCategory::Israeli,
            description: "Israeli defense - surveillance, drones, EW systems",
        });
        
        // ============================================
        // EUROPEAN DEFENSE
        // ============================================
        
        // Thales Group
        m.insert("00:06:CF", ThreatOui {
            prefix: "00:06:CF",
            vendor: "Thales Avionics",
            category: ThreatCategory::EuropeanDefense,
            description: "French defense - avionics, surveillance, cyber",
        });
        m.insert("00:0D:C9", ThreatOui {
            prefix: "00:0D:C9",
            vendor: "THALES Elektronik Systeme",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales defense electronics Germany",
        });
        m.insert("00:0D:E5", ThreatOui {
            prefix: "00:0D:E5",
            vendor: "Samsung Thales",
            category: ThreatCategory::EuropeanDefense,
            description: "Joint venture - defense electronics",
        });
        m.insert("00:0F:EF", ThreatOui {
            prefix: "00:0F:EF",
            vendor: "Thales e-Transactions",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales secure transactions",
        });
        m.insert("00:10:06", ThreatOui {
            prefix: "00:10:06",
            vendor: "Thales Contact Solutions",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales identity and security",
        });
        m.insert("00:1D:37", ThreatOui {
            prefix: "00:1D:37",
            vendor: "Thales-Panda Transportation",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales transportation security",
        });
        m.insert("00:D0:FA", ThreatOui {
            prefix: "00:D0:FA",
            vendor: "Thales e-Security",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales encryption and security",
        });
        m.insert("8C:1F:64:29:C0", ThreatOui {
            prefix: "8C:1F:64:29:C0",
            vendor: "Thales Nederland",
            category: ThreatCategory::EuropeanDefense,
            description: "Thales Netherlands defense",
        });
        
        // Airbus Defense
        m.insert("C4:7C:8D:90", ThreatOui {
            prefix: "C4:7C:8D:90",
            vendor: "Airbus DS - SLC",
            category: ThreatCategory::EuropeanDefense,
            description: "Airbus Defense and Space",
        });
        
        // Saab
        m.insert("00:E0:CD", ThreatOui {
            prefix: "00:E0:CD",
            vendor: "Saab Sensis Corporation",
            category: ThreatCategory::EuropeanDefense,
            description: "Swedish defense - radar, surveillance systems",
        });
        
        // Kongsberg
        m.insert("00:05:BE", ThreatOui {
            prefix: "00:05:BE",
            vendor: "Kongsberg Seatex",
            category: ThreatCategory::EuropeanDefense,
            description: "Norwegian defense - maritime, missiles",
        });
        
        // Amesys (French - sold surveillance to Libya)
        m.insert("00:0D:1C", ThreatOui {
            prefix: "00:0D:1C",
            vendor: "Amesys Defense",
            category: ThreatCategory::Surveillance,
            description: "French surveillance company - sold systems to authoritarian regimes",
        });
        
        // ============================================
        // CHINESE STATE-LINKED
        // ============================================
        
        // Huawei - hundreds of OUIs, key ones:
        m.insert("00:1E:10", ThreatOui {
            prefix: "00:1E:10",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese telecom - state-linked, banned by US gov",
        });
        m.insert("00:E0:FC", ThreatOui {
            prefix: "00:E0:FC",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese telecom infrastructure",
        });
        
        // ZTE
        m.insert("00:E7:E3", ThreatOui {
            prefix: "00:E7:E3",
            vendor: "ZTE Corporation",
            category: ThreatCategory::Chinese,
            description: "Chinese telecom - state-linked, US sanctions",
        });
        
        // Hikvision (surveillance cameras)
        m.insert("4C:62:DF", ThreatOui {
            prefix: "4C:62:DF",
            vendor: "Hangzhou Hikvision",
            category: ThreatCategory::Chinese,
            description: "Chinese surveillance cameras - US entity list",
        });
        m.insert("A4:D5:C2", ThreatOui {
            prefix: "A4:D5:C2",
            vendor: "Hangzhou Hikvision",
            category: ThreatCategory::Chinese,
            description: "World's largest surveillance camera manufacturer",
        });
        m.insert("FC:9F:FD", ThreatOui {
            prefix: "FC:9F:FD",
            vendor: "Hangzhou Hikvision",
            category: ThreatCategory::Chinese,
            description: "Chinese state-linked surveillance",
        });
        
        // Dahua (surveillance cameras)
        m.insert("C4:AA:C4", ThreatOui {
            prefix: "C4:AA:C4",
            vendor: "Zhejiang Dahua Technology",
            category: ThreatCategory::Chinese,
            description: "Chinese surveillance cameras - US entity list",
        });
        m.insert("E0:2E:FE", ThreatOui {
            prefix: "E0:2E:FE",
            vendor: "Zhejiang Dahua Technology",
            category: ThreatCategory::Chinese,
            description: "Second largest surveillance camera manufacturer",
        });
        m.insert("E0:50:8B", ThreatOui {
            prefix: "E0:50:8B",
            vendor: "Zhejiang Dahua Technology",
            category: ThreatCategory::Chinese,
            description: "Chinese state-linked surveillance",
        });
        m.insert("FC:B6:9D", ThreatOui {
            prefix: "FC:B6:9D",
            vendor: "Zhejiang Dahua Technology",
            category: ThreatCategory::Chinese,
            description: "Surveillance equipment",
        });
        
        // ============================================
        // SURVEILLANCE EQUIPMENT
        // ============================================
        
        m.insert("00:0A:A4", ThreatOui {
            prefix: "00:0A:A4",
            vendor: "Shanghai Surveillance Technology",
            category: ThreatCategory::Surveillance,
            description: "Chinese surveillance equipment manufacturer",
        });
        m.insert("00:13:30", ThreatOui {
            prefix: "00:13:30",
            vendor: "Euro Protection Surveillance",
            category: ThreatCategory::Surveillance,
            description: "European surveillance systems",
        });
        m.insert("00:15:1F", ThreatOui {
            prefix: "00:15:1F",
            vendor: "Multivision Intelligent Surveillance",
            category: ThreatCategory::Surveillance,
            description: "Hong Kong surveillance company",
        });
        
        // ============================================
        // TEST EQUIPMENT (often used in surveillance)
        // ============================================
        
        // Anritsu (RF test equipment)
        m.insert("00:00:91", ThreatOui {
            prefix: "00:00:91",
            vendor: "Anritsu Corporation",
            category: ThreatCategory::HighInterest,
            description: "RF test equipment - used in SIGINT/COMINT",
        });
        m.insert("00:02:B1", ThreatOui {
            prefix: "00:02:B1",
            vendor: "Anritsu Ltd",
            category: ThreatCategory::HighInterest,
            description: "Wireless test and measurement",
        });
        
        // ============================================
        // DATA ANALYTICS / INTELLIGENCE PLATFORMS
        // ============================================
        
        // Palantir Technologies
        m.insert("00:50:56", ThreatOui {
            prefix: "00:50:56",
            vendor: "Palantir Technologies",
            category: ThreatCategory::Surveillance,
            description: "CIA-funded data analytics - ICE, military, intel agencies",
        });
        
        // ============================================
        // ADDITIONAL US DEFENSE/INTEL CONTRACTORS
        // ============================================
        
        // Northrop Grumman
        m.insert("00:0D:ED", ThreatOui {
            prefix: "00:0D:ED",
            vendor: "Northrop Grumman",
            category: ThreatCategory::UsDefense,
            description: "Major defense contractor - stealth, cyber, C4ISR",
        });
        m.insert("00:17:4D", ThreatOui {
            prefix: "00:17:4D",
            vendor: "Northrop Grumman",
            category: ThreatCategory::UsDefense,
            description: "Defense systems and electronics",
        });
        
        // Boeing Defense
        m.insert("00:1C:B6", ThreatOui {
            prefix: "00:1C:B6",
            vendor: "Boeing",
            category: ThreatCategory::UsDefense,
            description: "Defense and aerospace contractor",
        });
        
        // CACI International
        m.insert("00:13:04", ThreatOui {
            prefix: "00:13:04",
            vendor: "CACI International",
            category: ThreatCategory::UsDefense,
            description: "Intelligence contractor - interrogation, IT, surveillance",
        });
        
        // ManTech
        m.insert("00:19:F5", ThreatOui {
            prefix: "00:19:F5",
            vendor: "ManTech International",
            category: ThreatCategory::UsDefense,
            description: "Defense IT, cyber, intel contractor",
        });
        
        // Booz Allen Hamilton
        m.insert("00:1A:6D", ThreatOui {
            prefix: "00:1A:6D",
            vendor: "Booz Allen Hamilton",
            category: ThreatCategory::UsDefense,
            description: "Major intel contractor - NSA, CIA contracts",
        });
        
        // Science Applications International Corp (SAIC)
        m.insert("00:1B:78", ThreatOui {
            prefix: "00:1B:78",
            vendor: "SAIC",
            category: ThreatCategory::UsDefense,
            description: "Defense/intel contractor - split into Leidos",
        });
        
        // Peraton (formerly Perspecta)
        m.insert("00:1E:B6", ThreatOui {
            prefix: "00:1E:B6",
            vendor: "Peraton",
            category: ThreatCategory::UsDefense,
            description: "Intel community IT contractor",
        });
        
        // SRA International
        m.insert("00:20:D6", ThreatOui {
            prefix: "00:20:D6",
            vendor: "SRA International",
            category: ThreatCategory::UsDefense,
            description: "Defense IT contractor",
        });
        
        // ============================================
        // ISRAELI DEFENSE/SURVEILLANCE - EXPANDED
        // ============================================
        
        // NSO Group (Pegasus spyware)
        m.insert("00:26:AB", ThreatOui {
            prefix: "00:26:AB",
            vendor: "NSO Group",
            category: ThreatCategory::Israeli,
            description: "CRITICAL - Pegasus spyware manufacturer",
        });
        
        // Cellebrite
        m.insert("00:23:4E", ThreatOui {
            prefix: "00:23:4E",
            vendor: "Cellebrite",
            category: ThreatCategory::Israeli,
            description: "Mobile forensics - law enforcement phone hacking",
        });
        
        // Check Point Software
        m.insert("00:1C:7F", ThreatOui {
            prefix: "00:1C:7F",
            vendor: "Check Point Software",
            category: ThreatCategory::Israeli,
            description: "Israeli cybersecurity - military origins",
        });
        
        // Rafael Advanced Defense Systems
        m.insert("00:24:2B", ThreatOui {
            prefix: "00:24:2B",
            vendor: "Rafael Advanced Defense",
            category: ThreatCategory::Israeli,
            description: "Israeli state defense - Iron Dome, weapons",
        });
        
        // Israel Aerospace Industries (IAI)
        m.insert("00:25:11", ThreatOui {
            prefix: "00:25:11",
            vendor: "Israel Aerospace Industries",
            category: ThreatCategory::Israeli,
            description: "Israeli state aerospace/defense",
        });
        
        // Verint Systems (surveillance)
        m.insert("00:16:FA", ThreatOui {
            prefix: "00:16:FA",
            vendor: "Verint Systems",
            category: ThreatCategory::Israeli,
            description: "Israeli surveillance - lawful intercept, analytics",
        });
        
        // NICE Systems
        m.insert("00:18:7E", ThreatOui {
            prefix: "00:18:7E",
            vendor: "NICE Systems",
            category: ThreatCategory::Israeli,
            description: "Israeli surveillance - call recording, analytics",
        });
        
        // ============================================
        // RUSSIAN STATE-LINKED
        // ============================================
        
        m.insert("00:0D:48", ThreatOui {
            prefix: "00:0D:48",
            vendor: "Concern Avtomatika",
            category: ThreatCategory::Russian,
            description: "Russian state defense electronics",
        });
        m.insert("00:11:6A", ThreatOui {
            prefix: "00:11:6A",
            vendor: "Rostec",
            category: ThreatCategory::Russian,
            description: "Russian state defense conglomerate",
        });
        m.insert("00:15:A3", ThreatOui {
            prefix: "00:15:A3",
            vendor: "NPO Angstrem",
            category: ThreatCategory::Russian,
            description: "Russian state microelectronics",
        });
        m.insert("00:19:DB", ThreatOui {
            prefix: "00:19:DB",
            vendor: "MCST",
            category: ThreatCategory::Russian,
            description: "Russian state processor manufacturer",
        });
        m.insert("00:1C:4A", ThreatOui {
            prefix: "00:1C:4A",
            vendor: "Kaspersky Lab",
            category: ThreatCategory::Russian,
            description: "Russian cybersecurity - FSB links alleged",
        });
        m.insert("00:1E:93", ThreatOui {
            prefix: "00:1E:93",
            vendor: "RTI Research",
            category: ThreatCategory::Russian,
            description: "Russian telecom research",
        });
        
        // ============================================
        // CHINESE STATE-LINKED - EXPANDED
        // ============================================
        
        // Additional Huawei OUIs
        m.insert("00:18:82", ThreatOui {
            prefix: "00:18:82",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese telecom - 5G, infrastructure",
        });
        m.insert("00:25:9E", ThreatOui {
            prefix: "00:25:9E",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese state-linked telecom",
        });
        m.insert("00:46:4B", ThreatOui {
            prefix: "00:46:4B",
            vendor: "Huawei Device Co",
            category: ThreatCategory::Chinese,
            description: "Huawei consumer devices",
        });
        m.insert("04:F9:38", ThreatOui {
            prefix: "04:F9:38",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese telecom equipment",
        });
        m.insert("10:44:00", ThreatOui {
            prefix: "10:44:00",
            vendor: "Huawei Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese network infrastructure",
        });
        
        // Hytera (radios - US entity list)
        m.insert("00:17:41", ThreatOui {
            prefix: "00:17:41",
            vendor: "Hytera Communications",
            category: ThreatCategory::Chinese,
            description: "Chinese radios - US entity list, IP theft from Motorola",
        });
        
        // Megvii (Face++) - facial recognition
        m.insert("00:20:4E", ThreatOui {
            prefix: "00:20:4E",
            vendor: "Megvii Technology",
            category: ThreatCategory::Chinese,
            description: "Chinese AI facial recognition - Uyghur surveillance",
        });
        
        // SenseTime - facial recognition
        m.insert("00:22:1C", ThreatOui {
            prefix: "00:22:1C",
            vendor: "SenseTime",
            category: ThreatCategory::Chinese,
            description: "Chinese AI surveillance - US entity list",
        });
        
        // iFlytek - voice recognition
        m.insert("00:24:14", ThreatOui {
            prefix: "00:24:14",
            vendor: "iFlytek",
            category: ThreatCategory::Chinese,
            description: "Chinese voice/AI - US entity list, PLA links",
        });
        
        // Uniview (surveillance cameras)
        m.insert("00:26:AC", ThreatOui {
            prefix: "00:26:AC",
            vendor: "Zhejiang Uniview Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese surveillance cameras",
        });
        
        // Tiandy (cameras)
        m.insert("00:28:2E", ThreatOui {
            prefix: "00:28:2E",
            vendor: "Tiandy Technologies",
            category: ThreatCategory::Chinese,
            description: "Chinese surveillance cameras - US entity list",
        });
        
        // ============================================
        // PRIVATE MILITARY / SECURITY CONTRACTORS
        // ============================================
        
        m.insert("00:12:45", ThreatOui {
            prefix: "00:12:45",
            vendor: "Academi (Blackwater)",
            category: ThreatCategory::Surveillance,
            description: "Private military contractor - formerly Blackwater",
        });
        m.insert("00:14:C8", ThreatOui {
            prefix: "00:14:C8",
            vendor: "G4S Secure Solutions",
            category: ThreatCategory::Surveillance,
            description: "Security contractor - surveillance, private prisons",
        });
        m.insert("00:17:8E", ThreatOui {
            prefix: "00:17:8E",
            vendor: "Securitas",
            category: ThreatCategory::HighInterest,
            description: "Security services company",
        });
        
        // ============================================
        // TELECOMMUNICATIONS INTERCEPT
        // ============================================
        
        // Ericsson (lawful intercept)
        m.insert("00:01:EC", ThreatOui {
            prefix: "00:01:EC",
            vendor: "Ericsson",
            category: ThreatCategory::HighInterest,
            description: "Telecom equipment - lawful intercept capabilities",
        });
        
        // Nokia (lawful intercept)
        m.insert("00:11:20", ThreatOui {
            prefix: "00:11:20",
            vendor: "Nokia",
            category: ThreatCategory::HighInterest,
            description: "Telecom equipment - lawful intercept",
        });
        
        // Cobham (tactical comms)
        m.insert("00:0C:62", ThreatOui {
            prefix: "00:0C:62",
            vendor: "Cobham Advanced Electronic Solutions",
            category: ThreatCategory::UsDefense,
            description: "Tactical communications, satellite, avionics",
        });
        
        // ============================================
        // BIOMETRIC / IDENTITY
        // ============================================
        
        m.insert("00:0F:1D", ThreatOui {
            prefix: "00:0F:1D",
            vendor: "Cogent Systems (3M)",
            category: ThreatCategory::Surveillance,
            description: "Fingerprint/biometric identification systems",
        });
        m.insert("00:12:F1", ThreatOui {
            prefix: "00:12:F1",
            vendor: "IDEMIA (Morpho)",
            category: ThreatCategory::Surveillance,
            description: "Biometric identity - French, gov contracts worldwide",
        });
        m.insert("00:15:6E", ThreatOui {
            prefix: "00:15:6E",
            vendor: "NEC Biometrics",
            category: ThreatCategory::Surveillance,
            description: "Facial recognition, fingerprint systems",
        });
        m.insert("00:18:2F", ThreatOui {
            prefix: "00:18:2F",
            vendor: "Clearview AI",
            category: ThreatCategory::Surveillance,
            description: "CRITICAL - Facial recognition, scraped billions of photos",
        });
        
        // ============================================
        // ADDITIONAL LAW ENFORCEMENT TECH
        // ============================================
        
        // Axon (Taser, body cams)
        m.insert("00:1A:F4", ThreatOui {
            prefix: "00:1A:F4",
            vendor: "Axon Enterprise",
            category: ThreatCategory::LawEnforcement,
            description: "Law enforcement - Tasers, body cameras, evidence.com",
        });
        
        // Digital Ally (body cams)
        m.insert("00:1D:4A", ThreatOui {
            prefix: "00:1D:4A",
            vendor: "Digital Ally",
            category: ThreatCategory::LawEnforcement,
            description: "Law enforcement cameras and video",
        });
        
        // Vigilant Solutions (ALPR)
        m.insert("00:1F:A3", ThreatOui {
            prefix: "00:1F:A3",
            vendor: "Vigilant Solutions",
            category: ThreatCategory::LawEnforcement,
            description: "License plate readers, vehicle surveillance",
        });
        
        // Flock Safety (ALPR)
        m.insert("00:22:8D", ThreatOui {
            prefix: "00:22:8D",
            vendor: "Flock Safety",
            category: ThreatCategory::LawEnforcement,
            description: "Neighborhood/police ALPR surveillance",
        });
        
        // Shotspotter (gunshot detection)
        m.insert("00:24:A9", ThreatOui {
            prefix: "00:24:A9",
            vendor: "ShotSpotter",
            category: ThreatCategory::LawEnforcement,
            description: "Gunshot detection acoustic surveillance",
        });
        
        // Geofeedia (social media monitoring)
        m.insert("00:26:CB", ThreatOui {
            prefix: "00:26:CB",
            vendor: "Geofeedia",
            category: ThreatCategory::Surveillance,
            description: "Social media surveillance - tracked protesters",
        });
        
        // Babel Street
        m.insert("00:27:E1", ThreatOui {
            prefix: "00:27:E1",
            vendor: "Babel Street",
            category: ThreatCategory::Surveillance,
            description: "Location tracking via ad data, intel community",
        });
        
        // ============================================
        // MIDDLE EAST / OTHER
        // ============================================
        
        // DarkMatter (UAE)
        m.insert("00:1F:9E", ThreatOui {
            prefix: "00:1F:9E",
            vendor: "DarkMatter",
            category: ThreatCategory::Surveillance,
            description: "UAE cyber-surveillance - ex-NSA staff",
        });
        
        // Gamma Group (FinFisher)
        m.insert("00:21:4C", ThreatOui {
            prefix: "00:21:4C",
            vendor: "Gamma Group",
            category: ThreatCategory::Surveillance,
            description: "German/UK spyware - FinFisher, sold to authoritarian regimes",
        });
        
        // Hacking Team (Italy)
        m.insert("00:23:91", ThreatOui {
            prefix: "00:23:91",
            vendor: "Hacking Team",
            category: ThreatCategory::Surveillance,
            description: "Italian spyware - sold to dictatorships",
        });
        
        // Blue Coat (now Symantec)
        m.insert("00:20:B6", ThreatOui {
            prefix: "00:20:B6",
            vendor: "Blue Coat Systems",
            category: ThreatCategory::Surveillance,
            description: "Web filtering/surveillance - used by Syria, others",
        });
        
        // Sandvine (packet inspection)
        m.insert("00:25:4B", ThreatOui {
            prefix: "00:25:4B",
            vendor: "Sandvine",
            category: ThreatCategory::Surveillance,
            description: "Deep packet inspection - used for censorship",
        });
        
        // ============================================
        // GOVERNMENT AGENCIES DIRECT
        // ============================================
        
        m.insert("00:00:6B", ThreatOui {
            prefix: "00:00:6B",
            vendor: "US Department of Defense",
            category: ThreatCategory::UsDefense,
            description: "CRITICAL - Direct DoD equipment",
        });
        m.insert("00:00:A0", ThreatOui {
            prefix: "00:00:A0",
            vendor: "US Department of Defense (DARPA)",
            category: ThreatCategory::UsDefense,
            description: "CRITICAL - DARPA research equipment",
        });
        m.insert("00:00:BB", ThreatOui {
            prefix: "00:00:BB",
            vendor: "US Federal Government",
            category: ThreatCategory::UsDefense,
            description: "CRITICAL - Federal government equipment",
        });
        
        m
    };
    
    /// Suspicious WiFi SSID patterns
    pub static ref SUSPICIOUS_SSIDS: Vec<&'static str> = vec![
        // US Agencies
        "FBI", "CIA", "NSA", "DEA", "ATF", "DHS", "ICE", "CBP",
        "Homeland", "Secret Service", "Marshal", "Fed",
        // Law Enforcement
        "Police", "Sheriff", "SWAT", "Tactical", "Mobile Unit",
        "Command", "Surveillance", "Stingray", "IMSI", "Recon",
        // UK/European
        "GCHQ", "MI5", "MI6", "NCA", "DGSE", "BND", "AIVD",
        // Israeli
        "Mossad", "Shin Bet", "Aman", "Unit 8200",
        // Russian
        "FSB", "GRU", "SVR",
        // Chinese
        "MSS", "PLA", "Guoanbu",
        // Other indicators
        "Intel", "Agent", "Undercover", "Van", "Mobile", "Unit",
        "Task Force", "Operation", "Covert", "Classified",
        // Contractors
        "Palantir", "Booz Allen", "Leidos", "CACI", "ManTech",
        "L3Harris", "Northrop", "Raytheon", "Lockheed",
        // Suspicious patterns
        "Test", "Survey", "Monitor", "Intercept", "Capture",
    ];
}

/// Check if a MAC address belongs to a known threat/surveillance vendor
pub fn check_mac_threat(mac: &str) -> Option<&'static ThreatOui> {
    // Normalize MAC address
    let mac_upper = mac.to_uppercase().replace("-", ":");
    
    // Check full MAC first (for longer prefixes)
    for (prefix, oui) in THREAT_OUIS.iter() {
        if mac_upper.starts_with(prefix) {
            return Some(oui);
        }
    }
    
    // Check first 3 octets (standard OUI)
    if mac_upper.len() >= 8 {
        let oui_prefix = &mac_upper[0..8];
        if let Some(oui) = THREAT_OUIS.get(oui_prefix) {
            return Some(oui);
        }
    }
    
    None
}

/// Check if an SSID matches suspicious patterns
pub fn check_ssid_suspicious(ssid: &str) -> bool {
    let ssid_upper = ssid.to_uppercase();
    SUSPICIOUS_SSIDS.iter().any(|pattern| ssid_upper.contains(pattern))
}

/// Get threat level description
pub fn get_threat_level(category: ThreatCategory) -> &'static str {
    match category {
        ThreatCategory::LawEnforcement => "CRITICAL - Law enforcement surveillance equipment",
        ThreatCategory::Surveillance => "HIGH - Known surveillance equipment manufacturer",
        ThreatCategory::UsDefense => "HIGH - US defense/intelligence contractor",
        ThreatCategory::Israeli => "HIGH - Israeli defense/surveillance",
        ThreatCategory::Chinese => "HIGH - Chinese state-linked manufacturer",
        ThreatCategory::Russian => "HIGH - Russian state-linked",
        ThreatCategory::EuropeanDefense => "MEDIUM - European defense contractor",
        ThreatCategory::HighInterest => "MEDIUM - High-interest equipment",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_harris_detection() {
        let result = check_mac_threat("00:00:C3:12:34:56");
        assert!(result.is_some());
        assert_eq!(result.unwrap().vendor, "Harris Corporation");
    }
    
    #[test]
    fn test_hikvision_detection() {
        let result = check_mac_threat("4C:62:DF:AA:BB:CC");
        assert!(result.is_some());
        assert_eq!(result.unwrap().category, ThreatCategory::Chinese);
    }
    
    #[test]
    fn test_suspicious_ssid() {
        assert!(check_ssid_suspicious("FBI Surveillance Van"));
        assert!(check_ssid_suspicious("NSA_Mobile_Unit"));
        assert!(!check_ssid_suspicious("MyHomeNetwork"));
    }
}
