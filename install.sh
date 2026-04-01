#!/bin/bash
# SIGINT-Deck Installer for Steam Deck
# 
# Usage: curl -fsSL https://raw.githubusercontent.com/naanprofit/sigint-deck/main/install.sh | bash
#    or: ./install.sh
#
# This script:
# 1. Downloads the latest release
# 2. Extracts and installs the binary
# 3. Sets up configuration (prompts for WiFi MAC addresses)
# 4. Creates systemd services
# 5. Configures channel hopping with sudoers
# 6. Creates desktop launcher
# 7. Optionally adds to Steam library

set -e

VERSION="${SIGINT_VERSION:-latest}"
INSTALL_DIR="$HOME/sigint-deck"
REPO="naanprofit/sigint-deck"
RELEASE_URL="https://github.com/$REPO/releases"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "╔════════════════════════════════════════════╗"
echo "║         SIGINT-Deck Installer              ║"
echo "║   Signals Intelligence for Steam Deck     ║"
echo "╚════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if running on Steam Deck
check_platform() {
    if [ -f /etc/os-release ]; then
        if grep -q "SteamOS" /etc/os-release; then
            echo -e "${GREEN}✓ Steam Deck detected${NC}"
            return 0
        fi
    fi
    echo -e "${YELLOW}⚠ Not running on Steam Deck - some features may not work${NC}"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
}

# Check for required tools
check_dependencies() {
    echo -e "${BLUE}Checking dependencies...${NC}"
    
    local missing=()
    
    for cmd in curl tar iw ip systemctl; do
        if ! command -v $cmd &> /dev/null; then
            missing+=($cmd)
        fi
    done
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo -e "${RED}Missing required tools: ${missing[*]}${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ All dependencies found${NC}"
}

# Detect WiFi interfaces
detect_interfaces() {
    echo -e "${BLUE}Detecting WiFi interfaces...${NC}"
    
    # Find internal WiFi (usually ath11k on Steam Deck)
    INTERNAL_IFACE=$(ip link show | grep -E "wlan[0-9]" | head -1 | awk -F: '{print $2}' | tr -d ' ')
    if [ -n "$INTERNAL_IFACE" ]; then
        INTERNAL_MAC=$(ip link show "$INTERNAL_IFACE" 2>/dev/null | grep ether | awk '{print $2}')
        echo -e "  Internal WiFi: ${GREEN}$INTERNAL_IFACE${NC} ($INTERNAL_MAC)"
    fi
    
    # Look for USB WiFi adapters
    echo ""
    echo "Available WiFi interfaces:"
    ip link show | grep -E "wlan[0-9]" | while read line; do
        iface=$(echo "$line" | awk -F: '{print $2}' | tr -d ' ')
        mac=$(ip link show "$iface" 2>/dev/null | grep ether | awk '{print $2}')
        driver=$(readlink /sys/class/net/$iface/device/driver 2>/dev/null | xargs basename 2>/dev/null || echo "unknown")
        echo "  $iface: $mac ($driver)"
    done
    echo ""
}

# Prompt for MAC addresses
configure_interfaces() {
    echo -e "${BLUE}WiFi Interface Configuration${NC}"
    echo ""
    echo "SIGINT-Deck needs to know your WiFi adapter MAC addresses to ensure"
    echo "consistent interface naming (wlan0 = internal, wlan1 = external USB)."
    echo ""
    
    detect_interfaces
    
    # Get internal MAC
    if [ -n "$INTERNAL_MAC" ]; then
        echo -e "Detected internal WiFi MAC: ${GREEN}$INTERNAL_MAC${NC}"
        read -p "Use this as wlan0? [Y/n] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Nn]$ ]]; then
            read -p "Enter internal WiFi MAC address: " INTERNAL_MAC
        fi
    else
        read -p "Enter internal WiFi MAC address (e.g., dc:2e:97:xx:xx:xx): " INTERNAL_MAC
    fi
    
    # Get external MAC
    echo ""
    echo "Now connect your external USB WiFi adapter (if not already connected)."
    read -p "Press Enter when ready..."
    
    detect_interfaces
    
    read -p "Enter external USB WiFi MAC address: " EXTERNAL_MAC
    
    # Validate MACs
    if [[ ! $INTERNAL_MAC =~ ^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$ ]]; then
        echo -e "${RED}Invalid internal MAC address format${NC}"
        exit 1
    fi
    if [[ ! $EXTERNAL_MAC =~ ^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$ ]]; then
        echo -e "${RED}Invalid external MAC address format${NC}"
        exit 1
    fi
    
    echo ""
    echo -e "${GREEN}Configuration:${NC}"
    echo "  wlan0 (internal): $INTERNAL_MAC"
    echo "  wlan1 (external): $EXTERNAL_MAC"
    echo ""
}

