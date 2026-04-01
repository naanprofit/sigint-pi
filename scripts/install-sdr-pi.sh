#!/bin/bash
# SIGINT-Pi SDR Tools Installer
# Installs SDR tools for Raspberry Pi OS (also works on Debian/Ubuntu)
#
# Usage: ./install-sdr-pi.sh [--minimal|--full|--interactive|--component]

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Installation state
INSTALL_RTLSDR=false
INSTALL_RTL433=false
INSTALL_HACKRF=false
INSTALL_KALIBRATE=false
INSTALL_LIMESDR=false
INSTALL_GPS=false
INSTALL_SOAPY=false

show_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════╗"
    echo "║           SIGINT-Pi SDR Tools Installer v2.0              ║"
    echo "║  RTL-SDR | rtl_433 | HackRF | LimeSDR | GPS | kalibrate   ║"
    echo "╚═══════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Installation Profiles:"
    echo "  --minimal      Core SDR tools only (RTL-SDR, rtl_433)"
    echo "  --full         All SDR tools and peripherals"
    echo "  --interactive  Interactive menu to choose components"
    echo ""
    echo "Individual Components:"
    echo "  --rtlsdr       RTL-SDR base tools (rtl_sdr, rtl_fm, rtl_power)"
    echo "  --rtl433       rtl_433 ISM band decoder"
    echo "  --hackrf       HackRF One tools (1 MHz - 6 GHz)"
    echo "  --limesdr      LimeSDR tools (100 kHz - 3.8 GHz)"
    echo "  --kalibrate    kalibrate-rtl cell tower scanner"
    echo "  --gps          GPS daemon (gpsd) for location"
    echo "  --soapy        SoapySDR abstraction layer"
    echo "  --all          Same as --full"
    echo ""
    echo "Examples:"
    echo "  $0 --minimal           # Quick install for basic monitoring"
    echo "  $0 --full              # Everything for advanced SIGINT"
    echo "  $0 --interactive       # Choose what to install"
    echo "  $0 --rtlsdr --rtl433   # Just RTL-SDR and decoder"
    echo "  $0 --limesdr --soapy   # LimeSDR with SoapySDR"
    exit 0
}

show_menu() {
    echo -e "${CYAN}Select components to install:${NC}"
    echo ""
    echo "  [1] RTL-SDR Base Tools     (rtl_sdr, rtl_fm, rtl_power, rtl_tcp)"
    echo "  [2] rtl_433                (ISM band decoder: 315/433/868/915 MHz)"
    echo "  [3] HackRF One             (TX/RX 1 MHz - 6 GHz)"
    echo "  [4] LimeSDR                (TX/RX 100 kHz - 3.8 GHz, full duplex)"
    echo "  [5] kalibrate-rtl          (Cell tower scanner/GSM frequency calibration)"
    echo "  [6] GPS Daemon             (gpsd for location tracking)"
    echo "  [7] SoapySDR               (Universal SDR abstraction layer)"
    echo ""
    echo "  [M] Minimal (1,2)"
    echo "  [F] Full (All)"
    echo "  [Q] Quit"
    echo ""
    echo -e "${YELLOW}Enter choices (e.g., 1 2 3 or M for minimal):${NC}"
    read -r choices
    
    for choice in $choices; do
        case $choice in
            1) INSTALL_RTLSDR=true ;;
            2) INSTALL_RTL433=true ;;
            3) INSTALL_HACKRF=true ;;
            4) INSTALL_LIMESDR=true ;;
            5) INSTALL_KALIBRATE=true ;;
            6) INSTALL_GPS=true ;;
            7) INSTALL_SOAPY=true ;;
            [Mm]) 
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                ;;
            [Ff]|[Aa]) 
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                INSTALL_HACKRF=true
                INSTALL_LIMESDR=true
                INSTALL_KALIBRATE=true
                INSTALL_GPS=true
                INSTALL_SOAPY=true
                ;;
            [Qq]) exit 0 ;;
            *) echo -e "${RED}Unknown option: $choice${NC}" ;;
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
            --minimal)
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                ;;
            --full|--all)
                INSTALL_RTLSDR=true
                INSTALL_RTL433=true
                INSTALL_HACKRF=true
                INSTALL_LIMESDR=true
                INSTALL_KALIBRATE=true
                INSTALL_GPS=true
                INSTALL_SOAPY=true
                ;;
            --interactive)
                show_banner
                show_menu
                ;;
            --rtlsdr) INSTALL_RTLSDR=true ;;
            --rtl433) INSTALL_RTL433=true ;;
            --hackrf) INSTALL_HACKRF=true ;;
            --limesdr) INSTALL_LIMESDR=true ;;
            --kalibrate) INSTALL_KALIBRATE=true ;;
            --gps) INSTALL_GPS=true ;;
            --soapy) INSTALL_SOAPY=true ;;
            *)
                echo -e "${RED}Unknown option: $arg${NC}"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
