# SIGINT-Pi

Portable signals intelligence and security monitoring device for Raspberry Pi Zero 2 W.

## Features

- **WiFi Monitoring** (802.11)
  - Device detection and tracking
  - Probe request analysis
  - Signal strength monitoring
  - Attack detection (deauth, evil twin, KARMA)
  - PCAP capture for forensics

- **Bluetooth/BLE Monitoring**
  - BLE advertisement scanning
  - AirTag/Tile tracker detection
  - Device type classification

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

## License

MIT License - See LICENSE file

## Security Note

This tool is for authorized security monitoring only. Always ensure you have permission to monitor wireless networks in your area. Unauthorized interception of wireless communications may be illegal in your jurisdiction.
