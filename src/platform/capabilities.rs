//! Hardware capability detection and runtime checks
//!
//! Performs startup checks for:
//! - WiFi interface existence and monitor mode support
//! - Bluetooth/BLE adapter availability
//! - GPS connectivity
//! - Power/battery status

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use tracing::{debug, error, info, warn};

/// Runtime hardware status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareStatus {
    pub wifi_ready: bool,
    pub wifi_interface: Option<String>,
    pub wifi_in_monitor_mode: bool,
    pub monitor_mode_supported: bool,
    pub is_internal_wifi: bool,
    
    pub ble_ready: bool,
    pub ble_adapter: Option<String>,
    
    pub gps_ready: bool,
    pub gps_source: Option<String>,
    pub gps_has_fix: bool,
    
    pub battery_present: bool,
    pub battery_percent: Option<u8>,
    pub on_ac_power: bool,
    
    pub low_power_mode_active: bool,
    
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl HardwareStatus {
    /// Perform all hardware capability checks
    pub fn check_all(wifi_interface: &str) -> Self {
        let mut status = Self::default();
        
        // WiFi checks
        let wifi_result = check_wifi_interface(wifi_interface);
        status.wifi_ready = wifi_result.exists && wifi_result.monitor_supported;
        status.wifi_interface = if wifi_result.exists {
            Some(wifi_interface.to_string())
        } else {
            None
        };
        status.wifi_in_monitor_mode = wifi_result.in_monitor_mode;
        status.monitor_mode_supported = wifi_result.monitor_supported;
        status.is_internal_wifi = wifi_result.is_internal;
        
        if !wifi_result.exists {
            status.errors.push(format!("WiFi interface '{}' not found", wifi_interface));
        } else if !wifi_result.monitor_supported {
            status.warnings.push(format!(
                "WiFi interface '{}' does not support monitor mode", wifi_interface
            ));
        }
        
        if wifi_result.is_internal && wifi_interface == "wlan0" {
            status.warnings.push(
                "Using internal WiFi adapter - monitor mode may not be supported. \
                 Consider using an external USB WiFi adapter.".to_string()
            );
        }
        
        // BLE checks
        let ble_result = check_bluetooth();
        status.ble_ready = ble_result.available;
        status.ble_adapter = ble_result.adapter_name;
        
        if !ble_result.available {
            if let Some(err) = ble_result.error {
                status.warnings.push(format!("Bluetooth: {}", err));
            }
        }
        
        // GPS checks
        let gps_result = check_gps();
        status.gps_ready = gps_result.available;
        status.gps_source = gps_result.source;
        status.gps_has_fix = gps_result.has_fix;
        
        if !gps_result.available {
            // GPS is optional, just log
            debug!("GPS not available: {:?}", gps_result.error);
        }
        
        // Power checks
        let power_result = check_power();
        status.battery_present = power_result.has_battery;
        status.battery_percent = power_result.percent;
        status.on_ac_power = power_result.on_ac;
        
        status
    }
    
    /// Check if the system is ready for WiFi capture
    pub fn can_capture_wifi(&self) -> bool {
        self.wifi_ready && self.monitor_mode_supported
    }
    
    /// Check if the system is ready for BLE scanning
    pub fn can_scan_ble(&self) -> bool {
        self.ble_ready
    }
    
    /// Get a summary suitable for logging
    pub fn summary(&self) -> String {
        let mut parts = vec![];
        
        parts.push(format!(
            "WiFi: {}",
            if self.wifi_ready { "ready" } else { "not ready" }
        ));
        parts.push(format!(
            "BLE: {}",
            if self.ble_ready { "ready" } else { "not ready" }
        ));
        parts.push(format!(
            "GPS: {}",
            if self.gps_ready { "ready" } else { "not ready" }
        ));
        
        if self.battery_present {
            parts.push(format!(
                "Battery: {}%{}",
                self.battery_percent.unwrap_or(0),
                if self.on_ac_power { " (charging)" } else { "" }
            ));
        }
        
        parts.join(" | ")
    }
}

/// WiFi interface check result
struct WifiCheckResult {
    exists: bool,
    monitor_supported: bool,
    in_monitor_mode: bool,
    is_internal: bool,
    current_mode: Option<String>,
}

