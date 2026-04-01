# SIGINT-Deck Steam Deck Deployment

## Overview

SIGINT-Deck runs on Steam Deck as a native binary (not container) for best performance and hardware access.

## What's Shared with SIGINT-Pi

Most features work on both platforms:

| Feature | SIGINT-Pi (Raspberry Pi) | SIGINT-Deck (Steam Deck) |
|---------|--------------------------|--------------------------|
| WiFi Monitor Mode | Yes (with compatible adapter) | Yes (external USB only) |
| BLE Scanning | Yes | Yes |
| GPS Support | Yes | Yes |
| Device Learning/ML | Yes | Yes |
| Anomaly Detection | Yes | Yes |
| Device Fingerprinting | Yes | Yes |
| Web Dashboard | Yes | Yes |
| LLM/AI Analysis | Yes | Yes |
| PCAP Capture | Yes | Yes |
| Threat Intel | Yes | Yes |
| Sound Alerts | Yes | Yes |

### Steam Deck Specific

| Feature | Notes |
|---------|-------|
| Gaming Mode Launch | Launch from Steam library |
| Channel Hopping Service | Separate systemd service (sudoers required) |
| Interface Naming | systemd .link files for wlan0/wlan1 |
| Internal WiFi | Does NOT support monitor mode |

## Hardware Requirements

| Component | Required | Notes |
|-----------|----------|-------|
| Steam Deck | Yes | LCD or OLED |
| External USB WiFi Adapter | Yes | Internal WiFi does NOT support monitor mode |
| USB GPS Receiver | Optional | For location tracking (U-Blox recommended) |
| USB Hub | Recommended | Powered hub for multiple devices |

### Tested Hardware

| Device | USB ID | Driver | Status |
|--------|--------|--------|--------|
| MediaTek MT7612U | 0e8d:7612 | mt76x2u | ✅ Working |
| U-Blox 7 GPS | 1546:01a7 | cdc_acm | ✅ Working |
| Steam Deck Internal WiFi | - | ath11k_pci | ❌ No monitor mode |

### Recommended WiFi Adapters

- **MediaTek MT7612U** (tested, confirmed working)
- Alfa AWUS036ACHM
- Alfa AWUS036ACH
- Panda PAU09

## Launch from Steam (Gaming Mode)

You can launch SIGINT-Deck from Steam's Gaming Mode:

### Setup

1. Copy the launch script:
   ```bash
   cp ~/sigint-pi/steamdeck/launch-in-steam.sh ~/sigint-pi/
   chmod +x ~/sigint-pi/launch-in-steam.sh
   ```

2. In **Desktop Mode**, open Steam

3. Go to **Games → Add a Non-Steam Game**

4. Click **Browse** and navigate to:
   ```
   /home/deck/sigint-pi/launch-in-steam.sh
   ```

5. Click **Add Selected Programs**

6. Find it in your library, right-click → **Properties**:
   - Rename to "SIGINT-Deck"
   - Optionally set a custom icon

### Usage in Gaming Mode

1. Launch "SIGINT-Deck" from your library
2. Press the **Steam button** to open overlay
3. Select **Web Browser** from overlay
4. Dashboard loads automatically at `http://localhost:8080`

The launch script:
- Starts all required services
- Opens the dashboard in Steam's browser
- Shows live status in the terminal

### Controller Navigation

The web dashboard is touch/controller friendly:
- D-pad/stick to navigate
- A button to select
- Tabs at top: WiFi, BLE, New, Alerts, Attacks, GPS, Settings

## Installation

### 1. Copy Files to Steam Deck

```bash
# From your development machine
scp -r sigint-pi deck@steamdeck:~/
```

### 2. Run Setup Script (requires sudo)

```bash
# On Steam Deck
sudo ~/sigint-pi/setup-steamdeck.sh
```

This script:
- Creates systemd .link files for persistent interface naming
- Locks internal WiFi to `wlan0` and external USB to `wlan1`
- Prevents NetworkManager from managing the external adapter
- Sets up GPS device permissions
- Configures gpsd (if installed)
- Creates monitor mode service

