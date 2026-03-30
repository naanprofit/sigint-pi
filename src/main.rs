mod config;
mod wifi;
mod bluetooth;
mod gps;
mod learning;
mod storage;
mod alerts;
mod web;
mod cloud;
mod platform;
mod power;
mod settings;

#[cfg(feature = "simulation")]
mod simulation;

use platform::{Platform, PlatformCapabilities, HardwareStatus};
use power::PowerManager;
use settings::LEGAL_DISCLAIMER;

use anyhow::Result;
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::sync::broadcast;
use std::sync::Arc;

use crate::config::Config;
use crate::storage::Database;
use crate::wifi::WifiScanner;
use crate::bluetooth::BleScanner;
use crate::gps::GpsClient;
use crate::learning::DeviceLearner;
use crate::alerts::AlertManager;

#[derive(Debug, Clone)]
pub enum ScanEvent {
    WifiDevice(wifi::WifiDevice),
    BleDevice(bluetooth::BleDevice),
    GpsUpdate(gps::GpsPosition),
    Attack(wifi::AttackEvent),
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("SIGINT-Pi starting up...");

    // Show legal disclaimer on first run or when requested
    if std::env::var("SIGINT_SHOW_DISCLAIMER").is_ok() || 
       !std::path::Path::new("/data/.disclaimer_accepted").exists() {
        eprintln!("{}", LEGAL_DISCLAIMER);
        if !std::env::var("SIGINT_ACCEPT_DISCLAIMER").is_ok() {
            eprintln!("Set SIGINT_ACCEPT_DISCLAIMER=1 to accept and continue.");
            std::process::exit(1);
        }
        // Create marker file
        let _ = std::fs::write("/data/.disclaimer_accepted", "accepted");
    }

    // Platform detection and logging
    platform::log_platform_info();
    let platform_caps = PlatformCapabilities::detect();

    let config = Config::load()?;
    info!("Configuration loaded");

    let db = Database::new(&config.database.path).await?;
    db.migrate().await?;
    info!("Database initialized");

    let (event_tx, _) = broadcast::channel::<ScanEvent>(1000);
    let db = Arc::new(db);
    let config = Arc::new(config);

    let mut handles = vec![];

    // Check for simulation mode
    let simulation_mode = std::env::var("SIGINT_SIMULATION").is_ok();

    // Perform hardware capability checks
    let hw_status = platform::HardwareStatus::check_all(&config.wifi.interface);
    info!("Hardware status: {}", hw_status.summary());
    
    for warning in &hw_status.warnings {
        warn!("{}", warning);
    }
    for error in &hw_status.errors {
        error!("{}", error);
    }
    
    #[cfg(feature = "simulation")]
    if simulation_mode {
        info!("Running in SIMULATION MODE - no real hardware required");
        let sim_tx = event_tx.clone();
        handles.push(tokio::spawn(async move {
            let mut engine = simulation::SimulationEngine::new();
            engine.run(sim_tx).await;
        }));
    }

    // WiFi Scanner (skip in simulation mode)
    #[cfg(feature = "simulation")]
    let skip_wifi = simulation_mode;
    #[cfg(not(feature = "simulation"))]
    let skip_wifi = false;

    if config.wifi.enabled && !skip_wifi {
        let wifi_tx = event_tx.clone();
        let wifi_config = config.wifi.clone();
        handles.push(tokio::spawn(async move {
            let scanner = WifiScanner::new(wifi_config);
            if let Err(e) = scanner.run(wifi_tx).await {
                warn!("WiFi scanner error: {}", e);
            }
        }));
        info!("WiFi scanner started");
    }

    // BLE Scanner (skip in simulation mode)
    #[cfg(feature = "simulation")]
    let skip_ble = simulation_mode;
    #[cfg(not(feature = "simulation"))]
    let skip_ble = false;

    if config.bluetooth.enabled && !skip_ble {
        let ble_tx = event_tx.clone();
        let ble_config = config.bluetooth.clone();
        handles.push(tokio::spawn(async move {
            let scanner = BleScanner::new(ble_config);
            if let Err(e) = scanner.run(ble_tx).await {
                warn!("BLE scanner error: {}", e);
            }
        }));
        info!("BLE scanner started");
    }

    // GPS Client (skip in simulation mode)
    #[cfg(feature = "simulation")]
    let skip_gps = simulation_mode;
    #[cfg(not(feature = "simulation"))]
    let skip_gps = false;

    if config.gps.enabled && !skip_gps {
        let gps_tx = event_tx.clone();
        let gps_config = config.gps.clone();
        handles.push(tokio::spawn(async move {
            let client = GpsClient::new(gps_config);
            if let Err(e) = client.run(gps_tx).await {
                warn!("GPS client error: {}", e);
            }
        }));
        info!("GPS client started");
    }

    // Device Learner (processes events, updates baselines)
    {
        let mut learner_rx = event_tx.subscribe();
        let learner_db = db.clone();
        let learner_config = config.clone();
        handles.push(tokio::spawn(async move {
            let learner = DeviceLearner::new(learner_db, learner_config);
            learner.run(&mut learner_rx).await;
        }));
        info!("Device learner started");
    }

    // Alert Manager
    {
        let mut alert_rx = event_tx.subscribe();
        let alert_db = db.clone();
        let alert_config = config.clone();
        handles.push(tokio::spawn(async move {
            let manager = AlertManager::new(alert_db, alert_config).await;
            if let Err(e) = manager.run(&mut alert_rx).await {
                warn!("Alert manager error: {}", e);
            }
        }));
        info!("Alert manager started");
    }

    // Web UI - runs in main thread to avoid Send issues
    if config.web.enabled {
        let web_rx = event_tx.subscribe();
        let web_db = db.clone();
        let web_config = config.clone();
        let port = config.web.port;
        
        // Spawn web server task
        tokio::spawn(async move {
            if let Err(e) = web::start_server(web_db, web_config, web_rx).await {
                warn!("Web server error: {}", e);
            }
        });
        info!("Web server started on port {}", port);
    }

    info!("All systems operational. Press Ctrl+C to stop.");

    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, cleaning up...");

    for handle in handles {
        handle.abort();
    }

    info!("SIGINT-Pi shutdown complete");
    Ok(())
}
