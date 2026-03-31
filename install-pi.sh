#!/bin/bash
# SIGINT-Pi Installer for Raspberry Pi
# 
# Supported: Pi Zero 2 W, Pi 3, Pi 4, Pi 5
# 
# Usage: curl -fsSL https://raw.githubusercontent.com/naanprofit/sigint-pi/main/install-pi.sh | bash

set -e

VERSION="${SIGINT_VERSION:-latest}"
INSTALL_DIR="$HOME/sigint-pi"
REPO="naanprofit/sigint-pi"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════╗"
echo "║         SIGINT-Pi Installer                ║"
echo "║   Signals Intelligence for Raspberry Pi   ║"
echo "╚════════════════════════════════════════════╝"
echo -e "${NC}"

# Detect Pi model and available RAM
detect_pi_model() {
    if [ -f /proc/device-tree/model ]; then
        PI_MODEL=$(cat /proc/device-tree/model | tr -d '\0')
        echo -e "${GREEN}Detected: $PI_MODEL${NC}"
    else
        PI_MODEL="Unknown"
        echo -e "${YELLOW}Could not detect Pi model${NC}"
    fi
    
    # Get total RAM in MB
    TOTAL_RAM=$(free -m | awk '/^Mem:/{print $2}')
    echo -e "Total RAM: ${TOTAL_RAM}MB"
    
    # Determine profile based on RAM
    if [ "$TOTAL_RAM" -lt 512 ]; then
        PROFILE="minimal"
        echo -e "${YELLOW}Profile: MINIMAL (headless, web UI only)${NC}"
    elif [ "$TOTAL_RAM" -lt 1024 ]; then
        PROFILE="standard"
        echo -e "${BLUE}Profile: STANDARD (web UI, optional TUI)${NC}"
    else
        PROFILE="full"
        echo -e "${GREEN}Profile: FULL (web UI + TUI)${NC}"
    fi
}

# Check dependencies
check_dependencies() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    
    local missing=()
    
    for cmd in curl tar iw ip systemctl gpsd; do
        if ! command -v $cmd &> /dev/null; then
            missing+=($cmd)
        fi
    done
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${YELLOW}Installing missing packages: ${missing[*]}${NC}"
        sudo apt-get update
        sudo apt-get install -y "${missing[@]}" gpsd gpsd-clients libpcap-dev bluez
    fi
    
    echo -e "${GREEN}✓ Dependencies satisfied${NC}"
}

# Optimize Pi Zero 2 W for headless operation
optimize_pi_zero() {
    if [ "$PROFILE" = "minimal" ]; then
        echo -e "${BLUE}Optimizing for Pi Zero 2 W...${NC}"
        
        # Reduce GPU memory to minimum (16MB)
        if ! grep -q "gpu_mem=16" /boot/config.txt 2>/dev/null; then
            echo -e "${YELLOW}Setting GPU memory to 16MB (requires reboot)${NC}"
            echo "gpu_mem=16" | sudo tee -a /boot/config.txt > /dev/null
            NEEDS_REBOOT=1
        fi
        
        # Disable HDMI to save power
        if command -v tvservice &> /dev/null; then
            sudo tvservice -o 2>/dev/null || true
        fi
        
        # Disable Bluetooth if using external adapter
        # (uncomment if needed)
        # sudo systemctl disable hciuart
        
        echo -e "${GREEN}✓ Pi Zero optimizations applied${NC}"
    fi
}

# Download and install binary
install_binary() {
    echo -e "${BLUE}Downloading SIGINT-Pi...${NC}"
    
    mkdir -p "$INSTALL_DIR"
    cd "$INSTALL_DIR"
    
    # Determine architecture
    ARCH=$(uname -m)
    case "$ARCH" in
        aarch64) ARCH_NAME="aarch64" ;;
        armv7l) ARCH_NAME="armv7" ;;
        armv6l) ARCH_NAME="armv6" ;;
        *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
    esac
    
    if [ "$VERSION" = "latest" ]; then
        DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/sigint-pi-$ARCH_NAME.tar.gz"
    else
        DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/sigint-pi-$ARCH_NAME.tar.gz"
    fi
    
    echo "Downloading from: $DOWNLOAD_URL"
    
    if curl -fsSL "$DOWNLOAD_URL" -o sigint-pi.tar.gz; then
        tar xzf sigint-pi.tar.gz
        rm sigint-pi.tar.gz
        chmod +x sigint-pi
        echo -e "${GREEN}✓ Binary installed${NC}"
    else
        echo -e "${RED}Download failed. Building from source...${NC}"
        build_from_source
    fi
}

