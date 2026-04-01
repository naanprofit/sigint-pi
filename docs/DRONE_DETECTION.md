# Drone Detection System

SIGINT-Deck provides comprehensive drone detection using multiple complementary methods:
- **RF Signal Detection** - Control links, video transmitters, telemetry
- **EMI Harmonic Detection** - Motor/ESC electronic noise signatures
- **Combined Detection** - Correlation of both methods for high-confidence identification

---

## Detection Methods Overview

| Method | Frequency Range | Detects | Hardware Required |
|--------|-----------------|---------|-------------------|
| RF 2.4 GHz | 2400-2500 MHz | Control links (DJI, WiFi drones) | HackRF |
| RF 5.8 GHz | 5650-5950 MHz | FPV video transmitters | HackRF |
| RF 915 MHz | 868-928 MHz | Long-range links (Crossfire, ELRS) | RTL-SDR or HackRF |
| EMI | 5-500 kHz | Motor/ESC PWM harmonics | RTL-SDR (direct sampling) |

---

## RF Signal Detection

### 2.4 GHz Control Band

Most consumer drones use 2.4 GHz for control links:

| Protocol | Bandwidth | Characteristics |
|----------|-----------|-----------------|
| DJI OcuSync | 40 MHz | FHSS OFDM, distinctive pattern |
| DJI Lightbridge | 20 MHz | Legacy DJI protocol |
| WiFi (802.11) | 20/40 MHz | Standard WiFi, used by toy drones |
| ExpressLRS 2.4G | 1 MHz | LoRa FHSS, racing/long-range |
| FrSky | 2 MHz | FHSS, hobby RC |

### 5.8 GHz Video Band

FPV video transmitters operate in 5.8 GHz ISM band:

| System | Channels | Characteristics |
|--------|----------|-----------------|
| Analog FPV | R1-R8 (Raceband) | 5658-5917 MHz, FM video |
| DJI Digital | 6 channels | 5660-5865 MHz, OFDM |
| HDZero | 8 channels | Low-latency digital |
| Walksnail | 6 channels | High-bandwidth digital |

### 915 MHz Long-Range

Used for extended-range control:

| System | Region | Power |
|--------|--------|-------|
| TBS Crossfire | USA (915 MHz) | Up to 2W |
| TBS Crossfire | EU (868 MHz) | Up to 25mW |
| ExpressLRS 900 | USA/EU | 10mW-1W |

---

## EMI Harmonic Detection

### How It Works

Brushless drone motors are driven by Electronic Speed Controllers (ESCs) that use Pulse Width Modulation (PWM) switching at specific frequencies. This switching creates electromagnetic interference (EMI) with harmonics that can be detected:

```
Fundamental (f): 24 kHz (BLHeli_S ESC)
Harmonics: 24, 48, 72, 96, 120, 144 kHz (1x, 2x, 3x, 4x, 5x, 6x)
```

### ESC Signature Database

| ESC Type | Fundamental | Harmonics | Common Use |
|----------|-------------|-----------|------------|
| BLHeli_S 24kHz | 24 kHz | 24, 48, 72, 96, 120, 144 | Most hobby drones |
| BLHeli_S 48kHz | 48 kHz | 48, 96, 144, 192, 240 | DShot-enabled |
| BLHeli_32 | 48-96 kHz | Variable | High-performance racing |
| Industrial 8kHz | 8 kHz | 8, 16, 24, 32, 40, 48 | T-Motor, large drones |
| Industrial 16kHz | 16 kHz | 16, 32, 48, 64, 80 | Hobbywing XRotor |
| DJI Stock | 16 kHz | 16, 32, 48, 64 | Consumer DJI |

### Detection Requirements

- **Minimum 3 harmonics** required for confirmed detection
- **RTL-SDR with direct sampling mode** (bypasses tuner for VLF/LF reception)
- Effective range: 5-100+ meters depending on drone size and motor current

### Why EMI Detection Matters

1. **Detects autonomous drones** - Drones in GPS/waypoint mode may minimize RF transmission
2. **Confirms active motors** - Distinguishes airborne drone from RF interference
3. **Works through obstacles** - Low-frequency EMI penetrates better than GHz RF
4. **Correlates with RF** - Combined detection provides ~95% confidence

---

## Combined Detection

### Confidence Levels

| Detection | RF Signals | EMI Signals | Confidence | Assessment |
|-----------|------------|-------------|------------|------------|
| Confirmed | Yes | Yes | 95% | Drone with active motors transmitting |
| Probable (RF) | Yes | No | 70% | RF signal in drone bands (could be other device) |
| Probable (EMI) | No | Yes | 60% | Motor EMI detected (could be autonomous mode) |
| None | No | No | - | No drone detected |

### Distance Estimation

