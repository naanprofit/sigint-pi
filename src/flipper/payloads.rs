//! Flipper Zero Payload Generation
//!
//! Templates and generators for Flipper Zero payloads:
//! - SubGHz signal files (.sub)
//! - Infrared remotes (.ir)
//! - BadUSB scripts (.txt)
//! - NFC cards (.nfc)
//! - RFID cards (.rfid)
//! - iButton keys (.ibtn)
//!
//! Based on Flipper Zero file format specifications.

use super::{PayloadType, FlipperResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// SubGHz signal protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubGhzProtocol {
    Princeton,
    Holtek,
    KeeLoq,
    StarLine,
    NiceFlo,
    NiceFlorS,
    CAME,
    CAME_TWEE,
    GateTX,
    Marantec,
    Intertechno,
    RAW,
}

/// SubGHz signal definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGhzSignal {
    pub name: String,
    pub frequency: u64,
    pub protocol: SubGhzProtocol,
    pub key: Option<String>,
    pub bit: Option<u8>,
    pub raw_data: Option<Vec<i32>>,
}

impl SubGhzSignal {
    /// Generate .sub file content
    pub fn to_sub_file(&self) -> String {
        let mut content = String::new();
        
        content.push_str("Filetype: Flipper SubGhz Key File\n");
        content.push_str("Version: 1\n");
        content.push_str(&format!("Frequency: {}\n", self.frequency));
        
        match self.protocol {
            SubGhzProtocol::RAW => {
                content.push_str("Preset: FuriHalSubGhzPresetOok650Async\n");
                content.push_str("Protocol: RAW\n");
                if let Some(raw) = &self.raw_data {
                    let raw_str: Vec<String> = raw.iter().map(|v| v.to_string()).collect();
                    content.push_str(&format!("RAW_Data: {}\n", raw_str.join(" ")));
                }
            }
            _ => {
                content.push_str("Preset: FuriHalSubGhzPresetOok650Async\n");
                content.push_str(&format!("Protocol: {:?}\n", self.protocol));
                if let Some(bit) = self.bit {
                    content.push_str(&format!("Bit: {}\n", bit));
                }
                if let Some(key) = &self.key {
                    content.push_str(&format!("Key: {}\n", key));
                }
            }
        }
        
        content
    }
}

/// Infrared signal definition  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfraredSignal {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub command: String,
}

/// Infrared remote definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfraredRemote {
    pub name: String,
    pub signals: Vec<InfraredSignal>,
}

impl InfraredRemote {
    /// Generate .ir file content
    pub fn to_ir_file(&self) -> String {
        let mut content = String::new();
        
        content.push_str("Filetype: IR signals file\n");
        content.push_str("Version: 1\n");
        content.push_str("#\n");
        
        for signal in &self.signals {
            content.push_str(&format!("name: {}\n", signal.name));
            content.push_str(&format!("type: parsed\n"));
            content.push_str(&format!("protocol: {}\n", signal.protocol));
            content.push_str(&format!("address: {}\n", signal.address));
            content.push_str(&format!("command: {}\n", signal.command));
            content.push_str("#\n");
        }
        
        content
    }
    
    /// Create a TV remote template
    pub fn tv_template(brand: &str) -> Self {
        Self {
            name: format!("{}_TV", brand),
            signals: vec![
                InfraredSignal {
                    name: "Power".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "00 00 00 00".to_string(),
                },
                InfraredSignal {
                    name: "Vol_up".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "01 00 00 00".to_string(),
                },
                InfraredSignal {
                    name: "Vol_dn".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "02 00 00 00".to_string(),
                },
                InfraredSignal {
                    name: "Ch_up".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "03 00 00 00".to_string(),
                },
                InfraredSignal {
                    name: "Ch_dn".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "04 00 00 00".to_string(),
                },
                InfraredSignal {
                    name: "Mute".to_string(),
                    protocol: "NECext".to_string(),
                    address: "00 00 00 00".to_string(),
                    command: "05 00 00 00".to_string(),
                },
            ],
        }
    }
}

/// BadUSB script definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadUsbScript {
    pub name: String,
    pub description: String,
    pub commands: Vec<BadUsbCommand>,
}

/// BadUSB commands (DuckyScript-like)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BadUsbCommand {
    Delay(u32),
    String(String),
    StringLn(String),
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    Gui(String),
    Ctrl(String),
    Alt(String),
    Shift(String),
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CapsLock,
    PrintScreen,
    Comment(String),
    // Flipper-specific
    DefaultDelay(u32),
    Id(String, String),  // VID, PID
    Release,
    Hold(String),
}

