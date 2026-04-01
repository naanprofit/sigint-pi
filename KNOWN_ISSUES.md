# Known Issues

Last updated: 2026-04-01

## Hardware Issues

### RTL-SDR PLL Lock Warning on Steam Deck
- **Symptom**: `[R82XX] PLL not locked!` in logs, `rtl_power` produces no CSV output
- **Affected**: Steam Deck (RTL2838UHIDIR with R820T tuner)
- **Impact**: `rtl_power` scans return empty data
- **Workaround**: The system automatically falls back to `rtl_sdr` raw IQ capture + Python FFT processing. This is slower (~3-5s per scan) but produces valid spectrum data. `rtl_fm` and `rtl_sdr` work fine for tuning/receiving.
- **Status**: Mitigated with fallback chain

### HackRF Sweep Pipe Error on Raspberry Pi
- **Symptom**: `hackrf_start_rx_sweep() failed: Pipe error (-1000)`
- **Affected**: Raspberry Pi with HackRF One (firmware `local-883a622`, API 1.04)
- **Impact**: `hackrf_sweep` always fails
- **Workaround**: The system falls back to `hackrf_transfer` raw IQ capture + Python FFT. `hackrf_transfer` works perfectly (16 MiB/s). Consider updating HackRF firmware to an official release.
- **Status**: Mitigated with fallback chain

### Steam Deck WiFi Channel Hopper Resource Exhaustion
- **Symptom**: Web server `accept thread stopped` after ~23 seconds, port 8085 stops responding
- **Cause**: Channel hopper spawning `sudo iw dev wlan1 set channel X` every 500ms with PAM auth, exhausting file descriptors
- **Fix**: Applied exponential backoff (30s after 10 consecutive failures). Fixed in commit ca377e3.
- **Status**: Fixed

## Software Issues

### LLM Settings Require Restart
- **Symptom**: After saving LLM settings, the running service uses old config for device analysis
- **Workaround**: Test Connection button now saves + reads from disk. For device analysis, restart the service after changing LLM settings.
- **Status**: Partially fixed (test works immediately, full analysis requires restart)

### RayHunter ADB Port Conflict
- **Symptom**: RayHunter shows "disconnected" in web UI
- **Cause**: Three different ports used historically (8080, 8081, 8082). The code now defaults to 8081 and auto-manages ADB forward.
- **Fix**: Set `rayhunter.api_url = "http://localhost:8081"` in config.toml. The system auto-creates `adb forward tcp:8081 tcp:8080`.
- **Status**: Fixed, ensure ADB is installed and phone connected via USB

### TSCM Sweep False Positives
- **Symptom**: Ambient RF noise triggers threat alerts (e.g., "Federal Surveillance" at -55 dBm)
- **Cause**: Threshold too sensitive; any signal in a surveillance band above -60 dBm triggers alert
- **Workaround**: The threshold is set per-sweep type. Use "quick" sweep for lower sensitivity. Signals below -45 dBm are likely ambient noise.
- **Status**: Under investigation - threshold tuning needed

### Spectrum Analyzer Dropdown - High Frequency Bands
- **Symptom**: "Drone 5.8 GHz" and "WiFi 2.4 GHz" bands return error on Deck (RTL-SDR only)
- **Cause**: These frequencies exceed RTL-SDR range (24-1766 MHz), requires HackRF
- **Fix**: UI now shows clear error message with hardware requirements
- **Status**: By design - hardware limitation

### Audio Streaming Latency
- **Symptom**: Browser audio from radio tuner has 1-3 second latency
- **Cause**: Chunked HTTP streaming with 100ms polling + Web Audio API buffer scheduling
- **Workaround**: Acceptable for monitoring. For real-time listening, use local `aplay` on the device.
- **Status**: Known limitation

## Device-Specific Notes

### Steam Deck
- Internal WiFi (`wlan0`) does NOT support monitor mode - external USB adapter required
- Sleep/suspend kills the web server - use `systemd-inhibit` or disable sleep
- SteamOS filesystem is read-only by default; use `~/bin/` for additional tools
- Device may go offline periodically when in Game Mode

### Raspberry Pi
- Pi Zero 2 W has limited RAM (512MB) - consider swap file
- USB bandwidth shared between WiFi adapter, HackRF, GPS, and RayHunter phone
- WiFi adapter must be on `wlan1`, built-in WiFi stays on `wlan0`
- HackRF One firmware may need updating for `hackrf_sweep` to work

### RayHunter (Orbic RC400L)
- Must be connected via USB data cable (not charge-only)
- ADB must be installed on the host (`sudo apt install adb`)
- Phone runs EFF RayHunter daemon at `/data/rayhunter/rayhunter-daemon`
- Web UI accessible at `http://localhost:8081` after ADB port forward
- Qualcomm MDM9207-MTP device (USB ID: 05c6:f601)
