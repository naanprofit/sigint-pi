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

## License

MIT License - See LICENSE file

## Security Note

This tool is for authorized security monitoring only. Always ensure you have permission to monitor wireless networks in your area. Unauthorized interception of wireless communications may be illegal in your jurisdiction.