### 3. Unplug and Replug External WiFi Adapter

**IMPORTANT:** After running setup, physically unplug and replug the USB WiFi adapter. This triggers the new naming rules.

### 4. Verify Interface Names

```bash
ip link show wlan0   # Should be internal (ath11k_pci)
ip link show wlan1   # Should be external (mt76x2u)
```

### 5. Enable User Service

```bash
# Enable lingering (runs service even when logged out)
loginctl enable-linger deck

# Enable and start the service
systemctl --user enable sigint-pi
systemctl --user start sigint-pi
```

### 6. Enable Monitor Mode

```bash
sudo systemctl start sigint-monitor-mode
systemctl --user restart sigint-pi
```

## Interface Naming (The Problem & Solution)

### The Problem

Steam Deck assigns WiFi interface names dynamically. When you plug in an external USB WiFi adapter, it can:
- Steal `wlan0` from the internal WiFi
- Get assigned random names like `wlan3`, `wlan6`, etc.
- Break your network connection

This happens because:
1. Steam Deck uses predictable interface naming based on device path
2. USB devices can enumerate in different orders
3. Desktop Mode and Game Mode handle networking differently

### The Solution

We use **systemd .link files** to assign permanent interface names based on MAC address:

```
/etc/systemd/network/10-wlan0-internal.link  -> Internal WiFi always wlan0
/etc/systemd/network/10-wlan1-external.link  -> External USB always wlan1
```

This is more reliable than udev `NAME=` rules on modern systemd systems.

### Interface Mapping

| Interface | MAC Address | Device | Purpose |
|-----------|-------------|--------|---------|
| `wlan0` | dc:2e:97:2f:8f:f8 | Internal (ath11k_pci) | Network connection |
| `wlan1` | 9c:ef:d5:f8:95:2d | External (mt76x2u) | Monitor mode capture |

**Note:** If your hardware has different MAC addresses, edit the setup script before running.

## GPS Setup

### Hardware

SIGINT-Pi supports U-Blox GPS receivers (tested with U-Blox 7).

### Requirements

GPS requires `gpsd` daemon. On Steam Deck:

```bash
# Disable read-only filesystem
sudo steamos-readonly disable

# Install gpsd
sudo pacman -S gpsd

# Re-enable read-only
sudo steamos-readonly enable

# Run setup script again to configure gpsd
sudo ~/sigint-pi/setup-steamdeck.sh
```

### Configuration

The setup script creates:
- `/etc/udev/rules.d/91-sigint-gps.rules` - Device permissions and `/dev/gps0` symlink
- `/etc/default/gpsd` - gpsd configuration pointing to `/dev/ttyACM1`

### Verify GPS

```bash
# Check device exists
ls -la /dev/ttyACM* /dev/gps0

# Test raw GPS data
cat /dev/ttyACM1

# Check gpsd status
systemctl status gpsd

# Test gpsd
gpsmon
```

## Services

### User Service: sigint-pi

```bash
# Status
systemctl --user status sigint-pi

# Restart
systemctl --user restart sigint-pi

# Logs
journalctl --user -u sigint-pi -f

# Enable at boot
systemctl --user enable sigint-pi
```

### System Service: sigint-monitor-mode

```bash
# Enable monitor mode
sudo systemctl start sigint-monitor-mode

# Disable monitor mode  
sudo systemctl stop sigint-monitor-mode

# Enable at boot
sudo systemctl enable sigint-monitor-mode

# Check status
sudo systemctl status sigint-monitor-mode
```

## Configuration

Config file: `~/sigint-pi/config.toml`

### Key Settings

```toml
[wifi]
enabled = true
interface = "wlan1"  # External USB adapter

[bluetooth]
enabled = true
detect_airtags = true

[gps]
enabled = true
gpsd_host = "127.0.0.1"
gpsd_port = 2947

[alerts.sound]
enabled = true
ninja_mode = false  # Set true for silent operation
```

## Web Dashboard

Access at: `http://steamdeck:8080` or `http://<ip>:8080`

