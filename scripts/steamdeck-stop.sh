#!/bin/bash
#
# SIGINT-Pi Steam Deck Stop Script
#
# Cleanly stops the SIGINT-Pi container

set -e

CONTAINER_NAME="sigint-pi"

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

# Check if container is running
is_container_running() {
    podman ps --format "{{.Names}}" 2>/dev/null | grep -q "^${CONTAINER_NAME}$"
}

main() {
    echo "=========================================="
    echo "  SIGINT-Pi Stop"
    echo "=========================================="
    echo ""
    
    if is_container_running; then
        log_info "Stopping SIGINT-Pi container..."
        podman stop "$CONTAINER_NAME"
        log_info "Container stopped"
        
        # Optionally remove the container
        read -p "Remove container? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            podman rm "$CONTAINER_NAME"
            log_info "Container removed"
        fi
    else
        log_warn "Container is not running"
    fi
    
    echo ""
    log_info "SIGINT-Pi stopped"
}

main "$@"
