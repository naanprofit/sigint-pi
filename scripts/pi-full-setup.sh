#!/bin/bash
# SIGINT-Pi Full Setup Script
# Installs all dependencies for WiFi, BLE, GPS, SDR, and RayHunter
#
# Supported hardware:
# - WiFi: Alfa AWUS036ACHM, RTL8812AU adapters (monitor mode)
# - GPS: U-blox U7 (VK-172, G-Mouse)
# - SDR: RTL-SDR, HackRF One, LimeSDR
# - BLE: Built-in Pi Bluetooth
# - RayHunter: Android phone via ADB
#
# Usage: curl -sSL https://raw.githubusercontent.com/.../pi-full-setup.sh | sudo bash
#    or: sudo ./pi-full-setup.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_section() { echo -e "\n${BLUE}=== $1 ===${NC}"; }

INSTALL_DIR="${INSTALL_DIR:-/opt/sigint-pi}"
CONFIG_DIR="/etc/sigint-pi"
DATA_DIR="/var/lib/sigint-pi"

# Check root
if [ "$EUID" -ne 0 ]; then
    log_error "Please run as root: sudo $0"
    exit 1
fi

# Check Pi
if ! grep -q "Raspberry Pi" /proc/cpuinfo 2>/dev/null; then
    log_warn "This doesn't appear to be a Raspberry Pi. Continuing anyway..."
fi

log_section "SIGINT-Pi Full Setup"
echo "This script will install all dependencies for:"
echo "  - WiFi monitor mode (aircrack-ng, iw)"
echo "  - Bluetooth/BLE scanning (bluez)"
echo "  - GPS support (gpsd) for U-blox U7"
echo "  - SDR tools (rtl-sdr, rtl_433, hackrf)"
echo "  - RayHunter support (adb)"
echo ""

# ============================================
# System Update
# ============================================
log_section "Updating System"
apt-get update
apt-get upgrade -y

# ============================================
# Core Dependencies
# ============================================
log_section "Installing Core Dependencies"
apt-get install -y \
    build-essential \
    git \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    libpcap-dev \
    libsqlite3-dev \
    sqlite3 \
    cmake

# ============================================
# Wireless Tools (WiFi Monitor Mode)
# ============================================
log_section "Installing WiFi Tools"
apt-get install -y \
    aircrack-ng \
    wireless-tools \
    iw \
    wpasupplicant \
    net-tools \
    tcpdump \
    arp-scan

# RTL8812AU driver for Alfa adapters (if kernel headers available)
if [ -d "/lib/modules/$(uname -r)/build" ]; then
    log_info "Installing RTL8812AU driver..."
    if [ ! -d "/tmp/rtl8812au" ]; then
        git clone https://github.com/aircrack-ng/rtl8812au.git /tmp/rtl8812au
    fi
    cd /tmp/rtl8812au
    make && make install || log_warn "RTL8812AU driver installation failed (may not be needed)"
    cd -
else
    log_warn "Kernel headers not found, skipping RTL8812AU driver"
fi

# ============================================
# Bluetooth/BLE
# ============================================
log_section "Installing Bluetooth Tools"
apt-get install -y \
    bluez \
    bluetooth \
    libbluetooth-dev \
    pi-bluetooth

# Enable Bluetooth
systemctl enable bluetooth
systemctl start bluetooth || true

# ============================================
# GPS (gpsd for U-blox U7)
# ============================================
log_section "Installing GPS Support"
apt-get install -y \
    gpsd \
    gpsd-clients

# GPS udev rules for U-blox devices
cat > /etc/udev/rules.d/99-gps.rules << 'EOF'
# U-blox GPS modules (VK-172, G-Mouse, etc.)
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a7", SYMLINK+="gps0", MODE="0666", GROUP="dialout"
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a8", SYMLINK+="gps0", MODE="0666", GROUP="dialout"
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a6", SYMLINK+="gps0", MODE="0666", GROUP="dialout"
# Prolific USB-Serial
SUBSYSTEM=="tty", ATTRS{idVendor}=="067b", ATTRS{idProduct}=="2303", SYMLINK+="gps0", MODE="0666", GROUP="dialout"
# FTDI USB-Serial
SUBSYSTEM=="tty", ATTRS{idVendor}=="0403", MODE="0666", GROUP="dialout"
EOF

