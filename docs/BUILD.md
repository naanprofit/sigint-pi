# Building SIGINT-Deck from Source

## Prerequisites

### All Platforms
- Rust toolchain (1.75+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Git: `git clone https://github.com/naanprofit/sigint-pi.git`

### Raspberry Pi (ARM64) - Cross-compile on Mac/Linux
- Docker with buildx: `docker buildx version`
- QEMU user static (for ARM64 emulation): usually included with Docker Desktop

### Steam Deck (x86_64) - Native build
- Developer mode enabled, password set
- Rust installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Build essentials: `sudo pacman -S base-devel openssl pkg-config`

## Build for Raspberry Pi (ARM64)

Cross-compile on a Mac or Linux machine using Docker:

```bash
cd sigint-deck

# Build ARM64 binary via Docker (takes ~8-10 minutes first time, ~1-2 min cached)
docker buildx build \
  --platform linux/arm64 \
  -f docker/Dockerfile.pi \
  --target builder \
  -t sigint-pi-builder:latest \
  --load .

# Extract the binary
docker create --name pi-extract sigint-pi-builder:latest
docker cp pi-extract:/build/target/release/sigint-deck ./sigint-deck-arm64
docker rm pi-extract

# Verify
file sigint-deck-arm64
# sigint-deck-arm64: ELF 64-bit LSB pie executable, ARM aarch64, ...
```

### Deploy to Pi

```bash
# Copy binary
scp sigint-deck-arm64 pi@<PI_IP>:~/sigint-deck/sigint-deck

# Set capabilities (required for WiFi monitor mode and raw sockets)
ssh pi@<PI_IP>
chmod +x ~/sigint-deck/sigint-deck
sudo setcap cap_net_raw,cap_net_admin+eip ~/sigint-deck/sigint-deck

# Copy static files and config
scp -r static/ pi@<PI_IP>:~/sigint-deck/
scp config.toml.example pi@<PI_IP>:~/sigint-deck/config.toml

# Start
cd ~/sigint-deck
SIGINT_ACCEPT_DISCLAIMER=1 ./sigint-deck
```

## Build for Steam Deck (x86_64)

Build natively on the Deck:

```bash
# SSH into Deck
ssh deck@<DECK_IP>

cd ~/sigint-deck

# Install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build release binary (takes ~2-3 minutes)
cargo build --release

# Binary is at target/release/sigint-deck
ls -lh target/release/sigint-deck

# Copy to working directory
cp target/release/sigint-deck ~/sigint-deck/sigint-deck

# Set capabilities
sudo setcap cap_net_raw,cap_net_admin+eip ~/sigint-deck/sigint-deck
```

## Build for Development (Mac/Linux)

```bash
cd sigint-deck

# Debug build (faster, larger binary)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check without building (faster feedback)
cargo check
```

## Docker Build Details

The `docker/Dockerfile.pi` is a multi-stage build:

1. **Builder stage**: Compiles the Rust binary for ARM64
   - Uses `rust:1.85-bookworm` base
   - Installs ARM64 cross-compilation toolchain
   - Produces `target/release/sigint-deck` (~12MB)

2. **Runtime stage** (optional): Packages binary with SDR tools
   - Includes rtl-sdr, hackrf, kalibrate-rtl, adb
   - Produces a complete Docker image (~340MB)

## Configuration

After building, create a config file:

```bash
cp config.toml.example ~/sigint-deck/config.toml
```

Key settings to modify:
- `device.name` - Device identifier
- `web.port` - Web UI port (8080 for Pi, 8085 for Deck)
- `wifi.interface` - WiFi adapter name (usually `wlan1`)
- `llm.enabled` / `llm.endpoint` - AI analysis settings
- `rayhunter.enabled` / `rayhunter.api_url` - IMSI catcher detection

## Systemd Service (Auto-start)

### Raspberry Pi

```bash
sudo tee /etc/systemd/system/sigint-deck.service << 'EOF'
[Unit]
Description=SIGINT-Deck Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/sigint-deck
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
ExecStart=/home/pi/sigint-deck/sigint-deck
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable sigint-deck
sudo systemctl start sigint-deck
```

### Steam Deck (User Service)

```bash
mkdir -p ~/.config/systemd/user

cat > ~/.config/systemd/user/sigint-deck.service << 'EOF'
[Unit]
Description=SIGINT-Deck Security Scanner
After=network.target bluetooth.target

[Service]
Type=simple
WorkingDirectory=/home/deck/sigint-deck
Environment=SIGINT_ACCEPT_DISCLAIMER=1
Environment=RUST_LOG=info
ExecStart=/home/deck/sigint-deck/sigint-deck
Restart=always
RestartSec=5

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reload
systemctl --user enable sigint-deck
systemctl --user start sigint-deck
```

## Verify Build

After deploying, verify the service is running:

```bash
# Check status
curl http://localhost:8080/api/status   # Pi
curl http://localhost:8085/api/status   # Deck

# Check SDR hardware detection
curl http://localhost:8080/api/sdr/status

# Check RayHunter (Pi only, with phone connected)
curl http://localhost:8080/api/rayhunter/status
```

## Troubleshooting

### Compile Errors
- Ensure Rust 1.75+ with `rustc --version`
- On Deck, ensure `openssl-devel` and `pkg-config` are installed
- If Docker build fails, try `docker buildx prune` to clear cache

### Runtime Errors
- `Permission denied` on WiFi: Run `sudo setcap cap_net_raw,cap_net_admin+eip ./sigint-deck`
- `Failed to set datalink type`: WiFi adapter not in monitor mode, run `sudo airmon-ng start wlan1`
- `accept thread stopped` on Deck: This was a known bug fixed in v0.2.2 (channel hopper backoff)
- Web UI not loading: Check port with `ss -tlnp | grep 8080` and firewall rules
