#!/bin/bash
# SIGINT-Pi RayHunter Integration Installer
# Sets up ADB and polling scripts for EFF RayHunter IMSI catcher detection
#
# Usage: ./install-rayhunter.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="${SIGINT_INSTALL_DIR:-/opt/sigint-pi}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════╗"
echo "║    SIGINT-Pi RayHunter Integration Setup      ║"
echo "║       EFF IMSI Catcher Detection              ║"
echo "╚═══════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if running as root for system-wide install
if [ "$EUID" -eq 0 ]; then
    SYSTEM_INSTALL=true
    SERVICE_DIR="/etc/systemd/system"
else
    SYSTEM_INSTALL=false
    SERVICE_DIR="$HOME/.config/systemd/user"
    mkdir -p "$SERVICE_DIR"
fi

# Install ADB
echo -e "${BLUE}[1/4] Installing ADB...${NC}"

if command -v apt-get &> /dev/null; then
    sudo apt-get update
    sudo apt-get install -y android-tools-adb
elif command -v pacman &> /dev/null; then
    sudo pacman -S --noconfirm android-tools
else
    echo -e "${YELLOW}Please install ADB manually${NC}"
fi

# Verify ADB
if command -v adb &> /dev/null; then
    echo -e "${GREEN}✓ ADB installed: $(adb version | head -1)${NC}"
else
    echo -e "${RED}✗ ADB installation failed${NC}"
    exit 1
fi

# Create polling script
echo -e "\n${BLUE}[2/4] Creating RayHunter polling script...${NC}"

mkdir -p "$INSTALL_DIR"
cat > "$INSTALL_DIR/rayhunter-poll.sh" << 'EOF'
#!/bin/bash
# RayHunter Status Poller for SIGINT-Pi
# Polls RayHunter via ADB and outputs JSON status

# Check ADB connection
if ! adb devices 2>/dev/null | grep -q "device$"; then
    echo '{"connected":false,"error":"No ADB device connected"}'
    exit 1
fi

# Get latest analysis file
LATEST=$(adb shell "ls -t /data/rayhunter/qmdl/*.ndjson 2>/dev/null | head -1" | tr -d "\r\n")

if [ -z "$LATEST" ]; then
    echo '{"connected":true,"analyzing":false,"error":"No analysis files found"}'
    exit 0
fi

# Get last analysis entry
LAST=$(adb shell "tail -1 $LATEST 2>/dev/null" | tr -d "\r")

# Check if RayHunter process is running
PID=$(adb shell "pgrep rayhunter" 2>/dev/null | tr -d "\r\n")

# Output JSON
echo "{\"connected\":true,\"running\":$([ -n \"$PID\" ] && echo true || echo false),\"latest_file\":\"$LATEST\",\"last_analysis\":$LAST}"
EOF
chmod +x "$INSTALL_DIR/rayhunter-poll.sh"

# Create ADB connection maintainer script
echo -e "\n${BLUE}[3/4] Creating ADB connection maintainer...${NC}"

cat > "$INSTALL_DIR/rayhunter-adb.sh" << 'EOF'
#!/bin/bash
# RayHunter ADB Connection Maintainer
# Keeps ADB connected and port forwarding active

LOG_TAG="rayhunter-adb"

log() {
    logger -t "$LOG_TAG" "$1"
    echo "$1"
}

while true; do
    # Start ADB server
    adb start-server 2>/dev/null
    
    # Wait for device
    log "Waiting for RayHunter device..."
    adb wait-for-device
    
    # Setup port forwarding (RayHunter web UI)
    adb forward tcp:8082 tcp:8080 2>/dev/null
    
    log "RayHunter ADB connected"
    
    # Monitor device connection
    while adb devices 2>/dev/null | grep -q "device$"; do
        sleep 10
    done
    
    log "RayHunter ADB disconnected, reconnecting..."
    sleep 5
done
EOF
chmod +x "$INSTALL_DIR/rayhunter-adb.sh"

# Create systemd service
echo -e "\n${BLUE}[4/4] Creating systemd service...${NC}"

if $SYSTEM_INSTALL; then
    cat > "$SERVICE_DIR/rayhunter-adb.service" << EOF
[Unit]
Description=RayHunter ADB Connection Manager
After=network.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/rayhunter-adb.sh
Restart=always
RestartSec=30

[Install]
WantedBy=multi-user.target
EOF
    sudo systemctl daemon-reload
    echo -e "${GREEN}✓ System service created${NC}"
    echo "  Enable with: sudo systemctl enable --now rayhunter-adb"
else
    cat > "$SERVICE_DIR/rayhunter-adb.service" << EOF
[Unit]
Description=RayHunter ADB Connection Manager
After=network.target

[Service]
Type=simple
ExecStart=$INSTALL_DIR/rayhunter-adb.sh
Restart=always
RestartSec=30

[Install]
WantedBy=default.target
EOF
    systemctl --user daemon-reload
    echo -e "${GREEN}✓ User service created${NC}"
    echo "  Enable with: systemctl --user enable --now rayhunter-adb"
fi

# Summary
echo -e "\n${GREEN}╔═══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║      RayHunter Integration Complete!          ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo ""
echo "1. Install RayHunter on your Pixel phone:"
echo "   https://github.com/EFForg/rayhunter"
echo ""
echo "2. Enable USB debugging on the phone:"
echo "   Settings → Developer Options → USB Debugging"
echo ""
echo "3. Connect phone via USB and authorize debugging"
echo ""
echo "4. Test connection:"
echo "   adb devices"
echo ""
echo "5. Enable the service:"
if $SYSTEM_INSTALL; then
    echo "   sudo systemctl enable --now rayhunter-adb"
else
    echo "   systemctl --user enable --now rayhunter-adb"
fi
echo ""
echo "6. Test the polling script:"
echo "   $INSTALL_DIR/rayhunter-poll.sh"
echo ""
echo -e "${BLUE}The IMSI tab in SIGINT-Pi will show RayHunter status${NC}"
echo ""
