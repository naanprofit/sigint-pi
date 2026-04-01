//! Flipper Zero Command Execution
//!
//! Implements all Flipper Zero actions from the V3SP3R command schema.
//! Each action is validated for risk level before execution.

use super::*;
use super::serial::FlipperSerial;
use serde_json::json;
use tracing::{debug, error, info, warn};

/// Flipper Zero command executor
pub struct FlipperExecutor {
    serial: FlipperSerial,
    auto_approve_level: RiskLevel,
}

impl FlipperExecutor {
    pub fn new() -> Self {
        Self {
            serial: FlipperSerial::new(),
            auto_approve_level: RiskLevel::Medium, // Default: auto-approve up to medium risk
        }
    }
    
    /// Set the maximum risk level for auto-approval
    pub fn set_auto_approve_level(&mut self, level: RiskLevel) {
        self.auto_approve_level = level;
    }
    
    /// Connect to Flipper Zero
    pub fn connect(&mut self, port: &str) -> Result<(), String> {
        self.serial.connect(port)
    }
    
    /// Disconnect from Flipper Zero
    pub fn disconnect(&mut self) {
        self.serial.disconnect()
    }
    
    /// Check connection status
    pub fn is_connected(&self) -> bool {
        self.serial.is_connected()
    }
    
    /// Get device info
    pub fn get_device(&self) -> Option<FlipperDevice> {
        if self.is_connected() {
            Some(FlipperDevice {
                port: self.serial.get_port().unwrap_or("").to_string(),
                connected: true,
                device_info: self.serial.get_device_info().cloned(),
            })
        } else {
            None
        }
    }
    
    /// Execute a Flipper action
    pub fn execute(&mut self, action: FlipperAction, force: bool) -> FlipperResult {
        // Check risk level
        let risk = action.risk_level();
        if !force && risk > self.auto_approve_level {
            return FlipperResult::error(format!(
                "Action requires approval (risk: {:?}). Use force=true to override.",
                risk
            ));
        }
        
        // Ensure connected
        if !self.is_connected() {
            return FlipperResult::error("Not connected to Flipper Zero");
        }
        
        // Execute action
        match action {
            FlipperAction::ListDirectory { path } => self.list_directory(&path),
            FlipperAction::ReadFile { path } => self.read_file(&path),
            FlipperAction::WriteFile { path, content } => self.write_file(&path, &content),
            FlipperAction::CreateDirectory { path } => self.create_directory(&path),
            FlipperAction::Delete { path, recursive } => self.delete(&path, recursive),
            FlipperAction::Move { path, destination_path } => self.move_file(&path, &destination_path),
            FlipperAction::Rename { path, new_name } => self.rename(&path, &new_name),
            FlipperAction::Copy { path, destination_path } => self.copy_file(&path, &destination_path),
            FlipperAction::GetDeviceInfo => self.get_device_info(),
            FlipperAction::GetStorageInfo => self.get_storage_info(),
            FlipperAction::ExecuteCli { command } => self.execute_cli(&command),
            FlipperAction::SubGhzTransmit { signal_file } => self.subghz_transmit(&signal_file),
            FlipperAction::SubGhzReceive { frequency, duration_ms } => self.subghz_receive(frequency, duration_ms),
            FlipperAction::IrTransmit { signal_file, signal_name } => self.ir_transmit(&signal_file, signal_name.as_deref()),
            FlipperAction::IrReceive { timeout_ms } => self.ir_receive(timeout_ms),
            FlipperAction::NfcEmulate { card_file } => self.nfc_emulate(&card_file),
            FlipperAction::NfcRead { timeout_ms } => self.nfc_read(timeout_ms),
            FlipperAction::RfidEmulate { card_file } => self.rfid_emulate(&card_file),
            FlipperAction::RfidRead { timeout_ms } => self.rfid_read(timeout_ms),
            FlipperAction::IButtonEmulate { key_file } => self.ibutton_emulate(&key_file),
            FlipperAction::BadUsbExecute { script_path } => self.badusb_execute(&script_path),
            FlipperAction::LaunchApp { app_name } => self.launch_app(&app_name),
            FlipperAction::LedControl { color, state } => self.led_control(color, state),
            FlipperAction::VibroControl { state } => self.vibro_control(state),
            FlipperAction::GpioSet { pin, state } => self.gpio_set(pin, state),
            FlipperAction::GpioRead { pin } => self.gpio_read(pin),
            FlipperAction::PushArtifact { artifact_type, path, content } => {
                self.push_artifact(artifact_type, &path, &content)
            }
            FlipperAction::ForgePayload { payload_type, spec } => {
                self.forge_payload(payload_type, &spec)
            }
        }
    }
    
