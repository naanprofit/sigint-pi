#!/bin/bash
# Setup Python virtual environment with STT/TTS dependencies
# Works on SteamOS (Deck), Raspberry Pi, and ClockworkPi uConsole
#
# Installs:
#   - faster-whisper (speech-to-text via Whisper)
#   - piper-tts (text-to-speech)
#
# The venv is created inside the install directory so it travels
# with the rest of the installation.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# Detect install directory
if [ -d "$HOME/sigint-deck" ]; then
    INSTALL_DIR="$HOME/sigint-deck"
elif [ -d "$HOME/sigint-pi" ]; then
    INSTALL_DIR="$HOME/sigint-pi"
elif [ -d "$HOME/sigint-clockworkpi" ]; then
    INSTALL_DIR="$HOME/sigint-clockworkpi"
else
    echo -e "${RED}No sigint install directory found. Run the install script first.${NC}"
    exit 1
fi

VENV_DIR="$INSTALL_DIR/venv"

echo ""
echo -e "${CYAN}======================================${NC}"
echo -e "${CYAN} Python STT/TTS Setup${NC}"
echo -e "${CYAN}======================================${NC}"
echo ""
echo -e "Install dir: ${CYAN}$INSTALL_DIR${NC}"
echo -e "Venv dir:    ${CYAN}$VENV_DIR${NC}"
echo ""

# ============================================
# Step 1: Create venv
# ============================================
echo -e "${YELLOW}[1/4] Creating Python virtual environment...${NC}"
if [ ! -d "$VENV_DIR" ]; then
    python3 -m venv "$VENV_DIR"
    echo -e "${GREEN}  venv created${NC}"
else
    echo -e "${GREEN}  venv already exists${NC}"
fi

# Upgrade pip
"$VENV_DIR/bin/pip" install --upgrade pip -q

# ============================================
# Step 2: Install faster-whisper (STT)
# ============================================
echo -e "${YELLOW}[2/4] Installing faster-whisper (speech-to-text)...${NC}"
"$VENV_DIR/bin/pip" install faster-whisper -q
echo -e "${GREEN}  faster-whisper installed${NC}"

# ============================================
# Step 3: Install piper-tts (TTS)
# ============================================
echo -e "${YELLOW}[3/4] Installing piper-tts (text-to-speech)...${NC}"
"$VENV_DIR/bin/pip" install piper-tts -q
echo -e "${GREEN}  piper-tts installed${NC}"

# ============================================
# Step 4: Setup whisper server systemd service
# ============================================
echo -e "${YELLOW}[4/4] Setting up whisper server service...${NC}"

# Detect if we should use system or user service
if command -v systemctl &>/dev/null; then
    SERVICE_DIR="$HOME/.config/systemd/user"
    mkdir -p "$SERVICE_DIR"

    cat > "$SERVICE_DIR/whisper-server.service" << SERVICE
[Unit]
Description=SIGINT Whisper Transcription Server
After=network.target

[Service]
Type=simple
WorkingDirectory=$INSTALL_DIR
Environment=WHISPER_MODEL=tiny
Environment=WHISPER_DEVICE=cpu
Environment=WHISPER_PORT=5000
ExecStart=$VENV_DIR/bin/python $INSTALL_DIR/whisper_server.py
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
SERVICE

    systemctl --user daemon-reload 2>/dev/null || true
    systemctl --user enable whisper-server.service 2>/dev/null || true
    echo -e "${GREEN}  whisper-server.service installed and enabled${NC}"
fi

# ============================================
# Verify
# ============================================
echo ""
echo -e "${CYAN}Verifying installation...${NC}"
"$VENV_DIR/bin/python" -c "import faster_whisper; print('  faster-whisper:', faster_whisper.__version__)" 2>/dev/null || echo -e "${RED}  faster-whisper FAILED${NC}"
"$VENV_DIR/bin/python" -c "import piper; print('  piper-tts: OK')" 2>/dev/null || echo -e "${RED}  piper-tts FAILED${NC}"

echo ""
echo -e "${GREEN}======================================${NC}"
echo -e "${GREEN} Setup Complete!${NC}"
echo -e "${GREEN}======================================${NC}"
echo ""
echo -e "${YELLOW}To start whisper server:${NC}"
echo "  systemctl --user start whisper-server"
echo ""
echo -e "${YELLOW}To test manually:${NC}"
echo "  $VENV_DIR/bin/python $INSTALL_DIR/whisper_server.py"
echo ""
echo -e "${YELLOW}Piper TTS usage:${NC}"
echo "  echo 'Hello world' | $VENV_DIR/bin/piper --model en_US-lessac-medium --output_file out.wav"
echo ""
echo -e "${YELLOW}Note: First run downloads the Whisper model (~40MB for tiny).${NC}"
echo ""
