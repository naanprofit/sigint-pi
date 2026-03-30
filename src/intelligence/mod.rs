//! Device Intelligence Module
//! 
//! Provides AI-powered device identification and threat assessment,
//! with local caching for offline operation.

pub mod llm_client;
pub mod device_analyzer;

pub use llm_client::{LlmClient, LlmConfig};
pub use device_analyzer::{DeviceAnalyzer, DeviceIntelligence, ThreatLevel};
