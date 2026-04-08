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
            .route("/wifi/reset-monitor", web::post().to(reset_wifi_monitor))
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
            // RayHunter IMSI Catcher Detection
            .route("/rayhunter/status", web::get().to(get_rayhunter_status))
            .route("/rayhunter/start-recording", web::post().to(rayhunter_start_recording))
            .route("/rayhunter/stop-recording", web::post().to(rayhunter_stop_recording))
            // SDR (Software Defined Radio)
            .route("/sdr/status", web::get().to(get_sdr_status))
            .route("/sdr/rtl433/devices", web::get().to(get_rtl433_devices))
            .route("/sdr/rtl433/start", web::post().to(start_rtl433))
            .route("/sdr/rtl433/stop", web::post().to(stop_rtl433))
            .route("/sdr/spectrum/scan", web::post().to(scan_spectrum))
            .route("/sdr/cellular/towers", web::get().to(get_cell_towers))
            .route("/sdr/cellular/scan", web::post().to(scan_cell_towers))
            .route("/sdr/drone/signals", web::get().to(get_drone_signals))
            .route("/sdr/drone/scan", web::post().to(scan_drones))
            .route("/sdr/drone/scan/full", web::post().to(scan_drones_full))
            .route("/sdr/drone/emi", web::post().to(scan_drone_emi))
            .route("/sdr/drone/start", web::post().to(start_drone_monitor))
            .route("/sdr/drone/stop", web::post().to(stop_drone_monitor))
            // SDR Presets
            .route("/sdr/presets", web::get().to(get_preset_lists))
            .route("/sdr/presets/{list_id}", web::get().to(get_preset_list))
            .route("/sdr/presets", web::post().to(create_preset_list))
            .route("/sdr/presets/{list_id}", web::delete().to(delete_preset_list))
            .route("/sdr/presets/{list_id}/add", web::post().to(add_preset))
            .route("/sdr/presets/{list_id}/{preset_id}", web::delete().to(remove_preset))
            .route("/sdr/presets/search", web::get().to(search_presets))
            .route("/sdr/presets/favorites", web::get().to(get_favorite_presets))
            // SDR Radio Reception
            .route("/sdr/radio/tune", web::post().to(tune_radio))
            .route("/sdr/radio/stop", web::post().to(stop_radio))
            .route("/sdr/radio/status", web::get().to(get_radio_status))
            .route("/sdr/radio/stream", web::get().to(stream_radio_audio))
            // TSCM Bug Detection
            .route("/sdr/tscm/sweep", web::post().to(start_tscm_sweep))
            .route("/sdr/tscm/status", web::get().to(get_tscm_status))
            .route("/sdr/tscm/stop", web::post().to(stop_tscm_sweep))
            .route("/sdr/tscm/threats", web::get().to(get_tscm_threats))
            // LLM Analysis
            .route("/llm/analyze-device", web::post().to(llm_analyze_device))
            .route("/llm/system-prompt", web::get().to(get_llm_system_prompt))
            // Contact Log (unified device history)
            .route("/contacts", web::get().to(get_contacts))
            .route("/contacts/export", web::get().to(export_contacts))
            .route("/contacts/{mac}", web::get().to(get_contact_detail))
            .route("/contacts/{mac}/timeline", web::get().to(get_contact_timeline))
            .route("/database/stats", web::get().to(get_database_stats))
            // Device Silencing
            .route("/devices/{mac}/silence", web::post().to(silence_device_alerts))
            .route("/devices/{mac}/unsilence", web::post().to(unsilence_device_alerts))
            .route("/devices/silenced", web::get().to(get_silenced_devices))
            // Ham Radio - Morse Decoder
            .route("/sdr/morse/start", web::post().to(start_morse_decoder))
            .route("/sdr/morse/stop", web::post().to(stop_morse_decoder))
            .route("/sdr/morse/status", web::get().to(get_morse_status))
            // SIEM Log
            .route("/siem/events", web::get().to(siem_get_events))
            .route("/siem/events", web::post().to(siem_add_event))
            .route("/siem/search", web::get().to(siem_search_events))
            .route("/siem/stats", web::get().to(siem_get_stats))
            .route("/siem/prune", web::post().to(siem_prune_logs))
            .route("/siem/export", web::get().to(siem_export_events))
            .route("/siem/forward/config", web::get().to(siem_get_forward_config))
            .route("/siem/forward/config", web::post().to(siem_set_forward_config))
            // Sentinel Mode
            .route("/sentinel/start", web::post().to(sentinel_start))
            .route("/sentinel/stop", web::post().to(sentinel_stop))
            .route("/sentinel/status", web::get().to(sentinel_status))
            // Threat Watchlist
            .route("/watchlist", web::get().to(watchlist_list))
            .route("/watchlist", web::post().to(watchlist_add))
            .route("/watchlist/{id}", web::delete().to(watchlist_remove))
            // Advanced SDR - Multi-device & Antenna Array
            .route("/sdr/devices/all", web::get().to(get_sdr_devices))
            .route("/sdr/antenna/config", web::get().to(get_antenna_config))
            .route("/sdr/antenna/add", web::post().to(add_antenna_position))
            .route("/sdr/antenna/{id}", web::delete().to(delete_antenna_position))
            // Sentinel Mode (continuous monitoring)
            .route("/sentinel/start", web::post().to(sentinel_start))
            .route("/sentinel/stop", web::post().to(sentinel_stop))
            .route("/sentinel/status", web::get().to(sentinel_status))
            // Threat Watchlist
            .route("/watchlist", web::get().to(watchlist_list))
            .route("/watchlist", web::post().to(watchlist_add))
            .route("/watchlist/{id}", web::delete().to(watchlist_remove))
            // TTS Alerts (browser-based)
            .route("/alerts/tts/config", web::get().to(get_tts_alert_config))
            .route("/alerts/tts/config", web::post().to(set_tts_alert_config))
            .route("/alerts/tts/pending", web::get().to(get_pending_tts_alerts))
            .route("/tts/generate", web::post().to(generate_tts_wav))
            // Legal
            .route("/legal", web::get().to(get_legal))
            .route("/legal/accept", web::post().to(accept_legal))
            .route("/legal/status", web::get().to(legal_status))
            // Soundboard
            .route("/soundboard/clips", web::get().to(soundboard_list_clips))
            .route("/soundboard/clips", web::post().to(soundboard_upload_clip))
            .route("/soundboard/clips/{id}", web::delete().to(soundboard_delete_clip))
            .route("/soundboard/clips/{id}/play", web::post().to(soundboard_play_clip))
            .route("/soundboard/clips/{id}/transmit", web::post().to(soundboard_transmit_clip))
            .route("/soundboard/clips/{id}/stream", web::get().to(soundboard_stream_clip))
            // Fast Food / Commercial RF
            .route("/fastfood/database", web::get().to(fastfood_get_database))
            .route("/fastfood/scan", web::post().to(fastfood_scan))
            .route("/fastfood/signals", web::get().to(fastfood_get_signals))
            // ML Inference
            .route("/ml/status", web::get().to(ml_get_status))
            .route("/ml/classify", web::post().to(ml_classify_signal))
            .route("/ml/features", web::post().to(ml_extract_features))
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
    
    // Try iw first (always available via sudo), fall back to iwconfig
    let (iw_ok, iw_out) = run_cmd("sudo", &["/usr/sbin/iw", "dev", interface, "info"]);
    let (mode, is_up) = if iw_ok {
        let mode = if iw_out.contains("type monitor") { "monitor" }
            else if iw_out.contains("type managed") { "managed" }
            else { "unknown" };
        let is_up = iw_out.contains("channel");
        (mode, is_up)
    } else {
        // Fallback to iwconfig
        match std::process::Command::new("iwconfig").arg(interface).output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mode = if stdout.contains("Mode:Monitor") { "monitor" }
                    else if stdout.contains("Mode:Managed") { "managed" }
                    else { "unknown" };
                let is_up = !stdout.contains("Not-Associated") && !stdout.contains("off/any");
                (mode, is_up)
            }
            Err(_) => ("error", false),
        }
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

/// Run a command and return (success, combined stdout+stderr)
fn run_cmd(cmd: &str, args: &[&str]) -> (bool, String) {
    match std::process::Command::new(cmd).args(args).output() {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let combined = format!("{}{}", stdout.trim(), if stderr.is_empty() { String::new() } else { format!(" | {}", stderr.trim()) });
            (out.status.success(), combined)
        }
        Err(e) => (false, format!("Command not found or failed to execute: {}", e)),
    }
}

/// Set WiFi interface mode (requires sudo)
async fn set_wifi_mode(
    config: web::Data<Arc<Config>>,
    body: web::Json<WifiModeRequest>,
) -> impl Responder {
    let interface = &config.wifi.interface;
    let mode = body.mode.to_lowercase();
    let mut steps: Vec<serde_json::Value> = Vec::new();

    if mode != "monitor" && mode != "managed" {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Mode must be 'monitor' or 'managed'"
        }));
    }

    // Check interface exists
    let (ok, out) = run_cmd("ip", &["link", "show", interface]);
    steps.push(serde_json::json!({"step": "check_interface", "cmd": format!("ip link show {}", interface), "ok": ok, "output": out}));
    if !ok {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": format!("Interface '{}' not found. Plug in a USB WiFi adapter that supports monitor mode.", interface),
            "steps": steps
        }));
    }

    // Check if adapter supports monitor mode (get phy for this interface, then check supported modes)
    let (_, phy_out) = run_cmd("sudo", &["/usr/sbin/iw", "dev", interface, "info"]);
    let phy_name = phy_out.lines()
        .find(|l| l.contains("wiphy"))
        .and_then(|l| l.split_whitespace().last())
        .map(|n| format!("phy{}", n))
        .unwrap_or_default();
    let (ok, out) = if !phy_name.is_empty() {
        run_cmd("sudo", &["/usr/sbin/iw", &phy_name, "info"])
    } else {
        run_cmd("sudo", &["/usr/sbin/iw", "list"])
    };
    let supports_monitor = out.contains("* monitor");
    steps.push(serde_json::json!({"step": "check_monitor_support", "cmd": format!("iw {} info", if phy_name.is_empty() { "list".to_string() } else { phy_name }), "ok": ok, "supports_monitor": supports_monitor}));
    if !supports_monitor {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": format!("Interface '{}' does not support monitor mode. Use an adapter like Alfa AWUS036ACHM or RT5572.", interface),
            "steps": steps
        }));
    }

    // Release interface from NetworkManager if present (safe, won't kill SSH)
    let (nm_ok, nm_out) = run_cmd("sudo", &["nmcli", "device", "set", interface, "managed", "no"]);
    steps.push(serde_json::json!({"step": "release_interface", "cmd": format!("nmcli device set {} managed no", interface), "ok": nm_ok, "output": nm_out}));

    // Bring interface down
    let (ok, out) = run_cmd("sudo", &["ip", "link", "set", interface, "down"]);
    steps.push(serde_json::json!({"step": "interface_down", "cmd": format!("sudo ip link set {} down", interface), "ok": ok, "output": out}));
    if !ok {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to bring {} down: {}. Is sudo passwordless? Add 'deck ALL=(ALL) NOPASSWD: /usr/sbin/iw,/usr/sbin/ip,/usr/sbin/airmon-ng' to /etc/sudoers.d/sigint", interface, out),
            "steps": steps
        }));
    }

    // Set mode
    let (ok, out) = run_cmd("sudo", &["/usr/sbin/iw", "dev", interface, "set", "type", &mode]);
    steps.push(serde_json::json!({"step": "set_mode", "cmd": format!("sudo iw dev {} set type {}", interface, mode), "ok": ok, "output": out}));
    if !ok {
        // Bring interface back up before returning error
        let (_, _) = run_cmd("sudo", &["ip", "link", "set", interface, "up"]);
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": format!("Failed to set {} mode on {}: {}", mode, interface, out),
            "steps": steps
        }));
    }

    // Bring interface up
    let (ok, out) = run_cmd("sudo", &["ip", "link", "set", interface, "up"]);
    steps.push(serde_json::json!({"step": "interface_up", "cmd": format!("sudo ip link set {} up", interface), "ok": ok, "output": out}));
    if !ok {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": false,
            "error": format!("Mode set but failed to bring {} up: {}", interface, out),
            "steps": steps
        }));
    }

    // Verify mode was actually set (try iw first, fall back to iwconfig)
    let (v_ok, verify_out) = run_cmd("sudo", &["/usr/sbin/iw", "dev", interface, "info"]);
    let actual_mode = if !v_ok {
        // Fallback to iwconfig
        let (_, iw_out) = run_cmd("iwconfig", &[interface]);
        if iw_out.contains("Mode:Monitor") { "monitor" }
        else if iw_out.contains("Mode:Managed") { "managed" }
        else { "unknown" }
    } else if verify_out.contains("type monitor") { "monitor" }
    else if verify_out.contains("type managed") { "managed" }
    else { "unknown" };
    steps.push(serde_json::json!({"step": "verify", "cmd": format!("iwconfig {}", interface), "actual_mode": actual_mode, "output": verify_out}));

    let success = actual_mode == mode;
    HttpResponse::Ok().json(serde_json::json!({
        "success": success,
        "interface": interface,
        "mode": actual_mode,
        "requested_mode": mode,
        "error": if !success { Some(format!("Mode is '{}' after switch attempt, expected '{}'", actual_mode, mode)) } else { None::<String> },
        "steps": steps
    }))
}

