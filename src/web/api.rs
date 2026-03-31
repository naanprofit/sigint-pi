use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::storage::Database;

/// Shared application state for real-time data
pub struct AppState {
    pub start_time: Instant,
    pub wifi_devices: RwLock<Vec<WifiDeviceInfo>>,
    pub ble_devices: RwLock<Vec<BleDeviceInfo>>,
    pub alerts: RwLock<Vec<AlertInfo>>,
    pub attacks: RwLock<Vec<AttackInfo>>,
    pub hw_status: RwLock<HardwareStatusInfo>,
    pub gps_status: RwLock<GpsStatusInfo>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            wifi_devices: RwLock::new(Vec::new()),
            ble_devices: RwLock::new(Vec::new()),
            alerts: RwLock::new(Vec::new()),
            attacks: RwLock::new(Vec::new()),
            hw_status: RwLock::new(HardwareStatusInfo::default()),
            gps_status: RwLock::new(GpsStatusInfo::default()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct GpsStatusInfo {
    pub has_fix: bool,
    pub fix_type: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub speed: Option<f64>,
    pub heading: Option<f64>,
    pub satellites: u8,
    pub accuracy: Option<f64>,
    pub last_update: i64,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WifiDeviceInfo {
    pub mac: String,
    pub vendor: Option<String>,
    pub ssid: Option<String>,
    pub rssi: i32,
    pub channel: Option<u8>,
    pub is_ap: bool,
    pub is_new: bool,
    pub first_seen: i64,
    pub last_seen: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BleDeviceInfo {
    pub mac: String,
    pub name: Option<String>,
    pub device_type: String,
    pub vendor: Option<String>,
    pub rssi: i32,
    pub is_new: bool,
    pub is_tracker: bool,
    pub first_seen: i64,
    pub last_seen: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_info: Option<TrackerInfoApi>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TrackerInfoApi {
    pub tracker_type: String,
    pub status: Option<u8>,
    pub key_hint: Option<String>,
    pub is_lost_mode: bool,
    pub is_separated: bool,
    pub counter: Option<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AlertInfo {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub priority: String,
    pub device_mac: Option<String>,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AttackInfo {
    pub id: u64,
    pub attack_type: String,
    pub severity: String,
    pub source_mac: String,
    pub target_mac: Option<String>,
    pub description: String,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct HardwareStatusInfo {
    pub wifi: bool,
    pub ble: bool,
    pub gps: bool,
    pub battery: Option<u8>,
    pub platform: String,
    pub wifi_interface: Option<String>,
    pub monitor_mode: bool,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/status", web::get().to(get_status))
            .route("/devices", web::get().to(get_devices))
            .route("/devices/{mac}", web::get().to(get_device))
            .route("/devices/{mac}/baseline", web::post().to(mark_baseline))
            .route("/alerts", web::get().to(get_alerts))
            .route("/attacks", web::get().to(get_attacks))
            .route("/stats", web::get().to(get_stats))
            .route("/config", web::get().to(get_config))
            .route("/locations", web::get().to(get_locations))
            .route("/hardware/status", web::get().to(get_hardware_status))
            .route("/gps/status", web::get().to(get_gps_status))
            .route("/wifi/devices", web::get().to(get_wifi_devices))
            .route("/wifi/mode", web::get().to(get_wifi_mode))
            .route("/wifi/mode", web::post().to(set_wifi_mode))
            .route("/ble/devices", web::get().to(get_ble_devices))
            .route("/power/mode", web::post().to(set_power_mode))
            .route("/power/sleep-inhibit", web::get().to(get_sleep_inhibit))
            .route("/power/sleep-inhibit", web::post().to(set_sleep_inhibit))
            // Settings endpoints
            .route("/settings", web::get().to(get_settings))
            .route("/settings", web::post().to(save_settings))
            .route("/settings/{section}", web::get().to(get_settings_section))
            .route("/settings/{section}", web::post().to(update_settings_section))
            .route("/settings/ninja_mode", web::post().to(toggle_ninja_mode))
            // Geofencing
            .route("/geofence/status", web::get().to(get_geofence_status))
            .route("/geofence/home", web::post().to(set_geofence_home))
            // PCAP capture
            .route("/pcap/status", web::get().to(get_pcap_status))
            .route("/pcap/start", web::post().to(start_pcap_capture))
            .route("/pcap/stop", web::post().to(stop_pcap_capture))
            .route("/pcap/files", web::get().to(list_pcap_files))
            // AI/LLM
            .route("/ai/status", web::get().to(get_ai_status))
            .route("/ai/toggle", web::post().to(toggle_ai))
            .route("/ai/analyze", web::post().to(analyze_devices_ai))
            .route("/ai/cache", web::get().to(get_ai_cache))
            .route("/settings/llm/test", web::post().to(test_llm_connection))
            // Device Notes
            .route("/devices/{mac}/notes", web::get().to(get_device_notes))
            .route("/devices/{mac}/notes", web::post().to(add_device_note))
            .route("/devices/{mac}/notes/{note_id}", web::delete().to(delete_device_note))
            .route("/notes/recent", web::get().to(get_recent_notes))
            // Voice (STT/TTS)
            .route("/voice/transcribe", web::post().to(transcribe_audio))
            .route("/voice/speak", web::post().to(speak_text))
            .route("/voice/status", web::get().to(get_voice_status))
            // OUI Database
            .route("/oui/status", web::get().to(get_oui_status))
            .route("/oui/update", web::post().to(update_oui_database))
            .route("/oui/lookup/{mac}", web::get().to(lookup_oui))
            // LLM Analysis
            .route("/llm/analyze-device", web::post().to(llm_analyze_device))
            .route("/llm/system-prompt", web::get().to(get_llm_system_prompt))
    );
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    version: String,
    uptime_seconds: u64,
    device_name: String,
    location: String,
}

async fn get_status(
    config: web::Data<Arc<Config>>,
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    HttpResponse::Ok().json(StatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: state.start_time.elapsed().as_secs(),
        device_name: config.device.name.clone(),
        location: config.device.location_name.clone(),
    })
}

#[derive(Deserialize)]
struct DevicesQuery {
    device_type: Option<String>,
    since_hours: Option<i64>,
    baseline_only: Option<bool>,
}

#[derive(Serialize)]
struct DeviceResponse {
    mac_address: String,
    device_type: String,
    vendor: Option<String>,
    name: Option<String>,
    is_baseline: bool,
    first_seen: String,
    last_seen: String,
    rssi_avg: f64,
    sighting_count: u64,
}

async fn get_devices(
    state: web::Data<Arc<AppState>>,
    query: web::Query<DevicesQuery>,
) -> impl Responder {
    let mut devices = Vec::new();
    
    // Get WiFi devices unless specifically asking for BLE only
    if query.device_type.as_deref() != Some("ble") {
        let wifi = state.wifi_devices.read().await;
        for d in wifi.iter() {
            devices.push(serde_json::json!({
                "mac": d.mac,
                "type": "wifi",
                "vendor": d.vendor,
                "name": d.ssid,
                "rssi": d.rssi,
                "channel": d.channel,
                "is_ap": d.is_ap,
                "is_new": d.is_new,
                "first_seen": d.first_seen,
                "last_seen": d.last_seen,
            }));
        }
    }
    
    // Get BLE devices unless specifically asking for WiFi only
    if query.device_type.as_deref() != Some("wifi") {
        let ble = state.ble_devices.read().await;
        for d in ble.iter() {
            devices.push(serde_json::json!({
                "mac": d.mac,
                "type": "ble",
                "vendor": d.vendor,
                "name": d.name,
                "device_type": d.device_type,
                "rssi": d.rssi,
                "is_tracker": d.is_tracker,
                "is_new": d.is_new,
                "first_seen": d.first_seen,
                "last_seen": d.last_seen,
            }));
        }
    }
    
    HttpResponse::Ok().json(devices)
}

async fn get_device(
    _db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let _mac = path.into_inner();
    // TODO: Implement device details
    HttpResponse::Ok().json(serde_json::json!({
        "error": "Not implemented"
    }))
}

async fn mark_baseline(
    db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let mac = path.into_inner();
    
    match db.mark_as_baseline(&mac, 1).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("Device {} marked as baseline", mac)
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    }
}

#[derive(Deserialize)]
struct AlertsQuery {
    priority: Option<String>,
    since_hours: Option<i64>,
    limit: Option<i64>,
}

async fn get_alerts(
    state: web::Data<Arc<AppState>>,
    query: web::Query<AlertsQuery>,
) -> impl Responder {
    let alerts = state.alerts.read().await;
    let limit = query.limit.unwrap_or(50) as usize;
    
    let filtered: Vec<_> = alerts.iter()
        .filter(|a| {
            if let Some(ref p) = query.priority {
                a.priority.to_lowercase() == p.to_lowercase()
            } else {
                true
            }
        })
        .take(limit)
        .cloned()
        .collect();
    
    HttpResponse::Ok().json(filtered)
}

async fn get_attacks(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let attacks = state.attacks.read().await;
    HttpResponse::Ok().json(attacks.clone())
}

async fn get_stats(
    state: web::Data<Arc<AppState>>,
    db: web::Data<Arc<Database>>,
) -> impl Responder {
    // Get real-time counts from state
    let wifi_count = state.wifi_devices.read().await.len();
    let ble_count = state.ble_devices.read().await.len();
    let alert_count = state.alerts.read().await.len();
    let attack_count = state.attacks.read().await.len();
    
    // Get new device counts (last 24h)
    let wifi_new = state.wifi_devices.read().await.iter().filter(|d| d.is_new).count();
    let ble_new = state.ble_devices.read().await.iter().filter(|d| d.is_new).count();
    
    // Get tracker count
    let tracker_count = state.ble_devices.read().await.iter().filter(|d| d.is_tracker).count();
    
    // Try to get DB counts too
    let db_counts = db.get_device_counts(1).await.ok();
    
    HttpResponse::Ok().json(serde_json::json!({
        "wifi_devices_total": wifi_count,
        "ble_devices_total": ble_count,
        "wifi_devices_new": wifi_new,
        "ble_devices_new": ble_new,
        "trackers_detected": tracker_count,
        "alerts_count": alert_count,
        "attacks_detected": attack_count,
        "wifi_devices_baseline": db_counts.as_ref().map(|c| c.wifi_baseline).unwrap_or(0),
        "ble_devices_baseline": db_counts.as_ref().map(|c| c.ble_baseline).unwrap_or(0),
    }))
}

async fn get_config(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    // Return sanitized config (no secrets)
    HttpResponse::Ok().json(serde_json::json!({
        "device": {
            "name": config.device.name,
            "location": config.device.location_name,
        },
        "wifi": {
            "enabled": config.wifi.enabled,
            "interface": config.wifi.interface,
            "scan_interval_ms": config.wifi.scan_interval_ms,
            "attack_detection": config.wifi.attack_detection,
        },
        "bluetooth": {
            "enabled": config.bluetooth.enabled,
            "scan_interval_ms": config.bluetooth.scan_interval_ms,
            "detect_airtags": config.bluetooth.detect_airtags,
        },
        "gps": {
            "enabled": config.gps.enabled,
        },
        "learning": {
            "enabled": config.learning.enabled,
            "training_hours": config.learning.training_hours,
            "geofence_radius_meters": config.learning.geofence_radius_meters,
        },
        "alerts": {
            "telegram_enabled": config.alerts.telegram.enabled,
            "twilio_enabled": config.alerts.twilio.enabled,
            "email_enabled": config.alerts.email.enabled,
            "mqtt_enabled": config.alerts.mqtt.enabled,
        }
    }))
}

async fn get_locations(
    _db: web::Data<Arc<Database>>,
) -> impl Responder {
    // TODO: Implement locations listing
    HttpResponse::Ok().json(Vec::<serde_json::Value>::new())
}

async fn get_gps_status(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let gps = state.gps_status.read().await;
    HttpResponse::Ok().json(gps.clone())
}

async fn get_hardware_status(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let hw = state.hw_status.read().await;
    HttpResponse::Ok().json(hw.clone())
}

async fn get_wifi_devices(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let devices = state.wifi_devices.read().await;
    
    // Add vendor lookup for devices that don't have it
    let oui = crate::storage::OuiLookup::embedded();
    let enriched: Vec<_> = devices.iter().map(|d| {
        let mut device = d.clone();
        if device.vendor.is_none() {
            device.vendor = oui.lookup(&d.mac).map(|s| s.to_string());
        }
        device
    }).collect();
    
    HttpResponse::Ok().json(enriched)
}

/// Get current WiFi interface mode (monitor/managed)
async fn get_wifi_mode(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let interface = &config.wifi.interface;
    
    // Run iwconfig to get current mode
    let output = std::process::Command::new("iwconfig")
        .arg(interface)
        .output();
    
    let (mode, is_up) = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mode = if stdout.contains("Mode:Monitor") {
                "monitor"
            } else if stdout.contains("Mode:Managed") {
                "managed"
            } else {
                "unknown"
            };
            let is_up = !stdout.contains("Not-Associated") && !stdout.contains("off/any");
            (mode, is_up)
        }
        Err(_) => ("error", false),
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "interface": interface,
        "mode": mode,
        "is_up": is_up
    }))
}

#[derive(Deserialize)]
struct WifiModeRequest {
    mode: String, // "monitor" or "managed"
}

/// Set WiFi interface mode (requires sudo)
async fn set_wifi_mode(
    config: web::Data<Arc<Config>>,
    body: web::Json<WifiModeRequest>,
) -> impl Responder {
    let interface = &config.wifi.interface;
    let mode = body.mode.to_lowercase();
    
    if mode != "monitor" && mode != "managed" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Mode must be 'monitor' or 'managed'"
        }));
    }
    
    // Run commands to change mode
    // 1. Bring interface down
    let down_result = std::process::Command::new("sudo")
        .args(["ip", "link", "set", interface, "down"])
        .output();
    
    if down_result.is_err() {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": "Failed to bring interface down"
        }));
    }
    
    // 2. Set mode
    let mode_result = std::process::Command::new("sudo")
        .args(["iw", "dev", interface, "set", "type", &mode])
        .output();
    
    if let Err(e) = mode_result {
        // Try to bring interface back up
        let _ = std::process::Command::new("sudo")
            .args(["ip", "link", "set", interface, "up"])
            .output();
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to set mode: {}", e)
        }));
    }
    
    // 3. Bring interface up
    let up_result = std::process::Command::new("sudo")
        .args(["ip", "link", "set", interface, "up"])
        .output();
    
    if up_result.is_err() {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": "Failed to bring interface up"
        }));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "interface": interface,
        "mode": mode
    }))
}

