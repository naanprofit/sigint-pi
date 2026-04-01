#!/bin/bash
set -e

# SIGINT-Deck Steam Deck Build Script
# Clones from GitHub and builds natively on x86_64

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

REPO_URL="https://github.com/naanprofit/sigint-deck.git"
BUILD_DIR="${1:-/tmp/sigint-deck-build}"
INSTALL_DIR="$HOME/sigint-deck"

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Deck Steam Deck Build Script${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

if [[ "$(uname -m)" != "x86_64" ]]; then
    echo -e "${RED}This script is for x86_64. For ARM Pi builds, use build-pi.sh${NC}"
    exit 1
fi

# ============================================
# Step 1: Check/install Rust
# ============================================
echo -e "${YELLOW}[1/5] Checking Rust toolchain...${NC}"
export PATH="$HOME/.cargo/bin:$HOME/bin:$PATH"

if ! command -v cargo &>/dev/null; then
    echo -e "${CYAN}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}Rust installed${NC}"
else
    echo -e "${GREEN}Rust $(rustc --version | awk '{print $2}') found${NC}"
fi

# ============================================
# Step 2: Clone or update source
# ============================================
echo -e "${YELLOW}[2/5] Fetching source code...${NC}"
if [ -d "$BUILD_DIR/.git" ]; then
    echo -e "${CYAN}Updating existing checkout...${NC}"
    cd "$BUILD_DIR"
    git pull --ff-only 2>/dev/null || { git fetch && git reset --hard origin/main; }
else
    echo -e "${CYAN}Cloning from $REPO_URL...${NC}"
    rm -rf "$BUILD_DIR"
    git clone "$REPO_URL" "$BUILD_DIR"
fi
cd "$BUILD_DIR"
echo -e "${GREEN}Source ready at $BUILD_DIR${NC}"

# ============================================
# Step 3: Build
# ============================================
echo -e "${YELLOW}[3/5] Building release binary...${NC}"

# Disable LTO for faster builds on Deck hardware
mkdir -p .cargo
cat > .cargo/config.toml << 'CFG'
[profile.release]
lto = false
codegen-units = 4
CFG

echo -e "${CYAN}Running cargo build --release (this takes 3-5 minutes)...${NC}"
cargo build --release 2>&1 | tail -5

BINARY="target/release/sigint-deck"
echo -e "${GREEN}Binary built: $(ls -lh $BINARY | awk '{print $5}')${NC}"

# ============================================
# Step 4: Install
# ============================================
echo -e "${YELLOW}[4/5] Installing...${NC}"
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$BINARY" "$INSTALL_DIR/sigint-deck"
chmod +x "$INSTALL_DIR/sigint-deck"

# Set capabilities
sudo setcap cap_net_raw,cap_net_admin+eip "$INSTALL_DIR/sigint-deck" 2>/dev/null || {
    echo -e "${YELLOW}  Could not set capabilities (need sudo). WiFi monitor mode may not work.${NC}"
}

# Add user to SDR device groups
for grp in rtlsdr plugdev dialout; do
    if getent group "$grp" > /dev/null 2>&1; then
        sudo usermod -aG "$grp" "$USER" 2>/dev/null || true
    fi
done

# Copy static files
cp -r static/ "$INSTALL_DIR/"

# Copy docs
cp -f LEGAL.md README.md CHANGELOG.md KNOWN_ISSUES.md CREDITS.md "$INSTALL_DIR/" 2>/dev/null || true

# Create default config if none exists
if [ ! -f "$INSTALL_DIR/config.toml" ]; then
    cat > "$INSTALL_DIR/config.toml" << 'CONFIG'
[device]
name = "sigint-deck-01"
location_name = "default"

[web]
enabled = true
port = 8085
host = "0.0.0.0"

[wifi]
enabled = true
interface = "wlan1"

[bluetooth]
enabled = true
detect_trackers = true

[gps]
enabled = false

[rayhunter]
enabled = false

[alerts]
[alerts.sound]
enabled = true
ninja_mode = true
volume = 50

[database]
path = "sigint.db"
CONFIG
    echo -e "${GREEN}  Default config created${NC}"
fi

echo -e "${GREEN}Installed to $INSTALL_DIR${NC}"

# ============================================
# Step 5: Setup user service
# ============================================
echo -e "${YELLOW}[5/5] Setting up systemd user service...${NC}"
mkdir -p ~/.config/systemd/user

cat > ~/.config/systemd/user/sigint-deck.service << EOF
[Unit]
Description=SIGINT-Deck Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
Environment=PATH=$HOME/.cargo/bin:$HOME/bin:/usr/local/bin:/usr/bin:/bin
Environment=LD_LIBRARY_PATH=$HOME/.local/lib:$HOME/bin/lib
ExecStart=$INSTALL_DIR/sigint-deck
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user enable sigint-deck
loginctl enable-linger $USER 2>/dev/null || true
echo -e "${GREEN}  Service installed and enabled${NC}"

# ============================================
# Summary
# ============================================
echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Build Complete!${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "Binary:    ${CYAN}$INSTALL_DIR/sigint-deck${NC}"
echo -e "Config:    ${CYAN}$INSTALL_DIR/config.toml${NC}"
echo -e "Web UI:    ${CYAN}http://$(hostname -I 2>/dev/null | awk '{print $1}'):8085${NC}"
echo ""
echo -e "${YELLOW}To start:${NC}"
echo "  systemctl --user start sigint-deck"
echo ""
echo -e "${YELLOW}To view logs:${NC}"
echo "  journalctl --user -u sigint-deck -f"
echo ""
echo -e "${YELLOW}To stop:${NC}"
echo "  systemctl --user stop sigint-deck"
echo ""
