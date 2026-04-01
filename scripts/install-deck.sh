#!/bin/bash
set -e

# SIGINT-Deck Steam Deck Installation Script
# Tested on: Steam Deck LCD/OLED (SteamOS 3.x)
# Usage: curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-deck/main/scripts/install-deck.sh | bash

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Deck Steam Deck Installer${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

INSTALL_DIR="$HOME/sigint-deck"
BIN_DIR="$HOME/bin"

# Detect platform
if [[ "$(uname -m)" != "x86_64" ]]; then
    echo -e "${RED}This script is for x86_64 (Steam Deck). For Pi, use install-pi.sh${NC}"
    exit 1
fi

echo -e "${GREEN}Platform: $(uname -m)${NC}"
echo -e "${GREEN}Install directory: $INSTALL_DIR${NC}"
echo ""

# ============================================
# Step 1: Unlock Filesystem (SteamOS)
# ============================================
echo -e "${YELLOW}[1/8] Checking filesystem...${NC}"
if command -v steamos-readonly &>/dev/null; then
    echo -e "${CYAN}SteamOS detected - disabling read-only mode temporarily...${NC}"
    sudo steamos-readonly disable 2>/dev/null || true
fi

# ============================================
# Step 2: System Dependencies via pacman
# ============================================
echo -e "${YELLOW}[2/8] Installing system dependencies...${NC}"
# SteamOS uses pacman (Arch-based)
if command -v pacman &>/dev/null; then
    sudo pacman -Sy --noconfirm --needed \
        base-devel \
        openssl \
        pkg-config \
        bluez \
        bluez-utils \
        wireless_tools \
        iw \
        python \
        curl \
        git \
        usbutils 2>/dev/null || true
fi

# ============================================
# Step 3: SDR Tools (from prebuilt or source)
# ============================================
echo -e "${YELLOW}[3/8] Installing SDR tools...${NC}"
mkdir -p "$BIN_DIR"

# RTL-SDR
if ! command -v rtl_sdr &>/dev/null && ! [ -f "$BIN_DIR/rtl_sdr" ]; then
    echo -e "${CYAN}Installing RTL-SDR...${NC}"
    sudo pacman -S --noconfirm rtl-sdr 2>/dev/null || {
        # Build from source
        cd /tmp
        rm -rf rtl-sdr-build
        git clone https://github.com/steve-m/librtlsdr.git rtl-sdr-build 2>/dev/null
        cd rtl-sdr-build
        mkdir build && cd build
        cmake .. -DCMAKE_INSTALL_PREFIX=$HOME/.local
        make -j$(nproc) && make install
        ln -sf $HOME/.local/bin/rtl_* $BIN_DIR/
        cd ~ && rm -rf /tmp/rtl-sdr-build
    }
    echo -e "${GREEN}RTL-SDR installed${NC}"
else
    echo -e "${GREEN}RTL-SDR already installed${NC}"
fi

# RTL_433
if ! command -v rtl_433 &>/dev/null && ! [ -f "$BIN_DIR/rtl_433" ]; then
    echo -e "${CYAN}Installing rtl_433...${NC}"
    cd /tmp
    rm -rf rtl_433-build
    git clone https://github.com/merbanan/rtl_433.git rtl_433-build 2>/dev/null
    cd rtl_433-build
    mkdir build && cd build
    cmake .. -DCMAKE_INSTALL_PREFIX=$HOME/.local
    make -j$(nproc) && make install
    ln -sf $HOME/.local/bin/rtl_433 $BIN_DIR/
    cd ~ && rm -rf /tmp/rtl_433-build
    echo -e "${GREEN}rtl_433 installed${NC}"
else
    echo -e "${GREEN}rtl_433 already installed${NC}"
fi

# HackRF (if present)
if ! command -v hackrf_info &>/dev/null && ! [ -f "$BIN_DIR/hackrf_info" ]; then
    echo -e "${CYAN}Installing HackRF tools...${NC}"
    sudo pacman -S --noconfirm hackrf 2>/dev/null || {
        cd /tmp
        rm -rf hackrf-build
        git clone https://github.com/greatscottgadgets/hackrf.git hackrf-build 2>/dev/null
        cd hackrf-build/host
        mkdir build && cd build
        cmake .. -DCMAKE_INSTALL_PREFIX=$HOME/.local
        make -j$(nproc) && make install
        ln -sf $HOME/.local/bin/hackrf_* $BIN_DIR/
        cd ~ && rm -rf /tmp/hackrf-build
    }
    echo -e "${GREEN}HackRF tools installed${NC}"
else
    echo -e "${GREEN}HackRF tools already installed${NC}"
fi

# Kalibrate-RTL
if ! command -v kal &>/dev/null && ! [ -f "$BIN_DIR/kal" ]; then
    echo -e "${CYAN}Installing kalibrate-rtl...${NC}"
    cd /tmp
    rm -rf kalibrate-rtl
    git clone https://github.com/steve-m/kalibrate-rtl.git 2>/dev/null
    cd kalibrate-rtl
    ./bootstrap && ./configure --prefix=$HOME/.local && make -j$(nproc) && make install
    ln -sf $HOME/.local/bin/kal $BIN_DIR/
    cd ~ && rm -rf /tmp/kalibrate-rtl
    echo -e "${GREEN}kalibrate-rtl installed${NC}"
else
    echo -e "${GREEN}kalibrate-rtl already installed${NC}"
fi

# ============================================
# Step 4: Blacklist DVB-T Driver
# ============================================
echo -e "${YELLOW}[4/8] Configuring SDR drivers...${NC}"
if ! grep -q "blacklist dvb_usb_rtl28xxu" /etc/modprobe.d/blacklist-rtlsdr.conf 2>/dev/null; then
    echo "blacklist dvb_usb_rtl28xxu" | sudo tee /etc/modprobe.d/blacklist-rtlsdr.conf
    echo "blacklist rtl2832" | sudo tee -a /etc/modprobe.d/blacklist-rtlsdr.conf
fi

# SDR udev rules - use MODE=0666 so any user can access, plus group for system rules
sudo tee /etc/udev/rules.d/20-rtlsdr.rules > /dev/null << 'UDEV'
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2832", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="601f", MODE="0666"
UDEV
sudo udevadm control --reload-rules 2>/dev/null || true
sudo udevadm trigger 2>/dev/null || true

# Add user to SDR device groups (rtlsdr on SteamOS, plugdev on Debian)
for grp in rtlsdr plugdev dialout; do
    if getent group "$grp" > /dev/null 2>&1; then
        sudo usermod -aG "$grp" "$USER" 2>/dev/null || true
        echo -e "${GREEN}  Added $USER to group $grp${NC}"
    fi
done
echo -e "${YELLOW}  NOTE: Group changes take effect after logout/login or reboot${NC}"

# ============================================
# Step 5: Install Rust
# ============================================
echo -e "${YELLOW}[5/8] Setting up Rust toolchain...${NC}"
if ! command -v rustc &>/dev/null && ! [ -f "$HOME/.cargo/bin/rustc" ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}Rust installed${NC}"
else
    echo -e "${GREEN}Rust already installed${NC}"
fi
export PATH="$HOME/.cargo/bin:$BIN_DIR:$PATH"

# ============================================
# Step 6: Clone and Build
# ============================================
echo -e "${YELLOW}[6/8] Building SIGINT-Deck...${NC}"
if [ -d "$INSTALL_DIR/.git" ]; then
    echo -e "${CYAN}Updating existing installation...${NC}"
    cd "$INSTALL_DIR"
    git pull --ff-only 2>/dev/null || true
else
    echo -e "${CYAN}Cloning repository...${NC}"
    git clone https://github.com/naanprofit/sigint-deck.git "$INSTALL_DIR" 2>/dev/null || {
        mkdir -p "$INSTALL_DIR"
    }