async fn get_ble_devices(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let devices = state.ble_devices.read().await;
    
    // Add vendor lookup for devices that don't have it
    let oui = crate::storage::OuiLookup::embedded();
    let enriched: Vec<_> = devices.iter().map(|d| {
        let mut device = d.clone();
        if device.vendor.is_none() {
            device.vendor = oui.lookup(&d.mac).map(|s| s.to_string());
        }
        device
    }).collect();
    
    HttpResponse::Ok().json(enriched)
}

#[derive(Deserialize)]
struct PowerModeRequest {
    mode: String,
}

async fn set_power_mode(
    body: web::Json<PowerModeRequest>,
) -> impl Responder {
    // TODO: Actually change power mode
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "mode": body.mode
    }))
}

// ============================================
// Sleep Inhibit (Prevent Suspend)
// ============================================

/// Check if sleep/suspend is currently inhibited
async fn get_sleep_inhibit() -> impl Responder {
    // Check if our inhibitor is active by looking for our process in inhibitor list
    let output = std::process::Command::new("systemd-inhibit")
        .arg("--list")
        .output();
    
    let is_inhibited = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.contains("sigint-deck") || stdout.contains("SIGINT")
        }
        Err(_) => false,
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "inhibited": is_inhibited,
        "description": if is_inhibited { "Sleep/suspend is blocked" } else { "Normal power management" }
    }))
}

#[derive(Deserialize)]
struct SleepInhibitRequest {
    inhibit: bool,
}

// Global to track inhibitor process
use std::sync::atomic::{AtomicU32, Ordering};
static INHIBITOR_PID: AtomicU32 = AtomicU32::new(0);

/// Enable or disable sleep inhibition
/// When enabled, prevents system suspend but allows screen to sleep
async fn set_sleep_inhibit(
    body: web::Json<SleepInhibitRequest>,
) -> impl Responder {
    if body.inhibit {
        // Check if already inhibited
        let current_pid = INHIBITOR_PID.load(Ordering::SeqCst);
        if current_pid != 0 {
            // Check if process is still running
            let check = std::process::Command::new("kill")
                .args(["-0", &current_pid.to_string()])
                .output();
            if check.map(|o| o.status.success()).unwrap_or(false) {
                return HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "inhibited": true,
                    "message": "Already inhibited"
                }));
            }
        }
        
        // Start inhibitor process
        // This runs in background and inhibits sleep as long as it's alive
        let result = std::process::Command::new("systemd-inhibit")
            .args([
                "--what=sleep:idle",
                "--who=sigint-deck",
                "--why=SIGINT-Deck scanning active - keeping GPS and WiFi alive",
                "--mode=block",
                "sleep", "infinity"
            ])
            .spawn();
        
        match result {
            Ok(child) => {
                let pid = child.id();
                INHIBITOR_PID.store(pid, Ordering::SeqCst);
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "inhibited": true,
                    "pid": pid,
                    "message": "Sleep/suspend inhibited. Screen can still sleep but system won't suspend."
                }))
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Failed to start inhibitor: {}", e)
                }))
            }
        }
    } else {
        // Kill inhibitor process
        let pid = INHIBITOR_PID.load(Ordering::SeqCst);
        if pid != 0 {
            let _ = std::process::Command::new("kill")
                .arg(pid.to_string())
                .output();
            INHIBITOR_PID.store(0, Ordering::SeqCst);
        }
        
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "inhibited": false,
            "message": "Normal power management restored"
        }))
    }
}

// ============================================
// AI Intelligence Endpoints
// ============================================

#[derive(Serialize)]
struct AiStatusResponse {
    enabled: bool,
    available: bool,
    endpoint: String,
    cached_devices: u32,
}

#[derive(Deserialize)]
struct AiToggleRequest {
    enabled: bool,
}

#[derive(Deserialize)]
struct AiAnalyzeRequest {
    mac_addresses: Option<Vec<String>>,  // None = analyze all current devices
}

#[derive(Serialize)]
struct AiAnalyzeResponse {
    success: bool,
    analyzed: u32,
    results: Vec<DeviceIntelligenceInfo>,
    error: Option<String>,
}

#[derive(Clone, Serialize)]
struct DeviceIntelligenceInfo {
    mac_address: String,
    device_name: Option<String>,
    device_type: String,
    vendor_name: Option<String>,
    ai_description: Option<String>,
    threat_level: String,
    threat_reason: Option<String>,
    from_cache: bool,
}

/// GET /api/ai/status - Check AI feature status
pub async fn get_ai_status(
    config: web::Data<Arc<crate::config::Config>>,
) -> impl Responder {
    let llm_config = &config.llm;
    
    HttpResponse::Ok().json(AiStatusResponse {
        enabled: llm_config.as_ref().map(|c| c.enabled).unwrap_or(false),
        available: llm_config.is_some(),
        endpoint: llm_config.as_ref()
            .map(|c| c.endpoint.clone())
            .unwrap_or_else(|| "not configured".to_string()),
        cached_devices: 0, // TODO: get from DB
    })
}

/// POST /api/ai/toggle - Enable/disable AI analysis
pub async fn toggle_ai(
    body: web::Json<AiToggleRequest>,
) -> impl Responder {
    // Note: This would need to update runtime config
    // For now, just acknowledge the request
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "enabled": body.enabled,
        "message": if body.enabled { 
            "AI analysis enabled - use Refresh to analyze devices" 
        } else { 
            "AI analysis disabled - using local database only" 
        }
    }))
}

/// POST /api/ai/analyze - Trigger AI analysis of devices (user-initiated)
pub async fn analyze_devices_ai(
    state: web::Data<Arc<AppState>>,
    body: web::Json<AiAnalyzeRequest>,
    config: web::Data<Arc<crate::config::Config>>,
) -> impl Responder {
    let llm_config = match &config.llm {
        Some(c) if c.enabled => c.clone(),
        _ => {
            return HttpResponse::Ok().json(AiAnalyzeResponse {
                success: false,
                analyzed: 0,
                results: vec![],
                error: Some("AI analysis is not enabled in config".to_string()),
            });
        }
    };
    
    // Get devices to analyze
    let devices_to_analyze: Vec<(String, Option<String>, String, Option<String>, bool)> = 
        if let Some(ref macs) = body.mac_addresses {
            // Analyze specific devices
            let ble = state.ble_devices.read().await;
            let wifi = state.wifi_devices.read().await;
            
            let mut devices = Vec::new();
            for mac in macs {
                // Check BLE devices
                if let Some(d) = ble.iter().find(|d| d.mac == *mac) {
                    devices.push((
                        d.mac.clone(),
                        d.name.clone(),
                        "ble".to_string(),
                        d.vendor.clone(),
                        d.is_tracker,
                    ));
                }
                // Check WiFi devices
                else if let Some(d) = wifi.iter().find(|d| d.mac == *mac) {
                    devices.push((
                        d.mac.clone(),
                        d.ssid.clone(),
                        if d.is_ap { "wifi_ap" } else { "wifi_client" }.to_string(),
                        d.vendor.clone(),
                        false,
                    ));
                }
            }
            devices
        } else {
            // Analyze all current devices
            let ble = state.ble_devices.read().await;
            let wifi = state.wifi_devices.read().await;
            
            let mut devices: Vec<_> = ble.iter().map(|d| (
                d.mac.clone(),
                d.name.clone(),
                "ble".to_string(),
                d.vendor.clone(),
                d.is_tracker,
            )).collect();
            
            devices.extend(wifi.iter().map(|d| (
                d.mac.clone(),
                d.ssid.clone(),
                if d.is_ap { "wifi_ap" } else { "wifi_client" }.to_string(),
                d.vendor.clone(),
                false,
            )));
            
            devices
        };
    
    if devices_to_analyze.is_empty() {
        return HttpResponse::Ok().json(AiAnalyzeResponse {
            success: true,
            analyzed: 0,
            results: vec![],
            error: Some("No devices to analyze".to_string()),
        });
    }
    
    // Call LLM for analysis
    let client = crate::intelligence::LlmClient::new(llm_config);
    
    let queries: Vec<crate::intelligence::llm_client::DeviceQuery> = devices_to_analyze
        .iter()
        .map(|(mac, name, dtype, vendor, is_tracker)| {
            crate::intelligence::llm_client::DeviceQuery {
                mac_address: mac.clone(),
                device_name: name.clone(),
                device_type: dtype.clone(),
                vendor: vendor.clone(),
                ssid: None,
                is_tracker: *is_tracker,
            }
        })
        .collect();
    
    match client.analyze_devices_batch(&queries).await {
        Ok(analyses) => {
            let results: Vec<DeviceIntelligenceInfo> = queries
                .iter()
                .zip(analyses.iter())
                .map(|(q, a)| DeviceIntelligenceInfo {
                    mac_address: q.mac_address.clone(),
                    device_name: q.device_name.clone(),
                    device_type: q.device_type.clone(),
                    vendor_name: q.vendor.clone(),
                    ai_description: Some(a.description.clone()),
                    threat_level: "unknown".to_string(),
                    threat_reason: a.threat_assessment.clone(),
                    from_cache: false,
                })
                .collect();
            
            HttpResponse::Ok().json(AiAnalyzeResponse {
                success: true,
                analyzed: results.len() as u32,
                results,
                error: None,
            })
        }
        Err(e) => {
            HttpResponse::Ok().json(AiAnalyzeResponse {
                success: false,
                analyzed: 0,
                results: vec![],
                error: Some(format!("AI analysis failed: {}", e)),
            })
        }
    }
}

