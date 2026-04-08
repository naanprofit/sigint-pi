# TODO: ML Integration & Military Drone Detection Improvements

## Context

The current codebase has **no machine learning** — zero ML crates in `Cargo.toml`. What is described as "anomaly detection" and "device fingerprinting" in the README is entirely rule-based: hardcoded thresholds, harmonic counting, and SQLite behavioral logs. The "AI/LLM integration" is an HTTP forwarding client to an external API, not on-device inference. Military drone detection is a static frequency lookup table with simple power thresholding — no waveform or modulation analysis.

The hardware stack (RTL-SDR, HackRF One, WiFi monitor mode, BLE) is tested and functional. This creates a real data collection opportunity. The path forward is: **capture labeled IQ data with existing hardware → train models in Python → export to ONNX → run inference in Rust via `ort` crate**.

---

## TODO Items

### 1. IQ Signal Classifier (Highest Priority)

**Goal:** Classify modulation type from raw IQ samples — OOK, FSK, GFSK, LoRa, OFDM, AM, FM, and drone-specific waveforms (OcuSync, DJI Lightbridge, ExpressLRS).

**Why:** Power thresholding in a known frequency band cannot distinguish a drone controller from a WiFi router sharing the same band. Modulation classification can.

**Implementation:**
- Train a 1D CNN or ResNet on IQ float32 windows (e.g. 1024 or 2048 samples) using the RadioML 2018 dataset as a base, fine-tuned on captures from HackRF
- Export trained model to ONNX
- Add `ort` crate to `Cargo.toml` for ONNX Runtime inference in the existing Rust service
- Wire into the existing SDR scan pipeline — classify each signal candidate before emitting a detection event

**Pi Zero 2W constraint:** Use quantized INT8 models, target <10MB model size. Inference on a 100ms IQ window should run at several Hz, which is sufficient for this use case. No constraint on Steam Deck.

**Data needed:** HackRF captures of known drone control links (DJI OcuSync, ExpressLRS 2.4G, TBS Crossfire 915 MHz) in your environment, labeled by protocol.

---

### 2. Autoencoder Anomaly Detection on Spectrum (Replaces Threshold Heuristics)

**Goal:** Replace the current `anomaly_threshold = 0.7` rule-based scoring with a learned baseline model of normal RF at your location.

### 3. EMI/ESC Harmonic Classifier

**Goal:** Replace the current "minimum 3 harmonics at exact integer multiples" rule with a classifier trained on FFT magnitude spectra from actual RTL-SDR direct-sampling captures.

### 4. WiFi/BLE Device Fingerprinting (Replace Rule-Based)

**Goal:** Train a proper classifier on inter-packet timing distributions, probe request sequences, and 802.11 IE tag patterns to fingerprint device types.

### 5. Military Drone RF Detection (Waveform Analysis)

**Goal:** Go beyond frequency band presence to actual waveform characterization for military drone datalinks.

---

## Dependency Changes Required

Add to `Cargo.toml`:

```toml
ort = { version = "2", features = ["copy-dylibs"] }
rustfft = "6"
```

## Data Collection Priority Order

1. **EMI captures** — easiest, can do indoors with drone on bench
2. **WiFi PCAP** — already implemented, just needs labeling
3. **IQ signal captures** — HackRF raw captures of known protocols
4. **Baseline spectrum** — leave RTL-SDR running overnight with rtl_power
5. **Military waveforms** — deprioritize; labeled data is hard to obtain