impl BadUsbScript {
    /// Generate BadUSB script content
    pub fn to_script(&self) -> String {
        let mut content = String::new();
        
        content.push_str(&format!("REM {}\n", self.description));
        content.push_str("REM Generated by SIGINT-Deck\n");
        content.push_str("\n");
        
        for cmd in &self.commands {
            let line = match cmd {
                BadUsbCommand::Delay(ms) => format!("DELAY {}", ms),
                BadUsbCommand::String(s) => format!("STRING {}", s),
                BadUsbCommand::StringLn(s) => format!("STRINGLN {}", s),
                BadUsbCommand::Enter => "ENTER".to_string(),
                BadUsbCommand::Tab => "TAB".to_string(),
                BadUsbCommand::Escape => "ESCAPE".to_string(),
                BadUsbCommand::Backspace => "BACKSPACE".to_string(),
                BadUsbCommand::Delete => "DELETE".to_string(),
                BadUsbCommand::Gui(key) => format!("GUI {}", key),
                BadUsbCommand::Ctrl(key) => format!("CTRL {}", key),
                BadUsbCommand::Alt(key) => format!("ALT {}", key),
                BadUsbCommand::Shift(key) => format!("SHIFT {}", key),
                BadUsbCommand::ArrowUp => "UP".to_string(),
                BadUsbCommand::ArrowDown => "DOWN".to_string(),
                BadUsbCommand::ArrowLeft => "LEFT".to_string(),
                BadUsbCommand::ArrowRight => "RIGHT".to_string(),
                BadUsbCommand::CapsLock => "CAPSLOCK".to_string(),
                BadUsbCommand::PrintScreen => "PRINTSCREEN".to_string(),
                BadUsbCommand::Comment(c) => format!("REM {}", c),
                BadUsbCommand::DefaultDelay(ms) => format!("DEFAULT_DELAY {}", ms),
                BadUsbCommand::Id(vid, pid) => format!("ID {}:{}", vid, pid),
                BadUsbCommand::Release => "RELEASE".to_string(),
                BadUsbCommand::Hold(key) => format!("HOLD {}", key),
            };
            content.push_str(&line);
            content.push('\n');
        }
        
        content
    }
    
    /// Create a simple "open notepad and type" script
    pub fn notepad_demo() -> Self {
        Self {
            name: "notepad_demo".to_string(),
            description: "Demo script - opens notepad and types a message".to_string(),
            commands: vec![
                BadUsbCommand::DefaultDelay(100),
                BadUsbCommand::Delay(1000),
                BadUsbCommand::Gui("r".to_string()),
                BadUsbCommand::Delay(500),
                BadUsbCommand::String("notepad".to_string()),
                BadUsbCommand::Enter,
                BadUsbCommand::Delay(1000),
                BadUsbCommand::StringLn("Hello from Flipper Zero!".to_string()),
                BadUsbCommand::StringLn("This is a demo BadUSB script.".to_string()),
            ],
        }
    }
    
    /// Create a Windows info gathering script
    pub fn windows_sysinfo() -> Self {
        Self {
            name: "sysinfo".to_string(),
            description: "Gather system information on Windows".to_string(),
            commands: vec![
                BadUsbCommand::DefaultDelay(100),
                BadUsbCommand::Delay(1000),
                BadUsbCommand::Comment("Open PowerShell".to_string()),
                BadUsbCommand::Gui("r".to_string()),
                BadUsbCommand::Delay(500),
                BadUsbCommand::String("powershell".to_string()),
                BadUsbCommand::Enter,
                BadUsbCommand::Delay(1500),
                BadUsbCommand::Comment("Gather system info".to_string()),
                BadUsbCommand::StringLn("Get-ComputerInfo | Select-Object CsName, WindowsVersion, OsArchitecture".to_string()),
                BadUsbCommand::Delay(500),
                BadUsbCommand::StringLn("Get-NetIPAddress | Where-Object AddressFamily -eq 'IPv4' | Select-Object IPAddress".to_string()),
            ],
        }
    }
}

/// NFC card types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NfcCardType {
    MifareClassic1k,
    MifareClassic4k,
    MifareUltralight,
    NTAG213,
    NTAG215,
    NTAG216,
}

/// NFC card definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfcCard {
    pub device_type: NfcCardType,
    pub uid: String,
    pub data: Vec<u8>,
}

