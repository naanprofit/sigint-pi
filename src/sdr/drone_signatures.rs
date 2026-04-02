use serde::{Deserialize, Serialize};

// DJI OUI prefixes (SZ DJI Technology Co., Ltd)
pub const DJI_OUIS: &[[u8; 3]] = &[
    [0x60, 0x60, 0x1F],
    [0x34, 0xD2, 0x62],
    [0x48, 0x1C, 0xB9],
    [0xE4, 0x7A, 0x2C],
    [0x58, 0xB8, 0x58],
    [0x04, 0xA8, 0x5A],
    [0x8C, 0x58, 0x23],
    [0x0C, 0x9A, 0xE6],
    [0x88, 0x29, 0x85],
    [0x4C, 0x43, 0xF6],
];

// Parrot SA OUI prefixes
pub const PARROT_OUIS: &[[u8; 3]] = &[
    [0x90, 0x03, 0xB7],
    [0xA0, 0x14, 0x3D],
    [0x00, 0x12, 0x1C],
    [0x00, 0x26, 0x7E],
];

// Autel Robotics
pub const AUTEL_OUIS: &[[u8; 3]] = &[
    [0x60, 0xC7, 0x98],
];

// BLE manufacturer IDs
pub const DJI_BLE_COMPANY_ID: u16 = 0x038F;
pub const ODID_BLE_SERVICE_UUID: u16 = 0xFFFA;
pub const ODID_AD_TYPE_SERVICE_DATA: u8 = 0x16;
pub const ODID_APP_CODE: u8 = 0x0D;

// WiFi NAN RemoteID frame markers
pub const WIFI_NAN_CATEGORY: u8 = 0x04;
pub const WIFI_NAN_ACTION_VENDOR_SPECIFIC: u8 = 0x09;
pub const WIFI_ALLIANCE_OUI: [u8; 3] = [0x50, 0x6F, 0x9A];
pub const NAN_OUI_TYPE: u8 = 0x13;

