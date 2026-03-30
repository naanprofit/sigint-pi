# SIGINT-Pi

Portable signals intelligence and security monitoring device for Raspberry Pi Zero 2 W and Steam Deck.

> ⚠️ **LEGAL DISCLAIMER**: This tool is for authorized security research and educational purposes only. Monitoring wireless communications without authorization may be illegal in your jurisdiction. You are solely responsible for ensuring your use complies with all applicable laws. See the [Legal Notice](#legal-notice) section for full details.

## Features

- **WiFi Monitoring** (802.11)
  - Device detection and tracking
  - Probe request analysis
  - Signal strength monitoring
  - Attack detection (deauth, evil twin, KARMA)
  - PCAP capture for forensics

- **Bluetooth/BLE Monitoring**
  - BLE advertisement scanning
  - AirTag/Tile/SmartTag tracker detection with extended data parsing
  - Device type classification (Phone, Wearable, Headphones, etc.)
  - Lost mode and separated device detection

- **Tracker Intelligence** (AirTag, Tile, SmartTag)
  - Detects Find My network devices (Apple AirTags)
  - Extracts status byte, counter, and privacy-preserving key hints
  - Identifies lost mode and separated-from-owner states
  - NFC data retrieval (if physically accessible)

- **AI/LLM Integration** (Optional)
  - Device analysis via local or cloud LLM
  - Support for llama.cpp, Ollama, LMStudio, OpenAI
  - On-demand analysis (user-triggered, not automatic)
  - Local caching for offline operation
  - Threat intelligence with 100+ surveillance equipment OUIs

- **Device Intelligence**
  - OUI vendor lookup
  - Automatic baseline learning
  - Anomaly detection
  - Geofenced location awareness

- **Multi-Channel Alerts**
  - Telegram Bot
  - Twilio SMS
  - Email (SMTP)
  - MQTT (Home Assistant integration)

- **GPS Integration**
  - Location tracking
  - Geofencing
  - Movement detection

## Hardware Requirements

| Component | Recommendation | Purpose |
|-----------|---------------|---------|
| Raspberry Pi Zero 2 W | Required | Main compute |
| USB Hub | Zero4U or similar | Connect multiple USB devices |
| WiFi Adapter | Alfa AWUS036ACHM | Monitor mode, dual-band |
| GPS Module | VK-162 u-blox 7 | Location tracking |
| MicroSD Card | 64GB+ high-endurance | Storage |
| Battery | PiSugar 2 Pro (5000mAh) | ~8-10 hours runtime |

## Quick Start

### 1. Prepare the Pi

```bash
# Flash Raspberry Pi OS Lite (64-bit) to SD card
# SSH into the Pi and run:
git clone https://github.com/yourusername/sigint-pi.git
cd sigint-pi
sudo bash scripts/setup.sh
```

### 2. Configure

```bash
sudo nano /etc/sigint-pi/config.toml
```

Set up at minimum:
- `device.name` - unique identifier
- `wifi.interface` - your external WiFi adapter (usually `wlan1`)
- `alerts.telegram` - for notifications

### 3. Build and Install

On the Pi (slower):
```bash
cargo build --release
sudo cp target/release/sigint-pi /opt/sigint-pi/
```

Cross-compile (faster):
```bash
# On your development machine:
./scripts/cross-compile.sh
scp target/armv7-unknown-linux-gnueabihf/release/sigint-pi pi@<pi-ip>:/opt/sigint-pi/
```

### 4. Start

```bash
sudo systemctl enable sigint-pi
sudo systemctl start sigint-pi
sudo journalctl -u sigint-pi -f  # View logs
```

## Web Interface

Access at `http://<pi-ip>:8080`

API endpoints:
- `GET /api/status` - System status
- `GET /api/devices` - List all devices
- `GET /api/alerts` - Recent alerts
- `GET /api/stats` - Device statistics

## Alert Priority Levels

| Priority | Trigger | Channels |
|----------|---------|----------|
| Critical | Active attack detected | Telegram, SMS, Email, MQTT |
| High | New device with strong signal, tracker | Telegram, SMS, MQTT |
| Medium | New device nearby | Telegram, MQTT |
| Low | Normal activity | MQTT only |

## Power Management

For extended battery life:
```toml
[power]
low_power_mode = true
battery_scan_interval_ms = 15000
```

Estimated runtime with PiSugar 2 Pro (5000mAh):
- Normal mode: ~8 hours
- Low power mode: ~12 hours

## Attack Detection

Detects:
- Deauthentication floods
- Disassociation attacks
- Evil twin access points
- KARMA/MANA attacks
- Beacon floods

---

## Telegram Bot Setup

Telegram provides free, instant push notifications to your phone. This is the recommended alert method.

### Step 1: Create a Bot

1. Open Telegram and search for **@BotFather**
2. Send `/newbot`
3. Follow the prompts:
   - Enter a name for your bot (e.g., "SIGINT-Pi Alerts")
   - Enter a username (must end in `bot`, e.g., `my_sigint_pi_bot`)
4. BotFather will give you a **token** like: `123456789:ABCdefGHIjklMNOpqrsTUVwxyz`
5. Save this token

### Step 2: Get Your Chat ID

1. Search for **@userinfobot** in Telegram
2. Send `/start`
3. It will reply with your **Chat ID** (a number like `123456789`)

### Step 3: Configure SIGINT-Pi

Edit your config file:

```bash
sudo nano /etc/sigint-pi/config.toml
```

Add your credentials:

```toml
[alerts.telegram]
enabled = true
bot_token = "123456789:ABCdefGHIjklMNOpqrsTUVwxyz"
chat_id = "123456789"
```

### Step 4: Test

Restart the service:

```bash
sudo systemctl restart sigint-pi
```

You should receive a test alert when a new device is detected. If running in simulation mode, alerts will start immediately.

### Alert Examples

You'll receive messages like:

```
🚨 ATTACK: DeauthFlood

WiFi Attack Detected!
Type: DeauthFlood
Severity: High
Source: AA:BB:CC:DD:EE:FF

📍 Location: home
🕐 Time: 2024-01-15 14:32:15 UTC
```

```
⚠️ New WiFi Device: AA:BB:CC:11:22:33

New device detected
MAC: AA:BB:CC:11:22:33
Vendor: Apple
RSSI: -45 dBm
Channel: 6

📍 Location: home
```

---

## Home Assistant Integration

SIGINT-Pi integrates with Home Assistant via MQTT, allowing you to:
- Display device counts on dashboards
- Trigger automations based on alerts
- Track presence based on detected devices
- Get notifications through HA's notification system

### Prerequisites

- Home Assistant with MQTT integration enabled
- MQTT broker (Mosquitto recommended - can run on HA or separately)

### Step 1: Set Up MQTT Broker

If you don't have an MQTT broker, install Mosquitto on Home Assistant:

1. Go to **Settings → Add-ons → Add-on Store**
2. Search for "Mosquitto broker"
3. Click **Install**, then **Start**
4. Go to **Configuration** tab and note the credentials

Or use the included Docker Compose:

```bash
cd /path/to/sigint-pi
docker-compose --profile mqtt up -d mosquitto
```

### Step 2: Configure SIGINT-Pi for MQTT

Edit your config:

```toml
[alerts.mqtt]
enabled = true
broker_host = "192.168.1.100"  # Your HA/MQTT broker IP
broker_port = 1883
client_id = "sigint-pi"
topic_prefix = "sigint"
username = "mqtt_user"         # Optional, if broker requires auth
password = "mqtt_password"     # Optional
```

### Step 3: Configure Home Assistant

Add MQTT sensors to your `configuration.yaml`:

```yaml
mqtt:
  sensor:
    # Device counts
    - name: "SIGINT WiFi Devices"
      state_topic: "sigint/stats"
      value_template: "{{ value_json.wifi_total }}"
      icon: mdi:wifi

    - name: "SIGINT BLE Devices"
      state_topic: "sigint/stats"
      value_template: "{{ value_json.ble_total }}"
      icon: mdi:bluetooth

    - name: "SIGINT New Devices"
      state_topic: "sigint/stats"
      value_template: "{{ value_json.new_devices }}"
      icon: mdi:account-alert

    # GPS Location
    - name: "SIGINT-Pi Location"
      state_topic: "sigint/gps"
      value_template: "{{ value_json.latitude }}, {{ value_json.longitude }}"
      icon: mdi:crosshairs-gps

  binary_sensor:
    # Attack detection
    - name: "SIGINT Attack Detected"
      state_topic: "sigint/alerts/critical"
      value_template: "{{ 'attack' in value_json.alert_type | lower }}"
      device_class: safety
      payload_on: "true"
      payload_off: "false"

    # Tracker detection (AirTag, Tile, etc.)
    - name: "SIGINT Tracker Detected"
      state_topic: "sigint/alerts/high"
      value_template: "{{ 'tracker' in value_json.alert_type | lower }}"
      device_class: presence
      payload_on: "true"
      payload_off: "false"
```

### Step 4: Create Automations

**Example: Send notification when attack detected**

```yaml
automation:
  - alias: "SIGINT Attack Alert"
    trigger:
      - platform: mqtt
        topic: "sigint/alerts/critical"
    action:
      - service: notify.mobile_app_your_phone
        data:
          title: "🚨 WiFi Attack Detected!"
          message: "{{ trigger.payload_json.description }}"
          data:
            priority: high
            ttl: 0

  - alias: "SIGINT Unknown Tracker Alert"
    trigger:
      - platform: mqtt
        topic: "sigint/alerts/high"
    condition:
      - condition: template
        value_template: "{{ 'tracker' in trigger.payload_json.alert_type | lower }}"
    action:
      - service: notify.mobile_app_your_phone
        data:
          title: "⚠️ Tracker Detected!"
          message: "{{ trigger.payload_json.message }}"

  - alias: "SIGINT New Strong Signal Device"
    trigger:
      - platform: mqtt
        topic: "sigint/alerts/high"
    condition:
      - condition: template
        value_template: "{{ trigger.payload_json.rssi | int > -50 }}"
    action:
      - service: notify.persistent_notification
        data:
          title: "New Device Nearby"
          message: "Strong signal device: {{ trigger.payload_json.device_mac }}"
```

### Step 5: Dashboard Cards

Add to your Lovelace dashboard:

```yaml
type: entities
title: SIGINT-Pi Security
entities:
  - entity: sensor.sigint_wifi_devices
    name: WiFi Devices
  - entity: sensor.sigint_ble_devices
    name: BLE Devices
  - entity: sensor.sigint_new_devices
    name: New Devices (24h)
  - entity: binary_sensor.sigint_attack_detected
    name: Attack Status
  - entity: binary_sensor.sigint_tracker_detected
    name: Tracker Alert
```

**Gauge card for signal monitoring:**

```yaml
type: gauge
entity: sensor.sigint_new_devices
name: Unknown Devices
min: 0
max: 20
severity:
  green: 0
  yellow: 5
  red: 10
```

### MQTT Topic Reference

SIGINT-Pi publishes to these topics:

| Topic | Description | Payload |
|-------|-------------|---------|
| `sigint/alerts/critical` | Attack alerts | JSON with alert details |
| `sigint/alerts/high` | High priority (trackers, strong signals) | JSON with alert details |
| `sigint/alerts/medium` | New devices | JSON with alert details |
| `sigint/alerts/low` | Normal activity | JSON with alert details |
| `sigint/devices/{mac}` | Per-device updates | JSON with RSSI, vendor, etc. |
| `sigint/stats` | Overall statistics | JSON with device counts |
| `sigint/gps` | GPS position | JSON with lat/lon |

### Example MQTT Payloads

**Alert payload:**
```json
{
  "id": "abc123",
  "priority": "High",
  "alert_type": "NewDevice",
  "title": "New WiFi Device: AA:BB:CC:DD:EE:FF",
  "message": "New device detected...",
  "device_mac": "AA:BB:CC:DD:EE:FF",
  "device_vendor": "Apple",
  "rssi": -45,
  "location": "home",
  "timestamp": "2024-01-15T14:32:15Z"
}
```

**Device payload:**
```json
{
  "mac": "AA:BB:CC:DD:EE:FF",
  "vendor": "Apple",
  "rssi": -52,
  "device_type": "wifi",
  "last_seen": "2024-01-15T14:35:00Z"
}
```

---

## Steam Deck Support

SIGINT-Pi runs on Steam Deck OLED/LCD using rootless Podman containers.

### Critical Hardware Requirements

> ⚠️ **The Steam Deck's internal WiFi (wlan0) does NOT support monitor mode!**
> You MUST use an external USB WiFi adapter for WiFi packet capture.

**Recommended Setup:**
| Component | Recommendation | Notes |
|-----------|---------------|-------|
| USB WiFi Adapter | Alfa AWUS036ACHM | Dual-band, excellent Linux support |
| USB GPS | VK-162 u-blox 7 | For location tracking |
| USB Hub | Powered hub recommended | Ensures stable power to devices |

### Installation on Steam Deck

```bash
# SSH into Steam Deck (enable SSH in Settings)
ssh deck@steamdeck.local

# Clone the repo
git clone https://github.com/naanprofit/sigint-pi.git
cd sigint-pi

# Create config directories
mkdir -p ~/.local/share/sigint-pi/data ~/.config/sigint-pi
cp config.toml.example ~/.config/sigint-pi/config.toml

# Build the container
podman build -f Containerfile.steamdeck -t sigint-pi:steamdeck .

# Run in simulation mode (no special hardware needed)
podman-compose -f podman-compose.steamdeck.yml --profile simulation up -d

# Or run with real hardware (requires external USB WiFi)
# First, put your USB WiFi adapter in monitor mode:
sudo ./scripts/wifi-monitor.sh wlan1

# Then start with hardware profile:
podman-compose -f podman-compose.steamdeck.yml --profile hardware up -d
```

### Adding to Steam as Non-Steam App

1. Copy launcher script: `cp scripts/steamdeck-launch.sh ~/bin/`
2. Make executable: `chmod +x ~/bin/steamdeck-launch.sh`
3. In Steam Desktop Mode: **Games → Add Non-Steam Game**
4. Browse to `~/bin/steamdeck-launch.sh` and add it
5. Optionally set a custom icon and controller configuration

### Steam Deck Scripts

| Script | Purpose |
|--------|---------|
| `steamdeck-launch.sh` | Start container and open browser |
| `steamdeck-stop.sh` | Stop container cleanly |
| `steamdeck-status.sh` | Show hardware and runtime status |
| `wifi-list.sh` | List wireless interfaces and capabilities |
| `wifi-monitor.sh` | Enable monitor mode on interface |
| `wifi-managed.sh` | Restore managed mode |

---

## Signal Messenger Alerts

Signal provides end-to-end encrypted alert delivery using signal-cli.

### Setup

1. **Install signal-cli:**
```bash
# Download from https://github.com/AsamK/signal-cli/releases
wget https://github.com/AsamK/signal-cli/releases/download/v0.13.2/signal-cli-0.13.2-Linux.tar.gz
tar xf signal-cli-*.tar.gz
sudo mv signal-cli-*/bin/signal-cli /usr/local/bin/
sudo mv signal-cli-*/lib /usr/local/lib/signal-cli
```

2. **Register a phone number (requires receiving SMS):**
```bash
signal-cli -a +1YOURNUMBER register
signal-cli -a +1YOURNUMBER verify CODE_FROM_SMS
```

3. **Configure SIGINT-Pi:**
```toml
[alerts.signal]
enabled = true
sender_number = "+1YOURNUMBER"
recipients = ["+1RECIPIENT1", "+1RECIPIENT2"]
signal_cli_path = "/usr/local/bin/signal-cli"
config_dir = "/etc/sigint-pi/signal"
min_priority = "high"
```

4. **Test:**
```bash
signal-cli -a +1YOURNUMBER send -m "Test from SIGINT-Pi" +1RECIPIENT
```

---

## OpenClaw Integration

OpenClaw is a centralized alert aggregation platform that can receive alerts from multiple SIGINT-Pi devices.

### Configuration

```toml
[alerts.openclaw]
enabled = true
api_url = "https://api.openclaw.io/v1/alerts"
api_key = "your-api-key-here"
device_id = "sigint-pi-001"
device_name = "SIGINT-Pi Living Room"
tags = ["home", "primary"]
include_raw_data = false
min_priority = "medium"
```

### API Key Setup

1. Create an account at your OpenClaw instance (or https://openclaw.io)
2. Navigate to **Settings → API Keys**
3. Create a new API key with `alerts:write` scope
4. Copy the key to your config.toml

### Alert Payload Format

SIGINT-Pi sends alerts to OpenClaw in this JSON format:

```json
{
    "device_id": "sigint-pi-001",
    "device_name": "SIGINT-Pi Living Room",
    "alert_type": "new_device|attack|tracker|geofence",
    "priority": "low|medium|high|critical",
    "title": "Alert Title",
    "message": "Detailed message",
    "timestamp": 1234567890,
    "location": {
        "latitude": 40.7128,
        "longitude": -74.0060
    },
    "tags": ["home", "primary"],
    "metadata": {
        "mac_address": "AA:BB:CC:DD:EE:FF",
        "vendor": "Apple Inc",
        "rssi": -45
    }
}
```

### Multi-Device Setup

For multiple SIGINT-Pi devices reporting to one OpenClaw instance:
- Use unique `device_id` for each device
- Use descriptive `device_name` for identification
- Use tags to group devices by location/purpose

### Testing

```bash
curl -X POST https://api.openclaw.io/v1/alerts \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"device_id":"test","alert_type":"test","title":"Test","message":"Test message","timestamp":0}'
```

---

## Tracker Detection & AirTag Data

SIGINT-Pi provides advanced detection and analysis of Bluetooth tracking devices.

### Supported Trackers

| Tracker | Detection | Extended Data |
|---------|-----------|---------------|
| Apple AirTag | ✅ | Status, Lost Mode, Counter, Key Hint |
| Apple Find My (3rd party) | ✅ | Status, Counter |
| Tile | ✅ | Basic |
| Samsung SmartTag | ✅ | Basic |

### What Data Can Be Extracted

**From BLE Advertisements (Passive Scanning):**

| Data | Available | Notes |
|------|-----------|-------|
| Detect as tracker | ✅ | Payload type 0x12 = Find My |
| Signal strength (RSSI) | ✅ | Estimate proximity |
| Status byte | ✅ | Indicates device state |
| Lost mode flag | ✅ | Partial - from status byte |
| Separated from owner | ✅ | From status byte (>3 days away) |
| Counter/nonce | ✅ | Changes every 15 minutes |
| Key hint (fingerprint) | ✅ | Privacy-preserving, for session correlation |
| Owner identity | ❌ | Encrypted, only Apple can decrypt |
| Serial number | ❌ | Not in BLE broadcasts |
| Device name | ❌ | Not in BLE broadcasts |
| Location history | ❌ | Stored on Apple servers only |

**From NFC Tap (Physical Access Required):**

| Data | Available | Notes |
|------|-----------|-------|
| Found URL | ✅ | Links to found.apple.com |
| Owner contact info | ✅ | Only if Lost Mode enabled by owner |
| Serial number | ❌ | Not exposed via NFC |

### Privacy by Design

Apple AirTags are designed with strong privacy protections:

1. **Rotating Public Key** - The EC P-224 public key rotates daily, preventing long-term tracking
2. **Rotating MAC Address** - BLE address changes with the key
3. **End-to-End Encryption** - Location data encrypted with owner's iCloud keys
4. **No Owner Identification** - Cannot determine who owns an AirTag from BLE alone

### Dashboard Display

The dashboard shows tracker-specific information:

- **Type Badge**: AirTag, Tile, SmartTag
- **LOST MODE**: Red badge when tracker appears to be in lost mode
- **SEPARATED**: Yellow badge when tracker is separated from owner (>3 days)
- **Key Hint**: Privacy-preserving fingerprint for session correlation (e.g., `ab12..ff`)
- **Status**: Raw status byte in hex
- **Counter**: Nonce value (changes every 15 minutes)

### Use Cases

1. **Detect Unwanted Tracking** - Alert when unknown trackers follow you
2. **Stalker Detection** - Identify persistent trackers across sessions
3. **Security Awareness** - Know what BLE trackers are in your environment
4. **Lost Item Recovery** - NFC tap reveals owner contact if Lost Mode enabled

### Ethical Considerations

- SIGINT-Pi cannot identify AirTag owners from BLE data alone
- Designed for defensive use (detecting trackers following you)
- Does not enable stalking or privacy invasion
- Key hints are session-only and not stored long-term

---

## AI/LLM Device Analysis

SIGINT-Pi can optionally use AI to analyze detected devices and provide threat assessments.

### Supported Providers

| Provider | Type | Configuration |
|----------|------|---------------|
| llama.cpp | Local | `http://localhost:8080/v1` |
| Ollama | Local | `http://localhost:11434/v1` |
| LMStudio | Local | `http://localhost:1234/v1` |
| OpenAI | Cloud | `https://api.openai.com/v1` |
| Custom | Any | OpenAI-compatible API endpoint |

### Configuration

```toml
[llm]
enabled = true
provider = "llamacpp"  # llamacpp, ollama, lmstudio, openai, custom
endpoint = "http://192.168.1.100:8080/v1"
model = "llama-3.2-3b"
api_key = ""  # Required for OpenAI, optional for local
timeout_secs = 30
max_tokens = 500
temperature = 0.3
```

### Features

- **On-Demand Analysis**: User-triggered, not automatic (saves resources)
- **Threat Assessment**: Identifies surveillance equipment, suspicious patterns
- **Device Classification**: Enhanced device type identification
- **Local Caching**: Stores descriptions for offline access
- **Privacy-Preserving**: Only analyzes OUI/vendor data, not personal info

### Threat Intelligence Database

SIGINT-Pi includes a built-in database of 100+ surveillance equipment OUIs:

| Category | Examples |
|----------|----------|
| US Defense | Harris (Stingray), L3Harris, Raytheon, Lockheed Martin |
| Law Enforcement | Cellebrite, Digital Intelligence, MSAB |
| Chinese State-Linked | Huawei, ZTE, Hikvision, Dahua |
| Israeli | NSO Group, Elbit Systems, Rafael |
| European | Thales, BAE Systems, Airbus Defence |

### API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/ai/status` | GET | Check AI availability |
| `/api/ai/analyze` | POST | Analyze device(s) |
| `/api/settings/llm` | GET/POST | Configure LLM settings |

### Dashboard Integration

- **AI Status Indicator**: Shows if AI is available
- **Analyze Button**: Trigger analysis for selected devices
- **Threat Badges**: Visual indicators for suspicious devices

---

## Sound Alerts & Ninja Mode

### Sound Alerts

SIGINT-Pi can play audio alerts for various events:

```toml
[alerts.sound]
enabled = true
ninja_mode = false
volume = 70
new_device_sound = true
tracker_sound = true
attack_sound = true
geofence_sounds = true
```

### Ninja Mode 🥷

**Ninja Mode** silences ALL audio alerts and can disable visual indicators for covert operation:

- Toggle via Settings UI or API: `POST /api/settings/ninja_mode`
- Keyboard shortcut in dashboard: Press `N`
- All sounds are immediately silenced
- Useful when you need silent monitoring

---

## Running on macOS (Development/Testing)

You can run SIGINT-Pi in simulation mode on your Mac for testing:

```bash
cd sigint-pi

# Copy and edit config
cp config.toml.example config.toml
nano config.toml  # Add your Telegram token

# Run with Docker
docker-compose up sigint-pi
```

Dashboard: http://localhost:8080

**Note:** WiFi monitor mode doesn't work on macOS. Simulation mode generates realistic fake device traffic for testing the full alert pipeline.

---

## Legal Notice

### ⚠️ IMPORTANT LEGAL DISCLAIMER

**READ THIS BEFORE USING SIGINT-Pi**

This software is provided for **authorized security research, penetration testing, and educational purposes ONLY**.

#### Legal Compliance

- **You are solely responsible** for ensuring your use of this tool complies with all applicable local, state, federal, and international laws.
- **Monitoring wireless communications without authorization is ILLEGAL** in most jurisdictions.
- The legality of passive WiFi monitoring varies by country and region.

#### Authorized Use Only

- **ONLY** use this tool on networks and devices you own or have explicit written permission to monitor.
- Unauthorized interception of communications may result in **criminal prosecution**.

#### Prohibited Uses

Do NOT use this tool to:
- Intercept, capture, or analyze communications of third parties without consent
- Stalk, harass, or invade the privacy of others
- Conduct attacks on networks or devices
- Any illegal or unethical purpose

#### Hardware Requirements

For proper operation, you MUST use appropriate hardware:

| Requirement | Details |
|-------------|---------|
| **WiFi Monitor Mode** | Requires an **external USB WiFi adapter** that supports monitor mode. The Steam Deck's internal WiFi does NOT support monitor mode. Recommended: Alfa AWUS036ACHM |
| **GPS** | Requires an **external USB GPS receiver** (e.g., VK-162 u-blox 7) |
| **Portable Power** | Use a **powered USB hub** to ensure stable power delivery to all connected USB devices |

#### No Warranty

This software is provided "AS IS" without warranty of any kind. The authors are not responsible for any damages, legal issues, or consequences arising from the use of this software.

---

## Support

If you find SIGINT-Pi useful, consider supporting development:

**Bitcoin:** `3GD3hpufcCPCemfQdoAFu9JH5Td5US1pzJ`

## License

MIT License - See LICENSE file

## Author

**sigkill**

---

*By using this software, you acknowledge that you have read, understood, and agree to comply with all applicable laws and the terms of this disclaimer.*