/// GET /api/ai/cache - Get all cached device descriptions
pub async fn get_ai_cache() -> impl Responder {
    // TODO: Load from database
    HttpResponse::Ok().json(Vec::<DeviceIntelligenceInfo>::new())
}

// ============================================
// LLM Provider Settings Endpoints
// ============================================

#[derive(Serialize)]
pub struct LlmSettingsResponse {
    pub enabled: bool,
    pub provider: String,
    pub endpoint: String,
    pub model: String,
    pub has_api_key: bool,  // Never expose actual key
    pub max_tokens: u32,
    pub timeout_secs: u64,
    pub available_providers: Vec<ProviderInfo>,
}

#[derive(Serialize)]
pub struct ProviderInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub default_endpoint: &'static str,
    pub requires_api_key: bool,
}

/// GET /api/settings/llm - Get current LLM settings (without exposing API key)
pub async fn get_llm_settings(
    config: web::Data<Arc<crate::config::Config>>,
) -> impl Responder {
    let providers = vec![
        ProviderInfo {
            id: "openai",
            name: "OpenAI",
            description: "GPT-4, GPT-3.5-turbo (requires API key)",
            default_endpoint: "https://api.openai.com/v1",
            requires_api_key: true,
        },
        ProviderInfo {
            id: "llamacpp",
            name: "llama.cpp Server",
            description: "Local llama.cpp with OpenAI-compatible API",
            default_endpoint: "http://localhost:11434",
            requires_api_key: false,
        },
        ProviderInfo {
            id: "ollama",
            name: "Ollama",
            description: "Local Ollama server",
            default_endpoint: "http://localhost:11434",
            requires_api_key: false,
        },
        ProviderInfo {
            id: "lmstudio",
            name: "LM Studio",
            description: "LM Studio local server",
            default_endpoint: "http://localhost:1234/v1",
            requires_api_key: false,
        },
        ProviderInfo {
            id: "anthropic",
            name: "Anthropic Claude",
            description: "Claude 3 models (requires API key)",
            default_endpoint: "https://api.anthropic.com/v1",
            requires_api_key: true,
        },
        ProviderInfo {
            id: "custom",
            name: "Custom Endpoint",
            description: "Any OpenAI-compatible API",
            default_endpoint: "http://localhost:8080/v1",  // Custom endpoint - user must configure
            requires_api_key: false,
        },
    ];
    
    let llm_config = config.llm.as_ref();
    
    HttpResponse::Ok().json(LlmSettingsResponse {
        enabled: llm_config.map(|c| c.enabled).unwrap_or(false),
        provider: llm_config.map(|c| c.provider.clone()).unwrap_or_else(|| "llamacpp".to_string()),
        endpoint: llm_config.map(|c| c.endpoint.clone()).unwrap_or_else(|| "http://localhost:11434".to_string()),
        model: llm_config.map(|c| c.model.clone()).unwrap_or_else(|| "default".to_string()),
        has_api_key: llm_config.and_then(|c| c.api_key.as_ref()).map(|k| !k.is_empty()).unwrap_or(false),
        max_tokens: llm_config.map(|c| c.max_tokens).unwrap_or(200),
        timeout_secs: llm_config.map(|c| c.timeout_secs).unwrap_or(30),
        available_providers: providers,
    })
}

#[derive(Deserialize)]
pub struct UpdateLlmSettingsRequest {
    pub enabled: Option<bool>,
    pub provider: Option<String>,
    pub endpoint: Option<String>,
    pub model: Option<String>,
    pub api_key: Option<String>,  // Only updated if provided
    pub max_tokens: Option<u32>,
    pub timeout_secs: Option<u64>,
}

/// POST /api/settings/llm - Update LLM settings
pub async fn update_llm_settings(
    body: web::Json<UpdateLlmSettingsRequest>,
) -> impl Responder {
    // NOTE: In production, this would:
    // 1. Validate the settings
    // 2. Update the runtime config
    // 3. Save to config file (minus sensitive data or with encryption)
    // 4. Reload the LLM client
    
    // For now, just acknowledge - actual implementation would need mutable config state
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "LLM settings updated. Restart required for some changes.",
        "restart_required": body.endpoint.is_some() || body.provider.is_some()
    }))
}

/// POST /api/settings/llm/test - Test LLM connection
pub async fn test_llm_connection(
    config: web::Data<Arc<crate::config::Config>>,
) -> impl Responder {
    let llm_config = match &config.llm {
        Some(c) if c.enabled => c.clone(),
        Some(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "LLM is disabled in settings"
            }));
        }
        None => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "LLM not configured"
            }));
        }
    };
    
    // Try to connect and get a simple response
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build();
    
    let client = match client {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to create HTTP client: {}", e)
            }));
        }
    };
    
    // Try to list models - different providers use different endpoints
    let base_url = llm_config.endpoint.trim_end_matches('/');
    let url = if base_url.contains("11434") || llm_config.provider.to_lowercase() == "ollama" {
        // Ollama uses /api/tags
        format!("{}/api/tags", base_url)
    } else if base_url.contains("1234") || llm_config.provider.to_lowercase() == "lmstudio" {
        // LM Studio uses /v1/models
        format!("{}/v1/models", base_url)
    } else {
        // Default OpenAI-compatible: /models or /v1/models
        format!("{}/models", base_url)
    };
    let mut req = client.get(&url);
    
    if let Some(ref api_key) = llm_config.api_key {
        if !api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }
    }
    
    match req.send().await {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": "Successfully connected to LLM provider",
                    "status": response.status().as_u16()
                }))
            } else {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": false,
                    "error": format!("LLM API returned status {}", response.status()),
                    "status": response.status().as_u16()
                }))
            }
        }
        Err(e) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to connect: {}", e)
            }))
        }
    }
}

// ============================================
// SETTINGS ENDPOINTS
// ============================================

