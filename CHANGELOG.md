# Changelog

## [0.2.3] - 2026-04-01

### Fixed
- **SDR Hardware Detection**: `SdrCapabilities::detect()` now checks actual hardware presence (USB device probing) instead of just binary existence. Previously a device with `hackrf_sweep` installed but no HackRF hardware would report `hackrf=true` and then fail on every scan.
- **Spectrum Analyzer Fallbacks**: When `rtl_power` fails (PLL lock issue on some RTL-SDR dongles) or `hackrf_sweep` fails (pipe error), the system falls back to raw IQ capture + Python FFT automatically.
- **RayHunter ADB Integration**: Fixed ADB `ensure_adb_forward()` to use full binary path (`/usr/bin/adb`), set `HOME` and `ANDROID_SDK_HOME` environment variables, and properly parse `adb devices` output (tab-delimited format).
- **RayHunter API Rewrite**: Replaced shell script polling (`rayhunter-poll.sh`) with direct HTTP client calls to the EFF RayHunter API. Web handler now accepts `Config` parameter for dynamic configuration.
- **RayHunter Port Consistency**: Standardized on port 8081 across config default, install scripts, and ADB forward setup. Previously had conflicting ports (8080, 8081, 8082).
- **LLM Settings Persistence**: "Test Connection" now saves to disk first, then reads the saved config for testing, instead of reading stale in-memory config from startup.
- **LLM URL Sanitization**: Added `sanitize_llm_url()` that strips URL fragments (`#/`), query strings, and trailing slashes that could break API calls (e.g., `http://host:8080/#/` -> `http://host:8080`).
- **LLM UI Missing Fields**: Added model name and API key input fields to the web UI settings form. Previously the UI only sent the endpoint URL.
- **Channel Hopper Backoff**: Deck channel hopper now waits 30s after 10 consecutive `iw set channel` failures instead of retrying every 500ms and exhausting system resources.
- **TSCM Frequency Truncation**: Fixed `start_mhz = start_hz / 1_000_000` integer truncation that dropped sub-MHz precision. Now uses proper float division.
- **EMI Scan Invalid Flag**: Removed `-E 2` flag from `rtl_power` EMI scans (not supported on all builds).
- **resolve_sdr_command() Wiring**: The custom binary path resolver is now actually used by all SDR command invocations. Previously defined but never called.
- **Blocking SDR Commands**: Converted `std::process::Command` to `tokio::process::Command` for all SDR operations to avoid blocking the async runtime.
- **RiskLevel Comparison**: Added `PartialOrd`/`Ord` derives to `RiskLevel` enum so threat levels can be compared.

### Added
- **Browser Audio Streaming**: New `/api/sdr/radio/stream` endpoint streams raw 16-bit signed LE PCM audio via chunked HTTP. Web UI uses Web Audio API to decode and play in real-time. Radio tuner now sends audio to browser instead of only local `aplay`.
- **Distance Estimation**: `estimate_distance()` function uses free-space path loss model (FSPL) to estimate RF source distance. Returns min/max range accounting for transmit power variance (1mW to 1W). Added to TSCM threat detections and drone scan results.
- **RayHunter HTTP Client**: New `RayHunterClient` struct with methods for all EFF RayHunter API endpoints: `get_system_stats()`, `get_manifest()`, `get_analysis_report()`, `get_full_status()`. Parses newline-delimited JSON from analysis reports.
- **RayHunter Recording Control**: New API endpoints `POST /api/rayhunter/start-recording` and `POST /api/rayhunter/stop-recording` for controlling RayHunter QMDL recording.
- **Continuous TSCM Scanning**: TSCM sweep loops continuously until stopped, with 1s polling interval. Shows current band, sweep count, per-threat first/last seen timestamps and sighting count.
- **Continuous Drone Monitoring**: Drone scan loops through military RF bands (UHF 320-400 MHz, ISM 868-930 MHz, L-band 1200-1300 MHz) plus EMI 24-30 MHz continuously until stopped.
- **Military Drone Frequency Database**: Research from open-source intelligence covering drones from Russia (Orlan-10, Lancet, ZALA), Iran (Shahed-136), China (CH-3A, CH-4B), Turkey (Bayraktar TB2), USA (Predator, ScanEagle, Switchblade, Anduril Ghost), and Israel (Hermes, Harop). Includes GPS L1, GLONASS L1, NATO STANAG 4586 bands.
- **Contact Tracking**: All RF detections (TSCM threats, drone signals, SDR anomalies) now track `first_seen`, `last_seen`, and `sightings` count.
- **TSCM Sweep Tests**: `tests/tscm_sweep_test.rs` with synthetic spectrum data for CSV parsing, threshold detection, multi-band detection, false positive rejection, and sightings accumulation.
- **Alert Types**: New alert categories: `DroneDetected`, `TscmThreat`, `RfAnomaly`, `SurveillanceDevice`, `GeofenceBreach` with per-type routing rules.
- **Comprehensive Documentation**: `KNOWN_ISSUES.md`, updated `BUILD.md`, `INSTALL.md`, install scripts for both Pi and Deck.
- **Install Scripts**: `scripts/install-pi.sh` (full Pi setup with dependencies, SDR tools, udev rules, systemd, ADB forward service), `scripts/install-deck.sh` (full Deck setup including Rust, SDR tools from source, native build).

