# Drone Detection System

SIGINT-Deck provides comprehensive drone detection using multiple complementary methods:
- **WiFi Detection** - OUI matching, SSID patterns, vendor-specific IEs, DJI DroneID
- **BLE Detection** - Open Drone ID (ASTM F3411-22a), manufacturer-specific advertisements
- **WiFi RemoteID** - NAN action frames, beacon-embedded RemoteID
- **RF Signal Detection** - Control links, video transmitters, telemetry
- **EMI Harmonic Detection** - Motor/ESC electronic noise signatures
- **Combined Detection** - Correlation of all methods for high-confidence identification

---

## Detection Methods Overview

| Method | Hardware | Detects | Confidence |
|--------|----------|---------|------------|
| WiFi OUI Match | wlan (monitor mode) | DJI, Parrot, Autel by MAC prefix | HIGH |
| WiFi SSID Pattern | wlan (monitor mode) | 30+ drone SSID patterns (DJI, Tello, ANAFI, etc.) | HIGH |
| WiFi NAN RemoteID | wlan (monitor mode) | ASTM F3411 compliant drones (DJI, Parrot) | HIGH |
| WiFi Beacon RemoteID | wlan (monitor mode) | Skydio, Autel, Parrot | HIGH |
| DJI DroneID (vendor IE) | wlan (monitor mode) | DJI serial, GPS, pilot location | HIGH |
| BLE Open Drone ID | Bluetooth (hci0) | Any ASTM F3411 drone (UUID 0xFFFA) | HIGH |
| RF 2.4 GHz | HackRF | Control links (DJI OcuSync, WiFi, ELRS) | MEDIUM |
| RF 5.8 GHz | HackRF | FPV video (analog/digital) | HIGH |
| RF 868/915 MHz | RTL-SDR or HackRF | Long-range links (Crossfire, ELRS) | HIGH |
| RF Military bands | RTL-SDR + HackRF | Military drone datalinks | MEDIUM |
| EMI Harmonics | RTL-SDR (direct sampling) | Motor/ESC PWM signatures | MEDIUM |

---

## Supported Manufacturers

### Consumer/Commercial Drones

| Manufacturer | WiFi OUI | SSID Patterns | BLE ODID | RF Signatures |
|-------------|----------|---------------|----------|---------------|
| **DJI** | 10 OUIs (60:60:1F, 34:D2:62, 48:1C:B9, E4:7A:2C, 58:B8:58, 04:A8:5A, 8C:58:23, 0C:9A:E6, 88:29:85, 4C:43:F6) | DJI-*, TELLO-*, Spark-*, RC400L, RC230, RC-N1/N2 | Yes (company ID 0x038F) | OcuSync 2/3/4, Lightbridge, O3 |
| **Parrot** | 4 OUIs (90:03:B7, A0:14:3D, 00:12:1C, 00:26:7E) | ANAFI-*, Bebop2-*, DISCO-*, Mambo-* | Yes | WiFi 802.11 |
| **Autel** | 1 OUI (60:C7:98) | Autel-*, EVO-*, SkyLink | Yes | SkyLink FHSS |
| **Skydio** | Chipset OUI | Skydio-* | Yes | WiFi 802.11ac/ax |
| **Yuneec** | Chipset OUI | YUNEEC-*, YuneecH520-* | Partial | Proprietary FHSS |
| **Holy Stone** | Chipset OUI | HolyStone-*, HS-* | No | WiFi 802.11n |
| **FIMI/Xiaomi** | Chipset OUI | FIMI_*, Xiaomi_* | No | Enhanced WiFi |
| **Hubsan** | Chipset OUI | Hubsan-*, ZINO-* | No | WiFi/custom |
| **Eachine** | Chipset OUI | EAchine-*, WiFi-UFO-* | No | WiFi/5.8 GHz analog |
| **SJRC** | Chipset OUI | SJRC* | No | WiFi |
| **JJRC** | Chipset OUI | JJRC* | No | WiFi |
| **MJX** | Chipset OUI | MJX* | No | WiFi |
| **Potensic** | Chipset OUI | Potensic* | No | WiFi |
| **Ruko** | Chipset OUI | Ruko* | No | WiFi |

### FPV Racing/Freestyle Drones

