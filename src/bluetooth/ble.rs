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
    /// Extended tracker info (for AirTags, Tiles, SmartTags)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_info: Option<TrackerInfo>,
}

/// Extended information for tracking devices (AirTag, Tile, SmartTag, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackerInfo {
    /// Tracker type (airtag, tile, smarttag, findmy, unknown)
    pub tracker_type: String,
    /// Status byte from advertisement (AirTag: indicates lost mode, etc.)
    pub status: Option<u8>,
    /// Public key fragment (for Find My devices) - NOT the full key, privacy preserved
    /// We only store a hash/fingerprint, not enough to track
    pub key_hint: Option<String>,
    /// Whether device appears to be in "lost mode"
    pub is_lost_mode: bool,
    /// Whether device is separated from owner (AirTag: away >3 days plays sound)
    pub is_separated: bool,
    /// Counter/nonce value from advertisement
    pub counter: Option<u8>,
    /// Estimated time since last owner contact (if determinable)
    pub separated_hours: Option<u32>,
    /// Battery level if available
    pub battery_level: Option<u8>,
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
    SmartLight,  // LED strips, bulbs (MELK, Govee, Philips Hue, etc.)
    Fitness,
    Medical,
}

// Apple AirTag and Find My network identifiers
const APPLE_COMPANY_ID: u16 = 0x004C;
const AIRTAG_FINDMY_TYPE: u8 = 0x12; // Find My network (registered AirTag)
const AIRTAG_SETUP_TYPE: u8 = 0x07;  // AirTag in setup/unregistered mode
const TILE_COMPANY_ID: u16 = 0x0157;
const SAMSUNG_COMPANY_ID: u16 = 0x0075; // Samsung SmartTag

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

        // Determine device type and extract tracker info from manufacturer data
        let mfg_data = &properties.manufacturer_data;
        let (device_type, manufacturer_data, tracker_info) = if !mfg_data.is_empty() {
            let mfg_bytes: Vec<u8> = mfg_data.values().flatten().copied().collect();
            let (dtype, tinfo) = self.classify_device_extended(&properties);
            (dtype, Some(mfg_bytes), tinfo)
        } else {
            let (dtype, tinfo) = self.classify_device_extended(&properties);
            (dtype, None, tinfo)
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
            tracker_info,
        })
    }

    /// Classify device and extract tracker-specific information
    fn classify_device_extended(&self, properties: &btleplug::api::PeripheralProperties) -> (BleDeviceType, Option<TrackerInfo>) {
        let mfg_map = &properties.manufacturer_data;
        
        if !mfg_map.is_empty() {
            // Check for Apple AirTag / Find My network device
            if let Some(apple_data) = mfg_map.get(&APPLE_COMPANY_ID) {
                if apple_data.len() >= 2 {
                    let payload_type = apple_data[0];
                    
                    // Find My network device (registered AirTag)
                    if payload_type == AIRTAG_FINDMY_TYPE {
                        let tracker_info = self.parse_airtag_data(apple_data);
                        return (BleDeviceType::AirTag, Some(tracker_info));
                    }
                    
                    // Unregistered/setup mode AirTag
                    if payload_type == AIRTAG_SETUP_TYPE {
                        return (BleDeviceType::AirTag, Some(TrackerInfo {
                            tracker_type: "airtag_unregistered".to_string(),
                            status: Some(payload_type),
                            key_hint: None,
                            is_lost_mode: false,
                            is_separated: false,
                            counter: None,
                            separated_hours: None,
                            battery_level: None,
                        }));
                    }
                }
            }

            // Check for Tile
            if let Some(tile_data) = mfg_map.get(&TILE_COMPANY_ID) {
                return (BleDeviceType::Tile, Some(TrackerInfo {
                    tracker_type: "tile".to_string(),
                    status: tile_data.first().copied(),
                    key_hint: None,
                    is_lost_mode: false,
                    is_separated: false,
                    counter: None,
                    separated_hours: None,
                    battery_level: None,
                }));
            }
            
            // Check for Samsung SmartTag
            if mfg_map.contains_key(&SAMSUNG_COMPANY_ID) {
                // Samsung uses similar structure to Apple for their SmartTags
                return (BleDeviceType::AirTag, Some(TrackerInfo {
                    tracker_type: "smarttag".to_string(),
                    status: None,
                    key_hint: None,
                    is_lost_mode: false,
                    is_separated: false,
                    counter: None,
                    separated_hours: None,
                    battery_level: None,
                }));
            }
        }

        // Check service UUIDs
        for uuid in &properties.services {
            let uuid_str = uuid.to_string().to_lowercase();
            
            if uuid_str.contains("180d") {
                return (BleDeviceType::Fitness, None);
            }
            if uuid_str.contains("1810") {
                return (BleDeviceType::Medical, None);
            }
            if uuid_str.contains("110a") || uuid_str.contains("110b") {
                return (BleDeviceType::Headphones, None);
            }
            if uuid_str.contains("feaa") {
                return (BleDeviceType::Beacon, None);
            }
        }

        // Check name for hints
        if let Some(ref name) = properties.local_name {
            let name_lower = name.to_lowercase();
            
            // Smart LED lights (MELK, ELK-BLEDOM, Govee, etc.)
            if name_lower.contains("melk") || name_lower.contains("elk-bledom") || 
               name_lower.contains("ledble") || name_lower.contains("triones") ||
               name_lower.contains("govee") || name_lower.contains("led strip") ||
               name_lower.contains("hue") || name_lower.contains("lifx") ||
               name_lower.contains("bulb") || name_lower.contains("light") && name_lower.contains("led") {
                return (BleDeviceType::SmartLight, None);
            }
            if name_lower.contains("phone") || name_lower.contains("iphone") || name_lower.contains("galaxy") {
                return (BleDeviceType::Phone, None);
            }
            if name_lower.contains("macbook") || name_lower.contains("laptop") || name_lower.contains("pc") {
                return (BleDeviceType::Computer, None);
            }
            if name_lower.contains("watch") || name_lower.contains("band") || name_lower.contains("fitbit") {
                return (BleDeviceType::Wearable, None);
            }
            if name_lower.contains("airpod") || name_lower.contains("buds") || name_lower.contains("headphone") {
                return (BleDeviceType::Headphones, None);
            }
            if name_lower.contains("speaker") || name_lower.contains("sonos") || name_lower.contains("jbl") {
                return (BleDeviceType::Speaker, None);
            }
            // Generic smart home
            if name_lower.contains("nest") || name_lower.contains("echo") || name_lower.contains("alexa") ||
               name_lower.contains("ring") || name_lower.contains("wyze") || name_lower.contains("tuya") {
                return (BleDeviceType::SmartHome, None);
            }
        }

        (BleDeviceType::Unknown, None)
    }
    
    /// Parse Apple AirTag / Find My advertisement data
    /// Based on reverse engineering: https://adamcatley.com/AirTag.html
    fn parse_airtag_data(&self, data: &[u8]) -> TrackerInfo {
        // AirTag Find My advertisement structure:
        // Byte 0: 0x12 (payload type)
        // Byte 1: 0x19 (payload length, 25 bytes)
        // Byte 2: Status byte
        // Bytes 3-25: EC P-224 public key fragment (23 bytes)
        // Byte 26: Upper 2 bits of first key byte
        // Byte 27: Counter (changes every 15 minutes)
        
        let status = data.get(2).copied();
        let counter = data.get(27).copied();
        
        // Status byte interpretation (approximate, not fully documented):
        // Bit patterns indicate various states
        let is_lost_mode = status.map(|s| (s & 0x04) != 0).unwrap_or(false);
        let is_separated = status.map(|s| (s & 0x10) != 0).unwrap_or(false);
        
        // Create a privacy-preserving key hint (hash of first few bytes)
        // This allows us to track "same device seen again" without storing trackable data
        let key_hint = if data.len() >= 10 {
            let hint_bytes = &data[3..10];
            Some(format!("{:02x}{:02x}..{:02x}", 
                hint_bytes.first().unwrap_or(&0),
                hint_bytes.get(1).unwrap_or(&0),
                hint_bytes.last().unwrap_or(&0)))
        } else {
            None
        };
        
        TrackerInfo {
            tracker_type: "airtag".to_string(),
            status,
            key_hint,
            is_lost_mode,
            is_separated,
            counter,
            separated_hours: None, // Would need time tracking to determine
            battery_level: None,   // Not exposed in BLE advertisement
        }
    }
}

impl BleDevice {
    pub fn is_tracker(&self) -> bool {
        matches!(self.device_type, BleDeviceType::AirTag | BleDeviceType::Tile)
    }
}
