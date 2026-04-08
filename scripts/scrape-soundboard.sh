#!/bin/bash
# Scrape free sound effects for the SIGINT-Pi soundboard
# Sources: freesound.org (via API), BBC Sound Effects, and procedurally generated tones
# Usage: ./scrape-soundboard.sh [output_dir]

set -e

OUTPUT_DIR="${1:-./soundboard/clips}"
mkdir -p "$OUTPUT_DIR"

echo "=== SIGINT-Pi Soundboard File Scraper ==="
echo "Output directory: $OUTPUT_DIR"
echo ""

# Check for required tools
for cmd in sox wget ffmpeg; do
    if ! command -v $cmd &>/dev/null; then
        echo "[WARN] $cmd not found. Install with: sudo apt install $cmd (or brew install $cmd)"
    fi
done

# ============================================
# Part 1: Generate procedural alert tones
# (Works offline, no downloads needed)
# ============================================
generate_tones() {
    echo "--- Generating procedural alert tones ---"

    if ! command -v sox &>/dev/null; then
        echo "[SKIP] sox not installed, skipping procedural tones"
        return
    fi

    # Drone detected - aggressive square wave chirp
    sox -n "$OUTPUT_DIR/alert_drone_detected.wav" \
        synth 0.2 square 660 synth 0.2 square 880 synth 0.2 square 660 \
        gain -6 2>/dev/null && echo "  [OK] alert_drone_detected.wav"

    # Tracker detected - triangle wave pulse
    sox -n "$OUTPUT_DIR/alert_tracker_detected.wav" \
        synth 0.15 triangle 440 synth 0.15 triangle 550 synth 0.15 triangle 440 \
        gain -6 2>/dev/null && echo "  [OK] alert_tracker_detected.wav"

    # Critical alert - sawtooth double-pulse
    sox -n "$OUTPUT_DIR/alert_critical.wav" \
        synth 0.15 sawtooth 880 synth 0.1 sawtooth 440 synth 0.15 sawtooth 880 synth 0.1 sawtooth 440 \
        gain -6 2>/dev/null && echo "  [OK] alert_critical.wav"

    # High priority - sine sweep
    sox -n "$OUTPUT_DIR/alert_high.wav" \
        synth 0.3 sine 660-880 \
        gain -6 2>/dev/null && echo "  [OK] alert_high.wav"

    # IMSI catcher siren - warbling siren
    sox -n "$OUTPUT_DIR/alert_imsi_catcher.wav" \
        synth 0.3 sine 600-900 synth 0.3 sine 900-600 synth 0.3 sine 600-900 \
        gain -3 2>/dev/null && echo "  [OK] alert_imsi_catcher.wav"

    # New device - short ping
    sox -n "$OUTPUT_DIR/alert_new_device.wav" \
        synth 0.1 sine 1200 synth 0.05 sine 1800 \
        fade 0 0.15 0.05 gain -8 2>/dev/null && echo "  [OK] alert_new_device.wav"

    # WiFi deauth attack - rapid beeping
    sox -n "$OUTPUT_DIR/alert_deauth_attack.wav" \
        synth 0.05 square 1000 synth 0.05 silence 1 0.05 0% \
        synth 0.05 square 1000 synth 0.05 silence 1 0.05 0% \
        synth 0.05 square 1000 synth 0.05 silence 1 0.05 0% \
        synth 0.1 square 800 \
        gain -6 2>/dev/null && echo "  [OK] alert_deauth_attack.wav"

    # Geofence breach - low menacing tone
    sox -n "$OUTPUT_DIR/alert_geofence.wav" \
        synth 0.5 sine 200 synth 0.3 sine 150 \
        gain -3 2>/dev/null && echo "  [OK] alert_geofence.wav"

    # Scan complete - pleasant chime
    sox -n "$OUTPUT_DIR/notify_scan_complete.wav" \
        synth 0.1 sine 880 synth 0.1 sine 1100 synth 0.2 sine 1320 \
        fade 0 0.4 0.1 gain -8 2>/dev/null && echo "  [OK] notify_scan_complete.wav"

    # Monitor mode restored
    sox -n "$OUTPUT_DIR/notify_monitor_restored.wav" \
        synth 0.15 sine 440 synth 0.15 sine 660 synth 0.15 sine 880 \
        fade 0 0.45 0.1 gain -8 2>/dev/null && echo "  [OK] notify_monitor_restored.wav"

    # TSCM threat - ominous descending
    sox -n "$OUTPUT_DIR/alert_tscm_threat.wav" \
        synth 0.2 sawtooth 1200 synth 0.2 sawtooth 800 synth 0.3 sawtooth 400 \
        gain -3 2>/dev/null && echo "  [OK] alert_tscm_threat.wav"

    # Radio static burst (for soundboard fun)
    sox -n "$OUTPUT_DIR/sfx_radio_static.wav" \
        synth 1.0 whitenoise band -n 2000 1000 \
        gain -10 2>/dev/null && echo "  [OK] sfx_radio_static.wav"

    # Walkie-talkie roger beep
    sox -n "$OUTPUT_DIR/sfx_roger_beep.wav" \
        synth 0.2 sine 1500 \
        fade 0 0.2 0.05 gain -6 2>/dev/null && echo "  [OK] sfx_roger_beep.wav"

    # NATO phonetic click (for PTT)
    sox -n "$OUTPUT_DIR/sfx_ptt_click.wav" \
        synth 0.03 whitenoise \
        gain -3 2>/dev/null && echo "  [OK] sfx_ptt_click.wav"

    echo ""
}

