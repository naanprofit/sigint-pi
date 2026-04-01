//! LLM Client for device analysis
//! 
//! Supports multiple LLM providers:
//! - LOCAL (RECOMMENDED): Ollama, llama.cpp - Privacy-preserving, no data leaves device
//! - Cloud: Anthropic Claude, xAI Grok, OpenAI
//! 
//! PRIVACY NOTE: Local LLMs are strongly recommended for security-sensitive operations.
//! Cloud providers receive device MAC addresses and network information.
//! 
//! RECOMMENDED LOCAL MODELS (by device):
//! 
//! Steam Deck (16GB RAM, x86_64):
//!   - phi3:mini (3.8B, 2.3GB) - Best balance of speed/quality
//!   - llama3.2:3b (3B, 2GB) - Fast, good for device ID
//!   - qwen2.5:3b (3B, 2GB) - Excellent for technical queries
//!   - tinyllama (1.1B, 637MB) - Fastest, basic descriptions only
//!   - gemma2:2b (2B, 1.6GB) - Good quality, moderate speed
//! 
//! Raspberry Pi 5 (8GB RAM, ARM64):
//!   - tinyllama (1.1B, 637MB) - Recommended for Pi
//!   - phi3:mini (3.8B, 2.3GB) - Works but slower
//!   - qwen2.5:0.5b (0.5B, 400MB) - Fastest, limited capability
//! 
//! Raspberry Pi 4/Zero 2W (limited RAM):
//!   - tinyllama (1.1B) - Only viable option
//!   - Consider cloud fallback for complex queries

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

pub use crate::config::LlmConfig;

/// Supported LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    /// Local llama.cpp server (RECOMMENDED - privacy preserving)
    #[serde(alias = "llama.cpp", alias = "llama")]
    LlamaCpp,
    /// Local Ollama server (RECOMMENDED - privacy preserving)
    Ollama,
    /// OpenAI API (cloud - data sent to OpenAI)
    #[serde(alias = "openai")]
    OpenAI,
    /// Anthropic Claude (cloud - data sent to Anthropic)
    #[serde(alias = "anthropic", alias = "claude")]
    Anthropic,
    /// xAI Grok (cloud - data sent to xAI)
    #[serde(alias = "xai", alias = "grok")]
    XAI,
}

impl Default for LlmProvider {
    fn default() -> Self {
        LlmProvider::Ollama // Default to local
    }
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmProvider::LlamaCpp => write!(f, "llama.cpp (local)"),
            LlmProvider::Ollama => write!(f, "Ollama (local)"),
            LlmProvider::OpenAI => write!(f, "OpenAI (cloud)"),
            LlmProvider::Anthropic => write!(f, "Anthropic Claude (cloud)"),
            LlmProvider::XAI => write!(f, "xAI Grok (cloud)"),
        }
    }
}

impl LlmProvider {
    pub fn is_local(&self) -> bool {
        matches!(self, LlmProvider::LlamaCpp | LlmProvider::Ollama)
    }
    
    pub fn is_cloud(&self) -> bool {
        !self.is_local()
    }
    
    pub fn default_endpoint(&self) -> &'static str {
        match self {
            LlmProvider::LlamaCpp => "http://localhost:8080",
            LlmProvider::Ollama => "http://localhost:11434",
            LlmProvider::OpenAI => "https://api.openai.com",
            LlmProvider::Anthropic => "https://api.anthropic.com",
            LlmProvider::XAI => "https://api.x.ai",
        }
    }
    
    pub fn default_model(&self) -> &'static str {
        match self {
            LlmProvider::LlamaCpp => "local",
            LlmProvider::Ollama => "phi3:mini",
            LlmProvider::OpenAI => "gpt-4o-mini",
            LlmProvider::Anthropic => "claude-3-haiku-20240307",
            LlmProvider::XAI => "grok-2",
        }
    }
    
    pub fn requires_api_key(&self) -> bool {
        self.is_cloud()
    }
    
    /// Privacy warning for cloud providers
    pub fn privacy_warning(&self) -> Option<&'static str> {
        match self {
            LlmProvider::OpenAI => Some(
                "WARNING: Using OpenAI. Device MACs and network data will be sent to OpenAI servers."
            ),
            LlmProvider::Anthropic => Some(
                "WARNING: Using Anthropic Claude. Device MACs and network data will be sent to Anthropic servers."
            ),
            LlmProvider::XAI => Some(
                "WARNING: Using xAI Grok. Device MACs and network data will be sent to xAI/X Corp servers."
            ),
            _ => None,
        }
    }
}

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