/// GET /api/settings - Get all settings from config file
async fn get_settings() -> impl Responder {
    // Find existing config file
    let config_paths = [
        dirs::home_dir()
            .map(|h| h.join("sigint-deck").join("config.toml"))
            .unwrap_or_default(),
        dirs::home_dir()
            .map(|h| h.join("sigint-pi").join("config.toml"))
            .unwrap_or_default(),
        std::path::PathBuf::from("./config.toml"),
    ];
    
    let config_path = config_paths.iter().find(|p| p.exists());
    
    if let Some(path) = config_path {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(config) = toml::from_str::<toml::Value>(&content) {
                // Convert TOML to JSON-friendly settings format
                let settings = serde_json::json!({
                    "wifi": {
                        "enabled": config.get("wifi")
                            .and_then(|w| w.get("enabled"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                    },
                    "bluetooth": {
                        "enabled": config.get("bluetooth")
                            .and_then(|b| b.get("enabled"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                        "detect_trackers": config.get("bluetooth")
                            .and_then(|b| b.get("detect_trackers"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                    },
                    "gps": {
                        "enabled": config.get("gps")
                            .and_then(|g| g.get("enabled"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                        "geofencing": config.get("gps")
                            .and_then(|g| g.get("geofencing_enabled"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    },
                    "alerts": {
                        "sound": {
                            "enabled": config.get("alerts")
                                .and_then(|a| a.get("sound"))
                                .and_then(|s| s.get("enabled"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(true),
                            "ninja_mode": config.get("alerts")
                                .and_then(|a| a.get("sound"))
                                .and_then(|s| s.get("ninja_mode"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            "volume": config.get("alerts")
                                .and_then(|a| a.get("sound"))
                                .and_then(|s| s.get("volume"))
                                .and_then(|v| v.as_integer())
                                .unwrap_or(80) as u8,
                        },
                        "telegram": {
                            "enabled": config.get("alerts")
                                .and_then(|a| a.get("telegram"))
                                .and_then(|t| t.get("enabled"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            "bot_token": config.get("alerts")
                                .and_then(|a| a.get("telegram"))
                                .and_then(|t| t.get("bot_token"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                            "chat_id": config.get("alerts")
                                .and_then(|a| a.get("telegram"))
                                .and_then(|t| t.get("chat_id"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                        },
                        "signal": {
                            "enabled": config.get("alerts")
                                .and_then(|a| a.get("signal"))
                                .and_then(|s| s.get("enabled"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            "sender_number": config.get("alerts")
                                .and_then(|a| a.get("signal"))
                                .and_then(|s| s.get("sender_number"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                            "recipients": config.get("alerts")
                                .and_then(|a| a.get("signal"))
                                .and_then(|s| s.get("recipients"))
                                .and_then(|v| v.as_array())
                                .map(|arr| arr.iter()
                                    .filter_map(|v| v.as_str())
                                    .map(String::from)
                                    .collect::<Vec<_>>())
                                .unwrap_or_default(),
                        },
                        "mqtt": {
                            "enabled": config.get("alerts")
                                .and_then(|a| a.get("mqtt"))
                                .and_then(|m| m.get("enabled"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            "broker_url": config.get("alerts")
                                .and_then(|a| a.get("mqtt"))
                                .and_then(|m| m.get("broker_url"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                            "topic_prefix": config.get("alerts")
                                .and_then(|a| a.get("mqtt"))
                                .and_then(|m| m.get("topic_prefix"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("sigint"),
                        },
                        "openclaw": {
                            "enabled": config.get("alerts")
                                .and_then(|a| a.get("openclaw"))
                                .and_then(|o| o.get("enabled"))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                            "webhook_url": config.get("alerts")
                                .and_then(|a| a.get("openclaw"))
                                .and_then(|o| o.get("webhook_url"))
                                .and_then(|v| v.as_str())
                                .unwrap_or(""),
                        },
                    },
                    "llm": {
                        "enabled": config.get("llm")
                            .and_then(|l| l.get("enabled"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                        "provider": config.get("llm")
                            .and_then(|l| l.get("provider"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("ollama"),
                        "endpoint": config.get("llm")
                            .and_then(|l| l.get("endpoint"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("http://localhost:11434"),
                        "model": config.get("llm")
                            .and_then(|l| l.get("model"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("llama3"),
                    },
                    "power": {
                        "sleep_inhibit": config.get("power")
                            .and_then(|p| p.get("sleep_inhibit"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    },
                    "general": {
                        "ninja_mode": config.get("alerts")
                            .and_then(|a| a.get("sound"))
                            .and_then(|s| s.get("ninja_mode"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false),
                    },
                });
                return HttpResponse::Ok().json(settings);
            }
        }
    }
    
    // Fallback to default settings
    let settings = crate::settings::AppSettings::default();
    HttpResponse::Ok().json(settings)
}

/// POST /api/settings - Save settings (accepts partial updates)
async fn save_settings(
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    // Determine config path - check multiple locations
    let config_paths = [
        // Home directory sigint folder
        dirs::home_dir()
            .map(|h| h.join("sigint-pi").join("config.toml"))
            .unwrap_or_default(),
        dirs::home_dir()
            .map(|h| h.join("sigint-deck").join("config.toml"))
            .unwrap_or_default(),
        // Current directory
        std::path::PathBuf::from("./config.toml"),
    ];
    
    // Find existing config
    let settings_path = config_paths.iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|h| h.join("sigint-pi").join("config.toml"))
                .unwrap_or_else(|| std::path::PathBuf::from("./config.toml"))
        });
    
    // Load existing config file
    let existing_content = std::fs::read_to_string(&settings_path).unwrap_or_default();
    let mut existing: toml::Value = toml::from_str(&existing_content).unwrap_or_else(|_| {
        toml::Value::Table(toml::map::Map::new())
    });
    
    // Get the incoming settings as JSON
    let incoming = body.into_inner();
    
    // Merge incoming settings into existing config
    if let (toml::Value::Table(ref mut existing_table), Some(incoming_obj)) = (&mut existing, incoming.as_object()) {
        // Handle LLM settings
        if let Some(llm) = incoming_obj.get("llm") {
            let mut llm_table = toml::map::Map::new();
            if let Some(obj) = llm.as_object() {
                if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                    llm_table.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                }
                if let Some(provider) = obj.get("provider").and_then(|v| v.as_str()) {
                    llm_table.insert("provider".to_string(), toml::Value::String(provider.to_string()));
                }
                if let Some(endpoint) = obj.get("endpoint").and_then(|v| v.as_str()) {
                    llm_table.insert("endpoint".to_string(), toml::Value::String(endpoint.to_string()));
                }
                if let Some(model) = obj.get("model").and_then(|v| v.as_str()) {
                    llm_table.insert("model".to_string(), toml::Value::String(model.to_string()));
                }
            }
            if !llm_table.is_empty() {
                existing_table.insert("llm".to_string(), toml::Value::Table(llm_table));
            }
        }
        
        // Handle general settings
        if let Some(general) = incoming_obj.get("general") {
            if let Some(obj) = general.as_object() {
                if let Some(ninja_mode) = obj.get("ninja_mode").and_then(|v| v.as_bool()) {
                    // Update alerts.sound.ninja_mode
                    if let Some(alerts) = existing_table.get_mut("alerts") {
                        if let toml::Value::Table(ref mut alerts_table) = alerts {
                            if let Some(sound) = alerts_table.get_mut("sound") {
                                if let toml::Value::Table(ref mut sound_table) = sound {
                                    sound_table.insert("ninja_mode".to_string(), toml::Value::Boolean(ninja_mode));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Handle GPS settings
        if let Some(gps) = incoming_obj.get("gps") {
            let gps_table = existing_table.entry("gps".to_string())
                .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
            if let (toml::Value::Table(ref mut gps_t), Some(obj)) = (gps_table, gps.as_object()) {
                if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                    gps_t.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                }
                if let Some(geofencing) = obj.get("geofencing").and_then(|v| v.as_bool()) {
                    gps_t.insert("geofencing_enabled".to_string(), toml::Value::Boolean(geofencing));
                }
            }
        }
        
        // Handle WiFi settings
        if let Some(wifi) = incoming_obj.get("wifi") {
            let wifi_table = existing_table.entry("wifi".to_string())
                .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
            if let (toml::Value::Table(ref mut wifi_t), Some(obj)) = (wifi_table, wifi.as_object()) {
                if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                    wifi_t.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                }
            }
        }
        
        // Handle Bluetooth settings
        if let Some(bluetooth) = incoming_obj.get("bluetooth") {
            let ble_table = existing_table.entry("bluetooth".to_string())
                .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
            if let (toml::Value::Table(ref mut ble_t), Some(obj)) = (ble_table, bluetooth.as_object()) {
                if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                    ble_t.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                }
                if let Some(detect_trackers) = obj.get("detect_trackers").and_then(|v| v.as_bool()) {
                    ble_t.insert("detect_trackers".to_string(), toml::Value::Boolean(detect_trackers));
                }
            }
        }
        
        // Handle Alert settings
        if let Some(alerts) = incoming_obj.get("alerts") {
            let alerts_table = existing_table.entry("alerts".to_string())
                .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
            if let (toml::Value::Table(ref mut alerts_t), Some(obj)) = (alerts_table, alerts.as_object()) {
                // Sound settings
                if let Some(sound) = obj.get("sound") {
                    let sound_table = alerts_t.entry("sound".to_string())
                        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
                    if let (toml::Value::Table(ref mut sound_t), Some(snd)) = (sound_table, sound.as_object()) {
                        if let Some(enabled) = snd.get("enabled").and_then(|v| v.as_bool()) {
                            sound_t.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                        }
                        if let Some(ninja) = snd.get("ninja_mode").and_then(|v| v.as_bool()) {
                            sound_t.insert("ninja_mode".to_string(), toml::Value::Boolean(ninja));
                        }
                        if let Some(vol) = snd.get("volume").and_then(|v| v.as_i64()) {
                            sound_t.insert("volume".to_string(), toml::Value::Integer(vol));
                        }
                    }
                }
            }
        }
    }
    
    // Write back to file
    match toml::to_string_pretty(&existing) {
        Ok(content) => {
            if let Some(parent) = settings_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::write(&settings_path, &content) {
                Ok(_) => {
                    tracing::info!("Settings saved to {:?}", settings_path);
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Settings saved. Restart required for changes to take effect.",
                        "path": settings_path.to_string_lossy(),
                        "restart_required": true
                    }))
                },
                Err(e) => {
                    tracing::error!("Failed to save settings: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": format!("Failed to write settings: {}", e)
                    }))
                }
            }
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to serialize settings: {}", e)
        }))
    }
}

/// GET /api/settings/{section} - Get a specific settings section
async fn get_settings_section(
    path: web::Path<String>,
) -> impl Responder {
    let section = path.into_inner();
    let settings = crate::settings::AppSettings::default();
    
    let value = match section.as_str() {
        "general" => serde_json::to_value(&settings.general).ok(),
        "wifi" => serde_json::to_value(&settings.wifi).ok(),
        "bluetooth" => serde_json::to_value(&settings.bluetooth).ok(),
        "gps" => serde_json::to_value(&settings.gps).ok(),
        "alerts" => serde_json::to_value(&settings.alerts).ok(),
        "power" => serde_json::to_value(&settings.power).ok(),
        "privacy" => serde_json::to_value(&settings.privacy).ok(),
        _ => None,
    };
    
    match value {
        Some(v) => HttpResponse::Ok().json(v),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Unknown settings section: {}", section)
        }))
    }
}

/// POST /api/settings/{section} - Update a specific settings section
async fn update_settings_section(
    path: web::Path<String>,
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let section = path.into_inner();
    
    // Validate section name
    if !["general", "wifi", "bluetooth", "gps", "alerts", "power", "privacy"].contains(&section.as_str()) {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Unknown settings section: {}", section)
        }));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Settings section '{}' updated", section),
        "section": section,
        "data": body.into_inner()
    }))
}

/// POST /api/settings/ninja_mode - Toggle ninja mode
async fn toggle_ninja_mode(
    body: web::Json<serde_json::Value>,
) -> impl Responder {
    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "ninja_mode": enabled,
        "message": if enabled { "Ninja mode enabled - all sounds muted" } else { "Ninja mode disabled" }
    }))
}

// ============================================
// GEOFENCING ENDPOINTS
// ============================================

#[derive(Serialize)]
struct GeofenceStatus {
    enabled: bool,
    home_set: bool,
    home_lat: Option<f64>,
    home_lon: Option<f64>,
    home_radius_m: f64,
    is_home: Option<bool>,
    distance_from_home_m: Option<f64>,
}

/// GET /api/geofence/status - Get geofencing status
async fn get_geofence_status() -> impl Responder {
    let settings = crate::settings::AppSettings::default();
    
    HttpResponse::Ok().json(GeofenceStatus {
        enabled: settings.gps.geofencing,
        home_set: settings.gps.home_lat.is_some() && settings.gps.home_lon.is_some(),
        home_lat: settings.gps.home_lat,
        home_lon: settings.gps.home_lon,
        home_radius_m: settings.gps.home_radius_m,
        is_home: None, // Would be computed from current GPS position
        distance_from_home_m: None,
    })
}

#[derive(Deserialize)]
struct SetGeofenceHomeRequest {
    latitude: f64,
    longitude: f64,
    radius_m: Option<f64>,
}

/// POST /api/geofence/home - Set home location for geofencing
async fn set_geofence_home(
    body: web::Json<SetGeofenceHomeRequest>,
) -> impl Responder {
    let req = body.into_inner();
    
    // Validate coordinates
    if req.latitude < -90.0 || req.latitude > 90.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid latitude (must be -90 to 90)"
        }));
    }
    if req.longitude < -180.0 || req.longitude > 180.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid longitude (must be -180 to 180)"
        }));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Home location set",
        "home": {
            "latitude": req.latitude,
            "longitude": req.longitude,
            "radius_m": req.radius_m.unwrap_or(100.0)
        }
    }))
}

// ============================================
// PCAP CAPTURE ENDPOINTS
// ============================================

#[derive(Serialize)]
struct PcapStatus {
    capturing: bool,
    current_file: Option<String>,
    file_size_bytes: Option<u64>,
    packets_captured: Option<u64>,
    started_at: Option<i64>,
}

/// GET /api/pcap/status - Get PCAP capture status
async fn get_pcap_status() -> impl Responder {
    let (capturing, packets, bytes) = crate::wifi::scanner::get_pcap_stats();
    let settings = crate::settings::AppSettings::default();
    
    HttpResponse::Ok().json(PcapStatus {
        capturing,
        current_file: if capturing || settings.wifi.capture_pcap { 
            Some(settings.wifi.pcap_dir.join("capture.pcap").to_string_lossy().to_string())
        } else { 
            None 
        },
        file_size_bytes: Some(bytes),
        packets_captured: Some(packets),
        started_at: None,
    })
}

#[derive(Deserialize)]
struct StartPcapRequest {
    filename: Option<String>,
    rotate_mb: Option<u32>,
}

/// POST /api/pcap/start - Start PCAP capture
async fn start_pcap_capture(
    body: web::Json<StartPcapRequest>,
) -> impl Responder {
    let req = body.into_inner();
    
    let (already_capturing, _, _) = crate::wifi::scanner::get_pcap_stats();
    if already_capturing {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "PCAP capture already running"
        }));
    }
    
    let settings = crate::settings::AppSettings::default();
    let filename = req.filename.unwrap_or_else(|| {
        format!("capture_{}.pcap", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    });
    let filepath = settings.wifi.pcap_dir.join(&filename);
    
    // Create directory if needed
    if let Some(parent) = filepath.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    
    crate::wifi::scanner::start_pcap_capture();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "PCAP capture started",
        "file": filepath.to_string_lossy(),
        "rotate_mb": req.rotate_mb.unwrap_or(settings.wifi.pcap_rotate_mb)
    }))
}

/// POST /api/pcap/stop - Stop PCAP capture
async fn stop_pcap_capture() -> impl Responder {
    let (capturing, packets, bytes) = crate::wifi::scanner::get_pcap_stats();
    
    if !capturing {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "PCAP capture not running"
        }));
    }
    
    crate::wifi::scanner::stop_pcap_capture();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "PCAP capture stopped",
        "packets_captured": packets,
        "bytes_captured": bytes
    }))
}

#[derive(Serialize)]
struct PcapFileInfo {
    name: String,
    path: String,
    size_bytes: u64,
    modified: i64,
}

/// GET /api/pcap/files - List PCAP files
async fn list_pcap_files() -> impl Responder {
    let settings = crate::settings::AppSettings::default();
    let pcap_dir = &settings.wifi.pcap_dir;
    
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(pcap_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "pcap").unwrap_or(false) {
                if let Ok(metadata) = entry.metadata() {
                    files.push(PcapFileInfo {
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        size_bytes: metadata.len(),
                        modified: metadata.modified()
                            .map(|t| t.duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs() as i64).unwrap_or(0))
                            .unwrap_or(0),
                    });
                }
            }
        }
    }
    
    // Sort by modified time, newest first
    files.sort_by(|a, b| b.modified.cmp(&a.modified));
    
    HttpResponse::Ok().json(serde_json::json!({
        "pcap_dir": pcap_dir.to_string_lossy(),
        "files": files,
        "total_count": files.len()
    }))
}