# Download latest release
download_release() {
    echo -e "${BLUE}Downloading SIGINT-Deck...${NC}"
    
    mkdir -p "$INSTALL_DIR"
    cd "$INSTALL_DIR"
    
    if [ "$VERSION" = "latest" ]; then
        DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url.*steamdeck.tar.gz" | cut -d '"' -f 4)
    else
        DOWNLOAD_URL="$RELEASE_URL/download/$VERSION/sigint-deck-$VERSION-steamdeck.tar.gz"
    fi
    
    if [ -z "$DOWNLOAD_URL" ]; then
        echo -e "${YELLOW}No release found, building from source...${NC}"
        build_from_source
        return
    fi
    
    echo "Downloading from: $DOWNLOAD_URL"
    curl -L -o sigint-deck.tar.gz "$DOWNLOAD_URL"
    
    echo "Extracting..."
    tar xzf sigint-deck.tar.gz
    rm sigint-deck.tar.gz
    
    echo -e "${GREEN}✓ Download complete${NC}"
}

# Build from source if no release
build_from_source() {
    echo -e "${BLUE}Building from source (this may take 5-10 minutes)...${NC}"
    
    # Check for podman
    if ! command -v podman &> /dev/null; then
        echo -e "${RED}Podman not found. Please install podman or download a release.${NC}"
        exit 1
    fi
    
    cd "$INSTALL_DIR"
    
    # Clone repo
    if [ ! -d ".git" ]; then
        git clone "https://github.com/$REPO.git" .
    fi
    
    # Build container
    podman build -f Containerfile.steamdeck -t sigint-deck:latest .
    
    # Extract binary
    podman save sigint-deck:latest -o /tmp/sigint-deck-image.tar
    cd /tmp
    mkdir -p extract && cd extract
    tar xf ../sigint-deck-image.tar
    
    for layer in *.tar; do
        if tar tvf "$layer" 2>/dev/null | grep -q 'sigint-deck$'; then
            tar xf "$layer" app/sigint-deck
            cp app/sigint-deck "$INSTALL_DIR/sigint-deck"
            break
        fi
    done
    
    cd "$INSTALL_DIR"
    rm -rf /tmp/extract /tmp/sigint-deck-image.tar
    
    echo -e "${GREEN}✓ Build complete${NC}"
}

# Install binary and set capabilities
install_binary() {
    echo -e "${BLUE}Installing binary...${NC}"
    
    chmod +x "$INSTALL_DIR/sigint-deck"
    
    # Create symlink for libpcap compatibility
    if [ ! -f /usr/lib/libpcap.so.0.8 ]; then
        echo "Creating libpcap symlink (requires sudo)..."
        sudo ln -sf /usr/lib/libpcap.so.1 /usr/lib/libpcap.so.0.8
    fi
    
    # Set capabilities for packet capture
    echo "Setting network capabilities (requires sudo)..."
    sudo setcap 'cap_net_raw,cap_net_admin+eip' "$INSTALL_DIR/sigint-deck"
    
    echo -e "${GREEN}✓ Binary installed${NC}"
}

