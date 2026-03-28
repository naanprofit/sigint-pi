use anyhow::Result;
use reqwest::Client;

use crate::config::TwilioConfig;
use super::Alert;

#[derive(Clone)]
pub struct TwilioAlert {
    client: Client,
    account_sid: String,
    auth_token: String,
    from_number: String,
    to_number: String,
}

impl TwilioAlert {
    pub fn new(config: &TwilioConfig) -> Self {
        Self {
            client: Client::new(),
            account_sid: config.account_sid.clone(),
            auth_token: config.auth_token.clone(),
            from_number: config.from_number.clone(),
            to_number: config.to_number.clone(),
        }
    }

    pub async fn send(&self, alert: &Alert) -> Result<()> {
        // Only send SMS for high priority alerts
        if !matches!(alert.priority, super::AlertPriority::Critical | super::AlertPriority::High) {
            return Ok(());
        }

        let message = format!(
            "[SIGINT-Pi] {}\n\n{}\n\nLocation: {}",
            alert.title,
            truncate(&alert.message, 300),
            alert.location.as_deref().unwrap_or("Unknown")
        );

        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let response = self.client
            .post(&url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&[
                ("From", &self.from_number),
                ("To", &self.to_number),
                ("Body", &message),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            anyhow::bail!("Twilio API error: {}", error);
        }

        Ok(())
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
