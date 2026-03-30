//! Webhook integration for external services
//!
//! Supports generic webhooks, OpenClaw, and other HTTP-based alert systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info};

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub enabled: bool,
    pub endpoints: Vec<WebhookEndpoint>,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoints: Vec::new(),
        }
    }
}

/// Individual webhook endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub auth: Option<WebhookAuth>,
    pub payload_format: PayloadFormat,
    pub enabled: bool,
    pub min_priority: String,
    pub timeout_secs: u64,
    pub retry_count: u32,
    /// Custom payload template (use {{field}} for substitution)
    pub custom_template: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Post,
    Put,
    Patch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebhookAuth {
    Bearer { token: String },
    Basic { username: String, password: String },
    Header { name: String, value: String },
    ApiKey { key: String, header_name: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadFormat {
    Json,
    OpenClaw,
    Slack,
    Discord,
    Custom,
}

/// OpenClaw-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawConfig {
    pub enabled: bool,
    /// OpenClaw API endpoint (e.g., https://api.openclaw.example/v1/alerts)
    pub api_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Device identifier for this SIGINT-Pi instance
    pub device_id: String,
    /// Device name (human-readable)
    pub device_name: String,
    /// Tags to add to all alerts
    pub tags: Vec<String>,
    /// Include raw device data in alerts
    pub include_raw_data: bool,
    /// Minimum priority to send
    pub min_priority: String,
}

impl Default for OpenClawConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_url: "https://api.openclaw.io/v1/alerts".to_string(),
            api_key: String::new(),
            device_id: String::new(),
            device_name: "SIGINT-Pi".to_string(),
            tags: vec!["sigint".to_string(), "wireless".to_string()],
            include_raw_data: false,
            min_priority: "medium".to_string(),
        }
    }
}

/// Alert payload for OpenClaw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawPayload {
    pub device_id: String,
    pub device_name: String,
    pub alert_type: String,
    pub priority: String,
    pub title: String,
    pub message: String,
    pub timestamp: i64,
    pub location: Option<OpenClawLocation>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenClawLocation {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
}

/// Webhook client
pub struct WebhookClient {
    config: WebhookConfig,
    openclaw_config: OpenClawConfig,
    http_client: reqwest::Client,
}

