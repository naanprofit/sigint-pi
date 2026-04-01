# SIGINT-Pi Hardware & Software Requirements

## Supported Hardware

### Required
- **Raspberry Pi Zero 2 W** (or Pi 3/4/5)
- **MicroSD Card** (16GB+ recommended)
- **Power Supply** (5V 2.5A minimum)

### Wireless Monitoring
| Device | Purpose | Interface | Notes |
|--------|---------|-----------|-------|
| **External WiFi Adapter** | Monitor mode scanning | wlan1 | Must support monitor mode (e.g., Alfa AWUS036ACHM, RTL8812AU) |
| **Internal WiFi** | Network connectivity | wlan0 | Pi's built-in WiFi for internet access |
| **Bluetooth** | BLE scanning, AirTag detection | hci0 | Pi's built-in Bluetooth |

### GPS
| Device | Purpose | Interface | Notes |
|--------|---------|-----------|-------|
| **U-blox U7 GPS** | Location tracking | /dev/ttyACM0 | VK-172, G-Mouse, or similar USB GPS |

### SDR (Software Defined Radio)
| Device | Purpose | Frequency Range | Notes |
|--------|---------|-----------------|-------|
| **RTL-SDR** | ISM band monitoring | 24-1766 MHz | rtl_433 compatible, budget option |
| **HackRF One** | Wideband spectrum | 1 MHz - 6 GHz | Drone detection, spectrum analysis |
| **LimeSDR** | Full-duplex SDR | 100 kHz - 3.8 GHz | Advanced applications |

### IMSI Catcher Detection
| Device | Purpose | Notes |
|--------|---------|-------|
| **Android Phone** | RayHunter host | Running EFF RayHunter app |
| **USB Cable** | ADB connection | Connect phone to Pi |

## Software Dependencies

### System Packages (APT)
```bash
# Core
build-essential git curl wget

# Wireless
aircrack-ng wireless-tools iw wpasupplicant

# Bluetooth
bluez bluetooth libbluetooth-dev

# GPS
gpsd gpsd-clients

# SDR - RTL-SDR
rtl-sdr librtlsdr-dev rtl-433

# SDR - HackRF (optional, compile from source on Pi)
# hackrf libhackrf-dev

# Network tools
tcpdump wireshark-common net-tools arp-scan

# Development
pkg-config libssl-dev libdbus-1-dev libpcap-dev libsqlite3-dev

# ADB for RayHunter
adb
```

### Python Packages (for Whisper STT)
```bash
pip3 install openai-whisper torch torchaudio
```

### Firmware
- RTL8812AU driver (for Alfa adapters)
- Bluetooth firmware (usually pre-installed)

## USB Device Summary

| Port | Device | Purpose |
|------|--------|---------|
| USB1 | WiFi Adapter | Monitor mode (wlan1) |
| USB2 | GPS (U-blox U7) | Location (/dev/ttyACM0) |
| USB3 | SDR (RTL-SDR/HackRF) | RF monitoring |
| USB4 | Android Phone | RayHunter via ADB |

**Note:** Pi Zero 2 W has only one USB port. Use a powered USB hub!

## Network Ports

| Port | Service | Protocol |
|------|---------|----------|
| 8080 | Web UI | HTTP |
| 2947 | gpsd | TCP |
| 1883 | MQTT (optional) | TCP |

## Configuration Files

| File | Purpose |
|------|---------|
| `/etc/sigint-pi/config.toml` | Main configuration |
| `/etc/default/gpsd` | GPS daemon config |
| `/etc/udev/rules.d/99-gps.rules` | GPS USB rules |
| `/etc/udev/rules.d/99-sdr.rules` | SDR USB rules |
| `/etc/systemd/system/sigint-pi.service` | Systemd service |

## Troubleshooting

### Bluetooth Not Working
Bluetooth may be soft-blocked by rfkill on fresh Pi installs:
```bash
# Check rfkill status
rfkill list

# Unblock Bluetooth
sudo rfkill unblock bluetooth

# Bring up interface
sudo hciconfig hci0 up

# Verify
hciconfig -a  # Should show "UP RUNNING"
```

### No External WiFi Adapter (wlan1)
The scanner expects an external USB WiFi adapter for monitor mode:
- Built-in wlan0 is used for network connectivity
- External adapter appears as wlan1
- Without wlan1, you'll see "Channel hop failed: No such device"

### GPS Retry Spam
If GPS is not connected, ensure it's disabled in config:
```toml
[gps]
enabled = false
```

## Quick Hardware Check

```bash
# WiFi adapters
iw dev

# Bluetooth (must be UP RUNNING)
hciconfig -a

# GPS
ls -la /dev/ttyACM* /dev/ttyUSB*
gpspipe -w -n 3

# SDR
rtl_test -t
hackrf_info
LimeUtil --find

# USB devices
lsusb
```
