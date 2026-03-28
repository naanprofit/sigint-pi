use anyhow::{Context, Result};
use pcap::{Capture, Device};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};
use std::time::Duration;

use crate::config::WifiConfig;
use crate::ScanEvent;
use super::parser::{parse_wifi_frame, WifiDevice};
use super::attacks::AttackDetector;

pub struct WifiScanner {
    config: WifiConfig,
}

impl WifiScanner {
    pub fn new(config: WifiConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, tx: broadcast::Sender<ScanEvent>) -> Result<()> {
        let interface = &self.config.interface;
        
        info!("Opening WiFi interface {} in monitor mode", interface);
        
        // Find the device
        let devices = Device::list().context("Failed to list network devices")?;
        let device = devices
            .into_iter()
            .find(|d| d.name == *interface)
            .context(format!("Interface {} not found", interface))?;

        // Open in monitor mode
        let mut cap = Capture::from_device(device)
            .context("Failed to create capture")?
            .promisc(true)
            .snaplen(65535)
            .timeout(1000)
            .open()
            .context("Failed to open capture (is interface in monitor mode?)")?;

        // Set datalink to IEEE 802.11 with radiotap header
        cap.set_datalink(pcap::Linktype::IEEE802_11_RADIOTAP)
            .context("Failed to set datalink type")?;

        info!("WiFi capture started on {}", interface);

        loop {
            match cap.next_packet() {
                Ok(packet) => {
                    if let Some(wifi_device) = parse_wifi_frame(packet.data) {
                        let _ = tx.send(ScanEvent::WifiDevice(wifi_device));
                    }
                }
                Err(pcap::Error::TimeoutExpired) => {
                    // Normal timeout, continue
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Err(e) => {
                    error!("Capture error: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}

/// Enable monitor mode on an interface (requires root)
pub fn enable_monitor_mode(interface: &str) -> Result<()> {
    use std::process::Command;

    // Bring interface down
    Command::new("ip")
        .args(["link", "set", interface, "down"])
        .status()
        .context("Failed to bring interface down")?;

    // Set monitor mode
    Command::new("iw")
        .args([interface, "set", "type", "monitor"])
        .status()
        .context("Failed to set monitor mode")?;

    // Bring interface up
    Command::new("ip")
        .args(["link", "set", interface, "up"])
        .status()
        .context("Failed to bring interface up")?;

    info!("Monitor mode enabled on {}", interface);
    Ok(())
}

/// Hop between WiFi channels for comprehensive scanning
pub async fn channel_hopper(interface: &str, channels: Vec<u8>) -> Result<()> {
    use std::process::Command;

    loop {
        for channel in &channels {
            Command::new("iw")
                .args([interface, "set", "channel", &channel.to_string()])
                .status()
                .ok();
            
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
}