// Vendor-specific IE tag
pub const VENDOR_SPECIFIC_IE_TAG: u8 = 0xDD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneWifiDetection {
    pub mac_address: String,
    pub ssid: Option<String>,
    pub rssi: i32,
    pub channel: u8,
    pub manufacturer: DroneManufacturer,
    pub detection_method: WifiDetectionMethod,
    pub remote_id: Option<RemoteIdData>,
    pub drone_id: Option<DjiDroneIdData>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneBleDetection {
    pub mac_address: String,
    pub name: Option<String>,
    pub rssi: i32,
    pub manufacturer: DroneManufacturer,
    pub remote_id: Option<RemoteIdData>,
    pub company_id: Option<u16>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DroneManufacturer {
    Dji,
    Parrot,
    Autel,
    Skydio,
    Yuneec,
    HolyStone,
    Fimi,
    Hubsan,
    Eachine,
    Sjrc,
    Jjrc,
    Mjx,
    Potensic,
    Ruko,
    Generic,
    Unknown,
}

impl DroneManufacturer {
    pub fn label(&self) -> &str {
        match self {
            Self::Dji => "DJI",
            Self::Parrot => "Parrot",
            Self::Autel => "Autel Robotics",
            Self::Skydio => "Skydio",
            Self::Yuneec => "Yuneec",
            Self::HolyStone => "Holy Stone",
            Self::Fimi => "FIMI/Xiaomi",
            Self::Hubsan => "Hubsan",
            Self::Eachine => "Eachine",
            Self::Sjrc => "SJRC",
            Self::Jjrc => "JJRC",
            Self::Mjx => "MJX",
            Self::Potensic => "Potensic",
            Self::Ruko => "Ruko",
            Self::Generic => "Generic Drone",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WifiDetectionMethod {
    OuiMatch,
    SsidPattern,
    NanRemoteId,
    BeaconRemoteId,
    VendorIeDroneId,
    ActionFrame,
}

// ASTM F3411-22a Open Drone ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteIdData {
    pub msg_type: OdidMessageType,
    pub protocol_version: u8,
    pub uas_id: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude_m: Option<f32>,
    pub height_agl_m: Option<f32>,
    pub speed_mps: Option<f32>,
    pub heading_deg: Option<f32>,
    pub operator_id: Option<String>,
    pub operator_lat: Option<f64>,
    pub operator_lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OdidMessageType {
    BasicId,
    LocationVector,
    Authentication,
    SelfId,
    System,
    OperatorId,
    MessagePack,
    Unknown(u8),
}

impl OdidMessageType {
    pub fn from_byte(b: u8) -> Self {
        match (b >> 4) & 0x0F {
            0x0 => Self::BasicId,
            0x1 => Self::LocationVector,
            0x2 => Self::Authentication,
            0x3 => Self::SelfId,
            0x4 => Self::System,
            0x5 => Self::OperatorId,
            0xF => Self::MessagePack,
            v => Self::Unknown(v),
        }
    }
}

// DJI DroneID (proprietary)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjiDroneIdData {
    pub serial_number: Option<String>,
    pub drone_lat: Option<f64>,
    pub drone_lon: Option<f64>,
    pub drone_alt_m: Option<f32>,
    pub pilot_lat: Option<f64>,
    pub pilot_lon: Option<f64>,
    pub home_lat: Option<f64>,
    pub home_lon: Option<f64>,
    pub speed_mps: Option<f32>,
    pub heading_deg: Option<f32>,
    pub product_type: Option<u8>,
}

/// Known drone SSID patterns (case-insensitive prefix matching)
pub const DRONE_SSID_PREFIXES: &[(&str, DroneManufacturer)] = &[
    ("DJI-", DroneManufacturer::Dji),
    ("DJI_", DroneManufacturer::Dji),
    ("TELLO-", DroneManufacturer::Dji),
    ("TELLO_", DroneManufacturer::Dji),
    ("Spark-", DroneManufacturer::Dji),
    ("ANAFI-", DroneManufacturer::Parrot),
    ("Anafi-", DroneManufacturer::Parrot),
    ("Bebop2-", DroneManufacturer::Parrot),
    ("DISCO-", DroneManufacturer::Parrot),
    ("Mambo-", DroneManufacturer::Parrot),
    ("Autel-", DroneManufacturer::Autel),
    ("AUTEL_", DroneManufacturer::Autel),
    ("EVO-", DroneManufacturer::Autel),
    ("Skydio-", DroneManufacturer::Skydio),
    ("SKYDIO-", DroneManufacturer::Skydio),
    ("YUNEEC-", DroneManufacturer::Yuneec),
    ("YuneecH520-", DroneManufacturer::Yuneec),
    ("FIMI_", DroneManufacturer::Fimi),
    ("Xiaomi_", DroneManufacturer::Fimi),
    ("Hubsan-", DroneManufacturer::Hubsan),
    ("ZINO-", DroneManufacturer::Hubsan),
    ("HolyStone-", DroneManufacturer::HolyStone),
    ("HS-", DroneManufacturer::HolyStone),
    ("FPV_", DroneManufacturer::Generic),
    ("WiFi-UFO-", DroneManufacturer::Eachine),
    ("EAchine-", DroneManufacturer::Eachine),
    ("SJRC", DroneManufacturer::Sjrc),
    ("JJRC", DroneManufacturer::Jjrc),
    ("MJX", DroneManufacturer::Mjx),
    ("Potensic", DroneManufacturer::Potensic),
    ("Ruko", DroneManufacturer::Ruko),
    ("SG-PRO", DroneManufacturer::Generic),
    ("VISUO", DroneManufacturer::Generic),
];

/// Additional SSID substrings that indicate drone controllers
/// (some DJI controllers use cellular modem SSIDs with embedded model numbers)
pub const DRONE_SSID_SUBSTRINGS: &[(&str, DroneManufacturer)] = &[
    ("RC400L", DroneManufacturer::Dji),    // DJI RC with LTE (Verizon)
    ("RC230", DroneManufacturer::Dji),     // DJI RC Pro
    ("RC231", DroneManufacturer::Dji),     // DJI RC Pro v2
    ("RC-N1", DroneManufacturer::Dji),     // DJI RC-N1
    ("RC-N2", DroneManufacturer::Dji),     // DJI RC-N2
    ("DJI-RC", DroneManufacturer::Dji),    // DJI RC variants
    ("OcuSync", DroneManufacturer::Dji),   // OcuSync link
    ("SkyLink", DroneManufacturer::Autel), // Autel SkyLink
];

pub fn mac_to_bytes(mac: &str) -> Option<[u8; 3]> {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() < 3 {
        return None;
    }
    Some([
        u8::from_str_radix(parts[0], 16).ok()?,
        u8::from_str_radix(parts[1], 16).ok()?,
        u8::from_str_radix(parts[2], 16).ok()?,
    ])
}

pub fn match_oui(mac: &str) -> Option<DroneManufacturer> {
    let oui = mac_to_bytes(mac)?;
    if DJI_OUIS.contains(&oui) {
        return Some(DroneManufacturer::Dji);
    }
    if PARROT_OUIS.contains(&oui) {
        return Some(DroneManufacturer::Parrot);
    }
    if AUTEL_OUIS.contains(&oui) {
        return Some(DroneManufacturer::Autel);
    }
    None
}

pub fn match_ssid(ssid: &str) -> Option<DroneManufacturer> {
    let ssid_upper = ssid.to_uppercase();
    for (prefix, mfr) in DRONE_SSID_PREFIXES {
        if ssid_upper.starts_with(&prefix.to_uppercase()) {
            return Some(mfr.clone());
        }
    }
    // Check substring patterns (e.g., "Verizon-RC400L-26" contains "RC400L")
    for (substr, mfr) in DRONE_SSID_SUBSTRINGS {
        if ssid_upper.contains(&substr.to_uppercase()) {
            return Some(mfr.clone());
        }
    }
    None
}

pub fn check_wifi_device_is_drone(mac: &str, ssid: Option<&str>) -> Option<(DroneManufacturer, WifiDetectionMethod)> {
    if let Some(mfr) = match_oui(mac) {
        return Some((mfr, WifiDetectionMethod::OuiMatch));
    }
    if let Some(ssid) = ssid {
        if let Some(mfr) = match_ssid(ssid) {
            return Some((mfr, WifiDetectionMethod::SsidPattern));
        }
    }
    None
}

/// Parse WiFi action frame for NAN RemoteID
/// Returns RemoteIdData if this is a valid ODID NAN frame
pub fn parse_nan_remoteid(frame_body: &[u8]) -> Option<RemoteIdData> {
    // Action frame body: Category(1) | Action(1) | OUI(3) | OUI Type(1) | NAN body
    if frame_body.len() < 7 {
        return None;
    }
    if frame_body[0] != WIFI_NAN_CATEGORY {
        return None;
    }
    if frame_body[1] != WIFI_NAN_ACTION_VENDOR_SPECIFIC {
        return None;
    }
    if frame_body[2..5] != WIFI_ALLIANCE_OUI {
        return None;
    }
    if frame_body[5] != NAN_OUI_TYPE {
        return None;
    }
    // NAN body contains service discovery frames with ODID payloads
    // Parse the NAN attributes to find ODID service data
    let nan_body = &frame_body[6..];
    parse_odid_from_nan(nan_body)
}

fn parse_odid_from_nan(data: &[u8]) -> Option<RemoteIdData> {
    // NAN attributes are TLV: AttrID(1) | Length(2-LE) | Body
    let mut offset = 0;
    while offset + 3 <= data.len() {
        let attr_id = data[offset];
        let attr_len = u16::from_le_bytes([data[offset + 1], data[offset + 2]]) as usize;
        offset += 3;
        if offset + attr_len > data.len() {
            break;
        }
        // Service Discovery Frame (attr_id = 0x03) may contain ODID
        if attr_id == 0x03 && attr_len > 6 {
            let sdf = &data[offset..offset + attr_len];
            if let Some(rid) = try_parse_odid_payload(sdf) {
                return Some(rid);
            }
        }
        offset += attr_len;
    }
    // Fallback: try parsing entire NAN body as ODID
    try_parse_odid_payload(data)
}

/// Parse vendor-specific IE in beacon for DJI DroneID
pub fn parse_vendor_ie_droneid(ie_data: &[u8]) -> Option<DjiDroneIdData> {
    // Vendor IE: OUI(3) | Type(1) | Data(variable)
    if ie_data.len() < 40 {
        return None;
    }
    let oui = [ie_data[0], ie_data[1], ie_data[2]];
    if !DJI_OUIS.contains(&oui) {
        return None;
    }
    // DJI DroneID payload starts after OUI + type byte
    let payload = &ie_data[4..];
    parse_dji_droneid_payload(payload)
}

fn parse_dji_droneid_payload(data: &[u8]) -> Option<DjiDroneIdData> {
    if data.len() < 58 {
        return None;
    }
    // Version(1) | SeqNum(2) | State(2) | SerialNum(16) | DroneLon(4) | DroneLat(4) |
    // DroneAlt(2) | Height(2) | VelN(2) | VelE(2) | VelUp(2) | Yaw(2) |
    // PilotLon(4) | PilotLat(4) | HomeLon(4) | HomeLat(4) | ProductType(1)
    let serial_bytes = &data[5..21];
    let serial = String::from_utf8_lossy(serial_bytes)
        .trim_matches('\0')
        .to_string();

    let drone_lon = i32::from_le_bytes([data[21], data[22], data[23], data[24]]) as f64 / 1e7;
    let drone_lat = i32::from_le_bytes([data[25], data[26], data[27], data[28]]) as f64 / 1e7;
    let drone_alt = i16::from_le_bytes([data[29], data[30]]) as f32;
    let height = i16::from_le_bytes([data[31], data[32]]) as f32;
    let speed_n = i16::from_le_bytes([data[33], data[34]]) as f32 / 100.0;
    let speed_e = i16::from_le_bytes([data[35], data[36]]) as f32 / 100.0;
    let speed = (speed_n * speed_n + speed_e * speed_e).sqrt();
    let heading = (i16::from_le_bytes([data[39], data[40]]) as f32) / 100.0;
    let pilot_lon = i32::from_le_bytes([data[41], data[42], data[43], data[44]]) as f64 / 1e7;
    let pilot_lat = i32::from_le_bytes([data[45], data[46], data[47], data[48]]) as f64 / 1e7;
    let home_lon = i32::from_le_bytes([data[49], data[50], data[51], data[52]]) as f64 / 1e7;
    let home_lat = i32::from_le_bytes([data[53], data[54], data[55], data[56]]) as f64 / 1e7;
    let product_type = data[57];

    // Validate coordinates (reject zeros / obviously invalid)
    let valid_lat = |v: f64| v.abs() > 0.1 && v.abs() < 90.0;
    let valid_lon = |v: f64| v.abs() > 0.1 && v.abs() < 180.0;

    Some(DjiDroneIdData {
        serial_number: if serial.is_empty() { None } else { Some(serial) },
        drone_lat: if valid_lat(drone_lat) { Some(drone_lat) } else { None },
        drone_lon: if valid_lon(drone_lon) { Some(drone_lon) } else { None },
        drone_alt_m: if drone_alt.abs() < 10000.0 { Some(drone_alt) } else { None },
        pilot_lat: if valid_lat(pilot_lat) { Some(pilot_lat) } else { None },
        pilot_lon: if valid_lon(pilot_lon) { Some(pilot_lon) } else { None },
        home_lat: if valid_lat(home_lat) { Some(home_lat) } else { None },
        home_lon: if valid_lon(home_lon) { Some(home_lon) } else { None },
        speed_mps: Some(speed),
        heading_deg: Some(heading),
        product_type: Some(product_type),
    })
}

/// Parse ASTM F3411 ODID payload (from BLE service data or WiFi NAN)
pub fn try_parse_odid_payload(data: &[u8]) -> Option<RemoteIdData> {
    if data.len() < 3 {
        return None;
    }
    // Find ODID magic: AD Type 0x16 + UUID 0xFFFA (LE: 0xFA 0xFF)
    for i in 0..data.len().saturating_sub(5) {
        if data[i] == ODID_AD_TYPE_SERVICE_DATA
            && i + 2 < data.len()
            && data[i + 1] == 0xFA
            && data[i + 2] == 0xFF
        {
            let msg_start = i + 3;
            if msg_start + 2 > data.len() {
                return None;
            }
            return parse_odid_message(&data[msg_start..]);
        }
    }
    // Try raw ODID (no AD wrapper)
    if data.len() >= 20 {
        let msg_type_byte = data[0];
        let proto = msg_type_byte & 0x0F;
        if proto <= 0x02 {
            return parse_odid_message(data);
        }
    }
    None
}

fn parse_odid_message(data: &[u8]) -> Option<RemoteIdData> {
    if data.len() < 2 {
        return None;
    }
    let msg_type = OdidMessageType::from_byte(data[0]);
    let proto_ver = data[0] & 0x0F;

    match msg_type {
        OdidMessageType::BasicId => parse_odid_basic_id(data, proto_ver),
        OdidMessageType::LocationVector => parse_odid_location(data, proto_ver),
        OdidMessageType::System => parse_odid_system(data, proto_ver),
        OdidMessageType::OperatorId => parse_odid_operator(data, proto_ver),
        OdidMessageType::MessagePack => {
            // Message pack: counter(1) | size(1) | count(1) | messages...
            if data.len() > 4 {
                let msg_size = data[2] as usize;
                let msg_count = data[3] as usize;
                let mut offset = 4;
                for _ in 0..msg_count {
                    if offset + msg_size > data.len() { break; }
                    if let Some(rid) = parse_odid_message(&data[offset..offset + msg_size]) {
                        return Some(rid);
                    }
                    offset += msg_size;
                }
            }
            None
        }
        _ => Some(RemoteIdData {
            msg_type,
            protocol_version: proto_ver,
            uas_id: None, latitude: None, longitude: None,
            altitude_m: None, height_agl_m: None, speed_mps: None,
            heading_deg: None, operator_id: None,
            operator_lat: None, operator_lon: None,
        }),
    }
}

fn parse_odid_basic_id(data: &[u8], proto_ver: u8) -> Option<RemoteIdData> {
    // Byte 1: ID Type (4 bits) | UA Type (4 bits)
    // Bytes 2-21: UAS ID (20 bytes, null-padded ASCII)
    if data.len() < 22 { return None; }
    let uas_id = String::from_utf8_lossy(&data[2..22])
        .trim_matches('\0')
        .to_string();
    Some(RemoteIdData {
        msg_type: OdidMessageType::BasicId,
        protocol_version: proto_ver,
        uas_id: if uas_id.is_empty() { None } else { Some(uas_id) },
        latitude: None, longitude: None, altitude_m: None,
        height_agl_m: None, speed_mps: None, heading_deg: None,
        operator_id: None, operator_lat: None, operator_lon: None,
    })
}

fn parse_odid_location(data: &[u8], proto_ver: u8) -> Option<RemoteIdData> {
    // Byte 1: Status(4) | HeightType(1) | EWDirection(1) | SpeedMult(1) | reserved
    // Byte 2: Direction (degrees)
    // Byte 3-4: Speed (u16, 0.25 m/s or 0.75 m/s units based on SpeedMult)
    // Byte 5-6: Vertical speed (i16, 0.5 m/s units)
    // Bytes 7-10: Latitude (i32, 1e-7 degrees)
    // Bytes 11-14: Longitude (i32, 1e-7 degrees)
    // Bytes 15-16: Pressure altitude (u16, 0.5m units, offset -1000m)
    // Bytes 17-18: Geodetic altitude (u16, 0.5m units, offset -1000m)
    // Bytes 19-20: Height AGL (u16, 0.5m units, offset -1000m)
    if data.len() < 21 { return None; }
    let direction = data[2] as f32 * if (data[1] & 0x04) != 0 { 1.0 } else { 1.0 };
    let speed_raw = u16::from_le_bytes([data[3], data[4]]);
    let speed_mult = if (data[1] & 0x02) != 0 { 0.75 } else { 0.25 };
    let speed = speed_raw as f32 * speed_mult;
    let lat = i32::from_le_bytes([data[7], data[8], data[9], data[10]]) as f64 / 1e7;
    let lon = i32::from_le_bytes([data[11], data[12], data[13], data[14]]) as f64 / 1e7;
    let alt_raw = u16::from_le_bytes([data[15], data[16]]);
    let alt = alt_raw as f32 * 0.5 - 1000.0;
    let height_raw = u16::from_le_bytes([data[19], data[20]]);
    let height = height_raw as f32 * 0.5 - 1000.0;

    Some(RemoteIdData {
        msg_type: OdidMessageType::LocationVector,
        protocol_version: proto_ver,
        uas_id: None,
        latitude: if lat.abs() > 0.001 { Some(lat) } else { None },
        longitude: if lon.abs() > 0.001 { Some(lon) } else { None },
        altitude_m: Some(alt),
        height_agl_m: Some(height),
        speed_mps: Some(speed),
        heading_deg: Some(direction),
        operator_id: None, operator_lat: None, operator_lon: None,
    })
}

fn parse_odid_system(data: &[u8], proto_ver: u8) -> Option<RemoteIdData> {
    // Bytes 2-5: Operator Latitude (i32, 1e-7 degrees)
    // Bytes 6-9: Operator Longitude (i32, 1e-7 degrees)
    if data.len() < 10 { return None; }
    let op_lat = i32::from_le_bytes([data[2], data[3], data[4], data[5]]) as f64 / 1e7;
    let op_lon = i32::from_le_bytes([data[6], data[7], data[8], data[9]]) as f64 / 1e7;
    Some(RemoteIdData {
        msg_type: OdidMessageType::System,
        protocol_version: proto_ver,
        uas_id: None, latitude: None, longitude: None,
        altitude_m: None, height_agl_m: None, speed_mps: None, heading_deg: None,
        operator_id: None,
        operator_lat: if op_lat.abs() > 0.001 { Some(op_lat) } else { None },
        operator_lon: if op_lon.abs() > 0.001 { Some(op_lon) } else { None },
    })
}

fn parse_odid_operator(data: &[u8], proto_ver: u8) -> Option<RemoteIdData> {
    // Byte 1: Operator ID type
    // Bytes 2-21: Operator ID (20 bytes ASCII)
    if data.len() < 22 { return None; }
    let op_id = String::from_utf8_lossy(&data[2..22])
        .trim_matches('\0')
        .to_string();
    Some(RemoteIdData {
        msg_type: OdidMessageType::OperatorId,
        protocol_version: proto_ver,
        uas_id: None, latitude: None, longitude: None,
        altitude_m: None, height_agl_m: None, speed_mps: None, heading_deg: None,
        operator_id: if op_id.is_empty() { None } else { Some(op_id) },
        operator_lat: None, operator_lon: None,
    })
}

/// Scan beacon frame body for vendor-specific IEs containing DJI DroneID
pub fn scan_beacon_for_droneid(frame_body: &[u8]) -> Option<DjiDroneIdData> {
    // Skip fixed fields in beacon (timestamp=8, interval=2, capabilities=2 = 12 bytes)
    if frame_body.len() < 12 { return None; }
    let mut offset = 12;
    while offset + 2 <= frame_body.len() {
        let tag = frame_body[offset];
        let len = frame_body[offset + 1] as usize;
        offset += 2;
        if offset + len > frame_body.len() { break; }
        if tag == VENDOR_SPECIFIC_IE_TAG && len >= 4 {
            if let Some(did) = parse_vendor_ie_droneid(&frame_body[offset..offset + len]) {
                return Some(did);
            }
        }
        offset += len;
    }
    None
}

/// Check BLE advertisement for Open Drone ID
pub fn parse_ble_odid(manufacturer_data: &[(u16, Vec<u8>)], service_data: &[(u16, Vec<u8>)]) -> Option<RemoteIdData> {
    // Check service data for UUID 0xFFFA
    for (uuid, data) in service_data {
        if *uuid == ODID_BLE_SERVICE_UUID && data.len() >= 2 {
            return parse_odid_message(data);
        }
    }
    // Check manufacturer data for DJI company ID
    for (company_id, data) in manufacturer_data {
        if *company_id == DJI_BLE_COMPANY_ID && data.len() >= 2 {
            return try_parse_odid_payload(data);
        }
    }
    None
}

/// Check if a BLE company ID belongs to a drone manufacturer
pub fn ble_company_is_drone(company_id: u16) -> Option<DroneManufacturer> {
    match company_id {
        0x038F => Some(DroneManufacturer::Dji),
        _ => None,
    }
}

// 5.8 GHz analog FPV frequency table (all bands)
pub const FPV_58_FREQUENCIES: &[(& str, &[(& str, u32)])] = &[
    ("A (Boscam)", &[
        ("A1", 5865), ("A2", 5845), ("A3", 5825), ("A4", 5805),
        ("A5", 5785), ("A6", 5765), ("A7", 5745), ("A8", 5725),
    ]),
    ("B (FlySky)", &[
        ("B1", 5733), ("B2", 5752), ("B3", 5771), ("B4", 5790),
        ("B5", 5809), ("B6", 5828), ("B7", 5847), ("B8", 5866),
    ]),
    ("E (DJI/Lumenier)", &[
        ("E1", 5705), ("E2", 5685), ("E3", 5665), ("E4", 5645),
        ("E5", 5885), ("E6", 5905), ("E7", 5925), ("E8", 5945),
    ]),
    ("F (FatShark)", &[
        ("F1", 5740), ("F2", 5760), ("F3", 5780), ("F4", 5800),
        ("F5", 5820), ("F6", 5840), ("F7", 5860), ("F8", 5880),
    ]),
    ("R (Raceband)", &[
        ("R1", 5658), ("R2", 5695), ("R3", 5732), ("R4", 5769),
        ("R5", 5806), ("R6", 5843), ("R7", 5880), ("R8", 5917),
    ]),
];