**RF-based (2.4/5.8 GHz):**
| Signal Strength | Estimated Distance |
|-----------------|-------------------|
| > -30 dBm | < 10 meters |
| -30 to -50 dBm | 10-50 meters |
| -50 to -70 dBm | 50-200 meters |
| < -70 dBm | 200-500+ meters |

**EMI-based (VLF/LF):**
| Signal Strength | Estimated Distance |
|-----------------|-------------------|
| > -20 dBm | < 5 meters |
| -20 to -35 dBm | 5-20 meters |
| -35 to -45 dBm | 20-50 meters |
| < -45 dBm | 50-100+ meters |

---

## API Endpoints

### RF-Only Scan
```bash
curl -X POST http://<device-ip>:8080/api/sdr/drone/scan
```

### EMI-Only Scan
```bash
curl -X POST http://<device-ip>:8080/api/sdr/drone/emi
```

### Full Combined Scan (Recommended)
```bash
curl -X POST http://<device-ip>:8080/api/sdr/drone/scan/full
```

### Response Example (Full Scan)
```json
{
  "success": true,
  "rf_scan": {
    "success": true,
    "signals": [
      {
        "band": "2.4GHz",
        "frequency_hz": 2437000000,
        "power_db": -45.2,
        "signal_type": "control",
        "threat_level": "medium"
      }
    ],
    "bands_scanned": ["2.4GHz", "5.8GHz"]
  },
  "emi_scan": {
    "success": true,
    "signals": [
      {
        "fundamental_khz": 48.0,
        "esc_type": "BLHeli_S 48kHz",
        "harmonics_detected": 5,
        "average_power_db": -32.1,
        "confidence": 0.83,
        "estimated_motor_count": 4,
        "estimated_distance": "< 20m"
      }
    ]
  },
  "combined_detections": [
    {
      "id": "drone_confirmed_1234567890",
      "detection_method": "rf_and_emi",
      "confidence": 0.95,
      "threat_level": "high",
      "description": "CONFIRMED: Drone detected via both RF transmission and motor EMI"
    }
  ],
  "detection_summary": {
    "rf_signals": 1,
    "emi_signals": 1,
    "confirmed_drones": 1
  }
}
```

---

## MCP Tools

For AI agent integration via MCP server:

| Tool | Description |
|------|-------------|
| `sdr_scan_drones` | RF-only scan (2.4/5.8 GHz) |
| `sdr_scan_drone_emi` | EMI-only scan (motor harmonics) |
| `sdr_scan_drones_full` | Combined RF + EMI scan |
| `sdr_start_drone_monitor` | Continuous monitoring |
| `sdr_stop_drone_monitor` | Stop monitoring |

---

## Hardware Requirements

### Minimum (EMI Detection)
- RTL-SDR with direct sampling support (RTL2832U-based)
- Frequency range: 500 Hz - 28.8 MHz (direct sampling mode)

### Recommended (Full Detection)
- **HackRF One** - 1 MHz to 6 GHz, TX/RX
- **RTL-SDR** - For EMI detection (direct sampling)
- Both devices for simultaneous RF + EMI scanning

### Antenna Recommendations

| Band | Antenna Type | Notes |
|------|--------------|-------|
| 2.4/5.8 GHz | Wideband discone or log-periodic | Directional for localization |
| VLF/LF (EMI) | Long wire or loop antenna | Built-in RTL-SDR antenna may work for close range |

---

## Drone Types and Signatures

### Consumer Drones (DJI, Autel, etc.)
- **RF**: OcuSync/Lightbridge on 2.4/5.8 GHz
- **EMI**: 16 kHz fundamental (stock ESCs)
- **Detection**: Usually easy due to continuous transmission

### FPV Racing Drones
- **RF**: Analog or digital video on 5.8 GHz, ExpressLRS/Crossfire control
- **EMI**: 24/48/96 kHz (BLHeli ESCs)
- **Detection**: Strong video signal, distinctive EMI pattern

### Autonomous/GPS Drones
- **RF**: Minimal (only telemetry bursts)
- **EMI**: Full motor signature
- **Detection**: EMI is primary detection method

### Large Industrial Drones
- **RF**: Often proprietary links
- **EMI**: 8 kHz fundamental (large motors, high current)
- **Detection**: Strong EMI due to high power motors

---

## Limitations

1. **EMI range is limited** - Effective to ~100m for typical quads
2. **Direct sampling mode** - Not all RTL-SDR dongles support it
3. **Environmental noise** - Other motors/electronics can create false positives
4. **Shielded drones** - Military/professional drones may have EMI shielding
5. **Ground clutter** - Electric vehicles, HVAC, etc. produce similar EMI

---

## Legal Notice

Drone detection is for **security awareness only**. Do not:
- Interfere with, jam, or disrupt drone communications
- Attempt to take control of drones
- Violate aviation regulations (FAA Part 107, etc.)

See [LEGAL.md](../LEGAL.md) for full terms.
