use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::config::RayHunterConfig;
use crate::ScanEvent;

/// RayHunter analysis result from the EFF RayHunter app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayHunterAnalysis {
    /// Current analysis status
    pub status: String,
    /// Whether IMSI catcher activity is suspected
    pub imsi_catcher_detected: bool,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Detailed findings
    pub findings: Vec<RayHunterFinding>,
    /// Current cell info
    pub cell_info: Option<CellInfo>,
    /// Timestamp of analysis
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayHunterFinding {
    pub finding_type: String,
    pub severity: String, // "info", "warning", "critical"
    pub description: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellInfo {
    pub mcc: Option<u16>,  // Mobile Country Code
    pub mnc: Option<u16>,  // Mobile Network Code
    pub lac: Option<u32>,  // Location Area Code
    pub cid: Option<u32>,  // Cell ID
    pub arfcn: Option<u32>, // Absolute Radio Frequency Channel Number
    pub signal_strength: Option<i32>,
    pub technology: Option<String>, // "GSM", "LTE", "5G"
}

/// RayHunter alert for broadcasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayHunterAlert {
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub analysis: RayHunterAnalysis,
    pub timestamp: DateTime<Utc>,
}

pub struct RayHunterClient {
    config: RayHunterConfig,
    client: reqwest::Client,
    last_analysis: Option<RayHunterAnalysis>,
}

