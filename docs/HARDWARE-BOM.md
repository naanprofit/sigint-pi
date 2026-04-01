# SIGINT-DECK Hardware Bill of Materials (BOM)

Comprehensive hardware configurations from ultra-budget to professional setups.

**Last Updated:** 2024-03 | **Prices:** Approximate USD, may vary

---

## Table of Contents
1. [Platform Overview](#platform-overview)
2. [Tier 1: Ultra-Budget ($50-100)](#tier-1-ultra-budget-50-100)
3. [Tier 2: Budget ($100-250)](#tier-2-budget-100-250)
4. [Tier 3: Capable ($250-500)](#tier-3-capable-250-500)
5. [Tier 4: Professional ($500-1500)](#tier-4-professional-500-1500)
6. [SDR Hardware Options](#sdr-hardware-options)
7. [WiFi Adapters (Monitor Mode)](#wifi-adapters-monitor-mode)
8. [Accessories & Peripherals](#accessories--peripherals)
9. [Complete Build Examples](#complete-build-examples)

---

## Platform Overview

| Platform | Pros | Cons | Best For |
|----------|------|------|----------|
| **Android** | Portable, built-in WiFi/BT/GPS, touchscreen | Limited SDR support, app restrictions | Mobile monitoring, WiFi scanning |
| **Raspberry Pi** | Low power, GPIO, good community | ARM compilation, limited USB bandwidth | Portable stations, embedded |
| **x86 Mini PC** | Full Linux compatibility, USB3, power | Larger, higher power draw | Base stations, heavy processing |
| **Steam Deck** | Powerful, portable, gaming controls | Expensive, read-only FS | Mobile SIGINT workstation |
| **Mac (Apple Silicon)** | Excellent battery, Unix-based | Expensive, some driver issues | Development, analysis |

---

## Tier 1: Ultra-Budget ($50-100)

### Option 1A: Raspberry Pi Zero 2 W
**Total: ~$45-60**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| SBC | Raspberry Pi Zero 2 W | $15 | Quad-core ARM Cortex-A53, 512MB RAM |
| Storage | 32GB microSD (Class 10) | $8 | Samsung EVO or SanDisk recommended |
| Power | 5V 2.5A USB-C adapter | $8 | Official or quality brand |
| Case | Basic case | $5 | Or 3D print |
| USB Hub | OTG hub (4-port) | $8 | For WiFi adapter + SDR |
| **Total** | | **~$44** | |

**Capabilities:** Basic WiFi scanning, Bluetooth monitoring, lightweight SDR (RTL-SDR)
**Limitations:** Single USB (via hub), 512MB RAM limits heavy processing

### Option 1B: Orange Pi Zero 3 (1GB)
**Total: ~$35-50**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| SBC | Orange Pi Zero 3 (1GB) | $18 | H618 quad-core, 1GB RAM, onboard WiFi |
| Storage | 32GB microSD | $8 | |
| Power | 5V 3A USB-C | $8 | |
| Case | Acrylic case | $5 | |
| **Total** | | **~$39** | |

**Capabilities:** Similar to Pi Zero 2 W but more RAM
**Limitations:** Smaller community, some driver issues

### Option 1C: Budget Android Phone (Used)
**Total: ~$30-60**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Phone | Used Pixel 3a / Moto G | $30-50 | Ensure bootloader unlockable |
| OTG Cable | USB-C OTG | $5 | For external adapters |
| **Total** | | **~$35-55** | |

**Capabilities:** WiFi scanning (apps), GPS, cellular, portable
**Limitations:** No monitor mode without root, limited SDR support

---

## Tier 2: Budget ($100-250)

### Option 2A: Raspberry Pi 4 (4GB) Full Kit
**Total: ~$120-150**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| SBC | Raspberry Pi 4 Model B (4GB) | $55 | Quad-core 1.8GHz, 4GB RAM |
| Storage | 64GB microSD | $12 | High endurance recommended |
| Power | Official 5V 3A USB-C | $10 | |
| Case | Argon ONE M.2 | $35 | Optional: adds M.2 SSD slot |
| Cooling | Heatsink + fan | $8 | Included in many cases |
| **Total** | | **~$120** | |

**Add-ons for SIGINT:**
| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| WiFi Adapter | Alfa AWUS036ACH | $50 | Dual-band, monitor mode |
| RTL-SDR | RTL-SDR Blog V4 | $35 | Best budget SDR |
| **Total w/ RF** | | **~$205** | |

### Option 2B: Mini PC (Intel N100)
**Total: ~$150-200**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Mini PC | Beelink Mini S12 Pro | $150 | N100, 8GB RAM, 256GB SSD |
| **or** | TRIGKEY G4 | $140 | N95, 8GB RAM, 256GB SSD |
| **or** | GMKtec G3 | $130 | N100, 8GB RAM, 128GB SSD |
| **Total** | | **~$130-150** | Runs any Linux distro |

**Specs typical for N100 mini PCs:**
- CPU: Intel N100 (4-core, 3.4GHz boost)
- RAM: 8-16GB DDR4
- Storage: 256-512GB NVMe
- Ports: 2x USB 3.0, 2x USB 2.0, HDMI, Ethernet
- Power: 15-25W TDP

### Option 2C: Orange Pi 5 (8GB)
**Total: ~$100-130**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| SBC | Orange Pi 5 (8GB) | $90 | RK3588S, 8GB RAM, M.2 slot |
| Storage | 128GB NVMe + 32GB SD | $25 | NVMe for speed |
| Power | 5V 4A USB-C | $10 | |
| Case | Metal case | $15 | With heatsink |
| **Total** | | **~$140** | |

**Capabilities:** Near-desktop performance, 8GB RAM handles heavy workloads

---

## Tier 3: Capable ($250-500)

### Option 3A: Steam Deck (Used/Refurbished)
**Total: ~$280-400**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Device | Steam Deck 64GB (used) | $250-300 | Or 256GB for $350 |
| Storage | 512GB microSD | $40 | For data/tools |
| USB-C Hub | 7-in-1 hub | $25 | HDMI, USB3, Ethernet |
| **Total** | | **~$315-365** | |

**Specs:**
- CPU: AMD Zen 2 4-core (2.4-3.5GHz)
- GPU: AMD RDNA 2 (8 CUs)
- RAM: 16GB LPDDR5
- Display: 7" 1280x800 touchscreen
- Battery: 40Wh (2-8 hours)

### Option 3B: Ryzen Mini PC
**Total: ~$300-400**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Mini PC | Beelink SER5 | $300 | Ryzen 5 5560U, 16GB, 500GB |
| **or** | MINISFORUM UM560 | $320 | Ryzen 5 5600H, 16GB, 512GB |
| **or** | GMKtec M5 Plus | $350 | Ryzen 7 5800H, 32GB, 512GB |
| **Total** | | **~$300-350** | |

### Option 3C: Mac Mini M1 (Used)
**Total: ~$350-450**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Computer | Mac Mini M1 (used) | $350-400 | 8GB/256GB base model |
| Adapter | USB-C hub | $30 | For additional ports |
| **Total** | | **~$380-430** | |

**Note:** Excellent for development, some WiFi adapter compatibility issues

---

## Tier 4: Professional ($500-1500)

### Option 4A: High-Performance Mini PC
**Total: ~$600-900**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Mini PC | MINISFORUM UM790 Pro | $650 | Ryzen 9 7940HS, 32GB, 1TB |
| **or** | Beelink GTR7 | $700 | Ryzen 9 7940HS |
| **or** | Intel NUC 13 Pro | $800 | i7-1360P, 32GB, 1TB |
| **Total** | | **~$650-800** | |

### Option 4B: Mac Mini M2 Pro
**Total: ~$1000-1300**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Computer | Mac Mini M2 Pro | $1000 | 16GB/512GB |
| Adapter | CalDigit hub | $100 | Premium dock |
| **Total** | | **~$1100** | |

### Option 4C: Framework Laptop 16
**Total: ~$1400-1800**

| Component | Model | Price | Notes |
|-----------|-------|-------|-------|
| Laptop | Framework 16 | $1400 | AMD 7840HS, 16GB, 512GB |
| Expansion | Input modules | $100 | LED matrix, numpad, etc. |
| **Total** | | **~$1500** | Modular, repairable |

---

## SDR Hardware Options

### Receive-Only SDRs

| Model | Frequency Range | Bandwidth | Price | Notes |
|-------|-----------------|-----------|-------|-------|
| **RTL-SDR Blog V3** | 500 kHz - 1.7 GHz | 2.4 MHz | $30 | Best budget option |
| **RTL-SDR Blog V4** | 500 kHz - 1.7 GHz | 2.4 MHz | $40 | Improved filtering |
| **Nooelec NESDR SMArt** | 25 MHz - 1.7 GHz | 2.4 MHz | $25 | Good quality clone |
| **Airspy Mini** | 24 - 1700 MHz | 6 MHz | $99 | Better dynamic range |
| **Airspy R2** | 24 - 1700 MHz | 10 MHz | $169 | Professional grade |
| **SDRplay RSPdx** | 1 kHz - 2 GHz | 10 MHz | $250 | Wide frequency range |

### Transmit-Capable SDRs

| Model | Frequency Range | Bandwidth | TX Power | Price | Notes |
|-------|-----------------|-----------|----------|-------|-------|
| **HackRF One** | 1 MHz - 6 GHz | 20 MHz | 0-15 dBm | $300 | Open source, versatile |
| **YARD Stick One** | 300-928 MHz | N/A | +10 dBm | $100 | Sub-GHz only |
| **LimeSDR Mini 2.0** | 10 MHz - 3.5 GHz | 40 MHz | 0-10 dBm | $250 | Full duplex |
| **Pluto SDR** | 325 MHz - 3.8 GHz | 20 MHz | +7 dBm | $150 | ADALM-PLUTO |
| **USRP B200** | 70 MHz - 6 GHz | 56 MHz | +10 dBm | $1100 | Professional |

### SDR Accessories

| Accessory | Price | Notes |
|-----------|-------|-------|
| RTL-SDR dipole antenna kit | $10 | Basic antennas |
| Wideband discone antenna | $30-50 | 25-1300 MHz |
| ADS-B antenna (1090 MHz) | $15 | For aircraft tracking |
| LNA (low noise amplifier) | $20-50 | Improves weak signal reception |
| Band-pass filters | $15-30 | Reduce interference |
| SMA cable + adapters | $10-20 | Various connectors |

---

## WiFi Adapters (Monitor Mode)

### Recommended Adapters

| Model | Chipset | Bands | Price | Monitor Mode | Packet Injection | Notes |
|-------|---------|-------|-------|--------------|------------------|-------|
| **Alfa AWUS036ACH** | RTL8812AU | 2.4/5 GHz | $50 | Yes | Yes | Gold standard |
| **Alfa AWUS036ACM** | MT7612U | 2.4/5 GHz | $45 | Yes | Yes | Excellent Linux support |
| **Alfa AWUS036AXML** | MT7921AU | 2.4/5/6 GHz | $55 | Yes | Yes | WiFi 6E support |
| **Alfa AWUS036NHA** | AR9271 | 2.4 GHz | $25 | Yes | Yes | Budget single-band |
| **Panda PAU09** | RT5572 | 2.4/5 GHz | $20 | Yes | Yes | Budget dual-band |
| **TP-Link Archer T3U Plus** | RTL8812BU | 2.4/5 GHz | $20 | Partial | Partial | Needs driver work |

### Chipset Compatibility Quick Reference

| Chipset | Linux Support | Monitor Mode | Recommended |
|---------|---------------|--------------|-------------|
| Atheros AR9271 | Excellent | Native | Yes |
| Ralink RT5572 | Excellent | Native | Yes |
| MediaTek MT7612U | Excellent | Native | Yes |
| MediaTek MT7921AU | Good | Native | Yes |
| Realtek RTL8812AU | Good | With drivers | Yes |
| Realtek RTL8812BU | Moderate | With drivers | Maybe |
| Intel AX200/AX210 | Good | Limited | No (integrated only) |

---

## Accessories & Peripherals

### Power & Batteries

| Item | Specs | Price | Notes |
|------|-------|-------|-------|
| USB-C PD Power Bank (65W) | 20000mAh | $40-60 | Powers Pi, mini PC |
| USB-C PD Power Bank (100W) | 26800mAh | $80-100 | Powers Steam Deck |
| 12V LiFePO4 battery | 6Ah | $50 | For field operations |
| Solar panel (foldable) | 30W USB-C | $50-80 | Off-grid charging |

### Storage

| Type | Capacity | Price | Notes |
|------|----------|-------|-------|
| microSD (high endurance) | 128GB | $15-20 | Samsung PRO Endurance |
| microSD (high endurance) | 256GB | $30-40 | For extended logging |
| USB 3.0 flash drive | 128GB | $15 | Fast data transfer |
| Portable SSD | 1TB | $80-100 | Samsung T7 / WD |
| NVMe SSD | 512GB | $40-50 | For mini PCs |

### Antennas

| Type | Frequency | Price | Notes |
|------|-----------|-------|-------|
| Rubber duck (SMA) | Wideband | $5-10 | Basic included antenna |
| Telescopic whip | 50-1000 MHz | $15 | Adjustable length |
| Discone (outdoor) | 25-1300 MHz | $50-80 | Wideband vertical |
| Yagi (directional) | Various | $30-60 | High gain, directional |
| Magnetic mount mobile | VHF/UHF | $20-30 | Vehicle mounting |
| Patch antenna | 2.4/5 GHz | $20 | Directional WiFi |

### Cases & Enclosures

| Type | Price | Notes |
|------|-------|-------|
| Pelican 1150 | $40 | Waterproof, foam insert |
| Apache 1800 (Harbor Freight) | $15 | Budget Pelican alternative |
| Ammo can | $15-20 | Rugged, RF shielding |
| 3D printed enclosure | $5-20 | Custom fit |
| Faraday bag | $15-30 | RF isolation for evidence |

---

## Complete Build Examples

### Build 1: "Pocket SIGINT" - Ultra-Portable ($80)

```
┌─────────────────────────────────────┐
│  POCKET SIGINT - $80 TOTAL         │
├─────────────────────────────────────┤
│  Pi Zero 2 W ................ $15  │
│  32GB microSD ............... $8   │
│  USB OTG Hub ................ $8   │
│  RTL-SDR V3 ................. $30  │
│  Telescopic antenna ......... $10  │
│  Power bank (10000mAh) ...... $15  │
│  Small case ................. $5   │
└─────────────────────────────────────┘
Capabilities: Basic SDR, ADS-B, FM, weather sat
Limitations: No WiFi monitor mode, limited processing
```

### Build 2: "Field Kit" - Portable Full-Featured ($250)

```
┌─────────────────────────────────────┐
│  FIELD KIT - $250 TOTAL            │
├─────────────────────────────────────┤
│  Raspberry Pi 4 (4GB) ....... $55  │
│  64GB microSD ............... $12  │
│  Argon ONE case ............. $25  │
│  Alfa AWUS036ACM ............ $45  │
│  RTL-SDR Blog V4 ............ $40  │
│  Antenna kit ................ $20  │
│  USB-C PD power bank ........ $40  │
│  Cables & adapters .......... $15  │
└─────────────────────────────────────┘
Capabilities: WiFi scanning, SDR, Bluetooth, full Linux
```

### Build 3: "Mobile Workstation" - Steam Deck ($450)

```
┌─────────────────────────────────────┐
│  MOBILE WORKSTATION - $450 TOTAL   │
├─────────────────────────────────────┤
│  Steam Deck 64GB (used) ..... $280 │
│  512GB microSD .............. $40  │
│  USB-C Hub (7-port) ......... $25  │
│  Alfa AWUS036ACH ............ $50  │
│  RTL-SDR Blog V4 ............ $40  │
│  Carrying case .............. $15  │
└─────────────────────────────────────┘
Capabilities: Full desktop Linux, touchscreen, gaming mode
```

### Build 4: "Base Station" - Fixed High-Performance ($600)

```
┌─────────────────────────────────────┐
│  BASE STATION - $600 TOTAL         │
├─────────────────────────────────────┤
│  Mini PC (N100/16GB) ........ $180 │
│  1TB NVMe SSD ............... $60  │
│  Alfa AWUS036AXML (WiFi 6E).. $55  │
│  HackRF One ................. $300 │
│  Discone antenna ............ $60  │
│  UPS (small) ................ $50  │
└─────────────────────────────────────┘
Capabilities: Full TX/RX, 24/7 operation, WiFi 6E
```

### Build 5: "Professional Kit" - Maximum Capability ($1500)

```
┌─────────────────────────────────────┐
│  PROFESSIONAL KIT - $1500 TOTAL    │
├─────────────────────────────────────┤
│  Mini PC (Ryzen 9) .......... $700 │
│  Alfa AWUS036AXML ........... $55  │
│  HackRF One + PortaPack ..... $450 │
│  LimeSDR Mini 2.0 ........... $250 │
│  Antenna selection .......... $150 │
│  Pelican case ............... $100 │
│  Accessories ................ $50  │
└─────────────────────────────────────┘
Capabilities: Full professional SIGINT/TSCM operations
```

---

## Minimum Viable Configurations

### Absolute Minimum - WiFi Only ($50)

```
Used Android phone (Pixel 3) .... $40
OTG Cable ....................... $5
WiFi Analyzer app ............... Free
```

### Minimum SDR Setup ($45)

```
RTL-SDR Blog V3 ................. $30
Telescopic antenna .............. $10
Laptop (existing) ............... $0
USB extension cable ............. $5
```

### Minimum Linux SIGINT ($100)

```
Raspberry Pi 4 (2GB) ............ $45
32GB microSD .................... $8
Power supply .................... $10
Case ............................ $8
Alfa AWUS036NHA (2.4GHz) ........ $25
```

---

## Notes

1. **Prices are approximate** and fluctuate. Check current prices before purchasing.
2. **Used/refurbished** equipment can significantly reduce costs.
3. **Amazon, AliExpress, eBay** have different price points - shop around.
4. **Antenna quality matters** - a $5 antenna upgrade often beats a $50 SDR upgrade.
5. **Power requirements** - ensure adequate power supply for USB peripherals.
6. **Legal compliance** - transmit-capable SDRs require appropriate licenses.

---

## Vendor Links (Reference Only)

- RTL-SDR: https://www.rtl-sdr.com/buy-rtl-sdr-dvb-t-dongles/
- Alfa Network: https://www.alfa.com.tw/
- Raspberry Pi: https://www.raspberrypi.com/
- Orange Pi: http://www.orangepi.org/
- HackRF: https://greatscottgadgets.com/hackrf/
- LimeSDR: https://limemicro.com/
