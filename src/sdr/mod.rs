//! SDR (Software Defined Radio) Integration Module
//! 
//! Provides support for:
//! - rtl_433: ISM band device detection (433/315/868/915 MHz)
//! - Spectrum monitoring and anomaly detection
//! - Cellular tower mapping (kalibrate-rtl)
//! - Drone/UAV detection (2.4/5.8 GHz)
//! - Trunked radio monitoring (P25/DMR)

pub mod rtl433;
pub mod spectrum;
pub mod cellular;
pub mod drone;
pub mod trunked;

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
        let rtl_sdr = check_command("rtl_test", &["-h"]);
        let rtl_433 = check_command("rtl_433", &["-h"]);
        let rtl_power = check_command("rtl_power", &["-h"]);
        let hackrf = check_command("hackrf_info", &[]);
        let limesdr = check_command("LimeUtil", &["--help"]);
        let kalibrate = check_command("kal", &["-h"]) || check_command("kalibrate-rtl", &["-h"]);
        
        let devices = detect_sdr_devices();
        
        Self {
            rtl_sdr,
            rtl_433,
            rtl_power,
            hackrf,
            limesdr,
            kalibrate,
            devices,
        }
    }
    
    pub fn any_available(&self) -> bool {
        self.rtl_sdr || self.hackrf || self.limesdr
    }
}

fn check_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| o.status.success() || o.status.code() == Some(1)) // -h often returns 1
        .unwrap_or(false)
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
