#!/bin/bash
# SIGINT-Deck Steam Launch Script
# Add this as a non-Steam game to launch from Gaming Mode
#
# Setup:
# 1. In Desktop Mode, open Steam
# 2. Games -> Add a Non-Steam Game
# 3. Browse to: /home/deck/sigint-pi/launch-in-steam.sh
# 4. Add the game
# 5. Right-click -> Properties -> Set a nice name like "SIGINT-Deck"
# 6. Optionally set a custom icon
#
# This script:
# - Ensures services are running
# - Opens the dashboard in Steam's web browser
# - Keeps running so Steam doesn't think the game crashed

SIGINT_DIR="/home/deck/sigint-pi"
DASHBOARD_URL="http://localhost:8080"

# Function to check if service is running
check_service() {
    systemctl --user is-active --quiet "$1"
}

# Function to start service if not running
ensure_service() {
    if ! check_service "$1"; then
        echo "Starting $1..."
        systemctl --user start "$1"
        sleep 2
    fi
}

# Ensure SIGINT-Deck services are running
echo "Starting SIGINT-Deck..."
ensure_service "sigint-pi"
ensure_service "channel-hop"

# Wait for web server to be ready
echo "Waiting for dashboard..."
for i in {1..30}; do
    if curl -s "$DASHBOARD_URL/api/status" > /dev/null 2>&1; then
        echo "Dashboard ready!"
        break
    fi
    sleep 1
done

# Open dashboard in Steam's web browser
# Steam uses CEF (Chromium Embedded Framework)
echo "Opening dashboard..."

# Method 1: Use xdg-open (works in Desktop Mode)
if [ -n "$DISPLAY" ] || [ -n "$WAYLAND_DISPLAY" ]; then
    xdg-open "$DASHBOARD_URL" 2>/dev/null &
fi

# Method 2: For Gaming Mode, use steam://openurl
# This opens Steam's built-in browser overlay
steam "steam://openurl/$DASHBOARD_URL" 2>/dev/null &

# Keep the script running so Steam doesn't exit
# Also display status in the terminal/Steam overlay
echo ""
echo "========================================"
echo "  SIGINT-Deck Dashboard"
echo "========================================"
echo ""
echo "Dashboard: $DASHBOARD_URL"
echo ""
echo "Press Steam button to access overlay browser"
echo "Or use a web browser at the URL above"
echo ""
echo "Status:"

# Show live status updates
while true; do
    STATUS=$(curl -s "$DASHBOARD_URL/api/hardware/status" 2>/dev/null)
    if [ -n "$STATUS" ]; then
        WIFI=$(echo "$STATUS" | grep -o '"wifi":[^,]*' | cut -d: -f2)
        BLE=$(echo "$STATUS" | grep -o '"ble":[^,]*' | cut -d: -f2)
        GPS=$(echo "$STATUS" | grep -o '"gps":[^,]*' | cut -d: -f2)
        BATT=$(echo "$STATUS" | grep -o '"battery":[^,}]*' | cut -d: -f2)
        
        # Get device counts
        STATS=$(curl -s "$DASHBOARD_URL/api/stats" 2>/dev/null)
        WIFI_COUNT=$(echo "$STATS" | grep -o '"wifi_devices":[0-9]*' | cut -d: -f2)
        BLE_COUNT=$(echo "$STATS" | grep -o '"ble_devices":[0-9]*' | cut -d: -f2)
        
        clear
        echo "========================================"
        echo "  SIGINT-Deck Status"
        echo "========================================"
        echo ""
        echo "  WiFi:     $WIFI  (${WIFI_COUNT:-0} devices)"
        echo "  BLE:      $BLE  (${BLE_COUNT:-0} devices)"
        echo "  GPS:      $GPS"
        echo "  Battery:  ${BATT:-?}%"
        echo ""
        echo "  Dashboard: $DASHBOARD_URL"
        echo ""
        echo "  Press Ctrl+C or Steam button to exit"
        echo "========================================"
    fi
    sleep 5
done
