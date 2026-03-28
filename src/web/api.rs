use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;
use crate::storage::Database;

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

async fn get_status(config: web::Data<Arc<Config>>) -> impl Responder {
    HttpResponse::Ok().json(StatusResponse {
        status: "running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // TODO: track uptime
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
    _db: web::Data<Arc<Database>>,
    _query: web::Query<DevicesQuery>,
) -> impl Responder {
    // TODO: Implement full device listing
    HttpResponse::Ok().json(Vec::<DeviceResponse>::new())
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
    _db: web::Data<Arc<Database>>,
    _query: web::Query<AlertsQuery>,
) -> impl Responder {
    // TODO: Implement alerts listing
    HttpResponse::Ok().json(Vec::<serde_json::Value>::new())
}

async fn get_attacks(
    _db: web::Data<Arc<Database>>,
) -> impl Responder {
    // TODO: Implement attacks listing
    HttpResponse::Ok().json(Vec::<serde_json::Value>::new())
}

async fn get_stats(
    db: web::Data<Arc<Database>>,
) -> impl Responder {
    match db.get_device_counts(1).await {
        Ok(counts) => HttpResponse::Ok().json(serde_json::json!({
            "wifi_devices_total": counts.wifi_total,
            "wifi_devices_baseline": counts.wifi_baseline,
            "ble_devices_total": counts.ble_total,
            "ble_devices_baseline": counts.ble_baseline,
            "wifi_devices_unknown": counts.wifi_total - counts.wifi_baseline,
            "ble_devices_unknown": counts.ble_total - counts.ble_baseline,
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    }
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