# Build from source if binary not available
build_from_source() {
    echo -e "${BLUE}Building from source...${NC}"
    
    # Install Rust if needed
    if ! command -v cargo &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Clone and build
    cd "$HOME"
    if [ -d sigint-pi-src ]; then
        cd sigint-pi-src && git pull
    else
        git clone https://github.com/$REPO.git sigint-pi-src
        cd sigint-pi-src
    fi
    
    cargo build --release
    
    mkdir -p "$INSTALL_DIR"
    cp target/release/sigint-pi "$INSTALL_DIR/"
    cp -r static "$INSTALL_DIR/"
    cp -r data "$INSTALL_DIR/"
    cp config.toml.example "$INSTALL_DIR/config.toml"
    
    echo -e "${GREEN}✓ Built from source${NC}"
}

# Detect WiFi interfaces
detect_wifi() {
    echo -e "${BLUE}Detecting WiFi interfaces...${NC}"
    
    for iface in $(ls /sys/class/net/ | grep -E "^wlan"); do
        driver=$(readlink /sys/class/net/$iface/device/driver 2>/dev/null | xargs basename 2>/dev/null || echo "unknown")
        mac=$(ip link show "$iface" 2>/dev/null | grep ether | awk '{print $2}')
        echo "  $iface: $mac ($driver)"
        
        # Check monitor mode support
        if iw phy $(cat /sys/class/net/$iface/phy80211/name 2>/dev/null) info 2>/dev/null | grep -q "monitor"; then
            echo -e "    ${GREEN}✓ Monitor mode supported${NC}"
            MONITOR_IFACE="$iface"
        else
            echo -e "    ${YELLOW}⚠ No monitor mode${NC}"
        fi
    done
}

# Configure interface
configure_wifi() {
    echo ""
    if [ -n "$MONITOR_IFACE" ]; then
        echo -e "Using ${GREEN}$MONITOR_IFACE${NC} for WiFi scanning"
        
        # Create config if not exists
        if [ ! -f "$INSTALL_DIR/config.toml" ]; then
            cp "$INSTALL_DIR/config.toml.example" "$INSTALL_DIR/config.toml" 2>/dev/null || \
            cat > "$INSTALL_DIR/config.toml" << EOF
[device]
name = "sigint-pi-01"
location_name = "Home Base"

[wifi]
enabled = true
interface = "$MONITOR_IFACE"
scan_interval_ms = 100
rssi_threshold = -90
attack_detection = true
pcap_enabled = false
pcap_path = "$INSTALL_DIR/pcap"
pcap_rotate_mb = 100

[bluetooth]
enabled = true
detect_trackers = true

[gps]
enabled = true
device = "/dev/ttyACM0"
gpsd_host = "localhost"
gpsd_port = 2947
geofencing_enabled = false

[database]
path = "$INSTALL_DIR/sigint.db"
retention_days = 30

[alerts]
cooldown_seconds = 300

[alerts.sound]
enabled = false
ninja_mode = true
volume = 50

[web]
enabled = true
host = "0.0.0.0"
port = 8080
cors_origins = ["*"]

[learning]
enabled = true
training_hours = 1
anomaly_threshold = 0.7

[power]
mode = "balanced"

[influxdb]
enabled = false
EOF
        fi
        
        # Update interface in config
        sed -i "s/interface = \".*\"/interface = \"$MONITOR_IFACE\"/" "$INSTALL_DIR/config.toml"
    else
        echo -e "${YELLOW}No monitor-capable WiFi found. Connect a USB adapter.${NC}"
    fi
}

# Setup GPS
setup_gps() {
    echo -e "${BLUE}Setting up GPS...${NC}"
    
    # Find GPS device
    GPS_DEV=""
    for dev in /dev/ttyACM* /dev/ttyUSB*; do
        if [ -e "$dev" ]; then
            GPS_DEV="$dev"
            break
        fi
    done
    
    if [ -n "$GPS_DEV" ]; then
        echo -e "GPS device found: ${GREEN}$GPS_DEV${NC}"
        
        # Configure gpsd
        sudo systemctl stop gpsd gpsd.socket 2>/dev/null || true
        
        cat << EOF | sudo tee /etc/default/gpsd > /dev/null
DEVICES="$GPS_DEV"
GPSD_OPTIONS="-n"
USBAUTO="true"
EOF
        
        sudo systemctl enable gpsd gpsd.socket
        sudo systemctl start gpsd.socket gpsd
        
        echo -e "${GREEN}✓ GPS configured${NC}"
    else
        echo -e "${YELLOW}No GPS device found${NC}"
    fi
}