# Configure gpsd
cat > /etc/default/gpsd << 'EOF'
START_DAEMON="true"
GPSD_OPTIONS="-n -G"
DEVICES="/dev/ttyACM0 /dev/ttyUSB0 /dev/gps0"
USBAUTO="true"
GPSD_SOCKET="/var/run/gpsd.sock"
EOF

# gpsd systemd override for reliability
mkdir -p /etc/systemd/system/gpsd.service.d
cat > /etc/systemd/system/gpsd.service.d/override.conf << 'EOF'
[Unit]
After=systemd-udevd.service
Wants=systemd-udevd.service

[Service]
Restart=always
RestartSec=5
ExecStart=
ExecStart=/usr/sbin/gpsd -N -G -n /dev/ttyACM0 /dev/ttyUSB0 /dev/gps0
EOF

systemctl daemon-reload
systemctl enable gpsd
systemctl restart gpsd || true

# ============================================
# SDR Tools
# ============================================
log_section "Installing SDR Tools"

# RTL-SDR
apt-get install -y \
    rtl-sdr \
    librtlsdr-dev \
    librtlsdr0

# rtl_433 (ISM band decoder)
if ! command -v rtl_433 &> /dev/null; then
    log_info "Installing rtl_433..."
    apt-get install -y rtl-433 || {
        # Build from source if package not available
        log_info "Building rtl_433 from source..."
        git clone https://github.com/merbanan/rtl_433.git /tmp/rtl_433
        cd /tmp/rtl_433
        mkdir build && cd build
        cmake ..
        make -j$(nproc)
        make install
        cd -
    }
fi

# kalibrate-rtl (cellular tower scanner)
if ! command -v kal &> /dev/null && ! command -v kalibrate-rtl &> /dev/null; then
    log_info "Installing kalibrate-rtl..."
    apt-get install -y kalibrate-rtl || {
        git clone https://github.com/steve-m/kalibrate-rtl.git /tmp/kalibrate-rtl
        cd /tmp/kalibrate-rtl
        ./bootstrap && ./configure && make -j$(nproc) && make install
        cd -
    }
fi

# HackRF (optional - takes a while to compile)
if ! command -v hackrf_info &> /dev/null; then
    log_info "Installing HackRF tools..."
    apt-get install -y hackrf libhackrf-dev || {
        log_info "Building HackRF from source..."
        apt-get install -y libfftw3-dev
        git clone https://github.com/greatscottgadgets/hackrf.git /tmp/hackrf
        cd /tmp/hackrf/host
        mkdir build && cd build
        cmake ..
        make -j$(nproc)
        make install
        ldconfig
        cd -
    }
fi

# SDR udev rules
cat > /etc/udev/rules.d/99-sdr.rules << 'EOF'
# RTL-SDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2832", MODE="0666", GROUP="plugdev"

# HackRF One
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="604b", MODE="0666", GROUP="plugdev"

# LimeSDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6108", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="601f", MODE="0666", GROUP="plugdev"
EOF

# Blacklist DVB-T drivers that interfere with RTL-SDR
cat > /etc/modprobe.d/rtlsdr-blacklist.conf << 'EOF'
blacklist dvb_usb_rtl28xxu
blacklist rtl2832
blacklist rtl2830
EOF

# ============================================
# RayHunter Support (ADB)
# ============================================
log_section "Installing RayHunter/ADB Support"
apt-get install -y adb

# ADB udev rules
cat >> /etc/udev/rules.d/99-android.rules << 'EOF'
# Android ADB
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="22b8", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="0bb4", MODE="0666", GROUP="plugdev"
EOF

# RayHunter helper script
cat > /usr/local/bin/rayhunter-connect.sh << 'EOF'
#!/bin/bash
# Connect to RayHunter on Android phone via ADB
# Usage: rayhunter-connect.sh [phone_ip]

PHONE_IP="${1:-}"