/// POST /api/wifi/reset-monitor - Reset wlan1 to monitor mode via helper script
async fn reset_wifi_monitor(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let interface = &config.wifi.interface;
    let (ok, out) = run_cmd("sudo", &["/usr/local/bin/sigint-monitor-mode", interface]);
    HttpResponse::Ok().json(serde_json::json!({
        "success": ok,
        "interface": interface,
        "output": out.trim(),
        "hint": if ok { "Monitor mode set. Restart sigint-pi to resume scanning." } else { "Failed. Check USB adapter is plugged in." }
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

/// Sanitize LLM endpoint URL: strip fragments (#...) and trailing slashes
fn sanitize_llm_url(url: &str) -> String {
    let url = url.trim();
    // Strip fragment (#/ or #anything)
    let url = if let Some(idx) = url.find('#') { &url[..idx] } else { url };
    url.trim_end_matches('/').to_string()
}

/// Read LLM config from saved config file (not in-memory)
fn read_llm_config_from_disk() -> Option<crate::config::LlmConfig> {
    let config_paths = [
        dirs::home_dir().map(|h| h.join("sigint-deck").join("config.toml")).unwrap_or_default(),
        dirs::home_dir().map(|h| h.join("sigint-pi").join("config.toml")).unwrap_or_default(),
        std::path::PathBuf::from("./config.toml"),
    ];
    for path in &config_paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(config) = toml::from_str::<crate::config::Config>(&content) {
                return config.llm;
            }
        }
    }
    None
}

/// POST /api/settings/llm/test - Test LLM connection
/// Reads from SAVED config file so it reflects the most recently saved settings
pub async fn test_llm_connection(
    _config: web::Data<Arc<crate::config::Config>>,
) -> impl Responder {
    // Read from disk so we get the latest saved settings, not the startup config
    let llm_config = match read_llm_config_from_disk() {
        Some(c) if c.enabled => c,
        Some(_) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "LLM is disabled in settings. Enable it and save first."
            }));
        }
        None => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": "LLM not configured. Set the endpoint and save first."
            }));
        }
    };
    
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build() {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::Ok().json(serde_json::json!({
                "success": false, "error": format!("HTTP client error: {}", e)
            }));
        }
    };
    
    let base_url = sanitize_llm_url(&llm_config.endpoint);
    let provider = llm_config.provider.to_lowercase();
    
    // Determine the right test URL based on provider
    let url = if provider == "ollama" || base_url.contains("11434") {
        format!("{}/api/tags", base_url)
    } else if provider == "lmstudio" || base_url.contains("1234") {
        format!("{}/v1/models", base_url)
    } else if provider == "llamacpp" || base_url.contains("8080") {
        // llama.cpp uses /v1/models or /health
        format!("{}/v1/models", base_url)
    } else {
        format!("{}/v1/models", base_url)
    };
    
    tracing::info!("Testing LLM connection: provider={}, url={}", provider, url);
    
    let mut req = client.get(&url);
    if let Some(ref api_key) = llm_config.api_key {
        if !api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }
    }
    
    match req.send().await {
        Ok(response) => {
            let status = response.status();
            let body_text = response.text().await.unwrap_or_default();
            if status.is_success() {
                // Try to extract model names from response
                let models: Vec<String> = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
                    json.get("data").and_then(|d| d.as_array())
                        .or_else(|| json.get("models").and_then(|m| m.as_array()))
                        .map(|arr| arr.iter().filter_map(|m| {
                            m.get("id").or(m.get("name")).and_then(|v| v.as_str()).map(String::from)
                        }).collect())
                        .unwrap_or_default()
                } else { vec![] };
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": format!("Connected to {} at {}", provider, base_url),
                    "models": models,
                    "status": status.as_u16()
                }))
            } else {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": false,
                    "error": format!("LLM API returned status {} - {}", status, &body_text[..body_text.len().min(200)]),
                    "url_tested": url,
                    "status": status.as_u16()
                }))
            }
        }
        Err(e) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to connect to {}: {}", url, e),
                "hint": "Check that the LLM server is running and the URL is correct"
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
                        "has_api_key": config.get("llm")
                            .and_then(|l| l.get("api_key"))
                            .and_then(|v| v.as_str())
                            .map(|k| !k.is_empty())
                            .unwrap_or(false),
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
            // Preserve existing LLM fields that UI might not send
            let existing_llm = existing_table.get("llm")
                .and_then(|v| v.as_table()).cloned()
                .unwrap_or_default();
            let mut llm_table = existing_llm;
            if let Some(obj) = llm.as_object() {
                if let Some(enabled) = obj.get("enabled").and_then(|v| v.as_bool()) {
                    llm_table.insert("enabled".to_string(), toml::Value::Boolean(enabled));
                }
                if let Some(provider) = obj.get("provider").and_then(|v| v.as_str()) {
                    llm_table.insert("provider".to_string(), toml::Value::String(provider.to_string()));
                }
                if let Some(endpoint) = obj.get("endpoint").and_then(|v| v.as_str()) {
                    // Sanitize URL: strip fragments (#/) and trailing slashes
                    llm_table.insert("endpoint".to_string(), toml::Value::String(sanitize_llm_url(endpoint)));
                }
                if let Some(model) = obj.get("model").and_then(|v| v.as_str()) {
                    llm_table.insert("model".to_string(), toml::Value::String(model.to_string()));
                }
                if let Some(api_key) = obj.get("api_key").and_then(|v| v.as_str()) {
                    if !api_key.is_empty() {
                        llm_table.insert("api_key".to_string(), toml::Value::String(api_key.to_string()));
                    }
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

/// POST /api/pcap/start - Start PCAP capture (accepts empty body)
async fn start_pcap_capture(
    body: Option<web::Json<StartPcapRequest>>,
) -> impl Responder {
    let req = body.map(|b| b.into_inner()).unwrap_or(StartPcapRequest {
        filename: None,
        rotate_mb: None,
    });
    
    let (already_capturing, _, _) = crate::wifi::scanner::get_pcap_stats();
    if already_capturing {
        return HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "PCAP capture already running"
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
        return HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "PCAP capture already stopped",
            "packets_captured": packets,
            "bytes_captured": bytes
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

fn find_venv_python() -> Option<String> {
    let home = std::env::var("HOME").unwrap_or_default();
    for dir in &["sigint-deck", "sigint-pi", "sigint-clockworkpi"] {
        let venv_py = format!("{}/{}/venv/bin/python", home, dir);
        if std::path::Path::new(&venv_py).exists() {
            return Some(venv_py);
        }
    }
    // Also check relative to current working directory
    if std::path::Path::new("./venv/bin/python").exists() {
        return Some("./venv/bin/python".to_string());
    }
    None
}

async fn get_voice_status() -> impl Responder {
    // Check if whisper server is running (preferred method)
    let whisper_url = std::env::var("WHISPER_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string());
    let whisper_server = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .ok()
        .and_then(|client| {
            // Use blocking since this is a quick health check
            std::thread::spawn(move || {
                tokio::runtime::Runtime::new().ok().and_then(|rt| {
                    rt.block_on(async {
                        client.get(format!("{}/health", whisper_url)).send().await.ok()
                            .filter(|r| r.status().is_success())
                    })
                })
            }).join().ok().flatten()
        })
        .is_some();
    
    // Check venv paths (SteamOS and other installs use a venv)
    let venv_python = find_venv_python();

    // Fallback: check if faster-whisper is available via CLI or Python
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
    
    let whisper_venv = venv_python.as_ref().map(|py| {
        std::process::Command::new(py)
            .args(["-c", "import faster_whisper; print('ok')"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }).unwrap_or(false);

    let whisper_available = whisper_server || whisper_local || whisper_py || whisper_venv;
    
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

    let piper_venv = venv_python.as_ref().map(|py| {
        std::process::Command::new(py)
            .args(["-c", "import piper; print('ok')"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }).unwrap_or(false);
    
    HttpResponse::Ok().json(serde_json::json!({
        "whisper": {
            "local_available": whisper_available,
            "server_running": whisper_server,
            "api_configured": false,
            "model": "tiny"
        },
        "piper": {
            "available": piper_available || piper_py || piper_venv,
            "model": "en_US-lessac-medium"
        },
        "recommended_stt": if whisper_available { "local" } else { "api" },
        "recommended_tts": if piper_available || piper_py || piper_venv { "piper" } else { "browser" }
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
    
    // Try whisper server first (running on port 5000)
    let whisper_url = std::env::var("WHISPER_URL").unwrap_or_else(|_| "http://127.0.0.1:5000".to_string());
    
    // Read the audio file and encode as base64
    let audio_data = match std::fs::read(&audio_path) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to read audio file: {}", e)
            }));
        }
    };
    let audio_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &audio_data);
    
    // Try the whisper server
    let client = match reqwest::Client::builder().timeout(std::time::Duration::from_secs(30)).build() {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to create HTTP client: {}", e)
            }));
        }
    };
    
    let response = client
        .post(format!("{}/transcribe", whisper_url))
        .json(&serde_json::json!({ "audio": audio_b64 }))
        .send()
        .await;
    
    match response {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<serde_json::Value>().await {
                Ok(json) => {
                    let text = json.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "transcription": text,
                        "model": "faster-whisper",
                        "source": "local-server"
                    }))
                }
                Err(e) => {
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": format!("Failed to parse whisper response: {}", e)
                    }))
                }
            }
        }
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Whisper server error ({}): {}", status, body),
                "hint": "Start whisper server: systemctl --user start whisper-server"
            }))
        }
        Err(e) => {
            // Whisper server not running - return helpful message
            HttpResponse::Ok().json(serde_json::json!({
                "success": false,
                "error": format!("Whisper server not available: {}", e),
                "hint": "Start whisper server: systemctl --user start whisper-server"
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
    let voice_name = body.voice.as_deref().unwrap_or("en_US-lessac-medium");
    let output_path = "/tmp/sigint-speech-output.wav";
    
    // Try piper - check venv first, then system
    let piper_cmd = find_venv_python()
        .map(|py| {
            let venv_dir = std::path::Path::new(&py).parent().unwrap().to_path_buf();
            let piper_bin = venv_dir.join("piper");
            if piper_bin.exists() {
                piper_bin.to_string_lossy().to_string()
            } else {
                "piper".to_string()
            }
        })
        .unwrap_or_else(|| "piper".to_string());

    // Resolve model path - check models/piper/ directory for .onnx files
    let home = std::env::var("HOME").unwrap_or_default();
    let model_path = ["sigint-deck", "sigint-pi", "sigint-clockworkpi"]
        .iter()
        .map(|d| format!("{}/{}/models/piper/{}.onnx", home, d, voice_name))
        .find(|p| std::path::Path::new(p).exists())
        .unwrap_or_else(|| voice_name.to_string());

    let result = std::process::Command::new(&piper_cmd)
        .args(["--model", &model_path, "--output_file", output_path])
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
// ============================================
// RayHunter IMSI Catcher Detection
// ============================================

async fn get_rayhunter_status(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let rh_config = config.rayhunter.clone().unwrap_or_default();
    let client = crate::rayhunter::RayHunterClient::new(rh_config);
    let status = client.get_full_status().await;
    HttpResponse::Ok().json(status)
}

async fn rayhunter_start_recording(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let rh_config = config.rayhunter.clone().unwrap_or_default();
    let base_url = rh_config.api_url.trim_end_matches('/');
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(5)).build().unwrap();
    match client.post(&format!("{}/api/start-recording", base_url)).send().await {
        Ok(r) if r.status().is_success() => HttpResponse::Ok().json(serde_json::json!({"success": true, "message": "Recording started"})),
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({"success": false, "error": format!("HTTP {}", r.status())})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success": false, "error": format!("{}", e)})),
    }
}

async fn rayhunter_stop_recording(
    config: web::Data<Arc<Config>>,
) -> impl Responder {
    let rh_config = config.rayhunter.clone().unwrap_or_default();
    let base_url = rh_config.api_url.trim_end_matches('/');
    let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(5)).build().unwrap();
    match client.post(&format!("{}/api/stop-recording", base_url)).send().await {
        Ok(r) if r.status().is_success() => HttpResponse::Ok().json(serde_json::json!({"success": true, "message": "Recording stopped"})),
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({"success": false, "error": format!("HTTP {}", r.status())})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success": false, "error": format!("{}", e)})),
    }
}

// ============================================
// SDR (Software Defined Radio) Endpoints
// ============================================

use crate::sdr::{SdrCapabilities, SdrEvent};
use crate::sdr::rtl433::{RfDevice, Rtl433Config};
use crate::sdr::spectrum::{SpectrumConfig, FrequencyBand};
use crate::sdr::cellular::{CellularConfig, CellTower};
use crate::sdr::drone::{DroneDetectorConfig, DroneSignal};

async fn get_sdr_status() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    HttpResponse::Ok().json(serde_json::json!({
        "available": caps.any_available(),
        "capabilities": {
            "rtl_sdr": caps.rtl_sdr,
            "rtl_433": caps.rtl_433,
            "rtl_power": caps.rtl_power,
            "hackrf": caps.hackrf,
            "limesdr": caps.limesdr,
            "kalibrate": caps.kalibrate
        },
        "devices": caps.devices
    }))
}

// Global state for rtl_433 (in production, use proper state management)
use std::sync::Mutex;
use once_cell::sync::Lazy;

static RTL433_DEVICES: Lazy<Mutex<Vec<RfDevice>>> = Lazy::new(|| Mutex::new(Vec::new()));
static RTL433_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

async fn get_rtl433_devices() -> impl Responder {
    let devices = RTL433_DEVICES.lock().unwrap();
    HttpResponse::Ok().json(&*devices)
}

async fn start_rtl433() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.rtl_433 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "rtl_433 not installed",
            "hint": "Run scripts/install-sdr.sh to install SDR tools"
        }));
    }
    
    let mut running = RTL433_RUNNING.lock().unwrap();
    if *running {
        return HttpResponse::Ok().json(serde_json::json!({
            "status": "already_running"
        }));
    }
    
    *running = true;
    
    // Start rtl_433 in background
    tokio::spawn(async move {
        let output = tokio::process::Command::new("rtl_433")
            .args(&[
                "-F", "json",
                "-M", "time:utc",
                "-M", "level",
                "-f", "433.92M",
                "-f", "315M",
                "-H", "30",
            ])
            .stdout(std::process::Stdio::piped())
            .spawn();
        
        if let Ok(mut child) = output {
            if let Some(stdout) = child.stdout.take() {
                use tokio::io::{BufReader, AsyncBufReadExt};
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                        if let Some(device) = RfDevice::from_json(&json) {
                            let mut devices = RTL433_DEVICES.lock().unwrap();
                            // Update or add device
                            if let Some(existing) = devices.iter_mut().find(|d| d.id == device.id) {
                                existing.last_seen = device.last_seen;
                                existing.count += 1;
                                existing.rssi = device.rssi;
                            } else {
                                devices.push(device);
                            }
                            // Keep only last 100 devices
                            if devices.len() > 100 {
                                devices.remove(0);
                            }
                        }
                    }
                }
            }
        }
        
        let mut running = RTL433_RUNNING.lock().unwrap();
        *running = false;
    });
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "frequencies": ["433.92 MHz", "315 MHz"]
    }))
}

async fn stop_rtl433() -> impl Responder {
    // Kill rtl_433 process
    let _ = std::process::Command::new("pkill")
        .args(&["-f", "rtl_433"])
        .output();
    
    let mut running = RTL433_RUNNING.lock().unwrap();
    *running = false;
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopped"
    }))
}

#[derive(Deserialize)]
struct SpectrumScanRequest {
    start_mhz: Option<u32>,
    end_mhz: Option<u32>,
    band: Option<String>,
}

async fn scan_spectrum(body: web::Json<SpectrumScanRequest>) -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.rtl_sdr && !caps.hackrf {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No SDR hardware detected",
            "hint": "Connect an RTL-SDR or HackRF device",
            "detected_devices": caps.devices.iter().map(|d| &d.name).collect::<Vec<_>>()
        }));
    }
    
    let (start_mhz, end_mhz) = if let Some(band_name) = &body.band {
        let alias_map: std::collections::HashMap<&str, &str> = [
            ("ism315", "ISM 315"), ("ism433", "ISM 433"), ("ism868", "ISM 868"), ("ism915", "ISM 915"),
            ("wifi24", "WiFi 2.4GHz"), ("wifi", "WiFi 2.4GHz"),
            ("gsm850", "Cellular 850"), ("gsm1900", "Cellular 1900"),
            ("lte700", "Cellular 700"), ("cellular700", "Cellular 700"),
            ("cellular850", "Cellular 850"), ("cellular1900", "Cellular 1900"),
            ("gps", "GPS L1"), ("gpsl1", "GPS L1"),
            ("drones", "Drone 5.8GHz"), ("drone", "Drone 5.8GHz"), ("fpv", "Drone 5.8GHz"),
        ].iter().cloned().collect();
        
        let bands = FrequencyBand::common_bands();
        let normalize = |s: &str| s.to_lowercase().replace(" ", "").replace("-", "").replace(".", "");
        let search_normalized = normalize(band_name);
        let resolved_name = alias_map.get(search_normalized.as_str()).map(|s| s.to_string());
        
        if let Some(band) = bands.iter().find(|b| {
            let name_normalized = normalize(&b.name);
            if let Some(ref resolved) = resolved_name {
                b.name == *resolved
            } else {
                name_normalized.contains(&search_normalized) || search_normalized.contains(&name_normalized)
            }
        }) {
            ((band.start_hz / 1_000_000) as u32, (band.end_hz / 1_000_000) as u32)
        } else {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Unknown band",
                "hint": format!("Band '{}' not found. Use: ism315, ism433, ism868, ism915, wifi24, gsm850, gsm1900, lte700, gps, drones", band_name),
                "available_bands": bands.iter().map(|b| &b.name).collect::<Vec<_>>()
            }));
        }
    } else {
        (body.start_mhz.unwrap_or(400), body.end_mhz.unwrap_or(500))
    };
    
    // Validate range against available hardware
    let rtl_max: u32 = 1766;
    let rtl_min: u32 = 24;
    let can_rtl = caps.rtl_sdr && start_mhz >= rtl_min && end_mhz <= rtl_max;
    let can_hackrf = caps.hackrf;
    
    if !can_rtl && !can_hackrf {
        if caps.rtl_sdr && (start_mhz < rtl_min || end_mhz > rtl_max) {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Range {}-{} MHz exceeds RTL-SDR limits (24-1766 MHz). No HackRF available.", start_mhz, end_mhz),
                "hint": "Connect a HackRF One for frequencies above 1.7 GHz or below 24 MHz"
            }));
        }
        return HttpResponse::BadRequest().json(serde_json::json!({ "error": "No suitable SDR hardware for this range" }));
    }
    
    let resolve = crate::sdr::resolve_sdr_command;
    
    // Try primary method, fall back to raw IQ capture + FFT if it fails
    let result = if can_rtl {
        // PRIMARY: rtl_power
        let rtl_power_cmd = resolve("rtl_power");
        let primary = tokio::process::Command::new(&rtl_power_cmd)
            .args(&["-f", &format!("{}M:{}M:100k", start_mhz, end_mhz), "-i", "1", "-1", "-g", "40"])
            .output().await;
        
        match &primary {
            Ok(o) if o.status.success() && !o.stdout.is_empty() => primary,
            _ => {
                // FALLBACK: rtl_sdr raw IQ capture + python FFT
                tracing::warn!("rtl_power failed, falling back to rtl_sdr + FFT");
                let center_hz = ((start_mhz as u64 + end_mhz as u64) / 2) * 1_000_000;
                let bw = ((end_mhz - start_mhz) as u64).max(1) * 1_000_000;
                let sample_rate = bw.min(2_400_000).max(1_000_000);
                let num_samples = sample_rate * 2; // 2 seconds of data
                let script = format!(
                    r#"import sys,struct,math
data=open('/tmp/sigint_iq.bin','rb').read()
n=len(data)//2
iq=[complex(data[i*2]-127.5,data[i*2+1]-127.5) for i in range(min(n,{fft_size}))]
N=len(iq)
if N==0: sys.exit(1)
# Simple DFT power spectrum
import cmath
half=N//2
powers=[]
step={sr}/N
for k in range(N):
    s=sum(iq[j]*cmath.exp(-2j*cmath.pi*k*j/N) for j in range(0,N,max(1,N//256)))
    p=20*math.log10(abs(s)/N+1e-10)
    powers.append(p)
# Output in rtl_power CSV format
hz_low={center}-{sr}//2
for i in range(0,len(powers),max(1,len(powers)//128)):
    hz=hz_low+int(i*step)
    chunk=powers[i:i+max(1,len(powers)//128)]
    avg=sum(chunk)/len(chunk) if chunk else -100
    print(f"2026-01-01, 00:00:00, {{hz}}, {{hz+int(step*len(chunk))}}, {{int(step)}}, {{len(chunk)}}, {{avg:.1f}}")
"#, fft_size=2048, sr=sample_rate, center=center_hz);
                let _ = tokio::fs::write("/tmp/sigint_fft.py", &script).await;
                tokio::process::Command::new("sh")
                    .args(&["-c", &format!(
                        "{} -f {} -s {} -n {} /tmp/sigint_iq.bin 2>/dev/null && python3 /tmp/sigint_fft.py",
                        resolve("rtl_sdr"), center_hz, sample_rate, num_samples
                    )]).output().await
            }
        }
    } else {
        // HackRF path
        let hackrf_sweep_cmd = resolve("hackrf_sweep");
        let primary = tokio::process::Command::new(&hackrf_sweep_cmd)
            .args(&["-f", &format!("{}:{}", start_mhz, end_mhz), "-w", "100000", "-1"])
            .output().await;
        
        match &primary {
            Ok(o) if o.status.success() && !o.stdout.is_empty() => primary,
            _ => {
                // FALLBACK: hackrf_transfer raw IQ capture + python FFT
                tracing::warn!("hackrf_sweep failed, falling back to hackrf_transfer + FFT");
                let center_hz = ((start_mhz as u64 + end_mhz as u64) / 2) * 1_000_000;
                let sample_rate: u64 = 8_000_000;
                let num_samples = sample_rate * 2;
                let script = format!(
                    r#"import sys,struct,math
data=open('/tmp/sigint_hrf_iq.bin','rb').read()
n=len(data)//2
iq=[complex((data[i*2]^0x80)-128,(data[i*2+1]^0x80)-128) for i in range(min(n,8192))]
N=len(iq)
if N==0: sys.exit(1)
import cmath
powers=[]
step={sr}/N
for k in range(N):
    s=sum(iq[j]*cmath.exp(-2j*cmath.pi*k*j/N) for j in range(0,N,max(1,N//512)))
    p=20*math.log10(abs(s)/N+1e-10)
    powers.append(p)
hz_low={center}-{sr}//2
for i in range(0,len(powers),max(1,len(powers)//128)):
    hz=hz_low+int(i*step)
    chunk=powers[i:i+max(1,len(powers)//128)]
    avg=sum(chunk)/len(chunk) if chunk else -100
    print(f"2026-01-01, 00:00:00, {{hz}}, {{hz+int(step*len(chunk))}}, {{int(step)}}, {{len(chunk)}}, {{avg:.1f}}")
"#, sr=sample_rate, center=center_hz);
                let _ = tokio::fs::write("/tmp/sigint_hrf_fft.py", &script).await;
                tokio::process::Command::new("sh")
                    .args(&["-c", &format!(
                        "{} -r /tmp/sigint_hrf_iq.bin -f {} -s {} -n {} 2>/dev/null && python3 /tmp/sigint_hrf_fft.py",
                        resolve("hackrf_transfer"), center_hz, sample_rate, num_samples
                    )]).output().await
            }
        }
    };
    
    match result {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let points: Vec<serde_json::Value> = stdout.lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 7 {
                        let hz_low: u64 = parts[2].trim().parse().ok()?;
                        let hz_step: u64 = parts[4].trim().parse().ok()?;
                        let powers: Vec<f64> = parts[6..].iter()
                            .filter_map(|p| p.trim().parse().ok())
                            .collect();
                        Some(serde_json::json!({
                            "start_hz": hz_low,
                            "step_hz": hz_step,
                            "powers": powers
                        }))
                    } else {
                        None
                    }
                })
                .collect();
            
            let tool = if can_rtl { "RTL-SDR" } else { "HackRF" };
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "start_mhz": start_mhz,
                "end_mhz": end_mhz,
                "tool": tool,
                "data": points
            }))
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stdout = String::from_utf8_lossy(&out.stdout);
            tracing::error!("Spectrum scan failed: stderr={}, stdout_len={}", stderr, stdout.len());
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Scan failed: {}", if stderr.is_empty() { "No output from SDR tool" } else { &stderr }),
                "hardware": caps.devices.iter().map(|d| &d.name).collect::<Vec<_>>()
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to execute scan: {}", e)
            }))
        }
    }
}

static CELL_TOWERS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));

async fn get_cell_towers() -> impl Responder {
    let towers = CELL_TOWERS.lock().unwrap();
    HttpResponse::Ok().json(&*towers)
}

async fn scan_cell_towers() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.kalibrate {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "kalibrate-rtl not installed",
            "hint": "Install kalibrate-rtl for cellular tower scanning"
        }));
    }
    
    // Scan GSM bands
    let bands = vec![("GSM850", "850"), ("PCS", "1900")];
    let mut found_towers = Vec::new();
    
    for (band_arg, band_name) in bands {
        let output = tokio::process::Command::new("kal")
            .args(&["-s", band_arg])
            .output()
            .await
            .or_else(|_| {
                std::process::Command::new("kalibrate-rtl")
                    .args(&["-s", band_arg])
                    .output()
            });
        
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                if line.contains("chan:") && line.contains("power:") {
                    // Parse: "chan: 128 (869.2MHz + 45Hz) power: 123456.78"
                    if let Some(tower) = parse_kal_line(line, band_name) {
                        found_towers.push(tower);
                    }
                }
            }
        }
    }
    
    // Update global state
    let mut towers = CELL_TOWERS.lock().unwrap();
    *towers = found_towers.clone();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "towers_found": found_towers.len(),
        "towers": found_towers
    }))
}