# Create configuration
create_config() {
    echo -e "${BLUE}Creating configuration...${NC}"
    
    if [ -f "$INSTALL_DIR/config.toml" ]; then
        echo "Config already exists, backing up..."
        cp "$INSTALL_DIR/config.toml" "$INSTALL_DIR/config.toml.bak"
    fi
    
    cat > "$INSTALL_DIR/config.toml" << EOF
# SIGINT-Deck Configuration
# Generated by installer on $(date)

[device]
name = "sigint-deck"
location_name = "home"

[wifi]
enabled = true
interface = "wlan1"
scan_interval_ms = 5000
rssi_threshold = -80
attack_detection = true
pcap_enabled = false
pcap_path = "$INSTALL_DIR/data/pcap"
pcap_rotate_mb = 100

[bluetooth]
enabled = true
scan_interval_ms = 10000
rssi_threshold = -90
detect_airtags = true

[gps]
enabled = false
gpsd_host = "127.0.0.1"
gpsd_port = 2947
update_interval_ms = 1000

[database]
path = "$INSTALL_DIR/data/sigint.db"
retention_days = 30

[web]
enabled = true
bind_address = "0.0.0.0"
port = 8080

[learning]
enabled = true
training_hours = 1
anomaly_threshold = 0.7
geofence_radius_meters = 100.0

[power]
low_power_mode = false
battery_scan_interval_ms = 15000
ac_scan_interval_ms = 5000

[alerts.sound]
enabled = false
ninja_mode = false
volume = 70

[llm]
enabled = false
provider = "ollama"
endpoint = "http://localhost:11434"
model = "tinyllama"
max_tokens = 200
timeout_secs = 60
EOF
    
    # Create data directories
    mkdir -p "$INSTALL_DIR/data/pcap"
    
    echo -e "${GREEN}✓ Configuration created${NC}"
}

# Setup systemd network links for interface naming
setup_interface_naming() {
    echo -e "${BLUE}Setting up interface naming (requires sudo)...${NC}"
    
    # Create systemd link files
    sudo mkdir -p /etc/systemd/network
    
    sudo tee /etc/systemd/network/10-wlan0-internal.link > /dev/null << EOF
# SIGINT-Deck: Lock internal WiFi to wlan0
[Match]
MACAddress=$INTERNAL_MAC

[Link]
Name=wlan0
EOF

    sudo tee /etc/systemd/network/10-wlan1-external.link > /dev/null << EOF
# SIGINT-Deck: Lock external USB adapter to wlan1
[Match]
MACAddress=$EXTERNAL_MAC

[Link]
Name=wlan1
EOF

    # Mark wlan1 as unmanaged by NetworkManager
    sudo mkdir -p /etc/NetworkManager/conf.d
    sudo tee /etc/NetworkManager/conf.d/90-sigint-unmanaged.conf > /dev/null << EOF
[keyfile]
unmanaged-devices=interface-name:wlan1
EOF

    echo -e "${GREEN}✓ Interface naming configured${NC}"
    echo -e "${YELLOW}⚠ Unplug and replug your USB WiFi adapter for changes to take effect${NC}"
}

# Setup channel hopping
setup_channel_hopping() {
    echo -e "${BLUE}Setting up channel hopping...${NC}"
    
    # Create channel hop script
    cat > "$INSTALL_DIR/channel-hop.sh" << 'EOF'
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
    chmod +x "$INSTALL_DIR/channel-hop.sh"
    
    # Create sudoers entry
    echo "Setting up passwordless sudo for iw (requires sudo)..."
    sudo tee /etc/sudoers.d/zzz-sigint-wifi > /dev/null << EOF
# SIGINT-Deck: Allow passwordless iw for channel hopping
$USER ALL=(ALL) NOPASSWD: /usr/bin/iw
Defaults:$USER !requiretty
EOF
    sudo chmod 440 /etc/sudoers.d/zzz-sigint-wifi
    
    echo -e "${GREEN}✓ Channel hopping configured${NC}"
}