| System | Frequency | Bandwidth | Detection Method |
|--------|-----------|-----------|------------------|
| Analog VTX (5 bands, 40 ch) | 5645-5945 MHz | ~18 MHz FM | HackRF spectrum |
| DJI O3/O3+ Digital | 5660-5945 MHz | ~40 MHz OFDM | HackRF spectrum |
| HDZero | 5725-5875 MHz | ~20 MHz | HackRF spectrum |
| Walksnail Avatar | 5725-5875 MHz | ~40 MHz OFDM | HackRF spectrum |
| ExpressLRS 2.4G | 2400-2480 MHz | ~800 kHz LoRa | HackRF/RTL-SDR |
| ExpressLRS 868/915 | 868/902-928 MHz | ~500 kHz LoRa | RTL-SDR |
| TBS Crossfire | 868/915 MHz | ~500 kHz LoRa | RTL-SDR |
| FrSky ACCST/ACCESS | 2400-2480 MHz | ~2 MHz FHSS | HackRF |
| Flysky AFHDS | 2408-2475 MHz | ~1 MHz GFSK | HackRF |

### Military/Government Drones

| Drone | Country | Frequencies | Detection Hardware |
|-------|---------|-------------|-------------------|
| **Orlan-10** | Russia | 200-450 MHz, 867-872 MHz, 915-920 MHz, 1.08-1.3 GHz, 2.2-2.7 GHz | RTL-SDR + HackRF |
| **Lancet/ZALA** | Russia | 867-872 MHz, 902-928 MHz, 2.2-2.4 GHz | RTL-SDR + HackRF |
| **Orion (Kronshtadt)** | Russia | 890-920 MHz, 4.2-5.7 GHz C-band | RTL-SDR + HackRF |
| **Shahed-136/Geran-2** | Iran | GPS/GLONASS guided (minimal RF), ~900 MHz pre-launch | RTL-SDR (limited) |
| **Mohajer-6** | Iran | 1.2-1.6 GHz L-band | RTL-SDR |
| **CH-3A** | China | 325-390 MHz, 520-790 MHz, 2.3-2.65 GHz | RTL-SDR + HackRF |
| **CH-4B** | China | 800-900 MHz, 4.2-6.1 GHz | RTL-SDR + HackRF |
| **Bayraktar TB2/TB3** | Turkey | ~900 MHz UHF, 1.6-2.0 GHz L-band, ~5 GHz C-band | RTL-SDR + HackRF |
| **Predator/Reaper** | USA | ~5 GHz C-band LOS, Ku-band SATCOM | HackRF (C-band only) |
| **Switchblade** | USA | 902-928 MHz | RTL-SDR |
| **Anduril Ghost** | USA | 902-928 MHz mesh, 2.4 GHz mesh | RTL-SDR + HackRF |
| **ScanEagle** | USA | 2.2-2.4 GHz S-band | HackRF |
| **Hermes 450/900** | Israel | 2.2-2.4 GHz S-band, Ku-band SATCOM | HackRF |
| **Heron** | Israel | 1.2-1.4 GHz L-band, Ku-band SATCOM | RTL-SDR + HackRF |
| **Harop** | Israel | ~900 MHz UHF | RTL-SDR |

---

## WiFi/BLE Detection (Real-Time, Passive)

WiFi and BLE drone detection runs continuously as part of the normal WiFi monitor and BLE scan loops. No additional configuration is needed.

### WiFi OUI Detection
Every WiFi frame source MAC is checked against known drone manufacturer OUI prefixes. DJI alone has 10 registered OUI blocks.

### WiFi SSID Pattern Matching
SSIDs are checked against 30+ prefix patterns (`DJI-*`, `TELLO-*`, `ANAFI-*`, etc.) and 8 substring patterns (`RC400L`, `RC230`, `SkyLink`, etc.) for controllers with cellular modems that don't use drone-manufacturer OUIs.

### WiFi NAN RemoteID (ASTM F3411-22a)
Monitors 802.11 action frames (type 0, subtype 13) for:
- Category 0x04 (Public Action)
- OUI 0x50:6F:9A (Wi-Fi Alliance) with type 0x13 (NAN)
- Parses ODID payloads: Basic ID, Location/Vector, System, Operator ID

### DJI DroneID (Proprietary)
Scans beacon vendor-specific IEs (tag 0xDD) with DJI OUIs for embedded DroneID data containing:
- Drone serial number, GPS coordinates, altitude, speed, heading
- Pilot GPS coordinates, home point GPS

### BLE Open Drone ID
Monitors BLE advertisements for service UUID 0xFFFA with ODID payloads:
- **Basic ID** — UAS serial number or registration
- **Location/Vector** — Latitude, longitude, altitude, speed, heading
- **System** — Operator location, area count
- **Operator ID** — Pilot registration number
- **Message Pack** — Multiple messages in one advertisement (BLE 5)

Also checks manufacturer data for DJI company ID 0x038F.

### API

```bash
# Get all detected drones (WiFi + BLE + RF)
curl http://<device-ip>:8085/api/sdr/drone/signals
```

Response includes `wifi_ble_detections` array with manufacturer, MAC, SSID, RSSI, detection method, and threat level.

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
| `get_detected_drones` | **[READ-ONLY]** Get all detected drones (WiFi/BLE/RF combined) |
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