fn parse_kal_line(line: &str, band: &str) -> Option<serde_json::Value> {
    // Parse: "chan: 128 (869.2MHz + 45Hz) power: 123456.78"
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }
    
    let arfcn: u32 = parts.get(1)?.parse().ok()?;
    let freq_str = line.split('(').nth(1)?.split("MHz").next()?.trim();
    let freq_mhz: f64 = freq_str.parse().ok()?;
    let power_str = line.split("power:").nth(1)?.trim();
    let power: f64 = power_str.parse().ok()?;
    
    Some(serde_json::json!({
        "band": band,
        "arfcn": arfcn,
        "frequency_mhz": freq_mhz,
        "power": power
    }))
}

static DRONE_SIGNALS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));
static DETECTED_DRONES: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));
static DRONE_COOLDOWN: Lazy<Mutex<std::collections::HashMap<String, u64>>> = Lazy::new(|| Mutex::new(std::collections::HashMap::new()));
static DRONE_ALERT_QUEUE: Lazy<Mutex<Vec<(String, String, Option<String>)>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn drain_drone_alerts() -> Vec<(String, String, Option<String>)> {
    let mut q = DRONE_ALERT_QUEUE.lock().unwrap();
    q.drain(..).collect()
}

pub fn register_drone_wifi(
    mac: &str,
    ssid: Option<&str>,
    rssi: i32,
    channel: u8,
    manufacturer: crate::sdr::drone_signatures::DroneManufacturer,
    method: crate::sdr::drone_signatures::WifiDetectionMethod,
) {
    register_drone_wifi_ex(mac, ssid, rssi, channel, manufacturer, method, None);
}

pub fn register_drone_wifi_ex(
    mac: &str,
    ssid: Option<&str>,
    rssi: i32,
    channel: u8,
    manufacturer: crate::sdr::drone_signatures::DroneManufacturer,
    method: crate::sdr::drone_signatures::WifiDetectionMethod,
    product_type: Option<u8>,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    {
        let mut cooldown = DRONE_COOLDOWN.lock().unwrap();
        if let Some(&last) = cooldown.get(mac) {
            if now - last < 120 {
                let mut drones = DETECTED_DRONES.lock().unwrap();
                if let Some(d) = drones.iter_mut().find(|d| d.get("mac").and_then(|v| v.as_str()) == Some(mac)) {
                    d["last_seen"] = serde_json::json!(now);
                    d["rssi"] = serde_json::json!(rssi);
                }
                return;
            }
        }
        cooldown.insert(mac.to_string(), now);
    }
    let mfr_label = manufacturer.label();
    let method_str = format!("{:?}", method);
    let threat = if rssi > -30 { "HIGH" } else if rssi > -50 { "MEDIUM" } else { "LOW" };
    
    let (controller_name, likely_drones) = crate::sdr::drone_signatures::identify_drone_type(ssid, product_type);
    let drone_type_str = if !likely_drones.is_empty() {
        likely_drones.join(" / ")
    } else {
        "Unknown Model".to_string()
    };
    
    let entry = serde_json::json!({
        "mac": mac, "ssid": ssid, "rssi": rssi, "channel": channel,
        "manufacturer": mfr_label, "detection_method": method_str,
        "threat_level": threat, "source": "wifi",
        "controller_model": controller_name,
        "likely_drone_models": likely_drones,
        "drone_type": drone_type_str,
        "first_seen": now, "last_seen": now,
    });
    {
        let mut drones = DETECTED_DRONES.lock().unwrap();
        if let Some(d) = drones.iter_mut().find(|d| d.get("mac").and_then(|v| v.as_str()) == Some(mac)) {
            *d = entry.clone();
        } else {
            drones.push(entry.clone());
        }
        if drones.len() > 100 { drones.remove(0); }
    }
    tracing::warn!(
        "DRONE DETECTED via WiFi: {} ({}) MAC={} RSSI={} CH={} method={} controller={} likely={}",
        mfr_label, ssid.unwrap_or("-"), mac, rssi, channel, method_str,
        controller_name.as_deref().unwrap_or("?"), drone_type_str
    );
    
    let alert_msg = format!(
        "DRONE DETECTED: {} {} ({}) at {}dBm on ch{} — likely: {}",
        mfr_label, controller_name.as_deref().unwrap_or("controller"),
        ssid.unwrap_or("unknown"), rssi, channel, drone_type_str
    );
    let priority = if rssi > -30 { "Critical" } else if rssi > -50 { "High" } else { "Medium" };
    {
        let mut q = DRONE_ALERT_QUEUE.lock().unwrap();
        q.push((priority.to_string(), alert_msg, Some(mac.to_string())));
    }
}

pub fn register_drone_ble(
    mac: &str,
    name: Option<&str>,
    rssi: i32,
    manufacturer: crate::sdr::drone_signatures::DroneManufacturer,
    _remote_id: Option<&crate::sdr::drone_signatures::RemoteIdData>,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    {
        let mut cooldown = DRONE_COOLDOWN.lock().unwrap();
        if let Some(&last) = cooldown.get(mac) {
            if now - last < 120 {
                let mut drones = DETECTED_DRONES.lock().unwrap();
                if let Some(d) = drones.iter_mut().find(|d| d.get("mac").and_then(|v| v.as_str()) == Some(mac)) {
                    d["last_seen"] = serde_json::json!(now);
                    d["rssi"] = serde_json::json!(rssi);
                }
                return;
            }
        }
        cooldown.insert(mac.to_string(), now);
    }
    let mfr_label = manufacturer.label();
    let entry = serde_json::json!({
        "mac": mac, "name": name, "rssi": rssi,
        "manufacturer": mfr_label, "detection_method": "BLE",
        "threat_level": if rssi > -40 { "HIGH" } else if rssi > -60 { "MEDIUM" } else { "LOW" },
        "source": "ble", "first_seen": now, "last_seen": now,
    });
    {
        let mut drones = DETECTED_DRONES.lock().unwrap();
        if let Some(d) = drones.iter_mut().find(|d| d.get("mac").and_then(|v| v.as_str()) == Some(mac)) {
            *d = entry.clone();
        } else {
            drones.push(entry.clone());
        }
        if drones.len() > 100 { drones.remove(0); }
    }
    tracing::warn!("DRONE DETECTED via BLE: {} name={} MAC={} RSSI={}", mfr_label, name.unwrap_or("-"), mac, rssi);
    
    let alert_msg = format!(
        "DRONE DETECTED (BLE): {} ({}) at {}dBm",
        mfr_label, name.unwrap_or("unknown"), rssi
    );
    let priority = if rssi > -30 { "Critical" } else if rssi > -50 { "High" } else { "Medium" };
    {
        let mut q = DRONE_ALERT_QUEUE.lock().unwrap();
        q.push((priority.to_string(), alert_msg, Some(mac.to_string())));
    }
}

async fn get_drone_signals() -> impl Responder {
    let rf_signals = DRONE_SIGNALS.lock().unwrap().clone();
    let wifi_ble_drones = DETECTED_DRONES.lock().unwrap().clone();
    HttpResponse::Ok().json(serde_json::json!({
        "rf_signals": rf_signals,
        "wifi_ble_detections": wifi_ble_drones,
        "total": rf_signals.len() + wifi_ble_drones.len(),
    }))
}

async fn scan_drones() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.hackrf && !caps.rtl_sdr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No SDR hardware available for drone detection",
            "hint": "HackRF covers 2.4/5.8 GHz drone bands. RTL-SDR covers sub-GHz control (868/915 MHz Crossfire/ELRS)."
        }));
    }
    
    let mut found_signals = Vec::new();
    let mut bands_scanned = Vec::new();
    
    // USB power safety: pause between SDR operations to avoid brownouts on Pi
    let usb_settle = std::time::Duration::from_millis(1500);

    if caps.hackrf {
        // Scan 2.4 GHz band (HackRF only)
        if let Some(data) = tscm_scan_band(2400.0, 2500.0, true, false).await {
            if let Some(signal) = detect_drone_signal(&data, 2_400_000_000, 2_500_000_000, "2.4GHz") {
                found_signals.push(signal);
            }
            bands_scanned.push("2.4GHz");
        }
        tokio::time::sleep(usb_settle).await;
        
        // Scan 5.8 GHz band (HackRF only)
        if let Some(data) = tscm_scan_band(5650.0, 5950.0, true, false).await {
            if let Some(signal) = detect_drone_signal(&data, 5_650_000_000, 5_950_000_000, "5.8GHz") {
                found_signals.push(signal);
            }
            bands_scanned.push("5.8GHz");
        }
        tokio::time::sleep(usb_settle).await;
        
        // Also scan sub-GHz with HackRF when no RTL-SDR available
        if !caps.rtl_sdr {
            if let Some(data) = tscm_scan_band(860.0, 930.0, true, false).await {
                if let Some(signal) = detect_drone_signal(&data, 860_000_000, 930_000_000, "868-915MHz") {
                    found_signals.push(signal);
                }
                bands_scanned.push("868-915MHz");
            }
            tokio::time::sleep(usb_settle).await;
        }
    }
    
    // RTL-SDR: scan key drone bands
    if caps.rtl_sdr {
        if let Some(data) = tscm_scan_band(860.0, 930.0, false, true).await {
            if let Some(signal) = detect_drone_signal(&data, 860_000_000, 930_000_000, "868-915MHz") {
                found_signals.push(signal);
            }
            bands_scanned.push("868-915MHz");
        }
    }
    
    // Update global state
    let mut signals = DRONE_SIGNALS.lock().unwrap();
    *signals = found_signals.clone();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "signals_found": found_signals.len(),
        "signals": found_signals,
        "bands_scanned": bands_scanned,
        "hardware_note": if !caps.hackrf { "No HackRF - 2.4/5.8 GHz scanning unavailable. Scanning sub-GHz drone bands only." } else { "Full spectrum drone scan" }
    }))
}

fn detect_drone_signal(output: &str, start_hz: u64, end_hz: u64, band: &str) -> Option<serde_json::Value> {
    // Find strongest signal in the band
    let mut max_power = f64::NEG_INFINITY;
    let mut max_freq = 0u64;
    
    for line in output.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 {
            continue;
        }
        
        if let (Ok(hz_low), Ok(hz_bin)) = (
            parts[2].trim().parse::<u64>(),
            parts[4].trim().parse::<u64>(),
        ) {
            for (i, db_str) in parts[6..].iter().enumerate() {
                if let Ok(power_db) = db_str.trim().parse::<f64>() {
                    let freq = hz_low + (i as u64 * hz_bin);
                    if freq >= start_hz && freq <= end_hz && power_db > max_power {
                        max_power = power_db;
                        max_freq = freq;
                    }
                }
            }
        }
    }
    
    // Only report if signal is strong enough (above noise floor)
    if max_power > -60.0 {
        let freq_mhz = max_freq as f64 / 1_000_000.0;
        let (dist_min, dist_max, dist_desc) = estimate_distance(max_power, freq_mhz);
        Some(serde_json::json!({
            "band": band,
            "frequency_hz": max_freq,
            "frequency_mhz": freq_mhz,
            "power_db": max_power,
            "signal_type": if band == "5.8GHz" { "video" } else { "control" },
            "threat_level": if max_power > -40.0 { "high" } else if max_power > -50.0 { "medium" } else { "low" },
            "estimated_distance": dist_desc,
            "distance_min_m": dist_min,
            "distance_max_m": dist_max
        }))
    } else {
        None
    }
}

/// Analyze wideband spectrum data for periodic peaks that indicate motor EMI
/// Works with data from rtl_power (24-30 MHz) or hackrf_sweep (1-30 MHz)
/// Looks for evenly-spaced spectral peaks that match ESC switching harmonics
fn analyze_emi_spectrum_wideband(output: &str) -> Vec<serde_json::Value> {
    let mut detections = Vec::new();
    let mut power_by_freq: std::collections::HashMap<u64, f64> = std::collections::HashMap::new();
    
    for line in output.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 { continue; }
        
        if let (Ok(hz_low), Ok(hz_step)) = (
            parts[2].trim().parse::<u64>(),
            parts[4].trim().parse::<u64>(),
        ) {
            for (i, db_str) in parts[6..].iter().enumerate() {
                if let Ok(power_db) = db_str.trim().parse::<f64>() {
                    let freq_hz = hz_low + (i as u64 * hz_step);
                    power_by_freq.insert(freq_hz, power_db);
                }
            }
        }
    }
    
    if power_by_freq.is_empty() { return detections; }
    
    // Calculate noise floor
    let mut all_powers: Vec<f64> = power_by_freq.values().cloned().collect();
    all_powers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let noise_floor = all_powers[all_powers.len() / 2]; // median
    let threshold = noise_floor + 10.0; // 10 dB above noise floor
    
    // Find peaks above threshold
    let mut peaks: Vec<(u64, f64)> = power_by_freq.iter()
        .filter(|(_, &p)| p > threshold)
        .map(|(&f, &p)| (f, p))
        .collect();
    peaks.sort_by_key(|(f, _)| *f);
    
    if peaks.len() < 3 { return detections; }
    
    // Look for evenly-spaced peaks (harmonic series)
    // Try different fundamental spacings: 8k, 16k, 24k, 48k, 96k Hz
    let esc_fundamentals = vec![
        (8000u64, "Industrial 8kHz", "T-Motor/Hobbywing"),
        (16000, "DJI/Industrial 16kHz", "DJI stock ESC"),
        (24000, "BLHeli_S 24kHz", "Common hobby ESC"),
        (48000, "BLHeli 48kHz", "DShot-enabled ESC"),
        (96000, "BLHeli_32 96kHz", "High-performance ESC"),
    ];
    
    for (fundamental_hz, name, esc_type) in &esc_fundamentals {
        let mut harmonics_found = Vec::new();
        let mut total_power = 0.0;
        let tolerance = *fundamental_hz / 4; // 25% tolerance
        
        // Check many harmonics (for higher-order that fall in scan range)
        for n in 1..=2000u64 {
            let expected_hz = fundamental_hz * n;
            
            // Only check frequencies in our scan range
            if let Some(min_freq) = peaks.first().map(|(f, _)| *f) {
                if expected_hz < min_freq.saturating_sub(tolerance) { continue; }
            }
            if let Some(max_freq) = peaks.last().map(|(f, _)| *f) {
                if expected_hz > max_freq + tolerance { break; }
            }
            
            for (freq, power) in &peaks {
                if (*freq as i64 - expected_hz as i64).unsigned_abs() <= tolerance {
                    harmonics_found.push(serde_json::json!({
                        "harmonic": n,
                        "frequency_hz": *freq,
                        "frequency_mhz": *freq as f64 / 1_000_000.0,
                        "power_db": *power
                    }));
                    total_power += power;
                    break;
                }
            }
        }
        
        if harmonics_found.len() >= 3 {
            let avg_power = total_power / harmonics_found.len() as f64;
            let confidence = (harmonics_found.len() as f64 / 10.0).min(1.0);
            let (estimated_motors, estimated_distance) = if avg_power > noise_floor + 30.0 {
                (4, "< 10m")
            } else if avg_power > noise_floor + 20.0 {
                (4, "< 30m")
            } else {
                (2, "< 100m")
            };
            
            detections.push(serde_json::json!({
                "fundamental_khz": *fundamental_hz as f64 / 1000.0,
                "esc_type": esc_type,
                "signature_name": name,
                "harmonics_detected": harmonics_found.len(),
                "harmonics": harmonics_found,
                "average_power_db": avg_power,
                "noise_floor_db": noise_floor,
                "confidence": confidence,
                "estimated_motor_count": estimated_motors,
                "estimated_distance": estimated_distance,
                "threat_level": if confidence > 0.7 { "high" } else if confidence > 0.4 { "medium" } else { "low" }
            }));
        }
    }
    
    detections
}

// ============================================
// EMI / Motor Harmonic Detection
// ============================================

static EMI_SIGNALS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));
static COMBINED_DRONE_DETECTIONS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Scan for drone motor EMI (electronic noise from ESCs)
async fn scan_drone_emi() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.rtl_sdr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "RTL-SDR required for EMI detection",
            "hint": "EMI detection uses direct sampling mode to detect motor harmonics in VLF/LF bands"
        }));
    }
    
    // Motor EMI detection strategy:
    // - ESC PWM fundamentals: 8/16/24/48/96 kHz
    // - Their higher harmonics extend into RTL-SDR tunable range (24+ MHz)
    // - e.g. 16kHz * 1500th harmonic = 24 MHz, 96kHz * 250th = 24 MHz
    // - We scan 24-30 MHz looking for periodic spectral peaks
    // - Also try direct sampling (-E 2) if RTL-SDR supports it
    // - HackRF can scan from 1 MHz and cover low harmonics directly
    
    let mut emi_detections = Vec::new();
    let mut scan_method = "none";
    
    // Method 1: Try HackRF first (best for low freq EMI, 1-30 MHz)
    if caps.hackrf {
        if let Ok(out) = tokio::process::Command::new("hackrf_sweep")
            .args(&["-f", "1:30", "-w", "10000", "-1"])
            .output()
            .await
        {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                emi_detections = analyze_emi_spectrum_wideband(&stdout);
                scan_method = "hackrf_1_30mhz";
            }
        }
    }
    
    // Method 2: RTL-SDR direct sampling (if supported)
    if emi_detections.is_empty() && caps.rtl_sdr {
        let output = tokio::process::Command::new("rtl_power")
            .args(&["-f", "500k:2M:1k", "-i", "2", "-1", "-E", "2"])
            .output()
            .await;
        
        if let Ok(out) = output {
            if out.status.success() && !out.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if !stdout.trim().is_empty() {
                    emi_detections = analyze_emi_spectrum(&stdout);
                    scan_method = "rtl_direct_sampling";
                }
            }
        }
    }
    
    // Method 3: RTL-SDR normal mode scan 24-30 MHz (look for high-order harmonics)
    if emi_detections.is_empty() && caps.rtl_sdr {
        if let Ok(out) = tokio::process::Command::new("rtl_power")
            .args(&["-f", "24M:30M:1k", "-i", "2", "-1"])
            .output()
            .await
        {
            if out.status.success() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                emi_detections = analyze_emi_spectrum_wideband(&stdout);
                scan_method = "rtl_24_30mhz";
            }
        }
    }
    
    // Update global state
    {
        let mut signals = EMI_SIGNALS.lock().unwrap();
        *signals = emi_detections.clone();
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "emi_signals_found": emi_detections.len(),
        "emi_signals": emi_detections,
        "scan_method": scan_method,
        "detection_method": "motor_esc_harmonics",
        "description": "Detects electronic noise from drone ESC/motor switching frequencies"
    }))
}

