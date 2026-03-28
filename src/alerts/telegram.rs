use anyhow::Result;
use reqwest::Client;
use serde_json::json;

use crate::config::TelegramConfig;
use super::Alert;

#[derive(Clone)]
pub struct TelegramAlert {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramAlert {
    pub fn new(config: &TelegramConfig) -> Self {
        Self {
            client: Client::new(),
            bot_token: config.bot_token.clone(),
            chat_id: config.chat_id.clone(),
        }
    }

    pub async fn send(&self, alert: &Alert) -> Result<()> {
        let emoji = match alert.priority {
            super::AlertPriority::Critical => "🚨",
            super::AlertPriority::High => "⚠️",
            super::AlertPriority::Medium => "📢",
            super::AlertPriority::Low => "ℹ️",
        };

        let text = format!(
            "{} *{}*\n\n{}\n\n📍 Location: {}\n🕐 Time: {}",
            emoji,
            escape_markdown(&alert.title),
            escape_markdown(&alert.message),
            alert.location.as_deref().unwrap_or("Unknown"),
            alert.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let response = self.client
            .post(&url)
            .json(&json!({
                "chat_id": self.chat_id,
                "text": text,
                "parse_mode": "MarkdownV2",
                "disable_notification": alert.priority == super::AlertPriority::Low
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            anyhow::bail!("Telegram API error: {}", error);
        }

        Ok(())
    }
}

fn escape_markdown(text: &str) -> String {
    text.replace("_", "\\_")
        .replace("*", "\\*")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
        .replace("!", "\\!")
}