/// Check a WiFi interface for existence and monitor mode support
fn check_wifi_interface(interface: &str) -> WifiCheckResult {
    let mut result = WifiCheckResult {
        exists: false,
        monitor_supported: false,
        in_monitor_mode: false,
        is_internal: interface == "wlan0",
        current_mode: None,
    };
    
    // Check if interface exists
    let sys_path = format!("/sys/class/net/{}", interface);
    if !Path::new(&sys_path).exists() {
        return result;
    }
    result.exists = true;
    
    // Get the phy for this interface
    let phy = get_phy_for_interface(interface);
    
    // Check current mode using iw
    if let Ok(output) = Command::new("iw")
        .args(["dev", interface, "info"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("type") {
                let mode = line.split_whitespace().last().unwrap_or("unknown");
                result.current_mode = Some(mode.to_string());
                result.in_monitor_mode = mode == "monitor";
            }
        }
    }
    
    // Check if monitor mode is supported
    if let Some(ref phy_name) = phy {
        if let Ok(output) = Command::new("iw")
            .args(["phy", phy_name, "info"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            result.monitor_supported = stdout.contains("* monitor");
        }
    }
    
    result
}

/// Get the phy name for a wireless interface
fn get_phy_for_interface(interface: &str) -> Option<String> {
    let phy_path = format!("/sys/class/net/{}/phy80211/name", interface);
    std::fs::read_to_string(&phy_path)
        .ok()
        .map(|s| s.trim().to_string())
}

/// Bluetooth check result
struct BleCheckResult {
    available: bool,
    adapter_name: Option<String>,
    error: Option<String>,
}

/// Check Bluetooth availability
fn check_bluetooth() -> BleCheckResult {
    let mut result = BleCheckResult {
        available: false,
        adapter_name: None,
        error: None,
    };
    
    // Try bluetoothctl
    if let Ok(output) = Command::new("bluetoothctl")
        .args(["show"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("Powered: yes") {
                result.available = true;
                
                // Extract controller name
                for line in stdout.lines() {
                    if line.contains("Name:") {
                        result.adapter_name = line.split(':')
                            .nth(1)
                            .map(|s| s.trim().to_string());
                        break;
                    }
                }
            } else if stdout.contains("Powered: no") {
                result.error = Some("Bluetooth adapter is powered off".to_string());
            }
        }
    }
    
    // Check for hci devices
    if !result.available && Path::new("/sys/class/bluetooth/hci0").exists() {
        result.available = true;
        result.adapter_name = Some("hci0".to_string());
    }
    
    if !result.available && result.error.is_none() {
        result.error = Some("No Bluetooth adapter found".to_string());
    }
    
    result
}

/// GPS check result
struct GpsCheckResult {
    available: bool,
    source: Option<String>,
    has_fix: bool,
    error: Option<String>,
}

/// Check GPS availability
fn check_gps() -> GpsCheckResult {
    let mut result = GpsCheckResult {
        available: false,
        source: None,
        has_fix: false,
        error: None,
    };
    
    // Check for gpsd service
    if let Ok(output) = Command::new("systemctl")
        .args(["is-active", "gpsd"])
        .output()
    {
        if output.status.success() {
            result.available = true;
            result.source = Some("gpsd".to_string());
            
            // Try to check for fix using gpspipe
            if let Ok(output) = Command::new("timeout")
                .args(["2", "gpspipe", "-w", "-n", "1"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("\"mode\":3") || stdout.contains("\"mode\":2") {
                    result.has_fix = true;
                }
            }
            
            return result;
        }
    }
    
    // Check for USB GPS devices
    for dev in &["/dev/ttyUSB0", "/dev/ttyACM0", "/dev/ttyUSB1"] {
        if Path::new(dev).exists() {
            result.available = true;
            result.source = Some(dev.to_string());
            return result;
        }
    }
    
    result.error = Some("No GPS device found".to_string());
    result
}

/// Power check result
struct PowerCheckResult {
    has_battery: bool,
    percent: Option<u8>,
    on_ac: bool,
}

/// Check power/battery status
fn check_power() -> PowerCheckResult {
    let mut result = PowerCheckResult {
        has_battery: false,
        percent: None,
        on_ac: true, // Default to AC if unknown
    };
    
    // Check for battery
    for bat in &["BAT0", "BAT1"] {
        let path = format!("/sys/class/power_supply/{}", bat);
        if Path::new(&path).exists() {
            result.has_battery = true;
            
            // Read capacity
            let cap_path = format!("{}/capacity", path);
            if let Ok(cap) = std::fs::read_to_string(&cap_path) {
                result.percent = cap.trim().parse().ok();
            }
            
            // Read status
            let status_path = format!("{}/status", path);
            if let Ok(status) = std::fs::read_to_string(&status_path) {
                result.on_ac = status.trim() == "Charging" || status.trim() == "Full";
            }
            
            break;
        }
    }
    
    // Also check AC adapter directly
    for ac in &["AC", "AC0", "ADP0", "ADP1"] {
        let path = format!("/sys/class/power_supply/{}/online", ac);
        if let Ok(online) = std::fs::read_to_string(&path) {
            result.on_ac = online.trim() == "1";
            break;
        }
    }
    
    result
}

/// Enable monitor mode on an interface
pub fn enable_monitor_mode(interface: &str) -> Result<(), String> {
    info!("Enabling monitor mode on {}", interface);
    
    // Check if already in monitor mode
    let check = check_wifi_interface(interface);
    if check.in_monitor_mode {
        info!("{} is already in monitor mode", interface);
        return Ok(());
    }
    
    if !check.monitor_supported {
        return Err(format!(
            "Interface {} does not support monitor mode", interface
        ));
    }
    
    // Bring interface down
    let status = Command::new("ip")
        .args(["link", "set", interface, "down"])
        .status()
        .map_err(|e| format!("Failed to bring down {}: {}", interface, e))?;
    
    if !status.success() {
        return Err(format!("Failed to bring down {} (may need root)", interface));
    }
    
    // Set monitor mode
    let status = Command::new("iw")
        .args(["dev", interface, "set", "type", "monitor"])
        .status()
        .map_err(|e| format!("Failed to set monitor mode: {}", e))?;
    
    if !status.success() {
        // Try to bring interface back up
        let _ = Command::new("ip")
            .args(["link", "set", interface, "up"])
            .status();
        return Err("Failed to set monitor mode (may need root or interface doesn't support it)".to_string());
    }
    
    // Bring interface up
    let status = Command::new("ip")
        .args(["link", "set", interface, "up"])
        .status()
        .map_err(|e| format!("Failed to bring up {}: {}", interface, e))?;
    
    if !status.success() {
        return Err(format!("Failed to bring up {} in monitor mode", interface));
    }
    
    info!("Monitor mode enabled on {}", interface);
    Ok(())
}

/// Disable monitor mode and restore managed mode
pub fn disable_monitor_mode(interface: &str) -> Result<(), String> {
    info!("Restoring managed mode on {}", interface);
    
    let status = Command::new("ip")
        .args(["link", "set", interface, "down"])
        .status()
        .map_err(|e| format!("Failed to bring down {}: {}", interface, e))?;
    
    if !status.success() {
        return Err(format!("Failed to bring down {}", interface));
    }
    
    let status = Command::new("iw")
        .args(["dev", interface, "set", "type", "managed"])
        .status()
        .map_err(|e| format!("Failed to set managed mode: {}", e))?;
    
    if !status.success() {
        return Err("Failed to set managed mode".to_string());
    }
    
    let status = Command::new("ip")
        .args(["link", "set", interface, "up"])
        .status()
        .map_err(|e| format!("Failed to bring up {}: {}", interface, e))?;
    
    if !status.success() {
        return Err(format!("Failed to bring up {}", interface));
    }
    
    info!("Managed mode restored on {}", interface);
    Ok(())
}

/// List all wireless interfaces and their capabilities
pub fn list_wireless_interfaces() -> Vec<WirelessInterfaceInfo> {
    let mut interfaces = vec![];
    
    if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let wireless_path = format!("/sys/class/net/{}/wireless", name);
            
            if Path::new(&wireless_path).exists() {
                let check = check_wifi_interface(&name);
                
                interfaces.push(WirelessInterfaceInfo {
                    name: name.clone(),
                    phy: get_phy_for_interface(&name),
                    supports_monitor: check.monitor_supported,
                    current_mode: check.current_mode.unwrap_or_else(|| "unknown".to_string()),
                    is_internal: check.is_internal,
                });
            }
        }
    }
    
    interfaces
}

/// Information about a wireless interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessInterfaceInfo {
    pub name: String,
    pub phy: Option<String>,
    pub supports_monitor: bool,
    pub current_mode: String,
    pub is_internal: bool,
}
