#!/bin/bash
#
# Enable monitor mode on a wireless interface
#
# Usage: ./wifi-monitor.sh <interface> [channel]
# Example: ./wifi-monitor.sh wlan1 6

set -e

INTERFACE="${1:-wlan1}"
CHANNEL="${2:-}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check root
if [[ $EUID -ne 0 ]]; then
    log_error "This script must be run as root (sudo)"
    exit 1
fi

# Check if iw is available
if ! command -v iw &> /dev/null; then
    log_error "'iw' command not found"
    exit 1
fi

# Check if interface exists
if [[ ! -d "/sys/class/net/$INTERFACE" ]]; then
    log_error "Interface '$INTERFACE' not found"
    echo ""
    echo "Available interfaces:"
    ls -1 /sys/class/net/ | grep -E "^wlan|^wlx|^wlp" || echo "  (none)"
    exit 1
fi

# Get phy
PHY=$(cat "/sys/class/net/$INTERFACE/phy80211/name" 2>/dev/null || echo "")
if [[ -z "$PHY" ]]; then
    log_error "Could not determine phy for $INTERFACE"
    exit 1
fi

# Check monitor mode support
if ! iw phy "$PHY" info 2>/dev/null | grep -q "* monitor"; then
    log_error "Interface $INTERFACE ($PHY) does not support monitor mode"
    
    if [[ "$INTERFACE" == "wlan0" ]]; then
        echo ""
        log_warn "wlan0 is typically the internal WiFi adapter."
        log_warn "On Steam Deck, it does NOT support monitor mode."
        echo ""
        echo "Connect an external USB WiFi adapter and try again."
    fi
    exit 1
fi

echo "=========================================="
echo "  Enabling Monitor Mode"
echo "=========================================="
echo ""
echo "  Interface: $INTERFACE"
echo "  PHY:       $PHY"
if [[ -n "$CHANNEL" ]]; then
    echo "  Channel:   $CHANNEL"
fi
echo ""

# Check current mode
current_mode=$(iw dev "$INTERFACE" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")
log_info "Current mode: $current_mode"

if [[ "$current_mode" == "monitor" ]]; then
    log_info "Interface is already in monitor mode"
    
    if [[ -n "$CHANNEL" ]]; then
        log_info "Setting channel to $CHANNEL..."
        iw dev "$INTERFACE" set channel "$CHANNEL"
    fi
    
    exit 0
fi

# Kill interfering processes
log_info "Stopping interfering processes..."
if command -v airmon-ng &> /dev/null; then
    airmon-ng check kill 2>/dev/null || true
else
    # Manual process killing
    pkill -9 wpa_supplicant 2>/dev/null || true
    pkill -9 NetworkManager 2>/dev/null || true
fi

# Bring interface down
log_info "Bringing down $INTERFACE..."
ip link set "$INTERFACE" down

# Set monitor mode
log_info "Setting monitor mode..."
if ! iw dev "$INTERFACE" set type monitor; then
    log_error "Failed to set monitor mode"
    ip link set "$INTERFACE" up
    exit 1
fi

# Bring interface up
log_info "Bringing up $INTERFACE..."
ip link set "$INTERFACE" up

# Set channel if specified
if [[ -n "$CHANNEL" ]]; then
    log_info "Setting channel to $CHANNEL..."
    iw dev "$INTERFACE" set channel "$CHANNEL"
fi

# Verify
new_mode=$(iw dev "$INTERFACE" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")

if [[ "$new_mode" == "monitor" ]]; then
    echo ""
    log_info "Monitor mode enabled successfully!"
    echo ""
    iw dev "$INTERFACE" info
else
    log_error "Failed to verify monitor mode"
    exit 1
fi
