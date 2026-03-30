use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::info;

/// OUI (Organizationally Unique Identifier) lookup for MAC address vendor identification
pub struct OuiLookup {
    database: HashMap<String, String>,
}

impl OuiLookup {
    /// Load OUI database from IEEE format file
    pub fn load(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open OUI file: {:?}", path))?;
        
        let reader = BufReader::new(file);
        let mut database = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            
            // IEEE format: "XX-XX-XX   (hex)		Vendor Name"
            // or Wireshark format: "XX:XX:XX	Vendor Name"
            if let Some((oui, vendor)) = parse_oui_line(&line) {
                database.insert(oui, vendor);
            }
        }

        info!("Loaded {} OUI entries", database.len());
        Ok(Self { database })
    }

    /// Create with embedded minimal database
    pub fn embedded() -> Self {
        let mut database = HashMap::new();
        
        // Common vendors (partial list)
        let entries = [
            ("00:00:0C", "Cisco Systems"),
            ("00:03:93", "Apple"),
            ("00:05:02", "Apple"),
            ("00:0A:27", "Apple"),
            ("00:0A:95", "Apple"),
            ("00:0D:93", "Apple"),
            ("00:10:FA", "Apple"),
            ("00:11:24", "Apple"),
            ("00:14:51", "Apple"),
            ("00:16:CB", "Apple"),
            ("00:17:F2", "Apple"),
            ("00:19:E3", "Apple"),
            ("00:1B:63", "Apple"),
            ("00:1C:B3", "Apple"),
            ("00:1D:4F", "Apple"),
            ("00:1E:52", "Apple"),
            ("00:1E:C2", "Apple"),
            ("00:1F:5B", "Apple"),
            ("00:1F:F3", "Apple"),
            ("00:21:E9", "Apple"),
            ("00:22:41", "Apple"),
            ("00:23:12", "Apple"),
            ("00:23:32", "Apple"),
            ("00:23:6C", "Apple"),
            ("00:23:DF", "Apple"),
            ("00:24:36", "Apple"),
            ("00:25:00", "Apple"),
            ("00:25:4B", "Apple"),
            ("00:25:BC", "Apple"),
            ("00:26:08", "Apple"),
            ("00:26:4A", "Apple"),
            ("00:26:B0", "Apple"),
            ("00:26:BB", "Apple"),
            ("00:50:56", "VMware"),
            ("00:0C:29", "VMware"),
            ("00:15:5D", "Microsoft Hyper-V"),
            ("08:00:27", "VirtualBox"),
            ("00:1A:11", "Google"),
            ("3C:5A:B4", "Google"),
            ("54:60:09", "Google"),
            ("00:17:C4", "Netgear"),
            ("00:1F:33", "Netgear"),
            ("00:22:3F", "Netgear"),
            ("00:26:F2", "Netgear"),
            ("00:1E:58", "D-Link"),
            ("00:22:B0", "D-Link"),
            ("00:24:01", "D-Link"),
            ("00:1C:DF", "Belkin"),
            ("00:17:3F", "Belkin"),
            ("00:22:75", "Belkin"),
            ("00:1A:2B", "Ayecom Technology"),
            ("00:1D:7E", "Cisco-Linksys"),
            ("00:21:29", "Cisco-Linksys"),
            ("00:22:6B", "Cisco-Linksys"),
            ("00:23:69", "Cisco-Linksys"),
            ("00:25:9C", "Cisco-Linksys"),
            ("00:1A:A0", "Dell"),
            ("00:21:9B", "Dell"),
            ("00:24:E8", "Dell"),
            ("00:25:64", "Dell"),
            ("00:26:B9", "Dell"),
            ("00:1C:23", "Dell"),
            ("00:1E:4F", "Dell"),
            ("00:22:19", "Dell"),
            ("00:12:3F", "Dell"),
            ("00:14:22", "Dell"),
            ("00:15:C5", "Dell"),
            ("00:18:8B", "Dell"),
            ("00:19:B9", "Dell"),
            ("00:1A:A0", "Dell"),
            ("00:1D:09", "Dell"),
            ("00:1E:C9", "Dell"),
            ("00:21:70", "Dell"),
            ("00:23:AE", "Dell"),
            ("00:24:E8", "Dell"),
            ("00:26:B9", "Dell"),
            ("B8:AC:6F", "Dell"),
            ("D4:BE:D9", "Dell"),
            ("00:1E:68", "Quanta Computer"),
            ("00:1B:FC", "ASUSTek Computer"),
            ("00:1D:60", "ASUSTek Computer"),
            ("00:22:15", "ASUSTek Computer"),
            ("00:23:54", "ASUSTek Computer"),
            ("00:24:8C", "ASUSTek Computer"),
            ("00:26:18", "ASUSTek Computer"),
            ("00:0E:A6", "ASUSTek Computer"),
            ("00:11:D8", "ASUSTek Computer"),
            ("00:13:D4", "ASUSTek Computer"),
            ("00:15:F2", "ASUSTek Computer"),
            ("00:17:31", "ASUSTek Computer"),
            ("00:18:F3", "ASUSTek Computer"),
            ("00:1A:92", "ASUSTek Computer"),
            ("00:1D:60", "ASUSTek Computer"),
            ("00:1F:C6", "ASUSTek Computer"),
            ("00:22:15", "ASUSTek Computer"),
            ("00:23:54", "ASUSTek Computer"),
            ("00:24:8C", "ASUSTek Computer"),
            ("00:E0:4C", "Realtek"),
            ("00:1D:0F", "TP-Link"),
            ("00:21:27", "TP-Link"),
            ("00:23:CD", "TP-Link"),
            ("00:25:86", "TP-Link"),
            ("00:27:19", "TP-Link"),
            ("10:FE:ED", "TP-Link"),
            ("14:CC:20", "TP-Link"),
            ("14:CF:92", "TP-Link"),
            ("18:A6:F7", "TP-Link"),
            ("1C:FA:68", "TP-Link"),
            ("20:DC:E6", "TP-Link"),
            ("24:69:68", "TP-Link"),
            ("28:EE:52", "TP-Link"),
            ("30:B5:C2", "TP-Link"),
            ("34:E8:94", "TP-Link"),
            ("38:83:45", "TP-Link"),
            ("3C:46:D8", "TP-Link"),
            ("44:32:C8", "TP-Link"),
            ("48:EE:0C", "TP-Link"),
            ("50:C7:BF", "TP-Link"),
            ("54:C8:0F", "TP-Link"),
            ("58:D9:D5", "TP-Link"),
            ("5C:89:9A", "TP-Link"),
            ("5C:E9:31", "TP-Link"),
            ("60:E3:27", "TP-Link"),
            ("64:56:01", "TP-Link"),
            ("64:66:B3", "TP-Link"),
            ("64:70:02", "TP-Link"),
            ("98:DA:C4", "TP-Link"),
            ("A0:F3:C1", "TP-Link"),
            ("AC:84:C6", "TP-Link"),
            ("B0:4E:26", "TP-Link"),
            ("B0:95:75", "TP-Link"),
            ("C0:25:E9", "TP-Link"),
            ("C4:6E:1F", "TP-Link"),
            ("C8:3A:35", "Tenda Technology"),
            ("00:1F:1F", "Edimax Technology"),
            ("00:21:91", "D-Link"),
            ("00:1E:58", "D-Link"),
            ("00:19:5B", "D-Link"),
            ("1C:7E:E5", "D-Link"),
            ("1C:BD:B9", "D-Link"),
            ("28:10:7B", "D-Link"),
            ("28:EE:52", "TP-Link Technologies"),
            ("30:B5:C2", "TP-Link Technologies"),
            ("34:96:72", "TP-Link Technologies"),
            ("50:C7:BF", "TP-Link Technologies"),
            ("54:C8:0F", "TP-Link Technologies"),
            ("60:E3:27", "TP-Link Technologies"),
            ("64:56:01", "TP-Link Technologies"),
            ("64:66:B3", "TP-Link Technologies"),
            ("64:70:02", "TP-Link Technologies"),
            ("6C:B0:CE", "Netgear"),
            ("84:1B:5E", "Netgear"),
            ("9C:3D:CF", "Netgear"),
            ("A0:21:B7", "Netgear"),
            ("A0:40:A0", "Netgear"),
            ("A4:2B:8C", "Netgear"),
            ("B0:7F:B9", "Netgear"),
            ("B0:B9:8A", "Netgear"),
            ("C0:3F:0E", "Netgear"),
            ("C4:04:15", "Netgear"),
            ("C8:9E:43", "Netgear"),
            ("CC:40:D0", "Netgear"),
            ("D4:CA:6D", "Netgear"),
            ("E0:46:9A", "Netgear"),
            ("E0:91:F5", "Netgear"),
            ("E4:F4:C6", "Netgear"),
            ("E8:FC:AF", "Netgear"),
            ("00:24:B2", "Netgear"),
            ("00:26:F2", "Netgear"),
            ("2C:B0:5D", "Netgear"),
            ("30:46:9A", "Netgear"),
            ("38:94:ED", "Netgear"),
            ("4C:60:DE", "Netgear"),
            ("20:E5:2A", "Netgear"),
            ("6C:B0:CE", "Netgear"),
            ("EC:1A:59", "Belkin International"),
            ("94:10:3E", "Belkin International"),
            ("C0:56:27", "Belkin International"),
            ("08:86:3B", "Belkin International"),
            ("B4:75:0E", "Belkin International"),
            ("E8:9F:80", "Belkin International"),
            // Samsung
            ("00:07:AB", "Samsung"),
            ("00:12:47", "Samsung"),
            ("00:12:FB", "Samsung"),
            ("00:15:B9", "Samsung"),
            ("00:16:32", "Samsung"),
            ("00:17:C9", "Samsung"),
            ("00:17:D5", "Samsung"),
            ("00:18:AF", "Samsung"),
            ("00:1A:8A", "Samsung"),
            ("00:1B:98", "Samsung"),
            ("00:1C:43", "Samsung"),
            ("00:1D:25", "Samsung"),
            ("00:1D:F6", "Samsung"),
            ("00:1E:7D", "Samsung"),
            ("00:1F:CC", "Samsung"),
            ("00:21:19", "Samsung"),
            ("00:21:4C", "Samsung"),
            ("00:21:D1", "Samsung"),
            ("00:23:39", "Samsung"),
            ("00:23:99", "Samsung"),
            ("00:23:D6", "Samsung"),
            ("00:24:54", "Samsung"),
            ("00:24:90", "Samsung"),
            ("00:24:91", "Samsung"),
            ("00:24:E9", "Samsung"),
            ("00:25:66", "Samsung"),
            ("00:25:67", "Samsung"),
            ("00:26:37", "Samsung"),
            ("00:26:5D", "Samsung"),
            ("00:26:5F", "Samsung"),
            // Raspberry Pi Foundation
            ("B8:27:EB", "Raspberry Pi Foundation"),
            ("DC:A6:32", "Raspberry Pi Trading"),
            ("E4:5F:01", "Raspberry Pi Trading"),
            // Espressif (ESP8266, ESP32)
            ("18:FE:34", "Espressif"),
            ("24:0A:C4", "Espressif"),
            ("24:62:AB", "Espressif"),
            ("24:6F:28", "Espressif"),
            ("24:B2:DE", "Espressif"),
            ("2C:3A:E8", "Espressif"),
            ("30:AE:A4", "Espressif"),
            ("3C:61:05", "Espressif"),
            ("3C:71:BF", "Espressif"),
            ("40:F5:20", "Espressif"),
            ("48:3F:DA", "Espressif"),
            ("4C:11:AE", "Espressif"),
            ("4C:75:25", "Espressif"),
            ("54:5A:A6", "Espressif"),
            ("5C:CF:7F", "Espressif"),
            ("60:01:94", "Espressif"),
            ("68:C6:3A", "Espressif"),
            ("70:03:9F", "Espressif"),
            ("7C:9E:BD", "Espressif"),
            ("80:7D:3A", "Espressif"),
            ("84:0D:8E", "Espressif"),
            ("84:CC:A8", "Espressif"),
            ("84:F3:EB", "Espressif"),
            ("8C:4B:14", "Espressif"),
            ("90:97:D5", "Espressif"),
            ("94:B5:55", "Espressif"),
            ("98:CD:AC", "Espressif"),
            ("98:F4:AB", "Espressif"),
            ("A0:20:A6", "Espressif"),
            ("A4:7B:9D", "Espressif"),
            ("A4:CF:12", "Espressif"),
            ("AC:67:B2", "Espressif"),
            ("B4:E6:2D", "Espressif"),
            ("BC:DD:C2", "Espressif"),
            ("C4:4F:33", "Espressif"),
            ("C8:2B:96", "Espressif"),
            ("CC:50:E3", "Espressif"),
            ("D8:A0:1D", "Espressif"),
            ("D8:BF:C0", "Espressif"),
            ("DC:4F:22", "Espressif"),
            ("E0:98:06", "Espressif"),
            ("EC:FA:BC", "Espressif"),
            ("F4:CF:A2", "Espressif"),
            // Intel
            ("00:02:B3", "Intel"),
            ("00:03:47", "Intel"),
            ("00:04:23", "Intel"),
            ("00:07:E9", "Intel"),
            ("00:0C:F1", "Intel"),
            ("00:0E:0C", "Intel"),
            ("00:0E:35", "Intel"),
            ("00:11:11", "Intel"),
            ("00:12:F0", "Intel"),
            ("00:13:02", "Intel"),
            ("00:13:20", "Intel"),
            ("00:13:CE", "Intel"),
            ("00:13:E8", "Intel"),
            ("00:15:00", "Intel"),
            ("00:15:17", "Intel"),
            ("00:16:6F", "Intel"),
            ("00:16:76", "Intel"),
            ("00:16:EA", "Intel"),
            ("00:16:EB", "Intel"),
            ("00:18:DE", "Intel"),
            ("00:19:D1", "Intel"),
            ("00:19:D2", "Intel"),
            ("00:1B:21", "Intel"),
            ("00:1B:77", "Intel"),
            ("00:1C:BF", "Intel"),
            ("00:1C:C0", "Intel"),
            ("00:1D:E0", "Intel"),
            ("00:1D:E1", "Intel"),
            ("00:1E:64", "Intel"),
            ("00:1E:65", "Intel"),
            ("00:1E:67", "Intel"),
            ("00:1F:3B", "Intel"),
            ("00:1F:3C", "Intel"),
            ("00:20:E0", "Intel"),
            ("00:21:5C", "Intel"),
            ("00:21:5D", "Intel"),
            ("00:21:6A", "Intel"),
            ("00:21:6B", "Intel"),
            ("00:22:FA", "Intel"),
            ("00:22:FB", "Intel"),
            ("00:23:14", "Intel"),
            ("00:23:15", "Intel"),
            ("00:24:D6", "Intel"),
            ("00:24:D7", "Intel"),
            ("00:26:C6", "Intel"),
            ("00:26:C7", "Intel"),
            ("00:27:10", "Intel"),
            ("34:02:86", "Intel"),
            ("34:13:E8", "Intel"),
            ("3C:A9:F4", "Intel"),
            ("40:25:C2", "Intel"),
            ("48:51:B7", "Intel"),
            ("4C:34:88", "Intel"),
            ("4C:80:93", "Intel"),
            ("58:91:CF", "Intel"),
            ("5C:51:4F", "Intel"),
            ("5C:C5:D4", "Intel"),
            ("60:36:DD", "Intel"),
            ("60:57:18", "Intel"),
            ("60:67:20", "Intel"),
            ("64:80:99", "Intel"),
            ("68:05:CA", "Intel"),
            ("68:17:29", "Intel"),
            ("6C:88:14", "Intel"),
            ("70:1C:E7", "Intel"),
            ("74:E5:43", "Intel"),
            ("7C:5C:F8", "Intel"),
            ("80:86:F2", "Intel"),
            ("80:9B:20", "Intel"),
            ("84:3A:4B", "Intel"),
            ("84:A6:C8", "Intel"),
            ("88:53:2E", "Intel"),
            ("8C:EC:4B", "Intel"),
            ("94:65:9C", "Intel"),
            ("98:4F:EE", "Intel"),
            ("9C:4E:36", "Intel"),
            ("A0:36:9F", "Intel"),
            ("A4:02:B9", "Intel"),
            ("A4:4C:C8", "Intel"),
            ("A4:C4:94", "Intel"),
            ("AC:72:89", "Intel"),
            ("AC:7B:A1", "Intel"),
            ("B4:6B:FC", "Intel"),
            ("B8:08:CF", "Intel"),
            ("B8:9A:2A", "Intel"),
            ("BC:77:37", "Intel"),
            ("C8:0A:A9", "Intel"),
            ("C8:1F:66", "Intel"),
            ("CC:3D:82", "Intel"),
            ("D0:7E:35", "Intel"),
            ("D4:3D:7E", "Intel"),
            ("D8:FC:93", "Intel"),
            ("DC:53:60", "Intel"),
            ("E0:94:67", "Intel"),
            ("E4:70:B8", "Intel"),
            ("E8:B1:FC", "Intel"),
            ("EC:0E:C4", "Intel"),
            ("F4:06:69", "Intel"),
            ("F8:16:54", "Intel"),
            ("F8:94:C2", "Intel"),
            ("FC:F8:AE", "Intel"),
            // IoT / Smart Home Devices
            ("C8:0F:10", "MELK/ELK-BLEDOM (BLE LED Strip)"),
            // Other common BLE LED controllers
            ("BE:FF:20", "ELK-BLEDOM (BLE LED Strip)"),
            ("BE:FF:E0", "ELK-BLEDOM (BLE LED Strip)"),
            // Tuya/Smart Life IoT
            ("D8:1F:12", "Tuya Smart"),
            ("DC:23:4D", "Tuya Smart"),
            ("84:0D:8E", "Tuya Smart"),
            ("A4:CF:12", "Tuya Smart"),
            // Govee LED
            ("A4:C1:38", "Govee (Smart LED)"),
            // Philips Hue
            ("00:17:88", "Philips Hue"),
            // LIFX
            ("D0:73:D5", "LIFX"),
            // Wyze
            ("2C:AA:8E", "Wyze"),
            ("7C:78:B2", "Wyze"),
            // Ring
            ("5C:47:5E", "Ring"),
            ("34:3E:A4", "Ring"),
            // Sonos
            ("00:0E:58", "Sonos"),
            ("5C:AA:FD", "Sonos"),
            ("78:28:CA", "Sonos"),
            ("94:9F:3E", "Sonos"),
            ("B8:E9:37", "Sonos"),
            // Nest/Google Home
            ("18:D6:C7", "Google Nest"),
            ("1C:F2:9A", "Google Nest"),
            ("54:60:09", "Google Nest"),
            ("F4:F5:D8", "Google Nest"),
            // Amazon Echo/Alexa
            ("00:FC:8B", "Amazon Echo"),
            ("0C:47:C9", "Amazon Echo"),
            ("34:D2:70", "Amazon Echo"),
            ("38:2F:E4", "Amazon Echo"),
            ("44:65:0D", "Amazon Echo"),
            ("50:DC:E7", "Amazon Echo"),
            ("68:37:E9", "Amazon Echo"),
            ("74:C2:46", "Amazon Echo"),
            ("84:D6:D0", "Amazon Echo"),
            ("A4:08:EA", "Amazon Echo"),
            ("FC:65:DE", "Amazon Echo"),
        ];

        for (oui, vendor) in entries {
            database.insert(oui.to_string(), vendor.to_string());
        }

        Self { database }
    }

    /// Look up vendor by MAC address
    pub fn lookup(&self, mac_address: &str) -> Option<&str> {
        // Normalize MAC address format
        let normalized = mac_address
            .to_uppercase()
            .replace("-", ":")
            .chars()
            .take(8)
            .collect::<String>();

        self.database.get(&normalized).map(|s| s.as_str())
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.database.len()
    }

    pub fn is_empty(&self) -> bool {
        self.database.is_empty()
    }
}

