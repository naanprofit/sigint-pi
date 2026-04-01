# HackRF One Integration for SIGINT-Deck

## Overview

The HackRF One provides wideband RF monitoring capabilities (1 MHz - 6 GHz) that complement WiFi/BLE scanning with deeper signal intelligence.

## Implemented Capabilities

### 1. Spectrum Sweeping (`hackrf_sweep`)
- **Wideband monitoring**: Scan large frequency ranges quickly
- **Drone detection**: Monitor 2.4 GHz and 5.8 GHz bands for drone control signals
- **Interference detection**: Find RF jammers and unauthorized transmitters

### 2. ISM Band Monitoring (via `rtl_433` with RTL-SDR, or custom HackRF)
- **433 MHz**: Garage doors, car key fobs, weather stations, some trackers
- **315 MHz**: US car key fobs, some security systems
- **868 MHz**: European ISM devices, LoRa
- **915 MHz**: US LoRa, smart meters

### 3. Cellular Tower Scanning (`kalibrate-rtl`)
- **GSM tower mapping**: Find cell towers and their frequencies
- **IMSI catcher correlation**: Compare with RayHunter data
- **Anomaly detection**: Spot fake base stations

## Planned/Possible Capabilities

### 4. Drone Detection & Tracking
```
Frequency ranges to monitor:
- 2.400-2.483 GHz: WiFi-based drones (DJI, Parrot)
- 5.725-5.875 GHz: 5 GHz drone video links
- 433 MHz: Some drone telemetry
- 900 MHz: Long-range drone control
- 1.2 GHz: Analog video transmitters (FPV)
```

### 5. Vehicle Key Fob Analysis
- Detect rolling code transmissions
- Log vehicle presence by RF signature
- Correlate with WiFi/BLE MAC addresses

### 6. LoRa/Meshtastic Monitoring
- Monitor LoRa channels for Meshtastic traffic
- Detect LoRa-based trackers
- Correlate with Meshtastic integration

### 7. Directed Energy Weapon Detection
Based on our research, monitor for:
- **Microwave weapons**: 1-100 GHz anomalous emissions
- **RF harassment**: Strong signals in specific bands
- **Pulsed signals**: Detect unusual modulation patterns

### 8. Surveillance Device Detection
- Bug sweeping: Scan for active RF transmitters
- Hidden camera detection: Look for video transmitter signatures
- GPS tracker detection: 433 MHz, cellular bands

## API Endpoints

### Spectrum Sweep
```
POST /api/sdr/sweep
{
  "start_mhz": 2400,
  "end_mhz": 2500,
  "bin_width_hz": 100000
}
```

### Drone Detection
```
GET /api/sdr/drone-scan
POST /api/sdr/drone-scan/start
POST /api/sdr/drone-scan/stop
```

### Cellular Tower Scan
```
POST /api/sdr/cellular/scan
{
  "band": "GSM850"  // GSM850, GSM900, DCS1800, PCS1900
}
```

## Hardware Requirements

- HackRF One (1 MHz - 6 GHz, 20 MHz bandwidth)
- Appropriate antennas:
  - Telescopic whip (general purpose)
  - 2.4 GHz directional (drone hunting)
  - Discone (wideband monitoring)

## Usage Examples

### Quick Spectrum Check
```bash
# 2.4 GHz band sweep
sudo hackrf_sweep -f 2400:2500 -w 100000 -1

# 433 MHz ISM band
sudo hackrf_sweep -f 430:440 -w 50000 -1
```

### Continuous Drone Monitoring
```bash
# Start via API
curl -X POST http://localhost:8080/api/sdr/drone-scan/start

# Check results
curl http://localhost:8080/api/sdr/drone-detections
```

### Cell Tower Mapping
```bash
# Scan GSM850 band
sudo kal -s GSM850

# Via API
curl -X POST http://localhost:8080/api/sdr/cellular/scan \
  -H "Content-Type: application/json" \
  -d '{"band": "GSM850"}'
```

## Integration with Other Sensors

### RayHunter Correlation
- Compare cell towers seen by RayHunter with HackRF scans
- Validate IMSI catcher alerts with RF anomaly detection
- Build comprehensive cellular environment map

### WiFi/BLE Correlation
- Correlate drone detections with WiFi probe requests
- Match vehicle key fobs with BLE beacons
- Build device profiles across RF bands

### Location Fingerprinting
- Use spectrum signatures as location identifiers
- Detect location changes by RF environment shift
- Complement GPS with RF-based positioning

## Safety Notes

1. **Transmit disabled**: SIGINT-Deck only uses HackRF for receiving
2. **Legal compliance**: Passive monitoring only; no active transmission
3. **Power management**: HackRF draws ~500mA; use powered USB hub
