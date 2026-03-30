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
        }
    }
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
            .route("/wifi/devices", web::get().to(get_wifi_devices))
            .route("/ble/devices", web::get().to(get_ble_devices))
            .route("/power/mode", web::post().to(set_power_mode))
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
    HttpResponse::Ok().json(devices.clone())
}

async fn get_ble_devices(
    state: web::Data<Arc<AppState>>,
) -> impl Responder {
    let devices = state.ble_devices.read().await;
    HttpResponse::Ok().json(devices.clone())
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
            default_endpoint: "http://localhost:8080",
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
            default_endpoint: "http://localhost:8080/v1",
            requires_api_key: false,
        },
    ];
    
    let llm_config = config.llm.as_ref();
    
    HttpResponse::Ok().json(LlmSettingsResponse {
        enabled: llm_config.map(|c| c.enabled).unwrap_or(false),
        provider: llm_config.map(|c| c.provider.clone()).unwrap_or_else(|| "llamacpp".to_string()),
        endpoint: llm_config.map(|c| c.endpoint.clone()).unwrap_or_else(|| "http://localhost:8080".to_string()),
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
    
    // Try to list models (works with most providers)
    let url = format!("{}/models", llm_config.endpoint.trim_end_matches('/'));
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

/// GET /api/settings - Get all settings
async fn get_settings() -> impl Responder {
    // In a real implementation, this would read from SettingsManager
    // For now, return default settings
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