fn parse_oui_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    
    // Skip comments and empty lines
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    // IEEE format: "XX-XX-XX   (hex)		Vendor Name"
    if line.contains("(hex)") {
        let parts: Vec<&str> = line.split("(hex)").collect();
        if parts.len() >= 2 {
            let oui = parts[0].trim().replace("-", ":").to_uppercase();
            let vendor = parts[1].trim().to_string();
            if oui.len() == 8 && !vendor.is_empty() {
                return Some((oui, vendor));
            }
        }
    }

    // Wireshark/simple format: "XX:XX:XX	Vendor Name"
    if let Some(tab_pos) = line.find('\t') {
        let oui = line[..tab_pos].trim().replace("-", ":").to_uppercase();
        let vendor = line[tab_pos..].trim().to_string();
        if oui.len() == 8 && !vendor.is_empty() {
            return Some((oui, vendor));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_lookup() {
        let oui = OuiLookup::embedded();
        
        assert_eq!(oui.lookup("B8:27:EB:12:34:56"), Some("Raspberry Pi Foundation"));
        assert_eq!(oui.lookup("00:00:0C:AA:BB:CC"), Some("Cisco Systems"));
        assert_eq!(oui.lookup("00:03:93:11:22:33"), Some("Apple"));
        assert!(oui.lookup("FF:FF:FF:FF:FF:FF").is_none());
    }

    #[test]
    fn test_lookup_normalization() {
        let oui = OuiLookup::embedded();
        
        // Different formats should work
        assert_eq!(oui.lookup("b8:27:eb:12:34:56"), Some("Raspberry Pi Foundation"));
        assert_eq!(oui.lookup("B8-27-EB-12-34-56"), Some("Raspberry Pi Foundation"));
    }
}