fi

show_banner

# Check what will be installed
echo -e "${CYAN}Installation plan:${NC}"
$INSTALL_RTLSDR && echo "  [x] RTL-SDR base tools"
$INSTALL_RTL433 && echo "  [x] rtl_433"
$INSTALL_HACKRF && echo "  [x] HackRF"
$INSTALL_LIMESDR && echo "  [x] LimeSDR"
$INSTALL_KALIBRATE && echo "  [x] kalibrate-rtl"
$INSTALL_GPS && echo "  [x] GPS (gpsd)"
$INSTALL_SOAPY && echo "  [x] SoapySDR"
echo ""

# Confirm
if [ -t 0 ]; then
    echo -e "${YELLOW}Proceed with installation? [Y/n]${NC}"
    read -r confirm
    if [[ "$confirm" =~ ^[Nn] ]]; then
        echo "Aborted."
        exit 0
    fi
fi

# Update package lists
echo -e "\n${BLUE}Updating package lists...${NC}"
sudo apt-get update

# Install common build dependencies
echo -e "${BLUE}Installing build dependencies...${NC}"
sudo apt-get install -y build-essential cmake git pkg-config libusb-1.0-0-dev \
    autoconf automake libtool

# ============================================
# 1. RTL-SDR Base Tools
# ============================================
if $INSTALL_RTLSDR; then
    echo -e "\n${GREEN}[RTL-SDR] Installing base tools...${NC}"
    
    if sudo apt-get install -y rtl-sdr librtlsdr-dev 2>/dev/null; then
        echo -e "${GREEN}✓ RTL-SDR installed via apt${NC}"
    else
        echo "Building RTL-SDR from source..."
        cd /tmp
        rm -rf rtl-sdr
        git clone https://github.com/osmocom/rtl-sdr.git
        cd rtl-sdr
        mkdir -p build && cd build
        cmake ../ -DINSTALL_UDEV_RULES=ON
        make -j$(nproc)
        sudo make install
        sudo ldconfig
        echo -e "${GREEN}✓ RTL-SDR built from source${NC}"
    fi
    
    # Blacklist DVB-T drivers
    if ! grep -q "blacklist dvb_usb_rtl28xxu" /etc/modprobe.d/blacklist-rtlsdr.conf 2>/dev/null; then
        echo -e "${BLUE}Blacklisting DVB-T kernel drivers...${NC}"
        echo "blacklist dvb_usb_rtl28xxu" | sudo tee /etc/modprobe.d/blacklist-rtlsdr.conf
        echo "blacklist rtl2832" | sudo tee -a /etc/modprobe.d/blacklist-rtlsdr.conf
        echo "blacklist rtl2830" | sudo tee -a /etc/modprobe.d/blacklist-rtlsdr.conf
    fi
    
    echo "  Tools: rtl_sdr, rtl_fm, rtl_power, rtl_tcp, rtl_test"
fi

# ============================================
# 2. rtl_433 - ISM Band Decoder
# ============================================
if $INSTALL_RTL433; then
    echo -e "\n${GREEN}[rtl_433] Installing ISM band decoder...${NC}"
    
    if sudo apt-get install -y rtl-433 2>/dev/null; then
        echo -e "${GREEN}✓ rtl_433 installed via apt${NC}"
    else
        echo "Building rtl_433 from source..."
        cd /tmp
        rm -rf rtl_433
        git clone https://github.com/merbanan/rtl_433.git
        cd rtl_433
        mkdir -p build && cd build
        cmake ..
        make -j$(nproc)
        sudo make install
        echo -e "${GREEN}✓ rtl_433 built from source${NC}"
    fi
    
    echo "  Decodes: weather stations, TPMS, door sensors, remotes"
    echo "  Frequencies: 315, 433.92, 868, 915 MHz"
fi

