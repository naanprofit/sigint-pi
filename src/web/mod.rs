mod api;

pub use api::{AppState, WifiDeviceInfo, BleDeviceInfo, AlertInfo, AttackInfo, HardwareStatusInfo, TrackerInfoApi};

use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tokio::sync::broadcast;
use anyhow::Result;
use chrono::Utc;

use crate::config::Config;
use crate::storage::Database;
use crate::ScanEvent;

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
                _ => {}
            }
        }
    });
    
    // Run actix in its own system
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .app_data(state_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api::configure)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run();
    
    server.await?;

    Ok(())
}
