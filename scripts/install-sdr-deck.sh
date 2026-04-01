#!/bin/bash
# SIGINT-Deck SDR Tools Installer for Steam Deck
# Uses pacman and AUR for SteamOS
#
# Usage: ./install-sdr-deck.sh [--minimal|--full|--interactive|--component]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Installation targets
INSTALL_RTLSDR=false
INSTALL_RTL433=false
INSTALL_HACKRF=false
INSTALL_LIMESDR=false
INSTALL_KALIBRATE=false
INSTALL_SOAPY=false

# Install to ~/bin for Steam Deck (no root needed for user binaries)
INSTALL_PREFIX="$HOME/bin"
mkdir -p "$INSTALL_PREFIX"
mkdir -p "$INSTALL_PREFIX/lib"

show_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║        SIGINT-Deck SDR Installer for Steam Deck           ║"
    echo "║   RTL-SDR | rtl_433 | HackRF | LimeSDR | kalibrate        ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Installation Profiles:"
    echo "  --minimal      Core SDR tools only (RTL-SDR, rtl_433)"
    echo "  --full         All SDR tools"
    echo "  --interactive  Interactive menu"
    echo ""
    echo "Individual Components:"
    echo "  --rtlsdr       RTL-SDR base tools"
    echo "  --rtl433       rtl_433 ISM band decoder"  
    echo "  --hackrf       HackRF One tools"
    echo "  --limesdr      LimeSDR tools"
    echo "  --kalibrate    kalibrate-rtl cell tower scanner"
    echo "  --soapy        SoapySDR abstraction layer"
    echo "  --all          Same as --full"
    echo ""
    echo "Tools install to ~/bin (add to PATH in ~/.bashrc)"
    exit 0
}

show_menu() {
    echo -e "${CYAN}Select components to install:${NC}"
    echo ""
    echo "  [1] RTL-SDR Base Tools"
    echo "  [2] rtl_433 (ISM decoder)"
    echo "  [3] HackRF One"
    echo "  [4] LimeSDR"
    echo "  [5] kalibrate-rtl"
    echo "  [6] SoapySDR"
    echo ""
    echo "  [M] Minimal (1,2)"
    echo "  [F] Full (All)"
    echo "  [Q] Quit"
    echo ""
    read -p "Enter choices: " choices
    
    for choice in $choices; do
        case $choice in
            1) INSTALL_RTLSDR=true ;;
            2) INSTALL_RTL433=true ;;
            3) INSTALL_HACKRF=true ;;
            4) INSTALL_LIMESDR=true ;;
            5) INSTALL_KALIBRATE=true ;;
            6) INSTALL_SOAPY=true ;;
            [Mm]) INSTALL_RTLSDR=true; INSTALL_RTL433=true ;;
            [Ff]|[Aa]) 
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                INSTALL_HACKRF=true
                INSTALL_LIMESDR=true
                INSTALL_KALIBRATE=true
                INSTALL_SOAPY=true
                ;;
            [Qq]) exit 0 ;;
        esac
    done
}

# Parse arguments
if [ $# -eq 0 ]; then
    show_banner
    show_menu
else
    for arg in "$@"; do
        case $arg in
            --help|-h) show_help ;;
            --minimal) INSTALL_RTLSDR=true; INSTALL_RTL433=true ;;
            --full|--all)
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                INSTALL_HACKRF=true
                INSTALL_LIMESDR=true
                INSTALL_KALIBRATE=true
                INSTALL_SOAPY=true
                ;;
            --interactive) show_banner; show_menu ;;
            --rtlsdr) INSTALL_RTLSDR=true ;;
            --rtl433) INSTALL_RTL433=true ;;
            --hackrf) INSTALL_HACKRF=true ;;
            --limesdr) INSTALL_LIMESDR=true ;;
            --kalibrate) INSTALL_KALIBRATE=true ;;
            --soapy) INSTALL_SOAPY=true ;;
            *) echo -e "${RED}Unknown: $arg${NC}"; exit 1 ;;
        esac
    done
fi

show_banner

# Check for Steam Deck
if [ -f /etc/os-release ] && grep -q "steamos" /etc/os-release; then
    echo -e "${GREEN}Detected Steam Deck / SteamOS${NC}"
    IS_STEAMDECK=true
else
    echo -e "${YELLOW}Warning: Not running on SteamOS${NC}"
    IS_STEAMDECK=false
fi

# Disable read-only filesystem if needed
if $IS_STEAMDECK; then
    echo -e "${BLUE}Temporarily disabling read-only filesystem...${NC}"
    sudo steamos-readonly disable 2>/dev/null || true
fi

# Install build dependencies via pacman
echo -e "\n${BLUE}Installing build dependencies...${NC}"
sudo pacman -S --needed --noconfirm base-devel cmake git libusb 2>/dev/null || {
    echo -e "${YELLOW}pacman failed, trying without sudo...${NC}"
}

# ============================================
# RTL-SDR
# ============================================
if $INSTALL_RTLSDR; then
    echo -e "\n${GREEN}[RTL-SDR] Installing...${NC}"
    
    # Check if already in ~/bin
    if [ -f "$INSTALL_PREFIX/rtl_sdr" ]; then
        echo -e "${GREEN}✓ RTL-SDR already installed in ~/bin${NC}"
    else
        # Try pacman first
        if sudo pacman -S --needed --noconfirm rtl-sdr 2>/dev/null; then
            echo -e "${GREEN}✓ RTL-SDR installed via pacman${NC}"
        else
            # Build from source to ~/bin
            echo "Building RTL-SDR from source..."
            cd /tmp
            rm -rf rtl-sdr
            git clone https://github.com/osmocom/rtl-sdr.git
            cd rtl-sdr
            mkdir -p build && cd build
            cmake ../ -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX" -DINSTALL_UDEV_RULES=OFF
            make -j$(nproc)
            make install
            echo -e "${GREEN}✓ RTL-SDR built to ~/bin${NC}"
        fi
    fi
fi

# ============================================
# rtl_433
# ============================================
if $INSTALL_RTL433; then
    echo -e "\n${GREEN}[rtl_433] Installing ISM decoder...${NC}"
    
    if [ -f "$INSTALL_PREFIX/rtl_433" ]; then
        echo -e "${GREEN}✓ rtl_433 already installed${NC}"
    else
        # Must build from source
        echo "Building rtl_433 from source..."
        cd /tmp
        rm -rf rtl_433
        git clone https://github.com/merbanan/rtl_433.git
        cd rtl_433
        mkdir -p build && cd build
        cmake ../ -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX"
        make -j$(nproc)
        make install
        echo -e "${GREEN}✓ rtl_433 built to ~/bin${NC}"
    fi
fi

# ============================================
# HackRF
# ============================================
if $INSTALL_HACKRF; then
    echo -e "\n${GREEN}[HackRF] Installing...${NC}"
    
    if [ -f "$INSTALL_PREFIX/hackrf_info" ]; then
        echo -e "${GREEN}✓ HackRF already installed${NC}"
    else
        if sudo pacman -S --needed --noconfirm hackrf 2>/dev/null; then
            echo -e "${GREEN}✓ HackRF installed via pacman${NC}"
        else
            echo "Building HackRF from source..."
            cd /tmp
            rm -rf hackrf
            git clone https://github.com/greatscottgadgets/hackrf.git
            cd hackrf/host
            mkdir -p build && cd build
            cmake ../ -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX"
            make -j$(nproc)
            make install
            echo -e "${GREEN}✓ HackRF built to ~/bin${NC}"
        fi
    fi
fi

# ============================================
# LimeSDR
# ============================================
if $INSTALL_LIMESDR; then
    echo -e "\n${GREEN}[LimeSDR] Installing...${NC}"
    
    if [ -f "$INSTALL_PREFIX/LimeUtil" ]; then
        echo -e "${GREEN}✓ LimeSDR already installed${NC}"
    else
        if sudo pacman -S --needed --noconfirm limesuite 2>/dev/null; then
            echo -e "${GREEN}✓ LimeSuite installed via pacman${NC}"
        else
            echo "Building LimeSuite from source..."
            sudo pacman -S --needed --noconfirm wxwidgets-gtk3 freeglut sqlite 2>/dev/null || true
            cd /tmp
            rm -rf LimeSuite
            git clone https://github.com/myriadrf/LimeSuite.git
            cd LimeSuite
            mkdir -p builddir && cd builddir
            cmake ../ -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX"
            make -j$(nproc)
            make install
            echo -e "${GREEN}✓ LimeSuite built to ~/bin${NC}"
        fi
    fi
fi

# ============================================
# SoapySDR
# ============================================
if $INSTALL_SOAPY; then
    echo -e "\n${GREEN}[SoapySDR] Installing...${NC}"
    
    if [ -f "$INSTALL_PREFIX/SoapySDRUtil" ] || command -v SoapySDRUtil &>/dev/null; then
        echo -e "${GREEN}✓ SoapySDR already installed${NC}"
    else
        if sudo pacman -S --needed --noconfirm soapysdr 2>/dev/null; then
            echo -e "${GREEN}✓ SoapySDR installed via pacman${NC}"
        else
            echo "Building SoapySDR from source..."
            cd /tmp
            rm -rf SoapySDR
            git clone https://github.com/pothosware/SoapySDR.git
            cd SoapySDR
            mkdir -p build && cd build
            cmake ../ -DCMAKE_INSTALL_PREFIX="$INSTALL_PREFIX"
            make -j$(nproc)
            make install
            echo -e "${GREEN}✓ SoapySDR built to ~/bin${NC}"
        fi
    fi
fi

# ============================================
# kalibrate-rtl
# ============================================
if $INSTALL_KALIBRATE; then
    echo -e "\n${GREEN}[kalibrate-rtl] Installing cell tower scanner...${NC}"
    
    if [ -f "$INSTALL_PREFIX/kal" ]; then
        echo -e "${GREEN}✓ kalibrate-rtl already installed${NC}"
    else
        sudo pacman -S --needed --noconfirm fftw autoconf automake libtool 2>/dev/null || true
        cd /tmp
        rm -rf kalibrate-rtl
        git clone https://github.com/steve-m/kalibrate-rtl.git
        cd kalibrate-rtl
        ./bootstrap
        ./configure --prefix="$INSTALL_PREFIX"
        make -j$(nproc)
        make install
        echo -e "${GREEN}✓ kalibrate-rtl built to ~/bin${NC}"
    fi
fi

# ============================================
# Setup PATH
# ============================================
echo -e "\n${BLUE}Checking PATH configuration...${NC}"

if ! grep -q 'export PATH="$HOME/bin:$PATH"' ~/.bashrc 2>/dev/null; then
    echo 'export PATH="$HOME/bin:$PATH"' >> ~/.bashrc
    echo 'export LD_LIBRARY_PATH="$HOME/bin/lib:$LD_LIBRARY_PATH"' >> ~/.bashrc
    echo -e "${GREEN}✓ Added ~/bin to PATH in ~/.bashrc${NC}"
    echo -e "${YELLOW}Run: source ~/.bashrc (or restart terminal)${NC}"
else
    echo -e "${GREEN}✓ ~/bin already in PATH${NC}"
fi

# Re-enable read-only filesystem
if $IS_STEAMDECK; then
    echo -e "\n${BLUE}Re-enabling read-only filesystem...${NC}"
    sudo steamos-readonly enable 2>/dev/null || true
fi

# ============================================
# Summary
# ============================================
echo -e "\n${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║           Installation Complete!                           ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}Tools in ~/bin:${NC}"
ls -1 "$INSTALL_PREFIX" 2>/dev/null | grep -E "rtl_|hackrf|Lime|Soapy|kal" | while read tool; do
    echo -e "  ${GREEN}✓${NC} $tool"
done

echo ""
echo -e "${CYAN}Quick test (run after: source ~/.bashrc):${NC}"
echo "  rtl_test -t        # Test RTL-SDR"
echo "  rtl_433 -G         # List rtl_433 devices"
echo "  hackrf_info        # HackRF info"
echo "  LimeUtil --find    # Find LimeSDR"
echo "  SoapySDRUtil --find  # Find all SDRs"
echo ""
