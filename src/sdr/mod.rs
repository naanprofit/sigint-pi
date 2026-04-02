//! SDR (Software Defined Radio) Integration Module
//! 
//! Provides support for:
//! - rtl_433: ISM band device detection (433/315/868/915 MHz)
//! - Spectrum monitoring and anomaly detection
//! - Cellular tower mapping (kalibrate-rtl)
//! - Drone/UAV detection (2.4/5.8 GHz)
//! - Trunked radio monitoring (P25/DMR)
//! - TSCM counter-surveillance sweeping
//! - Environmental sensors (Geiger, air quality, CBRN)
//! - ADS-B aircraft tracking
//! - Radio reception (FM, AM, SSB)
//! - Frequency presets and station management

pub mod rtl433;
pub mod spectrum;
pub mod cellular;
pub mod drone;
pub mod drone_signatures;
pub mod trunked;
pub mod tscm;
pub mod environmental;
pub mod presets;
pub mod cots_drones;
pub mod tradecraft;
pub mod consumer_tactical_radios;
pub mod advanced_threats;
pub mod energy_weapons;
pub mod consumer_false_positives;

use serde::{Deserialize, Serialize};
use std::process::Command;

/// SDR device types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SdrDevice {
    RtlSdr,
    HackRf,
    LimeSdr,
    Unknown,
}

/// Check which SDR tools are available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdrCapabilities {
    pub rtl_sdr: bool,
    pub rtl_433: bool,
    pub rtl_power: bool,
    pub hackrf: bool,
    pub limesdr: bool,
    pub kalibrate: bool,
    pub devices: Vec<SdrDeviceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdrDeviceInfo {
    pub device_type: SdrDevice,
    pub index: u32,
    pub name: String,
    pub serial: Option<String>,
}

impl SdrCapabilities {
    pub fn detect() -> Self {
        let devices = detect_sdr_devices();

        // Hardware detection: check if actual devices are connected, not just binaries
        let has_rtl_hw = devices.iter().any(|d| matches!(d.device_type, SdrDevice::RtlSdr));
        let has_hackrf_hw = devices.iter().any(|d| matches!(d.device_type, SdrDevice::HackRf));
        let has_lime_hw = devices.iter().any(|d| matches!(d.device_type, SdrDevice::LimeSdr));

        // Binary detection: needed for tools that work with available hardware
        let has_rtl_433_bin = check_command("rtl_433", &["-h"]);
        let has_rtl_power_bin = check_command("rtl_power", &["-h"]);
        let has_kal_bin = check_command("kal", &["-h"]) || check_command("kalibrate-rtl", &["-h"]);

        // Capabilities require BOTH hardware AND binary
        Self {
            rtl_sdr: has_rtl_hw,
            rtl_433: has_rtl_hw && has_rtl_433_bin,
            rtl_power: has_rtl_hw && has_rtl_power_bin,
            hackrf: has_hackrf_hw,
            limesdr: has_lime_hw,
            kalibrate: has_rtl_hw && has_kal_bin,
            devices,
        }
    }
    
    /// Check if hackrf_sweep actually works (not just detected)
    pub fn hackrf_sweep_works(&self) -> bool {
        if !self.hackrf { return false; }
        let cmd = resolve_sdr_command("hackrf_sweep");
        match Command::new(&cmd).args(&["-f", "100:101", "-1", "-N", "1"]).output() {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }

    pub fn any_available(&self) -> bool {
        self.rtl_sdr || self.hackrf || self.limesdr
    }
}

fn check_command(cmd: &str, args: &[&str]) -> bool {
    // Try the command directly (uses PATH)
    if Command::new(cmd)
        .args(args)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .output()
        .map(|o| o.status.success() || o.status.code() == Some(1))
        .unwrap_or(false)
    {
        return true;
    }
    
    // Also check common install locations (service may not have full PATH)
    let home = std::env::var("HOME").unwrap_or_default();
    let extra_paths = [
        format!("{}/bin/{}", home, cmd),
        format!("/usr/local/bin/{}", cmd),
        format!("/usr/bin/{}", cmd),
    ];
    
    for path in &extra_paths {
        if std::path::Path::new(path).exists() {
            return true;
        }
    }
    
    false
}

/// Resolve full path for an SDR command, checking ~/bin and standard locations
pub fn resolve_sdr_command(cmd: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let candidates = [
        format!("{}/bin/{}", home, cmd),
        format!("/usr/local/bin/{}", cmd),
        format!("/usr/bin/{}", cmd),
        cmd.to_string(),
    ];
    
    for path in &candidates {
        if std::path::Path::new(path).exists() {
            return path.clone();
        }
    }
    
    cmd.to_string()
}

fn detect_sdr_devices() -> Vec<SdrDeviceInfo> {
    let mut devices = Vec::new();
    
    // Check RTL-SDR devices
    if let Ok(output) = Command::new("rtl_test").arg("-t").output() {
        let stdout = String::from_utf8_lossy(&output.stderr);
        if stdout.contains("Found") && !stdout.contains("No supported") {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::RtlSdr,
                index: 0,
                name: "RTL-SDR".to_string(),
                serial: None,
            });
        }
    }
    
    // Check HackRF
    if let Ok(output) = Command::new("hackrf_info").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Serial number") {
            let serial = stdout.lines()
                .find(|l| l.contains("Serial number"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string());
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::HackRf,
                index: 0,
                name: "HackRF One".to_string(),
                serial,
            });
        }
    }
    
    // Check LimeSDR
    if let Ok(output) = Command::new("LimeUtil").arg("--find").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("LimeSDR") {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::LimeSdr,
                index: 0,
                name: "LimeSDR".to_string(),
                serial: None,
            });
        }
    }
    
    devices
}

/// Common SDR event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SdrEvent {
    /// RF device detected by rtl_433
    RfDevice(rtl433::RfDevice),
    /// Spectrum anomaly detected
    SpectrumAnomaly(spectrum::SpectrumAnomaly),
    /// Cell tower detected
    CellTower(cellular::CellTower),
    /// Drone signal detected
    DroneSignal(drone::DroneSignal),
    /// Trunked radio traffic
    TrunkedRadio(trunked::TrunkedTraffic),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capability_detection() {
        let caps = SdrCapabilities::detect();
        println!("SDR Capabilities: {:?}", caps);
    }
}
