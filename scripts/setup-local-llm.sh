#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# SIGINT-Deck Local LLM Setup
# ═══════════════════════════════════════════════════════════════════════════════
#
# This script installs a local LLM for privacy-preserving device analysis.
# All AI processing happens on-device - no data sent to cloud.
#
# Options:
#   1. Ollama (recommended) - Easy to use, manages models automatically
#   2. llama.cpp - More control, supports BitNet/ternary models
#
# Usage:
#   ./setup-local-llm.sh              # Interactive menu
#   ./setup-local-llm.sh ollama       # Install Ollama + tinyllama
#   ./setup-local-llm.sh llamacpp     # Install llama.cpp + download model
#   ./setup-local-llm.sh bitnet       # Install BitNet (experimental)
#
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

INSTALL_DIR="${SIGINT_INSTALL_DIR:-$HOME/sigint-deck}"
LLM_DIR="$INSTALL_DIR/llm"
MODELS_DIR="$LLM_DIR/models"

# Detect platform
detect_platform() {
    if grep -q "SteamOS" /etc/os-release 2>/dev/null; then
        PLATFORM="steamdeck"
        ARCH="x86_64"
        RAM_GB=16
    elif [ "$(uname -m)" = "aarch64" ]; then
        PLATFORM="arm64"
        ARCH="aarch64"
        RAM_GB=$(free -g | awk '/^Mem:/{print $2}')
    else
        PLATFORM="x86_64"
        ARCH="x86_64"
        RAM_GB=$(free -g | awk '/^Mem:/{print $2}')
    fi
    echo -e "${CYAN}Platform: $PLATFORM ($ARCH) - ${RAM_GB}GB RAM${NC}"
}

