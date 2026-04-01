#!/bin/bash
# SIGINT-Pi Steam Deck Setup Script
# Run with: sudo ~/sigint-pi/setup-steamdeck.sh
#
# This script permanently fixes WiFi interface naming and sets up GPS.
#
# PROBLEM: Steam Deck assigns wlan names dynamically, causing the external
# USB WiFi adapter to sometimes steal wlan0 from the internal WiFi,
# breaking your network connection.
#
# SOLUTION: Use systemd .link files to assign persistent names based on
# MAC address. This ensures:
#   - Internal WiFi (ath11k)  -> always wlan0
#   - External USB (mt76x2u)  -> always wlan1
#
# Hardware-specific MACs (edit if your hardware differs):
INTERNAL_MAC="dc:2e:97:2f:8f:f8"  # Steam Deck internal WiFi
EXTERNAL_MAC="9c:ef:d5:f8:95:2d"  # MediaTek MT7612U USB adapter

set -e

echo "========================================"
echo "  SIGINT-Pi Steam Deck Setup"
echo "========================================"
echo ""
echo "Internal WiFi MAC: $INTERNAL_MAC -> wlan0"
echo "External WiFi MAC: $EXTERNAL_MAC -> wlan1"
echo ""

# 1. Create systemd .link files for persistent interface naming
# This is more reliable than udev NAME= rules on modern systemd
echo "[1/6] Creating systemd link files..."
mkdir -p /etc/systemd/network

cat > /etc/systemd/network/10-wlan0-internal.link << EOF
# SIGINT-Pi: Lock internal WiFi to wlan0
[Match]
MACAddress=$INTERNAL_MAC

[Link]
Name=wlan0
EOF

cat > /etc/systemd/network/10-wlan1-external.link << EOF
# SIGINT-Pi: Lock external USB adapter to wlan1
[Match]
MACAddress=$EXTERNAL_MAC

[Link]
Name=wlan1
EOF

# 2. Create udev rules
echo "[2/6] Creating udev rules..."
cat > /etc/udev/rules.d/90-sigint-wifi.rules << EOF
# SIGINT-Pi: Mark external WiFi adapter as unmanaged by NetworkManager
# This prevents NM from trying to use it for internet connection
SUBSYSTEM=="net", ATTR{address}=="$EXTERNAL_MAC", ENV{NM_UNMANAGED}="1"
EOF

# GPS udev rule - create symlink and allow user access
cat > /etc/udev/rules.d/91-sigint-gps.rules << EOF
# SIGINT-Pi: U-Blox GPS receiver
# Creates /dev/gps0 symlink and allows user access
SUBSYSTEM=="tty", ATTRS{idVendor}=="1546", ATTRS{idProduct}=="01a7", MODE="0666", SYMLINK+="gps0"
EOF

# 3. Configure NetworkManager to ignore wlan1
echo "[3/6] Configuring NetworkManager..."
mkdir -p /etc/NetworkManager/conf.d
cat > /etc/NetworkManager/conf.d/90-sigint-unmanaged.conf << EOF
[keyfile]
# SIGINT-Pi: Ignore external WiFi adapter
unmanaged-devices=mac:$EXTERNAL_MAC;interface-name:wlan1
EOF

# 4. Create monitor mode service
echo "[4/6] Creating monitor mode service..."
cat > /etc/systemd/system/sigint-monitor-mode.service << 'EOF'
[Unit]
Description=SIGINT-Pi Monitor Mode Setup
After=network.target sys-subsystem-net-devices-wlan1.device
Wants=sys-subsystem-net-devices-wlan1.device

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/home/deck/sigint-pi/set-monitor-mode.sh
ExecStop=/home/deck/sigint-pi/unset-monitor-mode.sh

[Install]
WantedBy=multi-user.target
EOF

# 5. Create the monitor mode scripts
echo "[5/6] Creating helper scripts..."
cat > /home/deck/sigint-pi/set-monitor-mode.sh << 'SETMON'
#!/bin/bash
# Enable monitor mode on wlan1 (external USB adapter)
sleep 2
IFACE="wlan1"

if [ ! -e "/sys/class/net/$IFACE" ]; then
    echo "ERROR: wlan1 not found"
    echo "Make sure external WiFi adapter is plugged in"
    exit 1
fi

echo "Setting $IFACE to monitor mode..."
ip link set $IFACE down 2>/dev/null || true
iw $IFACE set type monitor 2>/dev/null || true
ip link set $IFACE up 2>/dev/null || true

# Update SIGINT-Pi config
sed -i 's/interface = "wlan[0-9]*"/interface = "wlan1"/' /home/deck/sigint-pi/config.toml
echo "Monitor mode enabled on $IFACE"
iw dev $IFACE info | grep type
SETMON
chmod +x /home/deck/sigint-pi/set-monitor-mode.sh

cat > /home/deck/sigint-pi/unset-monitor-mode.sh << 'UNSETMON'
#!/bin/bash
IFACE="wlan1"
[ -e "/sys/class/net/$IFACE" ] || exit 0
ip link set $IFACE down 2>/dev/null || true
iw $IFACE set type managed 2>/dev/null || true
echo "Monitor mode disabled on $IFACE"
UNSETMON
chmod +x /home/deck/sigint-pi/unset-monitor-mode.sh
chown deck:deck /home/deck/sigint-pi/set-monitor-mode.sh /home/deck/sigint-pi/unset-monitor-mode.sh

