//! Intelligence Tradecraft, Military Customs, ECM, and Evidence Collection
//!
//! Comprehensive database covering:
//! - Military radio procedures and customs
//! - Intelligence tradecraft (PUBLIC sources only)
//! - Electronic countermeasures (ECM/ECCM)
//! - TSCM techniques and equipment
//! - Passive collection legal framework
//! - Evidence collection procedures
//!
//! ╔══════════════════════════════════════════════════════════════════════════════╗
//! ║                        LEGAL AND ETHICAL NOTICE                              ║
//! ╠══════════════════════════════════════════════════════════════════════════════╣
//! ║  This information is compiled from PUBLIC sources only:                      ║
//! ║  - Wikipedia articles                                                        ║
//! ║  - Declassified government documents                                         ║
//! ║  - Published military field manuals                                          ║
//! ║  - Academic papers and textbooks                                             ║
//! ║  - Commercial product documentation                                          ║
//! ║                                                                              ║
//! ║  This module is for EDUCATIONAL and DEFENSIVE purposes only.                ║
//! ║  - Understand threats to protect against them                               ║
//! ║  - Know legal monitoring boundaries                                         ║
//! ║  - Support legitimate TSCM operations                                       ║
//! ║  - Aid lawful evidence collection                                           ║
//! ║                                                                              ║
//! ║  UNAUTHORIZED surveillance, interception, or jamming is ILLEGAL.            ║
//! ╚══════════════════════════════════════════════════════════════════════════════╝

use serde::{Deserialize, Serialize};

// ============================================================================
// MILITARY RADIO PROCEDURES AND CUSTOMS
// Source: Wikipedia "Radiotelephony procedure", NATO STANAG, public military manuals
// ============================================================================

/// NATO Phonetic Alphabet
/// Source: Wikipedia "NATO phonetic alphabet"
/// Standardized by ICAO, NATO, ITU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneticAlphabet {
    pub letter: char,
    pub word: &'static str,
    pub pronunciation: &'static str,
}

pub const NATO_PHONETIC_ALPHABET: [PhoneticAlphabet; 26] = [
    PhoneticAlphabet { letter: 'A', word: "Alfa", pronunciation: "AL-fah" },
    PhoneticAlphabet { letter: 'B', word: "Bravo", pronunciation: "BRAH-voh" },
    PhoneticAlphabet { letter: 'C', word: "Charlie", pronunciation: "CHAR-lee" },
    PhoneticAlphabet { letter: 'D', word: "Delta", pronunciation: "DELL-tah" },
    PhoneticAlphabet { letter: 'E', word: "Echo", pronunciation: "ECK-oh" },
    PhoneticAlphabet { letter: 'F', word: "Foxtrot", pronunciation: "FOKS-trot" },
    PhoneticAlphabet { letter: 'G', word: "Golf", pronunciation: "GOLF" },
    PhoneticAlphabet { letter: 'H', word: "Hotel", pronunciation: "hoh-TELL" },
    PhoneticAlphabet { letter: 'I', word: "India", pronunciation: "IN-dee-ah" },
    PhoneticAlphabet { letter: 'J', word: "Juliet", pronunciation: "JEW-lee-ett" },
    PhoneticAlphabet { letter: 'K', word: "Kilo", pronunciation: "KEY-loh" },
    PhoneticAlphabet { letter: 'L', word: "Lima", pronunciation: "LEE-mah" },
    PhoneticAlphabet { letter: 'M', word: "Mike", pronunciation: "MIKE" },
    PhoneticAlphabet { letter: 'N', word: "November", pronunciation: "no-VEM-ber" },
    PhoneticAlphabet { letter: 'O', word: "Oscar", pronunciation: "OSS-cah" },
    PhoneticAlphabet { letter: 'P', word: "Papa", pronunciation: "pah-PAH" },
    PhoneticAlphabet { letter: 'Q', word: "Quebec", pronunciation: "keh-BECK" },
    PhoneticAlphabet { letter: 'R', word: "Romeo", pronunciation: "ROW-me-oh" },
    PhoneticAlphabet { letter: 'S', word: "Sierra", pronunciation: "see-AIR-rah" },
    PhoneticAlphabet { letter: 'T', word: "Tango", pronunciation: "TANG-go" },
    PhoneticAlphabet { letter: 'U', word: "Uniform", pronunciation: "YOU-nee-form" },
    PhoneticAlphabet { letter: 'V', word: "Victor", pronunciation: "VIK-tah" },
    PhoneticAlphabet { letter: 'W', word: "Whiskey", pronunciation: "WISS-key" },
    PhoneticAlphabet { letter: 'X', word: "X-ray", pronunciation: "ECKS-ray" },
    PhoneticAlphabet { letter: 'Y', word: "Yankee", pronunciation: "YANG-key" },
    PhoneticAlphabet { letter: 'Z', word: "Zulu", pronunciation: "ZOO-loo" },
];