# Recommend model based on RAM
recommend_model() {
    if [ "$RAM_GB" -ge 16 ]; then
        RECOMMENDED_MODEL="phi3:mini"
        RECOMMENDED_SIZE="2.3GB"
    elif [ "$RAM_GB" -ge 8 ]; then
        RECOMMENDED_MODEL="tinyllama"
        RECOMMENDED_SIZE="637MB"
    else
        RECOMMENDED_MODEL="qwen2.5:0.5b"
        RECOMMENDED_SIZE="400MB"
    fi
    echo -e "${GREEN}Recommended model: $RECOMMENDED_MODEL ($RECOMMENDED_SIZE)${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# OLLAMA INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════

install_ollama() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}                Installing Ollama${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    # Check if already installed
    if command -v ollama &> /dev/null; then
        echo -e "${GREEN}✓ Ollama already installed${NC}"
        ollama --version
    else
        echo "Downloading and installing Ollama..."
        curl -fsSL https://ollama.ai/install.sh | sh
    fi
    
    # Start Ollama service
    echo "Starting Ollama service..."
    if systemctl --user is-active ollama &>/dev/null; then
        echo -e "${GREEN}✓ Ollama service already running${NC}"
    else
        # Try user service first, fall back to system
        systemctl --user start ollama 2>/dev/null || \
        sudo systemctl start ollama 2>/dev/null || \
        (ollama serve &>/dev/null &)
        sleep 3
    fi
    
    # Pull recommended model
    echo ""
    echo -e "${CYAN}Pulling $RECOMMENDED_MODEL model ($RECOMMENDED_SIZE)...${NC}"
    echo "This may take a few minutes on first run."
    ollama pull "$RECOMMENDED_MODEL"
    
    # Test
    echo ""
    echo "Testing model..."
    echo "What is 2+2?" | ollama run "$RECOMMENDED_MODEL" --nowordwrap 2>/dev/null | head -3
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                Ollama Installation Complete${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "Update your SIGINT-Deck config.toml:"
    echo ""
    echo -e "${CYAN}[llm]"
    echo "enabled = true"
    echo "provider = \"ollama\""
    echo "endpoint = \"http://localhost:11434\""
    echo -e "model = \"$RECOMMENDED_MODEL\"${NC}"
    echo ""
    
    # Update config if exists
    if [ -f "$INSTALL_DIR/config.toml" ]; then
        read -p "Update config.toml automatically? [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            update_config "ollama" "http://localhost:11434" "$RECOMMENDED_MODEL"
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# LLAMA.CPP INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════════

install_llamacpp() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}                Installing llama.cpp${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    
    mkdir -p "$LLM_DIR" "$MODELS_DIR"
    cd "$LLM_DIR"
    
    # Check for existing installation
    if [ -f "$LLM_DIR/llama.cpp/llama-server" ]; then
        echo -e "${GREEN}✓ llama.cpp already built${NC}"
    else
        echo "Cloning llama.cpp..."
        git clone https://github.com/ggerganov/llama.cpp.git 2>/dev/null || \
            (cd llama.cpp && git pull)
        
        cd llama.cpp
        
        echo "Building llama.cpp (this may take 5-10 minutes)..."
        make -j$(nproc) llama-server llama-cli
        
        echo -e "${GREEN}✓ llama.cpp built successfully${NC}"
    fi
    
    # Download TinyLlama model
    cd "$MODELS_DIR"
    MODEL_FILE="tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf"
    MODEL_URL="https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/$MODEL_FILE"
    
    if [ -f "$MODEL_FILE" ]; then
        echo -e "${GREEN}✓ TinyLlama model already downloaded${NC}"
    else
        echo "Downloading TinyLlama model (669MB)..."
        wget -q --show-progress "$MODEL_URL" -O "$MODEL_FILE"
    fi
    
    # Create systemd service
    echo "Creating llama.cpp service..."
    mkdir -p "$HOME/.config/systemd/user"
    cat > "$HOME/.config/systemd/user/llamacpp.service" << EOF
[Unit]
Description=llama.cpp Server for SIGINT-Deck
After=network.target

[Service]
Type=simple
WorkingDirectory=$LLM_DIR/llama.cpp
ExecStart=$LLM_DIR/llama.cpp/llama-server -m $MODELS_DIR/$MODEL_FILE --host 0.0.0.0 --port 8081 -c 2048
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
EOF

    systemctl --user daemon-reload
    systemctl --user enable llamacpp.service
    systemctl --user start llamacpp.service
    
    sleep 3
    
    # Test
    echo ""
    echo "Testing llama.cpp server..."
    curl -s http://localhost:8081/health && echo -e "${GREEN}✓ Server responding${NC}"
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                llama.cpp Installation Complete${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "Update your SIGINT-Deck config.toml:"
    echo ""
    echo -e "${CYAN}[llm]"
    echo "enabled = true"
    echo "provider = \"llamacpp\""
    echo "endpoint = \"http://localhost:8081\""
    echo -e "model = \"tinyllama\"${NC}"
    echo ""
    echo "Service commands:"
    echo "  Start:  systemctl --user start llamacpp"
    echo "  Stop:   systemctl --user stop llamacpp"
    echo "  Logs:   journalctl --user -u llamacpp -f"
    
    # Update config if exists
    if [ -f "$INSTALL_DIR/config.toml" ]; then
        read -p "Update config.toml automatically? [Y/n] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            update_config "llamacpp" "http://localhost:8081" "tinyllama"
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# BITNET INSTALLATION (Experimental)
# ═══════════════════════════════════════════════════════════════════════════════

install_bitnet() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}        Installing BitNet (1-bit/Ternary LLM) - EXPERIMENTAL${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${YELLOW}⚠ BitNet is experimental and may not work on all systems${NC}"
    echo -e "${YELLOW}⚠ Requires ~2GB disk space for build${NC}"
    echo ""
    
    read -p "Continue with BitNet installation? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Cancelled."
        return
    fi
    
    mkdir -p "$LLM_DIR" "$MODELS_DIR"
    cd "$LLM_DIR"
    
    # Clone BitNet repo
    if [ -d "BitNet" ]; then
        echo "Updating BitNet..."
        cd BitNet && git pull
    else
        echo "Cloning Microsoft BitNet..."
        git clone --recursive https://github.com/microsoft/BitNet.git
        cd BitNet
    fi
    
    # Install Python requirements
    echo "Installing Python dependencies..."
    python3 -m pip install --user -r requirements.txt 2>/dev/null || \
        pip3 install --user -r requirements.txt
    
    # Build
    echo "Building BitNet (this may take 10-15 minutes)..."
    python3 setup_env.py --hf-repo HF1BitLLM/Llama3-8B-1.58-100B-tokens -q i2_s
    
    echo ""
    echo -e "${GREEN}BitNet installation complete.${NC}"
    echo ""
    echo "To run BitNet:"
    echo "  cd $LLM_DIR/BitNet"
    echo "  python3 run_inference.py -m models/Llama3-8B-1.58-100B-tokens/ggml-model-i2_s.gguf -p 'What is an AirTag?'"
    echo ""
    echo -e "${YELLOW}Note: BitNet integration with SIGINT-Deck is experimental.${NC}"
    echo "Consider using Ollama or llama.cpp for production use."
}

# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

update_config() {
    local provider=$1
    local endpoint=$2
    local model=$3
    
    echo "Updating config.toml..."
    
    # Use Python for reliable TOML editing
    python3 << PYEOF
import re

with open("$INSTALL_DIR/config.toml", "r") as f:
    content = f.read()

# Update LLM section
llm_section = """[llm]
enabled = true
provider = "$provider"
endpoint = "$endpoint"
model = "$model"
max_tokens = 200
timeout_secs = 60"""

# Replace existing [llm] section or append
if "[llm]" in content:
    content = re.sub(r'\[llm\].*?(?=\n\[|\Z)', llm_section + '\n', content, flags=re.DOTALL)
else:
    content += '\n' + llm_section + '\n'

with open("$INSTALL_DIR/config.toml", "w") as f:
    f.write(content)

print("✓ Config updated")
PYEOF
    
    # Restart SIGINT-Deck to pick up changes
    if systemctl --user is-active sigint-deck &>/dev/null; then
        echo "Restarting SIGINT-Deck..."
        systemctl --user restart sigint-deck
    fi
}

show_menu() {
    echo -e "${BLUE}"
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║           SIGINT-Deck Local LLM Setup                      ║"
    echo "║                                                            ║"
    echo "║  Privacy-preserving AI - all processing stays on device   ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
    detect_platform
    recommend_model
    echo ""
    echo "Choose installation method:"
    echo ""
    echo -e "  ${GREEN}1)${NC} Ollama (Recommended)"
    echo "     Easy to use, automatic model management"
    echo "     Best for: Most users"
    echo ""
    echo -e "  ${GREEN}2)${NC} llama.cpp"
    echo "     More control, manual model loading"
    echo "     Best for: Advanced users, custom models"
    echo ""
    echo -e "  ${YELLOW}3)${NC} BitNet (Experimental)"
    echo "     1-bit/ternary models, very efficient"
    echo "     Best for: Experimentation, edge devices"
    echo ""
    echo -e "  ${RED}4)${NC} Cancel"
    echo ""
    read -p "Select [1-4]: " choice
    
    case $choice in
        1) install_ollama ;;
        2) install_llamacpp ;;
        3) install_bitnet ;;
        4) echo "Cancelled." ;;
        *) echo "Invalid choice" ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    detect_platform
    recommend_model
    
    case "${1:-}" in
        ollama)
            install_ollama
            ;;
        llamacpp|llama)
            install_llamacpp
            ;;
        bitnet)
            install_bitnet
            ;;
        "")
            show_menu
            ;;
        *)
            echo "Usage: $0 [ollama|llamacpp|bitnet]"
            exit 1
            ;;
    esac
}

main "$@"
