//! Platform detection and configuration
//! 
//! Supports runtime detection of:
//! - Raspberry Pi (all models)
//! - Steam Deck (LCD and OLED)
//! - Generic Linux
//! - macOS (simulation only)

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tracing::{info, warn};

/// Detected platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    RaspberryPi,
    SteamDeck,
    GenericLinux,
    MacOS,
    Unknown,
}

/// Platform capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCapabilities {
    pub platform: Platform,
    pub has_wifi: bool,
    pub wifi_supports_monitor: bool,
    pub has_bluetooth: bool,
    pub has_gps: bool,
    pub is_portable: bool,
    pub has_battery: bool,
    pub recommended_scan_interval_ms: u64,
    pub internal_wifi_interface: Option<String>,
    pub external_wifi_interface: Option<String>,
}

/// Platform-specific defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDefaults {
    pub wifi_interface: String,
    pub scan_interval_ms: u64,
    pub low_power_scan_interval_ms: u64,
    pub web_port: u16,
    pub enable_attack_detection: bool,
    pub enable_pcap: bool,
}

impl Platform {
    /// Detect the current platform at runtime
    pub fn detect() -> Self {
        // Check environment variable override first
        if let Ok(platform) = std::env::var("SIGINT_PLATFORM") {
            match platform.to_lowercase().as_str() {
                "steamdeck" | "steam_deck" | "deck" => return Platform::SteamDeck,
                "raspberrypi" | "raspberry_pi" | "pi" | "rpi" => return Platform::RaspberryPi,
                "linux" => return Platform::GenericLinux,
                "macos" | "darwin" => return Platform::MacOS,
                _ => {}
            }
        }

        // macOS detection
        if cfg!(target_os = "macos") {
            return Platform::MacOS;
        }

        // Steam Deck detection
        if Self::is_steam_deck() {
            return Platform::SteamDeck;
        }

        // Raspberry Pi detection
        if Self::is_raspberry_pi() {
            return Platform::RaspberryPi;
        }

        // Default to generic Linux
        if cfg!(target_os = "linux") {
            Platform::GenericLinux
        } else {
            Platform::Unknown
        }
    }

    /// Check if running on Steam Deck
    fn is_steam_deck() -> bool {
        // Check /etc/os-release for SteamOS
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            if content.contains("VARIANT_ID=steamdeck") || content.contains("ID=steamos") {
                return true;
            }
        }

        // Check for Steam Deck specific hardware
        if Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists() {
            if let Ok(vendor) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor") {
                if vendor.trim().to_lowercase().contains("valve") {
                    return true;
                }
            }
        }

        // Check for jupiter (Steam Deck codename)
        if let Ok(name) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
            if name.trim().to_lowercase().contains("jupiter") {
                return true;
            }
        }

        false
    }

    /// Check if running on Raspberry Pi
    fn is_raspberry_pi() -> bool {
        // Check /proc/cpuinfo
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            if content.contains("Raspberry Pi") || content.contains("BCM") {
                return true;
            }
        }

        // Check /proc/device-tree/model
        if let Ok(model) = std::fs::read_to_string("/proc/device-tree/model") {
            if model.to_lowercase().contains("raspberry") {
                return true;
            }
        }

        false
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Platform::RaspberryPi => "Raspberry Pi",
            Platform::SteamDeck => "Steam Deck",
            Platform::GenericLinux => "Linux",
            Platform::MacOS => "macOS",
            Platform::Unknown => "Unknown",
        }
    }
}

impl PlatformCapabilities {
    /// Detect capabilities for the current platform
    pub fn detect() -> Self {
        let platform = Platform::detect();
        
        match platform {
            Platform::SteamDeck => Self::steam_deck_capabilities(),
            Platform::RaspberryPi => Self::raspberry_pi_capabilities(),
            Platform::MacOS => Self::macos_capabilities(),
            _ => Self::generic_linux_capabilities(),
        }
    }

    fn steam_deck_capabilities() -> Self {
        // Steam Deck internal WiFi does NOT support monitor mode
        // Must use external USB adapter
        let external_wifi = Self::find_external_wifi_interface();
        
        Self {
            platform: Platform::SteamDeck,
            has_wifi: true,
            wifi_supports_monitor: external_wifi.is_some(), // Only if external adapter found
            has_bluetooth: true,
            has_gps: Self::check_gps_available(),
            is_portable: true,
            has_battery: true,
            recommended_scan_interval_ms: 10000, // Battery-conscious default
            internal_wifi_interface: Some("wlan0".to_string()),
            external_wifi_interface: external_wifi,
        }
    }

    fn raspberry_pi_capabilities() -> Self {
        Self {
            platform: Platform::RaspberryPi,
            has_wifi: true,
            wifi_supports_monitor: true, // With external adapter
            has_bluetooth: true,
            has_gps: Self::check_gps_available(),
            is_portable: true,
            has_battery: Self::check_battery_available(),
            recommended_scan_interval_ms: 5000,
            internal_wifi_interface: Some("wlan0".to_string()),
            external_wifi_interface: Self::find_external_wifi_interface(),
        }
    }