fi

cd "$INSTALL_DIR"
echo -e "${CYAN}Building release binary (this takes ~2-3 minutes)...${NC}"
cargo build --release 2>&1 | tail -3
cp target/release/sigint-deck "$INSTALL_DIR/sigint-deck"
sudo setcap cap_net_raw,cap_net_admin+eip "$INSTALL_DIR/sigint-deck" 2>/dev/null || true
echo -e "${GREEN}Binary built: $(ls -lh $INSTALL_DIR/sigint-deck | awk '{print $5}')${NC}"

# ============================================
# Step 7: Configuration
# ============================================
echo -e "${YELLOW}[7/8] Setting up configuration...${NC}"
if [ ! -f "$INSTALL_DIR/config.toml" ]; then
    cat > "$INSTALL_DIR/config.toml" << 'CONFIG'
[device]
name = "sigint-deck-01"
location_name = "default"

[web]
enabled = true
port = 8085
bind_address = "0.0.0.0"

[wifi]
enabled = true
interface = "wlan1"
channels_2ghz = [1, 6, 11]
channels_5ghz = [36, 149]

[bluetooth]
enabled = true
detect_trackers = true

[gps]
enabled = false
gpsd_host = "localhost"
gpsd_port = 2947

[rayhunter]
enabled = false
api_url = "http://localhost:8081"

