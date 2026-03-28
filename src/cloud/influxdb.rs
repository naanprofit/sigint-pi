use anyhow::Result;
use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

/// InfluxDB line protocol exporter for time-series metrics
pub struct InfluxExporter {
    client: Client,
    url: String,
    token: String,
    org: String,
    bucket: String,
    batch_size: usize,
    flush_interval: Duration,
    tx: mpsc::Sender<Metric>,
}

#[derive(Debug, Clone)]
pub struct Metric {
    pub measurement: String,
    pub tags: HashMap<String, String>,
    pub fields: HashMap<String, FieldValue>,
    pub timestamp_ns: i64,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Float(f64),
    Int(i64),
    String(String),
    Bool(bool),
}

impl InfluxExporter {
    pub async fn new(
        url: &str,
        token: &str,
        org: &str,
        bucket: &str,
    ) -> Result<Self> {
        let (tx, rx) = mpsc::channel::<Metric>(10000);
        
        let exporter = Self {
            client: Client::new(),
            url: url.to_string(),
            token: token.to_string(),
            org: org.to_string(),
            bucket: bucket.to_string(),
            batch_size: 100,
            flush_interval: Duration::from_secs(10),
            tx,
        };

        // Start background batch writer
        let client = exporter.client.clone();
        let url = exporter.url.clone();
        let token = exporter.token.clone();
        let org = exporter.org.clone();
        let bucket = exporter.bucket.clone();
        let batch_size = exporter.batch_size;
        let flush_interval = exporter.flush_interval;

        tokio::spawn(async move {
            batch_writer(rx, client, url, token, org, bucket, batch_size, flush_interval).await;
        });

        Ok(exporter)
    }