/// Procedure Words (Prowords)
/// Source: Wikipedia "Procedure word", Military communication manuals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proword {
    pub word: &'static str,
    pub meaning: &'static str,
    pub usage: &'static str,
}

pub const PROWORDS: &[Proword] = &[
    Proword { word: "ACKNOWLEDGE", meaning: "Confirm receipt", usage: "Requests confirmation that message was received" },
    Proword { word: "AFFIRMATIVE", meaning: "Yes", usage: "Positive confirmation" },
    Proword { word: "NEGATIVE", meaning: "No", usage: "Negative response" },
    Proword { word: "BREAK", meaning: "Separation", usage: "Separates text from other portions of message" },
    Proword { word: "BREAK BREAK", meaning: "Priority separation", usage: "Separates parts of message from different sources" },
    Proword { word: "COPY", meaning: "Understood", usage: "Message received and understood" },
    Proword { word: "CORRECTION", meaning: "Error in transmission", usage: "An error was made, correct version follows" },
    Proword { word: "DISREGARD", meaning: "Ignore", usage: "Disregard the previous transmission" },
    Proword { word: "EXECUTE", meaning: "Carry out", usage: "Perform the designated action immediately" },
    Proword { word: "I SAY AGAIN", meaning: "Repeat", usage: "I am repeating transmission or portion thereof" },
    Proword { word: "OUT", meaning: "End of transmission", usage: "Conversation is ended, no response expected" },
    Proword { word: "OVER", meaning: "Your turn", usage: "End of transmission, response expected" },
    Proword { word: "READ BACK", meaning: "Repeat message", usage: "Repeat this entire transmission back to me" },
    Proword { word: "ROGER", meaning: "Received", usage: "I have received your last transmission" },
    Proword { word: "SAY AGAIN", meaning: "Repeat", usage: "Repeat your last transmission" },
    Proword { word: "STANDBY", meaning: "Wait", usage: "I must pause for a few seconds" },
    Proword { word: "THIS IS", meaning: "Identification", usage: "Following is my call sign" },
    Proword { word: "WAIT", meaning: "Pause", usage: "I must pause longer than a few seconds" },
    Proword { word: "WAIT OUT", meaning: "Long pause", usage: "I must pause, will call you back" },
    Proword { word: "WILCO", meaning: "Will comply", usage: "I have received your message and will comply" },
    Proword { word: "WORDS TWICE", meaning: "Double words", usage: "Say each word twice due to poor conditions" },
];

/// Multi-Service Brevity Codes
/// Source: Wikipedia "Multi-service tactical brevity code"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrevityCode {
    pub code: &'static str,
    pub meaning: &'static str,
    pub category: &'static str,
}

