# SIGINT Threat Intelligence Database - Research Documentation

## Research Date: 2026-03-30
## Purpose: Comprehensive OUI/MAC identification for intelligence, defense, law enforcement, and surveillance equipment

---

## UNITED STATES

### Intelligence Community

#### NSA (National Security Agency)
- **Primary Contractors**: Lockheed Martin, Raytheon, Northrop Grumman, General Dynamics, L3Harris
- **TAO (Tailored Access Operations)**: ANT Catalog revealed hardware implants
- **ANT Catalog Items** (Snowden Leaks 2013):
  - COTTONMOUTH: USB implant for air-gapped networks
  - DEITYBOUNCE: Dell server implant
  - HEADWATER: Huawei router implant
  - JETPLOW: Cisco firewall implant
  - FEEDTROUGH: Juniper firewall implant
  - IRONCHEF: HP server implant
  - STUCCOMONTANA: Network tap
- **Sources**: Der Spiegel (2013), The Intercept (2016), Schneier on Security

#### NRO (National Reconnaissance Office)
- Satellite surveillance hardware
- Contractors: Lockheed Martin, Ball Aerospace, Northrop Grumman

#### CIA
- **In-Q-Tel** portfolio companies (tech surveillance investments)
- Palantir Technologies
- Keyhole Inc (now Google Earth)

#### FBI
- **Stingray/IMSI Catchers**: Harris Corporation (now L3Harris)
  - StingRay, StingRay II, Hailstorm, Kingfish
  - DRT (Digital Receiver Technology)
- **Cellebrite**: Mobile forensics
- **GrayKey**: iPhone unlocking

### Defense Contractors (OUI Registrations)
| Vendor | OUI Prefixes | Category |
|--------|--------------|----------|
| Lockheed Martin | Multiple | US Defense |
| Raytheon | 00:04:A3 | US Defense |
| Northrop Grumman | Multiple | US Defense |
| General Dynamics | 00:01:F5 | US Defense |
| L3Harris | 00:04:DE, 00:0B:E3 | US Defense/Law Enforcement |
| BAE Systems | 00:18:B4 | US/UK Defense |
| Booz Allen Hamilton | N/A | US Intel Contractor |
| SAIC | Multiple | US Intel Contractor |
| CACI International | N/A | US Intel Contractor |
| Leidos | N/A | US Intel Contractor |

### Law Enforcement Equipment
| Vendor | Equipment | OUI |
|--------|-----------|-----|
| L3Harris | Stingray IMSI Catchers | 00:04:DE |
| Axon (TASER) | Body Cameras | Multiple |
| Motorola Solutions | Police Radios/BWC | 00:04:56, 00:08:AF |
| Vigilant Solutions | ALPR | N/A |
| ShotSpotter | Gunshot Detection | N/A |
| Cellebrite | Mobile Forensics | N/A |
| GrayKey (Grayshift) | iPhone Forensics | N/A |

---

## RUSSIA

### Intelligence Services
- **FSB** (Federal Security Service) - Domestic
- **GRU** (Military Intelligence) - Foreign/Military
- **SVR** (Foreign Intelligence Service) - Foreign

### State Corporations
| Entity | Description | Category |
|--------|-------------|----------|
| Rostec | State defense conglomerate | Russian State |
| Ruselectronics | Electronics subsidiary | Russian State |
| Shvabe | Optical/surveillance | Russian State |
| Kalashnikov Concern | Defense | Russian State |

### Known Vendors (Sanctioned)
| Vendor | OUI Prefixes | Notes |
|--------|--------------|-------|
| Positive Technologies | N/A | Cyber offensive tools |
| Group-IB | N/A | Cyber intelligence |
| Kaspersky Lab | N/A | Antivirus (banned by US Gov) |

### Sources
- US Treasury OFAC Sanctions Lists
- Wilson Center Report on Rostec (2019)

---

## CHINA