# ============================================
# 3. HackRF Tools
# ============================================
if $INSTALL_HACKRF; then
    echo -e "\n${GREEN}[HackRF] Installing HackRF One tools...${NC}"
    
    if sudo apt-get install -y hackrf libhackrf-dev 2>/dev/null; then
        echo -e "${GREEN}✓ HackRF installed via apt${NC}"
    else
        echo "Building HackRF from source..."
        cd /tmp
        rm -rf hackrf
        git clone https://github.com/greatscottgadgets/hackrf.git
        cd hackrf/host
        mkdir -p build && cd build
        cmake ..
        make -j$(nproc)
        sudo make install
        sudo ldconfig
        echo -e "${GREEN}✓ HackRF built from source${NC}"
    fi
    
    echo "  Tools: hackrf_info, hackrf_transfer, hackrf_sweep"
    echo "  Range: 1 MHz - 6 GHz, 20 MHz bandwidth"
fi

# ============================================
# 4. LimeSDR Tools
# ============================================
if $INSTALL_LIMESDR; then
    echo -e "\n${GREEN}[LimeSDR] Installing LimeSDR tools...${NC}"
    
    # LimeSuite includes drivers and GUI
    if sudo apt-get install -y limesuite limesuite-udev 2>/dev/null; then
        echo -e "${GREEN}✓ LimeSuite installed via apt${NC}"
    else
        echo "Building LimeSuite from source..."
        sudo apt-get install -y libsqlite3-dev libwxgtk3.0-gtk3-dev freeglut3-dev
        cd /tmp
        rm -rf LimeSuite
        git clone https://github.com/myriadrf/LimeSuite.git
        cd LimeSuite
        mkdir -p builddir && cd builddir
        cmake ../
        make -j$(nproc)
        sudo make install
        sudo ldconfig
        
        # Install udev rules
        cd ../udev-rules
        sudo ./install.sh
        echo -e "${GREEN}✓ LimeSuite built from source${NC}"
    fi
    
    echo "  Tools: LimeUtil, LimeSuiteGUI, LimeQuickTest"
    echo "  Range: 100 kHz - 3.8 GHz, full duplex TX/RX"
fi

# ============================================
# 5. SoapySDR - Universal SDR Layer
# ============================================
if $INSTALL_SOAPY; then
    echo -e "\n${GREEN}[SoapySDR] Installing SDR abstraction layer...${NC}"
    
    # SoapySDR core and common modules
    sudo apt-get install -y soapysdr-tools libsoapysdr-dev \
        soapysdr0.8-module-rtlsdr \
        soapysdr0.8-module-hackrf \
        soapysdr0.8-module-lms7 \
        soapysdr0.8-module-all 2>/dev/null || {
        
        echo "Building SoapySDR from source..."
        cd /tmp
        rm -rf SoapySDR
        git clone https://github.com/pothosware/SoapySDR.git
        cd SoapySDR
        mkdir -p build && cd build
        cmake ..
        make -j$(nproc)
        sudo make install
        sudo ldconfig
        echo -e "${GREEN}✓ SoapySDR built from source${NC}"
    }
    
    echo "  Tool: SoapySDRUtil --find (detect all SDRs)"
    echo "  Supports: RTL-SDR, HackRF, LimeSDR, BladeRF, USRP, etc."
fi

# ============================================
# 6. kalibrate-rtl - Cell Tower Scanner
# ============================================
if $INSTALL_KALIBRATE; then
    echo -e "\n${GREEN}[kalibrate-rtl] Installing cell tower scanner...${NC}"
    
    # Must build from source
    cd /tmp
    rm -rf kalibrate-rtl
    git clone https://github.com/steve-m/kalibrate-rtl.git
    cd kalibrate-rtl
    
    # Run bootstrap with error handling
    if ./bootstrap && ./configure; then
        make -j$(nproc)
        sudo make install
        echo -e "${GREEN}✓ kalibrate-rtl installed${NC}"
    else
        echo -e "${YELLOW}Warning: kalibrate-rtl build failed${NC}"
        echo "You may need: sudo apt-get install libfftw3-dev"
    fi
    
    echo "  Tool: kal -s GSM850/GSM900/EGSM/DCS/PCS"
    echo "  Usage: Scan for cell towers, calibrate SDR frequency offset"
fi

# ============================================
# 7. GPS Daemon
# ============================================
if $INSTALL_GPS; then
    echo -e "\n${GREEN}[GPS] Installing GPS daemon...${NC}"
    
    sudo apt-get install -y gpsd gpsd-clients
    
    # Configure gpsd for common USB GPS receivers
    if [ ! -f /etc/default/gpsd.bak ]; then
        sudo cp /etc/default/gpsd /etc/default/gpsd.bak 2>/dev/null || true
    fi
    
    cat << 'EOF' | sudo tee /etc/default/gpsd
