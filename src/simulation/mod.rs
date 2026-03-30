//! Simulation mode for testing without real hardware
//! Generates realistic fake WiFi and BLE device data

use chrono::{DateTime, Utc};
use rand::prelude::*;
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::info;

use crate::bluetooth::{BleDevice, BleDeviceType};
use crate::gps::{GpsFixType, GpsPosition};
use crate::wifi::{AttackEvent, AttackSeverity, AttackType, FrameType, WifiDevice};
use crate::ScanEvent;

/// Simulated device database
pub struct SimulationEngine {
    wifi_devices: Vec<SimulatedWifiDevice>,
    ble_devices: Vec<SimulatedBleDevice>,
    base_position: GpsPosition,
    rng: StdRng,
}

struct SimulatedWifiDevice {
    mac: String,
    vendor: String,
    ssid: Option<String>,
    is_ap: bool,
    base_rssi: i32,
    channel: u8,
    appearance_prob: f64,
}

struct SimulatedBleDevice {
    mac: String,
    name: Option<String>,
    device_type: BleDeviceType,
    base_rssi: i32,
    appearance_prob: f64,
}

impl SimulationEngine {
    pub fn new() -> Self {
        let mut rng = StdRng::from_entropy();
        
        // Create pool of simulated WiFi devices
        let wifi_devices = vec![
            // Home network devices
            SimulatedWifiDevice {
                mac: "AA:BB:CC:11:22:33".to_string(),
                vendor: "Apple".to_string(),
                ssid: Some("HomeNetwork".to_string()),
                is_ap: false,
                base_rssi: -45,
                channel: 6,
                appearance_prob: 0.95,
            },
            SimulatedWifiDevice {
                mac: "AA:BB:CC:11:22:34".to_string(),
                vendor: "Samsung".to_string(),
                ssid: Some("HomeNetwork".to_string()),
                is_ap: false,
                base_rssi: -55,
                channel: 6,
                appearance_prob: 0.85,
            },
            // Router
            SimulatedWifiDevice {
                mac: "00:11:22:33:44:55".to_string(),
                vendor: "Netgear".to_string(),
                ssid: Some("HomeNetwork".to_string()),
                is_ap: true,
                base_rssi: -35,
                channel: 6,
                appearance_prob: 1.0,
            },
            // Neighbor's network
            SimulatedWifiDevice {
                mac: "66:77:88:99:AA:BB".to_string(),
                vendor: "TP-Link".to_string(),
                ssid: Some("NETGEAR-5G".to_string()),
                is_ap: true,
                base_rssi: -75,
                channel: 44,
                appearance_prob: 0.9,
            },
            // Passing devices (cars, pedestrians)
            SimulatedWifiDevice {
                mac: "DE:AD:BE:EF:00:01".to_string(),
                vendor: "Google".to_string(),
                ssid: None,
                is_ap: false,
                base_rssi: -70,
                channel: 1,
                appearance_prob: 0.1,
            },
            SimulatedWifiDevice {
                mac: "DE:AD:BE:EF:00:02".to_string(),
                vendor: "Intel".to_string(),
                ssid: None,
                is_ap: false,
                base_rssi: -80,
                channel: 11,
                appearance_prob: 0.05,
            },
            // IoT devices
            SimulatedWifiDevice {
                mac: "B8:27:EB:12:34:56".to_string(),
                vendor: "Raspberry Pi Foundation".to_string(),
                ssid: Some("HomeNetwork".to_string()),
                is_ap: false,
                base_rssi: -50,
                channel: 6,
                appearance_prob: 0.99,
            },
            SimulatedWifiDevice {
                mac: "5C:CF:7F:AA:BB:CC".to_string(),
                vendor: "Espressif".to_string(),
                ssid: Some("HomeNetwork".to_string()),
                is_ap: false,
                base_rssi: -60,
                channel: 6,
                appearance_prob: 0.98,
            },
        ];

        let ble_devices = vec![
            SimulatedBleDevice {
                mac: "11:22:33:44:55:66".to_string(),
                name: Some("AirPods Pro".to_string()),
                device_type: BleDeviceType::Headphones,
                base_rssi: -55,
                appearance_prob: 0.8,
            },
            SimulatedBleDevice {
                mac: "22:33:44:55:66:77".to_string(),
                name: Some("Apple Watch".to_string()),
                device_type: BleDeviceType::Wearable,
                base_rssi: -60,
                appearance_prob: 0.85,
            },
            SimulatedBleDevice {
                mac: "33:44:55:66:77:88".to_string(),
                name: Some("Tile Mate".to_string()),
                device_type: BleDeviceType::Tile,
                base_rssi: -70,
                appearance_prob: 0.15, // Passing by
            },
            SimulatedBleDevice {
                mac: "44:55:66:77:88:99".to_string(),
                name: None,
                device_type: BleDeviceType::AirTag,
                base_rssi: -75,
                appearance_prob: 0.05, // Rare - trigger alert
            },
            SimulatedBleDevice {
                mac: "55:66:77:88:99:AA".to_string(),
                name: Some("JBL Speaker".to_string()),
                device_type: BleDeviceType::Speaker,
                base_rssi: -45,
                appearance_prob: 0.9,
            },
            SimulatedBleDevice {
                mac: "66:77:88:99:AA:BB".to_string(),
                name: Some("Fitbit Charge".to_string()),
                device_type: BleDeviceType::Fitness,
                base_rssi: -65,
                appearance_prob: 0.7,
            },
        ];

        let base_position = GpsPosition {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: Some(10.0),
            speed: Some(0.0),
            heading: None,
            accuracy: Some(5.0),
            fix_type: GpsFixType::Fix3D,
            satellites: 8,
            timestamp: Utc::now(),
        };

        Self {
            wifi_devices,
            ble_devices,
            base_position,
            rng,
        }
    }