    pub async fn write_wifi_device(
        &self,
        mac: &str,
        vendor: Option<&str>,
        rssi: i32,
        channel: u8,
        is_new: bool,
        location: &str,
    ) -> Result<()> {
        let mut tags = HashMap::new();
        tags.insert("mac".to_string(), mac.to_string());
        tags.insert("location".to_string(), location.to_string());
        if let Some(v) = vendor {
            tags.insert("vendor".to_string(), v.to_string());
        }

        let mut fields = HashMap::new();
        fields.insert("rssi".to_string(), FieldValue::Int(rssi as i64));
        fields.insert("channel".to_string(), FieldValue::Int(channel as i64));
        fields.insert("is_new".to_string(), FieldValue::Bool(is_new));

        let metric = Metric {
            measurement: "wifi_device".to_string(),
            tags,
            fields,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        self.tx.send(metric).await.ok();
        Ok(())
    }

    pub async fn write_ble_device(
        &self,
        mac: &str,
        name: Option<&str>,
        rssi: i32,
        device_type: &str,
        is_tracker: bool,
        location: &str,
    ) -> Result<()> {
        let mut tags = HashMap::new();
        tags.insert("mac".to_string(), mac.to_string());
        tags.insert("device_type".to_string(), device_type.to_string());
        tags.insert("location".to_string(), location.to_string());
        if let Some(n) = name {
            tags.insert("name".to_string(), n.to_string());
        }

        let mut fields = HashMap::new();
        fields.insert("rssi".to_string(), FieldValue::Int(rssi as i64));
        fields.insert("is_tracker".to_string(), FieldValue::Bool(is_tracker));

        let metric = Metric {
            measurement: "ble_device".to_string(),
            tags,
            fields,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        self.tx.send(metric).await.ok();
        Ok(())
    }

    pub async fn write_gps(
        &self,
        lat: f64,
        lon: f64,
        altitude: Option<f64>,
        speed: Option<f64>,
        satellites: u8,
        device_name: &str,
    ) -> Result<()> {
        let mut tags = HashMap::new();
        tags.insert("device".to_string(), device_name.to_string());

        let mut fields = HashMap::new();
        fields.insert("latitude".to_string(), FieldValue::Float(lat));
        fields.insert("longitude".to_string(), FieldValue::Float(lon));
        fields.insert("satellites".to_string(), FieldValue::Int(satellites as i64));
        
        if let Some(alt) = altitude {
            fields.insert("altitude".to_string(), FieldValue::Float(alt));
        }
        if let Some(spd) = speed {
            fields.insert("speed".to_string(), FieldValue::Float(spd));
        }

        let metric = Metric {
            measurement: "gps_position".to_string(),
            tags,
            fields,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        self.tx.send(metric).await.ok();
        Ok(())
    }

    pub async fn write_attack(
        &self,
        attack_type: &str,
        severity: &str,
        source_mac: &str,
        location: &str,
    ) -> Result<()> {
        let mut tags = HashMap::new();
        tags.insert("attack_type".to_string(), attack_type.to_string());
        tags.insert("severity".to_string(), severity.to_string());
        tags.insert("source_mac".to_string(), source_mac.to_string());
        tags.insert("location".to_string(), location.to_string());

        let mut fields = HashMap::new();
        fields.insert("count".to_string(), FieldValue::Int(1));

        let metric = Metric {
            measurement: "attack".to_string(),
            tags,
            fields,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        self.tx.send(metric).await.ok();
        Ok(())
    }

    pub async fn write_alert(
        &self,
        priority: &str,
        alert_type: &str,
        device_mac: Option<&str>,
        location: &str,
    ) -> Result<()> {
        let mut tags = HashMap::new();
        tags.insert("priority".to_string(), priority.to_string());
        tags.insert("alert_type".to_string(), alert_type.to_string());
        tags.insert("location".to_string(), location.to_string());
        if let Some(mac) = device_mac {
            tags.insert("device_mac".to_string(), mac.to_string());
        }

        let mut fields = HashMap::new();
        fields.insert("count".to_string(), FieldValue::Int(1));

        let metric = Metric {
            measurement: "alert".to_string(),
            tags,
            fields,
            timestamp_ns: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        self.tx.send(metric).await.ok();
        Ok(())
    }
}

async fn batch_writer(
    mut rx: mpsc::Receiver<Metric>,
    client: Client,
    url: String,
    token: String,
    org: String,
    bucket: String,
    batch_size: usize,
    flush_interval: Duration,
) {
    let mut batch: Vec<Metric> = Vec::with_capacity(batch_size);
    let mut last_flush = std::time::Instant::now();

    loop {
        tokio::select! {
            Some(metric) = rx.recv() => {
                batch.push(metric);
                
                if batch.len() >= batch_size {
                    flush_batch(&client, &url, &token, &org, &bucket, &mut batch).await;
                    last_flush = std::time::Instant::now();
                }
            }
            _ = tokio::time::sleep(flush_interval) => {
                if !batch.is_empty() && last_flush.elapsed() >= flush_interval {
                    flush_batch(&client, &url, &token, &org, &bucket, &mut batch).await;
                    last_flush = std::time::Instant::now();
                }
            }
        }
    }
}

async fn flush_batch(
    client: &Client,
    url: &str,
    token: &str,
    org: &str,
    bucket: &str,
    batch: &mut Vec<Metric>,
) {
    if batch.is_empty() {
        return;
    }

    let line_protocol = batch
        .iter()
        .map(metric_to_line_protocol)
        .collect::<Vec<_>>()
        .join("\n");

    let write_url = format!("{}/api/v2/write?org={}&bucket={}&precision=ns", url, org, bucket);

    match client
        .post(&write_url)
        .header("Authorization", format!("Token {}", token))
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(line_protocol)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                debug!("Flushed {} metrics to InfluxDB", batch.len());
            } else {
                warn!(
                    "InfluxDB write failed: {} - {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                );
            }
        }
        Err(e) => {
            error!("InfluxDB connection error: {}", e);
        }
    }

    batch.clear();
}

fn metric_to_line_protocol(metric: &Metric) -> String {
    let mut line = metric.measurement.clone();

    // Add tags
    for (key, value) in &metric.tags {
        line.push(',');
        line.push_str(&escape_tag(key));
        line.push('=');
        line.push_str(&escape_tag(value));
    }

    line.push(' ');

    // Add fields
    let fields: Vec<String> = metric
        .fields
        .iter()
        .map(|(key, value)| {
            let v = match value {
                FieldValue::Float(f) => format!("{}", f),
                FieldValue::Int(i) => format!("{}i", i),
                FieldValue::String(s) => format!("\"{}\"", escape_string(s)),
                FieldValue::Bool(b) => format!("{}", b),
            };
            format!("{}={}", escape_field_key(key), v)
        })
        .collect();
    line.push_str(&fields.join(","));

    // Add timestamp
    line.push(' ');
    line.push_str(&metric.timestamp_ns.to_string());

    line
}

fn escape_tag(s: &str) -> String {
    s.replace(' ', "\\ ")
        .replace(',', "\\,")
        .replace('=', "\\=")
}

fn escape_field_key(s: &str) -> String {
    s.replace(' ', "\\ ")
        .replace(',', "\\,")
        .replace('=', "\\=")
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_protocol() {
        let mut tags = HashMap::new();
        tags.insert("mac".to_string(), "AA:BB:CC:DD:EE:FF".to_string());
        
        let mut fields = HashMap::new();
        fields.insert("rssi".to_string(), FieldValue::Int(-65));
        fields.insert("is_new".to_string(), FieldValue::Bool(true));

        let metric = Metric {
            measurement: "wifi_device".to_string(),
            tags,
            fields,
            timestamp_ns: 1234567890000000000,
        };

        let line = metric_to_line_protocol(&metric);
        assert!(line.starts_with("wifi_device,mac=AA:BB:CC:DD:EE:FF"));
        assert!(line.contains("rssi=-65i"));
        assert!(line.contains("is_new=true"));
        assert!(line.ends_with("1234567890000000000"));
    }
}
