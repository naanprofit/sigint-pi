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
    pub status: String,
    pub imsi_catcher_detected: bool,
    pub confidence: f64,
    pub findings: Vec<RayHunterFinding>,
    pub cell_info: Option<CellInfo>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RayHunterFinding {
    pub finding_type: String,
    pub severity: String,
    pub description: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellInfo {
    pub mcc: Option<u16>,
    pub mnc: Option<u16>,
    pub lac: Option<u32>,
    pub cid: Option<u32>,
    pub arfcn: Option<u32>,
    pub signal_strength: Option<i32>,
    pub technology: Option<String>,
}

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

    pub async fn check_connection(&self) -> Result<bool> {
        let url = format!("{}/api/system-stats", self.config.api_url.trim_end_matches('/'));
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Ensure ADB port forward is set up (RayHunter runs on phone port 8080)
    pub async fn ensure_adb_forward(&self) -> bool {
        let port = self.config.api_url.split(':').last()
            .and_then(|p| p.trim_matches('/').parse::<u16>().ok())
            .unwrap_or(8081);
        
        // Use full path and ensure HOME is set for ADB auth keys
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/pi".to_string());
        let adb_path = if std::path::Path::new("/usr/bin/adb").exists() {
            "/usr/bin/adb"
        } else if std::path::Path::new("/usr/local/bin/adb").exists() {
            "/usr/local/bin/adb"
        } else {
            "adb"
        };
        
        let adb_check = tokio::process::Command::new(adb_path)
            .env("HOME", &home)
            .env("ANDROID_SDK_HOME", &home)
            .args(&["devices"])
            .output().await;
        
        let has_device = match &adb_check {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.lines().any(|l| l.ends_with("\tdevice") || l.contains("\tdevice"))
            }
            Err(e) => {
                debug!("ADB command failed: {}", e);
                false
            }
        };
        
        if !has_device {
            debug!("No ADB device connected for RayHunter");
            return false;
        }
        
        let fwd = tokio::process::Command::new(adb_path)
            .env("HOME", &home)
            .env("ANDROID_SDK_HOME", &home)
            .args(&["forward", &format!("tcp:{}", port), "tcp:8080"])
            .output().await;
        
        match fwd {
            Ok(out) if out.status.success() => {
                info!("ADB forward tcp:{} -> tcp:8080 established", port);
                true
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                debug!("ADB forward failed: {}", stderr);
                false
            }
            Err(e) => {
                debug!("ADB forward error: {}", e);
                false
            }
        }
    }

    /// Get system stats from RayHunter
    pub async fn get_system_stats(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/system-stats", self.config.api_url.trim_end_matches('/'));
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("RayHunter system-stats returned {}", resp.status());
        }
        Ok(resp.json().await?)
    }

    /// Get QMDL manifest (recording entries)
    pub async fn get_manifest(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/qmdl-manifest", self.config.api_url.trim_end_matches('/'));
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("RayHunter qmdl-manifest returned {}", resp.status());
        }
        Ok(resp.json().await?)
    }

    /// Get live analysis report
    pub async fn get_live_analysis(&self) -> Result<RayHunterAnalysis> {
        let base = self.config.api_url.trim_end_matches('/');
        
        // Get the manifest to find the current entry name
        let manifest: serde_json::Value = self.get_manifest().await?;
        let entry_name = manifest.get("current_entry")
            .and_then(|e| e.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("live");
        
        // Get analysis report for the current entry
        let url = format!("{}/api/analysis-report/{}", base, entry_name);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("RayHunter analysis-report returned {}", resp.status());
        }
        let body = resp.text().await?;
        
        // Parse newline-delimited JSON
        let mut warnings = Vec::new();
        let mut last_timestamp = String::new();
        
        for line in body.lines() {
            if line.is_empty() { continue; }
            if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(ts) = entry.get("timestamp").and_then(|t| t.as_str()) {
                    last_timestamp = ts.to_string();
                }
                if let Some(analysis) = entry.get("analysis").and_then(|a| a.as_array()) {
                    for warning in analysis {
                        if let Some(events) = warning.get("events").and_then(|e| e.as_array()) {
                            for event in events {
                                if event.is_null() { continue; }
                                let msg = event.get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("Unknown warning");
                                warnings.push(RayHunterFinding {
                                    finding_type: "imsi_catcher".to_string(),
                                    severity: "critical".to_string(),
                                    description: msg.to_string(),
                                    details: Some(last_timestamp.clone()),
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // Also get the analyzer list from the first line
        let first_line = body.lines().next().unwrap_or("");
        let analyzers: Vec<String> = if let Ok(first) = serde_json::from_str::<serde_json::Value>(first_line) {
            first.get("analyzers").and_then(|a| a.as_array())
                .map(|arr| arr.iter().filter_map(|a| a.get("name").and_then(|n| n.as_str()).map(String::from)).collect())
                .unwrap_or_default()
        } else { vec![] };
        
        let has_warnings = !warnings.is_empty();
        let confidence = if has_warnings { 0.9 } else { 0.0 };
        
        Ok(RayHunterAnalysis {
            status: if has_warnings { "alert".to_string() } else { "monitoring".to_string() },
            imsi_catcher_detected: has_warnings,
            confidence,
            findings: warnings,
            cell_info: None,
            timestamp: Utc::now(),
        })
    }

    /// Get full status for the web UI
    pub async fn get_full_status(&self) -> serde_json::Value {
        // Ensure ADB forward is active
        let adb_ok = self.ensure_adb_forward().await;
        
        if !adb_ok {
            return serde_json::json!({
                "available": false,
                "connected": false,
                "error": "No ADB device found. Connect RayHunter phone via USB.",
                "hint": "The EFF RayHunter Orbic phone should be connected via USB"
            });
        }
        
        // Check connection
        match self.check_connection().await {
            Ok(true) => {},
            Ok(false) => {
                return serde_json::json!({
                    "available": true,
                    "connected": false,
                    "adb_connected": true,
                    "error": "ADB device found but RayHunter not responding. Is the daemon running?",
                    "hint": "Check if rayhunter-daemon is running on the phone"
                });
            }
            Err(e) => {
                return serde_json::json!({
                    "available": true,
                    "connected": false,
                    "adb_connected": true,
                    "error": format!("Connection error: {}", e)
                });
            }
        }
        
        // Get all data
        let system_stats = self.get_system_stats().await.ok();
        let manifest = self.get_manifest().await.ok();
        let analysis = self.get_live_analysis().await.ok();
        
        let current_entry = manifest.as_ref()
            .and_then(|m| m.get("current_entry").cloned());
        let entry_count = manifest.as_ref()
            .and_then(|m| m.get("entries").and_then(|e| e.as_array()).map(|a| a.len()))
            .unwrap_or(0);
        
        let warnings_count = analysis.as_ref()
            .map(|a| a.findings.len()).unwrap_or(0);
        let imsi_detected = analysis.as_ref()
            .map(|a| a.imsi_catcher_detected).unwrap_or(false);
        
        serde_json::json!({
            "available": true,
            "connected": true,
            "adb_connected": true,
            "version": current_entry.as_ref()
                .and_then(|e| e.get("rayhunter_version").and_then(|v| v.as_str()))
                .unwrap_or("unknown"),
            "system_os": current_entry.as_ref()
                .and_then(|e| e.get("system_os").and_then(|v| v.as_str()))
                .unwrap_or("unknown"),
            "recording": current_entry.is_some(),
            "current_entry": current_entry,
            "total_entries": entry_count,
            "system_stats": system_stats,
            "imsi_catcher_detected": imsi_detected,
            "warnings_count": warnings_count,
            "analysis": analysis,
            "threat_detected": imsi_detected,
            "analyzers": ["IMSI Requested", "Connection Release/Redirected Carrier 2G Downgrade", "LTE SIB 6/7 Downgrade"]
        })
    }

    pub async fn run(&mut self, tx: broadcast::Sender<ScanEvent>) {
        info!("Starting RayHunter client");
        
        if !self.ensure_adb_forward().await {
            warn!("No ADB device for RayHunter, will retry...");
        }
        
        match self.check_connection().await {
            Ok(true) => info!("RayHunter connected at {}", self.config.api_url),
            Ok(false) => {
                warn!("RayHunter not responding at {}. Will keep retrying.", self.config.api_url);
            }
            Err(e) => {
                error!("Failed to connect to RayHunter: {}", e);
            }
        }

        let poll_interval = Duration::from_secs(self.config.poll_interval_secs);
        let mut interval = tokio::time::interval(poll_interval);

        loop {
            interval.tick().await;
            
            // Re-ensure ADB forward (in case device was reconnected)
            let _ = self.ensure_adb_forward().await;

            match self.get_live_analysis().await {
                Ok(analysis) => {
                    let is_new_detection = self.is_new_detection(&analysis);
                    self.last_analysis = Some(analysis.clone());
                    let _ = tx.send(ScanEvent::RayHunterUpdate(analysis.clone()));

                    if analysis.imsi_catcher_detected && is_new_detection {
                        let alert = RayHunterAlert {
                            alert_type: "imsi_catcher".to_string(),
                            severity: "critical".to_string(),
                            message: self.format_alert_message(&analysis),
                            analysis: analysis.clone(),
                            timestamp: Utc::now(),
                        };
                        let _ = tx.send(ScanEvent::RayHunterAlert(alert));
                        warn!("IMSI CATCHER DETECTED! Confidence: {:.1}%", analysis.confidence * 100.0);
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
            Some(last) => !last.imsi_catcher_detected && current.imsi_catcher_detected,
            None => current.imsi_catcher_detected,
        }
    }

    fn format_alert_message(&self, analysis: &RayHunterAnalysis) -> String {
        let mut msg = format!(
            "IMSI CATCHER DETECTED\nConfidence: {:.1}%\nWarnings: {}\n",
            analysis.confidence * 100.0,
            analysis.findings.len()
        );
        for finding in &analysis.findings {
            msg.push_str(&format!("  - {}\n", finding.description));
        }
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_distance_basic() {
        let config = RayHunterConfig::default();
        let client = RayHunterClient::new(config);
        assert!(client.last_analysis.is_none());
    }
}
