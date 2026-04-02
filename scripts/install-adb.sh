#!/bin/bash
# SIGINT-Deck ADB Installer for RayHunter Integration
# Installs Android Debug Bridge to ~/bin for IMSI catcher detection
#
# Usage: ./install-adb.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="$HOME/bin"
TEMP_DIR="$HOME/.adb-install-tmp"

# Android SDK Platform Tools URL
PLATFORM_TOOLS_URL="https://dl.google.com/android/repository/platform-tools-latest-linux.zip"

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════╗"
echo "║     SIGINT-Deck ADB Installer                 ║"
echo "║   For RayHunter IMSI Catcher Detection        ║"
echo "╚═══════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if already installed
if [ -f "$INSTALL_DIR/adb" ]; then
    echo -e "${GREEN}ADB already installed at $INSTALL_DIR/adb${NC}"
    "$INSTALL_DIR/adb" version | head -1
    echo ""
    read -p "Reinstall? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

# Create directories
mkdir -p "$INSTALL_DIR" "$TEMP_DIR"
cd "$TEMP_DIR"

# Download
echo -e "${BLUE}Downloading Android Platform Tools...${NC}"
wget -q --show-progress "$PLATFORM_TOOLS_URL" -O platform-tools.zip

# Extract
echo -e "${BLUE}Extracting...${NC}"
unzip -q platform-tools.zip

# Install
echo -e "${BLUE}Installing to $INSTALL_DIR...${NC}"
cp platform-tools/adb "$INSTALL_DIR/"
cp platform-tools/fastboot "$INSTALL_DIR/"
cp -r platform-tools/lib64 "$INSTALL_DIR/" 2>/dev/null || true
chmod +x "$INSTALL_DIR/adb" "$INSTALL_DIR/fastboot"

# Cleanup
rm -rf "$TEMP_DIR"

# Update PATH if needed
if ! grep -q 'PATH.*\$HOME/bin' ~/.bashrc 2>/dev/null; then
    echo -e "\n${BLUE}Adding ~/bin to PATH...${NC}"
    echo '' >> ~/.bashrc
    echo '# ADB and other local binaries' >> ~/.bashrc
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
fi

# Create RayHunter polling script
echo -e "${BLUE}Creating RayHunter integration scripts...${NC}"

# Detect project directory
SIGINT_DIR="$HOME/sigint-pi"
[ -d "$HOME/sigint-deck" ] && SIGINT_DIR="$HOME/sigint-deck"
mkdir -p "$SIGINT_DIR"

cat > "$SIGINT_DIR/rayhunter-poll.sh" << 'EOF'
#!/bin/bash
# RayHunter Status Poller for SIGINT-Deck
export PATH="$HOME/bin:$PATH"

# Check ADB connection
if ! adb devices 2>/dev/null | grep -q "device$"; then
    echo '{"connected":false,"error":"No ADB device"}'
    exit 1
fi

# Get latest analysis file
LATEST=$(adb shell "ls -t /data/rayhunter/qmdl/*.ndjson 2>/dev/null | head -1" | tr -d "\r\n")
if [ -z "$LATEST" ]; then
    echo '{"connected":true,"analyzing":false}'
    exit 0
fi

# Get last entry
LAST=$(adb shell "tail -1 $LATEST 2>/dev/null" | tr -d "\r")

# Check if running
PID=$(adb shell "pgrep rayhunter" 2>/dev/null | tr -d "\r\n")

echo "{\"connected\":true,\"running\":$([ -n \"$PID\" ] && echo true || echo false),\"latest_file\":\"$LATEST\",\"last_analysis\":$LAST}"
EOF
chmod +x "$SIGINT_DIR/rayhunter-poll.sh"

# Create ADB connection maintainer
cat > "$SIGINT_DIR/rayhunter-adb.sh" << 'EOF'
#!/bin/bash
# RayHunter ADB Connection Maintainer
export PATH="$HOME/bin:$PATH"

while true; do
    adb start-server 2>/dev/null
    adb wait-for-device
    adb forward tcp:8082 tcp:8080 2>/dev/null
    echo "[$(date)] RayHunter ADB connected"
    
    while adb devices 2>/dev/null | grep -q "device$"; do
        sleep 10
    done
    
    echo "[$(date)] RayHunter ADB disconnected, waiting..."
    sleep 5
done
EOF
chmod +x "$SIGINT_DIR/rayhunter-adb.sh"

# Create systemd user service
mkdir -p "$HOME/.config/systemd/user"
cat > "$HOME/.config/systemd/user/rayhunter-adb.service" << EOF
[Unit]
Description=RayHunter ADB Connection Manager
After=network.target

[Service]
Type=simple
ExecStart=$SIGINT_DIR/rayhunter-adb.sh
Restart=always
RestartSec=30

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload

# Verify
echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║         ADB Installation Complete!            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
echo ""
"$INSTALL_DIR/adb" version | head -2
echo ""
echo -e "${YELLOW}Next steps for RayHunter:${NC}"
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
echo "5. Enable the ADB service:"
echo "   systemctl --user enable --now rayhunter-adb"
echo ""
echo "6. The IMSI tab in SIGINT-Deck will show RayHunter status"
echo ""
