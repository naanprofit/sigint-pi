#!/bin/bash
# SIGINT-Pi Setup Script for Raspberry Pi Zero 2 W
# Run as root: sudo bash setup.sh

set -e

echo "==================================="
echo "SIGINT-Pi Setup Script"
echo "==================================="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "Please run as root: sudo bash setup.sh"
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
echo "Detected architecture: $ARCH"

# Update system
echo ""
echo "[1/8] Updating system packages..."
apt-get update
apt-get upgrade -y

# Install dependencies
echo ""
echo "[2/8] Installing dependencies..."
apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    libpcap-dev \
    libbluetooth-dev \
    gpsd \
    gpsd-clients \
    wireless-tools \
    iw \
    aircrack-ng \
    tcpdump \
    sqlite3

# Install Rust if not present
echo ""
echo "[3/8] Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust already installed: $(rustc --version)"
fi

# Create directories
echo ""
echo "[4/8] Creating directories..."
mkdir -p /etc/sigint-pi
mkdir -p /var/lib/sigint-pi/pcap
mkdir -p /var/log/sigint-pi

# Copy example config if not exists
if [ ! -f /etc/sigint-pi/config.toml ]; then
    echo "Creating default configuration..."
    cp config.toml.example /etc/sigint-pi/config.toml
    echo "Please edit /etc/sigint-pi/config.toml with your settings"
fi

# Setup GPS daemon
echo ""
echo "[5/8] Configuring GPS daemon..."
cat > /etc/default/gpsd << 'EOF'
# Default settings for gpsd
START_DAEMON="true"
GPSD_OPTIONS="-n"
DEVICES="/dev/ttyUSB0"
USBAUTO="true"
EOF

systemctl enable gpsd
systemctl start gpsd || true

# Setup WiFi adapter for monitor mode
echo ""
echo "[6/8] Configuring WiFi adapter..."
cat > /usr/local/bin/enable-monitor-mode.sh << 'EOF'
#!/bin/bash
# Enable monitor mode on external WiFi adapter

INTERFACE="${1:-wlan1}"

echo "Enabling monitor mode on $INTERFACE..."

# Check if interface exists
if ! ip link show "$INTERFACE" &> /dev/null; then
    echo "Error: Interface $INTERFACE not found"
    echo "Available interfaces:"
    ip link show | grep -E "^[0-9]+:" | awk -F: '{print $2}'
    exit 1
fi

# Stop network manager from managing the interface
if systemctl is-active --quiet NetworkManager; then
    nmcli device set "$INTERFACE" managed no 2>/dev/null || true
fi

# Bring down, set monitor mode, bring up
ip link set "$INTERFACE" down
iw "$INTERFACE" set type monitor
ip link set "$INTERFACE" up

# Verify
MODE=$(iw "$INTERFACE" info | grep type | awk '{print $2}')
if [ "$MODE" = "monitor" ]; then
    echo "Success: $INTERFACE is now in monitor mode"
else
    echo "Warning: Failed to set monitor mode (current: $MODE)"
fi
EOF

chmod +x /usr/local/bin/enable-monitor-mode.sh

# Create systemd service
echo ""
echo "[7/8] Creating systemd service..."
cat > /etc/systemd/system/sigint-pi.service << 'EOF'
[Unit]
Description=SIGINT-Pi Security Monitor
After=network.target gpsd.service
Wants=gpsd.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/sigint-pi
ExecStartPre=/usr/local/bin/enable-monitor-mode.sh wlan1
ExecStart=/opt/sigint-pi/sigint-pi
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Power management
Nice=-10
IOSchedulingClass=realtime

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload

# Create OUI update script
echo ""
echo "[8/8] Creating OUI database update script..."
cat > /usr/local/bin/update-oui.sh << 'EOF'
#!/bin/bash
# Update OUI database from IEEE

OUI_URL="https://standards-oui.ieee.org/oui/oui.txt"
OUI_PATH="/var/lib/sigint-pi/oui.txt"

echo "Downloading OUI database from IEEE..."
curl -s "$OUI_URL" -o "$OUI_PATH.tmp"

if [ $? -eq 0 ]; then
    mv "$OUI_PATH.tmp" "$OUI_PATH"
    echo "OUI database updated: $(wc -l < "$OUI_PATH") lines"
else
    echo "Failed to download OUI database"
    rm -f "$OUI_PATH.tmp"
    exit 1
fi
EOF

chmod +x /usr/local/bin/update-oui.sh

# Download initial OUI database
/usr/local/bin/update-oui.sh || true

# Setup cron for OUI updates
(crontab -l 2>/dev/null; echo "0 4 1 * * /usr/local/bin/update-oui.sh") | crontab -

echo ""
echo "==================================="
echo "Setup Complete!"
echo "==================================="
echo ""
echo "Next steps:"
echo "1. Edit configuration: nano /etc/sigint-pi/config.toml"
echo "2. Build the application: cargo build --release"
echo "3. Install binary: cp target/release/sigint-pi /opt/sigint-pi/"
echo "4. Enable service: systemctl enable sigint-pi"
echo "5. Start service: systemctl start sigint-pi"
echo ""
echo "To check status: systemctl status sigint-pi"
echo "To view logs: journalctl -u sigint-pi -f"
echo ""
echo "Recommended WiFi adapters for monitor mode:"
echo "  - Alfa AWUS036ACH (dual-band, recommended)"
echo "  - Alfa AWUS036ACHM (dual-band)"
echo "  - Panda PAU09 (dual-band, compact)"
echo ""
