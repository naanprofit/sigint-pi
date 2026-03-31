use anyhow::{Context, Result};
use pcap::{Capture, Device, Savefile};
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};
use std::time::Duration;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::config::WifiConfig;
use crate::ScanEvent;
use super::parser::{parse_wifi_frame, WifiDevice};
use super::attacks::AttackDetector;

/// Global PCAP capture state
pub static PCAP_ENABLED: AtomicBool = AtomicBool::new(false);
pub static PCAP_PACKETS: AtomicU64 = AtomicU64::new(0);
pub static PCAP_BYTES: AtomicU64 = AtomicU64::new(0);

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

        // Setup PCAP file saving if enabled
        let mut savefile: Option<Savefile> = None;
        if self.config.pcap_enabled {
            savefile = self.setup_pcap_savefile(&cap)?;
        }

        loop {
            match cap.next_packet() {
                Ok(packet) => {
                    // Save to PCAP file if enabled
                    if let Some(ref mut sf) = savefile {
                        if PCAP_ENABLED.load(Ordering::Relaxed) {
                            sf.write(&packet);
                            PCAP_PACKETS.fetch_add(1, Ordering::Relaxed);
                            PCAP_BYTES.fetch_add(packet.data.len() as u64, Ordering::Relaxed);
                        }
                    }
                    
                    // Parse and send device info
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
            
            // Check if we need to rotate the pcap file
            if self.config.pcap_enabled && PCAP_ENABLED.load(Ordering::Relaxed) {
                let bytes = PCAP_BYTES.load(Ordering::Relaxed);
                let rotate_bytes = (self.config.pcap_rotate_mb as u64) * 1024 * 1024;
                if bytes >= rotate_bytes {
                    info!("Rotating PCAP file at {} bytes", bytes);
                    savefile = self.setup_pcap_savefile(&cap)?;
                    PCAP_BYTES.store(0, Ordering::Relaxed);
                }
            }
        }
    }
    
    fn setup_pcap_savefile(&self, cap: &Capture<pcap::Active>) -> Result<Option<Savefile>> {
        let pcap_dir = PathBuf::from(&self.config.pcap_path);
        
        // Create directory if it doesn't exist
        if !pcap_dir.exists() {
            std::fs::create_dir_all(&pcap_dir)
                .context("Failed to create PCAP directory")?;
        }
        
        // Generate filename with timestamp
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = pcap_dir.join(format!("capture_{}.pcap", timestamp));
        
        info!("Starting PCAP capture to {:?}", filename);
        
        let savefile = cap.savefile(&filename)
            .context("Failed to create PCAP savefile")?;
        
        PCAP_ENABLED.store(true, Ordering::Relaxed);
        PCAP_PACKETS.store(0, Ordering::Relaxed);
        PCAP_BYTES.store(0, Ordering::Relaxed);
        
        Ok(Some(savefile))
    }
}

/// Start PCAP capture (called from API)
pub fn start_pcap_capture() {
    PCAP_ENABLED.store(true, Ordering::Relaxed);
    info!("PCAP capture enabled via API");
}

/// Stop PCAP capture (called from API)
pub fn stop_pcap_capture() {
    PCAP_ENABLED.store(false, Ordering::Relaxed);
    info!("PCAP capture disabled via API");
}

/// Get PCAP capture stats
pub fn get_pcap_stats() -> (bool, u64, u64) {
    (
        PCAP_ENABLED.load(Ordering::Relaxed),
        PCAP_PACKETS.load(Ordering::Relaxed),
        PCAP_BYTES.load(Ordering::Relaxed),
    )
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
    use tracing::{info, warn};
    
    info!("Channel hopper starting on {} with {} channels", interface, channels.len());
    
    // Small delay to let scanner initialize
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Always use sudo -n (non-interactive) for iw commands
    let mut hop_count = 0u64;
    
    loop {
        for channel in &channels {
            let result = Command::new("sudo")
                .args(["-n", "iw", "dev", interface, "set", "channel", &channel.to_string()])
                .output();
            
            match result {
                Ok(output) => {
                    if output.status.success() {
                        hop_count += 1;
                        if hop_count % 20 == 1 {
                            info!("Channel hop #{}: now on ch{}", hop_count, channel);
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if hop_count == 0 {
                            warn!("Channel hop failed: {}", stderr.trim());
                        }
                    }
                }
                Err(e) => {
                    if hop_count == 0 {
                        warn!("Channel hop command error: {}", e);
                    }
                }
            }
            
            // 500ms per channel
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}
