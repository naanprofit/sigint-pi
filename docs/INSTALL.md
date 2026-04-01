# Installation Guide

## Quick Install

### Raspberry Pi (Zero 2 W, Pi 4, Pi 5)

```bash
# One-line install (sets up all dependencies)
curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-deck/main/scripts/install-pi.sh | bash
```

### Steam Deck

```bash
# One-line install (installs Rust, SDR tools, builds from source)
curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-deck/main/scripts/install-deck.sh | bash
```

## Manual Installation

### 1. System Requirements

#### Raspberry Pi
| Component | Minimum | Recommended |
|-----------|---------|-------------|
| Device | Pi Zero 2 W | Pi 4 (4GB) |
| OS | Debian Bookworm (ARM64) | Debian Bookworm |
| RAM | 512 MB | 2+ GB |
| Storage | 2 GB free | 8 GB+ |
| WiFi | USB adapter with monitor mode | Alfa AWUS036ACHM |

#### Steam Deck
| Component | Requirement |
|-----------|-------------|
| Device | Steam Deck LCD or OLED |
| OS | SteamOS 3.x |
| Storage | 5 GB free |
| WiFi | USB adapter (internal WiFi has no monitor mode) |

### 2. Dependencies

#### Raspberry Pi (apt)
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential pkg-config libssl-dev libsqlite3-dev \
    bluez bluetooth gpsd gpsd-clients \
    aircrack-ng iw wireless-tools \
    rtl-sdr librtlsdr-dev rtl-433 \
    hackrf libhackrf-dev \
    adb python3 curl git usbutils
```

#### Steam Deck (pacman)
```bash
sudo steamos-readonly disable
sudo pacman -Sy --needed \
    base-devel openssl pkg-config \
    bluez bluez-utils \
    wireless_tools iw \
    python curl git usbutils
sudo steamos-readonly enable
```

### 3. SDR Tools

#### RTL-SDR
Pre-installed via package managers above. Test with:
```bash
rtl_test -t
# Should show: Found 1 device(s), Tuner type: R820T/R820T2
```

#### HackRF
```bash
hackrf_info
# Should show: Found HackRF, Serial: ...
```

#### Kalibrate-RTL (Cell Tower Scanner)
Build from source if not available:
```bash
git clone https://github.com/steve-m/kalibrate-rtl.git /tmp/kalibrate-rtl
cd /tmp/kalibrate-rtl
./bootstrap && ./configure && make -j$(nproc)
sudo make install
```

#### RTL_433 (ISM Band Decoder)
```bash
# Debian/Pi
sudo apt-get install rtl-433

# Or build from source
git clone https://github.com/merbanan/rtl_433.git /tmp/rtl_433
cd /tmp/rtl_433 && mkdir build && cd build
cmake .. && make -j$(nproc) && sudo make install
```

### 4. USB Device Configuration

Create udev rules so SDR devices don't require root:
```bash
sudo tee /etc/udev/rules.d/20-rtlsdr.rules << 'EOF'
# RTL-SDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", GROUP="plugdev", MODE="0666"
# HackRF One
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", GROUP="plugdev", MODE="0666"
# LimeSDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="601f", GROUP="plugdev", MODE="0666"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger

# Blacklist DVB-T driver (conflicts with RTL-SDR)
echo "blacklist dvb_usb_rtl28xxu" | sudo tee /etc/modprobe.d/blacklist-rtlsdr.conf
echo "blacklist rtl2832" | sudo tee -a /etc/modprobe.d/blacklist-rtlsdr.conf
```

### 5. WiFi Monitor Mode

SIGINT-Deck requires a WiFi adapter that supports monitor mode:

```bash
# List wireless interfaces
iw dev

# Put adapter in monitor mode (usually wlan1)
sudo ip link set wlan1 down
sudo iw dev wlan1 set type monitor
sudo ip link set wlan1 up

# Or use airmon-ng
sudo airmon-ng start wlan1
```

**Tested adapters:**
- Alfa AWUS036ACHM (mt76x2u) - recommended
- Alfa AWUS036ACH (RTL8812AU) - needs driver install
- TP-Link TL-WN722N v1 (ath9k_htc) - 2.4 GHz only

### 6. GPS Setup

```bash
# Install gpsd
sudo apt-get install gpsd gpsd-clients

# Configure for USB GPS (VK-172 / U-blox U7)
sudo tee /etc/default/gpsd << 'EOF'
START_DAEMON="true"
GPSD_OPTIONS="-n"
DEVICES="/dev/ttyACM0"
USBAUTO="true"
EOF

sudo systemctl restart gpsd

# Test
gpsmon
# or
cgps -s
```

### 7. RayHunter IMSI Catcher Detection

RayHunter uses an Orbic RC400L phone with EFF's RayHunter firmware:

```bash
# Install ADB
sudo apt-get install adb    # Debian/Pi
sudo pacman -S android-tools # Arch/SteamOS

