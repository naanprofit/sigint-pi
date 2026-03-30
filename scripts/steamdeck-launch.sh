#!/bin/bash
#
# SIGINT-Pi Steam Deck Launcher
# 
# This script can be added as a non-Steam app in Steam to launch from Game Mode.
#
# Installation:
#   1. Copy this script to ~/bin/ or similar
#   2. Make executable: chmod +x ~/bin/steamdeck-launch.sh
#   3. In Steam Desktop Mode: Games -> Add Non-Steam Game
#   4. Browse to this script and add it
#   5. Optionally set a custom icon
#
# The script will:
#   1. Verify required directories exist
#   2. Start the container if not running
#   3. Wait for the web UI to become available
#   4. Open a browser window to the dashboard

set -e

# Configuration
CONTAINER_NAME="sigint-pi"
WEB_PORT=8080
WEB_URL="http://127.0.0.1:${WEB_PORT}"
DATA_DIR="${HOME}/.local/share/sigint-pi/data"
CONFIG_DIR="${HOME}/.config/sigint-pi"
COMPOSE_FILE="${HOME}/.local/share/sigint-pi/podman-compose.steamdeck.yml"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on Steam Deck
check_platform() {
    if [[ -f /etc/os-release ]] && grep -q "steamdeck" /etc/os-release; then
        log_info "Running on Steam Deck"
    else
        log_warn "Not running on Steam Deck - some features may not work"
    fi
}

# Ensure required directories exist
ensure_directories() {
    log_info "Checking directories..."
    
    if [[ ! -d "$DATA_DIR" ]]; then
        log_info "Creating data directory: $DATA_DIR"
        mkdir -p "$DATA_DIR"
    fi
    
    if [[ ! -d "$CONFIG_DIR" ]]; then
        log_info "Creating config directory: $CONFIG_DIR"
        mkdir -p "$CONFIG_DIR"
    fi
    
    # Copy default config if not present
    if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
        if [[ -f "${SCRIPT_DIR}/../config.toml.example" ]]; then
            log_info "Copying default config..."
            cp "${SCRIPT_DIR}/../config.toml.example" "$CONFIG_DIR/config.toml"
        elif [[ -f "/etc/sigint-pi/config.toml" ]]; then
            cp "/etc/sigint-pi/config.toml" "$CONFIG_DIR/config.toml"
        else
            log_warn "No default config found - container will use built-in defaults"
        fi
    fi
}

# Check if container is running
is_container_running() {
    podman ps --format "{{.Names}}" 2>/dev/null | grep -q "^${CONTAINER_NAME}$"
}

# Start the container
start_container() {
    log_info "Starting SIGINT-Pi container..."
    
    # Check if compose file exists locally
    if [[ -f "$COMPOSE_FILE" ]]; then
        cd "$(dirname "$COMPOSE_FILE")"
        podman-compose -f "$COMPOSE_FILE" up -d sigint-pi-sim
    else
        # Fall back to direct podman run
        log_info "Using direct podman run..."
        podman run -d \
            --name "$CONTAINER_NAME" \
            --replace \
            -p "${WEB_PORT}:8080" \
            -v "${DATA_DIR}:/data:Z" \
            -v "${CONFIG_DIR}:/etc/sigint-pi:Z" \
            -e SIGINT_PLATFORM=steamdeck \
            -e SIGINT_SIMULATION=1 \
            -e RUST_LOG=info \
            sigint-pi:steamdeck
    fi
}

# Wait for web UI to be available
wait_for_webui() {
    log_info "Waiting for web UI to start..."
    
    local max_attempts=30
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        if curl -s --connect-timeout 1 "${WEB_URL}/api/status" > /dev/null 2>&1; then
            log_info "Web UI is ready!"
            return 0
        fi
        
        attempt=$((attempt + 1))
        sleep 1
        echo -n "."
    done
    
    echo ""
    log_error "Web UI did not start within ${max_attempts} seconds"
    return 1
}

# Open browser to dashboard
open_browser() {
    log_info "Opening dashboard in browser..."
    
    # Try various browsers available on Steam Deck
    if command -v xdg-open &> /dev/null; then
        xdg-open "$WEB_URL" &
    elif command -v firefox &> /dev/null; then
        firefox "$WEB_URL" &
    elif command -v chromium &> /dev/null; then
        chromium "$WEB_URL" &
    else
        log_warn "No browser found - please open manually: $WEB_URL"
    fi
}

# Main function
main() {
    echo "=========================================="
    echo "  SIGINT-Pi Steam Deck Launcher"
    echo "=========================================="
    echo ""
    
    check_platform
    ensure_directories
    
    if is_container_running; then
        log_info "Container is already running"
    else
        start_container
    fi
    
    if wait_for_webui; then
        open_browser
        
        echo ""
        log_info "SIGINT-Pi is running!"
        echo ""
        echo "  Dashboard: $WEB_URL"
        echo "  Stop with: ~/bin/steamdeck-stop.sh"
        echo ""
        
        # Keep script running briefly to show output
        sleep 3
    else
        log_error "Failed to start SIGINT-Pi"
        echo ""
        echo "Check logs with: podman logs $CONTAINER_NAME"
        
        # Show logs on failure
        podman logs --tail 20 "$CONTAINER_NAME" 2>/dev/null || true
        
        sleep 10
        exit 1
    fi
}

main "$@"
