# SIGINT-Pi Quick Start Guide

## Prerequisites
- Raspberry Pi Zero 2 W (or Pi 3/4/5) with Raspberry Pi OS (Bookworm)
- SSH access to the Pi
- Internet connection

## 1. Initial Setup

```bash
# SSH to your Pi
ssh user@<device-ip>

# Create directory
mkdir -p ~/sigint-pi
cd ~/sigint-pi
```

## 2. Run Full Setup Script

```bash
# Download and run setup script
curl -sSL https://raw.githubusercontent.com/<repo>/scripts/pi-full-setup.sh | sudo bash

# Or if you have the script locally:
chmod +x pi-full-setup.sh
sudo ./pi-full-setup.sh
```

This installs:
- WiFi tools (aircrack-ng, iw)
- Bluetooth (bluez)
- GPS (gpsd)
- SDR tools (rtl-sdr, rtl_433, kalibrate-rtl, hackrf)
- ADB (for RayHunter)

## 3. Fix Bluetooth (Required on Fresh Install)

```bash
# Unblock Bluetooth
sudo rfkill unblock bluetooth

# Bring up interface
sudo hciconfig hci0 up

# Verify (should show "UP RUNNING")
hciconfig -a
```

## 4. Deploy Binary

Copy the pre-built binary from your development machine:
```bash
# From your dev machine
scp sigint-deck user@<device-ip>:~/sigint-pi/
scp -r static/ user@<device-ip>:~/sigint-pi/
scp -r data/ user@<device-ip>:~/sigint-pi/
scp config.toml.example user@<device-ip>:~/sigint-pi/config.toml
```

## 5. Configure

Edit `~/sigint-pi/config.toml`:

```toml
[device]
name = "sigint-pi-01"
location_name = "home"

[wifi]
enabled = true
interface = "wlan1"  # External USB adapter

[bluetooth]
enabled = true       # Must be true for BLE scanning

[gps]
enabled = false      # Set true only if GPS connected

[web]
enabled = true       # Required for web UI
bind_address = "0.0.0.0"
port = 8080
```

## 6. Run

```bash
cd ~/sigint-pi
chmod +x sigint-deck
sudo SIGINT_ACCEPT_DISCLAIMER=1 RUST_LOG=info ./sigint-deck
```

## 7. Access Web UI

Open in browser: `http://<pi-ip>:8080`

## 8. Run as Service (Optional)

```bash
# Create systemd service
sudo tee /etc/systemd/system/sigint-pi.service << 'EOF'
[Unit]
Description=SIGINT-Pi
After=network.target bluetooth.service

[Service]
Type=simple
WorkingDirectory=/home/pi/sigint-pi
ExecStart=/home/pi/sigint-pi/sigint-deck
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable sigint-pi
sudo systemctl start sigint-pi

# Check status
sudo systemctl status sigint-pi
```

## Hardware Checklist

| Hardware | Required | Status Check |
|----------|----------|--------------|
| Pi WiFi (wlan0) | Yes | `ip link show wlan0` |
| USB WiFi (wlan1) | For monitor mode | `iw dev` |
| Bluetooth | For BLE/AirTags | `hciconfig -a` (UP RUNNING) |
| USB GPS | For location | `ls /dev/ttyACM0` |
| RTL-SDR | For RF monitoring | `rtl_test -t` |

## Common Issues

### "Channel hop failed: No such device"
- External WiFi adapter (wlan1) not connected
- This is expected without USB WiFi adapter
- BLE scanning still works

### "BLE: ready" but no devices
- Run `sudo rfkill unblock bluetooth`
- Run `sudo hciconfig hci0 up`
- Restart sigint-deck

### GPS retry spam in logs
- Set `[gps] enabled = false` in config.toml

### Web UI not accessible
- Ensure `[web] enabled = true` in config.toml
- Check firewall: `sudo ufw allow 8080/tcp`
