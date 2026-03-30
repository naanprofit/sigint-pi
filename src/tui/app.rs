use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tokio::sync::broadcast;

use crate::ScanEvent;
use crate::wifi::WifiDevice;
use crate::bluetooth::{BleDevice, BleDeviceType};
use chrono::Utc;

use super::ui;

/// Sort options for device lists
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Signal,      // Strongest first
    Channel,     // By channel number
    DeviceType,  // Trackers first, then by type
    Threat,      // Most concerning first
    LastSeen,    // Most recent first
    Name,        // Alphabetical
}

impl SortBy {
    pub fn next(&self) -> Self {
        match self {
            SortBy::Signal => SortBy::Channel,
            SortBy::Channel => SortBy::DeviceType,
            SortBy::DeviceType => SortBy::Threat,
            SortBy::Threat => SortBy::LastSeen,
            SortBy::LastSeen => SortBy::Name,
            SortBy::Name => SortBy::Signal,
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            SortBy::Signal => "Signal",
            SortBy::Channel => "Channel",
            SortBy::DeviceType => "Type",
            SortBy::Threat => "Threat",
            SortBy::LastSeen => "Recent",
            SortBy::Name => "Name",
        }
    }
}

/// Active tab in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    WiFi,
    Bluetooth,
    Alerts,
    Threats,
    Help,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Dashboard => Tab::WiFi,
            Tab::WiFi => Tab::Bluetooth,
            Tab::Bluetooth => Tab::Alerts,
            Tab::Alerts => Tab::Threats,
            Tab::Threats => Tab::Help,
            Tab::Help => Tab::Dashboard,
        }
    }
    
    pub fn prev(&self) -> Self {
        match self {
            Tab::Dashboard => Tab::Help,
            Tab::WiFi => Tab::Dashboard,
            Tab::Bluetooth => Tab::WiFi,
            Tab::Alerts => Tab::Bluetooth,
            Tab::Threats => Tab::Alerts,
            Tab::Help => Tab::Threats,
        }
    }
}

/// Device with threat score for sorting
#[derive(Debug, Clone)]
pub struct ScoredDevice {
    pub mac: String,
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub rssi: i32,
    pub channel: Option<u8>,
    pub device_type: String,
    pub is_tracker: bool,
    pub is_new: bool,
    pub threat_score: f32,
    pub threat_reason: Option<String>,
    pub last_seen: i64,
    pub source: DeviceSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceSource {
    WiFi,
    Bluetooth,
}

/// Application state
pub struct App {
    pub running: bool,
    pub tab: Tab,
    pub sort_by: SortBy,
    pub sort_ascending: bool,
    pub selected_index: usize,
    pub scroll_offset: usize,
    
    // Device data
    pub wifi_devices: HashMap<String, WifiDevice>,
    pub ble_devices: HashMap<String, BleDevice>,
    pub alerts: Vec<AlertEntry>,
    
    // Stats
    pub total_wifi: usize,
    pub total_ble: usize,
    pub trackers_detected: usize,
    pub threats_detected: usize,
    pub new_devices: usize,
    
    // Hardware status
    pub wifi_enabled: bool,
    pub ble_enabled: bool,
    pub gps_enabled: bool,
    pub battery_percent: Option<u8>,
    pub current_channel: Option<u8>,
    
    // Timing
    pub last_update: Instant,
    pub uptime: Duration,
}

#[derive(Debug, Clone)]
pub struct AlertEntry {
    pub timestamp: i64,
    pub priority: String,
    pub message: String,
    pub device_mac: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            tab: Tab::Dashboard,
            sort_by: SortBy::Threat,
            sort_ascending: false,
            selected_index: 0,
            scroll_offset: 0,
            
            wifi_devices: HashMap::new(),
            ble_devices: HashMap::new(),
            alerts: Vec::new(),
            
            total_wifi: 0,
            total_ble: 0,
            trackers_detected: 0,
            threats_detected: 0,
            new_devices: 0,
            
            wifi_enabled: false,
            ble_enabled: false,
            gps_enabled: false,
            battery_percent: None,
            current_channel: None,
            