    pub async fn run(&mut self, tx: broadcast::Sender<ScanEvent>) {
        info!("Starting simulation engine");
        
        let mut wifi_interval = tokio::time::interval(Duration::from_millis(500));
        let mut ble_interval = tokio::time::interval(Duration::from_millis(1000));
        let mut gps_interval = tokio::time::interval(Duration::from_secs(5));
        let mut attack_check = tokio::time::interval(Duration::from_secs(30));

        let mut new_device_counter = 0u64;

        loop {
            tokio::select! {
                _ = wifi_interval.tick() => {
                    for device in &self.wifi_devices {
                        if self.rng.gen::<f64>() < device.appearance_prob {
                            let rssi_variance: i32 = self.rng.gen_range(-8..8);
                            
                            let wifi_device = WifiDevice {
                                mac_address: device.mac.clone(),
                                rssi: device.base_rssi + rssi_variance,
                                channel: device.channel,
                                frame_type: FrameType::Data,
                                ssid: device.ssid.clone(),
                                bssid: if device.is_ap { Some(device.mac.clone()) } else { None },
                                is_ap: device.is_ap,
                                vendor: Some(device.vendor.clone()),
                                first_seen: Utc::now(),
                                last_seen: Utc::now(),
                                probe_requests: vec![],
                                data_frames_count: self.rng.gen_range(1..100),
                            };
                            
                            let _ = tx.send(ScanEvent::WifiDevice(wifi_device));
                        }
                    }

                    // Occasionally generate a new unknown device
                    if self.rng.gen::<f64>() < 0.02 {
                        new_device_counter += 1;
                        let new_device = WifiDevice {
                            mac_address: format!("FA:KE:{:02X}:{:02X}:{:02X}:{:02X}",
                                self.rng.gen::<u8>(), self.rng.gen::<u8>(),
                                self.rng.gen::<u8>(), new_device_counter as u8),
                            rssi: self.rng.gen_range(-85..-50),
                            channel: *[1, 6, 11, 36, 44].choose(&mut self.rng).unwrap(),
                            frame_type: FrameType::Management,
                            ssid: None,
                            bssid: None,
                            is_ap: false,
                            vendor: Some("Unknown".to_string()),
                            first_seen: Utc::now(),
                            last_seen: Utc::now(),
                            probe_requests: vec![],
                            data_frames_count: 1,
                        };
                        
                        info!("Simulation: New unknown device {}", new_device.mac_address);
                        let _ = tx.send(ScanEvent::WifiDevice(new_device));
                    }
                }

                _ = ble_interval.tick() => {
                    for device in &self.ble_devices {
                        if self.rng.gen::<f64>() < device.appearance_prob {
                            let rssi_variance: i32 = self.rng.gen_range(-5..5);
                            
                            let ble_device = BleDevice {
                                mac_address: device.mac.clone(),
                                name: device.name.clone(),
                                rssi: device.base_rssi + rssi_variance,
                                device_type: device.device_type,
                                manufacturer_data: None,
                                service_uuids: vec![],
                                is_connectable: true,
                                tx_power: Some(-10),
                                vendor: None,
                                first_seen: Utc::now(),
                                last_seen: Utc::now(),
                                tracker_info: None,
                            };
                            
                            let _ = tx.send(ScanEvent::BleDevice(ble_device));
                        }
                    }
                }

                _ = gps_interval.tick() => {
                    // Slight GPS drift
                    let lat_drift = self.rng.gen_range(-0.00001..0.00001);
                    let lon_drift = self.rng.gen_range(-0.00001..0.00001);
                    
                    let position = GpsPosition {
                        latitude: self.base_position.latitude + lat_drift,
                        longitude: self.base_position.longitude + lon_drift,
                        altitude: self.base_position.altitude,
                        speed: Some(self.rng.gen_range(0.0..2.0)),
                        heading: Some(self.rng.gen_range(0.0..360.0)),
                        accuracy: Some(self.rng.gen_range(3.0..10.0)),
                        fix_type: GpsFixType::Fix3D,
                        satellites: self.rng.gen_range(6..12),
                        timestamp: Utc::now(),
                    };
                    
                    let _ = tx.send(ScanEvent::GpsUpdate(position));
                }

                _ = attack_check.tick() => {
                    // Occasionally simulate an attack (1% chance every 30 seconds)
                    if self.rng.gen::<f64>() < 0.01 {
                        let attack = AttackEvent {
                            attack_type: *[
                                AttackType::DeauthFlood,
                                AttackType::EvilTwin,
                                AttackType::BeaconFlood,
                            ].choose(&mut self.rng).unwrap(),
                            source_mac: format!("EV:IL:{:02X}:{:02X}:{:02X}:{:02X}",
                                self.rng.gen::<u8>(), self.rng.gen::<u8>(),
                                self.rng.gen::<u8>(), self.rng.gen::<u8>()),
                            target_mac: Some("FF:FF:FF:FF:FF:FF".to_string()),
                            bssid: Some("00:11:22:33:44:55".to_string()),
                            severity: AttackSeverity::High,
                            description: "Simulated attack for testing".to_string(),
                            timestamp: Utc::now(),
                            evidence: crate::wifi::AttackEvidence {
                                frame_count: self.rng.gen_range(10..100),
                                time_window_seconds: 10,
                                unique_targets: 1,
                                channels_affected: vec![6],
                            },
                        };
                        
                        info!("Simulation: Generated attack event {:?}", attack.attack_type);
                        let _ = tx.send(ScanEvent::Attack(attack));
                    }
                }
            }
        }
    }
}

impl Default for SimulationEngine {
    fn default() -> Self {
        Self::new()
    }
}