# Create systemd services
create_services() {
    echo -e "${BLUE}Creating systemd services...${NC}"
    
    mkdir -p "$HOME/.config/systemd/user"
    
    # Main service
    cat > "$HOME/.config/systemd/user/sigint-deck.service" << EOF
[Unit]
Description=SIGINT-Deck Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
ExecStart=$INSTALL_DIR/sigint-deck
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOF

    # Channel hopper service
    cat > "$HOME/.config/systemd/user/channel-hop.service" << EOF
[Unit]
Description=WiFi Channel Hopper for SIGINT-Deck
After=sigint-deck.service

[Service]
Type=simple
ExecStart=$INSTALL_DIR/channel-hop.sh wlan1
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
EOF

    # Monitor mode script (called on boot and resume from suspend)
    cat > "$INSTALL_DIR/set-monitor-mode.sh" << 'MONITOREOF'
#!/bin/bash
# Set external WiFi adapter to monitor mode
# Called by systemd on boot and resume from suspend
# Also cleans up phantom wlan interfaces

LOG_TAG="sigint-monitor"

log() {
    logger -t "$LOG_TAG" "$1"
    echo "$1"
}

# Wait for interface to appear
sleep 2

# Clean up phantom wlan interfaces (wlan2, wlan3, wlan137, etc.)
log "Cleaning up phantom interfaces..."
for iface in $(ip link show 2>/dev/null | grep -oE "wlan[0-9]+" | grep -vE "^wlan[01]$"); do
    log "Removing phantom interface: $iface"
    ip link delete "$iface" 2>/dev/null || true
done

# If wlan1 doesn't exist but mt76 device is present, reload driver
if ! ip link show wlan1 &>/dev/null; then
    if lsusb | grep -qi "mediatek\|0e8d:7612"; then
        log "External adapter present but wlan1 missing, reloading driver..."
        modprobe -r mt76x2u 2>/dev/null
        sleep 2
        modprobe mt76x2u 2>/dev/null
        sleep 3
    fi
fi

# Find external WiFi adapter (not wlan0)
IFACE=$(ip link show 2>/dev/null | grep -oE "wlan[0-9]+" | grep -v wlan0 | head -1)

if [ -z "$IFACE" ]; then
    log "No external WiFi adapter found"
    exit 0
fi

# Update config if interface name changed
CONFIG_FILE="$HOME/sigint-deck/config.toml"
if [ -f "$CONFIG_FILE" ]; then
    CURRENT_IFACE=$(grep 'interface = "wlan' "$CONFIG_FILE" | grep -oE 'wlan[0-9]+')
    if [ -n "$CURRENT_IFACE" ] && [ "$CURRENT_IFACE" != "$IFACE" ]; then
        log "Updating config from $CURRENT_IFACE to $IFACE"
        sed -i "s/interface = \"$CURRENT_IFACE\"/interface = \"$IFACE\"/" "$CONFIG_FILE"
    fi
fi

# Check current mode
CURRENT_MODE=$(iwconfig "$IFACE" 2>/dev/null | grep -oE "Mode:[A-Za-z]+" | cut -d: -f2)

if [ "$CURRENT_MODE" = "Monitor" ]; then
    log "$IFACE already in Monitor mode"
    exit 0
fi

log "Setting $IFACE to Monitor mode..."

# Disconnect from any network first
nmcli device disconnect "$IFACE" 2>/dev/null || true

# Set monitor mode
ip link set "$IFACE" down 2>/dev/null
iw dev "$IFACE" set type monitor 2>/dev/null
ip link set "$IFACE" up 2>/dev/null

# Verify
NEW_MODE=$(iwconfig "$IFACE" 2>/dev/null | grep -oE "Mode:[A-Za-z]+" | cut -d: -f2)
log "$IFACE is now in $NEW_MODE mode"
MONITOREOF
    chmod +x "$INSTALL_DIR/set-monitor-mode.sh"

    # Monitor mode service (runs on boot and resume)
    cat > "$HOME/.config/systemd/user/sigint-monitor-mode.service" << EOF
[Unit]
Description=Set WiFi adapter to Monitor mode
After=network.target
Before=sigint-deck.service

[Service]
Type=oneshot
ExecStart=/usr/bin/sudo $INSTALL_DIR/set-monitor-mode.sh
RemainAfterExit=yes

[Install]
WantedBy=default.target
EOF

    # Add sudoers entry for monitor mode script
    echo -e "${YELLOW}Adding sudoers entry for monitor mode script...${NC}"
    SUDOERS_MONITOR="$USER ALL=(ALL) NOPASSWD: $INSTALL_DIR/set-monitor-mode.sh"
    if ! sudo grep -qF "$SUDOERS_MONITOR" /etc/sudoers.d/sigint-deck 2>/dev/null; then
        echo "$SUDOERS_MONITOR" | sudo tee -a /etc/sudoers.d/sigint-deck > /dev/null
    fi

    # Reload and enable
    systemctl --user daemon-reload
    systemctl --user enable sigint-deck.service
    systemctl --user enable channel-hop.service
    systemctl --user enable sigint-monitor-mode.service
    
    # Enable lingering for user services
    sudo loginctl enable-linger "$USER"
    
    echo -e "${GREEN}✓ Services created and enabled${NC}"
}

