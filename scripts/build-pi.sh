#!/bin/bash
set -e

# SIGINT-Pi Raspberry Pi Build Script
# Clones from GitHub and builds for ARM64
# Can be run on the Pi itself (native) or on a Mac/Linux host (Docker cross-compile)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

REPO_URL="https://github.com/naanprofit/sigint-pi.git"
BUILD_DIR="${1:-/tmp/sigint-pi-build}"
INSTALL_DIR="$HOME/sigint-pi"

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Pi Pi Build Script${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

ARCH=$(uname -m)

# ============================================
# Step 1: Clone or update source
# ============================================
echo -e "${YELLOW}[1/4] Fetching source code...${NC}"
if [ -d "$BUILD_DIR/.git" ]; then
    echo -e "${CYAN}Updating existing checkout...${NC}"
    cd "$BUILD_DIR"
    git pull --ff-only 2>/dev/null || git fetch && git reset --hard origin/main
else
    echo -e "${CYAN}Cloning from $REPO_URL...${NC}"
    rm -rf "$BUILD_DIR"
    git clone "$REPO_URL" "$BUILD_DIR"
fi
cd "$BUILD_DIR"
echo -e "${GREEN}Source ready at $BUILD_DIR${NC}"

# ============================================
# Step 2: Build
# ============================================
if [[ "$ARCH" == "aarch64" || "$ARCH" == "armv7l" ]]; then
    # Native build on Pi
    echo -e "${YELLOW}[2/4] Building natively on ARM ($ARCH)...${NC}"
    
    if ! command -v cargo &>/dev/null; then
        echo -e "${CYAN}Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Create .cargo/config.toml to avoid LTO issues on limited hardware
    mkdir -p .cargo
    cat > .cargo/config.toml << 'CFG'
[profile.release]
lto = false
codegen-units = 4
CFG

    echo -e "${CYAN}Running cargo build --release (this may take 10-15 minutes)...${NC}"
    cargo build --release 2>&1 | tail -5
    
    BINARY="target/release/sigint-pi"
    
else
    # Cross-compile via Docker on Mac/Linux
    echo -e "${YELLOW}[2/4] Cross-compiling for ARM64 via Docker...${NC}"
    
    if ! command -v docker &>/dev/null; then
        echo -e "${RED}Docker is required for cross-compilation. Install Docker Desktop first.${NC}"
        exit 1
    fi
    
    echo -e "${CYAN}Building ARM64 binary via Docker (this may take 8-10 minutes first time)...${NC}"
    docker buildx build \
        --platform linux/arm64 \
        -f docker/Dockerfile.pi \
        --target builder \
        -t sigint-pi-builder:latest \
        --load .
    
    # Extract binary
    docker rm pi-extract 2>/dev/null || true
    docker create --name pi-extract sigint-pi-builder:latest
    docker cp pi-extract:/build/target/release/sigint-pi ./sigint-pi-arm64
    docker rm pi-extract
    
    BINARY="sigint-pi-arm64"
fi

echo -e "${GREEN}Binary built: $(ls -lh $BINARY | awk '{print $5}')${NC}"

# ============================================
# Step 3: Install
# ============================================
echo -e "${YELLOW}[3/4] Installing...${NC}"
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$BINARY" "$INSTALL_DIR/sigint-pi"
chmod +x "$INSTALL_DIR/sigint-pi"

# Copy static files
cp -r static/ "$INSTALL_DIR/"

# Copy docs
cp -f LEGAL.md README.md CHANGELOG.md KNOWN_ISSUES.md CREDITS.md "$INSTALL_DIR/" 2>/dev/null || true

# Create default config if none exists
if [ ! -f "$INSTALL_DIR/config.toml" ]; then
    if [ -f config.toml.example ]; then
        cp config.toml.example "$INSTALL_DIR/config.toml"
    fi
fi

# Set capabilities (Pi native only)
if [[ "$ARCH" == "aarch64" || "$ARCH" == "armv7l" ]]; then
    echo -e "${CYAN}Setting network capabilities...${NC}"
    sudo setcap cap_net_raw,cap_net_admin+eip "$INSTALL_DIR/sigint-pi" 2>/dev/null || {
        echo -e "${YELLOW}Could not set capabilities. Run with sudo or set manually:${NC}"
        echo -e "  sudo setcap cap_net_raw,cap_net_admin+eip $INSTALL_DIR/sigint-pi"
    }
    # Add user to SDR device groups
    for grp in plugdev rtlsdr bluetooth dialout; do
        if getent group "$grp" > /dev/null 2>&1; then
            sudo usermod -aG "$grp" "$USER" 2>/dev/null || true
        fi
    done
fi

echo -e "${GREEN}Installed to $INSTALL_DIR${NC}"

# ============================================
# Step 4: Verify
# ============================================
echo -e "${YELLOW}[4/4] Verifying...${NC}"
echo -e "  Binary: $(ls -lh $INSTALL_DIR/sigint-pi | awk '{print $5}')"
echo -e "  Static: $(ls $INSTALL_DIR/static/index.html 2>/dev/null && echo 'OK' || echo 'MISSING')"
echo -e "  Config: $(ls $INSTALL_DIR/config.toml 2>/dev/null && echo 'OK' || echo 'MISSING')"

# ============================================
# Summary
# ============================================
echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Build Complete!${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "Binary:    ${CYAN}$INSTALL_DIR/sigint-pi${NC}"
echo -e "Config:    ${CYAN}$INSTALL_DIR/config.toml${NC}"
echo ""
echo -e "${YELLOW}To start:${NC}"
echo "  cd $INSTALL_DIR && SIGINT_ACCEPT_DISCLAIMER=1 ./sigint-pi"
echo ""
echo -e "${YELLOW}To install as service:${NC}"
echo "  bash $BUILD_DIR/scripts/install-pi.sh"
echo ""