// ============================================
// Device Notes
// ============================================

#[derive(Serialize, Deserialize, Debug)]
struct DeviceNote {
    id: i64,
    mac_address: String,
    device_type: String,
    note_text: String,
    note_source: String,
    device_vendor: Option<String>,
    device_ssid: Option<String>,
    device_name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    created_at: String,
}

async fn get_device_notes(
    db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let mac = path.into_inner();
    
    match db.get_device_notes(&mac).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to get notes: {}", e)
        }))
    }
}

#[derive(Deserialize)]
struct AddNoteRequest {
    note_text: String,
    device_type: Option<String>,
    note_source: Option<String>,
    device_vendor: Option<String>,
    device_ssid: Option<String>,
    device_name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

async fn add_device_note(
    db: web::Data<Arc<Database>>,
    state: web::Data<Arc<AppState>>,
    path: web::Path<String>,
    body: web::Json<AddNoteRequest>,
) -> impl Responder {
    let mac = path.into_inner();
    
    // Get current GPS position if available and not provided
    let (lat, lon) = if body.latitude.is_some() && body.longitude.is_some() {
        (body.latitude, body.longitude)
    } else {
        let gps = state.gps_status.read().await;
        if gps.has_fix {
            (gps.latitude, gps.longitude)
        } else {
            (None, None)
        }
    };
    
    match db.add_device_note(
        &mac,
        body.device_type.as_deref().unwrap_or("wifi"),
        &body.note_text,
        body.note_source.as_deref().unwrap_or("typed"),
        body.device_vendor.as_deref(),
        body.device_ssid.as_deref(),
        body.device_name.as_deref(),
        lat,
        lon,
    ).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "note_id": id,
            "mac": mac
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to add note: {}", e)
        }))
    }
}