# Create desktop launcher
create_desktop_launcher() {
    echo -e "${BLUE}Creating desktop launcher...${NC}"
    
    mkdir -p "$HOME/.local/share/applications"
    
    # Create launch script
    cat > "$INSTALL_DIR/launch.sh" << 'EOF'
#!/bin/bash
INSTALL_DIR="$HOME/sigint-deck"
DASHBOARD_URL="http://localhost:8080"

# Ensure services are running
systemctl --user start sigint-deck.service 2>/dev/null
systemctl --user start channel-hop.service 2>/dev/null

# Wait for dashboard
for i in {1..30}; do
    curl -s "$DASHBOARD_URL/api/status" > /dev/null 2>&1 && break
    sleep 1
done

# Open dashboard
xdg-open "$DASHBOARD_URL" 2>/dev/null || echo "Dashboard: $DASHBOARD_URL"
EOF
    chmod +x "$INSTALL_DIR/launch.sh"
    
    # Create desktop entry
    cat > "$HOME/.local/share/applications/sigint-deck.desktop" << EOF
[Desktop Entry]
Name=SIGINT-Deck
Comment=Signals Intelligence Security Scanner
Exec=$INSTALL_DIR/launch.sh
Icon=network-wireless
Terminal=false
Type=Application
Categories=Network;Security;
EOF
    
    # Update desktop database
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
    
    echo -e "${GREEN}✓ Desktop launcher created${NC}"
}