# Setup RayHunter (if connected)
setup_rayhunter() {
    echo -e "${BLUE}Checking for RayHunter...${NC}"
    
    # Check for Android device via ADB
    if command -v adb &> /dev/null; then
        if adb devices | grep -q "device$"; then
            echo -e "${GREEN}Android device detected - checking for RayHunter...${NC}"
            
            # Forward RayHunter port
            adb forward tcp:8080 tcp:8080 2>/dev/null || true
            
            # Test connection
            if curl -s http://localhost:8080/api/status &>/dev/null; then
                echo -e "${GREEN}✓ RayHunter connected and accessible${NC}"
                RAYHUNTER_AVAILABLE=1
            else
                echo -e "${YELLOW}RayHunter not responding on port 8080${NC}"
            fi
        fi
    else
        echo -e "${YELLOW}ADB not installed - install with: sudo apt install adb${NC}"
    fi
}

# Create systemd services
create_services() {
    echo -e "${BLUE}Creating systemd services...${NC}"
    
    # Main service
    cat << EOF | sudo tee /etc/systemd/system/sigint-pi.service > /dev/null
[Unit]
Description=SIGINT-Pi Security Scanner
After=network.target gpsd.service bluetooth.service
Wants=gpsd.service bluetooth.service

[Service]
Type=simple
User=$USER
WorkingDirectory=$INSTALL_DIR
ExecStart=$INSTALL_DIR/sigint-pi --config $INSTALL_DIR/config.toml
Restart=always
RestartSec=5
Environment=RUST_LOG=info

# Capabilities for raw packet capture
AmbientCapabilities=CAP_NET_RAW CAP_NET_ADMIN

[Install]
WantedBy=multi-user.target
EOF

    # Channel hopper service
    cat << EOF | sudo tee /etc/systemd/system/sigint-channel-hop.service > /dev/null
[Unit]
Description=SIGINT-Pi Channel Hopper
After=sigint-pi.service
BindsTo=sigint-pi.service

[Service]
Type=simple
User=root
ExecStart=/bin/bash -c 'while true; do for ch in 1 6 11 2 3 4 5 7 8 9 10 36 40 44 48 52 56 60 64 100 104 108 112 116 120 124 128 132 136 140 149 153 157 161 165; do iw dev $MONITOR_IFACE set channel \$ch 2>/dev/null; sleep 0.3; done; done'
Restart=always
RestartSec=1

[Install]
WantedBy=multi-user.target
EOF

    # Set capabilities on binary
    sudo setcap 'cap_net_raw,cap_net_admin+eip' "$INSTALL_DIR/sigint-pi"
    
    # Reload and enable
    sudo systemctl daemon-reload
    sudo systemctl enable sigint-pi
    
    echo -e "${GREEN}✓ Services created${NC}"
}

# Create monitor mode setup script
create_monitor_script() {
    cat << 'EOF' > "$INSTALL_DIR/set-monitor-mode.sh"
#!/bin/bash
IFACE="${1:-wlan1}"

# Check if interface exists
if [ ! -d "/sys/class/net/$IFACE" ]; then
    echo "Interface $IFACE not found"
    exit 1
fi

# Set monitor mode
ip link set "$IFACE" down
iw dev "$IFACE" set type monitor
ip link set "$IFACE" up

echo "Monitor mode enabled on $IFACE"
iwconfig "$IFACE" | head -1
EOF
    chmod +x "$INSTALL_DIR/set-monitor-mode.sh"
}

# Start services
start_services() {
    echo -e "${BLUE}Starting services...${NC}"
    
    # Set monitor mode
    if [ -n "$MONITOR_IFACE" ]; then
        sudo "$INSTALL_DIR/set-monitor-mode.sh" "$MONITOR_IFACE"
    fi
    
    sudo systemctl start sigint-pi
    sleep 3
    
    if systemctl is-active --quiet sigint-pi; then
        echo -e "${GREEN}✓ SIGINT-Pi is running${NC}"
        
        # Get IP address
        IP=$(hostname -I | awk '{print $1}')
        echo ""
        echo -e "${GREEN}╔════════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║  SIGINT-Pi is ready!                       ║${NC}"
        echo -e "${GREEN}╠════════════════════════════════════════════╣${NC}"
        echo -e "${GREEN}║  Dashboard: http://$IP:8080          ║${NC}"
        echo -e "${GREEN}╚════════════════════════════════════════════╝${NC}"
    else
        echo -e "${RED}Service failed to start. Check logs:${NC}"
        echo "  sudo journalctl -u sigint-pi -n 50"
    fi
}

# Main installation
main() {
    detect_pi_model
    check_dependencies
    optimize_pi_zero
    install_binary
    detect_wifi
    configure_wifi
    setup_gps
    setup_rayhunter
    create_monitor_script
    create_services
    start_services
    
    if [ "$NEEDS_REBOOT" = "1" ]; then
        echo ""
        echo -e "${YELLOW}⚠ System optimizations require a reboot.${NC}"
        echo -e "Run: ${BLUE}sudo reboot${NC}"
    fi
}

main "$@"
