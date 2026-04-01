#!/bin/bash
# Channel hopper for WiFi scanning
# Cycles through 2.4GHz and 5GHz channels
# Requires: sudo -n /usr/bin/iw to work without password
# Setup: Add to /etc/sudoers.d/zzz-sigint-wifi:
#   deck ALL=(ALL) NOPASSWD: /usr/bin/iw
#   Defaults:deck !requiretty

IFACE=${1:-wlan1}
CHANNELS="1 2 3 4 5 6 7 8 9 10 11 36 40 44 48 149 153 157 161 165"

echo "Starting channel hopper on $IFACE"
echo "Channels: $CHANNELS"

while true; do
    for ch in $CHANNELS; do
        sudo -n /usr/bin/iw dev $IFACE set channel $ch 2>/dev/null
        sleep 0.3
    done
done