Features:
- Real-time BLE device list (sorted: trackers first, then NEW, then by signal)
- AirTag/tracker detection with visual highlighting
- WiFi device monitoring (when monitor mode enabled)
- Hardware status indicators
- GPS location display

## Troubleshooting

### No WiFi devices detected (most common issue)

**Symptoms:**
- Dashboard shows 0 WiFi devices
- BLE devices work fine
- Logs may show "Permission denied" or channel hop errors

**Cause:** External WiFi adapter is not in monitor mode (probably in "managed" mode and connected to a network).

**Diagnosis:**
```bash
# Check current mode
iw dev wlan1 info | grep type
# If it says "managed" instead of "monitor", that's the problem
```

**Solution:**
```bash
# Option 1: Use the helper script
sudo ~/sigint-deck/enable-monitor.sh wlan1
systemctl --user restart sigint-deck

# Option 2: Manual steps
sudo nmcli device disconnect wlan1
sudo nmcli device set wlan1 managed no
sudo ip link set wlan1 down
sudo iw wlan1 set type monitor  
sudo ip link set wlan1 up
systemctl --user restart sigint-deck

# Verify it worked
iw dev wlan1 info | grep type  # Should show "monitor"
curl -s http://localhost:8080/api/wifi/devices | python3 -c "import json,sys; print(f'WiFi devices: {len(json.load(sys.stdin))}')"
```

**Prevention:** The installer sets up a systemd service to enable monitor mode at boot. If it's not working:
```bash
sudo systemctl enable sigint-monitor-mode
sudo systemctl start sigint-monitor-mode
```

### Channel hop errors: "Device or resource busy (-16)"

**Symptoms:**
- Logs show `Channel hop failed: command failed: Device or resource busy (-16)`

**Cause:** This is normal and happens briefly when the adapter is busy processing packets.

**Solution:** No action needed. WiFi capture still works. If ALL channel hops fail continuously:
```bash
# Re-enable monitor mode
sudo ~/sigint-deck/enable-monitor.sh wlan1
systemctl --user restart sigint-deck
```

### Service stuck stopping / won't restart

**Symptoms:**
- `systemctl --user restart sigint-deck` hangs
- Status shows "deactivating (stop-sigterm)"

**Solution:**
```bash
# Force kill
pkill -9 sigint-deck

# Reset and restart
systemctl --user reset-failed sigint-deck
systemctl --user start sigint-deck channel-hop
```

### WiFi not working after suspend/resume

**Symptoms:**
- WiFi was working before Steam Deck went to sleep
- After waking up, no WiFi devices detected
- Interface may show as DOWN or in wrong mode
- Interface name may have changed (wlan137, wlan142, etc.)

**Cause:** The MT7612U driver doesn't handle suspend/resume well. The interface gets reset or disappears.

**Solution:**
```bash
# 1. Check if interface exists
ip link show | grep wlan

# 2. If interface has wrong name (e.g., wlan137), reload driver:
sudo modprobe -r mt76x2u
sleep 2
sudo modprobe mt76x2u
sleep 3

# 3. Set monitor mode
~/sigint-deck/set-monitor-mode.sh

# 4. Restart service
systemctl --user restart sigint-deck
```

**If interface keeps getting random names:**
```bash
# Clean up phantom interfaces (only wlan0 and wlan1 should exist)
for i in $(ip link show | grep -oE "wlan[0-9]+" | grep -vE "wlan[01]$"); do
    sudo ip link delete $i 2>/dev/null
done
```

**Prevention:** The installer creates a `sigint-monitor-mode.service` that runs on boot. For resume, you can use the web UI to toggle monitor mode, or run the script manually.

**Web UI Toggle:** Go to Settings > Scanning > Monitor Mode to toggle between monitor and managed mode.

### External adapter steals wlan0 / gets wrong name

**Symptoms:**
- Network connection drops when plugging in USB adapter
- External adapter shows as wlan3, wlan6, etc.
- `wlan1` doesn't exist

**Solution:**
1. Run the setup script:
   ```bash
   sudo ~/sigint-deck/setup-steamdeck.sh
   ```