/// Analyze EMI spectrum for motor harmonics
fn analyze_emi_spectrum(output: &str) -> Vec<serde_json::Value> {
    let mut detections = Vec::new();
    let mut power_by_freq: std::collections::HashMap<u64, f64> = std::collections::HashMap::new();
    
    // Parse spectrum data
    for line in output.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 {
            continue;
        }
        
        if let (Ok(hz_low), Ok(hz_step)) = (
            parts[2].trim().parse::<u64>(),
            parts[4].trim().parse::<u64>(),
        ) {
            for (i, db_str) in parts[6..].iter().enumerate() {
                if let Ok(power_db) = db_str.trim().parse::<f64>() {
                    let freq_hz = hz_low + (i as u64 * hz_step);
                    power_by_freq.insert(freq_hz, power_db);
                }
            }
        }
    }
    
    // Find peaks and check for harmonic patterns
    let mut peaks: Vec<(u64, f64)> = power_by_freq.iter()
        .filter(|(_, &p)| p > -50.0)  // Above threshold
        .map(|(&f, &p)| (f, p))
        .collect();
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Known ESC fundamental frequencies
    let esc_fundamentals = vec![
        (8000, "Industrial 8kHz", "T-Motor/Hobbywing"),
        (16000, "DJI/Industrial 16kHz", "DJI stock or industrial"),
        (24000, "BLHeli_S 24kHz", "Common hobby ESC"),
        (48000, "BLHeli 48kHz", "DShot-enabled ESC"),
        (96000, "BLHeli_32 96kHz", "High-performance ESC"),
    ];
    
    for (fundamental_hz, name, esc_type) in esc_fundamentals {
        let mut harmonics_found = Vec::new();
        let mut total_power = 0.0;
        
        // Check for harmonics (1x, 2x, 3x, 4x, 5x, 6x)
        for n in 1..=6u64 {
            let expected_hz = fundamental_hz * n;
            let tolerance = 500; // 500 Hz tolerance
            
            for (freq, power) in &peaks {
                if (*freq as i64 - expected_hz as i64).unsigned_abs() <= tolerance {
                    harmonics_found.push(serde_json::json!({
                        "harmonic": n,
                        "frequency_hz": *freq,
                        "frequency_khz": *freq as f64 / 1000.0,
                        "power_db": *power
                    }));
                    total_power += power;
                    break;
                }
            }
        }
        
        // Need at least 3 harmonics to confirm detection
        if harmonics_found.len() >= 3 {
            let avg_power = total_power / harmonics_found.len() as f64;
            let confidence = harmonics_found.len() as f64 / 6.0;
            
            // Estimate motor count and distance
            let (estimated_motors, estimated_distance) = if avg_power > -20.0 {
                (4, "< 5m")
            } else if avg_power > -30.0 {
                (4, "< 20m")
            } else if avg_power > -40.0 {
                (2, "< 50m")
            } else {
                (1, "< 100m")
            };
            
            detections.push(serde_json::json!({
                "fundamental_khz": fundamental_hz as f64 / 1000.0,
                "esc_type": esc_type,
                "signature_name": name,
                "harmonics_detected": harmonics_found.len(),
                "harmonics": harmonics_found,
                "average_power_db": avg_power,
                "confidence": confidence,
                "estimated_motor_count": estimated_motors,
                "estimated_distance": estimated_distance,
                "threat_level": if avg_power > -30.0 { "high" } else if avg_power > -40.0 { "medium" } else { "low" }
            }));
        }
    }
    
    detections
}

/// Full combined scan: RF + EMI
async fn scan_drones_full() -> impl Responder {
    let caps = SdrCapabilities::detect();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    
    let mut results = serde_json::json!({
        "success": true,
        "rf_scan": null,
        "emi_scan": null,
        "combined_detections": [],
        "detection_summary": { "rf_signals": 0, "emi_signals": 0, "confirmed_drones": 0 }
    });
    
    // === RF Scan ===
    let mut rf_signals = Vec::new();
    let mut rf_bands = Vec::new();
    
    if caps.hackrf {
        for (range, start_hz, end_hz, band, tool) in [
            ("2400:2500", 2_400_000_000u64, 2_500_000_000u64, "2.4GHz", "hackrf_sweep"),
            ("5650:5950", 5_650_000_000u64, 5_950_000_000u64, "5.8GHz", "hackrf_sweep"),
        ] {
            if let Ok(out) = tokio::process::Command::new(tool)
                .args(&["-f", range, "-w", "500000", "-1"]).output().await
            {
                if out.status.success() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if let Some(mut sig) = detect_drone_signal(&stdout, start_hz, end_hz, band) {
                        sig["first_seen"] = serde_json::json!(now);
                        sig["last_seen"] = serde_json::json!(now);
                        rf_signals.push(sig);
                    }
                    rf_bands.push(band);
                }
            }
        }
    }
    
    // RTL-SDR sub-GHz bands
    if caps.rtl_sdr {
        for (range, start_hz, end_hz, band) in [
            ("860M:880M:10k", 860_000_000u64, 880_000_000u64, "868MHz"),
            ("905M:930M:10k", 905_000_000u64, 930_000_000u64, "915MHz"),
        ] {
            if let Ok(out) = tokio::process::Command::new("rtl_power")
                .args(&["-f", range, "-i", "1", "-1"]).output().await
            {
                if out.status.success() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if let Some(mut sig) = detect_drone_signal(&stdout, start_hz, end_hz, band) {
                        sig["first_seen"] = serde_json::json!(now);
                        sig["last_seen"] = serde_json::json!(now);
                        rf_signals.push(sig);
                    }
                    rf_bands.push(band);
                }
            }
        }
    }
    
    results["rf_scan"] = serde_json::json!({
        "success": !rf_bands.is_empty(),
        "signals": rf_signals.clone(),
        "bands_scanned": rf_bands
    });
    
    // === EMI Scan (multiple methods) ===
    let mut emi_signals = Vec::new();
    let mut emi_method = "none";
    
    // Try HackRF low-band first
    if caps.hackrf {
        if let Ok(out) = tokio::process::Command::new("hackrf_sweep")
            .args(&["-f", "1:30", "-w", "10000", "-1"]).output().await
        {
            if out.status.success() && !out.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                emi_signals = analyze_emi_spectrum_wideband(&stdout);
                if !emi_signals.is_empty() { emi_method = "hackrf_1_30mhz"; }
            }
        }
    }
    
    // Fallback: RTL-SDR 24-30 MHz
    if emi_signals.is_empty() && caps.rtl_sdr {
        if let Ok(out) = tokio::process::Command::new("rtl_power")
            .args(&["-f", "24M:30M:1k", "-i", "2", "-1"]).output().await
        {
            if out.status.success() && !out.stdout.is_empty() {
                let stdout = String::from_utf8_lossy(&out.stdout);
                emi_signals = analyze_emi_spectrum_wideband(&stdout);
                if !emi_signals.is_empty() { emi_method = "rtl_24_30mhz"; }
            }
        }
    }
    
    // Add timestamps to EMI detections
    for sig in &mut emi_signals {
        sig["first_seen"] = serde_json::json!(now);
        sig["last_seen"] = serde_json::json!(now);
    }
    
    results["emi_scan"] = serde_json::json!({
        "success": !emi_signals.is_empty() || emi_method != "none",
        "signals": emi_signals.clone(),
        "method": emi_method
    });
    
    // === Combine detections ===
    let mut combined = Vec::new();
    
    if !rf_signals.is_empty() && !emi_signals.is_empty() {
        combined.push(serde_json::json!({
            "id": format!("drone_confirmed_{}", now),
            "detection_method": "rf_and_emi", "confidence": 0.95,
            "rf_signals": rf_signals, "emi_signatures": emi_signals,
            "threat_level": "high", "first_seen": now, "last_seen": now,
            "description": "CONFIRMED: Drone detected via both RF and motor EMI"
        }));
    } else if !rf_signals.is_empty() {
        combined.push(serde_json::json!({
            "id": format!("drone_rf_{}", now),
            "detection_method": "rf_only", "confidence": 0.7,
            "rf_signals": rf_signals, "threat_level": "medium",
            "first_seen": now, "last_seen": now,
            "description": "Possible drone detected via RF transmission"
        }));
    } else if !emi_signals.is_empty() {
        combined.push(serde_json::json!({
            "id": format!("drone_emi_{}", now),
            "detection_method": "emi_only", "confidence": 0.6,
            "emi_signatures": emi_signals, "threat_level": "medium",
            "first_seen": now, "last_seen": now,
            "description": "Possible drone detected via motor EMI"
        }));
    }
    
    results["combined_detections"] = serde_json::json!(combined);
    results["detection_summary"] = serde_json::json!({
        "rf_signals": rf_signals.len(),
        "emi_signals": emi_signals.len(),
        "confirmed_drones": if !rf_signals.is_empty() && !emi_signals.is_empty() { 1 } else { 0 },
        "possible_drones": combined.len()
    });
    
    // Update global state
    { let mut s = DRONE_SIGNALS.lock().unwrap(); *s = rf_signals; }
    { let mut e = EMI_SIGNALS.lock().unwrap(); *e = emi_signals; }
    { let mut d = COMBINED_DRONE_DETECTIONS.lock().unwrap(); *d = combined; }
    
    HttpResponse::Ok().json(results)
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
    
    // Check if LLM is configured - read from disk for latest saved settings
    let disk_llm = read_llm_config_from_disk();
    let llm_config = disk_llm.as_ref().or(config.llm.as_ref());
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
    let llm_endpoint = sanitize_llm_url(&llm.endpoint);
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
            format!("{}/api/chat", llm_endpoint),
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
        // OpenAI-compatible format (llama.cpp, LM Studio, etc.)
        (
            format!("{}/v1/chat/completions", llm_endpoint),
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

// ===== Contact Log API Endpoints =====

#[derive(Deserialize)]
pub struct ContactsQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    search: Option<String>,
}

/// Get all contacts (unified device history)
pub async fn get_contacts(
    db: web::Data<Arc<Database>>,
    query: web::Query<ContactsQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(100).min(1000);
    let offset = query.offset.unwrap_or(0);
    let search = query.search.as_deref();
    
    match db.get_all_contacts(limit, offset, search).await {
        Ok(mut contacts) => {
            // Add OUI lookup for contacts without vendor info
            let oui = crate::storage::OuiLookup::embedded();
            for contact in &mut contacts {
                if contact.vendor.is_none() {
                    contact.vendor = oui.lookup(&contact.mac_address).map(|s| s.to_string());
                }
            }
            
            let count = db.get_contact_count(search).await.unwrap_or(0);
            HttpResponse::Ok().json(serde_json::json!({
                "contacts": contacts,
                "total": count,
                "limit": limit,
                "offset": offset
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch contacts: {}", e)
        }))
    }
}

/// Get detailed contact info with sighting history
pub async fn get_contact_detail(
    db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let mac = path.into_inner();
    
    match db.get_contact_detail(&mac).await {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Contact not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch contact detail: {}", e)
        }))
    }
}

/// Get contact timeline (sightings over time for map/chart)
pub async fn get_contact_timeline(
    db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let mac = path.into_inner();
    
    match db.get_contact_detail(&mac).await {
        Ok(Some(detail)) => {
            // Extract timeline data for visualization
            let timeline: Vec<serde_json::Value> = detail.sightings.iter().map(|s| {
                serde_json::json!({
                    "timestamp": s.timestamp.timestamp(),
                    "rssi": s.rssi,
                    "channel": s.channel,
                    "latitude": s.latitude,
                    "longitude": s.longitude
                })
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "mac": mac,
                "timeline": timeline,
                "first_seen": detail.first_seen.timestamp(),
                "last_seen": detail.last_seen.timestamp(),
                "total_sightings": detail.times_seen
            }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Contact not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to fetch timeline: {}", e)
        }))
    }
}

/// Export all contacts for backup
pub async fn export_contacts(
    db: web::Data<Arc<Database>>,
) -> impl Responder {
    match db.export_contacts().await {
        Ok(data) => HttpResponse::Ok()
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Content-Disposition", "attachment; filename=\"sigint-contacts-export.json\""))
            .json(data),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Export failed: {}", e)
        }))
    }
}

/// Get database statistics
pub async fn get_database_stats(
    db: web::Data<Arc<Database>>,
) -> impl Responder {
    // Get counts from various tables
    let wifi_count = db.get_contact_count(None).await.unwrap_or(0);
    let notes = db.get_recent_notes(1).await.unwrap_or_default();
    let descriptions = db.get_all_device_descriptions().await.unwrap_or_default();
    
    HttpResponse::Ok().json(serde_json::json!({
        "total_contacts": wifi_count,
        "ai_descriptions": descriptions.len(),
        "has_notes": !notes.is_empty(),
        "database_version": "1.0"
    }))
}

// ============================================
// Continuous Drone Monitoring
// ============================================

static DRONE_MONITOR_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static DRONE_SCAN_COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
static DRONE_LAST_SCAN: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

async fn start_drone_monitor() -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.hackrf && !caps.rtl_sdr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No SDR hardware available",
            "hint": "HackRF scans 2.4/5.8 GHz. RTL-SDR scans 868/915 MHz + EMI."
        }));
    }
    
    {
        let running = DRONE_MONITOR_RUNNING.lock().unwrap();
        if *running {
            return HttpResponse::Ok().json(serde_json::json!({
                "status": "already_running"
            }));
        }
    }
    
    {
        let mut running = DRONE_MONITOR_RUNNING.lock().unwrap();
        *running = true;
    }
    {
        let mut count = DRONE_SCAN_COUNT.lock().unwrap();
        *count = 0;
    }
    
    let has_hackrf = caps.hackrf;
    let has_rtl = caps.rtl_sdr;
    
    let mut bands_available = Vec::new();
    if has_hackrf { bands_available.extend(["2.4GHz", "5.8GHz"]); }
    if has_rtl { bands_available.extend(["868MHz", "915MHz", "EMI"]); }
    let bands_list = bands_available.clone();
    
    tokio::spawn(async move {
        // USB power safety: settle time between SDR operations prevents brownouts on Pi
        let usb_settle = tokio::time::Duration::from_millis(1500);
        let cmd_timeout = std::time::Duration::from_secs(15);

        while {
            let running = DRONE_MONITOR_RUNNING.lock().unwrap();
            *running
        } {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            
            // === HackRF scans (2.4 GHz + 5.8 GHz) ===
            if has_hackrf {
                for (range, start_hz, end_hz, band) in [
                    ("2400:2500", 2_400_000_000u64, 2_500_000_000u64, "2.4GHz"),
                    ("5650:5950", 5_650_000_000u64, 5_950_000_000u64, "5.8GHz"),
                ] {
                    if !{ *DRONE_MONITOR_RUNNING.lock().unwrap() } { return; }
                    
                    let sweep_result = tokio::time::timeout(cmd_timeout,
                        tokio::process::Command::new("hackrf_sweep")
                            .args(&["-f", range, "-w", "500000", "-1"])
                            .output()
                    ).await;
                    
                    let stdout_data = match sweep_result {
                        Ok(Ok(out)) => {
                            let s = String::from_utf8_lossy(&out.stdout).to_string();
                            if s.contains(',') { Some(s) } else { None }
                        }
                        _ => None,
                    };
                    
                    // Fallback: use hackrf_transfer if sweep fails (pipe error on some firmware)
                    let stdout_data = if stdout_data.is_none() {
                        tracing::warn!("hackrf_sweep failed for {}, trying hackrf_transfer fallback", band);
                        let center_hz = (start_hz + end_hz) / 2;
                        let sr: u64 = 8_000_000;
                        let iq_path = format!("/tmp/sigint_drone_hrf_{}.bin", band.replace('.', "_"));
                        let xfer = tokio::time::timeout(cmd_timeout,
                            tokio::process::Command::new("hackrf_transfer")
                                .args(&["-r", &iq_path, "-f", &center_hz.to_string(),
                                       "-s", &sr.to_string(), "-n", &(sr * 2).to_string()])
                                .output()
                        ).await;
                        if let Ok(Ok(out)) = xfer {
                            if out.status.success() {
                                if let Some(csv) = iq_to_csv(&iq_path, center_hz, sr, true).await {
                                    Some(csv)
                                } else { None }
                            } else { None }
                        } else { None }
                    } else {
                        stdout_data
                    };
                    
                    if let Some(ref data) = stdout_data {
                        if let Some(mut signal) = detect_drone_signal(data, start_hz, end_hz, band) {
                            signal["first_seen"] = serde_json::json!(now);
                            signal["last_seen"] = serde_json::json!(now);
                            
                            let mut signals = DRONE_SIGNALS.lock().unwrap();
                            if let Some(existing) = signals.iter_mut().find(|s| s.get("band") == Some(&serde_json::json!(band))) {
                                let fs = existing.get("first_seen").cloned().unwrap_or(serde_json::json!(now));
                                signal["first_seen"] = fs;
                                signal["sightings"] = serde_json::json!(
                                    existing.get("sightings").and_then(|v| v.as_u64()).unwrap_or(0) + 1
                                );
                                *existing = signal;
                            } else {
                                signal["sightings"] = serde_json::json!(1);
                                signals.push(signal);
                            }
                        }
                    }
                    
                    // USB settle between scans
                    tokio::time::sleep(usb_settle).await;
                }
            }
            
            // Settle between switching from HackRF to RTL-SDR
            if has_hackrf && has_rtl {
                tokio::time::sleep(usb_settle).await;
            }
            
            // === RTL-SDR scans (military UHF + sub-GHz + L-band + EMI) ===
            if has_rtl {
                for (range, start_hz, end_hz, band) in [
                    ("320M:400M:100k", 320_000_000u64, 400_000_000u64, "UHF-Mil"),
                    ("860M:930M:10k",  860_000_000u64, 930_000_000u64, "868-915MHz"),
                    ("1200M:1300M:50k", 1_200_000_000u64, 1_300_000_000u64, "L-band"),
                ] {
                    if !{ *DRONE_MONITOR_RUNNING.lock().unwrap() } { return; }
                    
                    let rtl_result = tokio::time::timeout(cmd_timeout,
                        tokio::process::Command::new("rtl_power")
                            .args(&["-f", range, "-i", "1", "-1", "-g", "40"])
                            .output()
                    ).await;
                    
                    if let Ok(Ok(out)) = rtl_result {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        if let Some(mut signal) = detect_drone_signal(&stdout, start_hz, end_hz, band) {
                            signal["first_seen"] = serde_json::json!(now);
                            signal["last_seen"] = serde_json::json!(now);
                            
                            let mut signals = DRONE_SIGNALS.lock().unwrap();
                            if let Some(existing) = signals.iter_mut().find(|s| s.get("band") == Some(&serde_json::json!(band))) {
                                let fs = existing.get("first_seen").cloned().unwrap_or(serde_json::json!(now));
                                signal["first_seen"] = fs;
                                signal["sightings"] = serde_json::json!(
                                    existing.get("sightings").and_then(|v| v.as_u64()).unwrap_or(0) + 1
                                );
                                *existing = signal;
                            } else {
                                signal["sightings"] = serde_json::json!(1);
                                signals.push(signal);
                            }
                        }
                    }
                    
                    // USB settle between RTL-SDR band scans
                    tokio::time::sleep(usb_settle).await;
                }
                
                // EMI scan (24-30 MHz, look for motor harmonics)
                if !{ *DRONE_MONITOR_RUNNING.lock().unwrap() } { return; }
                
                let emi_result = tokio::time::timeout(cmd_timeout,
                    tokio::process::Command::new("rtl_power")
                        .args(&["-f", "24M:30M:1k", "-i", "2", "-1", "-g", "40"])
                        .output()
                ).await;
                
                if let Ok(Ok(out)) = emi_result {
                    if out.status.success() {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let emi_results = analyze_emi_spectrum_wideband(&stdout);
                        if !emi_results.is_empty() {
                            let mut emi = EMI_SIGNALS.lock().unwrap();
                            for mut sig in emi_results {
                                sig["first_seen"] = serde_json::json!(now);
                                sig["last_seen"] = serde_json::json!(now);
                                
                                let sig_name = sig.get("signature_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                if let Some(existing) = emi.iter_mut().find(|e| {
                                    e.get("signature_name").and_then(|v| v.as_str()).unwrap_or("") == sig_name
                                }) {
                                    let fs = existing.get("first_seen").cloned().unwrap_or(serde_json::json!(now));
                                    sig["first_seen"] = fs;
                                    sig["sightings"] = serde_json::json!(
                                        existing.get("sightings").and_then(|v| v.as_u64()).unwrap_or(0) + 1
                                    );
                                    *existing = sig;
                                } else {
                                    sig["sightings"] = serde_json::json!(1);
                                    emi.push(sig);
                                }
                            }
                        }
                    }
                }
            }
            
            {
                let mut count = DRONE_SCAN_COUNT.lock().unwrap();
                *count += 1;
            }
            {
                let mut last = DRONE_LAST_SCAN.lock().unwrap();
                *last = now;
            }
            
            // Longer pause between full scan cycles
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "bands": bands_list,
        "scan_interval_secs": 3,
        "hardware": {
            "hackrf": has_hackrf,
            "rtl_sdr": has_rtl
        }
    }))
}