# Connect phone via USB
adb devices
# Should show: 24f4085e  device (or similar)

# Set up port forward (RayHunter listens on phone port 8080)
adb forward tcp:8081 tcp:8080

# Verify RayHunter is running
curl http://localhost:8081/api/system-stats
# Should return JSON with version, OS info, etc.

# (Optional) Make ADB forward persistent via systemd
mkdir -p ~/.config/systemd/user
cat > ~/.config/systemd/user/adb-forward.service << 'EOF'
[Unit]
Description=ADB Port Forward for RayHunter

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/bin/bash -c 'sleep 5 && /usr/bin/adb start-server && /usr/bin/adb forward tcp:8081 tcp:8080'
ExecStop=/usr/bin/adb kill-server

[Install]
WantedBy=default.target
EOF
systemctl --user enable adb-forward
```

**RayHunter config in config.toml:**
```toml
[rayhunter]
enabled = true
api_url = "http://localhost:8081"
poll_interval_secs = 5
alert_on_suspicious = true
```

### 8. LLM Integration (Optional)

SIGINT-Deck can use a local or remote LLM for device analysis:

```bash
# llama.cpp (lightweight)
./llama-server -m model.gguf --host 0.0.0.0 --port 8080

# Ollama
ollama serve &
ollama pull llama3

# LMStudio
# Start via GUI, enable API server
```

**Config in config.toml:**
```toml
[llm]
enabled = true
endpoint = "http://192.168.50.229:8080/v1/chat/completions"
model = "local-model"
api_key = ""
```

Note: After saving LLM settings via the web UI, the test connection works immediately. Device analysis uses the settings from startup - restart the service to pick up changes.

### 9. Configuration Reference

Full `config.toml` with all options:

```toml
[device]
name = "sigint-pi-01"      # Device identifier
location_name = "default"   # Location label

[web]
enabled = true
port = 8080                 # 8080 for Pi, 8085 for Deck
host = "0.0.0.0"           # Listen on all interfaces

[wifi]
enabled = true
interface = "wlan1"         # External USB adapter
channels_2ghz = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
channels_5ghz = [36, 40, 44, 48, 149, 153, 157, 161, 165]

[bluetooth]
enabled = true
detect_trackers = true      # AirTag/Tile/SmartTag detection

[gps]
enabled = false             # Enable if USB GPS connected
gpsd_host = "localhost"
gpsd_port = 2947

[rayhunter]
enabled = true
api_url = "http://localhost:8081"
poll_interval_secs = 5
alert_on_suspicious = true

[llm]
enabled = false
endpoint = ""
model = ""
api_key = ""

[alerts]
[alerts.sound]
enabled = true
ninja_mode = false          # Suppress all sound alerts
volume = 80

[alerts.telegram]
enabled = false
bot_token = ""
chat_id = ""

[database]
path = "sigint.db"
```

### 10. Verify Installation

```bash
# Start the service
SIGINT_ACCEPT_DISCLAIMER=1 ./sigint-deck

# Check status
curl http://localhost:8080/api/status

# Check SDR hardware detection
curl http://localhost:8080/api/sdr/status
# Returns: {"rtl_sdr": true/false, "hackrf": true/false, ...}

# Check WiFi devices
curl http://localhost:8080/api/devices

# Check RayHunter
curl http://localhost:8080/api/rayhunter/status
```

## Troubleshooting

### WiFi: "Permission denied" or "Operation not permitted"
```bash
sudo setcap cap_net_raw,cap_net_admin+eip ./sigint-deck
```

### WiFi: "Failed to set datalink type"
The adapter is not in monitor mode:
```bash
sudo airmon-ng start wlan1
```

### RTL-SDR: "PLL not locked"
Known issue on some RTL-SDR dongles (especially on Deck). The system falls back to `rtl_sdr` raw capture + FFT. Functionality is preserved with slightly higher latency.

### HackRF: "Pipe error"
Known firmware issue. The system falls back to `hackrf_transfer` + FFT. Consider updating HackRF firmware:
```bash
hackrf_spiflash -w hackrf_one_usb.bin
```

### RayHunter: "Not Available" / "Disconnected"
1. Check phone is connected: `adb devices`
2. Check ADB forward: `adb forward tcp:8081 tcp:8080`
3. Verify RayHunter API: `curl http://localhost:8081/api/system-stats`
4. If ADB shows "unauthorized", accept the debug prompt on the phone screen

### Web UI: Port already in use
```bash
ss -tlnp | grep 8080
kill $(lsof -ti:8080)
```

### Deck: Web server stops after sleep
Steam Deck suspends all user processes. Use:
```bash
systemd-inhibit --what=sleep --who=sigint-deck ./sigint-deck
```

### Pi: Out of memory
Add swap:
```bash
sudo fallocate -l 1G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```