    // ========================================================================
    // File Operations
    // ========================================================================
    
    fn list_directory(&mut self, path: &str) -> FlipperResult {
        let output = match self.serial.send_command(&format!("storage list {}", path)) {
            Ok(o) => o,
            Err(e) => return FlipperResult::error(e),
        };
        
        let mut entries = Vec::new();
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("Storage") {
                continue;
            }
            
            // Parse: [D] dirname or [F] filename size
            let is_dir = line.starts_with("[D]");
            let is_file = line.starts_with("[F]");
            
            if is_dir || is_file {
                let parts: Vec<&str> = line[4..].split_whitespace().collect();
                if let Some(name) = parts.first() {
                    let size = if is_file && parts.len() > 1 {
                        parts[1].parse().ok()
                    } else {
                        None
                    };
                    
                    entries.push(FlipperFileEntry {
                        name: name.to_string(),
                        path: format!("{}/{}", path.trim_end_matches('/'), name),
                        is_directory: is_dir,
                        size,
                    });
                }
            }
        }
        
        FlipperResult::success(
            format!("Listed {} entries in {}", entries.len(), path),
            Some(json!({ "entries": entries }))
        )
    }
    
    fn read_file(&mut self, path: &str) -> FlipperResult {
        let output = match self.serial.send_command(&format!("storage read {}", path)) {
            Ok(o) => o,
            Err(e) => return FlipperResult::error(e),
        };
        
        FlipperResult::success(
            format!("Read {} bytes from {}", output.len(), path),
            Some(json!({ "content": output, "path": path }))
        )
    }
    
    fn write_file(&mut self, path: &str, content: &str) -> FlipperResult {
        // Write using storage write command
        // For binary or large files, this may need chunking
        let cmd = format!("storage write {}", path);
        
        if let Err(e) = self.serial.send_command(&cmd) {
            return FlipperResult::error(e);
        }
        
        // Send content
        if let Err(e) = self.serial.send_command(content) {
            return FlipperResult::error(e);
        }
        
        // Send Ctrl+C to end write
        if let Err(e) = self.serial.send_command("\x03") {
            return FlipperResult::error(e);
        }
        
        FlipperResult::success(
            format!("Wrote {} bytes to {}", content.len(), path),
            Some(json!({ "path": path, "size": content.len() }))
        )
    }
    
    fn create_directory(&mut self, path: &str) -> FlipperResult {
        match self.serial.send_command(&format!("storage mkdir {}", path)) {
            Ok(_) => FlipperResult::success(
                format!("Created directory {}", path),
                Some(json!({ "path": path }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn delete(&mut self, path: &str, recursive: bool) -> FlipperResult {
        let cmd = if recursive {
            format!("storage remove {}", path)
        } else {
            format!("storage remove {}", path)
        };
        
        match self.serial.send_command(&cmd) {
            Ok(_) => FlipperResult::success(
                format!("Deleted {}", path),
                Some(json!({ "path": path }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn move_file(&mut self, source: &str, dest: &str) -> FlipperResult {
        match self.serial.send_command(&format!("storage move {} {}", source, dest)) {
            Ok(_) => FlipperResult::success(
                format!("Moved {} to {}", source, dest),
                Some(json!({ "source": source, "destination": dest }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn rename(&mut self, path: &str, new_name: &str) -> FlipperResult {
        let parent = path.rsplit_once('/').map(|(p, _)| p).unwrap_or("");
        let new_path = format!("{}/{}", parent, new_name);
        self.move_file(path, &new_path)
    }
    
    fn copy_file(&mut self, source: &str, dest: &str) -> FlipperResult {
        match self.serial.send_command(&format!("storage copy {} {}", source, dest)) {
            Ok(_) => FlipperResult::success(
                format!("Copied {} to {}", source, dest),
                Some(json!({ "source": source, "destination": dest }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // Device Info
    // ========================================================================
    
    fn get_device_info(&mut self) -> FlipperResult {
        if let Err(e) = self.serial.refresh_device_info() {
            return FlipperResult::error(e);
        }
        
        if let Some(info) = self.serial.get_device_info() {
            FlipperResult::success(
                format!("Device: {}", info.name),
                Some(json!({
                    "name": info.name,
                    "firmware": info.firmware_version,
                    "hardware": info.hardware_version,
                    "battery": info.battery_level,
                    "sd_card": info.sd_card_present,
                    "sd_free_mb": info.sd_card_free_mb
                }))
            )
        } else {
            FlipperResult::error("Failed to get device info")
        }
    }
    
    fn get_storage_info(&mut self) -> FlipperResult {
        let output = match self.serial.send_command("storage info /ext") {
            Ok(o) => o,
            Err(e) => return FlipperResult::error(e),
        };
        
        FlipperResult::success("Storage info retrieved", Some(json!({ "info": output })))
    }
    
    // ========================================================================
    // CLI Execution
    // ========================================================================
    
    fn execute_cli(&mut self, command: &str) -> FlipperResult {
        // Sanitize command - block dangerous operations
        let blocked = ["factory_reset", "dfu", "update"];
        for b in blocked {
            if command.to_lowercase().contains(b) {
                return FlipperResult::error(format!("Command '{}' is blocked for safety", b));
            }
        }
        
        match self.serial.send_command(command) {
            Ok(output) => FlipperResult::success(
                format!("Executed: {}", command),
                Some(json!({ "command": command, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // SubGHz Operations
    // ========================================================================
    
    fn subghz_transmit(&mut self, signal_file: &str) -> FlipperResult {
        info!("SubGHz TX: {}", signal_file);
        
        // WARNING: RF transmission requires proper authorization
        match self.serial.send_command(&format!("subghz tx {}", signal_file)) {
            Ok(output) => FlipperResult::success(
                format!("SubGHz transmitted: {}", signal_file),
                Some(json!({ "file": signal_file, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn subghz_receive(&mut self, frequency: u64, duration_ms: u64) -> FlipperResult {
        let freq_mhz = frequency as f64 / 1_000_000.0;
        info!("SubGHz RX: {} MHz for {} ms", freq_mhz, duration_ms);
        
        match self.serial.send_command(&format!("subghz rx {}", frequency)) {
            Ok(output) => FlipperResult::success(
                format!("SubGHz receiving on {} MHz", freq_mhz),
                Some(json!({ "frequency": frequency, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // Infrared Operations
    // ========================================================================
    
    fn ir_transmit(&mut self, signal_file: &str, signal_name: Option<&str>) -> FlipperResult {
        let cmd = if let Some(name) = signal_name {
            format!("ir tx {} {}", signal_file, name)
        } else {
            format!("ir tx {}", signal_file)
        };
        
        match self.serial.send_command(&cmd) {
            Ok(output) => FlipperResult::success(
                format!("IR transmitted: {}", signal_file),
                Some(json!({ "file": signal_file, "signal": signal_name, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn ir_receive(&mut self, timeout_ms: u64) -> FlipperResult {
        match self.serial.send_command("ir rx") {
            Ok(output) => FlipperResult::success(
                "IR receiver started",
                Some(json!({ "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // NFC/RFID Operations
    // ========================================================================
    
    fn nfc_emulate(&mut self, card_file: &str) -> FlipperResult {
        match self.serial.send_command(&format!("nfc emulate {}", card_file)) {
            Ok(output) => FlipperResult::success(
                format!("NFC emulating: {}", card_file),
                Some(json!({ "file": card_file, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn nfc_read(&mut self, timeout_ms: u64) -> FlipperResult {
        match self.serial.send_command("nfc read") {
            Ok(output) => FlipperResult::success(
                "NFC reader started",
                Some(json!({ "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn rfid_emulate(&mut self, card_file: &str) -> FlipperResult {
        match self.serial.send_command(&format!("rfid emulate {}", card_file)) {
            Ok(output) => FlipperResult::success(
                format!("RFID emulating: {}", card_file),
                Some(json!({ "file": card_file, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn rfid_read(&mut self, timeout_ms: u64) -> FlipperResult {
        match self.serial.send_command("rfid read") {
            Ok(output) => FlipperResult::success(
                "RFID reader started",
                Some(json!({ "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn ibutton_emulate(&mut self, key_file: &str) -> FlipperResult {
        match self.serial.send_command(&format!("ibutton emulate {}", key_file)) {
            Ok(output) => FlipperResult::success(
                format!("iButton emulating: {}", key_file),
                Some(json!({ "file": key_file, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // BadUSB
    // ========================================================================
    
    fn badusb_execute(&mut self, script_path: &str) -> FlipperResult {
        warn!("BadUSB execution: {}", script_path);
        
        match self.serial.send_command(&format!("badusb {}", script_path)) {
            Ok(output) => FlipperResult::success(
                format!("BadUSB script started: {}", script_path),
                Some(json!({ "script": script_path, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // App Control
    // ========================================================================
    
    fn launch_app(&mut self, app_name: &str) -> FlipperResult {
        match self.serial.send_command(&format!("loader open {}", app_name)) {
            Ok(output) => FlipperResult::success(
                format!("Launched app: {}", app_name),
                Some(json!({ "app": app_name, "output": output }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // Hardware Control
    // ========================================================================
    
    fn led_control(&mut self, color: LedColor, state: bool) -> FlipperResult {
        let color_name = match color {
            LedColor::Red => "red",
            LedColor::Green => "green",
            LedColor::Blue => "blue",
            LedColor::Backlight => "backlight",
        };
        
        let cmd = format!("led {} {}", color_name, if state { "on" } else { "off" });
        
        match self.serial.send_command(&cmd) {
            Ok(_) => FlipperResult::success(
                format!("LED {} {}", color_name, if state { "on" } else { "off" }),
                Some(json!({ "color": color_name, "state": state }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn vibro_control(&mut self, state: bool) -> FlipperResult {
        let cmd = format!("vibro {}", if state { "on" } else { "off" });
        
        match self.serial.send_command(&cmd) {
            Ok(_) => FlipperResult::success(
                format!("Vibro {}", if state { "on" } else { "off" }),
                Some(json!({ "state": state }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn gpio_set(&mut self, pin: u8, state: bool) -> FlipperResult {
        let cmd = format!("gpio set {} {}", pin, if state { "1" } else { "0" });
        
        match self.serial.send_command(&cmd) {
            Ok(_) => FlipperResult::success(
                format!("GPIO {} set to {}", pin, state),
                Some(json!({ "pin": pin, "state": state }))
            ),
            Err(e) => FlipperResult::error(e),
        }
    }
    
    fn gpio_read(&mut self, pin: u8) -> FlipperResult {
        match self.serial.send_command(&format!("gpio read {}", pin)) {
            Ok(output) => {
                let state = output.trim() == "1";
                FlipperResult::success(
                    format!("GPIO {} = {}", pin, state),
                    Some(json!({ "pin": pin, "state": state }))
                )
            }
            Err(e) => FlipperResult::error(e),
        }
    }
    
    // ========================================================================
    // Payload Operations
    // ========================================================================
    
    fn push_artifact(&mut self, artifact_type: ArtifactType, path: &str, content: &str) -> FlipperResult {
        // Determine appropriate path based on artifact type
        let full_path = match artifact_type {
            ArtifactType::Fap => {
                if path.starts_with("/ext/apps") {
                    path.to_string()
                } else {
                    format!("{}/{}", paths::APPS, path)
                }
            }
            ArtifactType::Config | ArtifactType::Data => {
                if path.starts_with("/ext") {
                    path.to_string()
                } else {
                    format!("{}/{}", paths::APPS_DATA, path)
                }
            }
            ArtifactType::Executable => path.to_string(),
        };
        
        self.write_file(&full_path, content)
    }
    
    fn forge_payload(&mut self, payload_type: PayloadType, spec: &str) -> FlipperResult {
        // This would integrate with AI payload generation
        // For now, return info about what would be generated
        FlipperResult::success(
            format!("Payload generation requested: {:?}", payload_type),
            Some(json!({
                "payload_type": format!("{:?}", payload_type),
                "spec": spec,
                "status": "requires_ai_integration"
            }))
        )
    }
}

impl Default for FlipperExecutor {
    fn default() -> Self {
        Self::new()
    }
}