impl NfcCard {
    /// Generate .nfc file content (simplified)
    pub fn to_nfc_file(&self) -> String {
        let mut content = String::new();
        
        content.push_str("Filetype: Flipper NFC device\n");
        content.push_str("Version: 3\n");
        
        let device_type = match self.device_type {
            NfcCardType::MifareClassic1k => "Mifare Classic",
            NfcCardType::MifareClassic4k => "Mifare Classic",
            NfcCardType::MifareUltralight => "NTAG/Ultralight",
            NfcCardType::NTAG213 | NfcCardType::NTAG215 | NfcCardType::NTAG216 => "NTAG/Ultralight",
        };
        
        content.push_str(&format!("Device type: {}\n", device_type));
        content.push_str(&format!("UID: {}\n", self.uid));
        
        content
    }
}

/// Payload generator
pub struct PayloadGenerator;

impl PayloadGenerator {
    /// Generate a payload based on type and specification
    pub fn generate(payload_type: PayloadType, spec: &str) -> FlipperResult {
        match payload_type {
            PayloadType::SubGhz => Self::generate_subghz(spec),
            PayloadType::Infrared => Self::generate_ir(spec),
            PayloadType::BadUsb => Self::generate_badusb(spec),
            PayloadType::Nfc => Self::generate_nfc(spec),
            PayloadType::Rfid => Self::generate_rfid(spec),
            PayloadType::IButton => Self::generate_ibutton(spec),
        }
    }
    
    fn generate_subghz(spec: &str) -> FlipperResult {
        // Parse spec for frequency and protocol
        let signal = SubGhzSignal {
            name: "generated".to_string(),
            frequency: 433_920_000, // Default 433.92 MHz
            protocol: SubGhzProtocol::Princeton,
            key: Some("00 00 00 00 00 01".to_string()),
            bit: Some(24),
            raw_data: None,
        };
        
        let content = signal.to_sub_file();
        
        FlipperResult::success(
            "Generated SubGHz signal",
            Some(json!({
                "content": content,
                "filename": "generated.sub",
                "frequency": signal.frequency
            }))
        )
    }
    
    fn generate_ir(spec: &str) -> FlipperResult {
        let remote = InfraredRemote::tv_template("Generic");
        let content = remote.to_ir_file();
        
        FlipperResult::success(
            "Generated IR remote",
            Some(json!({
                "content": content,
                "filename": "generated.ir",
                "signals": remote.signals.len()
            }))
        )
    }
    
    fn generate_badusb(spec: &str) -> FlipperResult {
        // Default to notepad demo
        let script = BadUsbScript::notepad_demo();
        let content = script.to_script();
        
        FlipperResult::success(
            "Generated BadUSB script",
            Some(json!({
                "content": content,
                "filename": format!("{}.txt", script.name),
                "commands": script.commands.len()
            }))
        )
    }
    
    fn generate_nfc(spec: &str) -> FlipperResult {
        FlipperResult::success(
            "NFC generation requires card data",
            Some(json!({
                "supported_types": ["MifareClassic1k", "MifareClassic4k", "NTAG213", "NTAG215", "NTAG216"]
            }))
        )
    }
    
    fn generate_rfid(spec: &str) -> FlipperResult {
        FlipperResult::success(
            "RFID generation requires card data",
            Some(json!({
                "supported_types": ["EM4100", "HIDProx", "Indala"]
            }))
        )
    }
    
    fn generate_ibutton(spec: &str) -> FlipperResult {
        FlipperResult::success(
            "iButton generation requires key data",
            Some(json!({
                "supported_types": ["DS1990", "Cyfral", "Metakom"]
            }))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_subghz_signal() {
        let signal = SubGhzSignal {
            name: "test".to_string(),
            frequency: 433_920_000,
            protocol: SubGhzProtocol::Princeton,
            key: Some("00 00 00 01".to_string()),
            bit: Some(24),
            raw_data: None,
        };
        
        let content = signal.to_sub_file();
        assert!(content.contains("Filetype: Flipper SubGhz Key File"));
        assert!(content.contains("Frequency: 433920000"));
    }
    
    #[test]
    fn test_badusb_script() {
        let script = BadUsbScript::notepad_demo();
        let content = script.to_script();
        assert!(content.contains("REM"));
        assert!(content.contains("GUI r"));
        assert!(content.contains("notepad"));
    }
    
    #[test]
    fn test_ir_remote() {
        let remote = InfraredRemote::tv_template("Samsung");
        let content = remote.to_ir_file();
        assert!(content.contains("Filetype: IR signals file"));
        assert!(content.contains("Power"));
    }
}
