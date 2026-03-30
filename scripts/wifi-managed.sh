#!/bin/bash
#
# Restore managed mode on a wireless interface
#
# Usage: ./wifi-managed.sh <interface>
# Example: ./wifi-managed.sh wlan1

set -e

INTERFACE="${1:-wlan1}"

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

# Check if interface exists
if [[ ! -d "/sys/class/net/$INTERFACE" ]]; then
    log_error "Interface '$INTERFACE' not found"
    exit 1
fi

echo "=========================================="
echo "  Restoring Managed Mode"
echo "=========================================="
echo ""
echo "  Interface: $INTERFACE"
echo ""

# Check current mode
current_mode=$(iw dev "$INTERFACE" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")
log_info "Current mode: $current_mode"

if [[ "$current_mode" == "managed" ]]; then
    log_info "Interface is already in managed mode"
    exit 0
fi

# Bring interface down
log_info "Bringing down $INTERFACE..."
ip link set "$INTERFACE" down

# Set managed mode
log_info "Setting managed mode..."
if ! iw dev "$INTERFACE" set type managed; then
    log_error "Failed to set managed mode"
    ip link set "$INTERFACE" up
    exit 1
fi

# Bring interface up
log_info "Bringing up $INTERFACE..."
ip link set "$INTERFACE" up

# Restart NetworkManager if available
if systemctl is-active --quiet NetworkManager; then
    log_info "Restarting NetworkManager..."
    systemctl restart NetworkManager
fi

# Verify
new_mode=$(iw dev "$INTERFACE" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")

if [[ "$new_mode" == "managed" ]]; then
    echo ""
    log_info "Managed mode restored successfully!"
    echo ""
    iw dev "$INTERFACE" info
else
    log_error "Failed to verify managed mode"
    exit 1
fi
