use anyhow::Result;
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

use crate::config::EmailConfig;
use super::Alert;

#[derive(Clone)]
pub struct EmailAlert {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
    to_addresses: Vec<String>,
}

impl EmailAlert {
    pub fn new(config: &EmailConfig) -> Result<Self> {
        let creds = Credentials::new(
            config.smtp_user.clone(),
            config.smtp_password.clone(),
        );

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)?
            .port(config.smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            transport,
            from_address: config.from_address.clone(),
            to_addresses: config.to_addresses.clone(),
        })
    }

    pub async fn send(&self, alert: &Alert) -> Result<()> {
        let priority_tag = match alert.priority {
            super::AlertPriority::Critical => "[CRITICAL]",
            super::AlertPriority::High => "[HIGH]",
            super::AlertPriority::Medium => "[MEDIUM]",
            super::AlertPriority::Low => "[LOW]",
        };

        let subject = format!("{} SIGINT-Pi: {}", priority_tag, alert.title);

        let body = format!(
            r#"
SIGINT-Pi Security Alert
========================

Priority: {:?}
Type: {:?}
Time: {}
Location: {}

{}

---
Device MAC: {}
Device Vendor: {}
Signal Strength: {} dBm

---
This is an automated alert from SIGINT-Pi.
            "#,
            alert.priority,
            alert.alert_type,
            alert.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            alert.location.as_deref().unwrap_or("Unknown"),
            alert.message,
            alert.device_mac.as_deref().unwrap_or("N/A"),
            alert.device_vendor.as_deref().unwrap_or("Unknown"),
            alert.rssi.map(|r| r.to_string()).unwrap_or_else(|| "N/A".to_string()),
        );

        for to_address in &self.to_addresses {
            let email = Message::builder()
                .from(self.from_address.parse()?)
                .to(to_address.parse()?)
                .subject(&subject)
                .header(ContentType::TEXT_PLAIN)
                .body(body.clone())?;

            self.transport.send(email).await?;
        }

        Ok(())
    }
}