# ============================================
# Part 2: Download free sound effects
# (From public domain / CC0 sources)
# ============================================
download_free_sounds() {
    echo "--- Downloading free sound effects ---"

    if ! command -v wget &>/dev/null && ! command -v curl &>/dev/null; then
        echo "[SKIP] Neither wget nor curl available"
        return
    fi

    DL_CMD="wget -q -O"
    if ! command -v wget &>/dev/null; then
        DL_CMD="curl -sL -o"
    fi

    # BBC Sound Effects (public domain for personal/educational use)
    # These are example URLs - the BBC Sound Effects site allows downloads
    # but requires their terms. Using soundbible.com public domain instead.

    # Public domain sounds from soundbible.com / similar
    declare -A SOUNDS
    SOUNDS[sfx_air_horn]="https://soundbible.com/grab.php?id=2178&type=wav"
    SOUNDS[sfx_alarm_clock]="https://soundbible.com/grab.php?id=2197&type=wav"

    for name in "${!SOUNDS[@]}"; do
        local url="${SOUNDS[$name]}"
        local dest="$OUTPUT_DIR/${name}.wav"
        if [ -f "$dest" ]; then
            echo "  [EXISTS] ${name}.wav"
            continue
        fi
        echo -n "  Downloading ${name}.wav... "
        if $DL_CMD "$dest" "$url" 2>/dev/null; then
            # Verify it's actually a WAV
            if file "$dest" | grep -q "WAVE\|RIFF"; then
                echo "[OK]"
            else
                echo "[FAIL - not WAV, removing]"
                rm -f "$dest"
            fi
        else
            echo "[FAIL]"
            rm -f "$dest"
        fi
    done

    echo ""
}

# ============================================
# Part 3: Convert any MP3/OGG to WAV for aplay
# ============================================
convert_to_wav() {
    echo "--- Converting non-WAV files to WAV ---"

    if ! command -v ffmpeg &>/dev/null; then
        echo "[SKIP] ffmpeg not installed"
        return
    fi

    for f in "$OUTPUT_DIR"/*.mp3 "$OUTPUT_DIR"/*.ogg "$OUTPUT_DIR"/*.flac; do
        [ -f "$f" ] || continue
        local base="${f%.*}"
        local dest="${base}.wav"
        if [ -f "$dest" ]; then
            echo "  [EXISTS] $(basename "$dest")"
            continue
        fi
        echo -n "  Converting $(basename "$f") → WAV... "
        if ffmpeg -y -i "$f" -acodec pcm_s16le -ar 48000 -ac 1 "$dest" 2>/dev/null; then
            echo "[OK]"
        else
            echo "[FAIL]"
        fi
    done

    echo ""
}

# ============================================
# Part 4: Generate FRS/MURS channel test tones
# (For testing radio TX with identifiable signals)
# ============================================
generate_channel_tones() {
    echo "--- Generating channel ID test tones ---"

    if ! command -v sox &>/dev/null; then
        echo "[SKIP] sox not installed"
        return
    fi

    # CTCSS tones (PL tones) used on FRS/MURS
    # These are standard sub-audible tones for channel privacy
    declare -A CTCSS
    CTCSS[67.0]=1
    CTCSS[71.9]=2
    CTCSS[74.4]=3
    CTCSS[77.0]=4
    CTCSS[79.7]=5
    CTCSS[82.5]=6
    CTCSS[85.4]=7
    CTCSS[88.5]=8
    CTCSS[91.5]=9
    CTCSS[94.8]=10
    CTCSS[100.0]=12
    CTCSS[103.5]=13
    CTCSS[107.2]=14
    CTCSS[110.9]=15
    CTCSS[114.8]=16

    for freq in "${!CTCSS[@]}"; do
        local code="${CTCSS[$freq]}"
        local dest="$OUTPUT_DIR/ctcss_${freq}hz_code${code}.wav"
        if [ -f "$dest" ]; then continue; fi
        sox -n "$dest" synth 3.0 sine "$freq" gain -12 2>/dev/null
    done
    echo "  [OK] Generated ${#CTCSS[@]} CTCSS tone files"

    # 1 kHz test tone (standard radio test)
    sox -n "$OUTPUT_DIR/test_tone_1khz.wav" synth 3.0 sine 1000 gain -6 2>/dev/null
    echo "  [OK] test_tone_1khz.wav"

    # Channel ID announcement tones (beep + number)
    for ch in 1 2 3 4 5 6 7 8; do
        local freq=$((800 + ch * 100))
        sox -n "$OUTPUT_DIR/channel_id_frs${ch}.wav" \
            synth 0.1 sine 1500 synth 0.5 sine "$freq" fade 0 0.6 0.1 gain -6 2>/dev/null
    done
    echo "  [OK] Generated FRS channel ID tones"

    echo ""
}

# Run all generators
generate_tones
download_free_sounds
convert_to_wav
generate_channel_tones

# Summary
echo "=== Summary ==="
TOTAL=$(find "$OUTPUT_DIR" -type f \( -name "*.wav" -o -name "*.mp3" -o -name "*.ogg" \) | wc -l | tr -d ' ')
SIZE=$(du -sh "$OUTPUT_DIR" 2>/dev/null | cut -f1)
echo "Total clips: $TOTAL"
echo "Total size: $SIZE"
echo "Directory: $OUTPUT_DIR"
echo ""
echo "To deploy to Pi:  scp $OUTPUT_DIR/*.wav pi@<pi-ip>:/home/pi/sigint-pi/soundboard/clips/"
echo "To deploy to Deck: scp $OUTPUT_DIR/*.wav deck@<deck-ip>:/home/deck/sigint-deck/soundboard/clips/"