impl WebhookClient {
    pub fn new(config: WebhookConfig, openclaw_config: OpenClawConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            config,
            openclaw_config,
            http_client,
        }
    }

    /// Send an alert to all configured webhooks
    pub async fn send_alert(&self, alert: &AlertData) -> Vec<WebhookResult> {
        let mut results = Vec::new();

        // Send to OpenClaw if enabled
        if self.openclaw_config.enabled {
            let result = self.send_to_openclaw(alert).await;
            results.push(WebhookResult {
                endpoint: "OpenClaw".to_string(),
                success: result.is_ok(),
                error: result.err(),
            });
        }

        // Send to generic webhooks
        if self.config.enabled {
            for endpoint in &self.config.endpoints {
                if !endpoint.enabled {
                    continue;
                }

                let result = self.send_to_endpoint(endpoint, alert).await;
                results.push(WebhookResult {
                    endpoint: endpoint.name.clone(),
                    success: result.is_ok(),
                    error: result.err(),
                });
            }
        }

        results
    }

    /// Send alert to OpenClaw
    async fn send_to_openclaw(&self, alert: &AlertData) -> Result<(), String> {
        let config = &self.openclaw_config;

        let payload = OpenClawPayload {
            device_id: config.device_id.clone(),
            device_name: config.device_name.clone(),
            alert_type: alert.alert_type.clone(),
            priority: alert.priority.clone(),
            title: alert.title.clone(),
            message: alert.message.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            location: alert.location.as_ref().map(|loc| OpenClawLocation {
                latitude: loc.latitude,
                longitude: loc.longitude,
                altitude: loc.altitude,
                accuracy: loc.accuracy,
            }),
            tags: config.tags.clone(),
            metadata: alert.metadata.clone(),
            raw_data: if config.include_raw_data {
                alert.raw_data.clone()
            } else {
                None
            },
        };

        let response = self.http_client
            .post(&config.api_url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .header("X-Device-ID", &config.device_id)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if response.status().is_success() {
            info!("Alert sent to OpenClaw: {}", alert.title);
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(format!("OpenClaw returned {}: {}", status, body))
        }
    }

    /// Send alert to a generic webhook endpoint
    async fn send_to_endpoint(
        &self,
        endpoint: &WebhookEndpoint,
        alert: &AlertData,
    ) -> Result<(), String> {
        let payload = match endpoint.payload_format {
            PayloadFormat::Json => serde_json::to_value(alert)
                .map_err(|e| format!("JSON serialization failed: {}", e))?,
            PayloadFormat::OpenClaw => {
                serde_json::to_value(OpenClawPayload {
                    device_id: "sigint-pi".to_string(),
                    device_name: "SIGINT-Pi".to_string(),
                    alert_type: alert.alert_type.clone(),
                    priority: alert.priority.clone(),
                    title: alert.title.clone(),
                    message: alert.message.clone(),
                    timestamp: chrono::Utc::now().timestamp(),
                    location: None,
                    tags: vec![],
                    metadata: alert.metadata.clone(),
                    raw_data: None,
                }).map_err(|e| format!("JSON serialization failed: {}", e))?
            }
            PayloadFormat::Slack => serde_json::json!({
                "text": format!("*{}*\n{}", alert.title, alert.message),
                "attachments": [{
                    "color": match alert.priority.as_str() {
                        "critical" => "danger",
                        "high" => "warning",
                        _ => "good"
                    },
                    "fields": [
                        {"title": "Type", "value": alert.alert_type, "short": true},
                        {"title": "Priority", "value": alert.priority, "short": true}
                    ]
                }]
            }),
            PayloadFormat::Discord => serde_json::json!({
                "embeds": [{
                    "title": alert.title,
                    "description": alert.message,
                    "color": match alert.priority.as_str() {
                        "critical" => 0xFF0000,
                        "high" => 0xFFA500,
                        "medium" => 0xFFFF00,
                        _ => 0x00FF00
                    },
                    "fields": [
                        {"name": "Type", "value": alert.alert_type, "inline": true},
                        {"name": "Priority", "value": alert.priority, "inline": true}
                    ],
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }]
            }),
            PayloadFormat::Custom => {
                if let Some(template) = &endpoint.custom_template {
                    let rendered = template
                        .replace("{{title}}", &alert.title)
                        .replace("{{message}}", &alert.message)
                        .replace("{{priority}}", &alert.priority)
                        .replace("{{type}}", &alert.alert_type)
                        .replace("{{timestamp}}", &chrono::Utc::now().to_rfc3339());
                    serde_json::from_str(&rendered)
                        .unwrap_or_else(|_| serde_json::Value::String(rendered))
                } else {
                    serde_json::to_value(alert)
                        .map_err(|e| format!("JSON serialization failed: {}", e))?
                }
            }
        };

        let mut request = match endpoint.method {
            HttpMethod::Post => self.http_client.post(&endpoint.url),
            HttpMethod::Put => self.http_client.put(&endpoint.url),
            HttpMethod::Patch => self.http_client.patch(&endpoint.url),
        };

        // Add headers
        for (key, value) in &endpoint.headers {
            request = request.header(key.as_str(), value.as_str());
        }

        // Add authentication
        if let Some(auth) = &endpoint.auth {
            request = match auth {
                WebhookAuth::Bearer { token } => {
                    request.header("Authorization", format!("Bearer {}", token))
                }
                WebhookAuth::Basic { username, password } => {
                    request.basic_auth(username, Some(password))
                }
                WebhookAuth::Header { name, value } => {
                    request.header(name.as_str(), value.as_str())
                }
                WebhookAuth::ApiKey { key, header_name } => {
                    request.header(header_name.as_str(), key.as_str())
                }
            };
        }

        let response = request
            .json(&payload)
            .timeout(std::time::Duration::from_secs(endpoint.timeout_secs))
            .send()
            .await
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if response.status().is_success() {
            info!("Alert sent to webhook '{}': {}", endpoint.name, alert.title);
            Ok(())
        } else {
            let status = response.status();
            Err(format!("Webhook returned {}", status))
        }
    }

    /// Test webhook connectivity
    pub async fn test_endpoint(&self, endpoint_name: &str) -> Result<String, String> {
        let endpoint = self.config.endpoints
            .iter()
            .find(|e| e.name == endpoint_name)
            .ok_or_else(|| format!("Endpoint '{}' not found", endpoint_name))?;

        let test_alert = AlertData {
            alert_type: "test".to_string(),
            priority: "low".to_string(),
            title: "SIGINT-Pi Test Alert".to_string(),
            message: "This is a test message to verify webhook connectivity.".to_string(),
            location: None,
            metadata: HashMap::new(),
            raw_data: None,
        };

        self.send_to_endpoint(endpoint, &test_alert).await?;
        Ok(format!("Test alert sent to '{}'", endpoint_name))
    }

    /// Test OpenClaw connectivity
    pub async fn test_openclaw(&self) -> Result<String, String> {
        if !self.openclaw_config.enabled {
            return Err("OpenClaw not enabled".to_string());
        }

        let test_alert = AlertData {
            alert_type: "test".to_string(),
            priority: "low".to_string(),
            title: "SIGINT-Pi Connection Test".to_string(),
            message: "Testing connectivity from SIGINT-Pi device.".to_string(),
            location: None,
            metadata: HashMap::new(),
            raw_data: None,
        };

        self.send_to_openclaw(&test_alert).await?;
        Ok("Test alert sent to OpenClaw".to_string())
    }
}

