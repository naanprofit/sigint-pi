#!/bin/bash
set -e

# SIGINT-Deck Steam Deck Uninstall Script
# Removes the SIGINT-Deck installation, services, and optionally SDR tools

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

INSTALL_DIR="$HOME/sigint-deck"
BIN_DIR="$HOME/bin"

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Deck Steam Deck Uninstaller${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

# ============================================
# Confirm
# ============================================
echo -e "${YELLOW}This will remove SIGINT-Deck from this system.${NC}"
echo -e "Install directory: ${CYAN}$INSTALL_DIR${NC}"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 0
fi

# ============================================
# Step 1: Stop and disable user services
# ============================================
echo -e "${YELLOW}[1/6] Stopping services...${NC}"
if systemctl --user is-active --quiet sigint-deck.service 2>/dev/null; then
    systemctl --user stop sigint-deck.service
    echo -e "${GREEN}  Stopped sigint-deck.service${NC}"
fi
if systemctl --user is-enabled --quiet sigint-deck.service 2>/dev/null; then
    systemctl --user disable sigint-deck.service
    echo -e "${GREEN}  Disabled sigint-deck.service${NC}"
fi
rm -f ~/.config/systemd/user/sigint-deck.service
systemctl --user daemon-reload 2>/dev/null || true

# Kill any running sigint-deck process
pkill -f sigint-deck 2>/dev/null || true
echo -e "${GREEN}  All services stopped${NC}"

# ============================================
# Step 2: Remove binary and application files
# ============================================
echo -e "${YELLOW}[2/6] Removing application files...${NC}"
if [ -d "$INSTALL_DIR" ]; then
    if [ -f "$INSTALL_DIR/config.toml" ] || [ -f "$INSTALL_DIR/sigint.db" ]; then
        echo ""
        read -p "  Keep config.toml and sigint.db? (Y/n) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Nn]$ ]]; then
            rm -rf "$INSTALL_DIR"
            echo -e "${GREEN}  Removed $INSTALL_DIR (including config and database)${NC}"
        else
            BACKUP_DIR="/tmp/sigint-deck-backup-$$"
            mkdir -p "$BACKUP_DIR"
            cp "$INSTALL_DIR/config.toml" "$BACKUP_DIR/" 2>/dev/null || true
            cp "$INSTALL_DIR/sigint.db" "$BACKUP_DIR/" 2>/dev/null || true
            rm -rf "$INSTALL_DIR"
            mkdir -p "$INSTALL_DIR"
            cp "$BACKUP_DIR/config.toml" "$INSTALL_DIR/" 2>/dev/null || true
            cp "$BACKUP_DIR/sigint.db" "$INSTALL_DIR/" 2>/dev/null || true
            rm -rf "$BACKUP_DIR"
            echo -e "${GREEN}  Removed application files (kept config.toml and sigint.db)${NC}"
        fi
    else
        rm -rf "$INSTALL_DIR"
        echo -e "${GREEN}  Removed $INSTALL_DIR${NC}"
    fi
else
    echo -e "${GREEN}  $INSTALL_DIR not found, skipping${NC}"
fi

# ============================================
# Step 3: Remove SDR tools from ~/bin
# ============================================
echo ""
read -p "Remove SDR tools from ~/bin? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}[3/6] Removing SDR tools...${NC}"
    rm -f "$BIN_DIR/rtl_sdr" "$BIN_DIR/rtl_fm" "$BIN_DIR/rtl_power" "$BIN_DIR/rtl_test" "$BIN_DIR/rtl_adsb" "$BIN_DIR/rtl_eeprom" "$BIN_DIR/rtl_tcp"
    rm -f "$BIN_DIR/rtl_433"
    rm -f "$BIN_DIR/hackrf_info" "$BIN_DIR/hackrf_sweep" "$BIN_DIR/hackrf_transfer" "$BIN_DIR/hackrf_spiflash"
    rm -f "$BIN_DIR/kal"
    rm -f "$BIN_DIR/LimeUtil" "$BIN_DIR/SoapySDRUtil"
    rm -rf "$BIN_DIR/lib" 2>/dev/null || true
    # Remove from ~/.local too
    rm -f "$HOME/.local/bin/rtl_"* "$HOME/.local/bin/hackrf_"* "$HOME/.local/bin/kal" "$HOME/.local/bin/rtl_433" 2>/dev/null || true
    rm -rf "$HOME/.local/lib/librtl"* "$HOME/.local/lib/libhackrf"* 2>/dev/null || true
    echo -e "${GREEN}  Removed SDR tools${NC}"
else
    echo -e "${YELLOW}[3/6] Keeping SDR tools${NC}"
fi

# ============================================
# Step 4: Remove udev rules
# ============================================
echo -e "${YELLOW}[4/6] Removing udev rules...${NC}"
sudo rm -f /etc/udev/rules.d/20-rtlsdr.rules 2>/dev/null || true
sudo udevadm control --reload-rules 2>/dev/null || true
echo -e "${GREEN}  Removed SDR udev rules${NC}"

# ============================================
# Step 5: Remove driver blacklist
# ============================================
echo -e "${YELLOW}[5/6] Removing driver blacklist...${NC}"
sudo rm -f /etc/modprobe.d/blacklist-rtlsdr.conf 2>/dev/null || true
echo -e "${GREEN}  Removed RTL-SDR driver blacklist${NC}"

# ============================================
# Step 6: Optionally remove Rust toolchain
# ============================================
echo ""
read -p "Remove Rust toolchain (~1GB)? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}[6/6] Removing Rust...${NC}"
    if [ -f "$HOME/.cargo/bin/rustup" ]; then
        "$HOME/.cargo/bin/rustup" self uninstall -y 2>/dev/null || true
        echo -e "${GREEN}  Removed Rust toolchain${NC}"
    else
        echo -e "${GREEN}  Rust not installed via rustup${NC}"
    fi
else
    echo -e "${YELLOW}[6/6] Keeping Rust toolchain${NC}"
fi

# ============================================
# Clean up temp files and Cargo config override
# ============================================
rm -f /tmp/sigint-deck*.log
rm -f /tmp/sigint_*.bin
rm -f /tmp/sigint_*.py
rm -f /tmp/cargo_*.log
rm -f /tmp/build_done*
rm -rf "$INSTALL_DIR/.cargo" 2>/dev/null || true

# ============================================
# Summary
# ============================================
echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Uninstall Complete${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "Removed:"
echo -e "  - sigint-deck binary and static files"
echo -e "  - systemd user service"
echo -e "  - udev rules and driver blacklist"
echo -e "  - Temporary and build files"
echo ""
if [ -f "$INSTALL_DIR/config.toml" ] || [ -f "$INSTALL_DIR/sigint.db" ]; then
    echo -e "${YELLOW}Preserved: $INSTALL_DIR/config.toml, $INSTALL_DIR/sigint.db${NC}"
fi
echo ""
echo -e "To reinstall: ${CYAN}curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-deck/main/scripts/install-deck.sh | bash${NC}"
echo ""