### Changed
- **RayHunter UI**: Updated status display to show version, OS, recording status, total entries, and warning count instead of generic connected/disconnected states. Threat details now show structured findings instead of raw JSON.
- **tscm_scan_band()**: Automatically routes scans to the correct SDR hardware based on frequency range and available devices.
- **SDR Status Endpoint**: Returns actual device presence (probed via USB) not just binary availability.

## [0.2.2] - 2026-03-31

### Fixed
- **GPS Retry Loop**: Fixed infinite fast retry when gpsd is unavailable
  - Added exponential backoff (5s initial, doubling up to 300s max)
  - GPS module now checks config.enabled flag and sleeps forever if disabled
  - Reduced log spam after 4 consecutive failures (switches to debug level)
  - Fast-fail connection test before spawning reader thread

### Added
- **Comprehensive Hardware Documentation** (`docs/PI_REQUIREMENTS.md`)
  - Complete list of supported hardware (WiFi, BLE, GPS, SDR, RayHunter)
  - System package requirements for Raspberry Pi
  - USB device summary and configuration files
  - Quick hardware check commands

- **Pi Setup Scripts**
  - `scripts/pi-gps-setup.sh` - U-blox U7 GPS configuration with udev rules
  - `scripts/pi-full-setup.sh` - Complete system setup including:
    - WiFi tools (aircrack-ng, iw)
    - Bluetooth (bluez)
    - GPS (gpsd)
    - SDR tools (rtl-sdr, rtl_433, kalibrate-rtl, hackrf)
    - RayHunter support (adb)
    - Pi optimizations (GPU memory, swap)
    - Systemd service configuration

- **Docker Image Enhancements**
  - Added SDR tools to runtime images (Pi and Steam Deck):
    - rtl-sdr, rtl_433
    - hackrf (where available)
    - kalibrate-rtl (built from source)
    - adb for RayHunter
    - aircrack-ng for WiFi
  - Added udev rules for SDR devices

### Hardware Support
- **WiFi**: External USB adapters with monitor mode (Alfa AWUS036ACHM, RTL8812AU)
- **Bluetooth/BLE**: Built-in Pi Bluetooth for AirTag/tracker detection
- **GPS**: U-blox U7 via gpsd (VK-172, G-Mouse)
- **RTL-SDR**: ISM band monitoring with rtl_433
- **HackRF One**: Wideband spectrum analysis, drone detection
- **LimeSDR**: Advanced SDR applications
- **RayHunter**: IMSI catcher detection via Android phone + ADB

### Configuration Notes
- GPS can be disabled in config.toml to prevent any connection attempts
- Bluetooth must be unblocked via rfkill on Pi (`sudo rfkill unblock bluetooth`)
- Web server must be explicitly enabled (`[web] enabled = true`)
- WiFi scanner expects external adapter on wlan1 (not built-in wlan0)

### Deployment Notes
- Binary size: ~12MB (ARM64)
- Docker image size: ~340MB (Pi ARM64 with SDR tools)
- Tested on Raspberry Pi Zero 2 W (Debian Bookworm)

## [0.2.1] - Previous Release

### Features
- Multi-platform support (Steam Deck, Raspberry Pi)
- WiFi monitoring with monitor mode
- BLE scanning with AirTag detection
- GPS integration via gpsd
- SDR framework (RTL-SDR, HackRF, LimeSDR)
- RayHunter IMSI catcher detection
- OpenClaw mesh networking
- Meshtastic LoRa integration
- Web UI dashboard
- Alert system (Telegram, Twilio, Email, MQTT)
