//! Flipper Zero Serial Communication
//!
//! Handles USB CDC serial communication with Flipper Zero devices.
//! Based on Flipper Zero CLI protocol.

use super::*;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use tracing::{debug, error, info, warn};

#[cfg(feature = "flipper")]
use serialport::{SerialPort, SerialPortType};

/// Flipper Zero serial connection manager
pub struct FlipperSerial {
    port_path: Option<String>,
    #[cfg(feature = "flipper")]
    port: Option<Box<dyn SerialPort>>,
    device_info: Option<FlipperDeviceInfo>,
}

impl FlipperSerial {
    pub fn new() -> Self {
        Self {
            port_path: None,
            #[cfg(feature = "flipper")]
            port: None,
            device_info: None,
        }
    }
    
    /// Detect connected Flipper Zero devices
    #[cfg(feature = "flipper")]
    pub fn detect_devices() -> Vec<String> {
        let mut devices = Vec::new();
        
        if let Ok(ports) = serialport::available_ports() {
            for port in ports {
                // Flipper Zero USB VID:PID
                // VID: 0x0483 (STMicroelectronics)
                // PID: 0x5740 (Virtual COM Port)
                if let SerialPortType::UsbPort(usb_info) = &port.port_type {
                    if usb_info.vid == 0x0483 && usb_info.pid == 0x5740 {
                        info!("Found Flipper Zero at {}", port.port_name);
                        devices.push(port.port_name.clone());
                    }
                }
                
                // Also check for Flipper Zero in DFU mode or custom firmware
                // Some custom firmwares use different PIDs
                if let SerialPortType::UsbPort(usb_info) = &port.port_type {
                    if usb_info.vid == 0x0483 {
                        if let Some(product) = &usb_info.product {
                            if product.to_lowercase().contains("flipper") {
                                if !devices.contains(&port.port_name) {
                                    info!("Found Flipper device at {}: {}", port.port_name, product);
                                    devices.push(port.port_name.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Also check common paths on Linux/macOS
        #[cfg(unix)]
        {
            let exact_paths = [
                "/dev/ttyACM0",
                "/dev/ttyACM1",
                "/dev/ttyUSB0",
                "/dev/cu.usbmodemflip1",
            ];
            for path in exact_paths {
                if std::path::Path::new(path).exists() && !devices.contains(&path.to_string()) {
                    devices.push(path.to_string());
                }
            }
            // Glob match for macOS Flipper serial ports
            if let Ok(entries) = std::fs::read_dir("/dev") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("cu.usbmodem") {
                        let full = format!("/dev/{}", name);
                        if !devices.contains(&full) {
                            devices.push(full);
                        }
                    }
                }
            }
        }
        
        devices
    }
    
    #[cfg(not(feature = "flipper"))]
    pub fn detect_devices() -> Vec<String> {
        warn!("Flipper support not compiled in (missing 'flipper' feature)");
        Vec::new()
    }
    
    /// Connect to a Flipper Zero device
    #[cfg(feature = "flipper")]
    pub fn connect(&mut self, port_path: &str) -> Result<(), String> {
        info!("Connecting to Flipper Zero at {}", port_path);
        
        let port = serialport::new(port_path, 230400)
            .timeout(Duration::from_secs(5))
            .open()
            .map_err(|e| format!("Failed to open serial port: {}", e))?;
        
        self.port = Some(port);
        self.port_path = Some(port_path.to_string());
        
        // Send initial command to verify connection
        self.send_command("")?;
        
        // Get device info
        self.refresh_device_info()?;
        
        info!("Connected to Flipper Zero: {:?}", self.device_info);
        Ok(())
    }
    
    #[cfg(not(feature = "flipper"))]
    pub fn connect(&mut self, _port_path: &str) -> Result<(), String> {
        Err("Flipper support not compiled in".to_string())
    }
    
    /// Disconnect from device
    pub fn disconnect(&mut self) {
        #[cfg(feature = "flipper")]
        {
            self.port = None;
        }
        self.port_path = None;
        self.device_info = None;
        info!("Disconnected from Flipper Zero");
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        #[cfg(feature = "flipper")]
        {
            self.port.is_some()
        }
        #[cfg(not(feature = "flipper"))]
        {
            false
        }
    }
    
    /// Send CLI command and get response
    #[cfg(feature = "flipper")]
    pub fn send_command(&mut self, command: &str) -> Result<String, String> {
        let port = self.port.as_mut()
            .ok_or("Not connected to Flipper Zero")?;
        
        // Send command with newline
        let cmd = format!("{}\r\n", command);
        port.write_all(cmd.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        port.flush()
            .map_err(|e| format!("Flush error: {}", e))?;
        
        // Read response
        let mut reader = BufReader::new(port.try_clone().map_err(|e| e.to_string())?);
        let mut response = String::new();
        let mut lines_read = 0;
        const MAX_LINES: usize = 1000;
        
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // Skip echo of command
                    if line.trim() == command.trim() {
                        continue;
                    }
                    // Check for prompt (end of response)
                    if line.contains(">:") || line.trim() == ">" {
                        break;
                    }
                    response.push_str(&line);
                    lines_read += 1;
                    if lines_read >= MAX_LINES {
                        warn!("Response truncated at {} lines", MAX_LINES);
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => break,
                Err(e) => return Err(format!("Read error: {}", e)),
            }
        }
        
        Ok(response.trim().to_string())
    }
    
    #[cfg(not(feature = "flipper"))]
    pub fn send_command(&mut self, _command: &str) -> Result<String, String> {
        Err("Flipper support not compiled in".to_string())
    }
    
    /// Refresh device information
    pub fn refresh_device_info(&mut self) -> Result<(), String> {
        let info_output = self.send_command("info")?;
        
        // Parse device info from CLI output
        let mut name = String::new();
        let mut firmware = String::new();
        let mut hardware = String::new();
        let mut battery: u8 = 0;
        
        for line in info_output.lines() {
            if line.contains("Name:") {
                name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("firmware_version:") || line.contains("Firmware:") {
                firmware = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.contains("hardware_version:") || line.contains("Hardware:") {
                hardware = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
        
        // Get battery level
        if let Ok(power_output) = self.send_command("power info") {
            for line in power_output.lines() {
                if line.contains("Charge:") || line.contains("charge:") {
                    if let Some(pct) = line.split(':').nth(1) {
                        battery = pct.trim().trim_end_matches('%').parse().unwrap_or(0);
                    }
                }
            }
        }
        
        // Check SD card
        let sd_present = self.send_command("storage info /ext").is_ok();
        let sd_free = if sd_present {
            self.send_command("storage info /ext")
                .ok()
                .and_then(|s| {
                    s.lines()
                        .find(|l| l.contains("free:"))
                        .and_then(|l| l.split(':').nth(1))
                        .and_then(|s| s.trim().split_whitespace().next())
                        .and_then(|s| s.parse().ok())
                })
        } else {
            None
        };
        
        self.device_info = Some(FlipperDeviceInfo {
            name,
            firmware_version: firmware,
            hardware_version: hardware,
            battery_level: battery,
            sd_card_present: sd_present,
            sd_card_free_mb: sd_free,
        });
        
        Ok(())
    }
    
    /// Get current device info
    pub fn get_device_info(&self) -> Option<&FlipperDeviceInfo> {
        self.device_info.as_ref()
    }
    
    /// Get connected port path
    pub fn get_port(&self) -> Option<&str> {
        self.port_path.as_deref()
    }
}

impl Default for FlipperSerial {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_devices() {
        // This test will pass even without a device connected
        let devices = FlipperSerial::detect_devices();
        println!("Detected Flipper devices: {:?}", devices);
    }
}
