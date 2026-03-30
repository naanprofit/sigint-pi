//! Signal Messenger integration
//!
//! Sends alerts via Signal using signal-cli or the Signal REST API.
//! Provides end-to-end encrypted alert delivery.

use serde::{Deserialize, Serialize};
use std::process::Command;
use tracing::{debug, error, info, warn};

/// Signal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalConfig {
    pub enabled: bool,
    /// Signal phone number (with country code, e.g., +1234567890)
    pub sender_number: String,
    /// Recipient phone numbers
    pub recipients: Vec<String>,
    /// Path to signal-cli binary
    pub signal_cli_path: String,
    /// Signal-cli config directory
    pub config_dir: String,
    /// Use JSON-RPC mode (signal-cli daemon)
    pub use_jsonrpc: bool,
    /// JSON-RPC socket path
    pub jsonrpc_socket: String,
    /// Alert priority threshold (only send alerts >= this priority)
    pub min_priority: AlertPriority,
    /// Include device details in messages
    pub verbose_messages: bool,
    /// Rate limit: max messages per hour
    pub rate_limit_per_hour: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertPriority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sender_number: String::new(),
            recipients: Vec::new(),
            signal_cli_path: "signal-cli".to_string(),
            config_dir: "/etc/sigint-pi/signal".to_string(),
            use_jsonrpc: false,
            jsonrpc_socket: "/var/run/signal-cli/socket".to_string(),
            min_priority: AlertPriority::High,
            verbose_messages: false,
            rate_limit_per_hour: 30,
        }
    }
}

/// Signal messenger client
pub struct SignalClient {
    config: SignalConfig,
    message_count: std::sync::Mutex<(u32, std::time::Instant)>,
}

impl SignalClient {
    pub fn new(config: SignalConfig) -> Self {
        Self {
            config,
            message_count: std::sync::Mutex::new((0, std::time::Instant::now())),
        }
    }

    /// Check if rate limit allows sending
    fn check_rate_limit(&self) -> bool {
        let mut guard = self.message_count.lock().unwrap();
        let (count, start_time) = &mut *guard;
        
        if start_time.elapsed().as_secs() >= 3600 {
            *count = 0;
            *start_time = std::time::Instant::now();
        }
        
        if *count >= self.config.rate_limit_per_hour {
            warn!("Signal rate limit reached ({}/hour)", self.config.rate_limit_per_hour);
            return false;
        }
        
        *count += 1;
        true
    }

    /// Send an alert via Signal
    pub async fn send_alert(
        &self,
        title: &str,
        message: &str,
        priority: AlertPriority,
    ) -> Result<(), String> {
        if !self.config.enabled {
            return Ok(());
        }

        if priority < self.config.min_priority {
            debug!("Alert priority {:?} below threshold {:?}", priority, self.config.min_priority);
            return Ok(());
        }

        if !self.check_rate_limit() {
            return Err("Rate limit exceeded".to_string());
        }

        let emoji = match priority {
            AlertPriority::Critical => "🚨",
            AlertPriority::High => "⚠️",
            AlertPriority::Medium => "📢",
            AlertPriority::Low => "ℹ️",
        };

        let full_message = format!(
            "{} SIGINT-Pi Alert\n\n{}\n\n{}",
            emoji, title, message
        );

        if self.config.use_jsonrpc {
            self.send_via_jsonrpc(&full_message).await
        } else {
            self.send_via_cli(&full_message).await
        }
    }

    /// Send message using signal-cli command line
    async fn send_via_cli(&self, message: &str) -> Result<(), String> {
        for recipient in &self.config.recipients {
            let result = Command::new(&self.config.signal_cli_path)
                .args([
                    "--config", &self.config.config_dir,
                    "-a", &self.config.sender_number,
                    "send",
                    "-m", message,
                    recipient,
                ])
                .output();

            match result {
                Ok(output) => {
                    if output.status.success() {
                        info!("Signal message sent to {}", recipient);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        error!("Signal send failed: {}", stderr);
                        return Err(format!("signal-cli error: {}", stderr));
                    }
                }
                Err(e) => {
                    error!("Failed to run signal-cli: {}", e);
                    return Err(format!("Failed to run signal-cli: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Send message using signal-cli JSON-RPC daemon
    async fn send_via_jsonrpc(&self, message: &str) -> Result<(), String> {
        use tokio::net::UnixStream;
        use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};

        let socket_path = &self.config.jsonrpc_socket;
        
        let stream = UnixStream::connect(socket_path)
            .await
            .map_err(|e| format!("Failed to connect to signal-cli socket: {}", e))?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        for recipient in &self.config.recipients {
            let request = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "send",
                "params": {
                    "account": self.config.sender_number,
                    "recipient": [recipient],
                    "message": message
                },
                "id": 1
            });

            let request_str = format!("{}\n", request);
            writer.write_all(request_str.as_bytes())
                .await
                .map_err(|e| format!("Failed to write to socket: {}", e))?;

            let mut response = String::new();
            reader.read_line(&mut response)
                .await
                .map_err(|e| format!("Failed to read response: {}", e))?;

            debug!("Signal JSON-RPC response: {}", response);
        }

        Ok(())
    }

    /// Test Signal configuration
    pub async fn test_connection(&self) -> Result<String, String> {
        if !self.config.enabled {
            return Err("Signal alerts not enabled".to_string());
        }

        if self.config.sender_number.is_empty() {
            return Err("Sender number not configured".to_string());
        }

        if self.config.recipients.is_empty() {
            return Err("No recipients configured".to_string());
        }

        // Try to get account info
        let output = Command::new(&self.config.signal_cli_path)
            .args([
                "--config", &self.config.config_dir,
                "-a", &self.config.sender_number,
                "listIdentities",
            ])
            .output()
            .map_err(|e| format!("Failed to run signal-cli: {}", e))?;

        if output.status.success() {
            Ok("Signal configuration valid".to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Signal-cli error: {}", stderr))
        }
    }
}

/// Signal setup instructions
pub fn get_setup_instructions() -> &'static str {
    r#"
Signal Messenger Setup Instructions
====================================

SIGINT-Pi uses signal-cli for Signal integration. Follow these steps:

1. Install signal-cli:
   # Download from https://github.com/AsamK/signal-cli/releases
   wget https://github.com/AsamK/signal-cli/releases/download/v0.13.2/signal-cli-0.13.2-Linux.tar.gz
   tar xf signal-cli-*.tar.gz
   sudo mv signal-cli-*/bin/signal-cli /usr/local/bin/
   sudo mv signal-cli-*/lib /usr/local/lib/signal-cli

2. Register a phone number (requires receiving SMS):
   signal-cli -a +1YOURNUMBER register
   signal-cli -a +1YOURNUMBER verify CODE_FROM_SMS

3. Configure SIGINT-Pi (config.toml):
   [alerts.signal]
   enabled = true
   sender_number = "+1YOURNUMBER"
   recipients = ["+1RECIPIENT1", "+1RECIPIENT2"]
   signal_cli_path = "/usr/local/bin/signal-cli"
   config_dir = "/etc/sigint-pi/signal"
   min_priority = "high"

4. Test the configuration:
   signal-cli -a +1YOURNUMBER send -m "Test" +1RECIPIENT

For daemon mode (faster):
   signal-cli -a +1YOURNUMBER daemon --socket /var/run/signal-cli/socket &

Then set use_jsonrpc = true in config.

Security Notes:
- Signal provides end-to-end encryption
- Your signal-cli credentials are stored in config_dir
- Protect config_dir permissions (chmod 700)
"#
}
