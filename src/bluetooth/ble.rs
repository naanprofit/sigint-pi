use anyhow::{Context, Result};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::config::BluetoothConfig;
use crate::ScanEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BleDevice {
    pub mac_address: String,
    pub name: Option<String>,
    pub rssi: i32,
    pub device_type: BleDeviceType,
    pub manufacturer_data: Option<Vec<u8>>,
    pub service_uuids: Vec<String>,
    pub is_connectable: bool,
    pub tx_power: Option<i8>,
    pub vendor: Option<String>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BleDeviceType {
    Unknown,
    Phone,
    Computer,
    Wearable,
    Headphones,
    Speaker,
    Beacon,
    AirTag,
    Tile,
    SmartHome,
    Fitness,
    Medical,
}

// Apple AirTag and Find My network identifiers
const APPLE_COMPANY_ID: u16 = 0x004C;
const AIRTAG_TYPE: u8 = 0x12; // Find My network
const TILE_COMPANY_ID: u16 = 0x0157;

pub struct BleScanner {
    config: BluetoothConfig,
}

impl BleScanner {
    pub fn new(config: BluetoothConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, tx: broadcast::Sender<ScanEvent>) -> Result<()> {
        let manager = Manager::new().await.context("Failed to create BLE manager")?;
        
        let adapters = manager.adapters().await.context("Failed to get BLE adapters")?;
        let adapter = adapters
            .into_iter()
            .next()
            .context("No BLE adapter found")?;

        info!("BLE scanner using adapter: {:?}", adapter.adapter_info().await?);

        loop {
            // Start scanning
            adapter.start_scan(ScanFilter::default()).await
                .context("Failed to start BLE scan")?;

            tokio::time::sleep(Duration::from_millis(self.config.scan_interval_ms)).await;

            // Get discovered peripherals
            let peripherals = adapter.peripherals().await
                .context("Failed to get peripherals")?;

            for peripheral in peripherals {
                if let Some(ble_device) = self.process_peripheral(&peripheral).await {
                    // Filter by RSSI threshold
                    if ble_device.rssi >= self.config.rssi_threshold {
                        let _ = tx.send(ScanEvent::BleDevice(ble_device));
                    }
                }
            }

            // Stop scanning between intervals
            adapter.stop_scan().await.ok();
        }
    }

    async fn process_peripheral(&self, peripheral: &Peripheral) -> Option<BleDevice> {
        let properties = peripheral.properties().await.ok()??;
        let now = Utc::now();

        let mac_address = peripheral.id().to_string();
        let rssi = properties.rssi.unwrap_or(-100) as i32;

        // Determine device type from manufacturer data
        let mfg_data = &properties.manufacturer_data;
        let (device_type, manufacturer_data) = if !mfg_data.is_empty() {
            let mfg_bytes: Vec<u8> = mfg_data.values().flatten().copied().collect();
            let dtype = self.classify_device(&properties, &mfg_bytes);
            (dtype, Some(mfg_bytes))
        } else {
            (self.classify_device(&properties, &[]), None)
        };

        // Skip AirTags if not configured to detect them
        if device_type == BleDeviceType::AirTag && !self.config.detect_airtags {
            return None;
        }

        let service_uuids: Vec<String> = properties
            .services
            .iter()
            .map(|uuid| uuid.to_string())
            .collect();

        Some(BleDevice {
            mac_address,
            name: properties.local_name.clone(),
            rssi,
            device_type,
            manufacturer_data,
            service_uuids,
            is_connectable: true,  // Simplified
            tx_power: properties.tx_power_level.map(|p| p as i8),
            vendor: None, // Will be filled by OUI lookup
            first_seen: now,
            last_seen: now,
        })
    }

    fn classify_device(&self, properties: &btleplug::api::PeripheralProperties, _mfg_data: &[u8]) -> BleDeviceType {
        // Check manufacturer data for specific devices
        let mfg_map = &properties.manufacturer_data;
        if !mfg_map.is_empty() {
            // Check for Apple AirTag / Find My
            if let Some(apple_data) = mfg_map.get(&APPLE_COMPANY_ID) {
                if !apple_data.is_empty() && apple_data[0] == AIRTAG_TYPE {
                    return BleDeviceType::AirTag;
                }
            }

            // Check for Tile
            if mfg_map.contains_key(&TILE_COMPANY_ID) {
                return BleDeviceType::Tile;
            }
        }

        // Check service UUIDs
        for uuid in &properties.services {
            let uuid_str = uuid.to_string().to_lowercase();
            
            // Heart rate service
            if uuid_str.contains("180d") {
                return BleDeviceType::Fitness;
            }
            // Blood pressure
            if uuid_str.contains("1810") {
                return BleDeviceType::Medical;
            }
            // Audio (headphones/speakers)
            if uuid_str.contains("110a") || uuid_str.contains("110b") {
                return BleDeviceType::Headphones;
            }
            // iBeacon / Eddystone
            if uuid_str.contains("feaa") {
                return BleDeviceType::Beacon;
            }
        }

        // Check name for hints
        if let Some(ref name) = properties.local_name {
            let name_lower = name.to_lowercase();
            
            if name_lower.contains("phone") || name_lower.contains("iphone") || name_lower.contains("galaxy") {
                return BleDeviceType::Phone;
            }
            if name_lower.contains("macbook") || name_lower.contains("laptop") || name_lower.contains("pc") {
                return BleDeviceType::Computer;
            }
            if name_lower.contains("watch") || name_lower.contains("band") || name_lower.contains("fitbit") {
                return BleDeviceType::Wearable;
            }
            if name_lower.contains("airpod") || name_lower.contains("buds") || name_lower.contains("headphone") {
                return BleDeviceType::Headphones;
            }
            if name_lower.contains("speaker") || name_lower.contains("sonos") || name_lower.contains("jbl") {
                return BleDeviceType::Speaker;
            }
        }

        BleDeviceType::Unknown
    }
}

impl BleDevice {
    pub fn is_tracker(&self) -> bool {
        matches!(self.device_type, BleDeviceType::AirTag | BleDeviceType::Tile)
    }
}
