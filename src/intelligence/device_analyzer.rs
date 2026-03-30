//! Device Analyzer - Coordinates AI analysis with local caching

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::llm_client::{LlmClient, LlmConfig, DeviceQuery, DeviceAnalysis};
use crate::storage::Database;

/// Cached device intelligence from AI or local lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIntelligence {
    pub mac_address: String,
    pub device_name: Option<String>,
    pub device_type: String,
    pub vendor_name: Option<String>,
    pub ai_description: Option<String>,
    pub threat_level: ThreatLevel,
    pub threat_reason: Option<String>,
    pub category: Option<String>,
    pub from_cache: bool,
    pub analyzed_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreatLevel {
    Critical,
    High,
    Medium,
    Low,
    None,
    Unknown,
}

impl Default for ThreatLevel {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct DeviceAnalyzer {
    llm_client: Option<LlmClient>,
    db: Arc<Database>,
    cache: Arc<RwLock<std::collections::HashMap<String, DeviceIntelligence>>>,
}

impl DeviceAnalyzer {
    pub fn new(llm_config: Option<LlmConfig>, db: Arc<Database>) -> Self {
        let llm_client = llm_config
            .filter(|c| c.enabled)
            .map(LlmClient::new);
        
        Self {
            llm_client,
            db,
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Check if AI analysis is available
    pub fn is_ai_available(&self) -> bool {
        self.llm_client.as_ref().map(|c| c.is_enabled()).unwrap_or(false)
    }
    
    /// Get cached intelligence for a device (fast, no AI call)
    pub async fn get_cached(&self, mac: &str) -> Option<DeviceIntelligence> {
        // Check memory cache first
        {
            let cache = self.cache.read().await;
            if let Some(intel) = cache.get(mac) {
                return Some(intel.clone());
            }
        }
        
        // Check database
        if let Ok(Some(intel)) = self.db.get_device_description(mac).await {
            // Populate memory cache
            let mut cache = self.cache.write().await;
            cache.insert(mac.to_string(), intel.clone());
            return Some(intel);
        }
        
        None
    }
    
    /// Analyze a device with AI (on-demand, user-triggered)
    pub async fn analyze_device(
        &self,
        mac: &str,
        name: Option<&str>,
        device_type: &str,
        vendor: Option<&str>,
        ssid: Option<&str>,
        is_tracker: bool,
    ) -> Result<DeviceIntelligence> {
        // Check cache first
        if let Some(cached) = self.get_cached(mac).await {
            if cached.ai_description.is_some() {
                debug!("Using cached AI description for {}", mac);
                return Ok(cached);
            }
        }
        
        // Build query
        let query = DeviceQuery {
            mac_address: mac.to_string(),
            device_name: name.map(String::from),
            device_type: device_type.to_string(),
            vendor: vendor.map(String::from),
            ssid: ssid.map(String::from),
            is_tracker,
        };
        
        // Call AI if available
        let ai_description = if let Some(ref client) = self.llm_client {
            match client.analyze_device(&query).await {
                Ok(analysis) => Some(analysis.description),
                Err(e) => {
                    warn!("AI analysis failed: {}", e);
                    None
                }
            }
        } else {
            None
        };
        
        // Determine threat level
        let threat_level = self.assess_threat(mac, vendor, is_tracker);
        
        let intel = DeviceIntelligence {
            mac_address: mac.to_string(),
            device_name: name.map(String::from),
            device_type: device_type.to_string(),
            vendor_name: vendor.map(String::from),
            ai_description,
            threat_level: threat_level.0,
            threat_reason: threat_level.1,
            category: None,
            from_cache: false,
            analyzed_at: Some(Utc::now().timestamp()),
        };
        
        // Save to database and cache
        let _ = self.db.save_device_description(&intel).await;
        {
            let mut cache = self.cache.write().await;
            cache.insert(mac.to_string(), intel.clone());
        }
        
        Ok(intel)
    }
    
    /// Batch analyze multiple devices
    pub async fn analyze_devices_batch(
        &self,
        devices: Vec<(String, Option<String>, String, Option<String>, bool)>,
    ) -> Result<Vec<DeviceIntelligence>> {
        let mut results = Vec::new();
        let mut to_analyze = Vec::new();
        let mut indices = Vec::new();
        
        // Check cache for each device
        for (i, (mac, name, dtype, vendor, is_tracker)) in devices.iter().enumerate() {
            if let Some(cached) = self.get_cached(mac).await {
                if cached.ai_description.is_some() {
                    results.push((i, cached));
                    continue;
                }
            }
            
            to_analyze.push(DeviceQuery {
                mac_address: mac.clone(),
                device_name: name.clone(),
                device_type: dtype.clone(),
                vendor: vendor.clone(),
                ssid: None,
                is_tracker: *is_tracker,
            });
            indices.push(i);
        }
        
        // Batch analyze uncached devices
        if !to_analyze.is_empty() {
            if let Some(ref client) = self.llm_client {
                match client.analyze_devices_batch(&to_analyze).await {
                    Ok(analyses) => {
                        for (query, analysis) in to_analyze.iter().zip(analyses.iter()) {
                            let threat = self.assess_threat(
                                &query.mac_address,
                                query.vendor.as_deref(),
                                query.is_tracker,
                            );
                            
                            let intel = DeviceIntelligence {
                                mac_address: query.mac_address.clone(),
                                device_name: query.device_name.clone(),
                                device_type: query.device_type.clone(),
                                vendor_name: query.vendor.clone(),
                                ai_description: Some(analysis.description.clone()),
                                threat_level: threat.0,
                                threat_reason: threat.1,
                                category: analysis.device_category.clone(),
                                from_cache: false,
                                analyzed_at: Some(Utc::now().timestamp()),
                            };
                            
                            let _ = self.db.save_device_description(&intel).await;
                            results.push((indices[results.len() - devices.len() + to_analyze.len()], intel));
                        }
                    }
                    Err(e) => {
                        warn!("Batch AI analysis failed: {}", e);
                    }
                }
            }
        }
        
        // Sort by original index and extract
        results.sort_by_key(|(i, _)| *i);
        Ok(results.into_iter().map(|(_, intel)| intel).collect())
    }
    
    /// Assess threat level based on OUI and characteristics
    fn assess_threat(&self, mac: &str, vendor: Option<&str>, is_tracker: bool) -> (ThreatLevel, Option<String>) {
        use crate::threat_intel::{check_mac_threat, get_threat_level, ThreatCategory};
        
        // Check threat intel database
        if let Some(threat) = check_mac_threat(mac) {
            let level = match threat.category {
                ThreatCategory::LawEnforcement => ThreatLevel::Critical,
                ThreatCategory::Surveillance => ThreatLevel::High,
                ThreatCategory::UsDefense | ThreatCategory::Israeli | 
                ThreatCategory::Chinese | ThreatCategory::Russian => ThreatLevel::High,
                ThreatCategory::EuropeanDefense => ThreatLevel::Medium,
                ThreatCategory::HighInterest => ThreatLevel::Medium,
            };
            return (level, Some(threat.description.to_string()));
        }
        
        // Trackers are always at least medium threat
        if is_tracker {
            return (ThreatLevel::Medium, Some("Tracking device detected".to_string()));
        }
        
        // Unknown vendor could be suspicious
        if vendor.is_none() {
            return (ThreatLevel::Low, Some("Unknown vendor - may be spoofed MAC".to_string()));
        }
        
        (ThreatLevel::None, None)
    }
}