# GPS configuration for SIGINT-Pi
START_DAEMON="true"
USBAUTO="true"
DEVICES="/dev/ttyUSB0 /dev/ttyACM0"
GPSD_OPTIONS="-n"
EOF
    
    sudo systemctl enable gpsd
    sudo systemctl restart gpsd 2>/dev/null || true
    
    echo -e "${GREEN}✓ gpsd installed and configured${NC}"
    echo "  Tools: gpsd, gpsmon, cgps"
    echo "  Connect USB GPS and run: gpsmon"
fi

# ============================================
# udev rules for non-root access
# ============================================
echo -e "\n${BLUE}Setting up udev rules...${NC}"

# RTL-SDR
cat << 'EOF' | sudo tee /etc/udev/rules.d/rtl-sdr.rules
# RTL-SDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2832", MODE:="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0bda", ATTRS{idProduct}=="2838", MODE:="0666"
EOF

# HackRF
cat << 'EOF' | sudo tee /etc/udev/rules.d/53-hackrf.rules
# HackRF One
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6089", MODE:="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="604b", MODE:="0666"
EOF

# LimeSDR
cat << 'EOF' | sudo tee /etc/udev/rules.d/64-limesuite.rules
# LimeSDR
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", MODE:="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="1d50", ATTRS{idProduct}=="6108", MODE:="0666"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger

# ============================================
# Summary
# ============================================
echo -e "\n${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║           SDR Installation Complete!                       ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}Installed tools:${NC}"

command -v rtl_test &>/dev/null && echo -e "  ${GREEN}✓${NC} RTL-SDR: rtl_sdr, rtl_fm, rtl_power, rtl_tcp"
command -v rtl_433 &>/dev/null && echo -e "  ${GREEN}✓${NC} rtl_433: ISM band decoder"
command -v hackrf_info &>/dev/null && echo -e "  ${GREEN}✓${NC} HackRF: hackrf_info, hackrf_sweep"
command -v LimeUtil &>/dev/null && echo -e "  ${GREEN}✓${NC} LimeSDR: LimeUtil, LimeSuiteGUI"
command -v SoapySDRUtil &>/dev/null && echo -e "  ${GREEN}✓${NC} SoapySDR: SoapySDRUtil"
command -v kal &>/dev/null && echo -e "  ${GREEN}✓${NC} kalibrate-rtl: kal"
command -v gpsd &>/dev/null && echo -e "  ${GREEN}✓${NC} GPS: gpsd, gpsmon, cgps"

echo ""
echo -e "${CYAN}Missing tools:${NC}"
command -v rtl_test &>/dev/null || echo -e "  ${RED}✗${NC} RTL-SDR (run with --rtlsdr)"
command -v rtl_433 &>/dev/null || echo -e "  ${RED}✗${NC} rtl_433 (run with --rtl433)"
command -v hackrf_info &>/dev/null || echo -e "  ${RED}✗${NC} HackRF (run with --hackrf)"
command -v LimeUtil &>/dev/null || echo -e "  ${RED}✗${NC} LimeSDR (run with --limesdr)"
command -v SoapySDRUtil &>/dev/null || echo -e "  ${RED}✗${NC} SoapySDR (run with --soapy)"
command -v kal &>/dev/null || echo -e "  ${RED}✗${NC} kalibrate-rtl (run with --kalibrate)"
command -v gpsd &>/dev/null || echo -e "  ${RED}✗${NC} GPS (run with --gps)"

echo ""
echo -e "${YELLOW}IMPORTANT: Unplug and replug SDR devices for udev rules to take effect${NC}"
echo ""
echo -e "${CYAN}Quick test commands:${NC}"
echo "  rtl_test -t              # Test RTL-SDR connection"
echo "  rtl_433 -G               # List rtl_433 supported devices"
echo "  hackrf_info              # HackRF device info"
echo "  LimeUtil --find          # Find LimeSDR devices"
echo "  SoapySDRUtil --find      # Find all SoapySDR devices"
echo "  kal -s GSM850            # Scan GSM 850 band"
echo "  gpsmon                   # Monitor GPS"
echo ""
echo -e "${CYAN}Example usage:${NC}"
echo "  rtl_433 -f 433920000           # Monitor 433.92 MHz ISM"
echo "  rtl_fm -f 162.55M -M fm        # NOAA Weather Radio"
echo "  hackrf_sweep -f 2400:2500      # Scan 2.4 GHz WiFi band"
echo ""
