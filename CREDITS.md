# Credits and Acknowledgments

SIGINT-Deck builds upon the work of many open source projects and researchers. We gratefully acknowledge their contributions.

---

## Core Dependencies

### SDR Software
- **RTL-SDR** - https://github.com/osmocom/rtl-sdr
  - License: GPL-2.0
  - Core RTL2832U driver for software defined radio
  
- **rtl_433** - https://github.com/merbanan/rtl_433
  - License: GPL-2.0
  - ISM band decoder for 433MHz/315MHz devices
  - Author: Benjamin Larsson (merbanan)

- **HackRF** - https://github.com/greatscottgadgets/hackrf
  - License: GPL-2.0
  - Great Scott Gadgets
  
- **LimeSuite** - https://github.com/myriadrf/LimeSuite
  - License: Apache-2.0
  - Lime Microsystems / Myriad-RF

- **SoapySDR** - https://github.com/pothosware/SoapySDR
  - License: BSL-1.0
  - Pothosware / Josh Blum

- **kalibrate-rtl** - https://github.com/steve-m/kalibrate-rtl
  - License: BSD-2-Clause
  - Steve Markgraf (steve-m)

### Flipper Zero Integration

- **V3SP3R** - https://github.com/elder-plinius/V3SP3R
  - License: GPL-3.0
  - Author: elder-plinius (Pliny)
  - AI-powered Flipper Zero control via natural language
  - Inspiration for Flipper integration architecture, risk classification, 
    and execute_command_schema (28 actions)
  - Contributors: Claude (Anthropic)

- **Flipper Zero Firmware** - https://github.com/flipperdevices/flipperzero-firmware
  - License: GPL-3.0
  - Flipper Devices Inc.
  - CLI protocol and file format specifications

### Meshtastic

- **Meshtastic** - https://github.com/meshtastic/firmware
  - License: GPL-3.0
  - Meshtastic Project
  - LoRa mesh networking protocol

### TSCM / Counter-Surveillance Research

- **Granite Island Group / TSCM.com**
  - James M. Atkinson
  - Surveillance frequency database and TSCM methodology
  - https://www.tscm.com/

- **ENIGMA 2000 / Priyom.org**
  - Numbers station frequencies and scheduling
  - https://priyom.org/

### RF Protocol Research

- **FCC Part 15/90 Regulations**
  - Unlicensed device frequency allocations
  
- **3GPP Specifications**
  - Cellular band definitions (LTE, 5G NR)

- **SINCGARS / HAVEQUICK / Link 16**
  - Military radio specifications (public documentation)

---

## Hardware Platforms

### ClockworkPi uConsole
- **ClockworkPi** - https://www.clockworkpi.com/
  - Hardware design and base OS
  
- **HackerGadgets** - https://hackergadgets.com/
  - AIO V2 expansion board (RTL-SDR, LoRa, GPS, RTC)
  - uConsole upgrade kit
  - Author: Vileer

- **uConsole World** - https://uconsole.net/
  - Community documentation and expansion card roundups

### Raspberry Pi
- **Raspberry Pi Foundation** - https://www.raspberrypi.org/
  - Compute Module 4/5 hardware

### Steam Deck
- **Valve Corporation**
  - Steam Deck hardware and SteamOS

---

## Rust Crates

Core dependencies (see Cargo.toml for full list):

- `tokio` - Async runtime (MIT)
- `actix-web` - Web framework (MIT/Apache-2.0)
- `serde` - Serialization (MIT/Apache-2.0)
- `tracing` - Diagnostics (MIT)
- `pcap` - Packet capture (MIT/Apache-2.0)
- `btleplug` - Bluetooth LE (MIT/Apache-2.0)
- `serialport` - Serial communication (MPL-2.0)
- `rusqlite` - SQLite bindings (MIT)

---

## Research & Documentation Sources

### Drone Detection
- Academic research on UAV RF signatures
- ESC PWM harmonic analysis methodology
- DJI OcuSync/Lightbridge frequency documentation

### WiFi Security
- IEEE 802.11 specifications
- Aircrack-ng project documentation

### GPS
- NMEA 0183 protocol specification
- gpsd project (BSD)

---

## Contributors

### SIGINT-Deck Team
- Primary development and integration

### AI Assistance
- Claude (Anthropic) - Code generation and documentation assistance
- factory-droid[bot] - Automated contributions

---

## Legal Resources

- **Electronic Frontier Foundation (EFF)**
  - RayHunter IMSI catcher detection
  - https://github.com/EFForg/rayhunter

- **ACLU**
  - Stingray/IMSI catcher documentation

---

## Special Thanks

- The open source SDR community
- Flipper Zero community and firmware developers
- Meshtastic community
- Security researchers who document surveillance technologies
- Hardware hackers building expansion modules

---

## License Compliance

This project aims to comply with all upstream licenses. If you believe there is a license compliance issue, please open an issue.

Key licenses used by dependencies:
- GPL-2.0 (RTL-SDR, rtl_433, HackRF)
- GPL-3.0 (V3SP3R, Flipper firmware, Meshtastic)
- Apache-2.0 (LimeSuite)
- MIT (Various Rust crates)
- BSD (kalibrate-rtl, gpsd)

---

## Disclaimer

This software is provided for educational and authorized security research purposes only. The inclusion of any project in this credits file does not imply endorsement of SIGINT-Deck by that project.

This project is licensed under the GNU General Public License v3.0 (GPL-3.0).
See [LICENSE](LICENSE) for the full license text and [LEGAL.md](LEGAL.md) for important legal information.
