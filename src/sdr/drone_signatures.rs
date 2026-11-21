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

/// Controller model → likely drone type mapping
/// Returns (controller_model, likely_drones)
pub const CONTROLLER_DRONE_MAP: &[(&str, &str, &[&str])] = &[
    ("RC400L",  "DJI RC (LTE/4G)",     &["Mavic 3 Pro", "Mavic 3 Classic", "Air 3", "Mini 4 Pro"]),
    ("RC230",   "DJI RC Pro",           &["Mavic 3 Pro", "Mavic 3 Cine", "Inspire 3"]),
    ("RC231",   "DJI RC Pro v2",        &["Mavic 3 Pro", "Air 3"]),
    ("RC-N1",   "DJI RC-N1",           &["Mini 3 Pro", "Mini 3", "Mavic 3", "Air 2S", "Mini SE"]),
    ("RC-N2",   "DJI RC-N2",           &["Air 3", "Mini 4 Pro", "Mini 3 Pro"]),
    ("DJI-RC2", "DJI RC 2",            &["Air 3", "Mini 4 Pro"]),
    ("DJI-RC",  "DJI RC",              &["Mini 3 Pro", "Mavic 3", "Mini 4 Pro"]),
    ("OcuSync", "OcuSync Controller",  &["Mavic 2 Pro/Zoom", "Phantom 4 Pro V2"]),
    ("SkyLink", "Autel SkyLink",       &["EVO II Pro", "EVO II V3", "EVO Max 4T", "EVO Nano+"]),
];

/// DJI product_type byte → drone model (from DJI DroneID beacon payload)
pub const DJI_PRODUCT_TYPES: &[(u8, &str)] = &[
    (0x01, "Inspire 1"),
    (0x02, "Phantom 3 Series"),
    (0x03, "Phantom 3 4K / Phantom 3 Std"),
    (0x04, "Mavic Pro"),
    (0x05, "Spark"),
    (0x06, "Inspire 2"),
    (0x07, "Phantom 4 Pro"),
    (0x08, "Phantom 4 Advanced"),
    (0x09, "Mavic Air"),
    (0x0A, "Mavic 2 Pro"),
    (0x0B, "Mavic 2 Zoom"),
    (0x0E, "Mavic Mini"),
    (0x0F, "Mavic Air 2"),
    (0x10, "DJI FPV"),
    (0x11, "Mini 2"),
    (0x12, "Mavic 3"),
    (0x13, "Mavic 3 Cine"),
    (0x14, "Mini SE"),
    (0x15, "Mini 3 Pro"),
    (0x16, "Mavic 3 Classic"),
    (0x17, "Mavic 3 Pro"),
    (0x18, "Air 2S"),
    (0x19, "Air 3"),
    (0x1A, "Mini 4 Pro"),
    (0x1B, "Avata"),
    (0x1C, "Avata 2"),
    (0x1D, "Mini 3"),
    (0x1E, "Inspire 3"),
    (0x1F, "Matrice 350 RTK"),
    (0x20, "Matrice 30"),
    (0x21, "Matrice 30T"),
    (0x22, "DJI Flip"),
];

/// Resolve the likely drone model from an SSID or controller model string.
/// Returns (controller_name, likely_drone_models).
pub fn identify_drone_type(ssid: Option<&str>, product_type: Option<u8>) -> (Option<String>, Vec<String>) {
    // If we have a product_type byte from DroneID, that's definitive
    if let Some(pt) = product_type {
        for (code, model) in DJI_PRODUCT_TYPES {
            if *code == pt {
                return (Some(model.to_string()), vec![model.to_string()]);
            }
        }
    }

    // Otherwise try to match controller model from SSID
    if let Some(ssid) = ssid {
        let ssid_upper = ssid.to_uppercase();
        for (pattern, controller_name, drones) in CONTROLLER_DRONE_MAP {
            if ssid_upper.contains(&pattern.to_uppercase()) {
                return (
                    Some(controller_name.to_string()),
                    drones.iter().map(|s| s.to_string()).collect(),
                );
            }
        }
    }

    (None, vec![])
}

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
    remoteid::wifi::parse_nan_action(frame_body)
        .ok()
        .and_then(|rid| rid.frame().ok())
        .and_then(from_remoteid_frame)
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
            return remoteid::ble::parse_service_data(&data[i + 3..])
                .ok()
                .and_then(|rid| rid.frame().ok())
                .and_then(from_remoteid_frame);
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
    remoteid::Frame::decode(data).ok().and_then(from_remoteid_frame)
}

/// Map a decoded ASTM F3411 frame onto RemoteIdData. A message pack is merged
/// into one record, so Basic ID, Location, System and Operator ID all land.
fn from_remoteid_frame(frame: remoteid::Frame<'_>) -> Option<RemoteIdData> {
    let msgs: Vec<remoteid::Message> = match frame {
        remoteid::Frame::Message(m) => vec![m],
        remoteid::Frame::Pack(p) => p.messages().filter_map(Result::ok).collect(),
    };
    let first = msgs.first()?;
    let mut rid = RemoteIdData {
        msg_type: OdidMessageType::from_byte(u8::from(first.message_type()) << 4),
        protocol_version: first.protocol_version(),
        uas_id: None, latitude: None, longitude: None,
        altitude_m: None, height_agl_m: None, speed_mps: None,
        heading_deg: None, operator_id: None,
        operator_lat: None, operator_lon: None,
    };
    for m in &msgs {
        match m {
            remoteid::Message::BasicId(b) if rid.uas_id.is_none() => {
                rid.uas_id = b.uas_id_str().map(String::from);
            }
            remoteid::Message::Location(l) => {
                if let Some((lat, lon)) = l.position() {
                    rid.latitude = Some(lat);
                    rid.longitude = Some(lon);
                }
                rid.altitude_m = l.altitude_geo;
                rid.height_agl_m = l.height;
                rid.speed_mps = l.speed_horizontal;
                rid.heading_deg = l.direction;
            }
            remoteid::Message::System(sys) => {
                if let Some((lat, lon)) = sys.operator_position() {
                    rid.operator_lat = Some(lat);
                    rid.operator_lon = Some(lon);
                }
            }
            remoteid::Message::OperatorId(o) => {
                rid.operator_id = o.operator_id_str().map(String::from);
            }
            _ => {}
        }
    }
    Some(rid)
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
            return remoteid::ble::parse_service_data(data)
                .ok()
                .and_then(|rid| rid.frame().ok())
                .and_then(from_remoteid_frame);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_id() -> remoteid::Message {
        remoteid::Message::BasicId(
            remoteid::BasicId::new(
                remoteid::IdType::SerialNumber,
                remoteid::UaType::HelicopterOrMultirotor,
                "1596F3507452",
            )
            .unwrap(),
        )
    }

    #[test]
    fn ble_service_data_starts_after_app_code_and_counter() {
        // btleplug hands over UUID 0xFFFA service data: app code 0x0D, counter, message.
        let mut data = vec![ODID_APP_CODE, 0x07];
        data.extend_from_slice(&basic_id().encode().unwrap());
        let rid = parse_ble_odid(&[], &[(ODID_BLE_SERVICE_UUID, data)]).unwrap();
        assert_eq!(rid.msg_type, OdidMessageType::BasicId);
        assert_eq!(rid.protocol_version, 2);
        assert_eq!(rid.uas_id.as_deref(), Some("1596F3507452"));
    }

    #[test]
    fn ble_ad_structure_is_decoded() {
        let mut adv = vec![0x1E, ODID_AD_TYPE_SERVICE_DATA, 0xFA, 0xFF, ODID_APP_CODE, 0x07];
        adv.extend_from_slice(&basic_id().encode().unwrap());
        let rid = try_parse_odid_payload(&adv).unwrap();
        assert_eq!(rid.uas_id.as_deref(), Some("1596F3507452"));
    }

    #[test]
    fn nan_message_pack_yields_id_and_position() {
        let loc = remoteid::Message::Location(remoteid::Location {
            latitude: 37.7749,
            longitude: -122.4194,
            ..Default::default()
        });
        let mut pack = remoteid::MessagePackBuilder::new();
        pack.push(&basic_id()).unwrap();
        pack.push(&loc).unwrap();
        let mut info = vec![0x05];
        info.extend_from_slice(pack.as_bytes());
        let mut sda = remoteid::wifi::NAN_SERVICE_ID.to_vec();
        sda.extend_from_slice(&[0x01, 0x00, 0x10, info.len() as u8]);
        sda.extend_from_slice(&info);
        let mut body = vec![0x04, 0x09, 0x50, 0x6F, 0x9A, 0x13, 0x03];
        body.extend_from_slice(&(sda.len() as u16).to_le_bytes());
        body.extend_from_slice(&sda);

        let rid = parse_nan_remoteid(&body).unwrap();
        assert_eq!(rid.uas_id.as_deref(), Some("1596F3507452"));
        assert!((rid.latitude.unwrap() - 37.7749).abs() < 1e-6);
        assert!((rid.longitude.unwrap() + 122.4194).abs() < 1e-6);
    }
}