### Intelligence/Military
- **MSS** (Ministry of State Security) - Civilian intelligence
- **PLA** (People's Liberation Army) - Military
- **PLA-SSF** (Strategic Support Force) - Cyber/Space

### Banned/Restricted Vendors (US NDAA)
| Vendor | OUI Prefixes | Threat Level |
|--------|--------------|--------------|
| Huawei | 00:18:82, 00:1E:10, 00:25:9E, 00:46:4B, 00:E0:FC, 04:02:1F, 04:BD:70, 08:19:A6 | Critical |
| ZTE | 00:15:EB, 00:19:C6, 00:1E:73, 00:25:12, 00:26:ED | Critical |
| Hikvision | 18:68:CB, 28:57:BE, 44:19:B6, 54:C4:15, 5C:DD:70, 7C:11:CB, 80:A2:35, A4:14:37, BC:AD:28, C0:51:7E, C4:2F:90, D4:43:0E, E0:50:8B | High |
| Dahua | 00:1A:6B, 14:A7:8B, 20:A6:80, 34:E6:AD, 3C:EF:8C, 54:C4:15, 64:9B:CD, 90:02:A9, A0:BD:CD, AC:B9:2F, B0:41:1D, BC:32:5F, E0:2F:6D | High |
| Hytera | 00:C0:26 | High |
| ZTE/Nubia | Multiple | High |
| China Mobile | Multiple | Medium |
| China Telecom | Multiple | Medium |

### AI/Surveillance Companies (Entity List)
| Company | Focus | Status |
|---------|-------|--------|
| SenseTime | Facial Recognition | Sanctioned |
| Megvii (Face++) | Facial Recognition | Sanctioned |
| Yitu Technology | AI Surveillance | Sanctioned |
| CloudWalk | Facial Recognition | Sanctioned |
| iFlytek | Voice Recognition | Sanctioned |

### Sources
- FCC Covered List
- Commerce Department Entity List
- IPVM Reports on Hikvision/PLA ties (2021)
- Reuters/Trump Admin designation (2020)

---

## ISRAEL

### Intelligence
- **Mossad** - Foreign intelligence
- **Shin Bet** - Domestic security  
- **Unit 8200** - SIGINT (equivalent to NSA)
- **Aman** - Military intelligence

### Cyber/Surveillance Vendors
| Vendor | Products | OUI | Status |
|--------|----------|-----|--------|
| NSO Group | Pegasus spyware | N/A | US Sanctioned |
| Candiru | Spyware | N/A | US Sanctioned |
| Cellebrite | Mobile forensics | N/A | Active |
| Verint | SIGINT systems | 00:03:C3 | Active |
| NICE Systems | Surveillance | 00:80:F4 | Active |
| Check Point | Network security | 00:00:97 | Active |
| Elbit Systems | Defense | Multiple | Active |
| Rafael | Defense | N/A | Active |
| IAI | Defense/UAV | N/A | Active |
| Cognyte (Verint spin-off) | Intelligence | N/A | Active |

### Sources
- Citizen Lab Reports on NSO/Pegasus
- The Intercept - Unit 8200 coverage
- Commerce Dept Entity List (NSO, Candiru)

---

## UNITED KINGDOM

### Intelligence
- **GCHQ** - SIGINT (Five Eyes member)
- **MI5** - Domestic
- **MI6** - Foreign

### Programs (Snowden Leaks)
- **Tempora**: Fiber optic tapping program
- **Karma Police**: Web browsing surveillance

### Vendors
| Vendor | OUI | Category |
|--------|-----|----------|
| BAE Systems | 00:18:B4 | Defense |
| Leonardo UK | Multiple | Defense |
| QinetiQ | N/A | Defense Research |
| Sophos | 00:1A:8C | Cybersecurity |

### Sources
- Guardian/Snowden disclosures (2013)
- Privacy International

---

## GERMANY

### Intelligence
- **BND** (Bundesnachrichtendienst) - Foreign
- **BfV** - Domestic

### Vendors
| Vendor | OUI | Category |
|--------|-----|----------|
| Rohde & Schwarz | 00:01:A4, 00:04:B4 | SIGINT/Defense |
| Diehl Defence | N/A | Defense |
| Hensoldt | N/A | Sensors/Radar |

### Sources
- Wikipedia - Rohde & Schwarz SIGINT history
- Der Spiegel reporting

---

## FRANCE

### Intelligence
- **DGSE** - Foreign
- **DGSI** - Domestic

### Vendors
| Vendor | OUI | Category |
|--------|-----|----------|
| Thales | 00:01:74, 00:09:89 | Defense/Surveillance |
| Airbus Defence | Multiple | Defense |
| Nexa Technologies (Amesys) | N/A | Surveillance (Controversial) |
| Bull/Atos | Multiple | IT/Surveillance |

### Controversies
- Amesys sold surveillance to Libya (Gaddafi regime)
- DGSE implicated in Nexa case

### Sources
- Mediapart - Predator Files
- Intelligence Online

---

## MIDDLE EAST

### United Arab Emirates
| Entity | Products | Notes |
|--------|----------|-------|
| DarkMatter Group | Surveillance | US sanctions |
| Group 42 (G42) | AI/Surveillance | Chinese ties |

### Saudi Arabia
| Entity | Role |
|--------|------|
| GIP | Intelligence service |
| SAMI | Defense industry |

### Turkey
| Vendor | OUI | Category |
|--------|-----|----------|
| Aselsan | Multiple | Defense Electronics |
| Havelsan | N/A | Defense Software |
| TAI | N/A | Aerospace |

### Iran
- **VAJA/MOIS** - Intelligence
- Uses Chinese surveillance tech (Tiandy, Huawei)

### Sources
- Citizen Lab - UAE/Pegasus
- The Intercept - DarkMatter
- BBC - BAE sales to Middle East

---

## ASIA-PACIFIC

### Japan
| Vendor | OUI | Category |
|--------|-----|----------|
| Mitsubishi Electric | 00:00:F4, 00:06:B1 | Defense |
| NEC | 00:00:0D, 00:00:4C | Defense/IT |
| Fujitsu | 00:00:0E, 00:0B:5D | Defense/IT |

### South Korea
| Vendor | OUI | Category |
|--------|-----|----------|
| Hanwha Systems | Multiple | Defense |
| Hanwha Vision (Samsung) | 00:09:18, 00:16:6B, 00:1D:25, 00:24:90, 08:DF:1F | Surveillance |
| LIG Nex1 | N/A | Defense |

### India
| Vendor | OUI | Category |
|--------|-----|----------|
| Bharat Electronics (BEL) | N/A | Defense |
| DRDO | N/A | Defense Research |
| HAL | N/A | Aerospace |

### Australia
| Vendor | OUI | Category |
|--------|-----|----------|
| BAE Systems Australia | 00:18:B4 | Defense |
| Thales Australia | 00:01:74 | Defense |

### Sources
- Nautilus Institute - Japan SIGINT
- Defense industry publications

---

## SURVEILLANCE SOFTWARE VENDORS (No Direct OUI)

### Spyware/Intrusion
| Vendor | Country | Products | Status |
|--------|---------|----------|--------|
| NSO Group | Israel | Pegasus | Sanctioned |
| Candiru | Israel | Spyware | Sanctioned |
| Intellexa/Cytrox | Greece/N.Macedonia | Predator | Sanctioned |
| FinFisher/Gamma | UK/Germany | FinSpy | Defunct (2021) |
| Hacking Team | Italy | RCS | Defunct |
| Circles | Israel | SS7 Exploitation | Active |
| Quadream | Israel | Reign | Active |

### Sources
- Citizen Lab research
- Access Now reports
- Commerce Dept Entity List

---

## ELECTRONIC WARFARE / JAMMING

### Maduro Raid (2026) - Operation Absolute Resolve
| System | Function | Platform |
|--------|----------|----------|
| EA-18G Growler | ECM/Jamming | US Navy |
| EC-130H Compass Call | Comms Jamming | US Air Force |
| RQ-170 Sentinel | Stealth ISR | CIA/USAF |
| ALQ-99 | Jamming Pod | USN |
| NGJ (Next Gen Jammer) | Jamming | USN |

### Sources
- Reuters (Jan 2026)
- The War Zone
- Defense Scoop

---

## UAP/ANOMALOUS PHENOMENA RESEARCH

### Official Programs
- **AARO** (All-domain Anomaly Resolution Office) - DoD
- **NASA UAP Study** (2022-2023)
- **Galileo Project** (Harvard)

### Relevant Technologies
- Multi-spectral sensors
- Radio frequency monitoring
- Infrared detection

### Sources
- DoD AARO Reports (2024)
- NASA UAP Independent Study (2023)
- arxiv.org research papers

---

## FIVE EYES ALLIANCE

### Members
1. **USA** - NSA
2. **UK** - GCHQ
3. **Canada** - CSE
4. **Australia** - ASD
5. **New Zealand** - GCSB

### Extended (Nine Eyes)
- Add: Denmark, France, Netherlands, Norway

### Extended (Fourteen Eyes)
- Add: Belgium, Germany, Italy, Spain, Sweden

### Sources
- Wikipedia - UKUSA Agreement
- Privacy International
- Snowden disclosures

---

## TRACKING DEVICES

| Vendor | Products | OUI Prefixes |
|--------|----------|--------------|
| Apple | AirTag | Multiple Apple OUIs |
| Tile | Trackers | Multiple |
| Samsung | SmartTag | Samsung OUIs |
| Chipolo | Trackers | N/A |
| LandAirSea | GPS Trackers | N/A |
| Spy Tec | GPS Trackers | N/A |

---

## METHODOLOGY

### Sources Used
1. **Government Sources**
   - US Commerce Dept Entity List
   - FCC Covered Equipment List  
   - US Treasury OFAC Sanctions
   - CIA World Factbook

2. **Leaked Documents**
   - Snowden Archives (NSA/ANT Catalog)
   - Hacking Team breach (2015)
   - FinFisher breach (2014)
   - WikiLeaks Vault 7

3. **Research Organizations**
   - Citizen Lab (University of Toronto)
   - Privacy International
   - Electronic Frontier Foundation
   - Access Now
   - IPVM (surveillance research)

4. **News Sources**
   - The Intercept
   - Der Spiegel
   - The Guardian
   - Reuters
   - Intelligence Online

5. **Technical Sources**
   - IEEE OUI Registry
   - Wireshark OUI database
   - Security researcher publications

---

## DISCLAIMER

This database is compiled from publicly available sources for security research and defensive purposes only. The inclusion of a vendor does not imply any wrongdoing. Many listed organizations provide legitimate products and services. This information should be used responsibly for:
- Personal privacy protection
- Security research
- Authorized penetration testing
- Academic study

Unauthorized surveillance, hacking, or privacy violations are illegal.

---

## VERSION
- Version: 1.0.0
- Date: 2026-03-30
- Compiled by: SIGINT-Deck Project