# Create Steam launch script
create_steam_launcher() {
    echo -e "${BLUE}Creating Steam launcher...${NC}"
    
    cat > "$INSTALL_DIR/launch-in-steam.sh" << 'STEAMSCRIPT'
#!/bin/bash
INSTALL_DIR="$HOME/sigint-deck"
DASHBOARD_URL="http://localhost:8080"

# Ensure services running
systemctl --user start sigint-deck.service 2>/dev/null
systemctl --user start channel-hop.service 2>/dev/null

# Wait for dashboard
for i in {1..30}; do
    curl -s "$DASHBOARD_URL/api/status" > /dev/null 2>&1 && break
    sleep 1
done

# Open in Steam browser
steam "steam://openurl/$DASHBOARD_URL" 2>/dev/null &

# Show status
while true; do
    clear
    echo "════════════════════════════════════════"
    echo "         SIGINT-Deck Status"
    echo "════════════════════════════════════════"
    
    STATUS=$(curl -s "$DASHBOARD_URL/api/hardware/status" 2>/dev/null)
    if [ -n "$STATUS" ]; then
        echo "$STATUS" | python3 -c "
import sys, json
try:
    d = json.load(sys.stdin)
    print(f\"  WiFi:    {'✓' if d.get('wifi') else '✗'}\")
    print(f\"  BLE:     {'✓' if d.get('ble') else '✗'}\")
    print(f\"  GPS:     {'✓' if d.get('gps') else '✗'}\")
    print(f\"  Battery: {d.get('battery', '?')}%\")
except: pass
" 2>/dev/null
    fi
    
    echo ""
    echo "  Dashboard: $DASHBOARD_URL"
    echo "  Press Steam button for overlay browser"
    echo "════════════════════════════════════════"
    sleep 5
done
STEAMSCRIPT
    chmod +x "$INSTALL_DIR/launch-in-steam.sh"
    
    echo -e "${GREEN}✓ Steam launcher created${NC}"
}

# Setup Python venv for Steam integration
setup_python_venv() {
    echo -e "${BLUE}Setting up Python environment...${NC}"
    
    if ! command -v python3 &> /dev/null; then
        echo -e "${YELLOW}Python3 not found, skipping Steam integration${NC}"
        return 1
    fi
    
    # Create venv
    python3 -m venv "$INSTALL_DIR/.venv" 2>/dev/null || {
        echo -e "${YELLOW}Could not create venv, skipping Steam integration${NC}"
        return 1
    }
    
    # Install required Python packages
    echo -e "${BLUE}Installing Python packages...${NC}"
    "$INSTALL_DIR/.venv/bin/pip" install --quiet --upgrade pip 2>/dev/null
    "$INSTALL_DIR/.venv/bin/pip" install --quiet vdf 2>/dev/null || {
        echo -e "${YELLOW}Could not install vdf library${NC}"
    }
    
    # Install voice packages (optional - may fail on some systems)
    echo -e "${BLUE}Installing voice packages (optional)...${NC}"
    "$INSTALL_DIR/.venv/bin/pip" install --quiet faster-whisper 2>/dev/null && {
        echo -e "${GREEN}✓ faster-whisper installed (speech-to-text)${NC}"
    } || {
        echo -e "${YELLOW}⚠ faster-whisper not installed (voice notes disabled)${NC}"
    }
    
    "$INSTALL_DIR/.venv/bin/pip" install --quiet piper-tts 2>/dev/null && {
        echo -e "${GREEN}✓ piper-tts installed (text-to-speech)${NC}"
    } || {
        echo -e "${YELLOW}⚠ piper-tts not installed (TTS disabled)${NC}"
    }
    
    # Create add-to-steam script
    cat > "$INSTALL_DIR/add-to-steam.py" << 'STEAMPY'
#!/usr/bin/env python3
"""Add SIGINT-Deck to Steam as a non-Steam game"""
import os, sys
venv_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".venv", "lib")
for d in os.listdir(venv_path):
    if d.startswith("python"):
        sys.path.insert(0, os.path.join(venv_path, d, "site-packages"))
        break
import vdf

STEAM_PATH = os.path.expanduser("~/.local/share/Steam")
SHORTCUTS_PATH = os.path.join(STEAM_PATH, "userdata")
INSTALL_DIR = os.path.expanduser("~/sigint-deck")

def get_user_id():
    for entry in os.listdir(SHORTCUTS_PATH):
        if entry.isdigit(): return entry
    return None

def generate_shortcut_id(exe, name):
    key = f"{exe}{name}"
    crc = 0
    for char in key:
        crc = ((crc << 5) + crc + ord(char)) & 0xFFFFFFFF
    if crc > 0x7FFFFFFF: crc = crc - 0x100000000
    return crc

def add_shortcut():
    user_id = get_user_id()
    if not user_id:
        print("ERROR: Steam user directory not found")
        return False
    
    shortcuts_file = os.path.join(SHORTCUTS_PATH, user_id, "config", "shortcuts.vdf")
    shortcuts = {"shortcuts": {}}
    if os.path.exists(shortcuts_file):
        try:
            with open(shortcuts_file, "rb") as f:
                shortcuts = vdf.binary_load(f)
        except: pass
    
    for key, sc in shortcuts.get("shortcuts", {}).items():
        if "SIGINT" in str(sc.get("AppName", "")) or "sigint" in str(sc.get("Exe", "")).lower():
            print("SIGINT-Deck already in Steam"); return True
    
    existing_keys = [int(k) for k in shortcuts.get("shortcuts", {}).keys() if k.isdigit()]
    next_key = str(max(existing_keys) + 1) if existing_keys else "0"
    exe_path = os.path.join(INSTALL_DIR, "launch-steam.sh")
    
    shortcuts["shortcuts"][next_key] = {
        "appid": generate_shortcut_id(exe_path, "SIGINT-Deck"),
        "AppName": "SIGINT-Deck", "Exe": f'"{exe_path}"',
        "StartDir": f'"{INSTALL_DIR}"', "icon": "", "ShortcutPath": "",
        "LaunchOptions": "", "IsHidden": 0, "AllowDesktopConfig": 1,
        "AllowOverlay": 1, "OpenVR": 0, "Devkit": 0, "DevkitGameID": "",
        "DevkitOverrideAppID": 0, "LastPlayTime": 0, "FlatpakAppID": "",
        "tags": {"0": "Security", "1": "Tools"}
    }
    
    os.makedirs(os.path.dirname(shortcuts_file), exist_ok=True)
    with open(shortcuts_file, "wb") as f:
        vdf.binary_dump(shortcuts, f)
    print("SUCCESS: Added SIGINT-Deck to Steam!")
    print("Restart Steam or switch to Gaming Mode to see it.")
    return True

if __name__ == "__main__":
    try: add_shortcut()
    except Exception as e: print(f"Error: {e}"); sys.exit(1)
STEAMPY
    chmod +x "$INSTALL_DIR/add-to-steam.py"
    
    echo -e "${GREEN}✓ Python environment ready${NC}"
    return 0
}

# Offer to add to Steam
add_to_steam() {
    echo ""
    read -p "Would you like to add SIGINT-Deck to Steam library? [Y/n] " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        # Try automatic method first
        if [ -f "$INSTALL_DIR/.venv/bin/python" ]; then
            echo "Adding to Steam automatically..."
            "$INSTALL_DIR/.venv/bin/python" "$INSTALL_DIR/add-to-steam.py" && return 0
        fi
        
        # Fallback to manual instructions
        echo ""
        echo -e "${YELLOW}To add to Steam manually:${NC}"
        echo "1. Open Steam in Desktop Mode"
        echo "2. Go to Games → Add a Non-Steam Game"
        echo "3. Click Browse and select:"
        echo -e "   ${GREEN}$INSTALL_DIR/launch-steam.sh${NC}"
        echo "4. Click 'Add Selected Programs'"
        echo "5. Right-click the game → Properties → Rename to 'SIGINT-Deck'"
        echo ""
        read -p "Press Enter when done..."
    fi
}

# Enable monitor mode helper
setup_monitor_mode() {
    echo -e "${BLUE}Setting up monitor mode...${NC}"
    
    # Create monitor mode script
    cat > "$INSTALL_DIR/enable-monitor.sh" << 'EOF'
#!/bin/bash
# Enable monitor mode on external WiFi adapter
# Run with sudo or as root

IFACE=${1:-wlan1}

echo "Enabling monitor mode on $IFACE..."

# Disconnect from any network and disable NetworkManager control
nmcli device disconnect $IFACE 2>/dev/null || true
nmcli device set $IFACE managed no 2>/dev/null || true

# Set monitor mode
ip link set $IFACE down
iw $IFACE set type monitor
ip link set $IFACE up

# Verify
MODE=$(iw dev $IFACE info 2>/dev/null | grep type | awk '{print $2}')
if [ "$MODE" = "monitor" ]; then
    echo "✓ Monitor mode enabled on $IFACE"
else
    echo "✗ Failed to enable monitor mode on $IFACE"
    exit 1
fi
EOF
    chmod +x "$INSTALL_DIR/enable-monitor.sh"
    
    # Create systemd service that runs at boot
    sudo tee /etc/systemd/system/sigint-monitor-mode.service > /dev/null << EOF
[Unit]
Description=Enable monitor mode on wlan1 for SIGINT-Deck
After=network.target NetworkManager.service
Wants=network.target

[Service]
Type=oneshot
RemainAfterExit=yes
# Disconnect from NetworkManager
ExecStart=/usr/bin/nmcli device disconnect wlan1
ExecStart=/usr/bin/nmcli device set wlan1 managed no
# Set monitor mode
ExecStart=/usr/bin/ip link set wlan1 down
ExecStart=/usr/bin/iw wlan1 set type monitor
ExecStart=/usr/bin/ip link set wlan1 up
# On stop, restore managed mode
ExecStop=/usr/bin/ip link set wlan1 down
ExecStop=/usr/bin/iw wlan1 set type managed
ExecStop=/usr/bin/nmcli device set wlan1 managed yes

[Install]
WantedBy=multi-user.target
EOF

    sudo systemctl daemon-reload
    sudo systemctl enable sigint-monitor-mode.service
    
    # Also configure NetworkManager to ignore wlan1 permanently
    sudo mkdir -p /etc/NetworkManager/conf.d
    sudo tee /etc/NetworkManager/conf.d/90-sigint-unmanaged.conf > /dev/null << EOF
# SIGINT-Deck: Don't let NetworkManager control the external WiFi adapter
[keyfile]
unmanaged-devices=interface-name:wlan1
EOF
    
    # Restart NetworkManager to apply
    sudo systemctl restart NetworkManager 2>/dev/null || true
    
    # Enable monitor mode now
    echo "Enabling monitor mode now..."
    sudo "$INSTALL_DIR/enable-monitor.sh" wlan1 || true
    
    echo -e "${GREEN}✓ Monitor mode configured${NC}"
    echo -e "${YELLOW}Note: Monitor mode will be enabled automatically at boot${NC}"
}

# Print final instructions
print_summary() {
    echo ""
    echo -e "${GREEN}╔════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║     SIGINT-Deck Installation Complete!     ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Installation directory: $INSTALL_DIR"
    echo ""
    echo -e "${BLUE}Starting Services...${NC}"
    
    # Start services now
    systemctl --user start sigint-deck channel-hop 2>/dev/null
    sleep 2
    
    # Check if running
    if systemctl --user is-active sigint-deck >/dev/null 2>&1; then
        echo -e "${GREEN}✓ SIGINT-Deck is running${NC}"
        echo -e "${GREEN}✓ Dashboard: http://localhost:8080${NC}"
    else
        echo -e "${YELLOW}⚠ Service may need manual start (see troubleshooting below)${NC}"
    fi
    echo ""
    echo -e "${BLUE}Quick Commands:${NC}"
    echo "  Start:   systemctl --user start sigint-deck channel-hop"
    echo "  Stop:    systemctl --user stop sigint-deck channel-hop"
    echo "  Status:  systemctl --user status sigint-deck"
    echo "  Logs:    journalctl --user -u sigint-deck -f"
    echo "  TUI:     $INSTALL_DIR/sigint-deck --tui"
    echo ""
    echo -e "${BLUE}Troubleshooting:${NC}"
    echo ""
    echo "  ${YELLOW}No WiFi devices detected?${NC}"
    echo "  The adapter must be in monitor mode. Check and fix with:"
    echo "    iw dev wlan1 info | grep type    # Should show 'monitor'"
    echo "    sudo $INSTALL_DIR/enable-monitor.sh wlan1"
    echo "    systemctl --user restart sigint-deck"
    echo ""
    echo "  ${YELLOW}Interface busy or wrong mode?${NC}"
    echo "  NetworkManager may be controlling the adapter. Fix with:"
    echo "    sudo nmcli device set wlan1 managed no"
    echo "    sudo systemctl restart sigint-monitor-mode"
    echo ""
    echo "  ${YELLOW}Service won't start?${NC}"
    echo "    journalctl --user -u sigint-deck -n 50  # Check logs"
    echo ""
    echo "  ${YELLOW}Channel hopping errors?${NC}"
    echo "  Normal when adapter is busy. WiFi capture still works."
    echo ""
    echo -e "${RED}IMPORTANT: This tool is for authorized security research only.${NC}"
    echo -e "${RED}Only monitor networks you own or have permission to test.${NC}"
    echo ""
    echo -e "${BLUE}Optional Add-ons:${NC}"
    echo "  SDR Support (RTL-SDR, HackRF, LimeSDR):"
    echo "    $INSTALL_DIR/scripts/install-sdr.sh"
    echo ""
    echo "  RayHunter IMSI Catcher Detection:"
    echo "    $INSTALL_DIR/scripts/install-adb.sh"
    echo ""
}

# Offer optional SDR installation
offer_sdr_install() {
    echo ""
    echo -e "${BLUE}Optional: Install SDR Support?${NC}"
    echo "This adds support for RTL-SDR, HackRF, and LimeSDR devices."
    read -p "Install SDR tools? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -f "$INSTALL_DIR/scripts/install-sdr.sh" ]; then
            bash "$INSTALL_DIR/scripts/install-sdr.sh"
        else
            echo -e "${YELLOW}SDR install script not found, skipping${NC}"
        fi
    fi
}

# Offer optional RayHunter installation
offer_rayhunter_install() {
    echo ""
    echo -e "${BLUE}Optional: Install RayHunter IMSI Catcher Detection?${NC}"
    echo "This adds support for EFF's RayHunter (requires Pixel phone)."
    read -p "Install RayHunter/ADB? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        if [ -f "$INSTALL_DIR/scripts/install-adb.sh" ]; then
            bash "$INSTALL_DIR/scripts/install-adb.sh"
        else
            echo -e "${YELLOW}ADB install script not found, skipping${NC}"
        fi
    fi
}

# Main installation flow
main() {
    check_platform
    check_dependencies
    configure_interfaces
    download_release
    install_binary
    create_config
    setup_interface_naming
    setup_channel_hopping
    setup_monitor_mode
    create_services
    create_desktop_launcher
    create_steam_launcher
    setup_python_venv
    add_to_steam
    offer_sdr_install
    offer_rayhunter_install
    print_summary
}

# Run installer
main "$@"