pub const BREVITY_CODES: &[BrevityCode] = &[
    // Air-to-air
    BrevityCode { code: "BANDIT", meaning: "Known hostile aircraft", category: "air-to-air" },
    BrevityCode { code: "BOGEY", meaning: "Unknown aircraft", category: "air-to-air" },
    BrevityCode { code: "FRIENDLY", meaning: "Positively identified friendly", category: "air-to-air" },
    BrevityCode { code: "HOSTILE", meaning: "Identified enemy per ROE", category: "air-to-air" },
    BrevityCode { code: "MERGED", meaning: "Aircraft in same visual arena", category: "air-to-air" },
    BrevityCode { code: "SPLASH", meaning: "Target destroyed", category: "air-to-air" },
    BrevityCode { code: "WINCHESTER", meaning: "Out of ammunition", category: "air-to-air" },
    BrevityCode { code: "BINGO", meaning: "Minimum fuel for return", category: "air-to-air" },
    BrevityCode { code: "JOKER", meaning: "Fuel state requiring RTB consideration", category: "air-to-air" },
    
    // Electronic warfare
    BrevityCode { code: "MUSIC", meaning: "Electronic jamming active", category: "electronic-warfare" },
    BrevityCode { code: "SPIKED", meaning: "RWR indication of missile lock", category: "electronic-warfare" },
    BrevityCode { code: "NAILS", meaning: "RWR indication of AI radar", category: "electronic-warfare" },
    BrevityCode { code: "NAKED", meaning: "No RWR indications", category: "electronic-warfare" },
    BrevityCode { code: "NOTCH", meaning: "Maneuver to defeat radar", category: "electronic-warfare" },
    
    // General tactical
    BrevityCode { code: "ABORT", meaning: "Cease action/attack", category: "tactical" },
    BrevityCode { code: "CLEARED HOT", meaning: "Authorized to release weapons", category: "tactical" },
    BrevityCode { code: "CONTINUE", meaning: "Continue present action", category: "tactical" },
    BrevityCode { code: "FENCE IN", meaning: "Set systems for combat", category: "tactical" },
    BrevityCode { code: "FENCE OUT", meaning: "Set systems for transit", category: "tactical" },
    BrevityCode { code: "FEET WET", meaning: "Flying over water", category: "tactical" },
    BrevityCode { code: "FEET DRY", meaning: "Flying over land", category: "tactical" },
    BrevityCode { code: "RTB", meaning: "Return to base", category: "tactical" },
    BrevityCode { code: "WEAPONS FREE", meaning: "Fire at any target not friendly", category: "tactical" },
    BrevityCode { code: "WEAPONS HOLD", meaning: "Fire only in self-defense", category: "tactical" },
    BrevityCode { code: "WEAPONS TIGHT", meaning: "Fire only at identified hostiles", category: "tactical" },
];

// ============================================================================
// ELECTRONIC WARFARE (ECM/ECCM)
// Source: Wikipedia "Electronic warfare", "Radar jamming and deception"
// ============================================================================

/// Electronic Warfare Categories
/// Source: Wikipedia "Electronic warfare"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EwCategory {
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub description: &'static str,
    pub examples: Vec<&'static str>,
}

pub fn ew_categories() -> Vec<EwCategory> {
    vec![
        EwCategory {
            name: "Electronic Support",
            abbreviation: "ES",
            description: "Passive detection and analysis of electromagnetic emissions",
            examples: vec![
                "SIGINT collection",
                "Radar warning receivers (RWR)",
                "Direction finding (DF)",
                "Threat identification",
            ],
        },
        EwCategory {
            name: "Electronic Attack",
            abbreviation: "EA",
            description: "Using EM energy to attack or deny enemy use of EM spectrum",
            examples: vec![
                "Jamming",
                "Directed energy weapons",
                "Anti-radiation missiles (ARM)",
                "Expendable decoys",
            ],
        },
        EwCategory {
            name: "Electronic Protection",
            abbreviation: "EP",
            description: "Protecting friendly use of EM spectrum from EA",
            examples: vec![
                "ECCM techniques",
                "Frequency hopping",
                "Spread spectrum",
                "Low probability of intercept (LPI)",
            ],
        },
    ]
}

/// Jamming Techniques
/// Source: Wikipedia "Radar jamming and deception"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JammingTechnique {
    pub name: &'static str,
    pub technique_type: JammingType,
    pub description: &'static str,
    pub effectiveness: &'static str,
    pub countermeasures: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JammingType {
    Noise,
    Deception,
    Expendable,
}

pub fn jamming_techniques() -> Vec<JammingTechnique> {
    vec![
        // Noise jamming
        JammingTechnique {
            name: "Barrage Jamming",
            technique_type: JammingType::Noise,
            description: "Jamming across wide frequency band simultaneously",
            effectiveness: "Covers multiple frequencies but power spread thin",
            countermeasures: "Frequency hopping, burn-through range",
        },
        JammingTechnique {
            name: "Spot Jamming",
            technique_type: JammingType::Noise,
            description: "Concentrated jamming on single frequency",
            effectiveness: "High power on target frequency",
            countermeasures: "Frequency agility, spread spectrum",
        },
        JammingTechnique {
            name: "Sweep Jamming",
            technique_type: JammingType::Noise,
            description: "Jamming swept across frequency band",
            effectiveness: "Compromise between spot and barrage",
            countermeasures: "Fast frequency hopping",
        },
        
        // Deception jamming
        JammingTechnique {
            name: "Range Gate Pull-Off (RGPO)",
            technique_type: JammingType::Deception,
            description: "Creates false target that appears to move away",
            effectiveness: "Breaks radar lock, causes range errors",
            countermeasures: "Leading-edge tracking, ECCM logic",
        },
        JammingTechnique {
            name: "Velocity Gate Pull-Off (VGPO)",
            technique_type: JammingType::Deception,
            description: "Creates false Doppler shift",
            effectiveness: "Causes velocity tracking errors",
            countermeasures: "Multiple PRF, coherent processing",
        },
        JammingTechnique {
            name: "False Target Generation",
            technique_type: JammingType::Deception,
            description: "Creates multiple false radar returns",
            effectiveness: "Overwhelms tracking systems",
            countermeasures: "IFF, track-while-scan",
        },
        
        // Expendable countermeasures
        JammingTechnique {
            name: "Chaff",
            technique_type: JammingType::Expendable,
            description: "Metallic strips creating radar reflections",
            effectiveness: "Effective against pulse radars",
            countermeasures: "Doppler filtering, MTI",
        },
        JammingTechnique {
            name: "Flares",
            technique_type: JammingType::Expendable,
            description: "Hot decoys to defeat IR seekers",
            effectiveness: "Effective against older IR missiles",
            countermeasures: "Two-color seekers, imaging IR",
        },
        JammingTechnique {
            name: "Active Decoys",
            technique_type: JammingType::Expendable,
            description: "Self-powered radar reflectors/transmitters",
            effectiveness: "Creates convincing false targets",
            countermeasures: "Flight path analysis, IFF",
        },
    ]
}

// ============================================================================
// TSCM (Technical Surveillance Countermeasures)
// Source: Wikipedia "Countersurveillance", TSCM.com (Granite Island Group)
// ============================================================================

/// TSCM Equipment Categories
/// Source: Public TSCM vendor documentation, Wikipedia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TscmEquipment {
    pub category: &'static str,
    pub description: &'static str,
    pub detection_range: &'static str,
    pub examples: Vec<&'static str>,
    pub limitations: &'static str,
}

pub fn tscm_equipment() -> Vec<TscmEquipment> {
    vec![
        TscmEquipment {
            category: "Spectrum Analyzer",
            description: "Detects RF emissions across frequency bands",
            detection_range: "Typically 9 kHz to 6+ GHz",
            examples: vec![
                "REI OSCOR",
                "Tektronix RSA series",
                "Rohde & Schwarz FSH",
            ],
            limitations: "Cannot detect non-radiating or store-and-forward devices",
        },
        TscmEquipment {
            category: "Non-Linear Junction Detector (NLJD)",
            description: "Detects semiconductor junctions in hidden electronics",
            detection_range: "Through walls, floors, ceilings (limited depth)",
            examples: vec![
                "REI ORION",
                "Lornet",
                "Bohemia NLJD",
            ],
            limitations: "False positives from corrosion, nails, etc. (distinguishable by harmonic ratio)",
        },
        TscmEquipment {
            category: "Telephone Analyzer",
            description: "Tests phone lines for compromise",
            detection_range: "Wired telephone infrastructure",
            examples: vec![
                "REI TALAN",
                "CounterPro CPM-700",
            ],
            limitations: "May not detect sophisticated digital taps",
        },
        TscmEquipment {
            category: "Thermal Imager",
            description: "Detects heat from active electronics",
            detection_range: "Surface temperature variations",
            examples: vec![
                "FLIR cameras",
                "Seek Thermal",
            ],
            limitations: "Requires device to be powered and generating heat",
        },
        TscmEquipment {
            category: "Time Domain Reflectometer (TDR)",
            description: "Detects anomalies on cables/wiring",
            detection_range: "Cable runs",
            examples: vec![
                "Riser Bond TDR",
                "Fluke TDR",
            ],
            limitations: "Sophisticated taps may be impedance-matched",
        },
        TscmEquipment {
            category: "Video Lens Detector",
            description: "Detects reflections from camera lenses",
            detection_range: "Line of sight",
            examples: vec![
                "SpyFinder Pro",
                "Brickhouse Security detector",
            ],
            limitations: "Requires direct line of sight, may miss pinhole cameras",
        },
    ]
}

/// NLJD Operation Principles
/// Source: Wikipedia "Nonlinear junction detector", TSCM.com
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NljdPrinciple {
    pub aspect: &'static str,
    pub explanation: &'static str,
}

pub const NLJD_PRINCIPLES: &[NljdPrinciple] = &[
    NljdPrinciple {
        aspect: "Operating Principle",
        explanation: "Transmits RF signal; semiconductor junctions create harmonic responses",
    },
    NljdPrinciple {
        aspect: "2nd Harmonic",
        explanation: "Strong from semiconductor junctions (electronics)",
    },
    NljdPrinciple {
        aspect: "3rd Harmonic",
        explanation: "Strong from oxidized metal junctions (false positives)",
    },
    NljdPrinciple {
        aspect: "Harmonic Ratio",
        explanation: "2nd > 3rd indicates electronics; 3rd > 2nd indicates corrosion",
    },
    NljdPrinciple {
        aspect: "Typical Frequencies",
        explanation: "900 MHz, 2.4 GHz, or 3.6 GHz transmit frequency",
    },
    NljdPrinciple {
        aspect: "Detection Through",
        explanation: "Walls, floors, furniture, clothing (limited penetration)",
    },
];

// ============================================================================
// PASSIVE RF MONITORING LEGAL FRAMEWORK
// Source: FCC regulations, Wikipedia "Lawful interception"
// ============================================================================

/// Legal Framework for RF Monitoring (USA)
/// Source: FCC.gov, Wikipedia, public legal resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringLegalFramework {
    pub activity: &'static str,
    pub legal_status: LegalStatus,
    pub authority: &'static str,
    pub restrictions: &'static str,
    pub notes: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LegalStatus {
    GenerallyLegal,
    Restricted,
    RequiresLicense,
    Prohibited,
}

pub fn monitoring_legal_framework() -> Vec<MonitoringLegalFramework> {
    vec![
        MonitoringLegalFramework {
            activity: "Listening to amateur radio",
            legal_status: LegalStatus::GenerallyLegal,
            authority: "47 U.S.C. § 605",
            restrictions: "None for listening",
            notes: "Transmitting requires FCC license",
        },
        MonitoringLegalFramework {
            activity: "Listening to police/fire/EMS",
            legal_status: LegalStatus::GenerallyLegal,
            authority: "State laws vary",
            restrictions: "Some states restrict in vehicles; cannot use to aid crime",
            notes: "Many agencies encrypting; check state laws",
        },
        MonitoringLegalFramework {
            activity: "Listening to aviation",
            legal_status: LegalStatus::GenerallyLegal,
            authority: "No federal prohibition",
            restrictions: "None for listening",
            notes: "ATC on VHF AM 118-137 MHz",
        },
        MonitoringLegalFramework {
            activity: "Listening to marine",
            legal_status: LegalStatus::GenerallyLegal,
            authority: "No federal prohibition",
            restrictions: "None for listening",
            notes: "VHF marine 156-163 MHz",
        },
        MonitoringLegalFramework {
            activity: "Listening to cordless phones (pre-spread spectrum)",
            legal_status: LegalStatus::Prohibited,
            authority: "47 U.S.C. § 605, ECPA",
            restrictions: "Divulgence prohibited",
            notes: "Old analog cordless phones; modern DECT encrypted",
        },
        MonitoringLegalFramework {
            activity: "Listening to cellular",
            legal_status: LegalStatus::Prohibited,
            authority: "47 U.S.C. § 605, ECPA",
            restrictions: "Interception and divulgence prohibited",
            notes: "Modern cellular is encrypted",
        },
        MonitoringLegalFramework {
            activity: "Receiving broadcast radio/TV",
            legal_status: LegalStatus::GenerallyLegal,
            authority: "Intended for public reception",
            restrictions: "None",
            notes: "Includes FM, AM, digital TV",
        },
        MonitoringLegalFramework {
            activity: "Satellite downlinks",
            legal_status: LegalStatus::Restricted,
            authority: "Various",
            restrictions: "Encrypted content may not be descrambled without authorization",
            notes: "Unencrypted amateur, weather sats legal",
        },
        MonitoringLegalFramework {
            activity: "WiFi/Bluetooth interception",
            legal_status: LegalStatus::Prohibited,
            authority: "18 U.S.C. § 2511 (Wiretap Act)",
            restrictions: "Interception of content prohibited without consent",
            notes: "Passive detection of presence may be legal; content is not",
        },
    ]
}

// ============================================================================
// TEMPEST / EMANATIONS SECURITY
// Source: Wikipedia "TEMPEST (codename)", declassified NSA documents
// ============================================================================

/// TEMPEST Protection Levels
/// Source: Wikipedia, public NATO documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempestLevel {
    pub level: &'static str,
    pub nato_designation: &'static str,
    pub zone_requirement: &'static str,
    pub description: &'static str,
}

pub const TEMPEST_LEVELS: &[TempestLevel] = &[
    TempestLevel {
        level: "Level I",
        nato_designation: "SDIP-27 Level A (NATO AMSG 720B)",
        zone_requirement: "0-20 meters",
        description: "Highest level; for close-in threats",
    },
    TempestLevel {
        level: "Level II",
        nato_designation: "SDIP-27 Level B (NATO AMSG 788A)",
        zone_requirement: "20-100 meters",
        description: "Intermediate level",
    },
    TempestLevel {
        level: "Level III",
        nato_designation: "SDIP-27 Level C (NATO AMSG 784)",
        zone_requirement: "100+ meters",
        description: "Lowest level; for distant threats",
    },
];

/// Emanation Types
/// Source: Wikipedia "TEMPEST", academic papers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmanationType {
    pub emission_type: &'static str,
    pub source: &'static str,
    pub exploitable_info: &'static str,
    pub countermeasure: &'static str,
}

pub const EMANATION_TYPES: &[EmanationType] = &[
    EmanationType {
        emission_type: "CRT/Display emissions",
        source: "Video signal radiation",
        exploitable_info: "Screen contents (Van Eck phreaking)",
        countermeasure: "Shielded displays, TEMPEST monitors",
    },
    EmanationType {
        emission_type: "Keyboard emissions",
        source: "Key matrix scanning",
        exploitable_info: "Keystrokes/passwords",
        countermeasure: "Shielded keyboards, USB filtering",
    },
    EmanationType {
        emission_type: "Cable emissions",
        source: "Unshielded cables acting as antennas",
        exploitable_info: "Data on cables",
        countermeasure: "Shielded cables, fiber optics",
    },
    EmanationType {
        emission_type: "Power line emissions",
        source: "Data modulated onto power lines",
        exploitable_info: "Computer operations",
        countermeasure: "Power line filters",
    },
    EmanationType {
        emission_type: "Acoustic emissions",
        source: "Printer/keyboard sounds",
        exploitable_info: "Typed characters, printed text",
        countermeasure: "Sound masking, acoustic shielding",
    },
    EmanationType {
        emission_type: "LED emissions",
        source: "Activity indicators modulated by data",
        exploitable_info: "Network traffic, HDD activity",
        countermeasure: "Disable LEDs, cover indicators",
    },
];

// ============================================================================
// EVIDENCE COLLECTION AND CHAIN OF CUSTODY
// Source: NIJ, NIST, public forensics guidelines
// ============================================================================

/// Digital Evidence Collection Steps
/// Source: NIST SP 800-86, NIJ Digital Evidence Guide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceCollectionPhase {
    pub phase: &'static str,
    pub description: &'static str,
    pub key_actions: Vec<&'static str>,
    pub documentation_required: Vec<&'static str>,
}