async fn stop_drone_monitor() -> impl Responder {
    let mut running = DRONE_MONITOR_RUNNING.lock().unwrap();
    *running = false;
    
    let scan_count = { *DRONE_SCAN_COUNT.lock().unwrap() };
    let signals = { DRONE_SIGNALS.lock().unwrap().len() };
    let emi = { EMI_SIGNALS.lock().unwrap().len() };
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopped",
        "total_scans": scan_count,
        "rf_signals_found": signals,
        "emi_signals_found": emi
    }))
}

// ============================================
// SDR Presets API
// ============================================

use crate::sdr::presets::{PresetManager, FrequencyPreset, PresetList, Modulation, PresetCategory};

static PRESET_MANAGER: Lazy<Mutex<PresetManager>> = Lazy::new(|| {
    let presets_dir = std::env::var("SIGINT_PRESETS_DIR")
        .unwrap_or_else(|_| "/var/lib/sigint-pi/presets".to_string());
    Mutex::new(PresetManager::new(&presets_dir))
});

async fn get_preset_lists() -> impl Responder {
    let manager = PRESET_MANAGER.lock().unwrap();
    let lists: Vec<_> = manager.get_all_lists().iter().map(|l| {
        serde_json::json!({
            "id": l.id,
            "name": l.name,
            "description": l.description,
            "preset_count": l.presets.len(),
            "is_builtin": l.is_builtin
        })
    }).collect();
    
    HttpResponse::Ok().json(serde_json::json!({
        "lists": lists
    }))
}

async fn get_preset_list(path: web::Path<String>) -> impl Responder {
    let list_id = path.into_inner();
    let manager = PRESET_MANAGER.lock().unwrap();
    
    match manager.get_list(&list_id) {
        Some(list) => HttpResponse::Ok().json(list),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Preset list not found"
        }))
    }
}

#[derive(Deserialize)]
struct CreatePresetListRequest {
    name: String,
    description: Option<String>,
}

async fn create_preset_list(body: web::Json<CreatePresetListRequest>) -> impl Responder {
    let mut manager = PRESET_MANAGER.lock().unwrap();
    let list = manager.create_list(&body.name, body.description.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "list": list
    }))
}

async fn delete_preset_list(path: web::Path<String>) -> impl Responder {
    let list_id = path.into_inner();
    let mut manager = PRESET_MANAGER.lock().unwrap();
    
    match manager.delete_list(&list_id) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e }))
    }
}

#[derive(Deserialize)]
struct AddPresetRequest {
    name: String,
    frequency_hz: u64,
    modulation: String,
    bandwidth_hz: Option<u32>,
    category: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    squelch: Option<i32>,
    gain: Option<i32>,
    notes: Option<String>,
}

async fn add_preset(
    path: web::Path<String>,
    body: web::Json<AddPresetRequest>,
) -> impl Responder {
    let list_id = path.into_inner();
    let mut manager = PRESET_MANAGER.lock().unwrap();
    
    let modulation = match body.modulation.to_uppercase().as_str() {
        "AM" => Modulation::AM,
        "FM" | "NFM" => Modulation::NFM,
        "WFM" => Modulation::WFM,
        "USB" => Modulation::USB,
        "LSB" => Modulation::LSB,
        "CW" => Modulation::CW,
        "RAW" => Modulation::RAW,
        _ => Modulation::Unknown,
    };
    
    let category = body.category.as_ref().map(|c| {
        match c.to_lowercase().as_str() {
            "fm" | "fmbroadcast" => PresetCategory::FmBroadcast,
            "am" | "ambroadcast" => PresetCategory::AmBroadcast,
            "shortwave" => PresetCategory::Shortwave,
            "noaa" | "weather" => PresetCategory::NoaaWeather,
            "emergency" => PresetCategory::EmergencyServices,
            "aviation" | "airband" => PresetCategory::AirBand,
            "marine" => PresetCategory::MarineVhf,
            "ham" | "amateur" => PresetCategory::HamVhf,
            "numbers" => PresetCategory::NumbersStation,
            _ => PresetCategory::Custom,
        }
    }).unwrap_or(PresetCategory::Custom);
    
    let preset = FrequencyPreset {
        id: format!("user_{}_{}", list_id, body.frequency_hz),
        name: body.name.clone(),
        frequency_hz: body.frequency_hz,
        modulation,
        bandwidth_hz: body.bandwidth_hz,
        category,
        description: body.description.clone(),
        tags: body.tags.clone().unwrap_or_default(),
        squelch: body.squelch,
        gain: body.gain,
        favorite: false,
        last_used: None,
        notes: body.notes.clone(),
    };
    
    match manager.add_preset(&list_id, preset) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e }))
    }
}

async fn remove_preset(path: web::Path<(String, String)>) -> impl Responder {
    let (list_id, preset_id) = path.into_inner();
    let mut manager = PRESET_MANAGER.lock().unwrap();
    
    match manager.remove_preset(&list_id, &preset_id) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({ "success": true })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e }))
    }
}

#[derive(Deserialize)]
struct SearchPresetsQuery {
    q: String,
}

async fn search_presets(query: web::Query<SearchPresetsQuery>) -> impl Responder {
    let manager = PRESET_MANAGER.lock().unwrap();
    let results: Vec<_> = manager.search(&query.q).into_iter().cloned().collect();
    
    HttpResponse::Ok().json(serde_json::json!({
        "query": query.q,
        "results": results
    }))
}

async fn get_favorite_presets() -> impl Responder {
    let manager = PRESET_MANAGER.lock().unwrap();
    let favorites: Vec<_> = manager.get_favorites().into_iter().cloned().collect();
    
    HttpResponse::Ok().json(serde_json::json!({
        "favorites": favorites
    }))
}

// ============================================
// Radio Reception (rtl_fm)
// ============================================

static RADIO_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static RADIO_FREQ: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
static RADIO_MOD: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("fm".to_string()));

#[derive(Deserialize)]
struct TuneRadioRequest {
    frequency_hz: u64,
    modulation: Option<String>,
    squelch: Option<i32>,
    gain: Option<i32>,
    device_index: Option<u32>,
}

/// Estimate distance from signal strength using free-space path loss model
/// Returns (min_meters, max_meters, description)
pub fn estimate_distance(rssi_dbm: f64, frequency_mhz: f64) -> (f64, f64, String) {
    // Free-space path loss: FSPL(dB) = 20*log10(d) + 20*log10(f) + 32.44
    // where d = distance in km, f = frequency in MHz
    // Rearranged: d(km) = 10^((FSPL - 20*log10(f) - 32.44) / 20)
    //
    // Assume transmitter power varies by device type:
    // - Low power bug/tracker: 0 to 10 dBm (1-10 mW)
    // - Medium power radio: 20 dBm (100 mW)
    // - High power drone/radio: 27-30 dBm (500 mW - 1W)
    //
    // FSPL = tx_power - rssi (received signal = transmitted - path_loss)
    
    let freq_log = if frequency_mhz > 0.0 { 20.0 * frequency_mhz.log10() } else { 0.0 };
    
    // Low power estimate (assume 0 dBm tx = 1 mW, closest estimate)
    let fspl_low = 0.0 - rssi_dbm;
    let d_low_km = 10.0_f64.powf((fspl_low - freq_log - 32.44) / 20.0);
    let d_low_m = (d_low_km * 1000.0).max(0.1);
    
    // High power estimate (assume 30 dBm tx = 1W, farthest estimate)
    let fspl_high = 30.0 - rssi_dbm;
    let d_high_km = 10.0_f64.powf((fspl_high - freq_log - 32.44) / 20.0);
    let d_high_m = (d_high_km * 1000.0).max(0.1);
    
    let desc = if d_low_m < 1.0 {
        "Very close (< 1m) - possibly on your person or vehicle".to_string()
    } else if d_low_m < 10.0 {
        format!("Nearby ({:.0}m - {:.0}m) - same room or adjacent", d_low_m, d_high_m.min(100.0))
    } else if d_low_m < 100.0 {
        format!("Close ({:.0}m - {:.0}m) - same building or nearby", d_low_m, d_high_m.min(500.0))
    } else if d_low_m < 1000.0 {
        format!("Medium range ({:.0}m - {:.1}km)", d_low_m, d_high_m / 1000.0)
    } else {
        format!("Far ({:.1}km - {:.1}km)", d_low_m / 1000.0, d_high_m / 1000.0)
    };
    
    (d_low_m, d_high_m, desc)
}

async fn tune_radio(body: web::Json<TuneRadioRequest>) -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.rtl_sdr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "RTL-SDR not available",
            "hint": "Connect an RTL-SDR device"
        }));
    }
    
    // Stop any existing radio
    let _ = std::process::Command::new("pkill").args(&["-f", "rtl_fm"]).output();
    let _ = std::process::Command::new("pkill").args(&["-f", "ffmpeg.*sigint_radio"]).output();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    let resolve = crate::sdr::resolve_sdr_command;
    let modulation = body.modulation.clone().unwrap_or_else(|| "fm".to_string());
    let mod_arg = match modulation.to_lowercase().as_str() {
        "am" => "am",
        "wfm" | "wbfm" => "wbfm",
        "usb" => "usb",
        "lsb" => "lsb",
        "cw" => "cw",
        _ => "fm",
    };
    
    let sample_rate = if mod_arg == "wbfm" { "170000" } else { "24000" };
    let output_rate = if mod_arg == "wbfm" { "48000" } else { "24000" };

    // Build rtl_fm command
    let mut cmd_parts = vec![
        resolve("rtl_fm"),
        "-f".to_string(), body.frequency_hz.to_string(),
        "-M".to_string(), mod_arg.to_string(),
        "-s".to_string(), sample_rate.to_string(),
        "-r".to_string(), output_rate.to_string(),
    ];
    
    if let Some(sq) = body.squelch {
        cmd_parts.extend(["-l".to_string(), sq.to_string()]);
    }
    if let Some(g) = body.gain {
        cmd_parts.extend(["-g".to_string(), g.to_string()]);
    }
    if let Some(d) = body.device_index {
        cmd_parts.extend(["-d".to_string(), d.to_string()]);
    }
    
    // Write raw PCM to a named pipe, also pipe to local aplay if available
    // The /api/sdr/radio/stream endpoint will read from this pipe
    let pipe_path = "/tmp/sigint_radio.pcm";
    let _ = std::fs::remove_file(pipe_path);
    
    // Start rtl_fm writing raw PCM to a file (rolling buffer)
    let rtl_cmd = cmd_parts.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" ");
    let _child = std::process::Command::new("sh")
        .args(&["-c", &format!(
            "{} 2>/dev/null > {} &",
            rtl_cmd, pipe_path
        )])
        .spawn();
    
    {
        let mut running = RADIO_RUNNING.lock().unwrap();
        *running = true;
    }
    {
        let mut freq = RADIO_FREQ.lock().unwrap();
        *freq = body.frequency_hz;
    }
    {
        let mut m = RADIO_MOD.lock().unwrap();
        *m = modulation.clone();
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "tuned",
        "frequency_hz": body.frequency_hz,
        "frequency_mhz": body.frequency_hz as f64 / 1_000_000.0,
        "modulation": modulation,
        "sample_rate": output_rate.parse::<u32>().unwrap_or(24000),
        "stream_url": "/api/sdr/radio/stream"
    }))
}

/// GET /api/sdr/radio/stream - Stream raw PCM audio to browser
/// Returns chunked audio/pcm data (signed 16-bit LE, mono)
async fn stream_radio_audio() -> impl Responder {
    use actix_web::HttpResponse;
    use futures::stream;
    
    let running = { *RADIO_RUNNING.lock().unwrap() };
    if !running {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(r#"{"error":"Radio not running. Tune to a frequency first."}"#);
    }
    
    let pipe_path = "/tmp/sigint_radio.pcm";
    
    // Stream PCM data as chunked HTTP response
    // The browser will decode this with Web Audio API
    let stream = stream::unfold(
        (std::path::PathBuf::from(pipe_path), 0u64),
        |(path, mut offset)| async move {
            // Keep streaming while radio is running
            for _ in 0..600 { // Max ~60 seconds per connection (100ms * 600)
                let running = { *RADIO_RUNNING.lock().unwrap() };
                if !running { return None; }
                
                // Read available data from the PCM file
                if let Ok(data) = tokio::fs::read(&path).await {
                    let data_len = data.len() as u64;
                    if data_len > offset {
                        let new_data = data[offset as usize..].to_vec();
                        offset = data_len;
                        if !new_data.is_empty() {
                            return Some((Ok::<_, std::io::Error>(actix_web::web::Bytes::from(new_data)), (path, offset)));
                        }
                    }
                    // File might get too large, reset if > 10MB
                    if data_len > 10_000_000 {
                        let _ = tokio::fs::write(&path, &[]).await;
                        offset = 0;
                    }
                }
                
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            None
        },
    );
    
    HttpResponse::Ok()
        .content_type("audio/pcm")
        .insert_header(("X-Sample-Rate", "24000"))
        .insert_header(("X-Channels", "1"))
        .insert_header(("X-Bits", "16"))
        .insert_header(("X-Encoding", "signed-integer"))
        .insert_header(("Access-Control-Expose-Headers", "X-Sample-Rate, X-Channels, X-Bits"))
        .streaming(stream)
}

async fn stop_radio() -> impl Responder {
    let _ = std::process::Command::new("pkill").args(&["-f", "rtl_fm"]).output();
    let _ = std::process::Command::new("pkill").args(&["-f", "aplay"]).output();
    let _ = std::fs::remove_file("/tmp/sigint_radio.pcm");
    
    {
        let mut running = RADIO_RUNNING.lock().unwrap();
        *running = false;
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopped"
    }))
}

async fn get_radio_status() -> impl Responder {
    let running = { *RADIO_RUNNING.lock().unwrap() };
    let freq = { *RADIO_FREQ.lock().unwrap() };
    let modulation = { RADIO_MOD.lock().unwrap().clone() };
    
    HttpResponse::Ok().json(serde_json::json!({
        "running": running,
        "frequency_hz": freq,
        "frequency_mhz": freq as f64 / 1_000_000.0,
        "modulation": modulation
    }))
}

// ============================================
// TSCM Bug Detection / Counter-Surveillance
// ============================================

use crate::sdr::tscm::{SurveillanceBand, TscmSweepConfig, ThreatCategory, ThreatSeverity};

static TSCM_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static TSCM_THREATS: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));
static TSCM_PROGRESS: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));
static TSCM_SWEEP_COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));
static TSCM_CURRENT_BAND: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static TSCM_LAST_UPDATE: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

#[derive(Deserialize)]
struct TscmSweepRequest {
    sweep_type: Option<String>, // quick, standard, full, federal
    continuous: Option<bool>,   // true = loop until stopped
}

async fn start_tscm_sweep(body: web::Json<TscmSweepRequest>) -> impl Responder {
    let caps = SdrCapabilities::detect();
    
    if !caps.hackrf && !caps.rtl_sdr {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No SDR hardware available",
            "hint": "TSCM sweep requires RTL-SDR or HackRF"
        }));
    }
    
    {
        let running = TSCM_RUNNING.lock().unwrap();
        if *running {
            return HttpResponse::Ok().json(serde_json::json!({
                "status": "already_running"
            }));
        }
    }
    
    let sweep_config = match body.sweep_type.as_deref() {
        Some("quick") => TscmSweepConfig::quick_sweep(),
        Some("full") => TscmSweepConfig::full_sweep(),
        Some("federal") => TscmSweepConfig::federal_threat_sweep(),
        _ => TscmSweepConfig::standard_sweep(),
    };
    
    let continuous = body.continuous.unwrap_or(false);
    
    {
        let mut running = TSCM_RUNNING.lock().unwrap();
        *running = true;
    }
    {
        let mut progress = TSCM_PROGRESS.lock().unwrap();
        *progress = 0.0;
    }
    {
        let mut threats = TSCM_THREATS.lock().unwrap();
        threats.clear();
    }
    {
        let mut count = TSCM_SWEEP_COUNT.lock().unwrap();
        *count = 0;
    }
    
    let has_hackrf = caps.hackrf;
    let has_rtl = caps.rtl_sdr;
    let sweep_name = sweep_config.name.clone();
    let bands = sweep_config.bands.clone();
    let bands_count = bands.len();
    let threshold = sweep_config.threshold_db;
    
    tokio::spawn(async move {
        let threat_db = SurveillanceBand::threat_database();
        let total_bands = bands.len();
        
        loop {
            for (i, (start_hz, end_hz)) in bands.iter().enumerate() {
                if !{ *TSCM_RUNNING.lock().unwrap() } {
                    return;
                }
                
                // Update progress and current band
                {
                    let mut progress = TSCM_PROGRESS.lock().unwrap();
                    *progress = (i as f32 / total_bands as f32) * 100.0;
                }
                {
                    let mut band = TSCM_CURRENT_BAND.lock().unwrap();
                    *band = format!("{:.1}-{:.1} MHz", *start_hz as f64 / 1e6, *end_hz as f64 / 1e6);
                }
                {
                    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
                    let mut last = TSCM_LAST_UPDATE.lock().unwrap();
                    *last = now;
                }
                
                let start_mhz_f = *start_hz as f64 / 1e6;
                let end_mhz_f = *end_hz as f64 / 1e6;
                
                let scan_result = tscm_scan_band(start_mhz_f, end_mhz_f, has_hackrf, has_rtl).await;
                
                if let Some(stdout) = scan_result {
                    parse_sweep_output(&stdout, threshold, &threat_db);
                }
                
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
            
            // One full sweep complete
            {
                let mut count = TSCM_SWEEP_COUNT.lock().unwrap();
                *count += 1;
            }
            {
                let mut progress = TSCM_PROGRESS.lock().unwrap();
                *progress = 100.0;
            }
            
            if !continuous {
                break;
            }
            
            // Brief pause between sweeps in continuous mode
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        
        let mut running = TSCM_RUNNING.lock().unwrap();
        *running = false;
    });
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "sweep_type": sweep_name,
        "bands_to_scan": bands_count,
        "continuous": continuous
    }))
}