            last_update: Instant::now(),
            uptime: Duration::ZERO,
        }
    }
    
    pub fn on_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match key {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => self.running = false,
            
            // Tab navigation
            KeyCode::Tab => self.tab = self.tab.next(),
            KeyCode::BackTab => self.tab = self.tab.prev(),
            KeyCode::Char('1') => self.tab = Tab::Dashboard,
            KeyCode::Char('2') => self.tab = Tab::WiFi,
            KeyCode::Char('3') => self.tab = Tab::Bluetooth,
            KeyCode::Char('4') => self.tab = Tab::Alerts,
            KeyCode::Char('5') => self.tab = Tab::Threats,
            KeyCode::Char('?') | KeyCode::F(1) => self.tab = Tab::Help,
            
            // Sorting
            KeyCode::Char('s') => {
                self.sort_by = self.sort_by.next();
                self.selected_index = 0;
            }
            KeyCode::Char('S') => {
                self.sort_ascending = !self.sort_ascending;
            }
            
            // List navigation
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index += 1;
            }
            KeyCode::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(10);
            }
            KeyCode::PageDown => {
                self.selected_index += 10;
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.selected_index = 0;
            }
            KeyCode::End | KeyCode::Char('G') => {
                // Will be clamped in render
                self.selected_index = usize::MAX;
            }
            
            // Filtering (future)
            KeyCode::Char('/') => {
                // TODO: Enter search mode
            }
            
            // Refresh (force update)
            KeyCode::Char('r') | KeyCode::F(5) => {
                self.last_update = Instant::now();
            }
            
            _ => {}
        }
    }
    
    pub fn process_event(&mut self, event: ScanEvent) {
        match event {
            ScanEvent::WifiDevice(device) => {
                let is_new = !self.wifi_devices.contains_key(&device.mac_address);
                if is_new {
                    self.new_devices += 1;
                }
                self.wifi_devices.insert(device.mac_address.clone(), device);
                self.total_wifi = self.wifi_devices.len();
            }
            ScanEvent::BleDevice(device) => {
                let is_new = !self.ble_devices.contains_key(&device.mac_address);
                if is_new {
                    self.new_devices += 1;
                    if device.is_tracker() {
                        self.trackers_detected += 1;
                    }
                }
                self.ble_devices.insert(device.mac_address.clone(), device);
                self.total_ble = self.ble_devices.len();
            }
            ScanEvent::Alert { priority, message, device_mac } => {
                self.alerts.insert(0, AlertEntry {
                    timestamp: Utc::now().timestamp(),
                    priority: format!("{:?}", priority),
                    message,
                    device_mac,
                });
                if self.alerts.len() > 100 {
                    self.alerts.pop();
                }
            }
            ScanEvent::Attack(attack) => {
                self.threats_detected += 1;
                self.alerts.insert(0, AlertEntry {
                    timestamp: Utc::now().timestamp(),
                    priority: "Critical".to_string(),
                    message: format!("{:?}: {}", attack.attack_type, attack.description),
                    device_mac: Some(attack.source_mac),
                });
            }
            _ => {}
        }
        self.last_update = Instant::now();
    }
    
    /// Get sorted list of all devices with threat scores
    pub fn get_sorted_devices(&self) -> Vec<ScoredDevice> {
        let mut devices = Vec::new();
        let now = Utc::now();
        let new_threshold = chrono::Duration::minutes(5);
        
        // Add WiFi devices
        for (mac, dev) in &self.wifi_devices {
            let is_new = (now - dev.first_seen) < new_threshold;
            let threat_score = self.calculate_threat_score_wifi(dev, is_new);
            devices.push(ScoredDevice {
                mac: mac.clone(),
                name: dev.ssid.clone(),
                vendor: dev.vendor.clone(),
                rssi: dev.rssi,
                channel: Some(dev.channel),
                device_type: if dev.is_ap { "AP".to_string() } else { "Client".to_string() },
                is_tracker: false,
                is_new,
                threat_score: threat_score.0,
                threat_reason: threat_score.1,
                last_seen: dev.last_seen.timestamp(),
                source: DeviceSource::WiFi,
            });
        }
        
        // Add BLE devices
        for (mac, dev) in &self.ble_devices {
            let is_new = (now - dev.first_seen) < new_threshold;
            let threat_score = self.calculate_threat_score_ble(dev, is_new);
            devices.push(ScoredDevice {
                mac: mac.clone(),
                name: dev.name.clone(),
                vendor: dev.vendor.clone(),
                rssi: dev.rssi,
                channel: None,
                device_type: format!("{:?}", dev.device_type),
                is_tracker: dev.is_tracker(),
                is_new,
                threat_score: threat_score.0,
                threat_reason: threat_score.1,
                last_seen: dev.last_seen.timestamp(),
                source: DeviceSource::Bluetooth,
            });
        }
        
        // Sort
        devices.sort_by(|a, b| {
            let cmp = match self.sort_by {
                SortBy::Signal => b.rssi.cmp(&a.rssi),
                SortBy::Channel => {
                    let ac = a.channel.unwrap_or(255);
                    let bc = b.channel.unwrap_or(255);
                    ac.cmp(&bc)
                }
                SortBy::DeviceType => {
                    // Trackers first, then by type name
                    match (a.is_tracker, b.is_tracker) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.device_type.cmp(&b.device_type),
                    }
                }
                SortBy::Threat => {
                    b.threat_score.partial_cmp(&a.threat_score).unwrap_or(std::cmp::Ordering::Equal)
                }
                SortBy::LastSeen => b.last_seen.cmp(&a.last_seen),
                SortBy::Name => {
                    let an = a.name.as_deref().unwrap_or("");
                    let bn = b.name.as_deref().unwrap_or("");
                    an.cmp(bn)
                }
            };
            
            if self.sort_ascending {
                cmp.reverse()
            } else {
                cmp
            }
        });
        
        devices
    }
    
    fn calculate_threat_score_wifi(&self, device: &WifiDevice, is_new: bool) -> (f32, Option<String>) {
        let mut score = 0.0f32;
        let mut reasons = Vec::new();
        
        // Check threat intel database
        if let Some(threat) = crate::threat_intel::check_mac_threat(&device.mac_address) {
            score += 0.8;
            reasons.push(threat.vendor);
        }
        
        // Check suspicious SSID
        if let Some(ref ssid) = device.ssid {
            if crate::threat_intel::check_ssid_suspicious(ssid) {
                score += 0.5;
                reasons.push("Suspicious SSID");
            }
        }
        
        // New device = moderate concern
        if is_new {
            score += 0.3;
            reasons.push("New device");
        }
        
        // Very strong signal = nearby
        if device.rssi > -50 {
            score += 0.2;
            reasons.push("Very close");
        } else if device.rssi > -60 {
            score += 0.1;
        }
        
        // Hidden SSID
        if device.ssid.is_none() && device.is_ap {
            score += 0.2;
            reasons.push("Hidden network");
        }
        
        let reason = if reasons.is_empty() {
            None
        } else {
            Some(reasons.join(", "))
        };
        
        (score.min(1.0), reason)
    }
    
    fn calculate_threat_score_ble(&self, device: &BleDevice, is_new: bool) -> (f32, Option<String>) {
        let mut score = 0.0f32;
        let mut reasons = Vec::new();
        
        // Check threat intel database
        if let Some(threat) = crate::threat_intel::check_mac_threat(&device.mac_address) {
            score += 0.8;
            reasons.push(threat.vendor);
        }
        
        // Tracker = high concern
        if device.is_tracker() {
            score += 0.6;
            reasons.push("Tracker detected");
        }
        
        // New device
        if is_new {
            score += 0.2;
            reasons.push("New device");
        }
        
        // Very strong signal
        if device.rssi > -50 {
            score += 0.2;
            reasons.push("Very close");
        }
        
        // Unknown device type with no name
        if matches!(device.device_type, BleDeviceType::Unknown) && device.name.is_none() {
            score += 0.1;
            reasons.push("Unidentified");
        }
        
        let reason = if reasons.is_empty() {
            None
        } else {
            Some(reasons.join(", "))
        };
        
        (score.min(1.0), reason)
    }
    
    /// Get only high-threat devices
    pub fn get_threat_devices(&self) -> Vec<ScoredDevice> {
        self.get_sorted_devices()
            .into_iter()
            .filter(|d| d.threat_score > 0.3 || d.is_tracker)
            .collect()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the TUI application
pub async fn run_tui(mut event_rx: broadcast::Receiver<ScanEvent>) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    
    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &mut app))?;
        
        // Handle input with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key.code, key.modifiers);
            }
        }
        
        // Process scan events (non-blocking)
        while let Ok(event) = event_rx.try_recv() {
            app.process_event(event);
        }
        
        // Update tick
        if last_tick.elapsed() >= tick_rate {
            app.uptime += last_tick.elapsed();
            last_tick = Instant::now();
        }
        
        if !app.running {
            break;
        }
    }
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}
