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
    Airspy,
    AirspyMini,
    AirspyHfPlus,
    SdrPlay,
    KrakenSdr,    // 5x coherent RTL-SDR for direction finding
    KerberosSdr,  // 4x coherent RTL-SDR for direction finding
    PlutoSdr,     // ADALM-PLUTO
    Unknown,
}

impl SdrDevice {
    pub fn label(&self) -> &str {
        match self {
            Self::RtlSdr => "RTL-SDR",
            Self::HackRf => "HackRF One",
            Self::LimeSdr => "LimeSDR",
            Self::Airspy => "Airspy R2",
            Self::AirspyMini => "Airspy Mini",
            Self::AirspyHfPlus => "Airspy HF+ Discovery",
            Self::SdrPlay => "SDRplay RSP",
            Self::KrakenSdr => "KrakenSDR (5ch coherent)",
            Self::KerberosSdr => "KerberosSDR (4ch coherent)",
            Self::PlutoSdr => "ADALM-PLUTO",
            Self::Unknown => "Unknown SDR",
        }
    }

    pub fn supports_tx(&self) -> bool {
        matches!(self, Self::HackRf | Self::LimeSdr | Self::PlutoSdr)
    }

    pub fn supports_direction_finding(&self) -> bool {
        matches!(self, Self::KrakenSdr | Self::KerberosSdr)
    }

    pub fn channel_count(&self) -> u32 {
        match self {
            Self::KrakenSdr => 5,
            Self::KerberosSdr => 4,
            _ => 1,
        }
    }

    pub fn approx_price_usd(&self) -> u32 {
        match self {
            Self::RtlSdr => 30,
            Self::HackRf => 350,
            Self::LimeSdr => 300,
            Self::Airspy => 170,
            Self::AirspyMini => 100,
            Self::AirspyHfPlus => 170,
            Self::SdrPlay => 110,
            Self::KrakenSdr => 150,
            Self::KerberosSdr => 120,
            Self::PlutoSdr => 150,
            Self::Unknown => 0,
        }
    }
}

/// Antenna position for array direction finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntennaPosition {
    pub device_index: u32,
    pub device_serial: Option<String>,
    pub label: String,
    pub x_meters: f64,
    pub y_meters: f64,
    pub z_meters: f64,
    pub bearing_degrees: f64,
    pub antenna_type: String,
}

/// SDR array configuration for multi-device setups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdrArrayConfig {
    pub name: String,
    pub antennas: Vec<AntennaPosition>,
    pub center_freq_hz: u64,
    pub sample_rate: u32,
    pub coherent: bool,
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
    
    // Detect ALL RTL-SDR devices (rtl_test -t lists each with an index)
    let rtl_test_cmd = resolve_sdr_command("rtl_test");
    if let Ok(output) = Command::new(&rtl_test_cmd).arg("-t").output() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Parse lines like "  0:  Realtek, RTL2838UHIDIR, SN: 00000001"
        let mut rtl_count = 0;
        for line in stderr.lines() {
            if let Some(idx_str) = line.trim().strip_suffix(":") {
                // skip
            }
            // Match "  N:  Vendor, Product, SN: serial"
            let trimmed = line.trim();
            if trimmed.len() > 2 && trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) && trimmed.contains(':') {
                let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let idx = parts[0].trim().parse::<u32>().unwrap_or(rtl_count);
                    let detail = parts[1].trim();
                    let serial = detail.split("SN:").nth(1).map(|s| s.trim().to_string());
                    let name = detail.split(',').next().unwrap_or("RTL-SDR").trim();

                    // Check if this might be a KrakenSDR/KerberosSDR (multiple RTL-SDRs with matching serials)
                    devices.push(SdrDeviceInfo {
                        device_type: SdrDevice::RtlSdr,
                        index: idx,
                        name: format!("RTL-SDR #{} ({})", idx, name),
                        serial,
                    });
                    rtl_count += 1;
                }
            }
        }

        // If we found 5 RTL-SDRs, likely a KrakenSDR
        if rtl_count == 5 {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::KrakenSdr,
                index: 0,
                name: "KrakenSDR (5-channel coherent)".to_string(),
                serial: None,
            });
        } else if rtl_count == 4 {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::KerberosSdr,
                index: 0,
                name: "KerberosSDR (4-channel coherent)".to_string(),
                serial: None,
            });
        }

        // Fallback: if "Found N device(s)" but no parseable lines
        if rtl_count == 0 && stderr.contains("Found") && !stderr.contains("No supported") {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::RtlSdr,
                index: 0,
                name: "RTL-SDR".to_string(),
                serial: None,
            });
        }
    }
    
    // Detect ALL HackRF devices
    let hackrf_cmd = resolve_sdr_command("hackrf_info");
    if let Ok(output) = Command::new(&hackrf_cmd).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut hackrf_idx = 0;
        let mut current_serial: Option<String> = None;
        for line in stdout.lines() {
            if line.contains("Serial number") {
                current_serial = line.split(':').nth(1).map(|s| s.trim().to_string());
            }
            if line.contains("Board ID") || (line.contains("Serial number") && current_serial.is_some()) {
                // Each "Serial number" block = one HackRF
                if line.contains("Board ID") {
                    let name = if line.contains("Jawbreaker") { "HackRF Jawbreaker" }
                        else if line.contains("rad1o") { "rad1o" }
                        else { "HackRF One" };
                    devices.push(SdrDeviceInfo {
                        device_type: SdrDevice::HackRf,
                        index: hackrf_idx,
                        name: format!("{} #{}", name, hackrf_idx),
                        serial: current_serial.take(),
                    });
                    hackrf_idx += 1;
                }
            }
        }
        // Fallback for single HackRF with simpler output
        if hackrf_idx == 0 && stdout.contains("Serial number") {
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
    
    // Detect Airspy devices
    if let Ok(output) = Command::new("airspy_info").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("S/N") || stdout.contains("Board ID") {
            let serial = stdout.lines().find(|l| l.contains("S/N"))
                .and_then(|l| l.split(':').nth(1)).map(|s| s.trim().to_string());
            let dev_type = if stdout.contains("Mini") { SdrDevice::AirspyMini }
                else { SdrDevice::Airspy };
            let dev_name = dev_type.label().to_string();
            devices.push(SdrDeviceInfo {
                device_type: dev_type,
                index: 0,
                name: dev_name,
                serial,
            });
        }
    }

    // Detect Airspy HF+ Discovery
    if let Ok(output) = Command::new("airspyhf_info").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("S/N") {
            let serial = stdout.lines().find(|l| l.contains("S/N"))
                .and_then(|l| l.split(':').nth(1)).map(|s| s.trim().to_string());
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::AirspyHfPlus,
                index: 0,
                name: "Airspy HF+ Discovery".to_string(),
                serial,
            });
        }
    }

    // Detect SDRplay devices
    if let Ok(output) = Command::new("sdrplay_apiService").arg("--version").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::SdrPlay,
                index: 0,
                name: "SDRplay RSP".to_string(),
                serial: None,
            });
        }
    }

    // Detect ADALM-PLUTO
    if let Ok(output) = Command::new("iio_info").arg("-s").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("PlutoSDR") || stdout.contains("ADALM-PLUTO") {
            devices.push(SdrDeviceInfo {
                device_type: SdrDevice::PlutoSdr,
                index: 0,
                name: "ADALM-PLUTO".to_string(),
                serial: None,
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
