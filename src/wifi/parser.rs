use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiDevice {
    pub mac_address: String,
    pub rssi: i32,
    pub channel: u8,
    pub frame_type: FrameType,
    pub ssid: Option<String>,
    pub bssid: Option<String>,
    pub is_ap: bool,
    pub vendor: Option<String>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub probe_requests: Vec<String>,
    pub data_frames_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameType {
    Management,
    Control,
    Data,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeRequest {
    pub mac_address: String,
    pub ssid: String,
    pub rssi: i32,
    pub timestamp: DateTime<Utc>,
}

const RADIOTAP_MIN_LEN: usize = 8;
const IEEE80211_HEADER_MIN_LEN: usize = 24;

pub fn parse_wifi_frame(data: &[u8]) -> Option<WifiDevice> {
    if data.len() < RADIOTAP_MIN_LEN {
        return None;
    }

    // Parse radiotap header
    let radiotap_len = u16::from_le_bytes([data[2], data[3]]) as usize;
    if data.len() < radiotap_len + IEEE80211_HEADER_MIN_LEN {
        return None;
    }

    // Extract RSSI from radiotap (simplified - actual implementation would parse all fields)
    let rssi = extract_rssi(data, radiotap_len);

    // Parse 802.11 frame header
    let frame_start = radiotap_len;
    let frame_control = u16::from_le_bytes([data[frame_start], data[frame_start + 1]]);
    
    let frame_type = match (frame_control & 0x0C) >> 2 {
        0 => FrameType::Management,
        1 => FrameType::Control,
        2 => FrameType::Data,
        _ => FrameType::Unknown,
    };

    let frame_subtype = (frame_control & 0xF0) >> 4;

    // Extract MAC addresses based on frame type
    let (mac_address, bssid, is_ap) = extract_addresses(&data[frame_start..], frame_type)?;

    // Extract SSID for probe requests and beacons
    let ssid = if frame_type == FrameType::Management {
        match frame_subtype {
            0 | 4 | 5 | 8 => extract_ssid(&data[frame_start..]),
            _ => None,
        }
    } else {
        None
    };

    // Determine channel from radiotap or use 0
    let channel = extract_channel(data, radiotap_len).unwrap_or(0);

    let now = Utc::now();

    Some(WifiDevice {
        mac_address,
        rssi,
        channel,
        frame_type,
        ssid,
        bssid,
        is_ap: frame_subtype == 8, // Beacon frame
        vendor: None, // Will be filled by OUI lookup
        first_seen: now,
        last_seen: now,
        probe_requests: vec![],
        data_frames_count: 0,
    })
}

fn extract_rssi(data: &[u8], radiotap_len: usize) -> i32 {
    // Parse radiotap header to find signal strength field
    // Radiotap header format:
    // - Bytes 0-1: Version (0) and pad (0)
    // - Bytes 2-3: Header length (little-endian)
    // - Bytes 4-7: Present flags (little-endian)
    
    if data.len() < 8 || radiotap_len < 8 {
        return -80; // Default fallback
    }
    
    let present_flags = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    
    // Radiotap present flags bits:
    // Bit 5 (0x20): DBM Antenna Signal (i8)
    // Bit 10 (0x400): DBM Antenna Noise (i8)  
    // Bit 14 (0x4000): dB Antenna Signal (u8)
    
    let has_dbm_signal = (present_flags & 0x20) != 0;
    let has_tsft = (present_flags & 0x01) != 0;
    let has_flags = (present_flags & 0x02) != 0;
    let has_rate = (present_flags & 0x04) != 0;
    let has_channel = (present_flags & 0x08) != 0;
    let has_fhss = (present_flags & 0x10) != 0;
    
    if !has_dbm_signal {
        // Try alternate methods
        // Many adapters put signal at predictable offsets
        for offset in [14usize, 18, 22, 26, 30] {
            if offset < radiotap_len && offset < data.len() {
                let val = data[offset] as i8;
                // Valid RSSI is typically -20 to -100 dBm
                if val < 0 && val > -110 {
                    return val as i32;
                }
            }
        }
        return -80;
    }
    
    // Calculate offset to signal field based on present flags
    let mut offset = 8usize; // After fixed header
    
    // Check for extended present flags
    let mut check_ext = (present_flags & 0x80000000) != 0;
    while check_ext && offset + 4 <= data.len() {
        let ext_flags = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        offset += 4;
        check_ext = (ext_flags & 0x80000000) != 0;
    }
    
    // TSFT (8 bytes, 8-byte aligned)
    if has_tsft {
        offset = (offset + 7) & !7; // 8-byte alignment
        offset += 8;
    }
    
    // Flags (1 byte)
    if has_flags {
        offset += 1;
    }
    
    // Rate (1 byte)
    if has_rate {
        offset += 1;
    }
    
    // Channel (4 bytes, 2-byte aligned)
    if has_channel {
        offset = (offset + 1) & !1; // 2-byte alignment
        offset += 4;
    }
    
    // FHSS (2 bytes)
    if has_fhss {
        offset += 2;
    }
    
    // DBM Antenna Signal (1 byte, signed)
    if offset < data.len() && offset < radiotap_len {
        let rssi = data[offset] as i8;
        return rssi as i32;
    }
    
    -80 // Default fallback
}

fn extract_channel(data: &[u8], radiotap_len: usize) -> Option<u8> {
    // Parse radiotap to find channel field
    if data.len() < 8 || radiotap_len < 8 {
        return None;
    }
    
    let present_flags = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    
    let has_channel = (present_flags & 0x08) != 0;
    let has_tsft = (present_flags & 0x01) != 0;
    let has_flags = (present_flags & 0x02) != 0;
    let has_rate = (present_flags & 0x04) != 0;
    
    if !has_channel {
        return None;
    }
    
    // Calculate offset to channel field
    let mut offset = 8usize;
    
    // Check for extended present flags
    let mut check_ext = (present_flags & 0x80000000) != 0;
    while check_ext && offset + 4 <= data.len() {
        let ext_flags = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        offset += 4;
        check_ext = (ext_flags & 0x80000000) != 0;
    }
    
    // TSFT (8 bytes, 8-byte aligned)
    if has_tsft {
        offset = (offset + 7) & !7;
        offset += 8;
    }
    
    // Flags (1 byte)
    if has_flags {
        offset += 1;
    }
    
    // Rate (1 byte)
    if has_rate {
        offset += 1;
    }
    
    // Channel (2-byte aligned): 2 bytes frequency + 2 bytes flags
    offset = (offset + 1) & !1; // 2-byte alignment
    
    if offset + 2 <= data.len() && offset + 2 <= radiotap_len {
        let freq = u16::from_le_bytes([data[offset], data[offset + 1]]);
        Some(freq_to_channel(freq))
    } else {
        None
    }
}

fn freq_to_channel(freq: u16) -> u8 {
    match freq {
        2412 => 1, 2417 => 2, 2422 => 3, 2427 => 4, 2432 => 5,
        2437 => 6, 2442 => 7, 2447 => 8, 2452 => 9, 2457 => 10,
        2462 => 11, 2467 => 12, 2472 => 13, 2484 => 14,
        5180 => 36, 5200 => 40, 5220 => 44, 5240 => 48,
        5260 => 52, 5280 => 56, 5300 => 60, 5320 => 64,
        5500 => 100, 5520 => 104, 5540 => 108, 5560 => 112,
        5580 => 116, 5600 => 120, 5620 => 124, 5640 => 128,
        5660 => 132, 5680 => 136, 5700 => 140, 5720 => 144,
        5745 => 149, 5765 => 153, 5785 => 157, 5805 => 161, 5825 => 165,
        _ => 0,
    }
}

fn extract_addresses(frame: &[u8], frame_type: FrameType) -> Option<(String, Option<String>, bool)> {
    if frame.len() < IEEE80211_HEADER_MIN_LEN {
        return None;
    }

    // Address 1 is at offset 4, Address 2 at offset 10, Address 3 at offset 16
    let addr1 = format_mac(&frame[4..10]);
    let addr2 = format_mac(&frame[10..16]);
    let addr3 = format_mac(&frame[16..22]);

    // Frame control flags
    let to_ds = (frame[1] & 0x01) != 0;
    let from_ds = (frame[1] & 0x02) != 0;

    let (source, bssid, is_ap) = match (to_ds, from_ds) {
        (false, false) => (addr2.clone(), Some(addr3), false), // IBSS
        (false, true) => (addr3.clone(), Some(addr2), true),   // From AP
        (true, false) => (addr2.clone(), Some(addr1), false),  // To AP
        (true, true) => (addr2.clone(), None, false),          // WDS
    };

    Some((source, bssid, is_ap))
}

fn extract_ssid(frame: &[u8]) -> Option<String> {
    // SSID is in the management frame body after the fixed fields
    // For beacon/probe: 24 byte header + 12 bytes fixed fields = offset 36
    // Then tagged parameters start
    
    if frame.len() < 38 {
        return None;
    }

    let body_start = 24 + 12; // Header + fixed fields for beacon
    let mut offset = body_start;

    while offset + 2 < frame.len() {
        let tag_number = frame[offset];
        let tag_length = frame[offset + 1] as usize;

        if offset + 2 + tag_length > frame.len() {
            break;
        }

        if tag_number == 0 && tag_length > 0 {
            // SSID element
            let ssid_bytes = &frame[offset + 2..offset + 2 + tag_length];
            if let Ok(ssid) = String::from_utf8(ssid_bytes.to_vec()) {
                if !ssid.chars().all(|c| c == '\0') {
                    return Some(ssid);
                }
            }
        }

        offset += 2 + tag_length;
    }

    None
}

fn format_mac(bytes: &[u8]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_to_channel() {
        assert_eq!(freq_to_channel(2412), 1);
        assert_eq!(freq_to_channel(2437), 6);
        assert_eq!(freq_to_channel(5180), 36);
    }

    #[test]
    fn test_format_mac() {
        let bytes = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
        assert_eq!(format_mac(&bytes), "aa:bb:cc:dd:ee:ff");
    }
}
