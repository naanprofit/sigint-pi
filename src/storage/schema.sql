-- Locations table for geofencing
CREATE TABLE IF NOT EXISTS locations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    latitude REAL,
    longitude REAL,
    radius_meters REAL DEFAULT 100,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- WiFi devices
CREATE TABLE IF NOT EXISTS wifi_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT UNIQUE NOT NULL,
    vendor TEXT,
    is_ap BOOLEAN DEFAULT FALSE,
    is_baseline BOOLEAN DEFAULT FALSE,
    is_allowlisted BOOLEAN DEFAULT FALSE,
    is_blocklisted BOOLEAN DEFAULT FALSE,
    notes TEXT,
    location_id INTEGER REFERENCES locations(id),
    first_seen DATETIME NOT NULL,
    last_seen DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_wifi_mac ON wifi_devices(mac_address);
CREATE INDEX IF NOT EXISTS idx_wifi_location ON wifi_devices(location_id);
CREATE INDEX IF NOT EXISTS idx_wifi_baseline ON wifi_devices(is_baseline);

-- BLE devices
CREATE TABLE IF NOT EXISTS ble_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT UNIQUE NOT NULL,
    name TEXT,
    device_type TEXT,
    vendor TEXT,
    is_baseline BOOLEAN DEFAULT FALSE,
    is_allowlisted BOOLEAN DEFAULT FALSE,
    is_blocklisted BOOLEAN DEFAULT FALSE,
    is_tracker BOOLEAN DEFAULT FALSE,
    notes TEXT,
    location_id INTEGER REFERENCES locations(id),
    first_seen DATETIME NOT NULL,
    last_seen DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ble_mac ON ble_devices(mac_address);
CREATE INDEX IF NOT EXISTS idx_ble_location ON ble_devices(location_id);
CREATE INDEX IF NOT EXISTS idx_ble_baseline ON ble_devices(is_baseline);
CREATE INDEX IF NOT EXISTS idx_ble_tracker ON ble_devices(is_tracker);

-- Device sightings (time-series data)
CREATE TABLE IF NOT EXISTS sightings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER NOT NULL,
    device_type TEXT NOT NULL CHECK(device_type IN ('wifi', 'ble')),
    rssi INTEGER NOT NULL,
    channel INTEGER,
    ssid TEXT,
    latitude REAL,
    longitude REAL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_sightings_device ON sightings(device_id, device_type);
CREATE INDEX IF NOT EXISTS idx_sightings_time ON sightings(timestamp);

-- Probe requests (WiFi)
CREATE TABLE IF NOT EXISTS probe_requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id INTEGER REFERENCES wifi_devices(id),
    ssid TEXT NOT NULL,
    rssi INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_probes_device ON probe_requests(device_id);
CREATE INDEX IF NOT EXISTS idx_probes_ssid ON probe_requests(ssid);

-- Attack events
CREATE TABLE IF NOT EXISTS attacks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    attack_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    source_mac TEXT,
    target_mac TEXT,
    bssid TEXT,
    description TEXT,
    location_id INTEGER REFERENCES locations(id),
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_attacks_type ON attacks(attack_type);
CREATE INDEX IF NOT EXISTS idx_attacks_time ON attacks(timestamp);

-- Alerts sent
CREATE TABLE IF NOT EXISTS alerts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alert_type TEXT NOT NULL,
    priority TEXT NOT NULL,
    message TEXT NOT NULL,
    device_mac TEXT,
    channels_sent TEXT,
    acknowledged BOOLEAN DEFAULT FALSE,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_alerts_time ON alerts(timestamp);
CREATE INDEX IF NOT EXISTS idx_alerts_priority ON alerts(priority);

-- GPS tracks
CREATE TABLE IF NOT EXISTS gps_tracks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    altitude REAL,
    speed REAL,
    heading REAL,
    accuracy REAL,
    satellites INTEGER,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gps_time ON gps_tracks(timestamp);

-- Device learning data (for anomaly detection)
CREATE TABLE IF NOT EXISTS device_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT NOT NULL,
    location_id INTEGER REFERENCES locations(id),
    avg_rssi REAL,
    rssi_stddev REAL,
    typical_hours TEXT,  -- JSON array of typical hours seen
    visit_frequency REAL,  -- visits per day
    avg_visit_duration REAL,  -- minutes
    probe_patterns TEXT,  -- JSON of typical probed SSIDs
    last_profile_update DATETIME,
    UNIQUE(mac_address, location_id)
);

CREATE INDEX IF NOT EXISTS idx_profiles_mac ON device_profiles(mac_address);