[alerts]
[alerts.sound]
enabled = true
ninja_mode = true
volume = 50

[database]
path = "sigint.db"
CONFIG
    echo -e "${GREEN}Config created at $INSTALL_DIR/config.toml${NC}"
    echo -e "${YELLOW}NOTE: Default web port is 8085 to avoid conflict with Steam services${NC}"
else
    echo -e "${GREEN}Config already exists${NC}"
fi

# ============================================
# Step 8: User Systemd Service
# ============================================
echo -e "${YELLOW}[8/8] Setting up systemd user service...${NC}"
mkdir -p ~/.config/systemd/user

cat > ~/.config/systemd/user/sigint-deck.service << EOF
[Unit]
Description=SIGINT-Deck Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
Environment=PATH=$HOME/.cargo/bin:$BIN_DIR:/usr/local/bin:/usr/bin:/bin
Environment=LD_LIBRARY_PATH=$HOME/.local/lib
ExecStart=$INSTALL_DIR/sigint-deck
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user enable sigint-deck

# Enable lingering so user services start at boot
loginctl enable-linger $USER 2>/dev/null || true

# Re-lock filesystem if SteamOS
if command -v steamos-readonly &>/dev/null; then
    sudo steamos-readonly enable 2>/dev/null || true
fi

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
LOCAL_IP=$(ip -4 addr show | grep -oP '(?<=inet\s)(?!127\.)\d+\.\d+\.\d+\.\d+' | head -1 2>/dev/null || echo "localhost")
echo -e "Web UI:            ${CYAN}http://${LOCAL_IP}:8085${NC}"
echo ""
echo -e "${YELLOW}Quick start:${NC}"
echo "  systemctl --user start sigint-deck"
echo "  journalctl --user -u sigint-deck -f"
echo ""
echo -e "${YELLOW}Important notes:${NC}"
echo "  - Steam Deck internal WiFi does NOT support monitor mode"
echo "  - Connect an external USB WiFi adapter (e.g., Alfa AWUS036ACHM)"
echo "  - The adapter should appear as wlan1"
echo "  - RTL-SDR requires USB hub (not enough power from single port)"
echo "  - Port 8085 is used to avoid conflict with Steam services on 8080"
echo ""
echo -e "${YELLOW}SDR tools installed at:${NC}"
echo "  $BIN_DIR/"
echo "  $HOME/.local/bin/"
echo ""