// OpenAI-compatible request/response (works for llama.cpp, Ollama, OpenAI, xAI)
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    content: String,
}

// Anthropic-specific request/response
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    text: String,
}

pub struct LlmClient {
    config: LlmConfig,
    provider: LlmProvider,
    client: reqwest::Client,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        let provider = Self::parse_provider(&config.provider);
        
        // Log privacy warning for cloud providers
        if let Some(warning) = provider.privacy_warning() {
            warn!("{}", warning);
        }
        
        if provider.is_local() {
            info!("Using local LLM provider: {} - Data stays on device", provider);
        }
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { config, provider, client }
    }
    
    fn parse_provider(provider_str: &str) -> LlmProvider {
        match provider_str.to_lowercase().as_str() {
            "llamacpp" | "llama.cpp" | "llama" | "local" => LlmProvider::LlamaCpp,
            "ollama" => LlmProvider::Ollama,
            "openai" | "gpt" => LlmProvider::OpenAI,
            "anthropic" | "claude" => LlmProvider::Anthropic,
            "xai" | "grok" | "x" => LlmProvider::XAI,
            // LMStudio uses OpenAI-compatible API
            "lmstudio" | "lm_studio" | "lm-studio" => LlmProvider::OpenAI,
            // Custom/generic OpenAI-compatible
            "custom" | "openai-compatible" => LlmProvider::OpenAI,
            _ => {
                warn!("Unknown LLM provider '{}', treating as OpenAI-compatible", provider_str);
                LlmProvider::OpenAI
            }
        }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    pub fn provider(&self) -> LlmProvider {
        self.provider
    }
    
    pub fn is_local(&self) -> bool {
        self.provider.is_local()
    }
    
    /// Check if the LLM server is reachable
    pub async fn health_check(&self) -> Result<bool> {
        let url = match self.provider {
            LlmProvider::LlamaCpp => format!("{}/health", self.config.endpoint),
            LlmProvider::Ollama => format!("{}/api/tags", self.config.endpoint),
            LlmProvider::OpenAI => format!("{}/v1/models", self.config.endpoint),
            LlmProvider::Anthropic => return Ok(true), // No simple health endpoint
            LlmProvider::XAI => format!("{}/v1/models", self.config.endpoint),
        };
        
        let mut req = self.client.get(&url);
        if let Some(ref api_key) = self.config.api_key {
            req = self.add_auth_header(req, api_key);
        }
        
        match req.send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
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
            threat_assessment: self.extract_threat_assessment(device),
            device_category: self.guess_category(device),
            confidence: 0.8,
        })
    }
    
    /// Analyze multiple devices in one request
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
        
        let lines: Vec<&str> = response.lines().collect();
        let mut results = Vec::new();
        
        for (i, device) in devices.iter().enumerate() {
            let desc = lines.get(i)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Analysis unavailable".to_string());
            
            results.push(DeviceAnalysis {
                description: desc,
                threat_assessment: self.extract_threat_assessment(device),
                device_category: self.guess_category(device),
                confidence: 0.7,
            });
        }
        
        Ok(results)
    }
    
    /// Interactive help query
    pub async fn ask_help(&self, question: &str) -> Result<String> {
        if !self.config.enabled {
            return Ok("AI assistant disabled. Enable LLM in settings for help.".to_string());
        }
        
        let prompt = format!(
            "You are SIGINT-Deck's security assistant. Answer this question concisely (2-3 sentences max):\n\n{}",
            question
        );
        
        self.query_llm(&prompt).await
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
        match self.provider {
            LlmProvider::Anthropic => self.query_anthropic(prompt).await,
            _ => self.query_openai_compatible(prompt).await,
        }
    }
    
    /// Query OpenAI-compatible endpoints (llama.cpp, Ollama, OpenAI, xAI)
    async fn query_openai_compatible(&self, prompt: &str) -> Result<String> {
        let url = match self.provider {
            LlmProvider::Ollama => format!("{}/v1/chat/completions", self.config.endpoint),
            LlmProvider::LlamaCpp => format!("{}/v1/chat/completions", self.config.endpoint),
            LlmProvider::OpenAI => format!("{}/v1/chat/completions", self.config.endpoint),
            LlmProvider::XAI => format!("{}/v1/chat/completions", self.config.endpoint),
            _ => unreachable!(),
        };
        
        let request = OpenAIRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            max_tokens: self.config.max_tokens,
            temperature: Some(0.3), // Low temperature for factual responses
        };
        
        let mut req = self.client.post(&url)
            .header("Content-Type", "application/json");
        
        if let Some(ref api_key) = self.config.api_key {
            req = self.add_auth_header(req, api_key);
        }
        
        debug!("Querying {} at {}", self.provider, url);
        
        let response = req
            .json(&request)
            .send()
            .await
            .context(format!("Failed to connect to {}", self.provider))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("{} API error: {} - {}", self.provider, status, body);
            anyhow::bail!("{} API error: {}", self.provider, status);
        }
        
        let chat_response: OpenAIResponse = response
            .json()
            .await
            .context("Failed to parse LLM response")?;
        
        let content = chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No response".to_string());
        
        Ok(content)
    }
    
    /// Query Anthropic Claude API
    async fn query_anthropic(&self, prompt: &str) -> Result<String> {
        let url = format!("{}/v1/messages", self.config.endpoint);
        
        let request = AnthropicRequest {
            model: self.config.model.clone(),
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            max_tokens: self.config.max_tokens,
        };
        
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Anthropic requires API key"))?;
        
        debug!("Querying Anthropic Claude at {}", url);
        
        let response = self.client.post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Anthropic")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Anthropic API error: {} - {}", status, body);
            anyhow::bail!("Anthropic API error: {}", status);
        }
        
        let chat_response: AnthropicResponse = response
            .json()
            .await
            .context("Failed to parse Anthropic response")?;
        
        let content = chat_response.content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_else(|| "No response".to_string());
        
        Ok(content)
    }
    
    fn add_auth_header(&self, req: reqwest::RequestBuilder, api_key: &str) -> reqwest::RequestBuilder {
        match self.provider {
            LlmProvider::Anthropic => {
                req.header("x-api-key", api_key)
                   .header("anthropic-version", "2023-06-01")
            }
            _ => {
                req.header("Authorization", format!("Bearer {}", api_key))
            }
        }
    }
    
    fn extract_threat_assessment(&self, device: &DeviceQuery) -> Option<String> {
        if device.is_tracker {
            return Some("CRITICAL: Tracking device detected. May indicate surveillance.".to_string());
        }
        
        // Check vendor against threat intel
        if let Some(ref vendor) = device.vendor {
            let vendor_lower = vendor.to_lowercase();
            if vendor_lower.contains("hikvision") || vendor_lower.contains("dahua") {
                return Some("HIGH: Chinese state-linked surveillance camera manufacturer".to_string());
            }
            if vendor_lower.contains("harris") {
                return Some("CRITICAL: Harris Corporation - Stingray/IMSI catcher manufacturer".to_string());
            }
            if vendor_lower.contains("palantir") {
                return Some("HIGH: Palantir - Intelligence/surveillance data analytics".to_string());
            }
        }
        
        None
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
            if name_lower.contains("led") || name_lower.contains("light") || name_lower.contains("bulb") {
                return Some("Smart Lighting".to_string());
            }
        }
        
        None
    }
}