    fn macos_capabilities() -> Self {
        Self {
            platform: Platform::MacOS,
            has_wifi: false, // No monitor mode on macOS
            wifi_supports_monitor: false,
            has_bluetooth: true, // CoreBluetooth works natively (not in Docker)
            has_gps: false,
            is_portable: true,
            has_battery: true,
            recommended_scan_interval_ms: 5000,
            internal_wifi_interface: None,
            external_wifi_interface: None,
        }
    }

    fn generic_linux_capabilities() -> Self {
        Self {
            platform: Platform::GenericLinux,
            has_wifi: true,
            wifi_supports_monitor: true,
            has_bluetooth: true,
            has_gps: Self::check_gps_available(),
            is_portable: false,
            has_battery: Self::check_battery_available(),
            recommended_scan_interval_ms: 5000,
            internal_wifi_interface: None,
            external_wifi_interface: Self::find_external_wifi_interface(),
        }
    }

    /// Find an external WiFi interface (not wlan0 which is typically internal)
    fn find_external_wifi_interface() -> Option<String> {
        // Look for wlan1, wlan2, etc.
        for i in 1..10 {
            let iface = format!("wlan{}", i);
            let path = format!("/sys/class/net/{}", iface);
            if Path::new(&path).exists() {
                return Some(iface);
            }
        }

        // Also check for common USB adapter names
        for name in &["wlx", "wlp"] {
            if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let fname = entry.file_name().to_string_lossy().to_string();
                    if fname.starts_with(name) {
                        return Some(fname);
                    }
                }
            }
        }

        None
    }

    /// Check if GPS is available (gpsd or USB serial)
    fn check_gps_available() -> bool {
        // Check for gpsd
        if let Ok(output) = Command::new("systemctl")
            .args(["is-active", "gpsd"])
            .output()
        {
            if output.status.success() {
                return true;
            }
        }

        // Check for USB GPS device
        if Path::new("/dev/ttyUSB0").exists() || Path::new("/dev/ttyACM0").exists() {
            return true;
        }

        false
    }

    /// Check if running on battery
    fn check_battery_available() -> bool {
        Path::new("/sys/class/power_supply/BAT0").exists()
            || Path::new("/sys/class/power_supply/BAT1").exists()
    }
}

impl PlatformDefaults {
    /// Get defaults for the detected platform
    pub fn for_platform(platform: Platform) -> Self {
        match platform {
            Platform::SteamDeck => Self {
                wifi_interface: "wlan1".to_string(), // External adapter
                scan_interval_ms: 10000,
                low_power_scan_interval_ms: 30000,
                web_port: 8080,
                enable_attack_detection: true,
                enable_pcap: false, // Save disk space
            },
            Platform::RaspberryPi => Self {
                wifi_interface: "wlan1".to_string(), // External adapter
                scan_interval_ms: 5000,
                low_power_scan_interval_ms: 15000,
                web_port: 8080,
                enable_attack_detection: true,
                enable_pcap: true,
            },
            Platform::MacOS => Self {
                wifi_interface: "en0".to_string(),
                scan_interval_ms: 5000,
                low_power_scan_interval_ms: 15000,
                web_port: 8080,
                enable_attack_detection: false,
                enable_pcap: false,
            },
            _ => Self {
                wifi_interface: "wlan0".to_string(),
                scan_interval_ms: 5000,
                low_power_scan_interval_ms: 15000,
                web_port: 8080,
                enable_attack_detection: true,
                enable_pcap: true,
            },
        }
    }
}

/// Log platform detection results
pub fn log_platform_info() {
    let platform = Platform::detect();
    let caps = PlatformCapabilities::detect();
    
    info!("Platform detected: {}", platform.name());
    info!("  WiFi available: {}", caps.has_wifi);
    info!("  Monitor mode: {}", caps.wifi_supports_monitor);
    info!("  Bluetooth: {}", caps.has_bluetooth);
    info!("  GPS: {}", caps.has_gps);
    info!("  Battery: {}", caps.has_battery);
    
    if let Some(ref iface) = caps.external_wifi_interface {
        info!("  External WiFi: {}", iface);
    }
    
    // Platform-specific warnings
    match platform {
        Platform::SteamDeck => {
            warn!("Steam Deck internal WiFi (wlan0) does NOT support monitor mode!");
            warn!("An external USB WiFi adapter is REQUIRED for WiFi capture.");
            if caps.external_wifi_interface.is_none() {
                warn!("No external WiFi adapter detected. WiFi capture will be disabled.");
            }
        }
        Platform::MacOS => {
            warn!("macOS does not support WiFi monitor mode.");
            warn!("Running in simulation mode or BLE-only mode.");
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = Platform::detect();
        // Just ensure it doesn't panic
        assert!(matches!(
            platform,
            Platform::RaspberryPi | Platform::SteamDeck | Platform::GenericLinux | Platform::MacOS | Platform::Unknown
        ));
    }

    #[test]
    fn test_platform_defaults() {
        let defaults = PlatformDefaults::for_platform(Platform::SteamDeck);
        assert_eq!(defaults.wifi_interface, "wlan1");
        assert_eq!(defaults.scan_interval_ms, 10000);
    }
}
