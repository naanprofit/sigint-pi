//! Power management module
//!
//! Provides runtime power profile control for battery-powered operation.
//! Adjusts scan intervals and resource usage based on selected mode.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

/// Power mode profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PowerMode {
    /// Maximum performance, highest power usage
    Performance,
    /// Balanced performance and power
    #[default]
    Balanced,
    /// Extended battery life, slower scans
    LowPower,
}

impl PowerMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "performance" | "high" | "max" => PowerMode::Performance,
            "lowpower" | "low" | "powersave" | "eco" => PowerMode::LowPower,
            _ => PowerMode::Balanced,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PowerMode::Performance => "Performance",
            PowerMode::Balanced => "Balanced",
            PowerMode::LowPower => "Low Power",
        }
    }
}

/// Power profile settings derived from power mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerProfile {
    pub mode: PowerMode,
    pub wifi_scan_interval_ms: u64,
    pub ble_scan_interval_ms: u64,
    pub gps_update_interval_ms: u64,
    pub web_refresh_interval_ms: u64,
    pub enable_pcap: bool,
    pub enable_attack_detection: bool,
    pub enable_learning: bool,
    pub db_write_batch_size: usize,
}

impl PowerProfile {
    /// Create a profile for the given power mode
    pub fn for_mode(mode: PowerMode) -> Self {
        match mode {
            PowerMode::Performance => Self {
                mode,
                wifi_scan_interval_ms: 1000,    // 1 second
                ble_scan_interval_ms: 2000,     // 2 seconds
                gps_update_interval_ms: 1000,   // 1 second
                web_refresh_interval_ms: 1000,  // 1 second
                enable_pcap: true,
                enable_attack_detection: true,
                enable_learning: true,
                db_write_batch_size: 1,         // Immediate writes
            },
            PowerMode::Balanced => Self {
                mode,
                wifi_scan_interval_ms: 5000,    // 5 seconds
                ble_scan_interval_ms: 5000,     // 5 seconds
                gps_update_interval_ms: 5000,   // 5 seconds
                web_refresh_interval_ms: 5000,  // 5 seconds
                enable_pcap: true,
                enable_attack_detection: true,
                enable_learning: true,
                db_write_batch_size: 10,        // Batch writes
            },
            PowerMode::LowPower => Self {
                mode,
                wifi_scan_interval_ms: 30000,   // 30 seconds
                ble_scan_interval_ms: 30000,    // 30 seconds
                gps_update_interval_ms: 60000,  // 1 minute
                web_refresh_interval_ms: 30000, // 30 seconds
                enable_pcap: false,             // Save disk I/O
                enable_attack_detection: false, // Reduce CPU
                enable_learning: false,         // Reduce CPU
                db_write_batch_size: 50,        // Large batches
            },
        }
    }
}

/// Power manager for runtime profile switching
#[derive(Debug, Clone)]
pub struct PowerManager {
    current_profile: Arc<RwLock<PowerProfile>>,
}

impl PowerManager {
    /// Create a new power manager with the default mode
    pub fn new() -> Self {
        Self {
            current_profile: Arc::new(RwLock::new(PowerProfile::for_mode(PowerMode::default()))),
        }
    }

    /// Create a power manager with a specific initial mode
    pub fn with_mode(mode: PowerMode) -> Self {
        Self {
            current_profile: Arc::new(RwLock::new(PowerProfile::for_mode(mode))),
        }
    }

    /// Get the current power profile
    pub async fn get_profile(&self) -> PowerProfile {
        self.current_profile.read().await.clone()
    }

    /// Get the current power mode
    pub async fn get_mode(&self) -> PowerMode {
        self.current_profile.read().await.mode
    }

    /// Set the power mode
    pub async fn set_mode(&self, mode: PowerMode) {
        let mut profile = self.current_profile.write().await;
        if profile.mode != mode {
            info!("Switching power mode: {} -> {}", profile.mode.name(), mode.name());
            *profile = PowerProfile::for_mode(mode);
        }
    }

    /// Get the WiFi scan interval in milliseconds
    pub async fn wifi_scan_interval(&self) -> u64 {
        self.current_profile.read().await.wifi_scan_interval_ms
    }

    /// Get the BLE scan interval in milliseconds
    pub async fn ble_scan_interval(&self) -> u64 {
        self.current_profile.read().await.ble_scan_interval_ms
    }

    /// Get the GPS update interval in milliseconds
    pub async fn gps_update_interval(&self) -> u64 {
        self.current_profile.read().await.gps_update_interval_ms
    }

    /// Check if PCAP recording is enabled
    pub async fn pcap_enabled(&self) -> bool {
        self.current_profile.read().await.enable_pcap
    }

    /// Check if attack detection is enabled
    pub async fn attack_detection_enabled(&self) -> bool {
        self.current_profile.read().await.enable_attack_detection
    }

    /// Check if learning is enabled
    pub async fn learning_enabled(&self) -> bool {
        self.current_profile.read().await.enable_learning
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Power status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerStatus {
    pub mode: PowerMode,
    pub mode_name: String,
    pub profile: PowerProfile,
    pub battery_present: bool,
    pub battery_percent: Option<u8>,
    pub on_ac_power: bool,
}

impl PowerStatus {
    /// Create a power status from current state
    pub fn from_manager_and_battery(
        profile: PowerProfile,
        battery_present: bool,
        battery_percent: Option<u8>,
        on_ac_power: bool,
    ) -> Self {
        Self {
            mode: profile.mode,
            mode_name: profile.mode.name().to_string(),
            profile,
            battery_present,
            battery_percent,
            on_ac_power,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_power_manager() {
        let manager = PowerManager::new();
        
        // Default should be balanced
        assert_eq!(manager.get_mode().await, PowerMode::Balanced);
        
        // Switch to performance
        manager.set_mode(PowerMode::Performance).await;
        assert_eq!(manager.get_mode().await, PowerMode::Performance);
        assert_eq!(manager.wifi_scan_interval().await, 1000);
        
        // Switch to low power
        manager.set_mode(PowerMode::LowPower).await;
        assert_eq!(manager.get_mode().await, PowerMode::LowPower);
        assert_eq!(manager.wifi_scan_interval().await, 30000);
        assert!(!manager.pcap_enabled().await);
    }

    #[test]
    fn test_power_mode_from_str() {
        assert_eq!(PowerMode::from_str("performance"), PowerMode::Performance);
        assert_eq!(PowerMode::from_str("high"), PowerMode::Performance);
        assert_eq!(PowerMode::from_str("lowpower"), PowerMode::LowPower);
        assert_eq!(PowerMode::from_str("eco"), PowerMode::LowPower);
        assert_eq!(PowerMode::from_str("balanced"), PowerMode::Balanced);
        assert_eq!(PowerMode::from_str("unknown"), PowerMode::Balanced);
    }
}
