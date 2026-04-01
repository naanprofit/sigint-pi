#!/bin/bash
# SIGINT-Pi Docker Development Helper
# Quick commands for multi-arch development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

show_help() {
    cat << EOF
SIGINT-Pi Docker Development Helper

Usage: $0 <command> [options]

Commands:
  setup         Setup Docker buildx for multi-arch builds
  pi            Start Pi development environment (ARM64)
  deck          Start Steam Deck development environment (x86_64)
  both          Start both environments
  build-pi      Build Pi release image
  build-deck    Build Steam Deck release image
  test          Run tests
  shell <env>   Open shell in container (pi or deck)
  logs <env>    Follow container logs
  stop          Stop all containers
  clean         Stop and remove all containers/volumes
  status        Show container status

Examples:
  $0 setup              # First-time setup
  $0 deck               # Start Steam Deck dev
  $0 shell pi           # Open shell in Pi container
  $0 build-deck         # Build release for Steam Deck
EOF
}

check_docker() {
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    if ! docker info &> /dev/null; then
        log_error "Docker daemon is not running"
        exit 1
    fi
}

cmd_setup() {
    log_info "Setting up Docker buildx for multi-architecture builds..."
    
    # Check for existing builder
    if docker buildx inspect sigint-multiarch &>/dev/null; then
        log_warn "Builder 'sigint-multiarch' already exists"
    else
        docker buildx create --use --name sigint-multiarch --driver docker-container
        log_success "Created buildx builder 'sigint-multiarch'"
    fi
    
    docker buildx inspect --bootstrap
    
    # Install QEMU for ARM64 emulation
    log_info "Ensuring QEMU is available for ARM64 emulation..."
    docker run --privileged --rm tonistiigi/binfmt --install arm64 2>/dev/null || true
    
    log_success "Setup complete!"
    echo ""
    echo "Available platforms:"
    docker buildx inspect | grep Platforms
    echo ""
    echo "You can now run:"
    echo "  $0 deck    # Start Steam Deck development"
    echo "  $0 pi      # Start Pi development (emulated)"
}

cmd_pi() {
    log_info "Starting Pi Zero 2 W development environment (ARM64 emulated)..."
    log_warn "Note: ARM64 emulation on x86 Mac can be slow. Use for testing only."
    docker compose -f docker-compose.dev.yml up pi-dev
}

cmd_deck() {
    log_info "Starting Steam Deck development environment (x86_64 native)..."
    docker compose -f docker-compose.dev.yml up deck-dev
}

cmd_both() {
    log_info "Starting both development environments..."
    docker compose -f docker-compose.dev.yml up pi-dev deck-dev
}

cmd_build_pi() {
    log_info "Building Pi release image (ARM64)..."
    docker buildx build \
        --platform linux/arm64 \
        --target runtime \
        -f docker/Dockerfile.pi \
        -t sigint-pi:pi-latest \
        --load \
        .
    log_success "Pi image built: sigint-pi:pi-latest"
}

cmd_build_deck() {
    log_info "Building Steam Deck release image (x86_64)..."
    docker buildx build \
        --platform linux/amd64 \
        --target runtime \
        -f docker/Dockerfile.steamdeck \
        -t sigint-pi:deck-latest \
        --load \
        .
    log_success "Steam Deck image built: sigint-pi:deck-latest"
}

cmd_test() {
    log_info "Running tests..."
    docker compose -f docker-compose.dev.yml run --rm test-amd64
}

cmd_shell() {
    local env="${1:-deck}"
    case "$env" in
        pi)
            docker compose -f docker-compose.dev.yml exec pi-dev bash
            ;;
        deck)
            docker compose -f docker-compose.dev.yml exec deck-dev bash
            ;;
        *)
            log_error "Unknown environment: $env (use 'pi' or 'deck')"
            exit 1
            ;;
    esac
}

cmd_logs() {
    local env="${1:-all}"
    case "$env" in
        pi)
            docker compose -f docker-compose.dev.yml logs -f pi-dev
            ;;
        deck)
            docker compose -f docker-compose.dev.yml logs -f deck-dev
            ;;
        *)
            docker compose -f docker-compose.dev.yml logs -f
            ;;
    esac
}

cmd_stop() {
    log_info "Stopping all containers..."
    docker compose -f docker-compose.dev.yml down
    log_success "All containers stopped"
}

cmd_clean() {
    log_info "Stopping containers and removing volumes..."
    docker compose -f docker-compose.dev.yml down -v
    docker compose -f docker-compose.dev.yml --profile full down -v 2>/dev/null || true
    log_success "Cleaned up"
}

cmd_status() {
    echo "=== Container Status ==="
    docker compose -f docker-compose.dev.yml ps
    echo ""
    echo "=== Images ==="
    docker images | grep -E "^sigint-pi|^REPOSITORY" || echo "No sigint-pi images found"
    echo ""
    echo "=== Volumes ==="
    docker volume ls | grep -E "sigint|VOLUME" || echo "No sigint volumes found"
}

# Main
check_docker

case "${1:-help}" in
    setup)      cmd_setup ;;
    pi)         cmd_pi ;;
    deck)       cmd_deck ;;
    both)       cmd_both ;;
    build-pi)   cmd_build_pi ;;
    build-deck) cmd_build_deck ;;
    test)       cmd_test ;;
    shell)      cmd_shell "$2" ;;
    logs)       cmd_logs "$2" ;;
    stop)       cmd_stop ;;
    clean)      cmd_clean ;;
    status)     cmd_status ;;
    help|--help|-h)
        show_help
        ;;
    *)
        log_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac
