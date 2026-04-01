#!/bin/bash
# SIGINT-Deck SDR Tools Installer
# Installs RTL-SDR, HackRF, LimeSDR, and SoapySDR to ~/bin
# Survives SteamOS updates (read-only root filesystem workaround)
#
# Usage: ./install-sdr.sh [--all|--rtlsdr|--hackrf|--limesdr|--soapysdr]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="$HOME/bin"
LIB_DIR="$HOME/bin/lib"
TEMP_DIR="$HOME/.sdr-install-tmp"
ARCH_MIRROR="https://archive.archlinux.org/packages"

# Package URLs
RTLSDR_PKG="r/rtl-sdr/rtl-sdr-1%3A2.0.2-1-x86_64.pkg.tar.zst"
HACKRF_PKG="h/hackrf/hackrf-2024.02.1-3-x86_64.pkg.tar.zst"
LIMESUITE_PKG="l/limesuite/limesuite-23.11.0-4-x86_64.pkg.tar.zst"
SOAPYSDR_PKG="s/soapysdr/soapysdr-0.8.1-3-x86_64.pkg.tar.zst"

echo -e "${BLUE}"
echo "╔═══════════════════════════════════════════════╗"
echo "║       SIGINT-Deck SDR Tools Installer         ║"
echo "║   RTL-SDR | HackRF | LimeSDR | SoapySDR       ║"
echo "╚═══════════════════════════════════════════════╝"
echo -e "${NC}"

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
            echo ""
            echo "Options:"
            echo "  --all       Install all SDR tools (default)"
            echo "  --rtlsdr    Install RTL-SDR tools only"
            echo "  --hackrf    Install HackRF tools only"
            echo "  --limesdr   Install LimeSDR tools only"
            echo "  --soapysdr  Install SoapySDR (universal API)"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $arg${NC}"
            exit 1
            ;;
    esac
done

if $INSTALL_ALL; then
    INSTALL_RTLSDR=true
    INSTALL_HACKRF=true
    INSTALL_LIMESDR=true
    INSTALL_SOAPYSDR=true
fi

# Create directories
mkdir -p "$INSTALL_DIR" "$LIB_DIR" "$TEMP_DIR"
cd "$TEMP_DIR"

# Helper function to download and extract
extract_pkg() {
    local url="$1"
    local name="$2"
    
    echo -e "${BLUE}Downloading $name...${NC}"
    wget -q "$ARCH_MIRROR/$url" -O "$name.pkg.tar.zst" || {
        echo -e "${RED}Failed to download $name${NC}"
        return 1
    }
    
    echo "Extracting..."
    mkdir -p "$name"
    cd "$name"
    zstd -d "../$name.pkg.tar.zst" -o "$name.tar" 2>/dev/null
    tar xf "$name.tar"
    cd ..
}

# Install RTL-SDR
if $INSTALL_RTLSDR; then
    echo -e "\n${GREEN}[1/4] Installing RTL-SDR...${NC}"
    if extract_pkg "$RTLSDR_PKG" "rtlsdr"; then
        cp rtlsdr/usr/bin/rtl_* "$INSTALL_DIR/" 2>/dev/null || true
        cp rtlsdr/usr/lib/*.so* "$LIB_DIR/" 2>/dev/null || true
        echo -e "${GREEN}✓ RTL-SDR installed${NC}"
        echo "  Tools: rtl_sdr, rtl_fm, rtl_power, rtl_adsb, rtl_tcp, rtl_test"
    fi
fi

# Install HackRF
if $INSTALL_HACKRF; then
    echo -e "\n${GREEN}[2/4] Installing HackRF...${NC}"
    if extract_pkg "$HACKRF_PKG" "hackrf"; then
        cp hackrf/usr/bin/hackrf_* "$INSTALL_DIR/" 2>/dev/null || true
        cp hackrf/usr/lib/*.so* "$LIB_DIR/" 2>/dev/null || true
        echo -e "${GREEN}✓ HackRF installed${NC}"
        echo "  Tools: hackrf_info, hackrf_transfer, hackrf_sweep"
    fi
fi

# Install LimeSDR
if $INSTALL_LIMESDR; then
    echo -e "\n${GREEN}[3/4] Installing LimeSDR...${NC}"
    if extract_pkg "$LIMESUITE_PKG" "limesuite"; then
        cp limesuite/usr/bin/Lime* "$INSTALL_DIR/" 2>/dev/null || true
        cp limesuite/usr/lib/*.so* "$LIB_DIR/" 2>/dev/null || true
        echo -e "${GREEN}✓ LimeSDR installed${NC}"
        echo "  Tools: LimeUtil, LimeQuickTest, LimeSuiteGUI"
    fi
fi

# Install SoapySDR
if $INSTALL_SOAPYSDR; then
    echo -e "\n${GREEN}[4/4] Installing SoapySDR...${NC}"
    if extract_pkg "$SOAPYSDR_PKG" "soapysdr"; then
        cp soapysdr/usr/bin/Soapy* "$INSTALL_DIR/" 2>/dev/null || true
        cp soapysdr/usr/lib/*.so* "$LIB_DIR/" 2>/dev/null || true
        echo -e "${GREEN}✓ SoapySDR installed${NC}"
        echo "  Tools: SoapySDRUtil"
    fi
fi

# Cleanup
echo -e "\n${BLUE}Cleaning up...${NC}"
rm -rf "$TEMP_DIR"

# Update .bashrc
if ! grep -q 'LD_LIBRARY_PATH.*bin/lib' ~/.bashrc 2>/dev/null; then
    echo -e "\n${BLUE}Adding library path to .bashrc...${NC}"
    echo '' >> ~/.bashrc
    echo '# SDR tools library path' >> ~/.bashrc
    echo 'export LD_LIBRARY_PATH="$HOME/bin/lib:$LD_LIBRARY_PATH"' >> ~/.bashrc
fi

# Summary
echo -e "\n${GREEN}╔═══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║          SDR Installation Complete!            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════╝${NC}"
echo ""
echo "Installed tools:"
ls -1 "$INSTALL_DIR" | grep -E "^(rtl_|hackrf_|Lime|Soapy)" | sed 's/^/  /'
echo ""
echo -e "${YELLOW}To use immediately, run:${NC}"
echo "  export LD_LIBRARY_PATH=\"\$HOME/bin/lib:\$LD_LIBRARY_PATH\""
echo ""
echo -e "${YELLOW}Or restart your shell:${NC}"
echo "  source ~/.bashrc"
echo ""
echo -e "${BLUE}Test your SDR hardware:${NC}"
echo "  rtl_test -t          # RTL-SDR"
echo "  hackrf_info          # HackRF"
echo "  LimeUtil --find      # LimeSDR"
echo "  SoapySDRUtil --find  # All SDRs"
echo ""
