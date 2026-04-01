#!/bin/bash
# HackRF RF Monitor Script
# Monitors various frequency bands for interesting signals
#
# Usage: sudo ./hackrf-monitor.sh [mode]
# Modes: drone, ism, cellular, sweep, all

MODE="${1:-all}"
OUTPUT_DIR="/tmp/hackrf"
mkdir -p "$OUTPUT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_alert() { echo -e "${RED}[ALERT]${NC} $1"; }

check_hackrf() {
    if ! hackrf_info > /dev/null 2>&1; then
        echo "HackRF not found. Check USB connection."
        exit 1
    fi
    log_info "HackRF detected"
}

# Monitor 2.4GHz for drones (DJI, Parrot, etc.)
monitor_drone_24() {
    log_info "Monitoring 2.4 GHz for drone signals..."
    hackrf_transfer -r "$OUTPUT_DIR/drone_24.bin" \
        -f 2437000000 \
        -s 10000000 \
        -a 1 -l 32 -g 32 \
        -n 50000000 2>&1 | while read line; do
        if [[ "$line" == *"average power"* ]]; then
            power=$(echo "$line" | grep -oP '\-[\d.]+(?= dBfs)')
            if (( $(echo "$power > -30" | bc -l) )); then
                log_alert "Strong 2.4GHz signal detected: $power dBfs - possible drone!"
            fi
        fi
    done
}

# Monitor 5.8GHz for drone video links
monitor_drone_58() {
    log_info "Monitoring 5.8 GHz for drone video..."
    hackrf_transfer -r "$OUTPUT_DIR/drone_58.bin" \
        -f 5800000000 \
        -s 10000000 \
        -a 1 -l 32 -g 32 \
        -n 50000000 2>&1 | while read line; do
        if [[ "$line" == *"average power"* ]]; then
            power=$(echo "$line" | grep -oP '\-[\d.]+(?= dBfs)')
            if (( $(echo "$power > -35" | bc -l) )); then
                log_alert "Strong 5.8GHz signal: $power dBfs - possible drone video!"
            fi
        fi
    done
}

# Monitor 433 MHz ISM band (key fobs, sensors, some trackers)
monitor_ism_433() {
    log_info "Monitoring 433 MHz ISM band..."
    
    # Use rtl_433 if available (better decoding)
    if command -v rtl_433 &> /dev/null; then
        rtl_433 -f 433.92M -F json 2>/dev/null | while read line; do
            model=$(echo "$line" | jq -r '.model // empty' 2>/dev/null)
            if [ -n "$model" ]; then
                log_info "433 MHz device: $model"
                echo "$line" >> "$OUTPUT_DIR/ism_433.json"
            fi
        done
    else
        # Fallback to raw capture
        hackrf_transfer -r "$OUTPUT_DIR/ism_433.bin" \
            -f 433920000 \
            -s 2000000 \
            -n 10000000
    fi
}

# Monitor 915 MHz (US LoRa, smart meters)
monitor_ism_915() {
    log_info "Monitoring 915 MHz band..."
    hackrf_transfer -r "$OUTPUT_DIR/ism_915.bin" \
        -f 915000000 \
        -s 2000000 \
        -a 1 -l 24 -g 24 \
        -n 10000000 2>&1
}

# Cellular tower scan using kalibrate
scan_cellular() {
    log_info "Scanning for cellular towers..."
    
    if ! command -v kal &> /dev/null && ! command -v kalibrate-rtl &> /dev/null; then
        log_warn "kalibrate-rtl not installed, skipping cellular scan"
        return
    fi
    
    KAL_CMD="kal"
    command -v kalibrate-rtl &> /dev/null && KAL_CMD="kalibrate-rtl"
    
    for band in GSM850 GSM900 DCS1800 PCS1900; do
        log_info "Scanning $band..."
        $KAL_CMD -s $band 2>&1 | tee "$OUTPUT_DIR/cellular_$band.txt" | \
            grep "chan:" | while read line; do
            log_info "Tower found: $line"
        done
    done
}

# Wideband spectrum sweep
sweep_spectrum() {
    local start=${1:-100}
    local end=${2:-6000}
    
    log_info "Sweeping $start - $end MHz..."
    hackrf_sweep -f $start:$end -w 1000000 -1 -N 1 2>&1 | \
        tee "$OUTPUT_DIR/sweep_${start}_${end}.csv"
}

# Bug sweep mode - scan common surveillance frequencies
sweep_bugs() {
    log_info "Scanning for potential surveillance devices..."
    
    # Common bug/transmitter frequencies
    frequencies=(
        "88:108"    # FM band (cheap transmitters)
        "140:174"   # VHF (wireless mics)
        "400:512"   # UHF (body wires, bugs)
        "900:930"   # GSM bugs
        "1200:1300" # Video transmitters
        "2400:2500" # WiFi cameras
        "5700:5900" # 5GHz cameras
    )
    
    for freq in "${frequencies[@]}"; do
        start=$(echo $freq | cut -d: -f1)
        end=$(echo $freq | cut -d: -f2)
        log_info "Scanning $start - $end MHz..."
        
        hackrf_sweep -f $start:$end -w 500000 -1 -N 1 2>&1 | \
            awk -F, 'NF>6 {
                for(i=7; i<=NF; i++) {
                    if($i > -40) {
                        print "STRONG SIGNAL: " $3/1000000 " MHz, " $i " dB"
                    }
                }
            }' | while read line; do
            if [ -n "$line" ]; then
                log_alert "$line"
            fi
        done
    done
}

# Main
check_hackrf

case "$MODE" in
    drone)
        log_info "=== Drone Detection Mode ==="
        monitor_drone_24
        ;;
    ism)
        log_info "=== ISM Band Monitor ==="
        monitor_ism_433
        ;;
    cellular)
        log_info "=== Cellular Tower Scan ==="
        scan_cellular
        ;;
    sweep)
        log_info "=== Wideband Spectrum Sweep ==="
        sweep_spectrum 100 6000
        ;;
    bugs)
        log_info "=== Bug Sweep Mode ==="
        sweep_bugs
        ;;
    all)
        log_info "=== Full RF Survey ==="
        monitor_ism_433 &
        sleep 10
        scan_cellular
        sweep_bugs
        ;;
    *)
        echo "Usage: $0 [drone|ism|cellular|sweep|bugs|all]"
        exit 1
        ;;
esac

log_info "Output saved to $OUTPUT_DIR"
