use anyhow::Result;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

use crate::config::MqttConfig;
use super::Alert;

#[derive(Clone)]
pub struct MqttAlert {
    client: AsyncClient,
    topic_prefix: String,
}

impl MqttAlert {
    pub async fn new(config: &MqttConfig) -> Result<Self> {
        let mut options = MqttOptions::new(
            &config.client_id,
            &config.broker_host,
            config.broker_port,
        );
        
        options.set_keep_alive(Duration::from_secs(30));
        
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            options.set_credentials(username, password);
        }

        let (client, mut eventloop) = AsyncClient::new(options, 100);

        // Spawn event loop handler
        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("MQTT connection error: {}", e);
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        });

        Ok(Self {
            client,
            topic_prefix: config.topic_prefix.clone(),
        })
    }

    pub async fn publish(&self, alert: &Alert) -> Result<()> {
        let topic = format!(
            "{}/alerts/{:?}",
            self.topic_prefix,
            alert.priority
        ).to_lowercase();

        let payload = serde_json::to_string(alert)?;

        self.client
            .publish(&topic, QoS::AtLeastOnce, false, payload)
            .await?;

        // Also publish to device-specific topic if available
        if let Some(ref mac) = alert.device_mac {
            let device_topic = format!(
                "{}/devices/{}",
                self.topic_prefix,
                mac.replace(":", "")
            );
            
            let device_payload = serde_json::json!({
                "mac": mac,
                "vendor": alert.device_vendor,
                "rssi": alert.rssi,
                "last_seen": alert.timestamp,
                "alert_type": format!("{:?}", alert.alert_type),
            });

            self.client
                .publish(&device_topic, QoS::AtMostOnce, true, device_payload.to_string())
                .await?;
        }

        Ok(())
    }

    pub async fn publish_device_update(&self, mac: &str, rssi: i32, device_type: &str) -> Result<()> {
        let topic = format!(
            "{}/devices/{}",
            self.topic_prefix,
            mac.replace(":", "")
        );

        let payload = serde_json::json!({
            "mac": mac,
            "rssi": rssi,
            "device_type": device_type,
            "timestamp": chrono::Utc::now(),
        });

        self.client
            .publish(&topic, QoS::AtMostOnce, true, payload.to_string())
            .await?;

        Ok(())
    }
}
