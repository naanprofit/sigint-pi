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
        
        m
    };
    
    /// Suspicious WiFi SSID patterns
    pub static ref SUSPICIOUS_SSIDS: Vec<&'static str> = vec![
        "FBI",
        "CIA",
        "NSA",
        "DEA",
        "ATF",
        "DHS",
        "Surveillance",
        "Stingray",
        "IMSI",
        "Police",
        "Marshal",
        "Fed",
        "Agent",
        "Tactical",
        "Mobile Unit",
        "Command",
        "Recon",
        "Intel",
        "GCHQ",
        "MI5",
        "MI6",
        "Mossad",
        "FSB",
        "GRU",
        "MSS",
        "PLA",
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
