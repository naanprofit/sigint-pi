## sigint-pi v0.3.0 - Antenna Arrays, SIEM, Sentinel Mode, Drone DF

### Multi-Device SDR & Antenna Arrays
- Detect ALL RTL-SDR devices (indices), ALL HackRF devices (serials)
- Auto-identify KrakenSDR (5ch) and KerberosSDR (4ch) coherent arrays
- Added Airspy, SDRplay, PlutoSDR detection
- Antenna position database with X/Y/Z coordinates, bearing, gain
- Quick setup presets: 4-Sector, 4-Sector Dual, KrakenSDR Center, Full 13-antenna
- Interactive canvas map with compass rose, range rings, color-coded bearing lines

### SIEM Event System
- SQLite FTS5 full-text search across all events
- 4GB rolling log budget with automatic pruning
- Time presets (Last Hour/24h/7d/30d), custom date range picker
- Watch mode with 5-second auto-refresh and rolling time window
- Export events to JSON

### Sentinel Mode
- Continuous autonomous threat monitoring toggle
- Starts all SDR monitors, 30-second watchlist scanning loop
- Threat watchlist database with MAC and RF signature matching
- Alerts via TTS, webhook, email, Telegram

### Monitor Mode Diagnostics
- Step-by-step command output with pass/fail per step
- Replaced airmon-ng with nmcli device release (safe over SSH)
- sudo iw for phy detection and mode verification
- Frontend diagnostic panel shows exactly what failed and why

### Comprehensive Setup Guide & BOM
- Layer 1: 4-Sector sentries (RTL-SDR + wideband LPDA, HackRF + directional)
- Layer 2: Central KrakenSDR DF node ($749 + $249 Krakentenna)
- Full bill of materials with quantities, purposes, pricing
- Budget summary: $1,800-$3,700 added cost
- Baseline subtraction range estimates (EMI: 100-250m, RF: kilometers)
- Altitude coverage, antenna beamwidth guidance
- Fiber-optic drone detection notes
- Legal notes (receive-only, FCC Part 15)

### Easter Egg
- Hidden SIGINT Training Ops game link in Settings (click the dots 5x)

### Binary
- `sigint-pi-v0.3.0-aarch64-linux`: Raspberry Pi 4/5 (aarch64)