/// Execute a single band scan using the best available hardware
async fn tscm_scan_band(start_mhz: f64, end_mhz: f64, has_hackrf: bool, has_rtl: bool) -> Option<String> {
    let rtl_min = 24.0;
    let rtl_max = 1766.0;
    let resolve = crate::sdr::resolve_sdr_command;
    
    // HackRF path: for bands outside RTL-SDR range or if RTL-SDR unavailable
    if has_hackrf && (start_mhz < rtl_min || end_mhz > rtl_max || !has_rtl) {
        let h_start = start_mhz.max(1.0) as u32;
        let h_end = end_mhz.min(6000.0) as u32;
        if h_start >= h_end { return None; }
        
        // Try hackrf_sweep first (may return non-zero with pipe errors but still produce data)
        if let Ok(out) = tokio::process::Command::new(&resolve("hackrf_sweep"))
            .args(&["-f", &format!("{}:{}", h_start, h_end), "-w", "100000", "-1"])
            .output().await
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !stdout.is_empty() && stdout.contains(',') {
                return Some(stdout.to_string());
            }
        }
        // Fallback: hackrf_transfer + FFT
        tracing::warn!("hackrf_sweep produced no output for {}-{} MHz, using hackrf_transfer fallback", h_start, h_end);
        let center_hz = ((h_start as u64 + h_end as u64) / 2) * 1_000_000;
        let sr: u64 = 8_000_000;
        let iq_path = format!("/tmp/sigint_tscm_hrf_{}.bin", h_start);
        if let Ok(out) = tokio::process::Command::new(&resolve("hackrf_transfer"))
            .args(&["-r", &iq_path, "-f", &center_hz.to_string(), "-s", &sr.to_string(), "-n", &(sr * 2).to_string()])
            .output().await
        {
            if out.status.success() {
                if let Some(csv) = iq_to_csv(&iq_path, center_hz, sr, true).await {
                    return Some(csv);
                }
            }
        }
    }
    
    // RTL-SDR path
    if has_rtl && end_mhz >= rtl_min && start_mhz < rtl_max {
        let r_start = start_mhz.max(rtl_min);
        let r_end = end_mhz.min(rtl_max);
        if r_start >= r_end { return None; }
        let bin = if (r_end - r_start) > 100.0 { "500k" } else if (r_end - r_start) > 10.0 { "100k" } else { "10k" };
        
        // Try rtl_power first
        // Note: rtl_power may return exit code 1 due to PLL warnings but still produce
        // valid CSV data on stdout. Check stdout content regardless of exit code.
        if let Ok(out) = tokio::process::Command::new(&resolve("rtl_power"))
            .args(&["-f", &format!("{:.1}M:{:.1}M:{}", r_start, r_end, bin), "-i", "1", "-1", "-g", "40"])
            .output().await
        {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !stdout.is_empty() && stdout.contains(',') {
                return Some(stdout.to_string());
            }
            let stderr = String::from_utf8_lossy(&out.stderr);
            tracing::debug!("rtl_power {:.0}-{:.0} MHz: exit={}, stdout={} bytes, stderr={}",
                r_start, r_end, out.status, out.stdout.len(), stderr.lines().last().unwrap_or(""));
        }
        // Fallback: rtl_sdr + FFT
        tracing::warn!("rtl_power produced no output for {:.0}-{:.0} MHz, using rtl_sdr fallback", r_start, r_end);
        let center_hz = ((r_start as u64 + r_end as u64) / 2) * 1_000_000;
        let bw = ((r_end - r_start) as u64).max(1) * 1_000_000;
        let sr = bw.min(2_400_000).max(1_000_000);
        let iq_path = format!("/tmp/sigint_tscm_rtl_{}.bin", r_start as u32);
        if let Ok(out) = tokio::process::Command::new(&resolve("rtl_sdr"))
            .args(&["-f", &center_hz.to_string(), "-s", &sr.to_string(), "-n", &(sr * 2).to_string(), &iq_path])
            .output().await
        {
            if out.status.success() || std::path::Path::new(&iq_path).exists() {
                if let Some(csv) = iq_to_csv(&iq_path, center_hz, sr, false).await {
                    return Some(csv);
                }
            }
        }
    }
    
    None
}

/// Convert raw IQ binary file to rtl_power-compatible CSV using python FFT
pub async fn iq_to_csv(iq_path: &str, center_hz: u64, sample_rate: u64, is_hackrf: bool) -> Option<String> {
    let decode = if is_hackrf { "(data[i*2]^0x80)-128" } else { "data[i*2]-127.5" };
    let decode_q = if is_hackrf { "(data[i*2+1]^0x80)-128" } else { "data[i*2+1]-127.5" };
    let script = format!(
        r#"import sys,struct,math,cmath
data=open('{path}','rb').read()
n=len(data)//2
if n<256: sys.exit(1)
N=min(n,4096)
iq=[complex({d_i},{d_q}) for i in range(N)]
# Hanning window
import math as m
win=[0.5*(1-m.cos(2*m.pi*i/(N-1))) for i in range(N)]
iq=[iq[i]*win[i] for i in range(N)]
# DFT via slicing for speed
powers=[]
for k in range(N):
    s=sum(iq[j]*cmath.exp(-2j*cmath.pi*k*j/N) for j in range(0,N,max(1,N//256)))
    p=20*m.log10(abs(s)/N+1e-10)
    powers.append(p)
# Reorder: DC center
powers=powers[N//2:]+powers[:N//2]
step={sr}/N
hz_low={center}-{sr}//2
for i in range(0,len(powers),max(1,len(powers)//128)):
    hz=hz_low+int(i*step)
    chunk=powers[i:i+max(1,len(powers)//128)]
    avg=sum(chunk)/len(chunk) if chunk else -100
    print(f"2026-01-01, 00:00:00, {{hz}}, {{hz+int(step*len(chunk))}}, {{int(step)}}, {{len(chunk)}}, {{avg:.1f}}")
"#, path=iq_path, d_i=decode, d_q=decode_q, sr=sample_rate, center=center_hz);
    let py_path = format!("{}_fft.py", iq_path);
    if tokio::fs::write(&py_path, &script).await.is_err() { return None; }
    match tokio::process::Command::new("python3").arg(&py_path).output().await {
        Ok(out) if out.status.success() && !out.stdout.is_empty() => {
            let _ = tokio::fs::remove_file(iq_path).await;
            let _ = tokio::fs::remove_file(&py_path).await;
            Some(String::from_utf8_lossy(&out.stdout).to_string())
        }
        _ => None,
    }
}

/// Frequency grouping tolerance: signals within 1 MHz are considered the same emitter
const FREQ_GROUP_TOLERANCE_HZ: u64 = 1_000_000;

/// Severity rank for sorting (higher = more severe)
fn severity_rank(s: &str) -> u8 {
    match s {
        "Critical" => 4,
        "High" => 3,
        "Medium" => 2,
        "Low" => 1,
        _ => 0,
    }
}

/// Parse sweep output and match against threat database.
/// Groups detections by frequency (within tolerance) so a single signal
/// that matches multiple overlapping threat bands appears as ONE grouped
/// entry with the highest-severity label as primary and all matching
/// rules listed under `matched_rules`.
fn parse_sweep_output(stdout: &str, threshold: f64, threat_db: &[SurveillanceBand]) {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();

    // Phase 1: Collect all above-threshold (freq, power) hits from CSV
    struct RawHit { freq_hz: u64, power_db: f64 }
    let mut raw_hits: Vec<RawHit> = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 7 { continue; }
        if let (Ok(hz_low), Ok(hz_step)) = (
            parts[2].trim().parse::<u64>(),
            parts[4].trim().parse::<u64>(),
        ) {
            for (idx, db_str) in parts[6..].iter().enumerate() {
                if let Ok(power_db) = db_str.trim().parse::<f64>() {
                    if power_db > threshold {
                        let freq = hz_low + (idx as u64 * hz_step);
                        raw_hits.push(RawHit { freq_hz: freq, power_db });
                    }
                }
            }
        }
    }

    if raw_hits.is_empty() { return; }

    // Phase 2: Group hits by frequency (within tolerance) and find best power per group
    struct FreqGroup { center_hz: u64, peak_hz: u64, peak_db: f64 }
    let mut groups: Vec<FreqGroup> = Vec::new();
    for hit in &raw_hits {
        if let Some(g) = groups.iter_mut().find(|g| {
            (hit.freq_hz as i64 - g.center_hz as i64).unsigned_abs() <= FREQ_GROUP_TOLERANCE_HZ
        }) {
            if hit.power_db > g.peak_db {
                g.peak_db = hit.power_db;
                g.peak_hz = hit.freq_hz;
            }
        } else {
            groups.push(FreqGroup {
                center_hz: hit.freq_hz,
                peak_hz: hit.freq_hz,
                peak_db: hit.power_db,
            });
        }
    }

    // Phase 3: For each frequency group, find ALL matching threat bands
    let mut threats = TSCM_THREATS.lock().unwrap();

    for group in &groups {
        let mut matching_rules: Vec<(&SurveillanceBand, u8)> = Vec::new();
        for band in threat_db {
            if group.peak_hz >= band.start_hz && group.peak_hz <= band.end_hz {
                let sev_str = format!("{:?}", band.severity);
                matching_rules.push((band, severity_rank(&sev_str)));
            }
        }
        if matching_rules.is_empty() { continue; }

        // Sort by severity descending - highest severity becomes the primary label
        matching_rules.sort_by(|a, b| b.1.cmp(&a.1));
        let primary = matching_rules[0].0;
        let primary_sev = format!("{:?}", primary.severity);
        let primary_cat = format!("{:?}", primary.category);

        // Build matched_rules list for the UI
        let rules_json: Vec<serde_json::Value> = matching_rules.iter().map(|(band, _)| {
            serde_json::json!({
                "name": band.name,
                "category": format!("{:?}", band.category),
                "severity": format!("{:?}", band.severity),
                "description": band.description,
            })
        }).collect();

        // Check for consumer devices at this frequency (possible false positives)
        let consumer_matches = crate::sdr::consumer_false_positives::ConsumerDevice::devices_in_range(
            group.peak_hz.saturating_sub(500_000), // +/- 500 kHz tolerance
            group.peak_hz.saturating_add(500_000),
        );
        let consumer_json: Vec<serde_json::Value> = consumer_matches.iter().take(5).map(|d| {
            serde_json::json!({
                "name": d.name,
                "category": format!("{:?}", d.category),
                "brands": d.common_brands,
                "prevalence": format!("{:?}", d.prevalence),
                "description": d.description,
            })
        }).collect();

        // Try to find existing grouped threat at this frequency
        let existing = threats.iter_mut().find(|t| {
            if let Some(existing_hz) = t.get("group_center_hz").and_then(|v| v.as_u64()) {
                (group.center_hz as i64 - existing_hz as i64).unsigned_abs() <= FREQ_GROUP_TOLERANCE_HZ
            } else {
                false
            }
        });

        if let Some(existing) = existing {
            existing["last_seen"] = serde_json::json!(now);
            existing["sightings"] = serde_json::json!(
                existing.get("sightings").and_then(|v| v.as_u64()).unwrap_or(0) + 1
            );
            let prev_peak = existing.get("peak_power_db").and_then(|v| v.as_f64()).unwrap_or(f64::NEG_INFINITY);
            if group.peak_db > prev_peak {
                existing["peak_power_db"] = serde_json::json!(group.peak_db);
                existing["peak_frequency_hz"] = serde_json::json!(group.peak_hz);
            }
            existing["power_db"] = serde_json::json!(group.peak_db);
            existing["matched_rules"] = serde_json::json!(rules_json);
            existing["matched_rules_count"] = serde_json::json!(rules_json.len());
            if !consumer_json.is_empty() {
                existing["possible_benign"] = serde_json::json!(consumer_json);
            }
        } else {
            let freq_mhz = group.peak_hz as f64 / 1e6;
            let (dist_min, dist_max, dist_desc) = estimate_distance(group.peak_db, freq_mhz);
            threats.push(serde_json::json!({
                "frequency_hz": group.peak_hz,
                "frequency_mhz": freq_mhz,
                "group_center_hz": group.center_hz,
                "power_db": group.peak_db,
                "peak_power_db": group.peak_db,
                "peak_frequency_hz": group.peak_hz,
                "category": primary_cat,
                "severity": primary_sev,
                "band_name": primary.name,
                "description": primary.description,
                "source": primary.source,
                "first_seen": now,
                "last_seen": now,
                "sightings": 1u64,
                "matched_rules": rules_json,
                "matched_rules_count": matching_rules.len(),
                "estimated_distance_m": format!("{:.0} - {:.0}", dist_min, dist_max),
                "distance_description": dist_desc,
                "distance_min_m": dist_min,
                "distance_max_m": dist_max,
                "possible_benign": consumer_json
            }));
        }
    }
}

async fn get_tscm_status() -> impl Responder {
    let running = { *TSCM_RUNNING.lock().unwrap() };
    let progress = { *TSCM_PROGRESS.lock().unwrap() };
    let threats = { TSCM_THREATS.lock().unwrap().clone() };
    let sweep_count = { *TSCM_SWEEP_COUNT.lock().unwrap() };
    let current_band = { TSCM_CURRENT_BAND.lock().unwrap().clone() };
    let last_update = { *TSCM_LAST_UPDATE.lock().unwrap() };
    
    // Count unique frequency groups (likely distinct emitters)
    let unique_emitters = threats.len();
    let total_rules_matched: usize = threats.iter()
        .map(|t| t.get("matched_rules_count").and_then(|v| v.as_u64()).unwrap_or(1) as usize)
        .sum();

    HttpResponse::Ok().json(serde_json::json!({
        "running": running,
        "progress": progress,
        "threats_found": unique_emitters,
        "unique_emitters": unique_emitters,
        "total_rules_matched": total_rules_matched,
        "threats": threats,
        "sweep_count": sweep_count,
        "current_band": current_band,
        "last_update": last_update
    }))
}

async fn stop_tscm_sweep() -> impl Responder {
    {
        let mut running = TSCM_RUNNING.lock().unwrap();
        *running = false;
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopped"
    }))
}

async fn get_tscm_threats() -> impl Responder {
    // Return the full threat database for reference
    let bands = SurveillanceBand::threat_database();
    let threats: Vec<_> = bands.iter().map(|b| {
        serde_json::json!({
            "name": b.name,
            "start_mhz": b.start_hz as f64 / 1_000_000.0,
            "end_mhz": b.end_hz as f64 / 1_000_000.0,
            "category": format!("{:?}", b.category),
            "severity": format!("{:?}", b.severity),
            "description": b.description,
            "source": b.source
        })
    }).collect();
    
    HttpResponse::Ok().json(serde_json::json!({
        "threat_database": threats,
        "total_bands": threats.len()
    }))
}

// Legal disclaimer endpoint
async fn get_legal() -> impl Responder {
    use crate::settings::LEGAL_DISCLAIMER;
    
    // Try to read LEGAL.md from disk first, fall back to embedded disclaimer
    let legal_md = if let Some(dir) = crate::web::find_static_dir() {
        let legal_path = dir.join("../LEGAL.md");
        if legal_path.exists() {
            std::fs::read_to_string(&legal_path).ok()
        } else {
            // Also try in the static dir itself
            let alt = dir.join("LEGAL.md");
            if alt.exists() {
                std::fs::read_to_string(&alt).ok()
            } else {
                None
            }
        }
    } else {
        None
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "legal_text": legal_md.unwrap_or_else(|| LEGAL_DISCLAIMER.to_string()),
        "version": "1.0",
        "requires_acceptance": true
    }))
}

async fn accept_legal() -> impl Responder {
    let marker = std::path::Path::new("/data/.disclaimer_accepted");
    if let Some(parent) = marker.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(marker, format!("accepted_at={}", chrono::Utc::now().to_rfc3339()));
    
    // Also try home directory fallback
    if let Ok(home) = std::env::var("HOME") {
        let home_marker = format!("{}/.sigint-disclaimer-accepted", home);
        let _ = std::fs::write(&home_marker, format!("accepted_at={}", chrono::Utc::now().to_rfc3339()));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "accepted",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn legal_status() -> impl Responder {
    let accepted = std::path::Path::new("/data/.disclaimer_accepted").exists() || {
        if let Ok(home) = std::env::var("HOME") {
            std::path::Path::new(&format!("{}/.sigint-disclaimer-accepted", home)).exists()
        } else {
            false
        }
    };
    
    HttpResponse::Ok().json(serde_json::json!({
        "accepted": accepted
    }))
}

// ============================================
// Browser TTS Alert System
// ============================================

static TTS_ENABLED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static TTS_LAST_ALERT_ID: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

#[derive(Deserialize)]
struct TtsConfigRequest {
    enabled: bool,
}

async fn get_tts_alert_config() -> impl Responder {
    let enabled = *TTS_ENABLED.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "enabled": enabled,
        "engine": "browser",
        "piper_available": std::path::Path::new("/home/pi/sigint-pi/venv/bin/piper").exists(),
        "model_available": std::path::Path::new("/home/pi/sigint-pi/models/piper/en_US-lessac-medium.onnx").exists(),
    }))
}

async fn set_tts_alert_config(body: web::Json<TtsConfigRequest>) -> impl Responder {
    *TTS_ENABLED.lock().unwrap() = body.enabled;
    tracing::info!("TTS alerts {}", if body.enabled { "enabled" } else { "disabled" });
    HttpResponse::Ok().json(serde_json::json!({
        "enabled": body.enabled
    }))
}

async fn get_pending_tts_alerts(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let enabled = *TTS_ENABLED.lock().unwrap();
    if !enabled {
        return HttpResponse::Ok().json(serde_json::json!({"alerts": []}));
    }

    let alerts = state.alerts.read().await;
    let last_id = {
        let id = TTS_LAST_ALERT_ID.lock().unwrap();
        *id
    };

    // Return alerts newer than last spoken, priority Critical/High only
    let pending: Vec<serde_json::Value> = alerts.iter()
        .filter(|a| a.id > last_id)
        .filter(|a| {
            let p = a.priority.to_lowercase();
            p == "critical" || p == "high"
        })
        .map(|a| serde_json::json!({
            "id": a.id,
            "message": a.message,
            "priority": a.priority,
            "timestamp": a.timestamp,
        }))
        .collect();

    // Update last seen ID
    if let Some(max_id) = pending.iter().filter_map(|a| a["id"].as_u64()).max() {
        *TTS_LAST_ALERT_ID.lock().unwrap() = max_id;
    }

    HttpResponse::Ok().json(serde_json::json!({
        "alerts": pending
    }))
}

#[derive(Deserialize)]
struct TtsGenerateRequest {
    text: String,
}

async fn generate_tts_wav(body: web::Json<TtsGenerateRequest>) -> impl Responder {
    let text = &body.text;
    if text.is_empty() || text.len() > 500 {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "text must be 1-500 chars"}));
    }

    // Try Piper TTS (generates WAV)
    let piper_bin = "/home/pi/sigint-pi/venv/bin/piper";
    let model = "/home/pi/sigint-pi/models/piper/en_US-lessac-medium.onnx";

    if !std::path::Path::new(piper_bin).exists() || !std::path::Path::new(model).exists() {
        return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "piper not available, use browser Web Speech API"
        }));
    }

    let output_path = format!("/tmp/tts_{}.wav", std::process::id());
    let result = tokio::process::Command::new(piper_bin)
        .args(["--model", model, "--output_file", &output_path])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    match result {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                use tokio::io::AsyncWriteExt;
                let _ = stdin.write_all(text.as_bytes()).await;
                drop(stdin);
            }
            // Wait with timeout (Piper can be slow on Pi ARM)
            match tokio::time::timeout(
                std::time::Duration::from_secs(15),
                child.wait()
            ).await {
                Ok(Ok(status)) if status.success() => {
                    if let Ok(wav_data) = tokio::fs::read(&output_path).await {
                        let _ = tokio::fs::remove_file(&output_path).await;
                        return HttpResponse::Ok()
                            .content_type("audio/wav")
                            .body(wav_data);
                    }
                }
                _ => {
                    let _ = child.kill().await;
                    let _ = tokio::fs::remove_file(&output_path).await;
                }
            }
        }
        Err(_) => {}
    }

    HttpResponse::ServiceUnavailable().json(serde_json::json!({
        "error": "piper generation failed, use browser Web Speech API"
    }))
}

