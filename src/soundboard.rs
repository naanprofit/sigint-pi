use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{info, warn, error};

const CLIPS_DIR: &str = "/home/sigint/soundboard/clips";
const CLIPS_DIR_PI: &str = "/home/pi/sigint-pi/soundboard/clips";
const CLIPS_DIR_DECK: &str = "/home/deck/sigint-deck/soundboard/clips";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundClip {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub duration_ms: Option<u64>,
    pub tags: Vec<String>,
    pub alert_event: Option<String>,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}

// Frequencies where TX is absolutely forbidden (Hz)
const BLOCKED_TX_RANGES: &[(u64, u64, &str)] = &[
    // Cellular bands
    (698_000_000, 756_000_000, "Cellular 700 MHz (Band 12/13/17)"),
    (776_000_000, 787_000_000, "Cellular 700 MHz (Band 5)"),
    (824_000_000, 894_000_000, "Cellular 850 MHz"),
    (1710_000_000, 1755_000_000, "AWS-1 uplink"),
    (1850_000_000, 1995_000_000, "PCS 1900 MHz"),
    (2110_000_000, 2155_000_000, "AWS-1 downlink"),
    (2496_000_000, 2690_000_000, "BRS/EBS 2.5 GHz"),
    // Aviation
    (108_000_000, 137_000_000, "VHF Aviation"),
    (225_000_000, 400_000_000, "UHF Military Aviation"),
    (960_000_000, 1215_000_000, "DME/TACAN/ADS-B"),
    (1030_000_000, 1090_000_000, "ATC Radar/Transponder"),
    // Emergency
    (148_000_000, 150_800_000, "NOAA Weather / Emergency"),
    (406_000_000, 406_100_000, "EPIRB/ELT Emergency"),
];

pub fn get_clips_dir() -> PathBuf {
    for dir in [CLIPS_DIR, CLIPS_DIR_PI, CLIPS_DIR_DECK] {
        let p = PathBuf::from(dir);
        if p.parent().map(|p| p.exists()).unwrap_or(false) {
            let _ = std::fs::create_dir_all(&p);
            return p;
        }
    }
    let fallback = PathBuf::from("/tmp/sigint-soundboard/clips");
    let _ = std::fs::create_dir_all(&fallback);
    fallback
}

pub fn list_clips() -> Vec<SoundClip> {
    let dir = get_clips_dir();
    let mut clips = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "wav" || ext == "mp3" || ext == "ogg" || ext == "flac" {
                    let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    let name = path.file_stem().unwrap_or_default().to_string_lossy()
                        .replace('_', " ").replace('-', " ");
                    let meta = std::fs::metadata(&path).ok();
                    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    let created = meta.and_then(|m| m.created().ok())
                        .map(|t| DateTime::<Utc>::from(t))
                        .unwrap_or_else(Utc::now);
                    clips.push(SoundClip {
                        id: filename.clone(),
                        name,
                        filename,
                        duration_ms: None,
                        tags: vec![],
                        alert_event: None,
                        size_bytes: size,
                        created_at: created,
                    });
                }
            }
        }
    }
    clips.sort_by(|a, b| a.name.cmp(&b.name));
    clips
}

pub fn delete_clip(id: &str) -> Result<()> {
    let path = get_clips_dir().join(id);
    if path.exists() {
        std::fs::remove_file(&path).context("Failed to delete clip")?;
        Ok(())
    } else {
        anyhow::bail!("Clip not found: {}", id)
    }
}

pub fn play_clip_local(id: &str) -> Result<String> {
    let path = get_clips_dir().join(id);
    if !path.exists() {
        anyhow::bail!("Clip not found: {}", id);
    }
    let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
    let cmd = if ext == "wav" { "aplay" } else { "paplay" };
    let output = std::process::Command::new(cmd)
        .arg(path.to_string_lossy().as_ref())
        .output()
        .context(format!("Failed to run {}", cmd))?;
    if output.status.success() {
        Ok(format!("Playing {} via {}", id, cmd))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Playback failed: {}", stderr)
    }
}

pub fn check_tx_frequency(freq_hz: u64) -> Result<()> {
    for (start, end, desc) in BLOCKED_TX_RANGES {
        if freq_hz >= *start && freq_hz <= *end {
            anyhow::bail!("TX BLOCKED on {} Hz — protected band: {}", freq_hz, desc);
        }
    }
    Ok(())
}

pub fn transmit_clip(id: &str, freq_hz: u64, modulation: &str, power_dbm: u8) -> Result<String> {
    check_tx_frequency(freq_hz)?;

    let path = get_clips_dir().join(id);
    if !path.exists() {
        anyhow::bail!("Clip not found: {}", id);
    }

    if power_dbm > 47 {
        anyhow::bail!("Power {} dBm exceeds HackRF maximum (47 dBm)", power_dbm);
    }

    // Check if hackrf_transfer is available
    let has_hackrf = std::process::Command::new("which").arg("hackrf_transfer")
        .output().map(|o| o.status.success()).unwrap_or(false);
    if !has_hackrf {
        anyhow::bail!("hackrf_transfer not found. HackRF required for RF transmit.");
    }

    let has_sox = std::process::Command::new("which").arg("sox")
        .output().map(|o| o.status.success()).unwrap_or(false);

    // Build the transmit pipeline
    // sox -> resample to 48k mono -> csdr FM modulate -> hackrf_transfer
    let freq_str = freq_hz.to_string();
    let gain_str = power_dbm.to_string();
    let path_str = path.to_string_lossy().to_string();

    info!("TX: {} at {} Hz, mod={}, power={} dBm", id, freq_hz, modulation, power_dbm);

    // Check for csdr
    let has_csdr = std::process::Command::new("which").arg("csdr")
        .output().map(|o| o.status.success()).unwrap_or(false);

    if has_csdr && has_sox {
        // Full pipeline: sox | csdr FM | hackrf_transfer
        let shell_cmd = format!(
            "sox '{}' -r 48000 -c 1 -t raw -e signed -b 16 - 2>/dev/null | \
             csdr convert_i16_f | csdr gain_ff 0.9 | csdr dsb_fc | \
             csdr bandpass_fir_fft_cc 0 0.1 0.01 | csdr gain_ff 2000 | \
             csdr convert_f_samplerf 4800 | \
             hackrf_transfer -t /dev/stdin -f {} -s 4800000 -x {}",
            path_str, freq_str, gain_str
        );
        let output = std::process::Command::new("sh")
            .args(["-c", &shell_cmd])
            .output()
            .context("TX pipeline failed")?;
        if output.status.success() {
            Ok(format!("Transmitted {} at {} Hz ({}) {}dBm", id, freq_hz, modulation, power_dbm))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("TX failed: {}", stderr)
        }
    } else {
        // Fallback: direct hackrf_transfer with raw IQ (no modulation)
        anyhow::bail!("csdr and/or sox not installed. Run: sudo apt install sox csdr")
    }
}
