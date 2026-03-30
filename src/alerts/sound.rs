//! Sound alert module
//!
//! Provides audio alerts for various events with configurable sounds.
//! Supports "Ninja Mode" for silent operation.

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use tracing::{debug, warn, error};

/// Sound effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoundEffect {
    NewDevice,
    TrackerDetected,
    AttackDetected,
    CriticalAlert,
    HighAlert,
    MediumAlert,
    LowAlert,
    GeofenceEnter,
    GeofenceExit,
    SystemReady,
    SystemError,
}

impl SoundEffect {
    /// Get the default sound file name for this effect
    pub fn default_filename(&self) -> &'static str {
        match self {
            SoundEffect::NewDevice => "new_device.wav",
            SoundEffect::TrackerDetected => "tracker_alert.wav",
            SoundEffect::AttackDetected => "attack_alert.wav",
            SoundEffect::CriticalAlert => "critical.wav",
            SoundEffect::HighAlert => "high_alert.wav",
            SoundEffect::MediumAlert => "medium_alert.wav",
            SoundEffect::LowAlert => "low_alert.wav",
            SoundEffect::GeofenceEnter => "geofence_enter.wav",
            SoundEffect::GeofenceExit => "geofence_exit.wav",
            SoundEffect::SystemReady => "ready.wav",
            SoundEffect::SystemError => "error.wav",
        }
    }

    /// Get priority (higher = more important)
    pub fn priority(&self) -> u8 {
        match self {
            SoundEffect::CriticalAlert | SoundEffect::AttackDetected => 100,
            SoundEffect::TrackerDetected => 90,
            SoundEffect::HighAlert => 80,
            SoundEffect::GeofenceEnter | SoundEffect::GeofenceExit => 70,
            SoundEffect::MediumAlert => 60,
            SoundEffect::NewDevice => 50,
            SoundEffect::LowAlert => 40,
            SoundEffect::SystemReady | SoundEffect::SystemError => 30,
        }
    }
}

/// Sound configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    pub enabled: bool,
    pub ninja_mode: bool,
    pub volume: u8,
    pub sounds_dir: PathBuf,
    pub new_device_sound: bool,
    pub tracker_sound: bool,
    pub attack_sound: bool,
    pub alert_sounds: bool,
    pub geofence_sounds: bool,
    pub system_sounds: bool,
    pub cooldown_ms: u64,
}

impl Default for SoundConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            ninja_mode: false,
            volume: 70,
            sounds_dir: PathBuf::from("/app/sounds"),
            new_device_sound: true,
            tracker_sound: true,
            attack_sound: true,
            alert_sounds: true,
            geofence_sounds: true,
            system_sounds: true,
            cooldown_ms: 1000,
        }
    }
}

/// Sound player for audio alerts
pub struct SoundPlayer {
    config: SoundConfig,
    last_play_time: std::sync::Mutex<std::time::Instant>,
}

impl SoundPlayer {
    pub fn new(config: SoundConfig) -> Self {
        Self {
            config,
            last_play_time: std::sync::Mutex::new(std::time::Instant::now()),
        }
    }

    /// Check if ninja mode is active (all sounds silenced)
    pub fn is_ninja_mode(&self) -> bool {
        self.config.ninja_mode
    }

    /// Enable or disable ninja mode
    pub fn set_ninja_mode(&mut self, enabled: bool) {
        self.config.ninja_mode = enabled;
        if enabled {
            debug!("Ninja mode enabled - all sounds silenced");
        } else {
            debug!("Ninja mode disabled");
        }
    }

    /// Check if a sound effect is enabled
    fn is_effect_enabled(&self, effect: SoundEffect) -> bool {
        if !self.config.enabled || self.config.ninja_mode {
            return false;
        }

        match effect {
            SoundEffect::NewDevice => self.config.new_device_sound,
            SoundEffect::TrackerDetected => self.config.tracker_sound,
            SoundEffect::AttackDetected => self.config.attack_sound,
            SoundEffect::CriticalAlert | SoundEffect::HighAlert |
            SoundEffect::MediumAlert | SoundEffect::LowAlert => self.config.alert_sounds,
            SoundEffect::GeofenceEnter | SoundEffect::GeofenceExit => self.config.geofence_sounds,
            SoundEffect::SystemReady | SoundEffect::SystemError => self.config.system_sounds,
        }
    }

    /// Play a sound effect
    pub fn play(&self, effect: SoundEffect) {
        if !self.is_effect_enabled(effect) {
            return;
        }

        // Check cooldown
        {
            let mut last_time = self.last_play_time.lock().unwrap();
            let elapsed = last_time.elapsed().as_millis() as u64;
            if elapsed < self.config.cooldown_ms {
                debug!("Sound cooldown active, skipping {:?}", effect);
                return;
            }
            *last_time = std::time::Instant::now();
        }

        let sound_file = self.config.sounds_dir.join(effect.default_filename());
        
        if !sound_file.exists() {
            // Try to use system beep as fallback
            self.play_system_beep(effect);
            return;
        }

        self.play_file(&sound_file);
    }

    /// Play a sound file
    fn play_file(&self, path: &PathBuf) {
        let path_str = path.to_string_lossy().to_string();
        let volume = self.config.volume;

        std::thread::spawn(move || {
            // Pre-format volume strings to avoid temporary value issues
            let paplay_volume = format!("{}", volume as u32 * 655);
            let afplay_volume = format!("{}", volume as f32 / 100.0);
            
            // Try various audio players available on different platforms
            let players: [(&str, Vec<&str>); 4] = [
                ("paplay", vec!["--volume", &paplay_volume, &path_str]),
                ("aplay", vec![&path_str]),
                ("pw-play", vec![&path_str]),
                ("afplay", vec!["-v", &afplay_volume, &path_str]),
            ];

            for (player, args) in players {
                if let Ok(status) = Command::new(player).args(&args).status() {
                    if status.success() {
                        debug!("Played sound with {}", player);
                        return;
                    }
                }
            }

            warn!("No audio player available for sound playback");
        });
    }

    /// Play a system beep (fallback when sound files unavailable)
    fn play_system_beep(&self, effect: SoundEffect) {
        let frequency = match effect.priority() {
            90..=100 => 880,  // High pitch for critical
            70..=89 => 660,   // Medium-high
            50..=69 => 440,   // Medium
            _ => 330,         // Low
        };

        let duration = match effect.priority() {
            90..=100 => 300,
            70..=89 => 200,
            _ => 100,
        };

        std::thread::spawn(move || {
            // Try to use system beep
            #[cfg(target_os = "linux")]
            {
                let _ = Command::new("beep")
                    .args(["-f", &frequency.to_string(), "-l", &duration.to_string()])
                    .status();
            }
        });
    }
}

/// Generate default sound files (simple beeps)
pub fn generate_default_sounds(dir: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    
    // Note: In a real implementation, we'd embed WAV files or generate them
    // For now, we'll rely on system beep fallback
    debug!("Sound directory created at {:?}", dir);
    
    Ok(())
}