// ============================================
// Device Alert Silencing
// ============================================

async fn silence_device_alerts(path: web::Path<String>) -> impl Responder {
    let mac = path.into_inner();
    crate::alerts::silence_device(&mac);
    tracing::info!("Silenced alerts for device: {}", mac);
    HttpResponse::Ok().json(serde_json::json!({
        "silenced": true,
        "mac": mac.to_uppercase()
    }))
}

async fn unsilence_device_alerts(path: web::Path<String>) -> impl Responder {
    let mac = path.into_inner();
    crate::alerts::unsilence_device(&mac);
    tracing::info!("Unsilenced alerts for device: {}", mac);
    HttpResponse::Ok().json(serde_json::json!({
        "silenced": false,
        "mac": mac.to_uppercase()
    }))
}

async fn get_silenced_devices() -> impl Responder {
    let devices = crate::alerts::get_silenced_devices();
    HttpResponse::Ok().json(serde_json::json!({
        "silenced_devices": devices
    }))
}

// ============================================
// Ham Radio - Morse Decoder
// ============================================

static MORSE_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static MORSE_DECODED: Lazy<Mutex<Vec<serde_json::Value>>> = Lazy::new(|| Mutex::new(Vec::new()));
static MORSE_FREQ: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

#[derive(Deserialize)]
struct MorseStartRequest {
    frequency_hz: u64,
    #[serde(default = "default_wpm")]
    wpm: u32,
    #[serde(default = "default_tone_hz")]
    tone_hz: u32,
}

fn default_wpm() -> u32 { 20 }
fn default_tone_hz() -> u32 { 700 }

async fn start_morse_decoder(body: web::Json<MorseStartRequest>) -> impl Responder {
    let freq = body.frequency_hz;
    let wpm = body.wpm;
    let tone_hz = body.tone_hz;

    if *MORSE_RUNNING.lock().unwrap() {
        return HttpResponse::Conflict().json(serde_json::json!({"error": "Morse decoder already running"}));
    }

    *MORSE_RUNNING.lock().unwrap() = true;
    *MORSE_FREQ.lock().unwrap() = freq;
    MORSE_DECODED.lock().unwrap().clear();

    // Start rtl_fm in CW mode piped through a tone detector
    tokio::spawn(async move {
        let rtl_fm = tokio::process::Command::new("rtl_fm")
            .args([
                "-f", &freq.to_string(),
                "-M", "usb",
                "-s", "12000",
                "-g", "40",
                "-l", "0",
                "-"
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn();

        match rtl_fm {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    use tokio::io::AsyncReadExt;
                    let mut reader = tokio::io::BufReader::new(stdout);
                    let mut buf = vec![0u8; 2400]; // 100ms of 12kHz 16-bit audio
                    let sample_rate = 12000.0_f64;
                    let target_freq = tone_hz as f64;
                    let dot_duration = 1.2 / wpm as f64;

                    // Goertzel tone detection state
                    let mut tone_on = false;
                    let mut tone_start: f64 = 0.0;
                    let mut silence_start: f64 = 0.0;
                    let mut current_char = String::new();
                    let mut decoded_text = String::new();
                    let mut sample_count: u64 = 0;

                    loop {
                        if !*MORSE_RUNNING.lock().unwrap() { break; }

                        match reader.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                let samples = n / 2;
                                sample_count += samples as u64;
                                let time = sample_count as f64 / sample_rate;

                                // Goertzel algorithm for tone detection
                                let k = (0.5 + (samples as f64 * target_freq / sample_rate)) as usize;
                                let w = 2.0 * std::f64::consts::PI * k as f64 / samples as f64;
                                let coeff = 2.0 * w.cos();
                                let mut s0: f64 = 0.0;
                                let mut s1: f64 = 0.0;
                                let mut s2: f64 = 0.0;

                                for i in 0..samples {
                                    let sample = if i * 2 + 1 < n {
                                        i16::from_le_bytes([buf[i*2], buf[i*2+1]]) as f64 / 32768.0
                                    } else { 0.0 };
                                    s0 = coeff * s1 - s2 + sample;
                                    s2 = s1;
                                    s1 = s0;
                                }

                                let power = s1*s1 + s2*s2 - coeff*s1*s2;
                                let magnitude = power.sqrt();
                                let is_tone = magnitude > 0.05;

                                if is_tone && !tone_on {
                                    tone_on = true;
                                    tone_start = time;
                                    let silence_dur = time - silence_start;
                                    // Word gap (7 units)
                                    if silence_dur > dot_duration * 5.0 && !current_char.is_empty() {
                                        if let Some(ch) = morse_to_char(&current_char) {
                                            decoded_text.push(ch);
                                        }
                                        decoded_text.push(' ');
                                        current_char.clear();
                                    }
                                    // Char gap (3 units)
                                    else if silence_dur > dot_duration * 2.0 && !current_char.is_empty() {
                                        if let Some(ch) = morse_to_char(&current_char) {
                                            decoded_text.push(ch);
                                        }
                                        current_char.clear();
                                    }
                                } else if !is_tone && tone_on {
                                    tone_on = false;
                                    silence_start = time;
                                    let tone_dur = time - tone_start;
                                    if tone_dur > dot_duration * 2.0 {
                                        current_char.push('-');
                                    } else {
                                        current_char.push('.');
                                    }
                                }

                                // Periodically push decoded text
                                if !decoded_text.is_empty() && sample_count % 24000 == 0 {
                                    let now = std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap_or_default().as_secs();
                                    let mut decoded = MORSE_DECODED.lock().unwrap();
                                    decoded.push(serde_json::json!({
                                        "text": decoded_text.clone(),
                                        "frequency_hz": freq,
                                        "wpm": wpm,
                                        "timestamp": now
                                    }));
                                    if decoded.len() > 100 { decoded.remove(0); }
                                }
                            }
                            Err(_) => break,
                        }
                    }

                    // Final flush
                    if !current_char.is_empty() {
                        if let Some(ch) = morse_to_char(&current_char) {
                            decoded_text.push(ch);
                        }
                    }
                    if !decoded_text.is_empty() {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default().as_secs();
                        let mut decoded = MORSE_DECODED.lock().unwrap();
                        decoded.push(serde_json::json!({
                            "text": decoded_text,
                            "frequency_hz": freq,
                            "wpm": wpm,
                            "timestamp": now
                        }));
                    }
                }
                let _ = child.kill().await;
            }
            Err(e) => {
                tracing::error!("Failed to start rtl_fm for morse: {}", e);
            }
        }
        *MORSE_RUNNING.lock().unwrap() = false;
    });

    HttpResponse::Ok().json(serde_json::json!({
        "running": true,
        "frequency_hz": freq,
        "wpm": wpm,
        "tone_hz": tone_hz
    }))
}

async fn stop_morse_decoder() -> impl Responder {
    *MORSE_RUNNING.lock().unwrap() = false;
    HttpResponse::Ok().json(serde_json::json!({"running": false}))
}

async fn get_morse_status() -> impl Responder {
    let running = *MORSE_RUNNING.lock().unwrap();
    let freq = *MORSE_FREQ.lock().unwrap();
    let decoded = MORSE_DECODED.lock().unwrap().clone();
    HttpResponse::Ok().json(serde_json::json!({
        "running": running,
        "frequency_hz": freq,
        "decoded": decoded
    }))
}

fn morse_to_char(code: &str) -> Option<char> {
    match code {
        ".-"    => Some('A'), "-..."  => Some('B'), "-.-."  => Some('C'),
        "-.."   => Some('D'), "."     => Some('E'), "..-."  => Some('F'),
        "--."   => Some('G'), "...."  => Some('H'), ".."    => Some('I'),
        ".---"  => Some('J'), "-.-"   => Some('K'), ".-.."  => Some('L'),
        "--"    => Some('M'), "-."    => Some('N'), "---"   => Some('O'),
        ".--."  => Some('P'), "--.-"  => Some('Q'), ".-."   => Some('R'),
        "..."   => Some('S'), "-"     => Some('T'), "..-"   => Some('U'),
        "...-"  => Some('V'), ".--"   => Some('W'), "-..-"  => Some('X'),
        "-.--"  => Some('Y'), "--.."  => Some('Z'),
        "-----" => Some('0'), ".----" => Some('1'), "..---" => Some('2'),
        "...--" => Some('3'), "....-" => Some('4'), "....." => Some('5'),
        "-...." => Some('6'), "--..." => Some('7'), "---.." => Some('8'),
        "----." => Some('9'),
        ".-.-.-" => Some('.'), "--..--" => Some(','), "..--.." => Some('?'),
        ".----." => Some('\''), "-.-.--" => Some('!'), "-..-." => Some('/'),
        "-.--." => Some('('), "-.--.-" => Some(')'), ".-..." => Some('&'),
        "---..." => Some(':'), "-.-.-." => Some(';'), "-...-" => Some('='),
        ".-.-." => Some('+'), "-....-" => Some('-'), "..--.-" => Some('_'),
        ".-..-." => Some('"'), "...-..-" => Some('$'), ".--.-." => Some('@'),
        _ => None,
    }
}

// ============================================
// SIEM - Security Information & Event Management
// ============================================

const SIEM_MAX_BYTES: i64 = 4 * 1024 * 1024 * 1024; // 4GB rolling budget

#[derive(Deserialize)]
struct SiemQuery {
    #[serde(default = "default_siem_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
    severity: Option<String>,
    category: Option<String>,
    since: Option<String>,
    until: Option<String>,
}

fn default_siem_limit() -> i64 { 100 }

async fn siem_get_events(
    db: web::Data<Arc<crate::storage::Database>>,
    query: web::Query<SiemQuery>,
) -> impl Responder {
    match db.siem_search("", query.limit, query.offset,
                         query.severity.as_deref(), query.category.as_deref(),
                         query.since.as_deref(), query.until.as_deref()).await {
        Ok(events) => HttpResponse::Ok().json(serde_json::json!({
            "events": events,
            "count": events.len(),
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Failed to query events: {}", e)
        })),
    }
}

#[derive(Deserialize)]
struct SiemAddRequest {
    source: String,
    #[serde(default = "default_info")]
    severity: String,
    #[serde(default = "default_general")]
    category: String,
    device_mac: Option<String>,
    message: String,
    raw_data: Option<String>,
}

fn default_info() -> String { "info".to_string() }
fn default_general() -> String { "general".to_string() }

async fn siem_add_event(
    db: web::Data<Arc<crate::storage::Database>>,
    body: web::Json<SiemAddRequest>,
) -> impl Responder {
    match db.siem_insert(&body.source, &body.severity, &body.category,
                         body.device_mac.as_deref(), &body.message,
                         body.raw_data.as_deref(), None, None).await {
        Ok(id) => {
            // Auto-prune if over budget
            let _ = db.siem_prune(SIEM_MAX_BYTES).await;
            HttpResponse::Ok().json(serde_json::json!({"id": id}))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("{}", e)
        })),
    }
}

#[derive(Deserialize)]
struct SiemSearchQuery {
    q: String,
    #[serde(default = "default_siem_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
    severity: Option<String>,
    category: Option<String>,
    since: Option<String>,
    until: Option<String>,
}

async fn siem_search_events(
    db: web::Data<Arc<crate::storage::Database>>,
    query: web::Query<SiemSearchQuery>,
) -> impl Responder {
    match db.siem_search(&query.q, query.limit, query.offset,
                         query.severity.as_deref(), query.category.as_deref(),
                         query.since.as_deref(), query.until.as_deref()).await {
        Ok(events) => HttpResponse::Ok().json(serde_json::json!({
            "events": events,
            "count": events.len(),
            "query": query.q,
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("{}", e)
        })),
    }
}

async fn siem_get_stats(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    let (count, db_size) = db.siem_count().await.unwrap_or((0, 0));
    let severity_counts = db.siem_severity_counts().await.unwrap_or_default();
    let category_counts = db.siem_category_counts().await.unwrap_or_default();
    let sources = db.siem_recent_sources().await.unwrap_or_default();

    HttpResponse::Ok().json(serde_json::json!({
        "total_events": count,
        "db_size_bytes": db_size,
        "db_size_mb": db_size as f64 / (1024.0 * 1024.0),
        "budget_bytes": SIEM_MAX_BYTES,
        "budget_gb": 4,
        "budget_used_pct": if SIEM_MAX_BYTES > 0 { (db_size as f64 / SIEM_MAX_BYTES as f64 * 100.0) } else { 0.0 },
        "severity_counts": severity_counts.iter().map(|(s,c)| serde_json::json!({"severity": s, "count": c})).collect::<Vec<_>>(),
        "category_counts": category_counts.iter().map(|(s,c)| serde_json::json!({"category": s, "count": c})).collect::<Vec<_>>(),
        "sources": sources,
    }))
}

async fn siem_prune_logs(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    match db.siem_prune(SIEM_MAX_BYTES).await {
        Ok(deleted) => HttpResponse::Ok().json(serde_json::json!({
            "pruned": deleted,
            "budget_gb": 4,
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("{}", e)
        })),
    }
}

async fn siem_export_events(
    db: web::Data<Arc<crate::storage::Database>>,
    query: web::Query<SiemQuery>,
) -> impl Responder {
    let limit = query.limit.min(10000);
    match db.siem_search("", limit, 0,
                         query.severity.as_deref(), query.category.as_deref(),
                         query.since.as_deref(), query.until.as_deref()).await {
        Ok(events) => {
            let ndjson: String = events.iter()
                .map(|e| serde_json::to_string(e).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\n");
            HttpResponse::Ok()
                .content_type("application/x-ndjson")
                .insert_header(("Content-Disposition", "attachment; filename=\"siem_events.ndjson\""))
                .body(ndjson)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("{}", e)
        })),
    }
}

async fn siem_get_forward_config(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    let row = sqlx::query("SELECT enabled, forward_type, endpoint FROM siem_forward_config WHERE id = 1")
        .fetch_optional(db.pool()).await;
    match row {
        Ok(Some(r)) => {
            use sqlx::Row;
            HttpResponse::Ok().json(serde_json::json!({
                "enabled": r.get::<i32, _>("enabled") != 0,
                "forward_type": r.get::<String, _>("forward_type"),
                "endpoint": r.get::<String, _>("endpoint"),
            }))
        }
        _ => HttpResponse::Ok().json(serde_json::json!({
            "enabled": false,
            "forward_type": "syslog",
            "endpoint": "",
        })),
    }
}

#[derive(Deserialize)]
struct SiemForwardRequest {
    enabled: bool,
    forward_type: String,
    endpoint: String,
}

async fn siem_set_forward_config(
    db: web::Data<Arc<crate::storage::Database>>,
    body: web::Json<SiemForwardRequest>,
) -> impl Responder {
    let result = sqlx::query(
        "INSERT OR REPLACE INTO siem_forward_config (id, enabled, forward_type, endpoint, updated_at)
         VALUES (1, ?, ?, ?, CURRENT_TIMESTAMP)"
    )
    .bind(body.enabled as i32)
    .bind(&body.forward_type)
    .bind(&body.endpoint)
    .execute(db.pool()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "saved": true,
            "enabled": body.enabled,
            "forward_type": body.forward_type,
            "endpoint": body.endpoint,
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("{}", e)
        })),
    }
}

// ============================================
// Sentinel Mode - Continuous Threat Monitoring
// ============================================

static SENTINEL_RUNNING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static SENTINEL_START_TIME: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));
static SENTINEL_SCAN_COUNT: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

async fn sentinel_start(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    {
        let running = SENTINEL_RUNNING.lock().unwrap();
        if *running {
            return HttpResponse::Ok().json(serde_json::json!({"status": "already_running"}));
        }
    }

    *SENTINEL_RUNNING.lock().unwrap() = true;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    *SENTINEL_START_TIME.lock().unwrap() = now;
    *SENTINEL_SCAN_COUNT.lock().unwrap() = 0;

    let _ = db.siem_insert("sentinel", "info", "system", None,
        "Sentinel Mode activated - continuous threat monitoring enabled", None, None, None).await;

    let caps = crate::sdr::SdrCapabilities::detect();

    // Start drone monitor if SDR available
    if caps.hackrf || caps.rtl_sdr {
        if !*DRONE_MONITOR_RUNNING.lock().unwrap() {
            tokio::spawn(async {
                let client = reqwest::Client::new();
                let _ = client.post("http://127.0.0.1:8085/api/sdr/drone/start").send().await;
            });
        }
    }

    // Start RTL-433
    if !*RTL433_RUNNING.lock().unwrap() {
        tokio::spawn(async {
            let client = reqwest::Client::new();
            let _ = client.post("http://127.0.0.1:8085/api/sdr/rtl433/start").send().await;
        });
    }

    // Start TSCM
    if !*TSCM_RUNNING.lock().unwrap() {
        tokio::spawn(async {
            let client = reqwest::Client::new();
            let _ = client.post("http://127.0.0.1:8085/api/sdr/tscm/sweep").send().await;
        });
    }

    // Sentinel watchlist scanning loop (every 30s)
    let db_clone = db.get_ref().clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            if !*SENTINEL_RUNNING.lock().unwrap() { break; }
            *SENTINEL_SCAN_COUNT.lock().unwrap() += 1;

            let client = reqwest::Client::new();

            // Check WiFi devices against watchlist and threat intel
            if let Ok(resp) = client.get("http://127.0.0.1:8085/api/wifi/devices").send().await {
                if let Ok(devices) = resp.json::<Vec<serde_json::Value>>().await {
                    for dev in &devices {
                        if let Some(mac) = dev.get("mac").and_then(|m| m.as_str()) {
                            if let Ok(Some(entry)) = db_clone.watchlist_check_mac(mac).await {
                                let _ = db_clone.siem_insert("sentinel", "critical", "watchlist_hit",
                                    Some(mac), &format!("WATCHLIST HIT: {} - {}", mac, entry.threat_type),
                                    None, None, None).await;
                            }
                            if let Some(threat) = crate::threat_intel::check_mac_threat(mac) {
                                let _ = db_clone.siem_insert("sentinel", "high", "threat_intel_match",
                                    Some(mac), &format!("Threat Intel: {} - {} ({})", mac, threat.vendor, threat.description),
                                    None, None, None).await;
                            }
                        }
                    }
                }
            }

            // Check BLE devices against watchlist
            if let Ok(resp) = client.get("http://127.0.0.1:8085/api/ble/devices").send().await {
                if let Ok(devices) = resp.json::<Vec<serde_json::Value>>().await {
                    for dev in &devices {
                        if let Some(mac) = dev.get("mac").and_then(|m| m.as_str()) {
                            if let Ok(Some(entry)) = db_clone.watchlist_check_mac(mac).await {
                                let _ = db_clone.siem_insert("sentinel", "critical", "watchlist_hit",
                                    Some(mac), &format!("BLE WATCHLIST HIT: {} - {}", mac, entry.threat_type),
                                    None, None, None).await;
                            }
                        }
                    }
                }
            }

            // Restart monitors if they stopped
            if !*TSCM_RUNNING.lock().unwrap() && *SENTINEL_RUNNING.lock().unwrap() {
                let _ = client.post("http://127.0.0.1:8085/api/sdr/tscm/sweep").send().await;
            }
            if !*DRONE_MONITOR_RUNNING.lock().unwrap() && *SENTINEL_RUNNING.lock().unwrap() {
                let _ = client.post("http://127.0.0.1:8085/api/sdr/drone/start").send().await;
            }
        }
    });

    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "monitors": { "wifi": true, "ble": true, "drone_rf": caps.hackrf || caps.rtl_sdr,
            "tscm": caps.hackrf || caps.rtl_sdr, "rtl433": caps.rtl_433,
            "watchlist": true, "threat_intel": true }
    }))
}

