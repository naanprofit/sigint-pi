//! Flipper Zero Integration Module
//!
//! Provides USB serial communication with Flipper Zero devices for:
//! - SubGHz RF transmission/reception
//! - Infrared control
//! - NFC/RFID operations  
//! - BadUSB payload deployment
//! - GPIO control
//! - File system access
//!
//! Inspired by V3SP3R (https://github.com/elder-plinius/V3SP3R)
//! Licensed under GPL-3.0
//!
//! WARNING: RF transmission capabilities require proper authorization.
//! Unauthorized transmission is illegal under FCC regulations.

pub mod serial;
pub mod commands;
pub mod payloads;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Flipper Zero device connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipperDevice {
    pub port: String,
    pub connected: bool,
    pub device_info: Option<FlipperDeviceInfo>,
}

/// Flipper Zero device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipperDeviceInfo {
    pub name: String,
    pub firmware_version: String,
    pub hardware_version: String,
    pub battery_level: u8,
    pub sd_card_present: bool,
    pub sd_card_free_mb: Option<u64>,
}

/// Flipper Zero action types (based on V3SP3R execute_command_schema)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "args")]
pub enum FlipperAction {
    // File Operations
    ListDirectory { path: String },
    ReadFile { path: String },
    WriteFile { path: String, content: String },
    CreateDirectory { path: String },
    Delete { path: String, recursive: bool },
    Move { path: String, destination_path: String },
    Rename { path: String, new_name: String },
    Copy { path: String, destination_path: String },
    
    // Device Info
    GetDeviceInfo,
    GetStorageInfo,
    
    // CLI
    ExecuteCli { command: String },
    
    // Payload Operations
    PushArtifact { 
        artifact_type: ArtifactType,
        path: String,
        content: String,
    },
    ForgePayload {
        payload_type: PayloadType,
        spec: String,
    },
    
    // RF Operations
    SubGhzTransmit { signal_file: String },
    SubGhzReceive { frequency: u64, duration_ms: u64 },
    
    // IR Operations
    IrTransmit { signal_file: String, signal_name: Option<String> },
    IrReceive { timeout_ms: u64 },
    
    // NFC/RFID
    NfcEmulate { card_file: String },
    NfcRead { timeout_ms: u64 },
    RfidEmulate { card_file: String },
    RfidRead { timeout_ms: u64 },
    IButtonEmulate { key_file: String },
    
    // BadUSB
    BadUsbExecute { script_path: String },
    
    // App Control
    LaunchApp { app_name: String },
    
    // Hardware Control
    LedControl { color: LedColor, state: bool },
    VibroControl { state: bool },
    GpioSet { pin: u8, state: bool },
    GpioRead { pin: u8 },
}

/// Artifact types for payload deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    #[serde(rename = "fap")]
    Fap,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "executable")]
    Executable,
}

/// Payload types for generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayloadType {
    #[serde(rename = "subghz")]
    SubGhz,
    #[serde(rename = "ir")]
    Infrared,
    #[serde(rename = "badusb")]
    BadUsb,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "rfid")]
    Rfid,
    #[serde(rename = "ibutton")]
    IButton,
}

/// LED colors on Flipper Zero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LedColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "backlight")]
    Backlight,
}

/// Risk level for Flipper actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,      // Read-only operations
    Medium,   // File writes, emulation
    High,     // RF transmission, BadUSB execution
    Critical, // Destructive operations
}

impl FlipperAction {
    /// Get the risk level for this action
    pub fn risk_level(&self) -> RiskLevel {
        match self {
            // Low risk - read only
            Self::ListDirectory { .. } => RiskLevel::Low,
            Self::ReadFile { .. } => RiskLevel::Low,
            Self::GetDeviceInfo => RiskLevel::Low,
            Self::GetStorageInfo => RiskLevel::Low,
            Self::GpioRead { .. } => RiskLevel::Low,
            
            // Medium risk - file operations, emulation
            Self::WriteFile { .. } => RiskLevel::Medium,
            Self::CreateDirectory { .. } => RiskLevel::Medium,
            Self::Move { .. } => RiskLevel::Medium,
            Self::Rename { .. } => RiskLevel::Medium,
            Self::Copy { .. } => RiskLevel::Medium,
            Self::PushArtifact { .. } => RiskLevel::Medium,
            Self::NfcEmulate { .. } => RiskLevel::Medium,
            Self::RfidEmulate { .. } => RiskLevel::Medium,
            Self::IButtonEmulate { .. } => RiskLevel::Medium,
            Self::LaunchApp { .. } => RiskLevel::Medium,
            Self::LedControl { .. } => RiskLevel::Low,
            Self::VibroControl { .. } => RiskLevel::Low,
            Self::GpioSet { .. } => RiskLevel::Medium,
            Self::IrReceive { .. } => RiskLevel::Low,
            Self::NfcRead { .. } => RiskLevel::Low,
            Self::RfidRead { .. } => RiskLevel::Low,
            
            // High risk - RF transmission, code execution
            Self::SubGhzTransmit { .. } => RiskLevel::High,
            Self::SubGhzReceive { .. } => RiskLevel::Medium,
            Self::IrTransmit { .. } => RiskLevel::High,
            Self::BadUsbExecute { .. } => RiskLevel::High,
            Self::ForgePayload { .. } => RiskLevel::High,
            Self::ExecuteCli { .. } => RiskLevel::High,
            
            // Critical - destructive
            Self::Delete { .. } => RiskLevel::Critical,
        }
    }
    
    /// Check if action requires user confirmation
    pub fn requires_confirmation(&self) -> bool {
        matches!(self.risk_level(), RiskLevel::High | RiskLevel::Critical)
    }
}

/// Result of a Flipper operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipperResult {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl FlipperResult {
    pub fn success(message: impl Into<String>, data: Option<serde_json::Value>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
            error: None,
        }
    }
    
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: String::new(),
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Flipper Zero file system entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlipperFileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
}

/// Standard Flipper Zero directories
pub mod paths {
    pub const EXT_ROOT: &str = "/ext";
    pub const SUBGHZ: &str = "/ext/subghz";
    pub const INFRARED: &str = "/ext/infrared";
    pub const NFC: &str = "/ext/nfc";
    pub const RFID: &str = "/ext/lfrfid";
    pub const IBUTTON: &str = "/ext/ibutton";
    pub const BADUSB: &str = "/ext/badusb";
    pub const APPS: &str = "/ext/apps";
    pub const APPS_DATA: &str = "/ext/apps_data";
}
