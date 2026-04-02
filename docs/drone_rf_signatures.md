# Comprehensive Drone RF Detection Signatures Database
## For SIGINT Drone Detection System (WiFi Monitor Mode / BLE / RTL-SDR / HackRF)

---

# TABLE OF CONTENTS
1. [WiFi OUI / MAC Address Prefixes](#1-wifi-oui--mac-address-prefixes)
2. [DJI Drones — Complete Protocol Details](#2-dji-drones)
3. [Autel Robotics](#3-autel-robotics)
4. [Skydio](#4-skydio)
5. [Parrot](#5-parrot)
6. [Yuneec](#6-yuneec)
7. [Budget Drones (Holy Stone, FIMI, Hubsan, Eachine)](#7-budget-drones)
8. [FPV Racing — Analog 5.8 GHz](#8-fpv-analog-58ghz)
9. [FPV Racing — Digital Systems](#9-fpv-digital-systems)
10. [RC Control Link Protocols](#10-rc-control-link-protocols)
11. [BLE Open Drone ID (ASTM F3411)](#11-ble-open-drone-id)
12. [WiFi NAN RemoteID](#12-wifi-nan-remoteid)
13. [DJI DroneID Protocol](#13-dji-droneid-protocol)
14. [Military/Government Drones](#14-militarygovernment-drones)
15. [Sub-GHz Detection (RTL-SDR)](#15-sub-ghz-detection)
16. [SDR Detection Parameters](#16-sdr-detection-parameters)
17. [WiFi Frame Detection Guide](#17-wifi-frame-detection-guide)
18. [Known SSID Patterns](#18-known-ssid-patterns)
19. [Detection Priority Matrix](#19-detection-priority-matrix)

---

# 1. WiFi OUI / MAC Address Prefixes

## DJI (SZ DJI TECHNOLOGY CO.,LTD) — 10 OUI Prefixes
| OUI Prefix | Hex Bytes | Registration Date |
|------------|-----------|-------------------|
| 60:60:1F   | 0x60601F  | 2013-03-11 |
| 34:D2:62   | 0x34D262  | 2019-08-13 |
| 48:1C:B9   | 0x481CB9  | 2022-05-07 |
| E4:7A:2C   | 0xE47A2C  | 2023-10-19 |
| 58:B8:58   | 0x58B858  | 2024-07-26 |
| 04:A8:5A   | 0x04A85A  | 2025-01-09 |
| 8C:58:23   | 0x8C5823  | 2025-05-27 |
| 0C:9A:E6   | 0x0C9AE6  | 2025-08-14 |
| 88:29:85   | 0x882985  | 2025-10-29 |
| 4C:43:F6   | 0x4C43F6  | 2025-12-01 |

## Parrot SA
| OUI Prefix | Hex Bytes | Notes |
|------------|-----------|-------|
| 90:03:B7   | 0x9003B7  | Parrot SA |
| A0:14:3D   | 0xA0143D  | Parrot SA |
| 00:12:1C   | 0x00121C  | Parrot SA (older) |
| 00:26:7E   | 0x00267E  | Parrot SA |

## Autel Robotics
| OUI Prefix | Hex Bytes | Notes |
|------------|-----------|-------|
| 60:C7:98   | 0x60C798  | Autel Robotics (look for AUTEL INTELLIGENT TECHNOLOGY CO LTD) |

## Skydio
| OUI Prefix | Hex Bytes | Notes |
|------------|-----------|-------|
| Varies     | Check IEEE | Skydio Inc — check IEEE OUI registry; uses standard WiFi chipsets |

## Yuneec
| OUI Prefix | Hex Bytes | Notes |
|------------|-----------|-------|
| Varies     | Check IEEE | Yuneec International; may use Qualcomm/Realtek chipsets |

## Ryze Tech (DJI Tello)
| OUI Prefix | Hex Bytes | Notes |
|------------|-----------|-------|
| 60:60:1F   | 0x60601F  | Uses DJI OUI (Tello series) |

## Other Drone-Associated Chipset OUIs (Common in budget drones)
| OUI Prefix | Vendor | Notes |
|------------|--------|-------|
| Various Espressif (ESP32) | 30:AE:A4, 24:0A:C4, etc. | Common in DIY/budget drones |
| Various Realtek | 48:02:2A, etc. | Common WiFi chipset in consumer drones |

### Detection Code Pattern (Python pseudo-code):
```python
DJI_OUIS = [
    b'\x60\x60\x1f', b'\x34\xd2\x62', b'\x48\x1c\xb9',
    b'\xe4\x7a\x2c', b'\x58\xb8\x58', b'\x04\xa8\x5a',
    b'\x8c\x58\x23', b'\x0c\x9a\xe6', b'\x88\x29\x85',
    b'\x4c\x43\xf6'
]
PARROT_OUIS = [
    b'\x90\x03\xb7', b'\xa0\x14\x3d', b'\x00\x12\x1c',
    b'\x00\x26\x7e'
]
AUTEL_OUIS = [b'\x60\xc7\x98']
```

---

# 2. DJI Drones

## 2.1 Communication Protocols by Generation

### OcuSync 2.0 (Mavic Air 2, Mini 2, etc.)
- **Frequencies**: 2.4000–2.4835 GHz AND 5.7250–5.8500 GHz (auto-switching)
- **Bandwidth**: 10 MHz or 20 MHz selectable
- **Modulation**: OFDM with FHSS (Frequency Hopping Spread Spectrum)
- **Max Range**: ~10 km (FCC), ~6 km (CE)
- **Video Downlink**: Up to 12 Mbps
- **Hop Rate**: Dynamic, hundreds of hops/sec
- **Channels**: 2.4 GHz: channels across 2400-2483.5 MHz; 5.8 GHz: channels across 5725-5850 MHz

### OcuSync 3.0 (Mini 3 Pro, Mavic 3, Air 2S)
- **Frequencies**: 2.4000–2.4835 GHz AND 5.7250–5.8500 GHz (auto-switching)
- **Bandwidth**: 10 MHz, 20 MHz, or 40 MHz
- **Modulation**: OFDM with FHSS
- **Max Range**: ~15 km (FCC)
- **Video Downlink**: Up to 15.5 Mbps (Mavic 3)
- **4-antenna MIMO**: Yes (2T4R on some models)
- **Latency**: ~120 ms

### OcuSync 4.0 / DJI O4 (Mavic 3 Pro, Air 3, Mini 4 Pro)
- **Frequencies**: 2.4000–2.4835 GHz AND 5.7250–5.8500 GHz
- **Bandwidth**: Up to 40 MHz
- **Modulation**: OFDM with FHSS
- **Max Range**: ~20 km (FCC)
- **Video Downlink**: Up to 1080p/60fps live
- **Triple-antenna system**: Some models

### DJI O3 / O3+ (DJI FPV, Avata, Avata 2)
- **Frequencies**: 5.7250–5.8500 GHz (primary), 2.4000–2.4835 GHz (fallback)
- **Bandwidth**: ~40 MHz per channel (wide-band OFDM)
- **Modulation**: OFDM
- **Video**: Up to 1080p/100fps, 810p/60fps with <28ms latency
- **Channels**: 8 frequency channels in 5.8 GHz band

### WiFi (Tello, Mini SE, older Phantoms)
- **Frequencies**: 2.4 GHz standard WiFi (802.11n)
- **Protocol**: Standard 802.11 with proprietary app layer
- **Bandwidth**: 20 MHz or 40 MHz
- **SSID Pattern**: "TELLO-*", "DJI-*"

### Lightbridge 2 (Inspire 2, Phantom 4 Pro, Matrice 200 series)
- **Frequencies**: 2.4000–2.4835 GHz AND 5.7250–5.8500 GHz
- **Bandwidth**: 10 MHz or 20 MHz
- **Modulation**: OFDM with FHSS
- **Max Range**: ~7 km

## 2.2 DJI RemoteID Implementation
- **Method**: WiFi NAN (Neighbor Awareness Networking) — primary method
- **Also**: BLE 4 Legacy Advertisement (on newer firmware)
- **Standard**: ASTM F3411-22a / ASD-STAN EN 4709-002
- **WiFi NAN Action Frame Category**: 0x04 (Public Action)
- **WiFi NAN OUI**: 0x506F9A (Wi-Fi Alliance)
- **WiFi NAN OUI Type**: 0x13 (NAN)
- **RemoteID Service**: Published as NAN service with ODID message payloads
- **BLE Service UUID**: 0xFFFA (Open Drone ID)
- **BLE Company ID**: 0x038F (for DJI-specific BLE advertisements)

## 2.3 DJI DroneID (Proprietary — NOT RemoteID)
- **Transport**: Custom OFDM bursts embedded in the 2.4 GHz or 5.8 GHz control channel
- **Bandwidth**: ~10 MHz OFDM signal
- **Center Frequencies**: Variable within 2.4 GHz and 5.8 GHz bands
- **Modulation**: OFDM (similar to WiFi but NOT 802.11)
- **Data Content**: Drone serial number, GPS coordinates (drone + pilot), altitude, speed, heading, home point
- **Frame Structure**: Custom PHY layer, similar to 802.11 but with unique preamble
- **Detection**: Requires SDR (HackRF/USRP) tuned to DJI control frequencies, or passive WiFi sniffing for the WiFi-embedded variant
- **Key Tool**: `dji_droneid` (RUB-SysSec/DroneSecurity project) — decodes from I/Q samples
- **SDR Detection Center Freqs**: Scan 2.4 GHz and 5.8 GHz bands for ~10 MHz wide OFDM bursts

---

# 3. Autel Robotics

## SkyLink Protocol (EVO II, EVO Max, EVO Lite, Dragonfish)
- **Frequencies**: 2.4000–2.4835 GHz AND 5.7250–5.8500 GHz (dual-band, auto-switching)
- **Bandwidth**: ~20-40 MHz
- **Modulation**: OFDM with FHSS
- **Max Range**: ~15 km (SkyLink 2.0)
- **Video**: Up to 2.7K/30fps live
- **RemoteID**: WiFi Beacon and/or BLE (ASTM F3411 compliant)
- **SSID Pattern**: "Autel-*" or "EVO-*" or "AUTEL_*"
- **OUI**: Look for "Autel Intelligent Technology" in IEEE registry

## Autel RemoteID
- **Method**: WiFi Beacon + BLE 4/5
- **Compliance**: ASTM F3411-22a (mixed compliance — some models have issues per SkySafe testing)
- **BLE Service UUID**: 0xFFFA

---

# 4. Skydio

## Skydio 2+, X2, X10
- **Frequencies**: 2.4 GHz and 5 GHz (standard WiFi, 802.11ac/ax)
- **Protocol**: WiFi-based control via smartphone/controller
- **Bandwidth**: Standard WiFi channels (20/40/80 MHz)
- **RemoteID**: WiFi Beacon (ASTM F3411)
- **SSID Pattern**: "Skydio-*" or "SKYDIO-*"
- **Autonomy**: Heavy on-board AI processing, less reliant on continuous RC link
- **OUI**: Uses standard WiFi chipset OUIs (Qualcomm, etc.)

---

# 5. Parrot

## Parrot ANAFI / ANAFI USA / ANAFI AI
- **Frequencies**: 2.4 GHz and 5 GHz (802.11a/b/g/n/ac)
- **Protocol**: Standard WiFi (controlled via FreeFlight app)
- **Bandwidth**: 20/40 MHz standard WiFi
- **ANAFI AI**: Also supports 4G LTE cellular control
- **SSID Pattern**: "ANAFI-*", "Anafi-*", "DISCO-*" (Disco), "Bebop2-*" (Bebop)
- **RemoteID**: WiFi Beacon (ASTM F3411 — Parrot was one of the first compliant manufacturers)
- **OUI**: 90:03:B7 (Parrot SA) — most common
- **BLE**: Present for pairing, also used for RemoteID on some models
- **BLE Service UUID**: 0xFFFA (Open Drone ID)

---

# 6. Yuneec

## Typhoon H Plus, H520E, Mantis
- **Frequencies**: 2.4 GHz (primary), some models dual-band
- **Protocol**: Proprietary digital link
- **Bandwidth**: ~20 MHz
- **Modulation**: FHSS
- **RemoteID**: Limited/variable compliance
- **SSID Pattern**: "YuneecH520-*", "YUNEEC-*"
- **OUI**: Uses various chipset OUIs — look for Yuneec in vendor strings

---

# 7. Budget Drones

## Holy Stone
- **Frequencies**: 2.4 GHz (standard WiFi 802.11n)
- **Protocol**: WiFi FPV (direct AP mode)
- **SSID Pattern**: "HolyStone-*", "HS-*", "FPV_*"
- **RemoteID**: Generally NOT compliant (no standard RID)
- **OUI**: Various (Espressif, Realtek chipsets)

## FIMI / Xiaomi
- **Frequencies**: 2.4 GHz and 5.8 GHz
- **Protocol**: Enhanced WiFi / proprietary FHSS
- **SSID Pattern**: "FIMI_*", "Xiaomi_*"
- **FIMI X8 SE**: Uses TDMA-based link, 2.4/5.8 GHz
- **OUI**: Look for Xiaomi Communications Co Ltd OUIs

## Hubsan
- **Frequencies**: 2.4 GHz
- **Protocol**: Custom 2.4 GHz or WiFi depending on model
- **SSID Pattern**: "Hubsan-*", "ZINO-*"
- **RemoteID**: Not compliant

## Eachine
- **Frequencies**: 2.4 GHz (control), 5.8 GHz (analog FPV)
- **Protocol**: Standard toy-grade 2.4 GHz protocols, some WiFi FPV
- **SSID Pattern**: "EAchine-*", "WiFi-UFO-*"

---

# 8. FPV Analog 5.8GHz

## Complete 5.8 GHz Frequency Table (All Bands)

### Band A (Boscam A / Team BlackSheep)
| Channel | Frequency (MHz) |
|---------|-----------------|
| A1 | 5865 |
| A2 | 5845 |
| A3 | 5825 |
| A4 | 5805 |
| A5 | 5785 |
| A6 | 5765 |
| A7 | 5745 |
| A8 | 5725 |

### Band B (Boscam B / FlySky)
| Channel | Frequency (MHz) |
|---------|-----------------|
| B1 | 5733 |
| B2 | 5752 |
| B3 | 5771 |
| B4 | 5790 |
| B5 | 5809 |
| B6 | 5828 |
| B7 | 5847 |
| B8 | 5866 |

### Band E (Boscam E / DJI / Lumenier)
| Channel | Frequency (MHz) |
|---------|-----------------|
| E1 | 5705 |
| E2 | 5685 |
| E3 | 5665 |
| E4 | 5645 |
| E5 | 5885 |
| E6 | 5905 |
| E7 | 5925 |
| E8 | 5945 |

### Band F (ImmersionRC / FatShark)
| Channel | Frequency (MHz) |
|---------|-----------------|
| F1 | 5740 |
| F2 | 5760 |
| F3 | 5780 |
| F4 | 5800 |
| F5 | 5820 |
| F6 | 5840 |
| F7 | 5860 |
| F8 | 5880 |

### Band R (Raceband) — Most commonly used in racing
| Channel | Frequency (MHz) |
|---------|-----------------|
| R1 | 5658 |
| R2 | 5695 |
| R3 | 5732 |
| R4 | 5769 |
| R5 | 5806 |
| R6 | 5843 |
| R7 | 5880 |
| R8 | 5917 |

### Detection Parameters (Analog VTX)
- **Signal Type**: FM analog video (NTSC/PAL)
- **Bandwidth**: ~15-20 MHz per channel
- **Channel Spacing**: ~19-37 MHz (varies by band)
- **Power Levels**: 25 mW, 100 mW, 200 mW, 400 mW, 600 mW, 800 mW (typical)
- **Modulation**: FM (Frequency Modulation) for video carrier
- **Detection Method**: Power spectral density scan in 5645-5945 MHz range; look for ~18 MHz wide FM carriers
- **SDR Required**: HackRF (covers 5.8 GHz), NOT RTL-SDR (max 1766 MHz)

---

# 9. FPV Digital Systems

## 9.1 DJI Digital FPV (O3, Vista, Air Unit)
- **Frequency Range**: 5725–5850 MHz
- **Bandwidth**: ~40 MHz per channel (wide OFDM)
- **Channels**: 8 selectable (centered within 5.8 GHz band)
- **Channel Center Frequencies (approximate)**:
  - Ch 1: ~5660 MHz (outside ISM, used by DJI)
  - Ch 2: ~5700 MHz
  - Ch 3: ~5745 MHz
  - Ch 4: ~5785 MHz
  - Ch 5: ~5825 MHz
  - Ch 6: ~5865 MHz
  - Ch 7: ~5905 MHz (outside ISM in some regions)
  - Ch 8: ~5945 MHz (outside ISM in some regions)
- **Modulation**: OFDM (similar to 802.11ac)
- **Power**: Up to 1200 mW (25.5 dBm) EIRP
- **Detection**: Look for ~40 MHz wide OFDM signals in 5.7-5.9 GHz; distinct from standard WiFi due to wider bandwidth and non-standard channel centers
- **SDR Required**: HackRF

## 9.2 HDZero (formerly Shark Byte)
- **Frequency Range**: 5725–5875 MHz
- **Bandwidth**: ~20 MHz per channel (narrower than DJI)
- **Channels**: Uses standard 5.8 GHz FPV analog frequency table (same as analog bands)
- **Up to 8 simultaneous pilots** in one area
- **Modulation**: Digital, uncompressed video stream
- **Latency**: <1 ms visible latency
- **Video**: 720p60 (native), some modes 540p90
- **Detection**: ~20 MHz digital signals on standard 5.8 GHz FPV frequencies
- **SDR Required**: HackRF

## 9.3 Walksnail Avatar (Caddx)
- **Frequency Range**: 5725–5875 MHz
- **Bandwidth**: ~40 MHz per channel
- **Channels**: Up to 8 (similar channel plan to DJI)
- **Modulation**: OFDM digital
- **Video**: Up to 1080p/60fps
- **Latency**: ~22 ms
- **Power**: Up to 1200 mW
- **Detection**: Very similar signature to DJI O3 — ~40 MHz OFDM in 5.8 GHz band
- **SDR Required**: HackRF

---

# 10. RC Control Link Protocols

## 10.1 ExpressLRS (ELRS)
### 2.4 GHz Variant
- **Frequency Range**: 2400–2480 MHz ISM band
- **Modulation**: LoRa (Semtech SX1280/SX1281)
- **Bandwidth**: 812 kHz (LoRa BW800) at 500 Hz rate; variable based on packet rate
- **Packet Rates**: 25 Hz, 50 Hz, 100 Hz, 150 Hz, 200 Hz, 250 Hz, 333 Hz, 500 Hz, 1000 Hz
- **Hopping**: FHSS over up to 80 channels (2.4 GHz); binding phrase determines hop sequence
- **Hop Rate**: Matches packet rate
- **Signal Characteristics**: Very narrow-band LoRa chirps (~800 kHz), distinctive spread-spectrum chirp
- **Power**: 10 mW to 1000 mW (configurable)
- **Binding**: Binding phrase (hashed to generate hop sequence + unique ID)

### 868/915 MHz Variant
- **Frequency Range**: 868.0–870.0 MHz (EU) / 902–928 MHz (US/FCC)
- **Modulation**: LoRa (Semtech SX1276/SX1262)
- **Bandwidth**: ~500 kHz LoRa
- **Packet Rates**: 25 Hz, 50 Hz, 100 Hz, 150 Hz, 200 Hz
- **Hopping**: FHSS across the ISM band
- **Power**: Up to 100 mW (EU) / 1000 mW (US)
- **RTL-SDR Detection**: YES — within RTL-SDR range for 868/915 MHz
- **Detection Signature**: Look for periodic LoRa chirps at ~868 or ~915 MHz center, hopping across the band

## 10.2 TBS Crossfire
- **Frequency Range**: 868 MHz (EU) / 915 MHz (US/Rest of World)
- **Modulation**: LoRa (Semtech SX1272/SX1276)
- **Bandwidth**: ~500 kHz LoRa
- **Packet Rates**: 4 Hz (longest range) to 150 Hz
- **Hopping**: FHSS across ~30 channels in the ISM band
- **Power**: Up to 2000 mW (2W)
- **Telemetry**: Bi-directional (CRSF protocol over serial)
- **RTL-SDR Detection**: YES — 868/915 MHz range
- **Detection Signature**: Similar LoRa chirps as ELRS but different timing/sequence

## 10.3 FrSky ACCST / ACCESS
### ACCST (Advanced Continuous Channel Shifting Technology)
- **Frequency Range**: 2400–2480 MHz
- **Modulation**: FHSS with 47 hop channels
- **Bandwidth**: ~2 MHz per hop
- **Hop Rate**: ~22 ms hop interval
- **Protocols**: D8 (8 channels), D16 (16 channels)

### ACCESS (Advanced Communication Control Elevated Spread Spectrum)
- **Frequency Range**: 2400–2480 MHz
- **Modulation**: Enhanced FHSS with adaptive frequency agility
- **Bandwidth**: ~2 MHz per hop
- **Channels**: 24 radio channels
- **Hop Rate**: Higher than ACCST
- **Encryption**: AES-128
- **Detection**: Standard 2.4 GHz ISM band narrow FHSS signals, ~2 MHz bursts hopping rapidly

## 10.4 Flysky AFHDS 2A / AFHDS 3
- **Frequency Range**: 2408–2475 MHz
- **Modulation**: GFSK (Gaussian Frequency-Shift Keying)
- **Bandwidth**: ~1 MHz per channel
- **Channels**: 16 hopping channels
- **Hop Rate**: ~3.8 ms hop interval
- **Power**: <20 dBm
- **Detection**: Narrow GFSK bursts in 2.4 GHz band, rapid hopping

---

# 11. BLE Open Drone ID (ASTM F3411-22a)

## Core Specification
- **BLE Service UUID**: 0xFFFA (ASTM Open Drone ID)
- **Bluetooth Version**: BLE 4.x (Legacy Advertising) and BLE 5.x (Extended Advertising)
- **Advertisement Type**: BLE 4 Legacy = AD Type 0x16 (Service Data - 16-bit UUID) in standard advertising PDUs
- **BLE 5 Extended**: Uses AUX_ADV_IND with longer payloads
- **Max Payload**: BLE 4 Legacy = 27 bytes per AD struct; BLE 5 Extended = up to 255 bytes

## Message Types (ODID Message Type field, 4 bits)
| Type | Value | Description |
|------|-------|-------------|
| Basic ID | 0x0 | UAS ID (serial number, CAA registration, etc.) |
| Location/Vector | 0x1 | Lat, Lon, Alt, Speed, Heading, Vertical Speed, Timestamp |
| Authentication | 0x2 | Auth data (up to 255 bytes across multiple pages) |
| Self-ID | 0x3 | Operator-defined text description |
| System | 0x4 | Operator location, area count, area radius, area ceiling/floor |
| Operator ID | 0x5 | Operator/Pilot ID (e.g., FAA Trust certificate number) |
| Message Pack | 0xF | Multiple messages packed in one advertisement |

## BLE 4 Legacy Advertisement Data Format
```
AD Length (1 byte) | AD Type 0x16 (1 byte) | UUID 0xFFFA (2 bytes LE: 0xFA 0xFF) |
Counter (1 byte) | Message Type + Protocol Version (1 byte) | Message Data (variable)
```

### Byte-level breakdown:
- Bytes 0-1: AD Length, AD Type (0x16)
- Bytes 2-3: Service UUID 0xFFFA (little-endian: 0xFA, 0xFF)
- Byte 4: Application Code / Counter (0x0D for ODID)
- Byte 5: (Message Type << 4) | Protocol Version
  - Protocol Version: 0x02 for F3411-22a
- Bytes 6+: Message-type-specific payload (up to 19 bytes for BLE4)

## BLE 5 Extended Advertisement Format
- Uses longer payloads for Message Pack type (0xF)
- Can contain up to 9 messages in a single extended advertisement
- AD Type: 0x16 with UUID 0xFFFA, same as Legacy but more data

## Which Drones Use BLE vs WiFi NAN?
| Manufacturer | BLE RemoteID | WiFi NAN RemoteID | WiFi Beacon RemoteID |
|-------------|-------------|-------------------|---------------------|
| DJI | Yes (BLE 4/5) | Yes (primary) | No |
| Autel | Yes | Partial | Yes |
| Parrot | Yes (BLE 5) | Yes | Yes |
| Skydio | Yes | No | Yes (primary) |
| Yuneec | Partial | No | Partial |
| Holy Stone | No | No | No |

## Detection Code Pattern:
```python
ODID_BLE_UUID = 0xFFFA  # Little-endian bytes: 0xFA, 0xFF
ODID_AD_TYPE = 0x16     # Service Data - 16-bit UUID
ODID_APP_CODE = 0x0D    # Open Drone ID Application Code

def parse_odid_ble(ad_data):
    if ad_data[0] == 0x16 and ad_data[1:3] == b'\xfa\xff':
        msg_type = (ad_data[4] >> 4) & 0x0F
        proto_ver = ad_data[4] & 0x0F
        return msg_type, proto_ver, ad_data[5:]
```

---

# 12. WiFi NAN RemoteID

## WiFi Neighbor Awareness Networking (NAN) for ODID
- **Frame Type**: 802.11 Action Frame (Public Action)
- **Category**: 0x04 (Public Action)
- **Action Code**: 0x09 (Vendor Specific)
- **OUI**: 0x50 0x6F 0x9A (Wi-Fi Alliance)
- **OUI Type**: 0x13 (NAN)
- **NAN Service Discovery**: The drone publishes a NAN service carrying ODID payloads
- **Service Name**: "org.opendroneid.remoteid" (hashed for NAN)
- **Channel**: NAN discovery operates on channel 6 (2437 MHz) for 2.4 GHz, channel 44/149 for 5 GHz

## WiFi NAN Action Frame Structure
```
Frame Control (2B) | Duration (2B) | DA=FF:FF:FF:FF:FF:FF (6B) | SA (6B) | BSSID (6B) |
Seq Ctrl (2B) | Category=0x04 (1B) | Action=0x09 (1B) |
OUI=50:6F:9A (3B) | OUI Type=0x13 (1B) |
NAN Attributes (variable) containing ODID Service Discovery Frame
```

## Key Hex Bytes to Match:
```python
WIFI_NAN_CATEGORY = 0x04
WIFI_NAN_ACTION = 0x09
WIFI_ALLIANCE_OUI = b'\x50\x6f\x9a'
NAN_OUI_TYPE = 0x13
```

## What to Monitor:
1. **802.11 Action Frames** (Type 0, Subtype 13) with Category 0x04
2. **Vendor Specific Public Action** with OUI 0x506F9A, Type 0x13
3. Parse NAN Service Discovery Frame for ODID payloads
4. NAN Cluster IDs / NAN beacons on channel 6 (2.4 GHz)

---

# 13. DJI DroneID Protocol (Proprietary)

## Overview
DJI DroneID is a **proprietary protocol** (NOT standard RemoteID) that DJI embeds in their radio transmissions. It transmits drone and operator location unencrypted.

## Physical Layer
- **Modulation**: OFDM (9 MHz bandwidth, ~600 subcarriers)
- **Center Frequencies**: Within the 2.4 GHz or 5.8 GHz control bands
- **Burst Duration**: ~1 ms per DroneID burst
- **Subcarrier Spacing**: ~15 kHz (similar to LTE)
- **Cyclic Prefix**: Normal CP
- **OFDM Symbols**: Multiple symbols per burst
- **Encoding**: QPSK on data subcarriers

## WiFi Frame Variant
- DroneID can also be embedded in **WiFi Beacon frames** as **Vendor Specific Information Elements (IEs)**
- **IE Tag**: 0xDD (Vendor Specific)
- **OUI in IE**: DJI OUI bytes (varies by model; commonly starts with DJI OUI prefix)
- **Data**: Contains serialized drone telemetry: serial number, GPS, altitude, speed, operator GPS

## Data Fields in DroneID Message
| Field | Size | Description |
|-------|------|-------------|
| Version | 1 byte | Protocol version |
| Sequence Number | 2 bytes | Frame sequence |
| State Info | 2 bytes | Drone state flags |
| Serial Number | 16 bytes | DJI serial number (ASCII) |
| Drone Longitude | 4 bytes | int32, degrees * 1e7 |
| Drone Latitude | 4 bytes | int32, degrees * 1e7 |
| Drone Altitude | 2 bytes | int16, meters relative to takeoff |
| Height | 2 bytes | int16, height AGL |
| Velocity North | 2 bytes | int16, cm/s |
| Velocity East | 2 bytes | int16, cm/s |
| Velocity Up | 2 bytes | int16, cm/s |
| Yaw | 2 bytes | uint16, degrees * 100 |
| Pilot Longitude | 4 bytes | int32, degrees * 1e7 |
| Pilot Latitude | 4 bytes | int32, degrees * 1e7 |
| Home Longitude | 4 bytes | int32, degrees * 1e7 |
| Home Latitude | 4 bytes | int32, degrees * 1e7 |
| Product Type | 1 byte | DJI model identifier |
| UUID | 20 bytes | Unique identifier |

## SDR-Based Detection (HackRF)
1. Tune HackRF to 2.4 GHz band, ~20 MHz bandwidth
2. Look for OFDM bursts with ~9 MHz bandwidth
3. Use correlation with known DJI DroneID preamble
4. Demodulate OFDM symbols, extract data
5. Tools: `dji_droneid` from RUB-SysSec, `samples2djidroneid` by anarkiwi

## WiFi-Based Detection (Monitor Mode)
1. Capture all 802.11 Beacon frames
2. Look for Vendor Specific IEs (tag 0xDD) with DJI OUI
3. Parse the IE payload for DroneID data structure
4. Also check for NAN frames with ODID payloads

---

# 14. Military/Government Drones

## 14.1 Iranian Drones

### Shahed-136 / Geran-2 (Loitering Munition)
- **Navigation**: GPS (L1: 1575.42 MHz) + GLONASS (L1: 1602 MHz)
- **GPS Antenna**: 16-element CRPA (Controlled Reception Pattern Antenna) for anti-jamming
- **RTK**: Uses cellular modem-based RTK corrections (pre-programmed waypoints)
- **INS**: Inertial Navigation System backup
- **Command Link**: NONE during terminal flight (fully autonomous GPS-guided)
- **Pre-launch Programming**: May use ground station at various frequencies
- **Engine Noise**: Distinctive 2-stroke engine RF interference pattern (broadband EMI)
- **Detectable Signatures**: GPS/GLONASS receiver emissions (very weak), no active command link during flight
- **RF Detection Difficulty**: Very low RF signature — primarily detectable via radar/acoustic/IR

### Mohajer-6
- **Datalink**: C-band (4-8 GHz) for LOS operations
- **Video Downlink**: Likely C-band or Ku-band
- **Command Link**: Encrypted digital link
- **GPS**: L1 (1575.42 MHz)
- **Detection**: C-band scanning with HackRF (up to 6 GHz)

### Ababil Series
- **Navigation**: GPS-guided
- **Control**: Pre-programmed autonomous + optional ground command
- **Frequencies**: Likely VHF/UHF for basic command links (148-174 MHz, 400-470 MHz)

## 14.2 Russian Drones

### Orlan-10
- **Primary Data Link**: ~1.5 GHz (L-band) LOS
- **Command Frequencies**: Reported at 868 MHz and/or around 900 MHz for some variants
- **Video Downlink**: 1.0-1.5 GHz range
- **Frequency Band Variants** (per Armada International / open-source reporting):
  - 860-870 MHz (command/telemetry)
  - 900-930 MHz (control link)
  - 1.4-1.5 GHz (video/data downlink)
- **Modulation**: OFDM / FHSS variants
- **GPS**: L1 (1575.42 MHz) + GLONASS (1602 MHz)
- **EW Variant**: Leer-3 system uses Orlan-10 as jammer platform
- **RTL-SDR Detection**: YES for 860-930 MHz and potentially 1.4-1.5 GHz

### Lancet (ZALA)
- **Control Link**: ~900 MHz (estimated) for LOS
- **Video**: Digital link, likely L-band
- **Autonomy**: Semi-autonomous terminal guidance (optical/RF seeker)
- **Frequencies**: Probably 860-930 MHz for C2

### ZALA Series (421-16E, etc.)
- **Datalink**: ~900 MHz band / L-band
- **Range**: 15-50 km depending on model

### Kronshtadt Orion
- **Satellite Link**: Likely Ku-band (12-18 GHz) for BLOS
- **LOS Link**: C-band (4-8 GHz)
- **Partially detectable**: HackRF up to 6 GHz covers partial C-band

### Forpost (Israeli IAI Searcher II license)
- **Datalink**: C-band (5.0-5.25 GHz range reported)
- **LOS Range**: ~250 km
- **HackRF Detection**: Within range

## 14.3 Chinese Military

### Wing Loong (I/II)
- **Satellite Link**: Ku-band (12-18 GHz) — beyond HackRF range
- **LOS Link**: C-band (~5 GHz)
- **HackRF**: Can detect LOS C-band link

### CASC CH-4 / CH-5
- **Satellite Link**: Ku-band (beyond HackRF)
- **LOS Link**: C-band (4-6 GHz)
- **Similar to US MQ-1/MQ-9 architecture**

### DJI Matrice 300/350 RTK (Dual-Use)
- **Same as consumer DJI protocols** (OcuSync 3.0/4.0)
- **Additional**: RTK correction link (various, often cellular)
- **Detection**: Same as consumer DJI

## 14.4 Turkish Drones

### Bayraktar TB2 / TB3
- **Primary Datalink**: C-band (~5.0-5.4 GHz) LOS, 150 km range
- **Backup**: Likely UHF band for degraded operations
- **Video Downlink**: Digital encrypted, C-band
- **GPS**: L1 + L2 (dual frequency)
- **Satellite Link**: Not standard on TB2 (planned for TB3/Akıncı)
- **HackRF Detection**: C-band within range (up to 6 GHz)
- **Detection Signature**: ~5 GHz band, encrypted OFDM signal, moderate bandwidth

### Kargu / STM Alpagu (Loitering Munition)
- **Control**: LOS command link, estimated UHF (400-500 MHz) or L-band
- **Swarm Communication**: Mesh radio, possibly in 2.4 GHz or proprietary band
- **Autonomous**: Can operate independently of C2 link

## 14.5 US/NATO Small UAS

### Common C2 Frequency Bands (Group 1-3 UAS)
| Band | Frequency Range | Common Use |
|------|----------------|------------|
| VHF | 30-300 MHz | Legacy RC control, some tactical systems |
| UHF | 225-400 MHz | Military UAS C2 (MUOS/SATCOM backup) |
| L-band | 960-1215 MHz / 1350-1400 MHz | ATC, some UAS C2 |
| S-band | 2.2-2.5 GHz | US military UAS (CDL - Common Data Link) |
| C-band | 4.4-5.0 GHz | NATO UAS standardized band, STANAG 4586 |
| C-band (upper) | 5.03-5.091 GHz | CNPC (Control and Non-Payload Communications) |
| Ku-band | 14.0-14.5 GHz | SATCOM for BLOS UAS |

### RQ-20 Puma
- **Control**: L-band or S-band digital link
- **Video**: Digital encrypted downlink
- **Range**: ~20 km LOS
- **GPS**: Military P(Y) code + L1

### Switchblade 300/600
- **Control**: Encrypted digital link, likely L-band or low-UHF (400-500 MHz)
- **Terminal Guidance**: EO/IR autonomous (link not required)
- **RF Signature**: Brief during launch/loiter, minimal in terminal

### Coyote (RAYTHEON)
- **Control**: L-band C2 link
- **GPS**: Assisted targeting
- **Swarm Capable**: Mesh communication, estimated L-band

## 14.6 Israeli Drones

### IAI Harop (Loitering Munition)
- **Datalink**: C-band (~5 GHz) LOS
- **Satellite**: Potential Ku-band for extended range
- **Terminal**: Anti-radiation seeker (homes on radar emissions)
- **HackRF**: C-band detectable

### Hermes 450/900
- **Primary Link**: CDL-type, C-band (4-5 GHz)
- **Satellite**: Ku-band for BLOS
- **Range**: Up to 200 km (LOS)

---

# 15. Sub-GHz Detection (RTL-SDR Range: 24-1766 MHz)

## Detectable Signals Summary

### 433 MHz ISM Band (LPD433)
- **Range**: 433.050–434.775 MHz
- **Used By**: Some toy drones, ground station telemetry links, European RC
- **Modulation**: OOK, FSK, LoRa
- **RTL-433 Protocol IDs**: Various RC protocols detected by rtl_433

### 868 MHz (EU SRD Band)
- **Range**: 863–870 MHz
- **Used By**: ExpressLRS 868 MHz, TBS Crossfire 868 MHz, some military UAS (Orlan-10 variants)
- **Modulation**: LoRa (ELRS/CRSF), FHSS
- **Detection**: RTL-SDR with broadband scan, look for LoRa chirp spread spectrum
- **rtl_433**: Some protocols in this band

### 915 MHz (US ISM Band)
- **Range**: 902–928 MHz
- **Used By**: ExpressLRS 915 MHz, TBS Crossfire 915 MHz, SiK telemetry radios (MAVLink), some military UAS
- **Modulation**: LoRa (ELRS/CRSF), GFSK (SiK), FHSS
- **Detection**: RTL-SDR broadband scan 900-930 MHz
- **Power Levels**: Up to 1W (30 dBm) for ELRS/CRSF

### GPS/GNSS Frequencies (Monitor for Jamming/Spoofing)
| System | Frequency (MHz) | Signal |
|--------|-----------------|--------|
| GPS L1 | 1575.42 | C/A code, all drones |
| GPS L2 | 1227.60 | P(Y) code, military drones |
| GPS L5 | 1176.45 | New civil signal |
| GLONASS L1 | 1598.0625–1605.375 | FDMA, Russian drones |
| GLONASS L2 | 1242.9375–1248.625 | FDMA |
| Galileo E1 | 1575.42 | EU GNSS |
| BeiDou B1 | 1561.098 | Chinese GNSS |

**All GNSS frequencies are within RTL-SDR range and can be monitored for jamming activity.**

### 1.2-1.3 GHz (L-band Video)
- **Used By**: Long-range FPV video (analog), some military drone video downlinks
- **Range**: 1080-1360 MHz typically
- **Detection**: RTL-SDR (within range)
- **Note**: Also used by GPS, ADS-B (1090 MHz), cell towers

### Specific Military Sub-GHz Signatures to Monitor
| Frequency Range | Potential Source | Notes |
|----------------|-----------------|-------|
| 148-174 MHz | Military UHF handheld, drone C2 | SINCGARS, tactical radios |
| 225-400 MHz | Military UHF SATCOM | UAS C2 backup |
| 420-450 MHz | Military radiolocation | Shared with amateur |
| 860-870 MHz | Orlan-10 C2 link | LoRa-like modulation |
| 900-930 MHz | ELRS/CRSF, military drone C2 | Common detection band |
| 1350-1400 MHz | Military L-band UAS | Some tactical systems |
| 1452-1492 MHz | Video downlink | Some Russian/Chinese UAS |

---

# 16. SDR Detection Parameters

## RTL-SDR Scan Bands (24-1766 MHz)
| Center Freq | BW | What to Look For |
|-------------|-----|-----------------|
| 433.92 MHz | 2 MHz | RC toy drones, telemetry |
| 868 MHz | 10 MHz | ELRS/CRSF EU, Orlan-10 |
| 915 MHz | 26 MHz | ELRS/CRSF US, SiK radios |
| 1090 MHz | 2 MHz | ADS-B (for correlation with drone tracks) |
| 1575.42 MHz | 4 MHz | GPS L1 jamming detection |
| 1602 MHz | 10 MHz | GLONASS L1 jamming |
| 1400 MHz | 100 MHz | L-band video/data downlinks |

## HackRF Scan Bands (1 MHz - 6 GHz)
| Center Freq | BW | What to Look For |
|-------------|-----|-----------------|
| 2.44 GHz | 84 MHz | WiFi, ELRS 2.4G, FrSky, Flysky, DJI 2.4G |
| 5.8 GHz | 200 MHz | FPV analog, DJI digital, HDZero, Walksnail, DJI control |
| 5.2 GHz | 200 MHz | C-band military datalinks (TB2, NATO UAS) |
| 4.5 GHz | 500 MHz | C-band military (broader scan) |

## Signal Classification Parameters
| Protocol | Bandwidth | Modulation | Duty Cycle | Hop Rate |
|----------|-----------|------------|------------|----------|
| DJI OcuSync | 10-40 MHz | OFDM+FHSS | ~80% | Fast |
| DJI DroneID | ~9 MHz | OFDM | ~1% (burst) | N/A |
| Analog FPV 5.8G | ~18 MHz | FM | 100% (continuous) | None |
| DJI O3 Digital | ~40 MHz | OFDM | ~90% | Slow |
| HDZero | ~20 MHz | Digital | ~90% | None |
| Walksnail | ~40 MHz | OFDM | ~90% | Slow |
| ELRS 2.4G | ~0.8 MHz | LoRa | ~5-10% | Fast (per packet rate) |
| ELRS 868/915 | ~0.5 MHz | LoRa | ~5-10% | Fast |
| Crossfire | ~0.5 MHz | LoRa | ~5-10% | Medium |
| FrSky ACCST | ~2 MHz | FHSS | ~10% | ~45/sec |
| Flysky AFHDS | ~1 MHz | GFSK | ~15% | ~260/sec |
| Autel SkyLink | 20-40 MHz | OFDM+FHSS | ~80% | Fast |

---

# 17. WiFi Frame Detection Guide

## 802.11 Frame Types to Monitor

### For Drone Detection:
| Frame Type | Subtype | Description | Drone Relevance |
|-----------|---------|-------------|----------------|
| 0 (Mgmt) | 0 (Assoc Req) | Association | Drone connecting to controller |
| 0 (Mgmt) | 4 (Probe Req) | Probe | Drone scanning |
| 0 (Mgmt) | 5 (Probe Resp) | Probe Response | Controller responding |
| 0 (Mgmt) | 8 (Beacon) | Beacon | Drone AP or controller AP; may contain RemoteID or DroneID |
| 0 (Mgmt) | 13 (Action) | Action Frame | **WiFi NAN RemoteID** |
| 2 (Data) | 0 (Data) | Data | Control/telemetry traffic |

### Vendor Specific IE Detection (in Beacons):
```
Tag Number: 0xDD (Vendor Specific)
  OUI: 60:60:1F (DJI) → Likely DroneID data
  OUI: 50:6F:9A (Wi-Fi Alliance) → Possible NAN/RemoteID
  OUI: 90:03:B7 (Parrot) → Parrot drone data
```

### DJI Enhanced WiFi Protocol Markers:
- DJI drones using "Enhanced WiFi" create AP with SSIDs matching DJI patterns
- Beacon frames contain Vendor Specific IEs with DJI OUI
- **Monitor for**: Any beacon from DJI OUI MAC addresses with 0xDD IE tags

### WiFi NAN Discovery Channel:
- **2.4 GHz**: Channel 6 (2437 MHz) — NAN discovery window
- **5 GHz**: Channel 44 (5220 MHz) or Channel 149 (5745 MHz)
- NAN uses periodic Discovery Windows on these channels
- **Monitor these channels** for NAN frames carrying ODID

---

# 18. Known SSID Patterns

| Pattern | Manufacturer/Type | Notes |
|---------|------------------|-------|
| `DJI-*` | DJI (Enhanced WiFi mode) | Older models, Spark |
| `TELLO-*` | Ryze/DJI Tello | WiFi direct control |
| `ANAFI-*` | Parrot ANAFI | WiFi direct |
| `Anafi-*` | Parrot ANAFI | Variant casing |
| `Bebop2-*` | Parrot Bebop 2 | WiFi direct |
| `DISCO-*` | Parrot Disco | WiFi direct |
| `Mambo-*` | Parrot Mambo | WiFi/BLE |
| `Autel-*` | Autel Robotics | WiFi mode |
| `EVO-*` | Autel EVO series | WiFi mode |
| `Skydio-*` | Skydio | WiFi AP mode |
| `YUNEEC-*` | Yuneec | WiFi mode |
| `YuneecH520-*` | Yuneec H520 | WiFi mode |
| `FIMI_*` | FIMI/Xiaomi | WiFi mode |
| `Hubsan-*` | Hubsan | WiFi FPV |
| `ZINO-*` | Hubsan Zino series | WiFi FPV |
| `HolyStone-*` | Holy Stone | WiFi FPV |
| `HS-*` | Holy Stone (short) | WiFi FPV |
| `FPV_*` | Generic FPV drone | WiFi video |
| `WiFi-UFO-*` | Generic/Eachine | Toy drone WiFi |
| `SG-*` | SG906/other budget | WiFi FPV |
| `SJRC*` | SJRC drones | WiFi FPV |
| `JJRC*` | JJRC drones | WiFi FPV |
| `MJX*` | MJX drones | WiFi FPV |
| `Potensic*` | Potensic drones | WiFi FPV |
| `Ruko*` | Ruko drones | WiFi FPV |
| `VISUO*` | VISUO/XS drones | WiFi FPV |

### SSID Detection Regex:
```python
import re
DRONE_SSID_PATTERNS = [
    re.compile(r'^DJI[-_]', re.I),
    re.compile(r'^TELLO[-_]', re.I),
    re.compile(r'^ANAFI[-_]', re.I),
    re.compile(r'^Bebop', re.I),
    re.compile(r'^DISCO[-_]', re.I),
    re.compile(r'^Mambo[-_]', re.I),
    re.compile(r'^Autel[-_]', re.I),
    re.compile(r'^EVO[-_]', re.I),
    re.compile(r'^Skydio[-_]', re.I),
    re.compile(r'^YUNEEC', re.I),
    re.compile(r'^FIMI[-_]', re.I),
    re.compile(r'^Hubsan[-_]', re.I),
    re.compile(r'^ZINO[-_]', re.I),
    re.compile(r'^HolyStone[-_]', re.I),
    re.compile(r'^HS[-_]\d', re.I),
    re.compile(r'^FPV[-_]', re.I),
    re.compile(r'^WiFi[-_]UFO', re.I),
    re.compile(r'^SG[-_]\d', re.I),
    re.compile(r'^SJRC', re.I),
    re.compile(r'^JJRC', re.I),
    re.compile(r'^MJX', re.I),
    re.compile(r'^Potensic', re.I),
    re.compile(r'^Ruko', re.I),
]
```

---

# 19. Detection Priority Matrix

## By Detection Method & Drone Category

### WiFi Monitor Mode (wlan6) — Highest Priority
| Detection Method | Targets | Confidence |
|-----------------|---------|------------|
| OUI MAC matching | DJI, Parrot, Autel | HIGH |
| SSID pattern matching | All WiFi-based drones | HIGH |
| WiFi NAN RemoteID frames | DJI, Parrot (ASTM F3411) | HIGH |
| WiFi Beacon RemoteID | Skydio, Autel, Parrot | HIGH |
| Vendor Specific IE (DroneID) | DJI (proprietary) | HIGH |
| WiFi probe req/resp patterns | All WiFi drones | MEDIUM |

### BLE Scanner (hci0) — High Priority
| Detection Method | Targets | Confidence |
|-----------------|---------|------------|
| ODID Service UUID 0xFFFA | All RemoteID compliant drones | HIGH |
| DJI BLE manufacturer data | DJI drones | HIGH |
| Parrot BLE advertisements | Parrot drones | MEDIUM |
| BLE advertisement rate analysis | Various | LOW |

### RTL-SDR (24-1766 MHz) — Medium Priority
| Detection Method | Targets | Confidence |
|-----------------|---------|------------|
| 868/915 MHz LoRa detection | ELRS, Crossfire, military | HIGH |
| 433 MHz ISM scanning | Toy drones, telemetry | MEDIUM |
| GPS L1 jam detection (1575 MHz) | All GPS drones (indirect) | MEDIUM |
| L-band video/data (1.0-1.5 GHz) | Military UAS, long-range FPV | MEDIUM |

### HackRF (1 MHz - 6 GHz) — Critical for FPV & Military
| Detection Method | Targets | Confidence |
|-----------------|---------|------------|
| 5.8 GHz analog FPV scan | FPV racing drones | HIGH |
| 5.8 GHz digital FPV (40MHz OFDM) | DJI O3, Walksnail, HDZero | HIGH |
| DJI DroneID OFDM demod | DJI drones | HIGH |
| 2.4 GHz FHSS analysis | All 2.4G RC links | MEDIUM |
| C-band 4-6 GHz scan | Military (TB2, NATO UAS) | MEDIUM |
| DJI OcuSync FHSS in 2.4/5.8 GHz | DJI drones | MEDIUM |

---

## Key Reference Tools & Projects
| Tool | URL | Purpose |
|------|-----|---------|
| opendroneid-core-c | github.com/opendroneid/opendroneid-core-c | Parse ODID BLE/WiFi messages |
| DroneSecurity | github.com/RUB-SysSec/DroneSecurity | DJI DroneID research/decoder |
| samples2djidroneid | github.com/anarkiwi/samples2djidroneid | Decode DJI DroneID from I/Q |
| antsdr_dji_droneid | github.com/alphafox02/antsdr_dji_droneid | Real-time DJI DroneID detection |
| WiFi-RemoteID | github.com/lukeswitz/WiFi-RemoteID | WiFi RemoteID to Meshtastic |
| rtl_433 | github.com/merbanan/rtl_433 | Decode 433 MHz ISM protocols |
| RF-Drone-Detection | github.com/tesorrells/RF-Drone-Detection | WiFi-based drone detection |
| dji_protocol | github.com/samuelsadok/dji_protocol | DJI protocol documentation |

---

*Document generated from multi-source OSINT research. Frequency data compiled from manufacturer specifications, FCC filings, academic papers (NDSS 2023), open-source intelligence, and community documentation. Military frequencies are estimates based on publicly available analysis and should be treated as approximate.*