pub fn evidence_collection_phases() -> Vec<EvidenceCollectionPhase> {
    vec![
        EvidenceCollectionPhase {
            phase: "1. Identification",
            description: "Recognize and document potential evidence",
            key_actions: vec![
                "Survey scene for digital devices",
                "Identify volatile vs non-volatile data",
                "Note powered-on states",
                "Photograph scene before touching",
            ],
            documentation_required: vec![
                "Scene photographs",
                "Device inventory",
                "Initial observations",
            ],
        },
        EvidenceCollectionPhase {
            phase: "2. Collection",
            description: "Gather evidence while preserving integrity",
            key_actions: vec![
                "Collect volatile data first (RAM, running processes)",
                "Use write blockers for storage media",
                "Create forensic images (bit-for-bit copies)",
                "Calculate hash values (MD5/SHA)",
                "Bag and tag physical evidence",
            ],
            documentation_required: vec![
                "Hash values before/after",
                "Collection methods used",
                "Tools and versions",
                "Collector identification",
            ],
        },
        EvidenceCollectionPhase {
            phase: "3. Preservation",
            description: "Maintain evidence integrity over time",
            key_actions: vec![
                "Store in appropriate containers",
                "Control environmental factors",
                "Limit access to authorized personnel",
                "Maintain chain of custody log",
            ],
            documentation_required: vec![
                "Storage location",
                "Access log",
                "Environmental conditions",
            ],
        },
        EvidenceCollectionPhase {
            phase: "4. Analysis",
            description: "Examine evidence for relevant information",
            key_actions: vec![
                "Work only on forensic copies",
                "Document all analysis steps",
                "Use validated forensic tools",
                "Record findings systematically",
            ],
            documentation_required: vec![
                "Analysis methodology",
                "Tools used",
                "Findings with timestamps",
                "Examiner notes",
            ],
        },
        EvidenceCollectionPhase {
            phase: "5. Presentation",
            description: "Report findings in admissible format",
            key_actions: vec![
                "Prepare comprehensive report",
                "Include chain of custody documentation",
                "Be prepared for testimony",
                "Maintain objectivity",
            ],
            documentation_required: vec![
                "Final report",
                "Complete chain of custody",
                "Supporting exhibits",
            ],
        },
    ]
}

/// Chain of Custody Record
/// Source: NIST, forensics best practices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfCustodyEntry {
    pub timestamp: String,
    pub item_id: String,
    pub item_description: String,
    pub action: CustodyAction,
    pub from_person: String,
    pub to_person: String,
    pub location: String,
    pub reason: String,
    pub hash_value: Option<String>,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustodyAction {
    Collected,
    Transferred,
    Analyzed,
    Stored,
    Released,
    Disposed,
}

/// RF Evidence Collection Considerations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfEvidenceConsideration {
    pub consideration: &'static str,
    pub explanation: &'static str,
    pub best_practice: &'static str,
}

pub const RF_EVIDENCE_CONSIDERATIONS: &[RfEvidenceConsideration] = &[
    RfEvidenceConsideration {
        consideration: "Spectrum Capture",
        explanation: "RF signals are ephemeral and must be captured in real-time",
        best_practice: "Use calibrated equipment with timestamps; capture IQ data when possible",
    },
    RfEvidenceConsideration {
        consideration: "Device Isolation",
        explanation: "RF devices continue transmitting if powered",
        best_practice: "Use Faraday bags/cages to isolate RF-emitting evidence",
    },
    RfEvidenceConsideration {
        consideration: "Geolocation Data",
        explanation: "RF triangulation provides location but requires calibration",
        best_practice: "Document antenna positions, equipment calibration dates",
    },
    RfEvidenceConsideration {
        consideration: "Legal Authority",
        explanation: "RF interception may require warrants depending on content",
        best_practice: "Consult legal counsel; document authority for collection",
    },
    RfEvidenceConsideration {
        consideration: "Metadata vs Content",
        explanation: "Metadata (frequency, timing) may have different legal status than content",
        best_practice: "Separate metadata collection from content where legally required",
    },
];

// ============================================================================
// SURVEILLANCE DETECTION (PUBLIC TRADECRAFT)
// Source: Wikipedia "Clandestine HUMINT operational techniques", published books
// ============================================================================

/// Surveillance Detection Route (SDR) Elements
/// Source: Wikipedia, published intelligence tradecraft books
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdrElement {
    pub element_type: &'static str,
    pub purpose: &'static str,
    pub characteristics: Vec<&'static str>,
}