2. Unplug the external WiFi adapter
3. Wait 5 seconds
4. Plug it back in
5. Verify:
   ```bash
   ip link show wlan0  # Internal
   ip link show wlan1  # External
   ```

### WiFi scanner not working (interface not found)

**Symptoms:**
- Dashboard shows 0 WiFi devices
- Logs show "Interface wlan1 not found" or "Failed to open capture"

**Checklist:**

1. Check interface exists:
   ```bash
   ip link show wlan1
   ```

2. Check interface is in monitor mode:
   ```bash
   iw dev wlan1 info | grep type
   # Should show: type monitor
   ```

3. If not in monitor mode:
   ```bash
   sudo systemctl start sigint-monitor-mode
   ```

4. Restart SIGINT-Pi:
   ```bash
   systemctl --user restart sigint-pi
   ```

### BLE not detecting devices

**Symptoms:**
- Dashboard shows 0 BLE devices
- No AirTags detected

**Checklist:**

1. Check Bluetooth service:
   ```bash
   systemctl status bluetooth
   ```

2. Check adapter is available:
   ```bash
   bluetoothctl show
   ```

3. Check SIGINT-Pi logs:
   ```bash
   journalctl --user -u sigint-pi | grep -i ble
   ```

### GPS not working

**Symptoms:**
- GPS indicator shows red
- No location data

**Checklist:**

1. Check GPS device exists:
   ```bash
   ls -la /dev/ttyACM*
   ```

2. Test raw GPS output:
   ```bash
   cat /dev/ttyACM1
   # Should show NMEA sentences like $GPRMC, $GPGGA
   ```

3. Check gpsd is running:
   ```bash
   systemctl status gpsd
   ```

4. If gpsd not installed:
   ```bash
   sudo steamos-readonly disable
   sudo pacman -S gpsd
   sudo steamos-readonly enable
   sudo ~/sigint-pi/setup-steamdeck.sh
   ```

5. Test gpsd connection:
   ```bash
   gpsmon
   # or
   gpspipe -w
   ```

### Service keeps dying

**Symptoms:**
- Service stops unexpectedly
- Have to keep restarting

**Checklist:**

1. Check logs for errors:
   ```bash
   journalctl --user -u sigint-pi -n 100
   ```

2. Common causes:
   - Config file syntax error
   - Missing interface (adapter unplugged)
   - Permission denied (for WiFi capture)

3. Verify config syntax:
   ```bash
   cat ~/sigint-pi/config.toml | head -50
   ```

### Dashboard shows "Disconnected"

This is **normal behavior**. The dashboard uses polling (not WebSocket) to fetch data. The "Disconnected" indicator refers to WebSocket status which is not implemented.

Data still updates via REST API polling every 5-10 seconds.

### Network drops in Game Mode

Steam Deck handles networking differently in Game Mode. The setup script's systemd .link files should persist across modes, but if issues occur:

1. Switch to Desktop Mode
2. Verify interface names
3. Re-run setup if needed
4. Unplug/replug adapter

## Files

```
~/sigint-pi/
├── sigint-pi-bin          # Main binary
├── config.toml            # Configuration
├── sigint.log             # Application log
├── data/                  # Database and pcap files
├── static/                # Web dashboard files
├── setup-steamdeck.sh     # Initial setup (run with sudo)
├── set-monitor-mode.sh    # Enable monitor mode
├── unset-monitor-mode.sh  # Disable monitor mode
└── docs/
    └── README-STEAMDECK.md  # This file
```

### System Files Created by Setup

```
/etc/systemd/network/
├── 10-wlan0-internal.link    # Lock internal WiFi to wlan0
└── 10-wlan1-external.link    # Lock external USB to wlan1

/etc/udev/rules.d/
├── 90-sigint-wifi.rules      # Mark wlan1 as NM_UNMANAGED
└── 91-sigint-gps.rules       # GPS permissions and /dev/gps0 symlink

/etc/NetworkManager/conf.d/
└── 90-sigint-unmanaged.conf  # Ignore wlan1

/etc/systemd/system/
└── sigint-monitor-mode.service  # Auto-enable monitor mode

/etc/default/
└── gpsd                      # gpsd configuration (if installed)
```

