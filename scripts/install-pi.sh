#!/bin/bash
set -e

# SIGINT-Pi Raspberry Pi Installation Script
# Tested on: Raspberry Pi Zero 2 W, Pi 4, Pi 5 (Debian Bookworm ARM64)
# Usage: curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-pi/main/scripts/install-pi.sh | bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Pi Pi Installer${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

INSTALL_DIR="$HOME/sigint-pi"
BIN_DIR="$HOME/bin"

# Detect architecture
ARCH=$(uname -m)
if [[ "$ARCH" != "aarch64" && "$ARCH" != "armv7l" ]]; then
    echo -e "${RED}This script is for ARM-based Raspberry Pi (detected: $ARCH)${NC}"
    exit 1
fi

echo -e "${GREEN}Architecture: $ARCH${NC}"
echo -e "${GREEN}Install directory: $INSTALL_DIR${NC}"
echo ""

# ============================================
# Step 1: System Dependencies
# ============================================
echo -e "${YELLOW}[1/8] Installing system dependencies...${NC}"
sudo apt-get update -qq
sudo apt-get install -y -qq \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    bluez \
    bluetooth \
    gpsd \
    gpsd-clients \
    aircrack-ng \
    iw \
    wireless-tools \
    adb \
    python3 \
    python3-pip \
    curl \
    git \
    usbutils \
    autoconf \
    automake \
    libtool

# ============================================
# Step 2: RTL-SDR Tools
# ============================================
echo -e "${YELLOW}[2/8] Installing RTL-SDR tools...${NC}"
sudo apt-get install -y -qq \
    rtl-sdr \
    librtlsdr-dev \
    rtl-433 2>/dev/null || true

# Blacklist kernel DVB-T driver (conflicts with SDR)
if ! grep -q "blacklist dvb_usb_rtl28xxu" /etc/modprobe.d/blacklist-rtlsdr.conf 2>/dev/null; then
    echo -e "${CYAN}Blacklisting DVB-T kernel driver...${NC}"
    echo "blacklist dvb_usb_rtl28xxu" | sudo tee /etc/modprobe.d/blacklist-rtlsdr.conf
    echo "blacklist rtl2832" | sudo tee -a /etc/modprobe.d/blacklist-rtlsdr.conf
fi

# ============================================
# Step 3: HackRF Tools
# ============================================
echo -e "${YELLOW}[3/8] Installing HackRF tools...${NC}"
sudo apt-get install -y -qq \
    hackrf \
    libhackrf-dev 2>/dev/null || true

# ============================================
# Step 4: Kalibrate-RTL (Cell Tower Scanner)
# ============================================
echo -e "${YELLOW}[4/8] Installing kalibrate-rtl...${NC}"
if ! command -v kal &>/dev/null; then
    if [ -d /tmp/kalibrate-rtl ]; then rm -rf /tmp/kalibrate-rtl; fi
    cd /tmp
    git clone https://github.com/steve-m/kalibrate-rtl.git 2>/dev/null
    cd kalibrate-rtl
    ./bootstrap && ./configure && make -j$(nproc) && sudo make install
    cd ~
    rm -rf /tmp/kalibrate-rtl
    echo -e "${GREEN}kalibrate-rtl installed${NC}"
else
    echo -e "${GREEN}kalibrate-rtl already installed${NC}"
fi

# ============================================
# Step 5: SDR Udev Rules
# ============================================
echo -e "${YELLOW}[5/8] Setting up udev rules...${NC}"
sudo tee /etc/udev/rules.d/20-rtlsdr.rules > /dev/null << 'UDEV'
# RTL-SDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", GROUP="plugdev", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2832", GROUP="plugdev", MODE="0666"
# HackRF One
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", GROUP="plugdev", MODE="0666"
# LimeSDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="601f", GROUP="plugdev", MODE="0666"
UDEV
sudo udevadm control --reload-rules
sudo udevadm trigger

# Add user to all required groups for SDR device access
for grp in plugdev rtlsdr bluetooth dialout; do
    if getent group "$grp" > /dev/null 2>&1; then
        sudo usermod -aG "$grp" "$USER" 2>/dev/null || true
        echo -e "${GREEN}  Added $USER to group $grp${NC}"
    fi
done
echo -e "${YELLOW}  NOTE: Group changes take effect after logout/login or reboot${NC}"

