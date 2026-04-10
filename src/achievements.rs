use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use tracing::info;

// ============================================================================
// ACHIEVEMENT DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: AchievementCategory,
    pub icon: &'static str,
    pub rarity: Rarity,
    pub hidden: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AchievementCategory {
    Contacts,
    WiFi,
    Bluetooth,
    Drones,
    TSCM,
    Soundboard,
    SIEM,
    RayHunter,
    SDR,
    ML,
    System,
    Exploration,
    Milestones,
    Secret,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockedAchievement {
    pub id: String,
    pub unlocked_at: DateTime<Utc>,
    pub detail: Option<String>,
}

// All achievements
pub const ACHIEVEMENTS: &[Achievement] = &[
    // ========== CONTACTS (device counting) ==========
    Achievement { id: "first_contact", name: "First Contact", description: "Detect your first WiFi or BLE device", category: AchievementCategory::Contacts, icon: "📡", rarity: Rarity::Common, hidden: false },
    Achievement { id: "ten_contacts", name: "Getting Warmer", description: "Log 10 unique devices", category: AchievementCategory::Contacts, icon: "📊", rarity: Rarity::Common, hidden: false },
    Achievement { id: "fifty_contacts", name: "Neighborhood Watch", description: "Log 50 unique devices", category: AchievementCategory::Contacts, icon: "🏘️", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "hundred_contacts", name: "Census Taker", description: "Log 100 unique devices", category: AchievementCategory::Contacts, icon: "📋", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "fivehundred_contacts", name: "Signal Hound", description: "Log 500 unique devices", category: AchievementCategory::Contacts, icon: "🐕", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "thousand_contacts", name: "The Collector", description: "Log 1,000 unique devices", category: AchievementCategory::Contacts, icon: "🏆", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "five_thousand_contacts", name: "Panopticon", description: "Log 5,000 unique devices", category: AchievementCategory::Contacts, icon: "👁️", rarity: Rarity::Legendary, hidden: false },
    Achievement { id: "strong_signal", name: "Up Close and Personal", description: "Detect a device at -30 dBm or stronger", category: AchievementCategory::Contacts, icon: "💪", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "weak_signal", name: "Whisper in the Dark", description: "Detect a device at -90 dBm or weaker", category: AchievementCategory::Contacts, icon: "👻", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "baseline_set", name: "Know Thy Neighbor", description: "Complete baseline training for a location", category: AchievementCategory::Contacts, icon: "🎓", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "watched_device", name: "Person of Interest", description: "Add a device to your watched list", category: AchievementCategory::Contacts, icon: "🔍", rarity: Rarity::Common, hidden: false },
    Achievement { id: "silenced_ten", name: "Noise Cancelling", description: "Auto-silence 10 devices", category: AchievementCategory::Contacts, icon: "🔇", rarity: Rarity::Common, hidden: false },

    // ========== WiFi ==========
    Achievement { id: "first_wifi", name: "Hello WiFi", description: "Detect your first WiFi device", category: AchievementCategory::WiFi, icon: "📶", rarity: Rarity::Common, hidden: false },
    Achievement { id: "first_ap", name: "Access Granted", description: "Detect your first access point", category: AchievementCategory::WiFi, icon: "🔐", rarity: Rarity::Common, hidden: false },
    Achievement { id: "deauth_detected", name: "Under Fire", description: "Detect a deauthentication attack", category: AchievementCategory::WiFi, icon: "⚔️", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "evil_twin", name: "Seeing Double", description: "Detect an evil twin attack", category: AchievementCategory::WiFi, icon: "👯", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "karma_attack", name: "Bad Karma", description: "Detect a KARMA attack", category: AchievementCategory::WiFi, icon: "☠️", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "pcap_started", name: "Packet Rat", description: "Start a PCAP capture session", category: AchievementCategory::WiFi, icon: "📦", rarity: Rarity::Common, hidden: false },
    Achievement { id: "hidden_ssid", name: "Invisible Man", description: "Detect a hidden SSID network", category: AchievementCategory::WiFi, icon: "🫥", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "fifty_aps", name: "Wardriver", description: "Detect 50 unique access points", category: AchievementCategory::WiFi, icon: "🚗", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "probe_hunter", name: "Probe Hunter", description: "Capture 100 probe requests", category: AchievementCategory::WiFi, icon: "🎯", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "monitor_mode", name: "All Ears", description: "Successfully enter monitor mode", category: AchievementCategory::WiFi, icon: "👂", rarity: Rarity::Common, hidden: false },

    // ========== Bluetooth ==========
    Achievement { id: "first_ble", name: "Bluetooth Pioneer", description: "Detect your first BLE device", category: AchievementCategory::Bluetooth, icon: "🦷", rarity: Rarity::Common, hidden: false },
    Achievement { id: "tracker_found", name: "Stalker Alert", description: "Detect a tracking device (AirTag/Tile/SmartTag)", category: AchievementCategory::Bluetooth, icon: "🚨", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "airtag_found", name: "Apple Picking", description: "Detect an AirTag", category: AchievementCategory::Bluetooth, icon: "🍎", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "tile_found", name: "Tile Hunter", description: "Detect a Tile tracker", category: AchievementCategory::Bluetooth, icon: "🔲", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "smarttag_found", name: "Galaxy Brain", description: "Detect a Samsung SmartTag", category: AchievementCategory::Bluetooth, icon: "🌌", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "hundred_ble", name: "Bluetooth Bloodhound", description: "Detect 100 unique BLE devices", category: AchievementCategory::Bluetooth, icon: "🐕‍🦺", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "five_trackers", name: "Counter-Surveillance Pro", description: "Detect 5 different tracking devices", category: AchievementCategory::Bluetooth, icon: "🕵️", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "lost_mode_found", name: "Lost and Found", description: "Detect a tracker in Lost Mode", category: AchievementCategory::Bluetooth, icon: "📍", rarity: Rarity::Rare, hidden: false },

    // ========== Drones ==========
    Achievement { id: "first_drone", name: "Bogey Inbound", description: "Detect your first drone", category: AchievementCategory::Drones, icon: "🛸", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "dji_detected", name: "Made in China", description: "Detect a DJI drone", category: AchievementCategory::Drones, icon: "🇨🇳", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "fpv_detected", name: "Speed Demon", description: "Detect an FPV racing drone", category: AchievementCategory::Drones, icon: "🏎️", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "drone_emi", name: "Motor Mouth", description: "Detect a drone via EMI harmonic analysis", category: AchievementCategory::Drones, icon: "⚡", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "drone_rf_emi", name: "Double Confirmation", description: "Detect a drone via both RF and EMI simultaneously", category: AchievementCategory::Drones, icon: "✅", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "drone_remoteid", name: "ID Please", description: "Decode a WiFi RemoteID broadcast", category: AchievementCategory::Drones, icon: "🪪", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "five_drones", name: "Drone Swarm", description: "Detect 5 different drones", category: AchievementCategory::Drones, icon: "🐝", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "unknown_drone", name: "Unidentified Aerial", description: "Detect a drone with unknown type classification", category: AchievementCategory::Drones, icon: "❓", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "military_drone", name: "Eyes in the Sky", description: "Detect a military drone frequency", category: AchievementCategory::Drones, icon: "🎖️", rarity: Rarity::Legendary, hidden: true },

    // ========== TSCM ==========
    Achievement { id: "first_tscm", name: "Bug Sweep", description: "Run your first TSCM sweep", category: AchievementCategory::TSCM, icon: "🔦", rarity: Rarity::Common, hidden: false },
    Achievement { id: "threat_found", name: "We're Not Alone", description: "Detect a surveillance threat during TSCM sweep", category: AchievementCategory::TSCM, icon: "🐛", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "clean_sweep", name: "All Clear", description: "Complete a full TSCM sweep with no threats", category: AchievementCategory::TSCM, icon: "✨", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "ten_sweeps", name: "Professional Paranoid", description: "Complete 10 TSCM sweeps", category: AchievementCategory::TSCM, icon: "🧹", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "bumper_beeper", name: "Car Trouble", description: "Detect a bumper beeper/GPS tracker frequency", category: AchievementCategory::TSCM, icon: "🚗", rarity: Rarity::Epic, hidden: false },

    // ========== Soundboard ==========
    Achievement { id: "first_clip", name: "Sound Check", description: "Play your first soundboard clip", category: AchievementCategory::Soundboard, icon: "🔊", rarity: Rarity::Common, hidden: false },
    Achievement { id: "all_alerts_played", name: "Tone Deaf", description: "Play every alert tone at least once", category: AchievementCategory::Soundboard, icon: "🎵", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "ctcss_played", name: "Subaudible", description: "Play a CTCSS tone", category: AchievementCategory::Soundboard, icon: "🔈", rarity: Rarity::Common, hidden: false },
    Achievement { id: "clip_uploaded", name: "DJ Drop", description: "Upload a custom soundboard clip", category: AchievementCategory::Soundboard, icon: "🎤", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "roger_beep", name: "Roger Roger", description: "Play the roger beep", category: AchievementCategory::Soundboard, icon: "📻", rarity: Rarity::Common, hidden: false },

    // ========== SIEM ==========
    Achievement { id: "first_siem", name: "Analyst on Duty", description: "View the SIEM event log", category: AchievementCategory::SIEM, icon: "📊", rarity: Rarity::Common, hidden: false },
    Achievement { id: "siem_search", name: "Digital Forensics", description: "Perform a SIEM search", category: AchievementCategory::SIEM, icon: "🔎", rarity: Rarity::Common, hidden: false },
    Achievement { id: "siem_export", name: "Evidence Locker", description: "Export SIEM events", category: AchievementCategory::SIEM, icon: "💾", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "hundred_events", name: "Paper Trail", description: "Log 100 SIEM events", category: AchievementCategory::SIEM, icon: "📃", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "thousand_events", name: "Big Data", description: "Log 1,000 SIEM events", category: AchievementCategory::SIEM, icon: "🗄️", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "siem_watch", name: "Night Shift", description: "Use SIEM live watch mode for 5 minutes", category: AchievementCategory::SIEM, icon: "🌙", rarity: Rarity::Uncommon, hidden: false },

    // ========== RayHunter ==========
    Achievement { id: "first_rayhunter", name: "Bad Cop No Donut", description: "Get your first RayHunter IMSI catcher detection", category: AchievementCategory::RayHunter, icon: "🍩", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "rayhunter_connected", name: "Phone Home", description: "Connect RayHunter successfully", category: AchievementCategory::RayHunter, icon: "📱", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "rayhunter_recording", name: "On the Record", description: "Start a QMDL recording session", category: AchievementCategory::RayHunter, icon: "⏺️", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "three_imsi", name: "Stingray Spotter", description: "Detect 3 separate IMSI catcher events", category: AchievementCategory::RayHunter, icon: "🦈", rarity: Rarity::Legendary, hidden: false },
    Achievement { id: "five_imsi", name: "Surveillance State", description: "Detect 5 IMSI catcher events", category: AchievementCategory::RayHunter, icon: "🏛️", rarity: Rarity::Legendary, hidden: true },

    // ========== SDR ==========
    Achievement { id: "first_sdr", name: "Tuned In", description: "Use an SDR device for the first time", category: AchievementCategory::SDR, icon: "📻", rarity: Rarity::Common, hidden: false },
    Achievement { id: "spectrum_viewed", name: "Spectrum Painter", description: "View the spectrum analyzer", category: AchievementCategory::SDR, icon: "🌈", rarity: Rarity::Common, hidden: false },
    Achievement { id: "fm_listened", name: "Radio Star", description: "Listen to an FM broadcast", category: AchievementCategory::SDR, icon: "🎶", rarity: Rarity::Common, hidden: false },
    Achievement { id: "fastfood_detected", name: "Drive-Thru Interceptor", description: "Detect a fast food drive-thru frequency", category: AchievementCategory::SDR, icon: "🍔", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "area51_tuned", name: "Dreamland", description: "Tune to an Area 51/NTTR frequency", category: AchievementCategory::SDR, icon: "👽", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "security_freq", name: "Rent-a-Cop", description: "Detect a security company frequency", category: AchievementCategory::SDR, icon: "🛡️", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "morse_decoded", name: "Dit Dah", description: "Decode a Morse code message", category: AchievementCategory::SDR, icon: "📟", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "hackrf_used", name: "Full Duplex", description: "Use a HackRF device", category: AchievementCategory::SDR, icon: "🔧", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "cell_tower", name: "Tower Mapper", description: "Scan for cell towers", category: AchievementCategory::SDR, icon: "📡", rarity: Rarity::Uncommon, hidden: false },

    // ========== ML ==========
    Achievement { id: "ml_anomaly", name: "Something's Off", description: "ML anomaly detector flags an unknown spectrum signature", category: AchievementCategory::ML, icon: "🤖", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "ml_baseline_20", name: "Machine Learner", description: "ML baseline reaches 20 samples", category: AchievementCategory::ML, icon: "🧠", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "ml_classify", name: "Signal Classifier", description: "ML classifies a signal modulation type", category: AchievementCategory::ML, icon: "🏷️", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "fft_analyzed", name: "Fourier Transform", description: "Run FFT spectral analysis on a signal", category: AchievementCategory::ML, icon: "📈", rarity: Rarity::Common, hidden: false },

    // ========== System / Exploration ==========
    Achievement { id: "first_boot", name: "Power On", description: "Start SIGINT-Deck for the first time", category: AchievementCategory::System, icon: "⚡", rarity: Rarity::Common, hidden: false },
    Achievement { id: "steam_mode", name: "Couch Operator", description: "Run SIGINT-Deck in Steam Deck Game Mode", category: AchievementCategory::System, icon: "🎮", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "desktop_mode", name: "Desktop Warrior", description: "Run SIGINT-Deck in Desktop Mode", category: AchievementCategory::System, icon: "🖥️", rarity: Rarity::Common, hidden: false },
    Achievement { id: "rtfm", name: "RTFM", description: "Open the documentation/about page", category: AchievementCategory::Exploration, icon: "📖", rarity: Rarity::Common, hidden: false },
    Achievement { id: "settings_opened", name: "Tinkerer", description: "Open the Settings page", category: AchievementCategory::Exploration, icon: "⚙️", rarity: Rarity::Common, hidden: false },
    Achievement { id: "ninja_mode", name: "Silent Assassin", description: "Enable Ninja Mode", category: AchievementCategory::System, icon: "🥷", rarity: Rarity::Common, hidden: false },
    Achievement { id: "geofence_set", name: "Perimeter Secured", description: "Set up a geofence", category: AchievementCategory::System, icon: "🚧", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "note_added", name: "Field Notes", description: "Add a note to a device", category: AchievementCategory::Exploration, icon: "📝", rarity: Rarity::Common, hidden: false },
    Achievement { id: "browser_sound", name: "Can You Hear Me Now", description: "Enable browser sound alerts", category: AchievementCategory::System, icon: "🔔", rarity: Rarity::Common, hidden: false },
    Achievement { id: "browser_tts", name: "Voice of God", description: "Enable browser TTS alerts", category: AchievementCategory::System, icon: "🗣️", rarity: Rarity::Common, hidden: false },
    Achievement { id: "twelve_hour_run", name: "Marathon Runner", description: "Run SIGINT-Deck continuously for 12 hours", category: AchievementCategory::Milestones, icon: "🏃", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "twentyfour_hour_run", name: "Night Owl", description: "Run continuously for 24 hours", category: AchievementCategory::Milestones, icon: "🦉", rarity: Rarity::Epic, hidden: false },
    Achievement { id: "seven_day_run", name: "Always Watching", description: "Run for 7 cumulative days", category: AchievementCategory::Milestones, icon: "👁️", rarity: Rarity::Legendary, hidden: false },
    Achievement { id: "legal_read", name: "I Read the EULA", description: "View the legal disclaimer", category: AchievementCategory::Exploration, icon: "⚖️", rarity: Rarity::Common, hidden: false },
    Achievement { id: "llm_query", name: "Ask the Oracle", description: "Query the LLM about a device", category: AchievementCategory::Exploration, icon: "🔮", rarity: Rarity::Uncommon, hidden: false },
    Achievement { id: "flipper_connected", name: "Dolphin Trainer", description: "Connect a Flipper Zero", category: AchievementCategory::System, icon: "🐬", rarity: Rarity::Rare, hidden: false },
    Achievement { id: "preset_saved", name: "Bookmarked", description: "Save a commercial RF monitor preset", category: AchievementCategory::SDR, icon: "🔖", rarity: Rarity::Common, hidden: false },

    // ========== Secret / Hidden ==========
    Achievement { id: "fbi_van", name: "FBI Surveillance Van", description: "Detect a WiFi network named 'FBI Surveillance Van'", category: AchievementCategory::Secret, icon: "🚐", rarity: Rarity::Epic, hidden: true },
    Achievement { id: "rickroll", name: "Never Gonna Give You Up", description: "Play the air horn soundboard clip 10 times", category: AchievementCategory::Secret, icon: "🎺", rarity: Rarity::Rare, hidden: true },
    Achievement { id: "midnight_scan", name: "Midnight Operator", description: "Run a scan between midnight and 4 AM", category: AchievementCategory::Secret, icon: "🌑", rarity: Rarity::Uncommon, hidden: true },
    Achievement { id: "all_tabs", name: "Explorer", description: "Visit every tab in the dashboard", category: AchievementCategory::Secret, icon: "🗺️", rarity: Rarity::Uncommon, hidden: true },
    Achievement { id: "easter_egg", name: "SIGINT Training Ops", description: "Find the game easter egg in Settings", category: AchievementCategory::Secret, icon: "🎮", rarity: Rarity::Rare, hidden: true },
];

// ============================================================================
// ACHIEVEMENT STATE (runtime)
// ============================================================================

pub static UNLOCKED: Lazy<Mutex<Vec<UnlockedAchievement>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static COUNTERS: Lazy<Mutex<HashMap<String, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static START_TIME: Lazy<Mutex<DateTime<Utc>>> = Lazy::new(|| Mutex::new(Utc::now()));
static PLAYED_CLIPS: Lazy<Mutex<std::collections::HashSet<String>>> = Lazy::new(|| Mutex::new(std::collections::HashSet::new()));
static VISITED_TABS: Lazy<Mutex<std::collections::HashSet<String>>> = Lazy::new(|| Mutex::new(std::collections::HashSet::new()));

pub fn init() {
    let mut start = START_TIME.lock().unwrap();
    *start = Utc::now();
    // Try to load from disk
    load_from_disk();
    // First boot achievement
    try_unlock("first_boot", None);
}

fn is_unlocked(id: &str) -> bool {
    UNLOCKED.lock().unwrap().iter().any(|a| a.id == id)
}

pub fn try_unlock(id: &str, detail: Option<String>) -> bool {
    if is_unlocked(id) { return false; }
    // Verify achievement exists
    if !ACHIEVEMENTS.iter().any(|a| a.id == id) { return false; }

    let ua = UnlockedAchievement {
        id: id.to_string(),
        unlocked_at: Utc::now(),
        detail,
    };
    UNLOCKED.lock().unwrap().push(ua);
    info!("Achievement unlocked: {}", id);
    save_to_disk();
    true
}

pub fn increment(counter: &str) -> u64 {
    let mut counters = COUNTERS.lock().unwrap();
    let val = counters.entry(counter.to_string()).or_insert(0);
    *val += 1;
    let v = *val;
    drop(counters);
    // Check counter-based achievements
    check_counter_achievements(counter, v);
    save_to_disk();
    v
}

pub fn get_counter(counter: &str) -> u64 {
    COUNTERS.lock().unwrap().get(counter).copied().unwrap_or(0)
}

pub fn record_clip_played(clip_name: &str) {
    PLAYED_CLIPS.lock().unwrap().insert(clip_name.to_string());
    try_unlock("first_clip", Some(clip_name.to_string()));
    if clip_name.contains("roger") { try_unlock("roger_beep", None); }
    if clip_name.contains("ctcss") { try_unlock("ctcss_played", None); }

    let count = increment("clips_played");
    if clip_name.contains("air_horn") && count >= 10 {
        // Actually track air horn specifically
        let horn_count = increment("air_horn_plays");
        if horn_count >= 10 { try_unlock("rickroll", None); }
    }

    // Check if all alert tones played
    let clips = PLAYED_CLIPS.lock().unwrap();
    let alert_tones = ["alert_drone", "alert_tracker", "alert_imsi", "alert_tscm",
                       "alert_critical", "alert_high", "alert_new_device", "alert_geofence"];
    if alert_tones.iter().all(|t| clips.iter().any(|c| c.contains(t))) {
        drop(clips);
        try_unlock("all_alerts_played", None);
    }
}

pub fn record_tab_visited(tab: &str) {
    VISITED_TABS.lock().unwrap().insert(tab.to_string());
    let known_tabs = ["wifi", "ble", "sdr", "tscm", "drones", "siem",
                      "soundboard", "fastfood", "rayhunter", "settings", "about"];
    let visited = VISITED_TABS.lock().unwrap();
    if known_tabs.iter().all(|t| visited.contains(*t)) {
        drop(visited);
        try_unlock("all_tabs", None);
    }
}

fn check_counter_achievements(counter: &str, value: u64) {
    match counter {
        "unique_devices" => {
            if value >= 1 { try_unlock("first_contact", None); }
            if value >= 10 { try_unlock("ten_contacts", None); }
            if value >= 50 { try_unlock("fifty_contacts", None); }
            if value >= 100 { try_unlock("hundred_contacts", None); }
            if value >= 500 { try_unlock("fivehundred_contacts", None); }
            if value >= 1000 { try_unlock("thousand_contacts", None); }
            if value >= 5000 { try_unlock("five_thousand_contacts", None); }
        }
        "wifi_devices" => {
            if value >= 1 { try_unlock("first_wifi", None); }
        }
        "ble_devices" => {
            if value >= 1 { try_unlock("first_ble", None); }
            if value >= 100 { try_unlock("hundred_ble", None); }
        }
        "access_points" => {
            if value >= 1 { try_unlock("first_ap", None); }
            if value >= 50 { try_unlock("fifty_aps", None); }
        }
        "trackers_found" => {
            if value >= 1 { try_unlock("tracker_found", None); }
            if value >= 5 { try_unlock("five_trackers", None); }
        }
        "drones_detected" => {
            if value >= 1 { try_unlock("first_drone", None); }
            if value >= 5 { try_unlock("five_drones", None); }
        }
        "tscm_sweeps" => {
            if value >= 1 { try_unlock("first_tscm", None); }
            if value >= 10 { try_unlock("ten_sweeps", None); }
        }
        "siem_events" => {
            if value >= 100 { try_unlock("hundred_events", None); }
            if value >= 1000 { try_unlock("thousand_events", None); }
        }
        "imsi_detections" => {
            if value >= 1 { try_unlock("first_rayhunter", None); }
            if value >= 3 { try_unlock("three_imsi", None); }
            if value >= 5 { try_unlock("five_imsi", None); }
        }
        "probe_requests" => {
            if value >= 100 { try_unlock("probe_hunter", None); }
        }
        "silenced_devices" => {
            if value >= 10 { try_unlock("silenced_ten", None); }
        }
        _ => {}
    }
}

pub fn check_uptime_achievements() {
    let start = *START_TIME.lock().unwrap();
    let hours = Utc::now().signed_duration_since(start).num_hours();
    if hours >= 12 { try_unlock("twelve_hour_run", None); }
    if hours >= 24 { try_unlock("twentyfour_hour_run", None); }

    // Cumulative days tracked by counter
    let total_hours = get_counter("total_uptime_hours");
    if total_hours >= 168 { try_unlock("seven_day_run", None); }
}

pub fn check_midnight() {
    let hour = Utc::now().hour();
    if hour < 4 { try_unlock("midnight_scan", None); }
}

use chrono::Timelike;

// ============================================================================
// PERSISTENCE (JSON file)
// ============================================================================

#[derive(Serialize, Deserialize)]
struct AchievementState {
    unlocked: Vec<UnlockedAchievement>,
    counters: HashMap<String, u64>,
}

fn state_path() -> std::path::PathBuf {
    let dirs = ["/home/deck/sigint-deck/data", "/home/pi/sigint-pi/data", "./data"];
    for d in dirs {
        let p = std::path::PathBuf::from(d);
        if p.exists() || std::fs::create_dir_all(&p).is_ok() {
            return p.join("achievements.json");
        }
    }
    std::path::PathBuf::from("./data/achievements.json")
}

fn save_to_disk() {
    let state = AchievementState {
        unlocked: UNLOCKED.lock().unwrap().clone(),
        counters: COUNTERS.lock().unwrap().clone(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = std::fs::write(state_path(), json);
    }
}

fn load_from_disk() {
    let path = state_path();
    if let Ok(json) = std::fs::read_to_string(&path) {
        if let Ok(state) = serde_json::from_str::<AchievementState>(&json) {
            *UNLOCKED.lock().unwrap() = state.unlocked;
            *COUNTERS.lock().unwrap() = state.counters;
            info!("Loaded {} achievements from disk", UNLOCKED.lock().unwrap().len());
        }
    }
}

// ============================================================================
// QUERY
// ============================================================================

pub fn get_all() -> Vec<serde_json::Value> {
    let unlocked = UNLOCKED.lock().unwrap();
    ACHIEVEMENTS.iter().map(|a| {
        let ua = unlocked.iter().find(|u| u.id == a.id);
        let show = !a.hidden || ua.is_some();
        serde_json::json!({
            "id": a.id,
            "name": if show { a.name } else { "???" },
            "description": if show { a.description } else { "Hidden achievement" },
            "category": format!("{:?}", a.category),
            "icon": if show { a.icon } else { "🔒" },
            "rarity": format!("{:?}", a.rarity),
            "hidden": a.hidden,
            "unlocked": ua.is_some(),
            "unlocked_at": ua.map(|u| u.unlocked_at.to_rfc3339()),
            "detail": ua.and_then(|u| u.detail.clone()),
        })
    }).collect()
}

pub fn get_summary() -> serde_json::Value {
    let unlocked = UNLOCKED.lock().unwrap();
    let total = ACHIEVEMENTS.len();
    let unlocked_count = unlocked.len();
    let visible = ACHIEVEMENTS.iter().filter(|a| !a.hidden || unlocked.iter().any(|u| u.id == a.id)).count();
    let latest = unlocked.last().cloned();

    serde_json::json!({
        "total_achievements": total,
        "unlocked": unlocked_count,
        "visible": visible,
        "completion_pct": (unlocked_count as f64 / total as f64 * 100.0).round(),
        "latest": latest.map(|l| serde_json::json!({
            "id": l.id,
            "unlocked_at": l.unlocked_at.to_rfc3339(),
        })),
        "counters": *COUNTERS.lock().unwrap(),
    })
}