## Security Notes

- WiFi capture requires root (handled by monitor-mode service)
- BLE scanning works without root
- Dashboard accessible on local network only
- No data sent externally unless alerts configured
- GPS location is only stored locally unless webhook alerts enabled

## Startup Orchestration

SIGINT-Pi requires multiple components started in the correct order:

1. **gpsd** - GPS daemon (needs GPS device)
2. **Monitor mode** - External WiFi in monitor mode (needs sudo)
3. **Capabilities** - Binary needs `cap_net_raw` for packet capture
4. **sigint-pi** - Main application service

### Using the Master Start Script (Recommended)

```bash
# Start everything in correct order
sudo ~/sigint-pi/start-sigint.sh

# Stop everything
sudo ~/sigint-pi/stop-sigint.sh

# Check status
~/sigint-pi/status-sigint.sh
```

### Auto-Start at Boot

```bash
# Enable master orchestration service
sudo systemctl enable sigint-pi-master

# This will run start-sigint.sh at boot
```

### Manual Control

```bash
# Start GPS daemon
sudo gpsd -n /dev/ttyACM1

# Enable monitor mode
sudo ip link set wlan1 down
sudo iw wlan1 set type monitor
sudo ip link set wlan1 up

# Set capabilities (required for WiFi capture without root)
sudo setcap cap_net_raw,cap_net_admin+eip ~/sigint-pi/sigint-pi-bin

# Start service
systemctl --user start sigint-pi
```

## Channel Hopping Setup

WiFi can only capture packets on one channel at a time. Channel hopping rapidly cycles through all channels to capture traffic across the spectrum.

### 1. Create Sudoers Entry

The channel hopper needs passwordless sudo for `iw` commands:

```bash
sudo sh -c 'cat > /etc/sudoers.d/zzz-sigint-wifi << EOF
deck ALL=(ALL) NOPASSWD: /usr/bin/iw
Defaults:deck !requiretty
EOF
chmod 440 /etc/sudoers.d/zzz-sigint-wifi'
```

**Important:** File must be named `zzz-*` to be processed AFTER the `wheel` group entry.

### 2. Create Channel Hopper Script

```bash
cat > ~/sigint-pi/channel-hop.sh << 'EOF'
#!/bin/bash
IFACE=${1:-wlan1}
CHANNELS="1 2 3 4 5 6 7 8 9 10 11 36 40 44 48 149 153 157 161 165"

while true; do
    for ch in $CHANNELS; do
        sudo -n /usr/bin/iw dev $IFACE set channel $ch 2>/dev/null
        sleep 0.3
    done
done
EOF
chmod +x ~/sigint-pi/channel-hop.sh
```

### 3. Create Systemd Service

```bash
cat > ~/.config/systemd/user/channel-hop.service << 'EOF'
[Unit]
Description=WiFi Channel Hopper
After=sigint-pi.service

[Service]
Type=simple
ExecStart=/home/deck/sigint-pi/channel-hop.sh wlan1
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user enable --now channel-hop
```

### 4. Verify Channel Hopping

```bash
# Watch channels change
for i in {1..20}; do 
  iw dev wlan1 info | grep channel
  sleep 0.4
done
```

You should see channels cycling: 1, 2, 3... 11, 36, 40... 165, 1, 2...

## LLM/AI Configuration

SIGINT-Deck supports AI-powered device analysis via local LLM.

### Config File Settings

Add to `~/sigint-pi/config.toml`:

```toml
[llm]
enabled = true
provider = "llamacpp"
endpoint = "http://localhost:8080/v1"  # Your LLM server
model = "2slot-MM-local-gguf"               # Your model name
max_tokens = 200
timeout_secs = 60
```

### Test LLM Connection

```bash
curl http://localhost:8080/api/ai/status
```

### Web UI

The Settings tab in the dashboard provides:
- AI enable/disable toggle
- Provider selection
- Endpoint configuration
- Test connection button

## Device Learning & Anomaly Detection