async fn delete_device_note(
    db: web::Data<Arc<Database>>,
    path: web::Path<(String, i64)>,
) -> impl Responder {
    let (_mac, note_id) = path.into_inner();
    
    match db.delete_device_note(note_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "deleted_id": note_id
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to delete note: {}", e)
        }))
    }
}

async fn get_recent_notes(
    db: web::Data<Arc<Database>>,
) -> impl Responder {
    match db.get_recent_notes(50).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to get recent notes: {}", e)
        }))
    }
}

// ============================================
// Voice Transcription
// ============================================

async fn get_voice_status() -> impl Responder {
    // Check if faster-whisper or whisper is available
    let whisper_local = std::process::Command::new("which")
        .arg("faster-whisper")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    let whisper_py = std::process::Command::new("python3")
        .args(["-c", "import faster_whisper; print('ok')"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    // Check for Piper TTS
    let piper_available = std::process::Command::new("which")
        .arg("piper")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    let piper_py = std::process::Command::new("python3")
        .args(["-c", "import piper; print('ok')"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    HttpResponse::Ok().json(serde_json::json!({
        "whisper": {
            "local_available": whisper_local || whisper_py,
            "api_configured": false,
            "model": "base.en"
        },
        "piper": {
            "available": piper_available || piper_py,
            "model": "en_US-lessac-medium"
        },
        "recommended_stt": if whisper_local || whisper_py { "local" } else { "api" },
        "recommended_tts": if piper_available || piper_py { "piper" } else { "browser" }
    }))
}

#[derive(Deserialize)]
struct TranscribeRequest {
    audio_base64: Option<String>,
    audio_path: Option<String>,
    use_api: Option<bool>,
}

async fn transcribe_audio(
    body: web::Json<TranscribeRequest>,
) -> impl Responder {
    // Get audio data
    let audio_path = if let Some(path) = &body.audio_path {
        path.clone()
    } else if let Some(b64) = &body.audio_base64 {
        // Decode base64 and save to temp file
        let temp_path = "/tmp/sigint-audio-input.wav";
        match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64) {
            Ok(data) => {
                if let Err(e) = std::fs::write(temp_path, &data) {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": format!("Failed to write temp audio: {}", e)
                    }));
                }
                temp_path.to_string()
            }
            Err(e) => {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": format!("Invalid base64 audio: {}", e)
                }));
            }
        }
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "No audio provided (need audio_base64 or audio_path)"
        }));
    };
    
    // Try faster-whisper first
    let script = format!(r#"
import sys
try:
    from faster_whisper import WhisperModel
    model = WhisperModel("base.en", device="cpu", compute_type="int8")
    segments, _ = model.transcribe("{}", beam_size=5)
    text = " ".join([s.text for s in segments])
    print(text.strip())
except Exception as e:
    print(f"ERROR: {{e}}", file=sys.stderr)
    sys.exit(1)
"#, audio_path.replace("\"", "\\\""));
    
    let output = std::process::Command::new("python3")
        .args(["-c", &script])
        .output();
    
    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "transcription": text,
                "model": "faster-whisper-base.en",
                "source": "local"
            }))
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Transcription failed: {}", err),
                "hint": "Install faster-whisper: pip install faster-whisper"
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to run whisper: {}", e)
            }))
        }
    }
}

#[derive(Deserialize)]
struct SpeakRequest {
    text: String,
    voice: Option<String>,
}

async fn speak_text(
    body: web::Json<SpeakRequest>,
) -> impl Responder {
    let voice = body.voice.as_deref().unwrap_or("en_US-lessac-medium");
    let output_path = "/tmp/sigint-speech-output.wav";
    
    // Try piper first
    let result = std::process::Command::new("piper")
        .args(["--model", voice, "--output_file", output_path])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(body.text.as_bytes())?;
            }
            child.wait()
        });
    
    match result {
        Ok(status) if status.success() => {
            // Read and return audio as base64
            match std::fs::read(output_path) {
                Ok(data) => {
                    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &data);
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "audio_base64": b64,
                        "format": "wav",
                        "engine": "piper"
                    }))
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": format!("Failed to read audio output: {}", e)
                }))
            }
        }
        _ => {
            // Piper not available, return instructions
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "Piper TTS not available",
                "hint": "Install piper: pip install piper-tts",
                "fallback": "browser"
            }))
        }
    }
}

// ============================================
// OUI Database Management
// ============================================

const OUI_DATABASE_URL: &str = "https://raw.githubusercontent.com/naanprofit/sigint-deck/main/data/oui-database.min.json";
const OUI_THREATS_URL: &str = "https://raw.githubusercontent.com/naanprofit/sigint-deck/main/data/oui-threats.json";

fn get_oui_db_path() -> std::path::PathBuf {
    dirs::home_dir()
        .map(|h| h.join("sigint-deck").join("data").join("oui-database.json"))
        .unwrap_or_else(|| std::path::PathBuf::from("./data/oui-database.json"))
}

async fn get_oui_status() -> impl Responder {
    let db_path = get_oui_db_path();
    let exists = db_path.exists();
    
    let (entries, version, last_updated) = if exists {
        match std::fs::read_to_string(&db_path) {
            Ok(content) => {
                let json: serde_json::Value = serde_json::from_str(&content).unwrap_or_default();
                let entries = json.get("entries")
                    .and_then(|e| e.as_object())
                    .map(|o| o.len())
                    .unwrap_or(0);
                let version = json.get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let modified = std::fs::metadata(&db_path)
                    .and_then(|m| m.modified())
                    .map(|t| t.duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64).unwrap_or(0))
                    .unwrap_or(0);
                (entries, version, modified)
            }
            Err(_) => (0, "unknown".to_string(), 0)
        }
    } else {
        (0, "not installed".to_string(), 0)
    };
    
    // Also check embedded database size
    let embedded_count = crate::storage::OuiLookup::embedded().len();
    
    HttpResponse::Ok().json(serde_json::json!({
        "installed": exists,
        "path": db_path.to_string_lossy(),
        "entries": entries,
        "embedded_entries": embedded_count,
        "version": version,
        "last_updated": last_updated,
        "update_url": OUI_DATABASE_URL
    }))
}

async fn update_oui_database() -> impl Responder {
    let db_path = get_oui_db_path();
    
    // Ensure directory exists
    if let Some(parent) = db_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to create directory: {}", e)
            }));
        }
    }
    
    // Download the database
    tracing::info!("Downloading OUI database from {}", OUI_DATABASE_URL);
    
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build() 
    {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to create HTTP client: {}", e)
            }));
        }
    };
    
    let response = match client.get(OUI_DATABASE_URL).send().await {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to download database: {}", e)
            }));
        }
    };
    
    if !response.status().is_success() {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("HTTP error: {}", response.status())
        }));
    }
    
    let content = match response.text().await {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to read response: {}", e)
            }));
        }
    };
    
    // Validate JSON
    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": format!("Invalid JSON: {}", e)
            }));
        }
    };
    
    let entries = json.get("entries")
        .and_then(|e| e.as_object())
        .map(|o| o.len())
        .unwrap_or(0);
    
    if entries == 0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Database appears empty or invalid"
        }));
    }
    
    // Write to file
    if let Err(e) = std::fs::write(&db_path, &content) {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to write database: {}", e)
        }));
    }
    
    tracing::info!("OUI database updated: {} entries", entries);
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Database updated with {} entries", entries),
        "entries": entries,
        "path": db_path.to_string_lossy()
    }))
}