pub fn sdr_elements() -> Vec<SdrElement> {
    vec![
        SdrElement {
            element_type: "Timing Points",
            purpose: "Force surveillance to reveal themselves through timing constraints",
            characteristics: vec![
                "Buses/trains with fixed schedules",
                "Timed traffic lights",
                "Elevator sequences",
            ],
        },
        SdrElement {
            element_type: "Chokepoints",
            purpose: "Funnel followers into observable areas",
            characteristics: vec![
                "Single entrances/exits",
                "Narrow passages",
                "Stairs and escalators",
            ],
        },
        SdrElement {
            element_type: "Observation Points",
            purpose: "Positions to observe for surveillance",
            characteristics: vec![
                "Windows with reflections",
                "Multiple sightlines",
                "Natural pause points",
            ],
        },
        SdrElement {
            element_type: "Cover Stops",
            purpose: "Legitimate reasons to stop and observe",
            characteristics: vec![
                "Coffee shops",
                "Bookstores (browsing)",
                "ATM machines",
            ],
        },
        SdrElement {
            element_type: "Direction Changes",
            purpose: "Force followers to react visibly",
            characteristics: vec![
                "U-turns",
                "Entering then exiting buildings",
                "Crossing streets unexpectedly",
            ],
        },
    ]
}

/// Dead Drop Concealment Methods
/// Source: Wikipedia "Dead drop", published intelligence history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadDropMethod {
    pub method: &'static str,
    pub description: &'static str,
    pub historical_use: &'static str,
}

pub const DEAD_DROP_METHODS: &[DeadDropMethod] = &[
    DeadDropMethod {
        method: "Hollow spike",
        description: "Metal spike pushed into ground, hollow interior holds message",
        historical_use: "Cold War CIA/KGB operations",
    },
    DeadDropMethod {
        method: "Magnetic container",
        description: "Small magnetic box attached to metal surfaces",
        historical_use: "Modern tradecraft",
    },
    DeadDropMethod {
        method: "Hollowed objects",
        description: "Everyday items (rocks, tree stumps) with hidden cavities",
        historical_use: "Extensively documented in CIA/KGB history",
    },
    DeadDropMethod {
        method: "Dead letter box",
        description: "Pre-arranged location (loose brick, etc.)",
        historical_use: "Traditional espionage since ancient times",
    },
    DeadDropMethod {
        method: "Digital dead drop",
        description: "Cloud storage, draft emails, blockchain",
        historical_use: "Modern digital espionage (documented in public court cases)",
    },
];

/// Signal Sites (Signaling Methods)
/// Source: Wikipedia "Clandestine HUMINT", declassified documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalMethod {
    pub signal_type: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

pub const SIGNAL_METHODS: &[SignalMethod] = &[
    SignalMethod {
        signal_type: "Visual signal",
        description: "Pre-arranged visual indicator at public location",
        example: "Chalk mark on mailbox, flower pot position",
    },
    SignalMethod {
        signal_type: "Timing signal",
        description: "Action performed at specific time/date",
        example: "Newspaper ad, radio broadcast at specific time",
    },
    SignalMethod {
        signal_type: "Load signal",
        description: "Indicates dead drop has been filled",
        example: "Tape on lamppost, thumbtack on bulletin board",
    },
    SignalMethod {
        signal_type: "Unload signal",
        description: "Confirms dead drop has been emptied",
        example: "Different chalk mark, moved object",
    },
    SignalMethod {
        signal_type: "Danger signal",
        description: "Warning of compromise or surveillance",
        example: "Absence of expected signal, specific phrase in communication",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phonetic_alphabet() {
        assert_eq!(NATO_PHONETIC_ALPHABET[0].word, "Alfa");
        assert_eq!(NATO_PHONETIC_ALPHABET[25].word, "Zulu");
    }
    
    #[test]
    fn test_prowords() {
        assert!(PROWORDS.iter().any(|p| p.word == "WILCO"));
    }
    
    #[test]
    fn test_ew_categories() {
        let cats = ew_categories();
        assert_eq!(cats.len(), 3);
        assert!(cats.iter().any(|c| c.abbreviation == "ES"));
    }
}
