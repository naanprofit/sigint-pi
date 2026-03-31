#!/bin/bash
# SIGINT-Pi SDR Tools Installer
# Installs RTL-SDR, HackRF, and LimeSDR support
#
# Usage: ./install-sdr.sh [--all|--rtlsdr|--hackrf|--limesdr]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════╗"
echo "║        SIGINT-Pi SDR Tools Installer          ║"
echo "║   RTL-SDR | HackRF | LimeSDR | SoapySDR       ║"
echo "╚═══════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${YELLOW}Note: Some operations may require sudo${NC}"
fi

# Detect platform
if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO="$ID"
else
    DISTRO="unknown"
fi

echo "Detected: $DISTRO"

# Parse arguments
INSTALL_RTLSDR=false
INSTALL_HACKRF=false
INSTALL_LIMESDR=false
INSTALL_SOAPYSDR=false
INSTALL_ALL=false

if [ $# -eq 0 ]; then
    INSTALL_ALL=true
fi

for arg in "$@"; do
    case $arg in
        --all) INSTALL_ALL=true ;;
        --rtlsdr) INSTALL_RTLSDR=true ;;
        --hackrf) INSTALL_HACKRF=true ;;
        --limesdr) INSTALL_LIMESDR=true ;;
        --soapysdr) INSTALL_SOAPYSDR=true ;;
        --help|-h)
            echo "Usage: $0 [--all|--rtlsdr|--hackrf|--limesdr|--soapysdr]"
            exit 0
            ;;
    esac
done

if $INSTALL_ALL; then
    INSTALL_RTLSDR=true
    INSTALL_HACKRF=true
    INSTALL_LIMESDR=true
    INSTALL_SOAPYSDR=true
fi

# Install based on distro
case "$DISTRO" in
    raspbian|debian|ubuntu)
        echo -e "\n${BLUE}Installing SDR packages via apt...${NC}"
        
        # Update package list
        sudo apt-get update
        
        if $INSTALL_RTLSDR; then
            echo -e "\n${GREEN}Installing RTL-SDR...${NC}"
            sudo apt-get install -y rtl-sdr librtlsdr-dev
        fi
        
        if $INSTALL_HACKRF; then
            echo -e "\n${GREEN}Installing HackRF...${NC}"
            sudo apt-get install -y hackrf libhackrf-dev
        fi
        
        if $INSTALL_LIMESDR; then
            echo -e "\n${GREEN}Installing LimeSDR...${NC}"
            sudo apt-get install -y limesuite liblimesuite-dev
        fi
        
        if $INSTALL_SOAPYSDR; then
            echo -e "\n${GREEN}Installing SoapySDR...${NC}"
            sudo apt-get install -y soapysdr-tools libsoapysdr-dev
            # Install SoapySDR modules for each SDR
            sudo apt-get install -y soapysdr-module-rtlsdr 2>/dev/null || true
            sudo apt-get install -y soapysdr-module-hackrf 2>/dev/null || true
            sudo apt-get install -y soapysdr-module-lms7 2>/dev/null || true
        fi
        ;;
        
    arch|steamos)
        echo -e "\n${BLUE}Installing SDR packages via pacman...${NC}"
        
        # Check if read-only (Steam Deck)
        if ! touch /tmp/.write-test 2>/dev/null; then
            echo -e "${RED}Read-only filesystem detected (Steam Deck)${NC}"
            echo "Use the Steam Deck install script instead:"
            echo "  ~/sigint-deck/scripts/install-sdr.sh"
            exit 1
        fi
        
        if $INSTALL_RTLSDR; then
            sudo pacman -S --noconfirm rtl-sdr
        fi
        
        if $INSTALL_HACKRF; then
            sudo pacman -S --noconfirm hackrf
        fi
        
        if $INSTALL_LIMESDR; then
            sudo pacman -S --noconfirm limesuite
        fi
        
        if $INSTALL_SOAPYSDR; then
            sudo pacman -S --noconfirm soapysdr
        fi
        ;;
        
    *)
        echo -e "${RED}Unsupported distribution: $DISTRO${NC}"
        echo "Please install SDR packages manually:"
        echo "  - rtl-sdr"
        echo "  - hackrf"
        echo "  - limesuite"
        echo "  - soapysdr"
        exit 1
        ;;
esac

# Setup udev rules for non-root access
echo -e "\n${BLUE}Setting up udev rules...${NC}"

cat | sudo tee /etc/udev/rules.d/99-sdr.rules > /dev/null << 'EOF'
# RTL-SDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2832", MODE="0666"

# HackRF
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="604b", MODE="0666"

# LimeSDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="601f", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6108", MODE="0666"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger

# Blacklist kernel DVB drivers for RTL-SDR
echo -e "\n${BLUE}Blacklisting DVB kernel modules...${NC}"
cat | sudo tee /etc/modprobe.d/blacklist-rtlsdr.conf > /dev/null << 'EOF'
# Blacklist DVB drivers to allow RTL-SDR access
blacklist dvb_usb_rtl28xxu
blacklist rtl2832
blacklist rtl2830
EOF

# Summary
echo -e "\n${GREEN}╔═══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║          SDR Installation Complete!            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Installed tools:${NC}"
which rtl_sdr 2>/dev/null && echo "  ✓ RTL-SDR: $(which rtl_sdr)"
which hackrf_info 2>/dev/null && echo "  ✓ HackRF: $(which hackrf_info)"
which LimeUtil 2>/dev/null && echo "  ✓ LimeSDR: $(which LimeUtil)"
which SoapySDRUtil 2>/dev/null && echo "  ✓ SoapySDR: $(which SoapySDRUtil)"

echo ""
echo -e "${YELLOW}Note: Unplug and replug your SDR device for udev rules to take effect${NC}"
echo ""
echo -e "${BLUE}Test your SDR:${NC}"
echo "  rtl_test -t          # RTL-SDR"
echo "  hackrf_info          # HackRF"
echo "  LimeUtil --find      # LimeSDR"
echo "  SoapySDRUtil --find  # All SDRs"
echo ""