# Try USB first
if adb devices | grep -q "device$"; then
    echo "Connected via USB"
    adb forward tcp:8080 tcp:8080
    echo "RayHunter available at http://localhost:8080"
    exit 0
fi

# Try WiFi if IP provided
if [ -n "$PHONE_IP" ]; then
    adb connect "$PHONE_IP:5555"
    adb forward tcp:8080 tcp:8080
    echo "RayHunter available at http://localhost:8080"
    exit 0
fi

echo "No Android device found. Connect via USB or provide IP address."
exit 1
EOF
chmod +x /usr/local/bin/rayhunter-connect.sh

# ============================================
# Reload udev
# ============================================
udevadm control --reload-rules
udevadm trigger

# ============================================
# Create directories
# ============================================
log_section "Creating Directories"
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/pcap"
mkdir -p /var/log/sigint-pi

# ============================================
# Pi Optimizations
# ============================================
log_section "Applying Pi Optimizations"

# Reduce GPU memory (headless)
if ! grep -q "gpu_mem=16" /boot/config.txt 2>/dev/null && ! grep -q "gpu_mem=16" /boot/firmware/config.txt 2>/dev/null; then
    BOOT_CONFIG=""
    [ -f /boot/firmware/config.txt ] && BOOT_CONFIG="/boot/firmware/config.txt"
    [ -f /boot/config.txt ] && BOOT_CONFIG="/boot/config.txt"
    if [ -n "$BOOT_CONFIG" ]; then
        echo "gpu_mem=16" >> "$BOOT_CONFIG"
        log_info "Set GPU memory to 16MB"
    fi
fi

# Increase swap for compilation
if [ ! -f /swapfile ] || [ "$(stat -c%s /swapfile 2>/dev/null)" -lt 2000000000 ]; then
    log_info "Creating 2GB swap file..."
    swapoff /swapfile 2>/dev/null || true
    dd if=/dev/zero of=/swapfile bs=1M count=2048
    chmod 600 /swapfile
    mkswap /swapfile
    swapon /swapfile
    grep -q "/swapfile" /etc/fstab || echo "/swapfile none swap sw 0 0" >> /etc/fstab
fi

# ============================================
# Systemd Service
# ============================================
log_section "Creating Systemd Service"
cat > /etc/systemd/system/sigint-pi.service << EOF
[Unit]
Description=SIGINT-Pi Wireless Intelligence Platform
After=network.target gpsd.service bluetooth.service
Wants=gpsd.service bluetooth.service

[Service]
Type=simple
ExecStart=$INSTALL_DIR/sigint-deck
WorkingDirectory=$INSTALL_DIR
Restart=always
RestartSec=10
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload

# ============================================
# Summary
# ============================================
log_section "Setup Complete!"

echo ""
echo "Installed components:"
echo "  ✓ WiFi tools (aircrack-ng, iw)"
echo "  ✓ Bluetooth (bluez)"
echo "  ✓ GPS (gpsd)"
echo "  ✓ RTL-SDR (rtl_sdr, rtl_433)"
command -v hackrf_info &>/dev/null && echo "  ✓ HackRF"
command -v kal &>/dev/null && echo "  ✓ kalibrate-rtl"
echo "  ✓ ADB (RayHunter support)"
echo ""
echo "Directories:"
echo "  Install: $INSTALL_DIR"
echo "  Config:  $CONFIG_DIR"
echo "  Data:    $DATA_DIR"
echo ""
echo "Next steps:"
echo "  1. Copy sigint-deck binary to $INSTALL_DIR/"
echo "  2. Copy static/ and data/ directories"
echo "  3. Edit $CONFIG_DIR/config.toml"
echo "  4. Enable service: sudo systemctl enable sigint-pi"
echo "  5. Start service: sudo systemctl start sigint-pi"
echo ""
echo "Hardware check:"
echo "  - WiFi:  iw dev"
echo "  - BLE:   hciconfig -a"
echo "  - GPS:   gpspipe -w -n 3"
echo "  - SDR:   rtl_test -t"
echo ""
log_info "Reboot recommended: sudo reboot"