/// Get recommended local model for the current platform
pub fn get_recommended_local_model() -> (&'static str, &'static str) {
    #[cfg(target_arch = "x86_64")]
    {
        // Steam Deck or desktop - more RAM available
        ("phi3:mini", "Best balance of speed and quality for x86_64 (2.3GB)")
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        // Raspberry Pi 5 or similar ARM64
        ("tinyllama", "Lightweight model for ARM64 devices (637MB)")
    }
    
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        ("tinyllama", "Lightweight model for limited hardware (637MB)")
    }
}

/// Print local LLM setup instructions
pub fn print_local_llm_guide() {
    let (model, desc) = get_recommended_local_model();
    
    println!("
╔══════════════════════════════════════════════════════════════════════╗
║                    LOCAL LLM SETUP GUIDE                             ║
║                                                                       ║
║  For maximum privacy, run AI analysis locally on your device.        ║
║  No data leaves your system when using local LLMs.                   ║
╚══════════════════════════════════════════════════════════════════════╝

RECOMMENDED FOR YOUR DEVICE:
  Model: {model}
  {desc}

OPTION 1: Ollama (Easiest)
─────────────────────────────────────────────────────────────────────
  # Install Ollama
  curl -fsSL https://ollama.ai/install.sh | sh
  
  # Pull recommended model
  ollama pull {model}
  
  # Start server (runs on port 11434)
  ollama serve
  
  # Config:
  [llm]
  enabled = true
  provider = \"ollama\"
  endpoint = \"http://localhost:11434\"
  model = \"{model}\"

OPTION 2: llama.cpp (More Control)
─────────────────────────────────────────────────────────────────────
  # Download llama.cpp
  git clone https://github.com/ggerganov/llama.cpp
  cd llama.cpp && make -j
  
  # Download a GGUF model (example: Phi-3 Mini)
  wget https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf
  
  # Run server
  ./llama-server -m Phi-3-mini-4k-instruct-q4.gguf --port 8080
  
  # Config:
  [llm]
  enabled = true
  provider = \"llamacpp\"
  endpoint = \"http://localhost:8080\"
  model = \"local\"

MODELS BY HARDWARE:
─────────────────────────────────────────────────────────────────────
  Steam Deck (16GB RAM):
    - phi3:mini      (3.8B, 2.3GB)  ★ Recommended
    - llama3.2:3b    (3B, 2GB)      Fast, good quality
    - qwen2.5:3b     (3B, 2GB)      Technical queries
    - gemma2:2b      (2B, 1.6GB)    Balanced
    
  Raspberry Pi 5 (8GB):
    - tinyllama      (1.1B, 637MB)  ★ Recommended
    - phi3:mini      (3.8B, 2.3GB)  Slower but better
    - qwen2.5:0.5b   (0.5B, 400MB)  Fastest
    
  Pi 4/Zero 2W (4GB or less):
    - tinyllama      (1.1B, 637MB)  Only viable option

CLOUD PROVIDERS (Privacy Trade-off):
─────────────────────────────────────────────────────────────────────
  If local LLM isn't feasible, cloud options are available.
  WARNING: Device MACs and network data will be sent to cloud servers.
  
  Anthropic Claude:
    [llm]
    provider = \"anthropic\"
    endpoint = \"https://api.anthropic.com\"
    model = \"claude-3-haiku-20240307\"
    api_key = \"sk-ant-...\"
  
  xAI Grok:
    [llm]
    provider = \"xai\"
    endpoint = \"https://api.x.ai\"
    model = \"grok-2\"
    api_key = \"xai-...\"
  
  OpenAI:
    [llm]
    provider = \"openai\"
    endpoint = \"https://api.openai.com\"
    model = \"gpt-4o-mini\"
    api_key = \"sk-...\"
", model=model, desc=desc);
}