# 6. Set up GPS permissions and gpsd
echo "[6/7] Setting up GPS..."
usermod -a -G uucp deck 2>/dev/null || true

# Check if gpsd is available
if command -v gpsd &> /dev/null; then
    mkdir -p /etc/default
    cat > /etc/default/gpsd << EOF
# SIGINT-Pi GPS configuration
DEVICES="/dev/ttyACM1"
GPSD_OPTIONS="-n"
START_DAEMON="true"
USBAUTO="false"
EOF
    systemctl enable gpsd 2>/dev/null || true
    echo "  gpsd configured"
else
    echo "  NOTE: gpsd not installed"
    echo "  GPS features require gpsd. Install with:"
    echo "    sudo steamos-readonly disable"
    echo "    sudo pacman -S gpsd"
    echo "    sudo steamos-readonly enable"
fi

# 7. Set up libpcap symlink and binary capabilities
echo "[7/7] Setting up WiFi capture permissions..."

# Create libpcap symlink (binary was compiled against older libpcap)
if [ ! -e /usr/lib/libpcap.so.0.8 ]; then
    ln -sf /usr/lib/libpcap.so.1 /usr/lib/libpcap.so.0.8
    echo "  Created libpcap.so.0.8 symlink"
fi

# Set network capture capabilities on binary
BIN="/home/deck/sigint-pi/sigint-pi-bin"
if [ -f "$BIN" ]; then
    setcap cap_net_raw,cap_net_admin+eip "$BIN"
    echo "  Set network capabilities on binary"
fi

# Apply all changes
echo ""
echo "Applying changes..."
udevadm control --reload-rules
udevadm trigger --subsystem-match=net
udevadm trigger --subsystem-match=tty
systemctl daemon-reload
systemctl restart systemd-networkd 2>/dev/null || true
systemctl restart NetworkManager 2>/dev/null || true
systemctl enable sigint-monitor-mode.service

# 7. Set up channel hopping
echo "[7/8] Setting up channel hopping..."

# Create sudoers entry for passwordless iw
cat > /etc/sudoers.d/zzz-sigint-wifi << 'EOF'
# SIGINT-Deck: Allow passwordless iw for channel hopping
deck ALL=(ALL) NOPASSWD: /usr/bin/iw
Defaults:deck !requiretty
EOF
chmod 440 /etc/sudoers.d/zzz-sigint-wifi

# Copy channel hopper script
SCRIPT_DIR="$(dirname "$0")"
if [ -f "$SCRIPT_DIR/channel-hop.sh" ]; then
    cp "$SCRIPT_DIR/channel-hop.sh" /home/deck/sigint-pi/channel-hop.sh
    chmod +x /home/deck/sigint-pi/channel-hop.sh
    chown deck:deck /home/deck/sigint-pi/channel-hop.sh
fi

# Copy Steam launch script
if [ -f "$SCRIPT_DIR/launch-in-steam.sh" ]; then
    cp "$SCRIPT_DIR/launch-in-steam.sh" /home/deck/sigint-pi/launch-in-steam.sh
    chmod +x /home/deck/sigint-pi/launch-in-steam.sh
    chown deck:deck /home/deck/sigint-pi/launch-in-steam.sh
    echo "  Steam launch script installed: ~/sigint-pi/launch-in-steam.sh"
fi

# 8. Set up channel-hop user service
echo "[8/8] Setting up channel-hop service..."
mkdir -p /home/deck/.config/systemd/user
cat > /home/deck/.config/systemd/user/channel-hop.service << 'EOF'
[Unit]
Description=WiFi Channel Hopper for SIGINT-Deck
After=sigint-pi.service

[Service]
Type=simple
ExecStart=/home/deck/sigint-pi/channel-hop.sh wlan1
Restart=always
RestartSec=3

[Install]
WantedBy=default.target
EOF
chown deck:deck /home/deck/.config/systemd/user/channel-hop.service

echo ""
echo "========================================"
echo "  Setup Complete!"
echo "========================================"
echo ""
echo "IMPORTANT: Unplug and replug the external WiFi adapter now!"
echo ""
echo "After replugging, verify with:"
echo "  ip link show wlan0   # Should be internal (ath11k)"
echo "  ip link show wlan1   # Should be external (mt76x2u)"
echo ""
echo "Then enable monitor mode and channel hopping:"
echo "  sudo systemctl start sigint-monitor-mode"
echo "  systemctl --user daemon-reload"
echo "  systemctl --user enable --now channel-hop"
echo "  systemctl --user restart sigint-pi"
echo ""
echo "Interface mapping (permanent):"
echo "  wlan0 = Internal WiFi (dc:2e:97:2f:8f:f8) - network connection"
echo "  wlan1 = External USB (9c:ef:d5:f8:95:2d) - monitor mode capture"
echo ""
echo "Verify channel hopping:"
echo "  for i in {1..10}; do iw dev wlan1 info | grep channel; sleep 0.4; done"
echo ""
echo "To add to Steam (Gaming Mode):"
echo "  1. Open Steam in Desktop Mode"
echo "  2. Games -> Add a Non-Steam Game"
echo "  3. Browse to: /home/deck/sigint-pi/launch-in-steam.sh"
echo "  4. Rename to 'SIGINT-Deck' in properties"
echo ""
