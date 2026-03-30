#!/bin/bash
#
# List wireless interfaces and their monitor mode capabilities
#

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "=========================================="
echo "  Wireless Interface Scanner"
echo "=========================================="
echo ""

# Check if iw is available
if ! command -v iw &> /dev/null; then
    echo -e "${RED}Error: 'iw' command not found${NC}"
    echo "Install with: sudo pacman -S iw  (or equivalent)"
    exit 1
fi

# Get list of wireless interfaces
echo -e "${CYAN}Detected Wireless Interfaces:${NC}"
echo ""

for iface in /sys/class/net/*; do
    if [[ -d "$iface/wireless" ]]; then
        name=$(basename "$iface")
        phy=$(cat "$iface/phy80211/name" 2>/dev/null || echo "unknown")
        
        # Get current mode
        mode=$(iw dev "$name" info 2>/dev/null | grep "type" | awk '{print $2}' || echo "unknown")
        
        # Get MAC address
        mac=$(cat "$iface/address" 2>/dev/null || echo "unknown")
        
        # Check driver
        driver=$(basename "$(readlink "$iface/device/driver")" 2>/dev/null || echo "unknown")
        
        # Check if monitor mode is supported
        monitor_support="No"
        if iw phy "$phy" info 2>/dev/null | grep -q "* monitor"; then
            monitor_support="Yes"
        fi
        
        echo -e "  ${GREEN}$name${NC} ($phy)"
        echo "    MAC:      $mac"
        echo "    Driver:   $driver"
        echo "    Mode:     $mode"
        
        if [[ "$monitor_support" == "Yes" ]]; then
            echo -e "    Monitor:  ${GREEN}Supported${NC}"
        else
            echo -e "    Monitor:  ${RED}Not Supported${NC}"
        fi
        
        # Check if this is likely the internal adapter
        if [[ "$name" == "wlan0" ]]; then
            echo -e "    Type:     ${YELLOW}Internal (likely)${NC}"
        else
            echo -e "    Type:     ${GREEN}External (likely)${NC}"
        fi
        
        echo ""
    fi
done

# Summary
echo -e "${CYAN}Summary:${NC}"
echo ""

# Find best interface for monitor mode
best_iface=""
for iface in /sys/class/net/*; do
    if [[ -d "$iface/wireless" ]]; then
        name=$(basename "$iface")
        phy=$(cat "$iface/phy80211/name" 2>/dev/null || echo "")
        
        if [[ -n "$phy" ]] && iw phy "$phy" info 2>/dev/null | grep -q "* monitor"; then
            if [[ "$name" != "wlan0" ]]; then
                best_iface="$name"
                break
            elif [[ -z "$best_iface" ]]; then
                best_iface="$name"
            fi
        fi
    fi
done

if [[ -n "$best_iface" ]]; then
    echo -e "  Recommended interface for capture: ${GREEN}$best_iface${NC}"
    
    if [[ "$best_iface" == "wlan0" ]]; then
        echo ""
        echo -e "  ${YELLOW}Warning: wlan0 is typically the internal WiFi adapter.${NC}"
        echo -e "  ${YELLOW}On Steam Deck, it does NOT support monitor mode.${NC}"
        echo -e "  ${YELLOW}Connect an external USB WiFi adapter for capture.${NC}"
    fi
else
    echo -e "  ${RED}No monitor-mode capable interface found!${NC}"
    echo ""
    echo "  Recommended adapters:"
    echo "    - Alfa AWUS036ACHM (dual-band, excellent Linux support)"
    echo "    - Panda PAU09 (dual-band, compact)"
    echo "    - Alfa AWUS036ACH (dual-band)"
fi

echo ""
