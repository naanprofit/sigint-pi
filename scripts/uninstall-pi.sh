#!/bin/bash
set -e

# SIGINT-Pi Raspberry Pi Uninstall Script
# Removes the SIGINT-Pi installation, services, and optionally SDR tools

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

INSTALL_DIR="$HOME/sigint-pi"

echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} SIGINT-Pi Pi Uninstaller${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""

# ============================================
# Confirm
# ============================================
echo -e "${YELLOW}This will remove SIGINT-Pi from this system.${NC}"
echo -e "Install directory: ${CYAN}$INSTALL_DIR${NC}"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 0
fi

# ============================================
# Step 1: Stop and disable services
# ============================================
echo -e "${YELLOW}[1/6] Stopping services...${NC}"
if systemctl is-active --quiet sigint-pi.service 2>/dev/null; then
    sudo systemctl stop sigint-pi.service
    echo -e "${GREEN}  Stopped sigint-pi.service${NC}"
fi
if systemctl is-enabled --quiet sigint-pi.service 2>/dev/null; then
    sudo systemctl disable sigint-pi.service
    echo -e "${GREEN}  Disabled sigint-pi.service${NC}"
fi
sudo rm -f /etc/systemd/system/sigint-pi.service
sudo systemctl daemon-reload 2>/dev/null || true

# Stop user services
if systemctl --user is-active --quiet sigint-pi.service 2>/dev/null; then
    systemctl --user stop sigint-pi.service
fi
if systemctl --user is-enabled --quiet sigint-pi.service 2>/dev/null; then
    systemctl --user disable sigint-pi.service
fi
rm -f ~/.config/systemd/user/sigint-pi.service

# Stop ADB forward service
if systemctl --user is-active --quiet adb-forward.service 2>/dev/null; then
    systemctl --user stop adb-forward.service
fi
if systemctl --user is-enabled --quiet adb-forward.service 2>/dev/null; then
    systemctl --user disable adb-forward.service
fi
rm -f ~/.config/systemd/user/adb-forward.service
systemctl --user daemon-reload 2>/dev/null || true

# Kill any running sigint-pi process
pkill -f sigint-pi 2>/dev/null || true
echo -e "${GREEN}  All services stopped${NC}"

# ============================================
# Step 2: Remove binary and application files
# ============================================
echo -e "${YELLOW}[2/6] Removing application files...${NC}"
if [ -d "$INSTALL_DIR" ]; then
    # Preserve config and database if user wants
    if [ -f "$INSTALL_DIR/config.toml" ] || [ -f "$INSTALL_DIR/sigint.db" ]; then
        echo ""
        read -p "  Keep config.toml and sigint.db? (Y/n) " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Nn]$ ]]; then
            rm -rf "$INSTALL_DIR"
            echo -e "${GREEN}  Removed $INSTALL_DIR (including config and database)${NC}"
        else
            # Remove everything except config and db
            BACKUP_DIR="/tmp/sigint-pi-backup-$$"
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
# Step 3: Remove udev rules
# ============================================
echo -e "${YELLOW}[3/6] Removing udev rules...${NC}"
sudo rm -f /etc/udev/rules.d/20-rtlsdr.rules
sudo udevadm control --reload-rules 2>/dev/null || true
echo -e "${GREEN}  Removed SDR udev rules${NC}"

# ============================================
# Step 4: Remove DVB-T blacklist
# ============================================
echo -e "${YELLOW}[4/6] Removing driver blacklist...${NC}"
sudo rm -f /etc/modprobe.d/blacklist-rtlsdr.conf
echo -e "${GREEN}  Removed RTL-SDR driver blacklist${NC}"

# ============================================
# Step 5: Optionally remove SDR packages
# ============================================
echo ""
read -p "Remove SDR packages (rtl-sdr, hackrf, kalibrate-rtl)? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}[5/6] Removing SDR packages...${NC}"
    sudo apt-get remove -y rtl-sdr librtlsdr-dev rtl-433 hackrf libhackrf-dev 2>/dev/null || true
    # Remove kalibrate if installed from source
    sudo rm -f /usr/local/bin/kal
    echo -e "${GREEN}  Removed SDR packages${NC}"
else
    echo -e "${YELLOW}[5/6] Keeping SDR packages${NC}"
fi

# ============================================
# Step 6: Clean up temp files
# ============================================
echo -e "${YELLOW}[6/6] Cleaning up...${NC}"
rm -f /tmp/sigint-pi.log
rm -f /tmp/sigint_*.bin
rm -f /tmp/sigint_*.py
echo -e "${GREEN}  Cleaned up temp files${NC}"

# ============================================
# Summary
# ============================================
echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Uninstall Complete${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "Removed:"
echo -e "  - sigint-pi binary and static files"
echo -e "  - systemd services (system and user)"
echo -e "  - ADB forward service"
echo -e "  - udev rules and driver blacklist"
echo -e "  - Temporary files"
echo ""
if [ -f "$INSTALL_DIR/config.toml" ] || [ -f "$INSTALL_DIR/sigint.db" ]; then
    echo -e "${YELLOW}Preserved: $INSTALL_DIR/config.toml, $INSTALL_DIR/sigint.db${NC}"
fi
echo ""
echo -e "To reinstall: ${CYAN}curl -sSL https://raw.githubusercontent.com/naanprofit/sigint-pi/main/scripts/install-pi.sh | bash${NC}"
echo ""
