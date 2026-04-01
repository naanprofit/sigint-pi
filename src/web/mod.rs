pub(crate) mod api;

pub use api::{AppState, WifiDeviceInfo, BleDeviceInfo, AlertInfo, AttackInfo, HardwareStatusInfo, TrackerInfoApi, GpsStatusInfo};

use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::broadcast;
use anyhow::Result;
use chrono::Utc;
use tracing::{info, warn};

use crate::config::Config;
use crate::storage::Database;
use crate::ScanEvent;

/// Find the static files directory
/// Tries multiple locations to support different deployment scenarios
pub(crate) fn find_static_dir() -> Option<PathBuf> {
    // Get current working directory
    let cwd = std::env::current_dir().ok();
    
    // Get executable directory
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    
    // Locations to try, in order of preference
    let candidates = [
        // 1. ./static (relative to cwd)
        cwd.as_ref().map(|d| d.join("static")),
        // 2. Next to executable
        exe_dir.as_ref().map(|d| d.join("static")),
        // 3. /app/static (container)
        Some(PathBuf::from("/app/static")),
        // 4. ~/sigint-deck/static (user install)
        dirs::home_dir().map(|h| h.join("sigint-pi").join("static")),
    ];
    
    for candidate in candidates.into_iter().flatten() {
        if candidate.exists() && candidate.is_dir() {
            // Verify index.html exists
            if candidate.join("index.html").exists() {
                info!("Found static directory at: {:?}", candidate);
                return Some(candidate);
            }
        }
    }
    
    warn!("Static files directory not found! Web dashboard will not be available.");
    warn!("Searched locations:");
    if let Some(ref d) = cwd {
        warn!("  - {:?}/static", d);
    }
    if let Some(ref d) = exe_dir {
        warn!("  - {:?}/static", d);
    }
    warn!("  - /app/static");
    if let Some(h) = dirs::home_dir() {
        warn!("  - {:?}/sigint-deck/static", h);
    }
    
    None
}