# ============================================
# Step 6: Clone/Update Repository
# ============================================
echo -e "${YELLOW}[6/8] Setting up SIGINT-Pi...${NC}"
mkdir -p "$INSTALL_DIR"

if [ -d "$INSTALL_DIR/.git" ]; then
    echo -e "${CYAN}Updating existing installation...${NC}"
    cd "$INSTALL_DIR"
    git pull --ff-only 2>/dev/null || true
else
    echo -e "${CYAN}Cloning repository...${NC}"
    git clone https://github.com/naanprofit/sigint-pi.git "$INSTALL_DIR" 2>/dev/null || true
fi

# ============================================
# Step 7: Configuration
# ============================================
echo -e "${YELLOW}[7/8] Setting up configuration...${NC}"
if [ ! -f "$INSTALL_DIR/config.toml" ]; then
    cat > "$INSTALL_DIR/config.toml" << 'CONFIG'
[device]
name = "sigint-pi-01"
location_name = "default"

[web]
enabled = true
port = 8080
host = "0.0.0.0"

[wifi]
enabled = true
interface = "wlan1"
channels_2ghz = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
channels_5ghz = [36, 40, 44, 48, 149, 153, 157, 161, 165]

[bluetooth]
enabled = true
detect_trackers = true

[gps]
enabled = false
gpsd_host = "localhost"
gpsd_port = 2947

[rayhunter]
enabled = true
api_url = "http://localhost:8081"
poll_interval_secs = 5
alert_on_suspicious = true

[alerts]
[alerts.sound]
enabled = true
ninja_mode = false
volume = 80

[database]
path = "sigint.db"
CONFIG
    echo -e "${GREEN}Config created at $INSTALL_DIR/config.toml${NC}"
else
    echo -e "${GREEN}Config already exists${NC}"
fi

# ============================================
# Step 8: Systemd Service
# ============================================
echo -e "${YELLOW}[8/8] Setting up systemd service...${NC}"
sudo tee /etc/systemd/system/sigint-pi.service > /dev/null << EOF
[Unit]
Description=SIGINT-Pi Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$INSTALL_DIR
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
Environment=HOME=$HOME
ExecStart=$INSTALL_DIR/sigint-pi
Restart=always
RestartSec=5
AmbientCapabilities=CAP_NET_RAW CAP_NET_ADMIN

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable sigint-pi

# ============================================
# Setup ADB for RayHunter
# ============================================
echo -e "${YELLOW}Setting up ADB for RayHunter...${NC}"
# Ensure ADB server starts at boot
mkdir -p ~/.config/systemd/user
cat > ~/.config/systemd/user/adb-forward.service << 'ADB'
[Unit]
Description=ADB Port Forward for RayHunter
After=multi-user.target

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/bin/bash -c 'sleep 5 && /usr/bin/adb start-server && /usr/bin/adb forward tcp:8081 tcp:8080'
ExecStop=/usr/bin/adb kill-server

[Install]
WantedBy=default.target
ADB
systemctl --user daemon-reload
systemctl --user enable adb-forward.service

# ============================================
# Summary
# ============================================
echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Installation Complete!${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "Install directory: ${CYAN}$INSTALL_DIR${NC}"
echo -e "Config file:       ${CYAN}$INSTALL_DIR/config.toml${NC}"
echo -e "Web UI:            ${CYAN}http://$(hostname -I | awk '{print $1}'):8080${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Download or build the binary (see docs/BUILD.md)"
echo "  2. Place it at $INSTALL_DIR/sigint-pi"
echo "  3. Set capabilities: sudo setcap cap_net_raw,cap_net_admin+eip $INSTALL_DIR/sigint-pi"
echo "  4. Connect USB WiFi adapter to wlan1"
echo "  5. Start: sudo systemctl start sigint-pi"
echo ""
echo -e "${YELLOW}Optional hardware:${NC}"
echo "  - RTL-SDR USB dongle for spectrum analysis"
echo "  - HackRF One for wideband scanning"
echo "  - USB GPS (VK-172) for location tracking"
echo "  - Orbic RC400L with EFF RayHunter for IMSI catcher detection"
echo ""
echo -e "${YELLOW}RayHunter setup:${NC}"
echo "  1. Connect Orbic phone via USB data cable"
echo "  2. Run: adb devices (should show device)"
echo "  3. Run: adb forward tcp:8081 tcp:8080"
echo "  4. Verify: curl http://localhost:8081/api/system-stats"
echo ""