SIGINT-Deck learns your environment over time, similar to Pwnagotchi.

### How It Works

1. **Training Period** (default: 1 hour)
   - Collects device statistics
   - Learns which devices are "normal"
   - No anomaly alerts during training

2. **After Training**
   - New devices flagged immediately
   - Known devices scored for anomalous behavior
   - Alerts on unusual patterns

### What's Learned Per Device

| Data | Timeframe | Used For |
|------|-----------|----------|
| Signal strength (RSSI) | Immediate | Distance estimation, anomaly scoring |
| Time-of-day patterns | Hours | Detecting unusual appearance times |
| Visit frequency | Days | Identifying stalking/following |
| Probe requests | Immediate | Device fingerprinting |
| Preferred channels | Hours | Behavioral profiling |

### Anomaly Scoring

Devices are scored 0.0 - 1.0:

- **< 0.5**: Normal
- **0.5 - 0.7**: Slightly unusual
- **> 0.7**: Anomalous (alert triggered)

Scoring factors:
- RSSI deviation from baseline (30%)
- Unusual time of appearance (30%)
- Behavioral changes (40%)

### Device Fingerprinting

Creates behavioral fingerprints to:
- Classify device type (Smartphone, Laptop, IoT, etc.)
- Detect MAC randomization (same device, different MAC)
- Calculate mobility score (stationary vs mobile)

### Configuration

```toml
[learning]
enabled = true
training_hours = 1              # Reduce for faster learning
anomaly_threshold = 0.7         # Lower = more sensitive
geofence_radius_meters = 100.0  # Resets learning when you move
```

### Speed Up Learning

For testing, use shorter training:
```toml
training_hours = 0.5  # 30 minutes
```

Or disable learning for immediate anomaly alerts:
```toml
enabled = false  # All devices treated as anomalies
```

## PCAP Capture

### Enable in Config

```toml
[wifi]
pcap_enabled = true
pcap_path = "/home/deck/sigint-pi/data/pcap"
pcap_rotate_mb = 100
```

### API Control

```bash
# Start capture
curl -X POST -H "Content-Type: application/json" -d '{}' http://localhost:8080/api/pcap/start

# Check status
curl http://localhost:8080/api/pcap/status

# Stop capture
curl -X POST -H "Content-Type: application/json" -d '{}' http://localhost:8080/api/pcap/stop

# List files
curl http://localhost:8080/api/pcap/files
```

### View Captured Files

```bash
ls -la ~/sigint-pi/data/pcap/
# Files named: capture_YYYYMMDD_HHMMSS.pcap
```

## Quick Reference

```bash
# Start everything (recommended)
sudo ~/sigint-pi/start-sigint.sh

# Stop everything
sudo ~/sigint-pi/stop-sigint.sh

# Check status
~/sigint-pi/status-sigint.sh

# View logs
journalctl --user -u sigint-pi -f

# Check hardware status
curl http://localhost:8080/api/hardware/status

# Check detected devices
curl http://localhost:8080/api/stats

# Check channel hopping
systemctl --user status channel-hop
```

## Technical Notes

### Why Capabilities Are Needed

WiFi packet capture requires raw socket access, which normally needs root. Instead of running the whole application as root, we use Linux capabilities:

```bash
setcap cap_net_raw,cap_net_admin+eip ~/sigint-pi/sigint-pi-bin
```

This grants only the specific permissions needed.

### Why libpcap Symlink Is Needed

The binary was compiled against `libpcap.so.0.8` but Steam Deck has `libpcap.so.1`. When capabilities are set on a binary, `LD_LIBRARY_PATH` is ignored for security reasons. The symlink provides compatibility:

```bash
ln -sf /usr/lib/libpcap.so.1 /usr/lib/libpcap.so.0.8
```

### Interface Naming with systemd .link Files

Modern systemd uses `.link` files for interface naming, which is more reliable than udev `NAME=` rules:

```
/etc/systemd/network/10-wlan0-internal.link  # Internal -> wlan0
/etc/systemd/network/10-wlan1-external.link  # External -> wlan1
```

These match by MAC address and assign fixed names.
