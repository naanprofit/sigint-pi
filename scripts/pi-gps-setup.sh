#!/bin/bash
# SIGINT-Pi GPS Setup Script
# Configures gpsd for U-blox U7 GPS USB module (VK-172 and similar)
#
# The U7 GPS module appears as /dev/ttyACM0 (CDC ACM device)
# Common U7-based modules: VK-172, G-Mouse, generic U-blox 7 USB
#
# Usage: sudo ./pi-gps-setup.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

if [ "$EUID" -ne 0 ]; then
    log_error "Please run as root (sudo)"
    exit 1
fi

log_info "Setting up GPS for SIGINT-Pi..."

# Install gpsd if not present
if ! command -v gpsd &> /dev/null; then
    log_info "Installing gpsd..."
    apt-get update
    apt-get install -y gpsd gpsd-clients
fi

# Detect GPS device
GPS_DEVICE=""
if [ -e /dev/ttyACM0 ]; then
    GPS_DEVICE="/dev/ttyACM0"
    log_info "Found GPS at /dev/ttyACM0 (U-blox USB CDC)"
elif [ -e /dev/ttyUSB0 ]; then
    GPS_DEVICE="/dev/ttyUSB0"
    log_info "Found GPS at /dev/ttyUSB0 (USB Serial)"
elif [ -e /dev/serial0 ]; then
    GPS_DEVICE="/dev/serial0"
    log_info "Found GPS at /dev/serial0 (GPIO UART)"
else
    log_warn "No GPS device detected. Will configure for auto-detection."
    GPS_DEVICE="/dev/ttyACM0"
fi

# Create udev rules for U-blox GPS modules
log_info "Creating udev rules for U-blox GPS..."
cat > /etc/udev/rules.d/99-gps.rules << 'EOF'
# U-blox GPS modules (VK-172, G-Mouse, etc.)
# U-blox 7 (1546:01a7)
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a7", SYMLINK+="gps0", MODE="0666", GROUP="dialout"

# U-blox 8 (1546:01a8) 
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a8", SYMLINK+="gps0", MODE="0666", GROUP="dialout"

# U-blox generic (1546:01a6)
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a6", SYMLINK+="gps0", MODE="0666", GROUP="dialout"

# Prolific USB-Serial (common GPS adapter)
SUBSYSTEM=="tty", ATTRS{idVendor}=="067b", ATTRS{idProduct}=="2303", SYMLINK+="gps0", MODE="0666", GROUP="dialout"

# FTDI USB-Serial (another common GPS adapter)
SUBSYSTEM=="tty", ATTRS{idVendor}=="0403", MODE="0666", GROUP="dialout"

# SiRF GPS
SUBSYSTEM=="tty", ATTRS{idVendor}=="0525", SYMLINK+="gps0", MODE="0666", GROUP="dialout"
EOF

udevadm control --reload-rules
udevadm trigger

# Configure gpsd
log_info "Configuring gpsd..."
cat > /etc/default/gpsd << EOF
# GPS daemon configuration for SIGINT-Pi
# Auto-configured for U-blox U7 GPS module

START_DAEMON="true"
GPSD_OPTIONS="-n -G"
DEVICES="${GPS_DEVICE}"
USBAUTO="true"
GPSD_SOCKET="/var/run/gpsd.sock"
EOF

# Create systemd override for gpsd to handle USB hotplug
mkdir -p /etc/systemd/system/gpsd.service.d
cat > /etc/systemd/system/gpsd.service.d/override.conf << 'EOF'
[Unit]
# Ensure gpsd starts after USB subsystem is ready
After=systemd-udevd.service
Wants=systemd-udevd.service

[Service]
# Restart if GPS is unplugged/replugged
Restart=always
RestartSec=5

# Allow binding to all interfaces for network access
ExecStart=
ExecStart=/usr/sbin/gpsd -N -G -n /dev/ttyACM0 /dev/ttyUSB0 /dev/gps0
EOF

# Reload systemd and restart gpsd
systemctl daemon-reload
systemctl enable gpsd
systemctl restart gpsd || true

# Wait for gpsd to start
sleep 2

# Test GPS
log_info "Testing GPS connection..."
if gpspipe -w -n 5 2>/dev/null | grep -q "TPV\|SKY"; then
    log_info "GPS is working! Receiving data."
else
    log_warn "GPS not receiving data yet. This is normal if:"
    log_warn "  - GPS module needs time to acquire satellites (can take 1-5 minutes outdoors)"
    log_warn "  - GPS module is indoors (needs clear sky view)"
    log_warn "  - GPS device is not plugged in"
fi

# Show GPS device info
log_info "GPS device information:"
if [ -e /dev/ttyACM0 ]; then
    echo "  /dev/ttyACM0: $(udevadm info -q property -n /dev/ttyACM0 2>/dev/null | grep -E 'ID_VENDOR=|ID_MODEL=' | tr '\n' ' ')"
fi
if [ -e /dev/ttyUSB0 ]; then
    echo "  /dev/ttyUSB0: $(udevadm info -q property -n /dev/ttyUSB0 2>/dev/null | grep -E 'ID_VENDOR=|ID_MODEL=' | tr '\n' ' ')"
fi

# Show gpsd status
log_info "gpsd service status:"
systemctl status gpsd --no-pager -l | head -10

echo ""
log_info "GPS setup complete!"
echo ""
echo "Useful commands:"
echo "  gpsmon              - Interactive GPS monitor"
echo "  cgps -s             - GPS client with satellite view"
echo "  gpspipe -w          - Raw GPS JSON stream"
echo "  systemctl status gpsd - Check gpsd service"
echo ""
echo "If GPS isn't working:"
echo "  1. Ensure GPS module is plugged in"
echo "  2. Take device outdoors with clear sky view"
echo "  3. Wait 1-5 minutes for satellite acquisition"
echo "  4. Check: ls -la /dev/ttyACM* /dev/ttyUSB* /dev/gps*"
