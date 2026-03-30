use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use std::path::Path;

use crate::wifi::WifiDevice;
use crate::bluetooth::BleDevice;
use crate::gps::GpsPosition;
use crate::wifi::AttackEvent;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(path: &Path) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let url = format!("sqlite:{}?mode=rwc", path.display());
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .context("Failed to connect to database")?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::query(include_str!("schema.sql"))
            .execute(&self.pool)
            .await
            .context("Failed to run migrations")?;
        Ok(())
    }

    // ===== Device Operations =====

    pub async fn upsert_wifi_device(&self, device: &WifiDevice, location_id: i64) -> Result<i64> {
        let result = sqlx::query(r#"
            INSERT INTO wifi_devices (mac_address, vendor, is_ap, first_seen, last_seen, location_id)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(mac_address) DO UPDATE SET
                last_seen = excluded.last_seen,
                vendor = COALESCE(excluded.vendor, wifi_devices.vendor)
            RETURNING id
        "#)
            .bind(&device.mac_address)
            .bind(&device.vendor)
            .bind(device.is_ap)
            .bind(device.first_seen)
            .bind(device.last_seen)
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get::<i64, _>("id"))
    }

    pub async fn upsert_ble_device(&self, device: &BleDevice, location_id: i64) -> Result<i64> {
        let device_type = format!("{:?}", device.device_type);
        let result = sqlx::query(r#"
            INSERT INTO ble_devices (mac_address, name, device_type, vendor, first_seen, last_seen, location_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(mac_address) DO UPDATE SET
                last_seen = excluded.last_seen,
                name = COALESCE(excluded.name, ble_devices.name),
                vendor = COALESCE(excluded.vendor, ble_devices.vendor)
            RETURNING id
        "#)
            .bind(&device.mac_address)
            .bind(&device.name)
            .bind(&device_type)
            .bind(&device.vendor)
            .bind(device.first_seen)
            .bind(device.last_seen)
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get::<i64, _>("id"))
    }

    pub async fn record_sighting(
        &self,
        device_id: i64,
        device_type: &str,
        rssi: i32,
        channel: Option<u8>,
        ssid: Option<&str>,
        position: Option<&GpsPosition>,
    ) -> Result<()> {
        sqlx::query(r#"
            INSERT INTO sightings (device_id, device_type, rssi, channel, ssid, latitude, longitude)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#)
            .bind(device_id)
            .bind(device_type)
            .bind(rssi)
            .bind(channel.map(|c| c as i32))
            .bind(ssid)
            .bind(position.map(|p| p.latitude))
            .bind(position.map(|p| p.longitude))
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ===== Query Operations =====

    pub async fn is_device_known(&self, mac_address: &str, location_id: i64) -> Result<bool> {
        let wifi_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM wifi_devices WHERE mac_address = ? AND location_id = ? AND is_baseline = 1"
        )
            .bind(mac_address)
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        if wifi_count > 0 {
            return Ok(true);
        }

        let ble_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ble_devices WHERE mac_address = ? AND location_id = ? AND is_baseline = 1"
        )
            .bind(mac_address)
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(ble_count > 0)
    }

    pub async fn get_device_history(&self, mac_address: &str, hours: i64) -> Result<Vec<SightingRecord>> {
        let since = Utc::now() - chrono::Duration::hours(hours);
        
        let rows = sqlx::query(r#"
            SELECT device_type, rssi, channel, ssid, latitude, longitude, timestamp
            FROM sightings s
            JOIN wifi_devices w ON s.device_id = w.id AND s.device_type = 'wifi'
            WHERE w.mac_address = ? AND s.timestamp > ?
            UNION ALL
            SELECT device_type, rssi, channel, NULL as ssid, latitude, longitude, timestamp
            FROM sightings s
            JOIN ble_devices b ON s.device_id = b.id AND s.device_type = 'ble'
            WHERE b.mac_address = ? AND s.timestamp > ?
            ORDER BY timestamp DESC
        "#)
            .bind(mac_address)
            .bind(since)
            .bind(mac_address)
            .bind(since)
            .fetch_all(&self.pool)
            .await?;

        let records = rows.iter().map(|row| SightingRecord {
            device_type: row.get("device_type"),
            rssi: row.get("rssi"),
            channel: row.get("channel"),
            ssid: row.get("ssid"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            timestamp: row.get("timestamp"),
        }).collect();

        Ok(records)
    }

    pub async fn mark_as_baseline(&self, mac_address: &str, location_id: i64) -> Result<()> {
        sqlx::query("UPDATE wifi_devices SET is_baseline = 1 WHERE mac_address = ? AND location_id = ?")
            .bind(mac_address)
            .bind(location_id)
            .execute(&self.pool)
            .await?;

        sqlx::query("UPDATE ble_devices SET is_baseline = 1 WHERE mac_address = ? AND location_id = ?")
            .bind(mac_address)
            .bind(location_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ===== Location Operations =====

    pub async fn get_or_create_location(&self, name: &str, lat: Option<f64>, lon: Option<f64>) -> Result<i64> {
        let result = sqlx::query(r#"
            INSERT INTO locations (name, latitude, longitude)
            VALUES (?, ?, ?)
            ON CONFLICT(name) DO UPDATE SET
                latitude = COALESCE(excluded.latitude, locations.latitude),
                longitude = COALESCE(excluded.longitude, locations.longitude)
            RETURNING id
        "#)
            .bind(name)
            .bind(lat)
            .bind(lon)
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get::<i64, _>("id"))
    }

    // ===== Attack Logging =====

    pub async fn log_attack(&self, attack: &AttackEvent, location_id: i64) -> Result<()> {
        let attack_type = format!("{:?}", attack.attack_type);
        let severity = format!("{:?}", attack.severity);
        
        sqlx::query(r#"
            INSERT INTO attacks (attack_type, severity, source_mac, target_mac, bssid, description, location_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#)
            .bind(&attack_type)
            .bind(&severity)
            .bind(&attack.source_mac)
            .bind(&attack.target_mac)
            .bind(&attack.bssid)
            .bind(&attack.description)
            .bind(location_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ===== Alert Logging =====

    pub async fn log_alert(&self, alert_type: &str, priority: &str, message: &str, device_mac: Option<&str>) -> Result<()> {
        sqlx::query(r#"
            INSERT INTO alerts (alert_type, priority, message, device_mac)
            VALUES (?, ?, ?, ?)
        "#)
            .bind(alert_type)
            .bind(priority)
            .bind(message)
            .bind(device_mac)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ===== Cleanup =====

    pub async fn cleanup_old_data(&self, retention_days: u32) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        
        let result = sqlx::query("DELETE FROM sightings WHERE timestamp < ?")
            .bind(cutoff)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    // ===== Statistics =====

    pub async fn get_device_counts(&self, location_id: i64) -> Result<DeviceCounts> {
        let wifi_total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM wifi_devices WHERE location_id = ?"
        )
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        let wifi_baseline: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM wifi_devices WHERE location_id = ? AND is_baseline = 1"
        )
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        let ble_total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ble_devices WHERE location_id = ?"
        )
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        let ble_baseline: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ble_devices WHERE location_id = ? AND is_baseline = 1"
        )
            .bind(location_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(DeviceCounts {
            wifi_total: wifi_total as u64,
            wifi_baseline: wifi_baseline as u64,
            ble_total: ble_total as u64,
            ble_baseline: ble_baseline as u64,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SightingRecord {
    pub device_type: String,
    pub rssi: i32,
    pub channel: Option<i32>,
    pub ssid: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DeviceCounts {
    pub wifi_total: u64,
    pub wifi_baseline: u64,
    pub ble_total: u64,
    pub ble_baseline: u64,
}

// ===== Device Intelligence Methods =====

impl Database {
    /// Get cached device description from database
    pub async fn get_device_description(&self, mac: &str) -> Result<Option<crate::intelligence::DeviceIntelligence>> {
        let row = sqlx::query(r#"
            SELECT mac_address, device_name, device_type, vendor_name, ai_description,
                   threat_level, threat_level as threat_reason, tags, updated_at
            FROM device_descriptions
            WHERE mac_address = ?
        "#)
            .bind(mac)
            .fetch_optional(&self.pool)
            .await?;
        
        match row {
            Some(row) => {
                let threat_str: Option<String> = row.get("threat_level");
                let threat_level = match threat_str.as_deref() {
                    Some("critical") => crate::intelligence::ThreatLevel::Critical,
                    Some("high") => crate::intelligence::ThreatLevel::High,
                    Some("medium") => crate::intelligence::ThreatLevel::Medium,
                    Some("low") => crate::intelligence::ThreatLevel::Low,
                    Some("none") => crate::intelligence::ThreatLevel::None,
                    _ => crate::intelligence::ThreatLevel::Unknown,
                };
                
                Ok(Some(crate::intelligence::DeviceIntelligence {
                    mac_address: row.get("mac_address"),
                    device_name: row.get("device_name"),
                    device_type: row.get::<String, _>("device_type"),
                    vendor_name: row.get("vendor_name"),
                    ai_description: row.get("ai_description"),
                    threat_level,
                    threat_reason: row.get("threat_reason"),
                    category: None,
                    from_cache: true,
                    analyzed_at: None,
                }))
            }
            None => Ok(None),
        }
    }
    
    /// Save device description to database
    pub async fn save_device_description(&self, intel: &crate::intelligence::DeviceIntelligence) -> Result<()> {
        let threat_str = match intel.threat_level {
            crate::intelligence::ThreatLevel::Critical => "critical",
            crate::intelligence::ThreatLevel::High => "high",
            crate::intelligence::ThreatLevel::Medium => "medium",
            crate::intelligence::ThreatLevel::Low => "low",
            crate::intelligence::ThreatLevel::None => "none",
            crate::intelligence::ThreatLevel::Unknown => "unknown",
        };
        
        let is_threat = matches!(
            intel.threat_level,
            crate::intelligence::ThreatLevel::Critical | 
            crate::intelligence::ThreatLevel::High |
            crate::intelligence::ThreatLevel::Medium
        );
        
        sqlx::query(r#"
            INSERT INTO device_descriptions 
                (mac_address, device_name, device_type, vendor_name, ai_description, 
                 threat_level, is_threat, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(mac_address) DO UPDATE SET
                device_name = COALESCE(excluded.device_name, device_descriptions.device_name),
                ai_description = COALESCE(excluded.ai_description, device_descriptions.ai_description),
                threat_level = excluded.threat_level,
                is_threat = excluded.is_threat,
                times_seen = device_descriptions.times_seen + 1,
                last_seen = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
        "#)
            .bind(&intel.mac_address)
            .bind(&intel.device_name)
            .bind(&intel.device_type)
            .bind(&intel.vendor_name)
            .bind(&intel.ai_description)
            .bind(threat_str)
            .bind(is_threat)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Get all devices with AI descriptions (for export/review)
    pub async fn get_all_device_descriptions(&self) -> Result<Vec<crate::intelligence::DeviceIntelligence>> {
        let rows = sqlx::query(r#"
            SELECT mac_address, device_name, device_type, vendor_name, ai_description,
                   threat_level, times_seen, updated_at
            FROM device_descriptions
            ORDER BY updated_at DESC
            LIMIT 500
        "#)
            .fetch_all(&self.pool)
            .await?;
        
        let mut results = Vec::new();
        for row in rows {
            let threat_str: Option<String> = row.get("threat_level");
            let threat_level = match threat_str.as_deref() {
                Some("critical") => crate::intelligence::ThreatLevel::Critical,
                Some("high") => crate::intelligence::ThreatLevel::High,
                Some("medium") => crate::intelligence::ThreatLevel::Medium,
                Some("low") => crate::intelligence::ThreatLevel::Low,
                Some("none") => crate::intelligence::ThreatLevel::None,
                _ => crate::intelligence::ThreatLevel::Unknown,
            };
            
            results.push(crate::intelligence::DeviceIntelligence {
                mac_address: row.get("mac_address"),
                device_name: row.get("device_name"),
                device_type: row.get::<String, _>("device_type"),
                vendor_name: row.get("vendor_name"),
                ai_description: row.get("ai_description"),
                threat_level,
                threat_reason: None,
                category: None,
                from_cache: true,
                analyzed_at: None,
            });
        }
        
        Ok(results)
    }
}