async fn sentinel_stop(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    *SENTINEL_RUNNING.lock().unwrap() = false;
    let _ = db.siem_insert("sentinel", "info", "system", None,
        "Sentinel Mode deactivated", None, None, None).await;
    let client = reqwest::Client::new();
    let _ = client.post("http://127.0.0.1:8085/api/sdr/drone/stop").send().await;
    let _ = client.post("http://127.0.0.1:8085/api/sdr/tscm/stop").send().await;
    let _ = client.post("http://127.0.0.1:8085/api/sdr/rtl433/stop").send().await;
    HttpResponse::Ok().json(serde_json::json!({"status": "stopped"}))
}

async fn sentinel_status() -> impl Responder {
    let running = *SENTINEL_RUNNING.lock().unwrap();
    let start = *SENTINEL_START_TIME.lock().unwrap();
    let scans = *SENTINEL_SCAN_COUNT.lock().unwrap();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    HttpResponse::Ok().json(serde_json::json!({
        "running": running,
        "uptime_seconds": if running && start > 0 { now - start } else { 0 },
        "scan_cycles": scans,
        "monitors": { "wifi": true, "ble": true,
            "drone_rf": *DRONE_MONITOR_RUNNING.lock().unwrap(),
            "tscm": *TSCM_RUNNING.lock().unwrap(),
            "rtl433": *RTL433_RUNNING.lock().unwrap() }
    }))
}

// ============================================
// Threat Watchlist
// ============================================

#[derive(Deserialize)]
struct WatchlistAddRequest {
    mac_address: Option<String>,
    signature: Option<String>,
    threat_type: String,
    description: Option<String>,
}

async fn watchlist_list(db: web::Data<Arc<crate::storage::Database>>) -> impl Responder {
    match db.watchlist_list().await {
        Ok(entries) => HttpResponse::Ok().json(serde_json::json!({"watchlist": entries})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

async fn watchlist_add(
    db: web::Data<Arc<crate::storage::Database>>,
    body: web::Json<WatchlistAddRequest>,
) -> impl Responder {
    match db.watchlist_add(body.mac_address.as_deref(), body.signature.as_deref(),
        &body.threat_type, body.description.as_deref(), "manual").await {
        Ok(id) => {
            let _ = db.siem_insert("watchlist", "info", "watchlist_update", body.mac_address.as_deref(),
                &format!("Added to watchlist: {} ({})",
                    body.mac_address.as_deref().or(body.signature.as_deref()).unwrap_or("?"), body.threat_type),
                None, None, None).await;
            HttpResponse::Ok().json(serde_json::json!({"id": id}))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

async fn watchlist_remove(
    db: web::Data<Arc<crate::storage::Database>>,
    path: web::Path<i64>,
) -> impl Responder {
    match db.watchlist_remove(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"removed": true})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

// ============================================
// Advanced SDR - Multi-device & Antenna Array
// ============================================

async fn get_sdr_devices() -> impl Responder {
    let caps = crate::sdr::SdrCapabilities::detect();
    let devices: Vec<serde_json::Value> = caps.devices.iter().map(|d| {
        serde_json::json!({
            "device_type": format!("{:?}", d.device_type),
            "index": d.index,
            "name": d.name,
            "serial": d.serial,
            "label": d.device_type.label(),
            "supports_tx": d.device_type.supports_tx(),
            "supports_df": d.device_type.supports_direction_finding(),
            "channels": d.device_type.channel_count(),
            "approx_price_usd": d.device_type.approx_price_usd(),
        })
    }).collect();

    let cheaper_alternatives: Vec<serde_json::Value> = vec![
        serde_json::json!({
            "device": "RTL-SDR Blog V4", "price_usd": 30, "freq_range": "500 kHz - 1766 MHz",
            "notes": "Best budget RX-only SDR. 8-bit ADC. Great for monitoring, ADS-B, trunking.",
            "buy": "https://www.rtl-sdr.com/buy-rtl-sdr-dvb-t-dongles/"
        }),
        serde_json::json!({
            "device": "Airspy Mini", "price_usd": 100, "freq_range": "24 - 1700 MHz",
            "notes": "12-bit ADC, 6 MHz bandwidth. Much better sensitivity than RTL-SDR. RX-only.",
            "buy": "https://airspy.com/airspy-mini/"
        }),
        serde_json::json!({
            "device": "SDRplay RSP1B", "price_usd": 110, "freq_range": "1 kHz - 2 GHz",
            "notes": "14-bit ADC, 10 MHz bandwidth. Best value wideband receiver. RX-only.",
            "buy": "https://www.sdrplay.com/rsp1b/"
        }),
        serde_json::json!({
            "device": "KrakenSDR", "price_usd": 150, "freq_range": "24 - 1766 MHz",
            "notes": "5x coherent RTL-SDR. Direction finding, passive radar. Best value for DF arrays.",
            "buy": "https://www.crowdsupply.com/krakenrf/krakensdr"
        }),
        serde_json::json!({
            "device": "ADALM-PLUTO", "price_usd": 150, "freq_range": "325 - 3800 MHz (hackable to 70-6000 MHz)",
            "notes": "Full duplex TX/RX. 12-bit ADC. Cheaper than HackRF for TX applications.",
            "buy": "https://www.analog.com/en/resources/evaluation-hardware-and-software/evaluation-boards-kits/adalm-pluto.html"
        }),
        serde_json::json!({
            "device": "Airspy R2", "price_usd": 170, "freq_range": "24 - 1700 MHz",
            "notes": "12-bit ADC, 10 MHz bandwidth. Professional-grade RX sensitivity.",
            "buy": "https://airspy.com/airspy-r2/"
        }),
    ];

    HttpResponse::Ok().json(serde_json::json!({
        "detected_devices": devices,
        "device_count": devices.len(),
        "has_direction_finding": devices.iter().any(|d| d["supports_df"].as_bool().unwrap_or(false)),
        "has_tx": devices.iter().any(|d| d["supports_tx"].as_bool().unwrap_or(false)),
        "cheaper_alternatives": cheaper_alternatives,
        "capabilities": {
            "rtl_sdr": caps.rtl_sdr,
            "hackrf": caps.hackrf,
            "rtl_433": caps.rtl_433,
            "limesdr": caps.limesdr,
        }
    }))
}

async fn get_antenna_config(
    db: web::Data<Arc<crate::storage::Database>>,
) -> impl Responder {
    let antennas = sqlx::query("SELECT a.*, d.device_type, d.serial, d.label as device_label FROM antenna_positions a LEFT JOIN sdr_devices d ON a.sdr_device_id = d.id ORDER BY a.id")
        .fetch_all(db.pool()).await;
    let arrays = sqlx::query("SELECT * FROM sdr_array_configs ORDER BY created_at DESC")
        .fetch_all(db.pool()).await;

    HttpResponse::Ok().json(serde_json::json!({
        "antennas": antennas.unwrap_or_default().iter().map(|r| {
            use sqlx::Row;
            serde_json::json!({
                "id": r.get::<i64, _>("id"),
                "sdr_device_id": r.get::<Option<i64>, _>("sdr_device_id"),
                "label": r.get::<String, _>("label"),
                "x_meters": r.get::<f64, _>("x_meters"),
                "y_meters": r.get::<f64, _>("y_meters"),
                "z_meters": r.get::<f64, _>("z_meters"),
                "bearing_degrees": r.get::<f64, _>("bearing_degrees"),
                "antenna_type": r.get::<String, _>("antenna_type"),
                "gain_dbi": r.get::<f64, _>("gain_dbi"),
            })
        }).collect::<Vec<_>>(),
        "arrays": arrays.unwrap_or_default().iter().map(|r| {
            use sqlx::Row;
            serde_json::json!({
                "id": r.get::<i64, _>("id"),
                "name": r.get::<String, _>("name"),
                "coherent": r.get::<i32, _>("coherent") != 0,
                "active": r.get::<i32, _>("active") != 0,
            })
        }).collect::<Vec<_>>(),
    }))
}

#[derive(Deserialize)]
struct AntennaAddRequest {
    sdr_device_id: Option<i64>,
    label: String,
    x_meters: f64,
    y_meters: f64,
    z_meters: f64,
    bearing_degrees: f64,
    antenna_type: String,
    gain_dbi: f64,
}

async fn add_antenna_position(
    db: web::Data<Arc<crate::storage::Database>>,
    body: web::Json<AntennaAddRequest>,
) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO antenna_positions (sdr_device_id, label, x_meters, y_meters, z_meters, bearing_degrees, antenna_type, gain_dbi) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    ).bind(body.sdr_device_id).bind(&body.label).bind(body.x_meters).bind(body.y_meters)
     .bind(body.z_meters).bind(body.bearing_degrees).bind(&body.antenna_type).bind(body.gain_dbi)
     .execute(db.pool()).await;
    match result {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({"id": r.last_insert_rowid()})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

async fn delete_antenna_position(
    db: web::Data<Arc<crate::storage::Database>>,
    path: web::Path<i64>,
) -> impl Responder {
    let _ = sqlx::query("DELETE FROM antenna_positions WHERE id = ?")
        .bind(path.into_inner()).execute(db.pool()).await;
    HttpResponse::Ok().json(serde_json::json!({"removed": true}))
}

// ============================================
// Soundboard endpoints
// ============================================

async fn soundboard_list_clips() -> impl Responder {
    let clips = crate::soundboard::list_clips();
    HttpResponse::Ok().json(serde_json::json!({"clips": clips}))
}

async fn soundboard_upload_clip(
    mut payload: actix_multipart::Multipart,
) -> impl Responder {
    use futures::StreamExt;
    let clips_dir = crate::soundboard::get_clips_dir();
    let mut saved_name = String::new();
    while let Some(Ok(mut field)) = payload.next().await {
        let content_disp = field.content_disposition().cloned();
        let filename = content_disp
            .as_ref()
            .and_then(|cd| cd.get_filename().map(|s| s.to_string()))
            .unwrap_or_else(|| "upload.wav".to_string());
        let safe_name: String = filename.chars()
            .map(|c: char| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
            .collect();
        let dest = clips_dir.join(&safe_name);
        let mut f = match std::fs::File::create(&dest) {
            Ok(f) => f,
            Err(e) => return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": format!("Failed to create file: {}", e)})),
        };
        use std::io::Write;
        while let Some(Ok(chunk)) = field.next().await {
            let _ = f.write_all(&chunk);
        }
        saved_name = safe_name;
    }
    if saved_name.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "No file uploaded"}));
    }
    HttpResponse::Ok().json(serde_json::json!({"uploaded": saved_name}))
}

async fn soundboard_delete_clip(
    path: web::Path<String>,
) -> impl Responder {
    match crate::soundboard::delete_clip(&path.into_inner()) {
        Ok(()) => HttpResponse::Ok().json(serde_json::json!({"deleted": true})),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

async fn soundboard_play_clip(
    path: web::Path<String>,
) -> impl Responder {
    match crate::soundboard::play_clip_local(&path.into_inner()) {
        Ok(msg) => HttpResponse::Ok().json(serde_json::json!({"status": msg})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

#[derive(Deserialize)]
struct TransmitRequest {
    frequency_hz: u64,
    modulation: Option<String>,
    power_dbm: Option<u8>,
    authorized: Option<bool>,
}

async fn soundboard_transmit_clip(
    path: web::Path<String>,
    body: web::Json<TransmitRequest>,
) -> impl Responder {
    if body.authorized != Some(true) {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "TX requires explicit authorization. Set authorized: true in request body."
        }));
    }
    let modulation = body.modulation.as_deref().unwrap_or("FM");
    let power = body.power_dbm.unwrap_or(20);
    match crate::soundboard::transmit_clip(&path.into_inner(), body.frequency_hz, modulation, power) {
        Ok(msg) => HttpResponse::Ok().json(serde_json::json!({"status": msg})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("{}", e)})),
    }
}

async fn soundboard_stream_clip(
    path: web::Path<String>,
) -> impl Responder {
    let clips_dir = crate::soundboard::get_clips_dir();
    let clip_path = clips_dir.join(path.into_inner());
    if !clip_path.exists() {
        return HttpResponse::NotFound().json(serde_json::json!({"error": "Clip not found"}));
    }
    match std::fs::read(&clip_path) {
        Ok(data) => {
            let ext = clip_path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
            let content_type = match ext.as_str() {
                "wav" => "audio/wav",
                "mp3" => "audio/mpeg",
                "ogg" => "audio/ogg",
                "flac" => "audio/flac",
                _ => "application/octet-stream",
            };
            HttpResponse::Ok().content_type(content_type).body(data)
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": format!("Failed to read clip: {}", e)})),
    }
}

// ============================================
// Fast Food / Commercial RF endpoints
// ============================================

async fn fastfood_get_database() -> impl Responder {
    let db = crate::fastfood_rf::commercial_rf_database();
    HttpResponse::Ok().json(serde_json::json!({"bands": db}))
}

static FASTFOOD_SIGNALS: Lazy<std::sync::Mutex<Vec<crate::fastfood_rf::CommercialRfSignal>>> =
    Lazy::new(|| std::sync::Mutex::new(Vec::new()));

#[derive(Deserialize)]
struct FastFoodScanRequest {
    band_group: Option<String>,
}

async fn fastfood_scan(
    body: Option<web::Json<FastFoodScanRequest>>,
) -> impl Responder {
    let band_filter = body.map(|b| b.band_group.clone()).flatten();
    let db = crate::fastfood_rf::commercial_rf_database();
    let bands: Vec<_> = if let Some(ref filter) = band_filter {
        db.iter().filter(|b| b.band_group.to_lowercase().contains(&filter.to_lowercase())).collect()
    } else {
        db.iter().collect()
    };

    // Check if rtl_fm or hackrf is available
    let has_rtl = std::process::Command::new("which").arg("rtl_fm")
        .output().map(|o| o.status.success()).unwrap_or(false);
    let has_hackrf = std::process::Command::new("which").arg("hackrf_transfer")
        .output().map(|o| o.status.success()).unwrap_or(false);

    if !has_rtl && !has_hackrf {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "No SDR hardware available (need rtl_fm or hackrf_transfer)",
            "bands_in_database": bands.len()
        }));
    }

    // For each band, do a quick power measurement via rtl_power or hackrf_sweep
    let mut detected = Vec::new();
    for band in &bands {
        let center = (band.start_hz + band.end_hz) / 2;
        // Quick power check with rtl_power (1 second sweep)
        if has_rtl && center < 1_766_000_000 {
            let output = std::process::Command::new("timeout")
                .args(["2", "rtl_power", "-f",
                    &format!("{}:{}:25000", band.start_hz, band.end_hz),
                    "-i", "1", "-1", "-"])
                .output();
            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                // Parse rtl_power output for max power
                let mut max_power: f64 = -100.0;
                for line in stdout.lines() {
                    for field in line.split(',').skip(6) {
                        if let Ok(p) = field.trim().parse::<f64>() {
                            if p > max_power { max_power = p; }
                        }
                    }
                }
                if max_power > -60.0 {
                    if let Some(sig) = crate::fastfood_rf::classify_signal(center, max_power) {
                        detected.push(sig);
                    }
                }
            }
        }
    }

    if let Ok(mut sigs) = FASTFOOD_SIGNALS.lock() {
        *sigs = detected.clone();
    }

    HttpResponse::Ok().json(serde_json::json!({
        "bands_scanned": bands.len(),
        "signals_detected": detected.len(),
        "signals": detected,
        "sdr_available": {"rtl_sdr": has_rtl, "hackrf": has_hackrf}
    }))
}

async fn fastfood_get_signals() -> impl Responder {
    let sigs = FASTFOOD_SIGNALS.lock().map(|s| s.clone()).unwrap_or_default();
    HttpResponse::Ok().json(serde_json::json!({"signals": sigs}))
}

// ============================================
// ML Inference endpoints
// ============================================

async fn ml_get_status() -> impl Responder {
    let status = crate::ml::get_ml_status();
    HttpResponse::Ok().json(status)
}

#[derive(Deserialize)]
struct MlClassifyRequest {
    iq_samples: Option<Vec<f32>>,
    features: Option<Vec<f32>>,
}

async fn ml_classify_signal(
    body: web::Json<MlClassifyRequest>,
) -> impl Responder {
    // If IQ samples provided, extract features first
    let features = if let Some(ref iq) = body.iq_samples {
        let mags = crate::ml::features::iq_to_fft_magnitude(iq, 1024.min(iq.len() / 2));
        let sf = crate::ml::features::extract_spectral_features(&mags);
        sf.to_vec()
    } else if let Some(ref f) = body.features {
        f.clone()
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Provide either iq_samples or features"
        }));
    };

    HttpResponse::Ok().json(serde_json::json!({
        "status": "no_model_loaded",
        "features_extracted": features.len(),
        "note": "Place ONNX models in ml/models/ and rebuild with --features ml to enable classification",
        "onnx_available": cfg!(feature = "ml")
    }))
}

#[derive(Deserialize)]
struct MlFeaturesRequest {
    iq_samples: Vec<f32>,
    window_size: Option<usize>,
}

async fn ml_extract_features(
    body: web::Json<MlFeaturesRequest>,
) -> impl Responder {
    let window = body.window_size.unwrap_or(1024);
    let magnitudes = crate::ml::features::iq_to_fft_magnitude(&body.iq_samples, window);
    let features = crate::ml::features::extract_spectral_features(&magnitudes);
    let harmonics = crate::ml::features::detect_harmonics(&magnitudes, 3, 2);

    HttpResponse::Ok().json(serde_json::json!({
        "spectral_features": features,
        "feature_vector": features.to_vec(),
        "num_magnitude_bins": magnitudes.len(),
        "harmonic_series_detected": harmonics.len(),
        "harmonics": harmonics
    }))
}
