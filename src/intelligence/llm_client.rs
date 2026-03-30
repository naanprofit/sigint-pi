//! LLM Client for device analysis
//! 
//! Connects to local LLM (Ollama/llama.cpp) for device explanations.
//! All API credentials are loaded from config at runtime, never hardcoded.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

// Re-export LlmConfig from config module
pub use crate::config::LlmConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceQuery {
    pub mac_address: String,
    pub device_name: Option<String>,
    pub device_type: String,
    pub vendor: Option<String>,
    pub ssid: Option<String>,
    pub is_tracker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAnalysis {
    pub description: String,
    pub threat_assessment: Option<String>,
    pub device_category: Option<String>,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageResponse,
}

#[derive(Debug, Deserialize)]
struct ChatMessageResponse {
    content: String,
}

pub struct LlmClient {
    config: LlmConfig,
    client: reqwest::Client,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { config, client }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    /// Analyze a single device
    pub async fn analyze_device(&self, device: &DeviceQuery) -> Result<DeviceAnalysis> {
        if !self.config.enabled {
            return Ok(DeviceAnalysis {
                description: "AI analysis disabled".to_string(),
                threat_assessment: None,
                device_category: None,
                confidence: 0.0,
            });
        }
        
        let prompt = self.build_device_prompt(device);
        let response = self.query_llm(&prompt).await?;
        
        Ok(DeviceAnalysis {
            description: response,
            threat_assessment: None,
            device_category: self.guess_category(device),
            confidence: 0.8,
        })
    }
    
    /// Analyze multiple devices in one request (more efficient)
    pub async fn analyze_devices_batch(&self, devices: &[DeviceQuery]) -> Result<Vec<DeviceAnalysis>> {
        if !self.config.enabled || devices.is_empty() {
            return Ok(devices.iter().map(|_| DeviceAnalysis {
                description: "AI analysis disabled".to_string(),
                threat_assessment: None,
                device_category: None,
                confidence: 0.0,
            }).collect());
        }
        
        let prompt = self.build_batch_prompt(devices);
        let response = self.query_llm(&prompt).await?;
        
        // Parse response - expect one line per device
        let lines: Vec<&str> = response.lines().collect();
        let mut results = Vec::new();
        
        for (i, device) in devices.iter().enumerate() {
            let desc = lines.get(i).map(|s| s.to_string())
                .unwrap_or_else(|| "Analysis unavailable".to_string());
            
            results.push(DeviceAnalysis {
                description: desc,
                threat_assessment: None,
                device_category: self.guess_category(device),
                confidence: 0.7,
            });
        }
        
        Ok(results)
    }
    
    fn build_device_prompt(&self, device: &DeviceQuery) -> String {
        let mut info = format!("MAC: {}", device.mac_address);
        
        if let Some(ref name) = device.device_name {
            info.push_str(&format!(", Name: {}", name));
        }
        if let Some(ref vendor) = device.vendor {
            info.push_str(&format!(", Vendor: {}", vendor));
        }
        if let Some(ref ssid) = device.ssid {
            info.push_str(&format!(", SSID: {}", ssid));
        }
        if device.is_tracker {
            info.push_str(", TYPE: TRACKER/AIRTAG");
        }
        
        format!(
            "You are a wireless security analyst. In ONE brief sentence (max 20 words), \
            identify this {} device and note any security concerns:\n\n{}\n\n\
            Format: [Device type] - [Brief description]",
            device.device_type, info
        )
    }
    
    fn build_batch_prompt(&self, devices: &[DeviceQuery]) -> String {
        let mut device_list = String::new();
        
        for (i, device) in devices.iter().enumerate() {
            device_list.push_str(&format!("{}. ", i + 1));
            
            if let Some(ref name) = device.device_name {
                device_list.push_str(&format!("{} ", name));
            }
            device_list.push_str(&format!("({}) ", device.mac_address));
            
            if let Some(ref vendor) = device.vendor {
                device_list.push_str(&format!("- {} ", vendor));
            }
            if device.is_tracker {
                device_list.push_str("[TRACKER] ");
            }
            device_list.push('\n');
        }
        
        format!(
            "You are a wireless security analyst. For each device below, provide a ONE LINE \
            explanation (max 15 words) identifying the device type and any concerns.\n\n\
            Devices:\n{}\n\
            Respond with numbered lines matching the input.",
            device_list
        )
    }
    
    async fn query_llm(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/v1/chat/completions", self.config.endpoint);
        
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            max_tokens: self.config.max_tokens,
        };
        
        let mut req = self.client.post(&url)
            .header("Content-Type", "application/json");
        
        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }
        
        debug!("Querying LLM at {}", url);
        
        let response = req
            .json(&request)
            .send()
            .await
            .context("Failed to connect to LLM")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("LLM API error: {} - {}", status, body);
            anyhow::bail!("LLM API error: {}", status);
        }
        
        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse LLM response")?;
        
        let content = chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No response".to_string());
        
        Ok(content)
    }
    
    fn guess_category(&self, device: &DeviceQuery) -> Option<String> {
        if device.is_tracker {
            return Some("Tracker".to_string());
        }
        
        if let Some(ref name) = device.device_name {
            let name_lower = name.to_lowercase();
            if name_lower.contains("phone") || name_lower.contains("iphone") || name_lower.contains("galaxy") {
                return Some("Smartphone".to_string());
            }
            if name_lower.contains("watch") || name_lower.contains("band") {
                return Some("Wearable".to_string());
            }
            if name_lower.contains("tv") || name_lower.contains("roku") || name_lower.contains("fire") {
                return Some("Smart TV/Streaming".to_string());
            }
        }
        
        None
    }
}