/// Alert data for webhooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertData {
    pub alert_type: String,
    pub priority: String,
    pub title: String,
    pub message: String,
    pub location: Option<LocationData>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub raw_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
}

/// Result of webhook delivery
#[derive(Debug, Clone)]
pub struct WebhookResult {
    pub endpoint: String,
    pub success: bool,
    pub error: Option<String>,
}

/// OpenClaw integration documentation
pub fn get_openclaw_docs() -> &'static str {
    r#"
OpenClaw Integration
=====================

OpenClaw is a centralized alert aggregation platform that can receive alerts 
from multiple SIGINT-Pi devices and provide unified monitoring and analysis.

Configuration
-------------

Add the following to your config.toml:

[alerts.openclaw]
enabled = true
api_url = "https://api.openclaw.io/v1/alerts"
api_key = "your-api-key-here"
device_id = "sigint-pi-001"
device_name = "SIGINT-Pi Living Room"
tags = ["home", "primary"]
include_raw_data = false
min_priority = "medium"

API Key Setup
-------------

1. Create an account at https://openclaw.io (or your self-hosted instance)
2. Navigate to Settings -> API Keys
3. Create a new API key with "alerts:write" scope
4. Copy the key to your config.toml

Alert Payload Format
--------------------

SIGINT-Pi sends alerts in the following JSON format:

{
    "device_id": "sigint-pi-001",
    "device_name": "SIGINT-Pi Living Room",
    "alert_type": "new_device|attack|tracker|geofence",
    "priority": "low|medium|high|critical",
    "title": "Alert Title",
    "message": "Detailed message",
    "timestamp": 1234567890,
    "location": {
        "latitude": 40.7128,
        "longitude": -74.0060,
        "altitude": null,
        "accuracy": 10.0
    },
    "tags": ["home", "primary"],
    "metadata": {
        "mac_address": "AA:BB:CC:DD:EE:FF",
        "vendor": "Apple Inc",
        "rssi": -45
    },
    "raw_data": null
}

Self-Hosted OpenClaw
--------------------

If running your own OpenClaw instance:

1. Deploy OpenClaw server (see OpenClaw documentation)
2. Update api_url to point to your instance
3. Generate API key from your instance admin panel

Multi-Device Setup
------------------

For multiple SIGINT-Pi devices reporting to one OpenClaw instance:

1. Use unique device_id for each device
2. Use descriptive device_name for identification  
3. Use tags to group devices by location/purpose

Testing
-------

Test your OpenClaw connection:

curl -X POST https://api.openclaw.io/v1/alerts \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"device_id":"test","alert_type":"test","title":"Test","message":"Test message","timestamp":0}'
"#
}
