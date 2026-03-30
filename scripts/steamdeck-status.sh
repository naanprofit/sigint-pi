#!/bin/bash
#
# SIGINT-Pi Steam Deck Status Script
#
# Shows runtime and hardware status

set -e

CONTAINER_NAME="sigint-pi"
WEB_URL="http://127.0.0.1:8080"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
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

section() {
    echo ""
    echo -e "${CYAN}=== $1 ===${NC}"
}

# Check if container is running
is_container_running() {
    podman ps --format "{{.Names}}" 2>/dev/null | grep -q "^${CONTAINER_NAME}$"
}

main() {
    echo "=========================================="
    echo "  SIGINT-Pi Status"
    echo "=========================================="
    
    section "Container Status"
    if is_container_running; then
        echo -e "  Container: ${GREEN}Running${NC}"
        podman ps --filter "name=${CONTAINER_NAME}" --format "  Image: {{.Image}}\n  Created: {{.Created}}\n  Status: {{.Status}}"
    else
        echo -e "  Container: ${RED}Not running${NC}"
    fi
    
    section "Web UI"
    if curl -s --connect-timeout 2 "${WEB_URL}/api/status" > /dev/null 2>&1; then
        echo -e "  Status: ${GREEN}Available${NC}"
        echo "  URL: ${WEB_URL}"
        
        # Get app status
        status=$(curl -s "${WEB_URL}/api/status" 2>/dev/null)
        if [[ -n "$status" ]]; then
            echo "  App Status: $status"
        fi
    else
        echo -e "  Status: ${RED}Unavailable${NC}"
    fi
    
    section "Wireless Interfaces"
    echo "  Interfaces:"
    for iface in /sys/class/net/wlan*; do
        if [[ -d "$iface" ]]; then
            name=$(basename "$iface")
            mode=$(iw dev "$name" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")
            echo "    - $name: $mode"
        fi
    done
    
    # Check for monitor mode support
    echo ""
    echo "  Monitor Mode Support:"
    for phy in /sys/class/ieee80211/phy*; do
        if [[ -d "$phy" ]]; then
            phy_name=$(basename "$phy")
            if iw phy "$phy_name" info 2>/dev/null | grep -q "* monitor"; then
                echo -e "    - $phy_name: ${GREEN}Supported${NC}"
            else
                echo -e "    - $phy_name: ${RED}Not supported${NC}"
            fi
        fi
    done
    
    section "Bluetooth"
    if command -v bluetoothctl &> /dev/null; then
        bt_status=$(bluetoothctl show 2>/dev/null | grep "Powered" | awk '{print $2}')
        if [[ "$bt_status" == "yes" ]]; then
            echo -e "  Status: ${GREEN}Available${NC}"
            bluetoothctl show 2>/dev/null | grep -E "Name:|Powered:" | sed 's/^/  /'
        else
            echo -e "  Status: ${YELLOW}Powered off${NC}"
        fi
    else
        echo -e "  Status: ${RED}Not available${NC}"
    fi
    
    section "GPS"
    if [[ -e /dev/ttyUSB0 ]]; then
        echo -e "  USB GPS: ${GREEN}/dev/ttyUSB0 present${NC}"
    elif [[ -e /dev/ttyACM0 ]]; then
        echo -e "  USB GPS: ${GREEN}/dev/ttyACM0 present${NC}"
    else
        echo -e "  USB GPS: ${YELLOW}Not detected${NC}"
    fi
    
    if systemctl is-active --quiet gpsd 2>/dev/null; then
        echo -e "  gpsd: ${GREEN}Running${NC}"
    else
        echo -e "  gpsd: ${YELLOW}Not running${NC}"
    fi
    
    section "Power"
    if [[ -f /sys/class/power_supply/BAT1/capacity ]]; then
        capacity=$(cat /sys/class/power_supply/BAT1/capacity)
        status=$(cat /sys/class/power_supply/BAT1/status)
        echo "  Battery: ${capacity}% (${status})"
    else
        echo "  Battery: Unknown"
    fi
    
    section "USB Devices"
    echo "  Connected:"
    lsusb 2>/dev/null | grep -v "Linux Foundation" | sed 's/^/    /' || echo "    Unable to list USB devices"
    
    echo ""
    echo "=========================================="
}

main "$@"
