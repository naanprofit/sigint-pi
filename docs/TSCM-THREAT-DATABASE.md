# SIGINT-Deck Technical Surveillance Countermeasures (TSCM) Threat Database

**WARNING**: This documentation is for DEFENSIVE counter-surveillance purposes only. 
Unauthorized interception of communications is illegal in most jurisdictions.

## Table of Contents

1. [Overview](#overview)
2. [Surveillance Frequency Bands](#surveillance-frequency-bands)
3. [Device Signatures](#device-signatures)
4. [Federal/Government Frequencies](#federal-government-frequencies)
5. [Numbers Stations](#numbers-stations)
6. [Military Communications](#military-communications)
7. [IMSI Catchers](#imsi-catchers)
8. [Environmental Sensors](#environmental-sensors)
9. [Sweep Configurations](#sweep-configurations)
10. [Detection Methods](#detection-methods)

---

## Overview

This database compiles known surveillance device frequencies, RF signatures, and detection methods from:
- Granite Island Group / TSCM.com (James M. Atkinson)
- FCC Part 15/90 regulations
- ENIGMA 2000 / Priyom.org (numbers stations)
- Military specifications (SINCGARS, HAVEQUICK, Link 16)
- Academic research on IMSI catchers
- 3GPP cellular specifications

---

## Surveillance Frequency Bands

### Carrier Current (Power Line) Bugs
The most common and hardest to detect bugs use power lines for transmission.

| Band | Frequency | Notes |
|------|-----------|-------|
| VLF Carrier Current | 50 kHz - 750 kHz | Uses AC power lines, 47 CFR 15.219 |
| HF Carrier Current | 300 kHz - 50 MHz | AC mains antenna, 47 CFR 15.207 |

### VHF Audio Bugs (Most Popular)

| Band | Frequency | Description |
|------|-----------|-------------|
| Ultra Low Power | 25-80 MHz | Micro-watt devices |
| FM Broadcast Band | 65-130 MHz | Hides in FM broadcast, Part 15 |
| Body Wire Band I | 130-150 MHz | Wireless microphones |
| Body Wire Band II | 150-174 MHz | Very common body wires |
| Body Wire Band III | 174-225 MHz | In-band wireless mics |

### UHF Surveillance Devices

| Band | Frequency | Description |
|------|-----------|-------------|
| Tactical Bug Band | 225-400 MHz | Throw-away bugs, beer can bugs |
| Micro-Powered Bugs | 290-330 MHz | Cigarette butt bugs, wafer bugs (5-10 µW) |
| **SpyShop Popular** | 330-440 MHz | 398.605, 399.030 MHz very popular |
| ISM 433 MHz | 430-550 MHz | 433.920, 418 MHz popular |
| ISM 900 MHz | 800-990 MHz | 902-985 MHz video bugs |

### Specific SpyShop Frequencies (High Priority)

These frequencies are documented as the most commonly used by commercial spy shops:

| Frequency | Type | Source |
|-----------|------|--------|
| **398.605 MHz** | Audio Bug | TSCM.com - Very Popular |
| **300.455 MHz** | Audio Bug | TSCM.com - Popular |
| **399.030 MHz** | Audio Bug | TSCM.com - Popular |
| **433.920 MHz** | Audio/Video | ISM Band |
| **418.000 MHz** | Audio Bug | Common |

### Microwave Video Bugs

| Band | Frequency | Notes |
|------|-----------|-------|
| 1.2 GHz Band | 1.1-1.3 GHz | Very popular video |
| 1.4 GHz Band | 1.4-1.5 GHz | Common video |
| 1.7 GHz Band | 1.7-1.9 GHz | **1.710-1.755 GHz very popular** |
| **2.4 GHz Band** | 2.4-2.5 GHz | **EXTREMELY popular** |
| **5.8 GHz Band** | 5.6-7.5 GHz | **Becoming very popular** |
| 8-13 GHz | 8.1-13 GHz | High-end video bugs |

---

## Federal Government Frequencies

### Primary Federal Surveillance Bands

From TSCM.com documentation:

| Band | Primary | Secondary |
|------|---------|-----------|
| VHF | 25-75 MHz | - |
| VHF | 135-175 MHz | 175-220 MHz |
| UHF | 225-440 MHz | 440-525 MHz |
| Microwave | 630-890 MHz | 890 MHz - 1.71 GHz |
| Microwave | 1.71-1.95 GHz | 1.95-5.50 GHz |
| High MW | 5.50-12.5 GHz | 12.5-39.6 GHz |

### Specific Federal Frequencies (Documented)

| Frequency | Agency | Use |
|-----------|--------|-----|
| 27.5750 MHz | Customs | Low power < 5W |
| 27.5850 MHz | Customs | Low power < 5W |
| 40.1200 MHz | Federal | Bumper beepers |
| 40.1700 MHz | Federal | Bumper beepers |
| 40.2200 MHz | Federal | Bumper beepers |
| 40.2700 MHz | Federal | Bumper beepers |
| 163.1000 MHz | Customs | Low power < 30W |
| **164.9125 MHz** | FBI | Surveillance |
| **165.9125 MHz** | ATF | Surveillance |
| 166.2875 MHz | ATF | Operations |
| 170.4125 MHz | ATF | Operations |
| 171.6000 MHz | DEA | CH.2 |
| 172.2000 MHz | DEA | CH.1 |
| 406.2750 MHz | Secret Service | |
| **407.8000 MHz** | Secret Service/CIA | Also State Dept |
| 408.0500 MHz | Federal Shared | |
| 408.5000 MHz | Secret Service | |
| 408.5750 MHz | Federal Shared | |
| 408.9750 MHz | Secret Service | |
| 409.4000 MHz | Federal Shared | |
| **418.0500 MHz** | DEA | Low power |
| **418.0750 MHz** | DEA | Low power |
| **418.5750 MHz** | DEA/Customs | Low power |
| 418.6750 MHz | DEA | F4 EMILY Surveillance |
| 418.7500 MHz | DEA | F3 GAIL Surveillance |
| 418.9000 MHz | DEA | F2 CINDY Surveillance |

### Wide Band Hopping Frequencies

Federal agencies use frequency hopping centered on UHF TV channels:
- 510 MHz ± 25 MHz hopping width
- 670 MHz ± 25 MHz hopping width

---

## Numbers Stations

### Active Numbers Stations (as of 2024-2025)

From Priyom.org and ENIGMA 2000:

| Designator | Name | Language | Operator | Frequencies (kHz) |
|------------|------|----------|----------|-------------------|
| E11 | Polish 11 | English | Polish Intelligence | 4780, 5422, 6825, 8190 |
| S06 | Russian Man | Russian | Russian Intel | 4625, 5154, 6853, 7039 |
| **S28** | **The Buzzer (UVB-76)** | Russian | Russian Military | 4625 |
| **S30** | **The Pip** | Russian | Russian Military | 3756, 5448 |
| HM01 | Cuban | Spanish | Cuban DI | 5855, 7375, 9330, 11435 |
| V26 | New Star Radio | Chinese | PLA | 8300, 9725, 11430, 13750 |
| M14 | - | Morse | Russian Military | 5240, 6840, 8120 |

### New Iranian Station (March 2026)

A new Farsi numbers station was detected broadcasting during the Iran conflict:
- **7910 kHz USB** - transmits structured numeric groups
- Traced to US military base in Germany
- Schedule appears repeatable

### Historical Stations (Inactive but Notable)

| Designator | Name | Operator | Status |
|------------|------|----------|--------|
| E03 | Lincolnshire Poacher | MI6/GCHQ | Ceased 2008 |
| E10 | Mossad | Israeli Mossad | Ceased |

---

## Military Communications

### US Military Tactical Radio

| System | Frequency | Modulation | Notes |
|--------|-----------|------------|-------|
| SINCGARS | 30-88 MHz | FM/FHSS | Army/Marines tactical |
| HAVEQUICK | 225-400 MHz | AM/FHSS | Air-ground |
| Link 16 | 969-1206 MHz | TDMA/FHSS | Tactical data link |
| SATCOM | Various | Various | Encrypted |

### Federal Law Enforcement

| Band | Frequency | Users |
|------|-----------|-------|
| VHF | 162-174 MHz | FBI, DEA, ATF, USMS |
| UHF | 406-420 MHz | FBI, Secret Service, DEA |
| 700 MHz | 758-775 MHz | FirstNet public safety |
| 800 MHz | 851-869 MHz | P25 trunked systems |

### Private Military Contractors (PMC)

Limited public information available. Known to use:
- Commercial satellite phones (Iridium, Thuraya)
- HF/VHF/UHF tactical radios
- Encrypted digital systems
- Often indistinguishable from military communications

---

## IMSI Catchers / Cell Site Simulators

### Frequency Bands

| Band | Frequency | Technology |
|------|-----------|------------|
| 700 MHz | 698-756 MHz | LTE Band 12/13/17 |
| 850 MHz | 824-894 MHz | GSM/CDMA/LTE |
| 1900 MHz | 1850-1990 MHz | PCS GSM/LTE |
| AWS | 1710-2155 MHz | LTE |

### Detection Indicators

- Forced 2G downgrade (GSM instead of LTE)
- IMSI Identity Request from tower
- Unusual cell ID
- Strong signal from unknown tower
- Unexpected roaming indicator

### Known Equipment

| Device | Manufacturer | Notes |
|--------|--------------|-------|
| StingRay I/II | Harris Corp | Original IMSI catcher |
| Hailstorm | Harris Corp | Newer version |
| Gossamer | Harris Corp | Portable |
| DRTBox/DirtBox | Digital Receiver Technology | Airborne capable |
| Triggerfish | Unknown | FBI use documented |

---

## Environmental Sensors

### Geiger Counters (Linux Compatible)

| Model | Interface | Features | Cost |
|-------|-----------|----------|------|
| GQ GMC-320+ | USB Serial | Basic, data logging | $100-150 |
| GQ GMC-500+ | USB + WiFi | Dual tube, alpha/beta/gamma | $200-250 |
| Radiacode 101/102 | BT + USB | Isotope identification | $300-400 |
| MightyOhm Kit | Serial TTL | DIY, open source | $100 |

### Radiation Levels

| Level | µSv/h | Action |
|-------|-------|--------|
| Normal | < 0.2 | Background |
| Elevated | 0.2-0.5 | Monitor |
| High | 0.5-1.0 | Investigate |
| Very High | 1.0-10.0 | Leave area |
| **Dangerous** | > 10.0 | **Evacuate immediately** |

### Air Quality Sensors

| Sensor | Measures | Interface | Cost |
|--------|----------|-----------|------|
| Sensirion SCD4x | CO2, Temp, Humidity | I2C | $40-50 |
| Sensirion SEN55 | PM, VOC, NOx, T/H | I2C | $30-50 |
| Sensirion SEN66 | PM, VOC, NOx, CO2, T/H | I2C | $50-70 |
| Plantower PMS5003 | PM1.0/2.5/10 | UART | $20-30 |

### AQI Categories

| AQI | Category | PM2.5 (µg/m³) |
|-----|----------|---------------|
| 0-50 | Good | 0-12 |
| 51-100 | Moderate | 12.1-35.4 |
| 101-150 | Unhealthy (Sensitive) | 35.5-55.4 |
| 151-200 | Unhealthy | 55.5-150.4 |
| 201-300 | Very Unhealthy | 150.5-250.4 |
| 301+ | **Hazardous** | 250.5+ |

---

## Sweep Configurations

### Quick Sweep (5 minutes)

Best for rapid initial check:

| Band | Frequency |
|------|-----------|
| FM Band Bugs | 65-130 MHz |
| UHF Bugs | 330-550 MHz |
| Video/Cellular | 800 MHz - 1 GHz |
| 2.4 GHz Video | 2.4-2.5 GHz |

### Standard Sweep (30 minutes)

Recommended minimum for serious threats:

| Band | Frequency |
|------|-----------|
| Carrier Current | 100 kHz - 50 MHz |
| VHF/UHF | 50-500 MHz |
| Microwave | 500 MHz - 3 GHz |
| High Video | 5-6.5 GHz |

### Full TSCM Sweep (2+ hours)

Professional-grade inspection:

| Band | Frequency |
|------|-----------|
| VLF | 9 kHz - 150 kHz |
| Carrier Current | 150 kHz - 50 MHz |
| VHF/UHF | 50 MHz - 1 GHz |
| Microwave | 1-6 GHz |
| High Microwave | 6-18 GHz |
| K-Band | 18-26.5 GHz |

### Federal Threat Sweep

Focus on government surveillance bands:

| Band | Frequency |
|------|-----------|
| Federal Primary I | 25-75 MHz |
| Federal Primary II | 135-220 MHz |
| Federal Primary III | 225-525 MHz |
| Federal Microwave | 630 MHz - 1.95 GHz |
| High Federal | 1.95-5.5 GHz |
| Very High | 5.5-12.5 GHz |

---

## Detection Methods

### RF Signal Analysis

1. **Spectrum Analysis**: Monitor for unexpected signals
2. **Baseline Comparison**: Establish normal spectrum, detect deviations
3. **Signal Fingerprinting**: Match known device signatures
4. **Power Level Analysis**: Bugs typically < 50 mW
5. **Modulation Analysis**: Identify FM, AM, digital, spread spectrum

### Carrier Current Detection

1. Use coupler to monitor power lines
2. Check frequencies 50 kHz - 50 MHz
3. Look for audio-correlated signals
4. Check all circuits, not just outlets

### Non-Linear Junction Detection (NLJD)

- Detects semiconductor junctions (even unpowered bugs)
- Requires specialized equipment
- False positives from corrosion, nails, etc.

### Infrared Detection

- 850-950 nm most common for IR bugs
- Use IR camera or detector
- Check for modulated IR signals

### Physical Inspection

- Visual inspection of all surfaces
- Check outlets, light fixtures, smoke detectors
- Examine phone jacks, ethernet ports
- Inspect HVAC vents and ducts
- Check furniture, picture frames, clocks

---

## References

1. Granite Island Group - http://www.tscm.com/
2. Priyom.org - https://priyom.org/
3. ENIGMA 2000 - http://www.signalshed.com/
4. The Conet Project - Archive.org
5. EFF - https://www.eff.org/pages/cell-site-simulatorsimsi-catchers
6. 3GPP Specifications - https://www.3gpp.org/
7. FCC Part 15/90 Regulations
8. MIL-STD Communications Specifications

---

*This document is for educational and defensive purposes only.*
*Always comply with all applicable laws regarding radio monitoring.*