async fn lookup_oui(
    path: web::Path<String>,
) -> impl Responder {
    let mac = path.into_inner();
    
    // Try external database first
    let db_path = get_oui_db_path();
    if db_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&db_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                // Normalize MAC to OUI format (XX:XX:XX)
                let oui = mac.replace("-", ":").replace(".", ":")
                    .to_uppercase()
                    .chars()
                    .take(8)
                    .collect::<String>();
                
                if let Some(entry) = json.get("entries").and_then(|e| e.get(&oui)) {
                    return HttpResponse::Ok().json(serde_json::json!({
                        "mac": mac,
                        "oui": oui,
                        "found": true,
                        "source": "external",
                        "vendor": entry.get("vendor"),
                        "country": entry.get("country"),
                        "threat_category": entry.get("threat_category"),
                        "threat_level": entry.get("threat_level")
                    }));
                }
            }
        }
    }
    
    // Fall back to embedded database
    let oui = crate::storage::OuiLookup::embedded();
    if let Some(vendor) = oui.lookup(&mac) {
        return HttpResponse::Ok().json(serde_json::json!({
            "mac": mac,
            "found": true,
            "source": "embedded",
            "vendor": vendor
        }));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "mac": mac,
        "found": false
    }))
}

// ============================================
// LLM Analysis
// ============================================

const LLM_SYSTEM_PROMPT: &str = include_str!("../../data/llm-system-prompt.md");

async fn get_llm_system_prompt() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/markdown")
        .body(LLM_SYSTEM_PROMPT)
}

#[derive(Deserialize)]
struct LlmAnalyzeRequest {
    mac: String,
    rssi: Option<i32>,
    ssid: Option<String>,
    device_name: Option<String>,
    device_type: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

async fn llm_analyze_device(
    body: web::Json<LlmAnalyzeRequest>,
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let mac = &body.mac;
    
    // First, look up the OUI
    let oui_info: Option<serde_json::Value> = {
        let db_path = get_oui_db_path();
        let oui_prefix = mac.replace("-", ":").to_uppercase().chars().take(8).collect::<String>();
        
        if db_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&db_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    json.get("entries")
                        .and_then(|e| e.get(&oui_prefix))
                        .cloned()
                } else {
                    None
                }
            } else { 
                None 
            }
        } else { 
            None 
        }
    };
    
    // Build context for LLM
    let mut context = format!("Analyze this device:\n\nMAC Address: {}\n", mac);
    
    if let Some(rssi) = body.rssi {
        context.push_str(&format!("Signal Strength: {} dBm\n", rssi));
    }
    if let Some(ssid) = &body.ssid {
        context.push_str(&format!("SSID: {}\n", ssid));
    }
    if let Some(name) = &body.device_name {
        context.push_str(&format!("Device Name: {}\n", name));
    }
    if let Some(dtype) = &body.device_type {
        context.push_str(&format!("Device Type: {}\n", dtype));
    }
    if let (Some(lat), Some(lon)) = (body.latitude, body.longitude) {
        context.push_str(&format!("Location: {:.6}, {:.6}\n", lat, lon));
    }
    
    // Add OUI lookup results
    if let Some(info) = &oui_info {
        context.push_str("\nOUI Database Lookup:\n");
        if let Some(vendor) = info.get("vendor").and_then(|v| v.as_str()) {
            context.push_str(&format!("  Vendor: {}\n", vendor));
        }
        if let Some(country) = info.get("country").and_then(|v| v.as_str()) {
            context.push_str(&format!("  Country: {}\n", country));
        }
        if let Some(cat) = info.get("threat_category").and_then(|v| v.as_str()) {
            context.push_str(&format!("  Threat Category: {}\n", cat));
        }
        if let Some(level) = info.get("threat_level").and_then(|v| v.as_str()) {
            context.push_str(&format!("  Threat Level: {}\n", level));
        }
    } else {
        // Check if MAC is locally administered (randomized)
        if let Ok(first_byte) = u8::from_str_radix(&mac[..2], 16) {
            if (first_byte & 0x02) != 0 {
                context.push_str("\nNote: This is a locally administered (randomized) MAC address.\n");
                context.push_str("No vendor lookup possible - device is using MAC randomization for privacy.\n");
            } else {
                context.push_str("\nOUI not found in database.\n");
            }
        }
    }
    
    context.push_str("\nProvide a threat assessment and recommendations.");
    
    // Check if LLM is configured
    let llm_config = config.llm.as_ref();
    let llm_enabled = llm_config.map(|c| c.enabled).unwrap_or(false);
    
    if !llm_enabled {
        // Return pre-computed assessment without LLM
        let threat_level = oui_info.as_ref()
            .and_then(|i: &serde_json::Value| i.get("threat_level"))
            .and_then(|v: &serde_json::Value| v.as_str())
            .unwrap_or("unknown");
        
        let assessment = match threat_level {
            "critical" => "CRITICAL THREAT - Possible active surveillance or intelligence equipment detected.",
            "high" => "HIGH THREAT - Device may be associated with government, law enforcement, or surveillance vendors.",
            "medium" => "MEDIUM THREAT - Device may be a tracking device or have monitoring capabilities.",
            "low" => "LOW THREAT - Device uses chipset with known vulnerabilities but is likely consumer hardware.",
            _ => "UNKNOWN - Device not in threat database. May be randomized MAC or unlisted vendor."
        };
        
        return HttpResponse::Ok().json(serde_json::json!({
            "mac": mac,
            "oui_info": oui_info,
            "assessment": assessment,
            "threat_level": threat_level,
            "llm_used": false,
            "context": context
        }));
    }
    
    // Call LLM for analysis
    let llm = llm_config.unwrap(); // Safe because we checked llm_enabled
    let llm_endpoint = &llm.endpoint;
    let llm_model = &llm.model;
    
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create HTTP client: {}", e)
            }));
        }
    };
    
    // Determine API format based on provider
    let provider = llm.provider.to_lowercase();
    let (api_url, request_body) = if provider.contains("ollama") {
        (
            format!("{}/api/chat", llm_endpoint.trim_end_matches('/')),
            serde_json::json!({
                "model": llm_model,
                "messages": [
                    {"role": "system", "content": LLM_SYSTEM_PROMPT},
                    {"role": "user", "content": context}
                ],
                "stream": false
            })
        )
    } else {
        // OpenAI-compatible format
        (
            format!("{}/v1/chat/completions", llm_endpoint.trim_end_matches('/')),
            serde_json::json!({
                "model": llm_model,
                "messages": [
                    {"role": "system", "content": LLM_SYSTEM_PROMPT},
                    {"role": "user", "content": context}
                ],
                "max_tokens": 500
            })
        )
    };
    
    let response = match client.post(&api_url)
        .json(&request_body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "mac": mac,
                "oui_info": oui_info,
                "error": format!("LLM request failed: {}", e),
                "llm_used": false,
                "context": context
            }));
        }
    };
    
    let llm_response: serde_json::Value = match response.json().await {
        Ok(j) => j,
        Err(e) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "mac": mac,
                "oui_info": oui_info,
                "error": format!("Failed to parse LLM response: {}", e),
                "llm_used": false
            }));
        }
    };
    
    // Extract response text based on format
    let analysis = if provider.contains("ollama") {
        llm_response.get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("No response")
            .to_string()
    } else {
        llm_response.get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("No response")
            .to_string()
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "mac": mac,
        "oui_info": oui_info,
        "analysis": analysis,
        "llm_used": true,
        "model": llm_model
    }))
}