pub async fn start_server(
    db: Arc<Database>,
    config: Arc<Config>,
    state: Arc<AppState>,
    mut event_rx: broadcast::Receiver<ScanEvent>,
) -> Result<()> {
    let db_data = web::Data::new(db);
    let config_data = web::Data::new(config.clone());
    let state_data = web::Data::new(state.clone());
    
    let bind_addr = format!("{}:{}", config.web.bind_address, config.web.port);
    
    // Spawn event handler to update state from scan events
    let state_clone = state.clone();
    tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            match event {
                ScanEvent::WifiDevice(device) => {
                    let mut devices = state_clone.wifi_devices.write().await;
                    let now = Utc::now().timestamp();
                    
                    // Devices are "new" for 60 seconds after first seen
                    const NEW_DEVICE_WINDOW_SECS: i64 = 60;
                    
                    // Check if device exists
                    if let Some(existing) = devices.iter_mut().find(|d| d.mac == device.mac_address) {
                        existing.rssi = device.rssi;
                        existing.last_seen = now;
                        // Update is_new based on time since first seen
                        existing.is_new = (now - existing.first_seen) < NEW_DEVICE_WINDOW_SECS;
                    } else {
                        devices.push(WifiDeviceInfo {
                            mac: device.mac_address.clone(),
                            vendor: device.vendor.clone(),
                            ssid: device.ssid.clone(),
                            rssi: device.rssi,
                            channel: Some(device.channel),
                            is_ap: device.is_ap,
                            is_new: true,
                            first_seen: now,
                            last_seen: now,
                        });
                    }
                    
                    // Keep only recent devices (last 5 minutes)
                    let cutoff = now - 300;
                    devices.retain(|d| d.last_seen > cutoff);
                }
                ScanEvent::BleDevice(device) => {
                    let mut devices = state_clone.ble_devices.write().await;
                    let now = Utc::now().timestamp();
                    let is_tracker = matches!(device.device_type, 
                        crate::bluetooth::BleDeviceType::AirTag | 
                        crate::bluetooth::BleDeviceType::Tile
                    );
                    
                    // Devices are "new" for 60 seconds after first seen
                    const NEW_DEVICE_WINDOW_SECS: i64 = 60;
                    
                    // Convert tracker_info if present
                    let tracker_info_api = device.tracker_info.as_ref().map(|ti| TrackerInfoApi {
                        tracker_type: ti.tracker_type.clone(),
                        status: ti.status,
                        key_hint: ti.key_hint.clone(),
                        is_lost_mode: ti.is_lost_mode,
                        is_separated: ti.is_separated,
                        counter: ti.counter,
                    });
                    
                    if let Some(existing) = devices.iter_mut().find(|d| d.mac == device.mac_address) {
                        existing.rssi = device.rssi;
                        existing.last_seen = now;
                        // Update is_new based on time since first seen
                        existing.is_new = (now - existing.first_seen) < NEW_DEVICE_WINDOW_SECS;
                        // Update tracker info if present
                        if tracker_info_api.is_some() {
                            existing.tracker_info = tracker_info_api;
                        }
                    } else {
                        devices.push(BleDeviceInfo {
                            mac: device.mac_address.clone(),
                            name: device.name.clone(),
                            device_type: format!("{:?}", device.device_type),
                            vendor: device.vendor.clone(),
                            rssi: device.rssi,
                            is_new: true,
                            is_tracker,
                            first_seen: now,
                            last_seen: now,
                            tracker_info: tracker_info_api,
                        });
                    }
                    
                    let cutoff = now - 300;
                    devices.retain(|d| d.last_seen > cutoff);
                }
                ScanEvent::Attack(attack) => {
                    let mut attacks = state_clone.attacks.write().await;
                    let now = Utc::now().timestamp();
                    let id = attacks.len() as u64 + 1;
                    attacks.insert(0, AttackInfo {
                        id,
                        attack_type: format!("{:?}", attack.attack_type),
                        severity: format!("{:?}", attack.severity),
                        source_mac: attack.source_mac.clone(),
                        target_mac: attack.target_mac.clone(),
                        description: attack.description.clone(),
                        timestamp: now,
                    });
                    if attacks.len() > 100 {
                        attacks.pop();
                    }
                }
                ScanEvent::Alert { priority, message, device_mac } => {
                    let mut alerts = state_clone.alerts.write().await;
                    let now = Utc::now().timestamp();
                    let id = alerts.len() as u64 + 1;
                    alerts.insert(0, AlertInfo {
                        id,
                        title: "Alert".to_string(),
                        message,
                        priority: format!("{:?}", priority),
                        device_mac,
                        timestamp: now,
                    });
                    if alerts.len() > 100 {
                        alerts.pop();
                    }
                }
                ScanEvent::GpsUpdate(position) => {
                    let mut gps = state_clone.gps_status.write().await;
                    let now = Utc::now().timestamp();
                    let has_fix = matches!(position.fix_type, 
                        crate::gps::GpsFixType::Fix2D | 
                        crate::gps::GpsFixType::Fix3D |
                        crate::gps::GpsFixType::DGPS
                    );
                    gps.has_fix = has_fix;
                    gps.fix_type = format!("{:?}", position.fix_type);
                    // Only update coordinates if we have a fix
                    if has_fix {
                        gps.latitude = Some(position.latitude);
                        gps.longitude = Some(position.longitude);
                        gps.altitude = position.altitude;
                        gps.speed = position.speed;
                        gps.heading = position.heading;
                        gps.accuracy = position.accuracy;
                    } else {
                        // Clear stale coordinates when no fix
                        gps.latitude = None;
                        gps.longitude = None;
                        gps.altitude = None;
                        gps.speed = None;
                        gps.heading = None;
                        gps.accuracy = None;
                    }
                    gps.satellites = position.satellites;
                    gps.last_update = now;
                }
                _ => {}
            }
        }
    });
    
    // Determine static files directory
    // Try multiple locations: ./static, ../static, or relative to executable
    let static_dir = find_static_dir();
    info!("Static files directory: {:?}", static_dir);
    
    // Run actix in its own system
    let server = HttpServer::new(move || {
        let mut app = App::new()
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .app_data(state_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api::configure);
        
        // Only add static file service if directory exists
        if let Some(ref dir) = static_dir {
            if dir.exists() {
                app = app.service(
                    actix_files::Files::new("/", dir.clone())
                        .index_file("index.html")
                        .prefer_utf8(true)
                );
            }
        }
        
        app
    })
    .bind(&bind_addr)?
    .run();
    
    server.await?;

    Ok(())
}