impl RayHunterClient {
    pub fn new(config: RayHunterConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config,
            client,
            last_analysis: None,
        }
    }

    /// Check if RayHunter is available
    pub async fn check_connection(&self) -> Result<bool> {
        let url = format!("{}/api/status", self.config.api_url);
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Get current analysis from RayHunter
    pub async fn get_analysis(&self) -> Result<RayHunterAnalysis> {
        let url = format!("{}/api/analysis", self.config.api_url);
        
        let response = self.client.get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("RayHunter API returned {}", response.status());
        }

        // RayHunter returns JSON with analysis data
        // This is based on the expected RayHunter API format
        let data: serde_json::Value = response.json().await?;
        
        // Parse RayHunter response into our structure
        let analysis = self.parse_rayhunter_response(data)?;
        
        Ok(analysis)
    }

    fn parse_rayhunter_response(&self, data: serde_json::Value) -> Result<RayHunterAnalysis> {
        // RayHunter API response format (based on EFF RayHunter)
        let status = data.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let mut findings = Vec::new();
        let mut imsi_catcher_detected = false;
        let mut confidence = 0.0;

        // Check for suspicious indicators
        if let Some(analysis) = data.get("analysis") {
            if let Some(suspicious) = analysis.get("suspicious") {
                imsi_catcher_detected = suspicious.as_bool().unwrap_or(false);
            }
            
            if let Some(conf) = analysis.get("confidence") {
                confidence = conf.as_f64().unwrap_or(0.0);
            }

            // Parse individual findings
            if let Some(items) = analysis.get("findings").and_then(|f| f.as_array()) {
                for item in items {
                    findings.push(RayHunterFinding {
                        finding_type: item.get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        severity: item.get("severity")
                            .and_then(|v| v.as_str())
                            .unwrap_or("info")
                            .to_string(),
                        description: item.get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        details: item.get("details")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                    });
                }
            }
        }

        // Parse cell info
        let cell_info = data.get("cell").map(|cell| CellInfo {
            mcc: cell.get("mcc").and_then(|v| v.as_u64()).map(|v| v as u16),
            mnc: cell.get("mnc").and_then(|v| v.as_u64()).map(|v| v as u16),
            lac: cell.get("lac").and_then(|v| v.as_u64()).map(|v| v as u32),
            cid: cell.get("cid").and_then(|v| v.as_u64()).map(|v| v as u32),
            arfcn: cell.get("arfcn").and_then(|v| v.as_u64()).map(|v| v as u32),
            signal_strength: cell.get("signal").and_then(|v| v.as_i64()).map(|v| v as i32),
            technology: cell.get("technology").and_then(|v| v.as_str()).map(String::from),
        });

        Ok(RayHunterAnalysis {
            status,
            imsi_catcher_detected,
            confidence,
            findings,
            cell_info,
            timestamp: Utc::now(),
        })
    }

    /// Start polling RayHunter for updates
    pub async fn run(&mut self, tx: broadcast::Sender<ScanEvent>) {
        info!("Starting RayHunter client");
        
        // Check initial connection
        match self.check_connection().await {
            Ok(true) => info!("RayHunter connected at {}", self.config.api_url),
            Ok(false) => {
                warn!("RayHunter not responding at {}", self.config.api_url);
                return;
            }
            Err(e) => {
                error!("Failed to connect to RayHunter: {}", e);
                return;
            }
        }

        let poll_interval = Duration::from_secs(self.config.poll_interval_secs);
        let mut interval = tokio::time::interval(poll_interval);

        loop {
            interval.tick().await;

            match self.get_analysis().await {
                Ok(analysis) => {
                    // Check if this is a new detection
                    let is_new_detection = self.is_new_detection(&analysis);
                    
                    // Update last analysis
                    self.last_analysis = Some(analysis.clone());

                    // Send update event
                    let _ = tx.send(ScanEvent::RayHunterUpdate(analysis.clone()));

                    // If IMSI catcher detected and it's new, send alert
                    if analysis.imsi_catcher_detected && is_new_detection {
                        let alert = RayHunterAlert {
                            alert_type: "imsi_catcher".to_string(),
                            severity: "critical".to_string(),
                            message: self.format_alert_message(&analysis),
                            analysis: analysis.clone(),
                            timestamp: Utc::now(),
                        };
                        
                        let _ = tx.send(ScanEvent::RayHunterAlert(alert));
                        
                        warn!("⚠️ IMSI CATCHER DETECTED! Confidence: {:.1}%", 
                              analysis.confidence * 100.0);
                    }
                }
                Err(e) => {
                    debug!("Failed to get RayHunter analysis: {}", e);
                }
            }
        }
    }

    fn is_new_detection(&self, current: &RayHunterAnalysis) -> bool {
        match &self.last_analysis {
            Some(last) => {
                // New detection if we weren't detecting before
                !last.imsi_catcher_detected && current.imsi_catcher_detected
            }
            None => current.imsi_catcher_detected,
        }
    }

    fn format_alert_message(&self, analysis: &RayHunterAnalysis) -> String {
        if let Some(template) = &self.config.alert_template {
            // Use custom template
            let mut msg = template.clone();
            msg = msg.replace("{confidence}", &format!("{:.1}", analysis.confidence * 100.0));
            msg = msg.replace("{status}", &analysis.status);
            if let Some(cell) = &analysis.cell_info {
                msg = msg.replace("{mcc}", &cell.mcc.map(|v| v.to_string()).unwrap_or_default());
                msg = msg.replace("{mnc}", &cell.mnc.map(|v| v.to_string()).unwrap_or_default());
                msg = msg.replace("{cid}", &cell.cid.map(|v| v.to_string()).unwrap_or_default());
            }
            msg
        } else {
            // Default message
            let mut msg = format!(
                "🚨 IMSI CATCHER DETECTED\nConfidence: {:.1}%\n",
                analysis.confidence * 100.0
            );

            if let Some(cell) = &analysis.cell_info {
                if let (Some(mcc), Some(mnc)) = (cell.mcc, cell.mnc) {
                    msg.push_str(&format!("Network: {}-{}\n", mcc, mnc));
                }
                if let Some(cid) = cell.cid {
                    msg.push_str(&format!("Cell ID: {}\n", cid));
                }
                if let Some(tech) = &cell.technology {
                    msg.push_str(&format!("Technology: {}\n", tech));
                }
            }

            for finding in &analysis.findings {
                if finding.severity == "critical" || finding.severity == "warning" {
                    msg.push_str(&format!("• {}\n", finding.description));
                }
            }

            msg
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rayhunter_response() {
        let config = RayHunterConfig::default();
        let client = RayHunterClient::new(config);
        
        let json = serde_json::json!({
            "status": "analyzing",
            "analysis": {
                "suspicious": true,
                "confidence": 0.85,
                "findings": [
                    {
                        "type": "frequency_change",
                        "severity": "warning",
                        "description": "Unusual frequency hop detected"
                    }
                ]
            },
            "cell": {
                "mcc": 310,
                "mnc": 410,
                "cid": 12345,
                "technology": "LTE"
            }
        });

        let analysis = client.parse_rayhunter_response(json).unwrap();
        assert!(analysis.imsi_catcher_detected);
        assert!((analysis.confidence - 0.85).abs() < 0.01);
        assert_eq!(analysis.findings.len(), 1);
    }
}
