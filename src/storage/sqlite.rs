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

/// Sighting record for detailed contact history
#[derive(Debug, Clone, serde::Serialize)]
pub struct SightingRecord {
    pub device_type: String,
    pub rssi: i32,
    pub channel: Option<i32>,
    pub ssid: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timestamp: DateTime<Utc>,
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

    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
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

    // ===== Device Notes =====

    pub async fn get_device_notes(&self, mac: &str) -> Result<Vec<serde_json::Value>> {
        let rows = sqlx::query(r#"
            SELECT id, mac_address, device_type, note_text, note_source,
                   device_vendor, device_ssid, device_name, latitude, longitude,
                   created_at
            FROM device_notes
            WHERE mac_address = ?
            ORDER BY created_at DESC
        "#)
            .bind(mac)
            .fetch_all(&self.pool)
            .await?;
        
        let mut notes = Vec::new();
        for row in rows {
            notes.push(serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "mac_address": row.get::<String, _>("mac_address"),
                "device_type": row.get::<String, _>("device_type"),
                "note_text": row.get::<String, _>("note_text"),
                "note_source": row.get::<String, _>("note_source"),
                "device_vendor": row.get::<Option<String>, _>("device_vendor"),
                "device_ssid": row.get::<Option<String>, _>("device_ssid"),
                "device_name": row.get::<Option<String>, _>("device_name"),
                "latitude": row.get::<Option<f64>, _>("latitude"),
                "longitude": row.get::<Option<f64>, _>("longitude"),
                "created_at": row.get::<String, _>("created_at"),
            }));
        }
        Ok(notes)
    }

    pub async fn add_device_note(
        &self,
        mac: &str,
        device_type: &str,
        note_text: &str,
        note_source: &str,
        device_vendor: Option<&str>,
        device_ssid: Option<&str>,
        device_name: Option<&str>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Result<i64> {
        let result = sqlx::query(r#"
            INSERT INTO device_notes 
            (mac_address, device_type, note_text, note_source, device_vendor, device_ssid, device_name, latitude, longitude)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING id
        "#)
            .bind(mac)
            .bind(device_type)
            .bind(note_text)
            .bind(note_source)
            .bind(device_vendor)
            .bind(device_ssid)
            .bind(device_name)
            .bind(latitude)
            .bind(longitude)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(result.get::<i64, _>("id"))
    }

    pub async fn delete_device_note(&self, note_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM device_notes WHERE id = ?")
            .bind(note_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_recent_notes(&self, limit: i64) -> Result<Vec<serde_json::Value>> {
        let rows = sqlx::query(r#"
            SELECT id, mac_address, device_type, note_text, note_source,
                   device_vendor, device_ssid, device_name, latitude, longitude,
                   created_at
            FROM device_notes
            ORDER BY created_at DESC
            LIMIT ?
        "#)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        
        let mut notes = Vec::new();
        for row in rows {
            notes.push(serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "mac_address": row.get::<String, _>("mac_address"),
                "device_type": row.get::<String, _>("device_type"),
                "note_text": row.get::<String, _>("note_text"),
                "note_source": row.get::<String, _>("note_source"),
                "device_vendor": row.get::<Option<String>, _>("device_vendor"),
                "device_ssid": row.get::<Option<String>, _>("device_ssid"),
                "device_name": row.get::<Option<String>, _>("device_name"),
                "latitude": row.get::<Option<f64>, _>("latitude"),
                "longitude": row.get::<Option<f64>, _>("longitude"),
                "created_at": row.get::<String, _>("created_at"),
            }));
        }
        Ok(notes)
    }

    // ===== Device Discovery (auto-tag with location) =====

    pub async fn record_device_discovery(
        &self,
        mac: &str,
        device_type: &str,
        vendor: Option<&str>,
        ssid: Option<&str>,
        device_name: Option<&str>,
        rssi: i32,
        position: Option<&GpsPosition>,
    ) -> Result<()> {
        let (lat, lon, alt) = position.map(|p| (Some(p.latitude), Some(p.longitude), p.altitude))
            .unwrap_or((None, None, None));
        
        sqlx::query(r#"
            INSERT INTO device_discoveries 
            (mac_address, device_type, vendor, ssid, device_name, rssi, latitude, longitude, altitude)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(mac_address, device_type) DO NOTHING
        "#)
            .bind(mac)
            .bind(device_type)
            .bind(vendor)
            .bind(ssid)
            .bind(device_name)
            .bind(rssi)
            .bind(lat)
            .bind(lon)
            .bind(alt)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    // ===== Contact History (Unified Device Log) =====

    /// Get all contacts (devices) with full history - unified view
    pub async fn get_all_contacts(&self, limit: i64, offset: i64, search: Option<&str>) -> Result<Vec<ContactRecord>> {
        let search_pattern = search.map(|s| format!("%{}%", s));
        
        let rows = sqlx::query(r#"
            SELECT * FROM (
                SELECT 
                    w.mac_address,
                    'wifi' as device_type,
                    w.vendor,
                    NULL as device_name,
                    w.first_seen,
                    w.last_seen,
                    w.is_baseline,
                    w.is_ap,
                    (SELECT COUNT(*) FROM sightings s WHERE s.device_id = w.id AND s.device_type = 'wifi') as sighting_count,
                    (SELECT AVG(rssi) FROM sightings s WHERE s.device_id = w.id AND s.device_type = 'wifi') as avg_rssi,
                    (SELECT latitude FROM sightings s WHERE s.device_id = w.id AND s.device_type = 'wifi' ORDER BY timestamp DESC LIMIT 1) as last_lat,
                    (SELECT longitude FROM sightings s WHERE s.device_id = w.id AND s.device_type = 'wifi' ORDER BY timestamp DESC LIMIT 1) as last_lon,
                    dd.ai_description,
                    dd.threat_level
                FROM wifi_devices w
                LEFT JOIN device_descriptions dd ON w.mac_address = dd.mac_address
                WHERE (? IS NULL OR w.mac_address LIKE ? OR w.vendor LIKE ?)
                
                UNION ALL
                
                SELECT 
                    b.mac_address,
                    'ble' as device_type,
                    b.vendor,
                    b.name as device_name,
                    b.first_seen,
                    b.last_seen,
                    b.is_baseline,
                    0 as is_ap,
                    (SELECT COUNT(*) FROM sightings s WHERE s.device_id = b.id AND s.device_type = 'ble') as sighting_count,
                    (SELECT AVG(rssi) FROM sightings s WHERE s.device_id = b.id AND s.device_type = 'ble') as avg_rssi,
                    (SELECT latitude FROM sightings s WHERE s.device_id = b.id AND s.device_type = 'ble' ORDER BY timestamp DESC LIMIT 1) as last_lat,
                    (SELECT longitude FROM sightings s WHERE s.device_id = b.id AND s.device_type = 'ble' ORDER BY timestamp DESC LIMIT 1) as last_lon,
                    dd.ai_description,
                    dd.threat_level
                FROM ble_devices b
                LEFT JOIN device_descriptions dd ON b.mac_address = dd.mac_address
                WHERE (? IS NULL OR b.mac_address LIKE ? OR b.vendor LIKE ? OR b.name LIKE ?)
            )
            ORDER BY last_seen DESC
            LIMIT ? OFFSET ?
        "#)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        let mut contacts = Vec::new();
        for row in rows {
            contacts.push(ContactRecord {
                mac_address: row.get("mac_address"),
                device_type: row.get("device_type"),
                vendor: row.get("vendor"),
                device_name: row.get("device_name"),
                first_seen: row.get("first_seen"),
                last_seen: row.get("last_seen"),
                is_baseline: row.get("is_baseline"),
                is_ap: row.get("is_ap"),
                sighting_count: row.get::<i64, _>("sighting_count") as u64,
                avg_rssi: row.get("avg_rssi"),
                last_latitude: row.get("last_lat"),
                last_longitude: row.get("last_lon"),
                ai_description: row.get("ai_description"),
                threat_level: row.get("threat_level"),
            });
        }
        Ok(contacts)
    }

    /// Get total contact count for pagination
    pub async fn get_contact_count(&self, search: Option<&str>) -> Result<i64> {
        let search_pattern = search.map(|s| format!("%{}%", s));
        
        let count: i64 = sqlx::query_scalar(r#"
            SELECT (
                SELECT COUNT(*) FROM wifi_devices 
                WHERE (? IS NULL OR mac_address LIKE ? OR vendor LIKE ?)
            ) + (
                SELECT COUNT(*) FROM ble_devices 
                WHERE (? IS NULL OR mac_address LIKE ? OR vendor LIKE ? OR name LIKE ?)
            )
        "#)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(count)
    }

    /// Get detailed contact info with all sightings
    pub async fn get_contact_detail(&self, mac: &str) -> Result<Option<ContactDetail>> {
        // Get WiFi device
        let wifi_row = sqlx::query(r#"
            SELECT id, mac_address, vendor, is_ap, is_baseline, first_seen, last_seen, notes
            FROM wifi_devices WHERE mac_address = ?
        "#)
            .bind(mac)
            .fetch_optional(&self.pool)
            .await?;

        // Get BLE device
        let ble_row = sqlx::query(r#"
            SELECT id, mac_address, name, device_type, vendor, is_baseline, is_tracker, first_seen, last_seen, notes
            FROM ble_devices WHERE mac_address = ?
        "#)
            .bind(mac)
            .fetch_optional(&self.pool)
            .await?;

        let (device_id, device_type, device_name, vendor, is_baseline, first_seen, last_seen, notes) = 
            if let Some(row) = wifi_row {
                (
                    row.get::<i64, _>("id"),
                    "wifi".to_string(),
                    None::<String>,
                    row.get::<Option<String>, _>("vendor"),
                    row.get::<bool, _>("is_baseline"),
                    row.get::<DateTime<Utc>, _>("first_seen"),
                    row.get::<DateTime<Utc>, _>("last_seen"),
                    row.get::<Option<String>, _>("notes"),
                )
            } else if let Some(row) = ble_row {
                (
                    row.get::<i64, _>("id"),
                    "ble".to_string(),
                    row.get::<Option<String>, _>("name"),
                    row.get::<Option<String>, _>("vendor"),
                    row.get::<bool, _>("is_baseline"),
                    row.get::<DateTime<Utc>, _>("first_seen"),
                    row.get::<DateTime<Utc>, _>("last_seen"),
                    row.get::<Option<String>, _>("notes"),
                )
            } else {
                return Ok(None);
            };

        // Get sightings
        let sightings = sqlx::query(r#"
            SELECT rssi, channel, ssid, latitude, longitude, timestamp
            FROM sightings
            WHERE device_id = ? AND device_type = ?
            ORDER BY timestamp DESC
            LIMIT 100
        "#)
            .bind(device_id)
            .bind(&device_type)
            .fetch_all(&self.pool)
            .await?;

        let sighting_records: Vec<SightingRecord> = sightings.iter().map(|row| SightingRecord {
            device_type: device_type.clone(),
            rssi: row.get("rssi"),
            channel: row.get("channel"),
            ssid: row.get("ssid"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            timestamp: row.get("timestamp"),
        }).collect();

        // Get AI description
        let desc_row = sqlx::query(r#"
            SELECT ai_description, threat_level, times_seen FROM device_descriptions WHERE mac_address = ?
        "#)
            .bind(mac)
            .fetch_optional(&self.pool)
            .await?;

        let (ai_description, threat_level, times_seen) = desc_row
            .map(|row| (
                row.get::<Option<String>, _>("ai_description"),
                row.get::<Option<String>, _>("threat_level"),
                row.get::<i64, _>("times_seen") as u64,
            ))
            .unwrap_or((None, None, 0));

        // Get notes
        let notes_data = self.get_device_notes(mac).await.unwrap_or_default();

        Ok(Some(ContactDetail {
            mac_address: mac.to_string(),
            device_type,
            device_name,
            vendor,
            is_baseline,
            first_seen,
            last_seen,
            times_seen,
            notes,
            ai_description,
            threat_level,
            sightings: sighting_records,
            user_notes: notes_data,
        }))
    }

    /// Export all contacts as JSON for backup/sync
    pub async fn export_contacts(&self) -> Result<serde_json::Value> {
        let contacts = self.get_all_contacts(10000, 0, None).await?;
        let notes = self.get_recent_notes(1000).await?;
        let descriptions = self.get_all_device_descriptions().await?;
        
        Ok(serde_json::json!({
            "exported_at": Utc::now().to_rfc3339(),
            "version": "1.0",
            "contacts": contacts,
            "notes": notes,
            "descriptions": descriptions.iter().map(|d| serde_json::json!({
                "mac": d.mac_address,
                "name": d.device_name,
                "type": d.device_type,
                "vendor": d.vendor_name,
                "description": d.ai_description,
                "threat_level": format!("{:?}", d.threat_level),
            })).collect::<Vec<_>>()
        }))
    }
}

/// Unified contact record for history view
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContactRecord {
    pub mac_address: String,
    pub device_type: String,
    pub vendor: Option<String>,
    pub device_name: Option<String>,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub is_baseline: bool,
    pub is_ap: bool,
    pub sighting_count: u64,
    pub avg_rssi: Option<f64>,
    pub last_latitude: Option<f64>,
    pub last_longitude: Option<f64>,
    pub ai_description: Option<String>,
    pub threat_level: Option<String>,
}

/// Detailed contact info with full sighting history
#[derive(Debug, Clone, serde::Serialize)]
pub struct ContactDetail {
    pub mac_address: String,
    pub device_type: String,
    pub device_name: Option<String>,
    pub vendor: Option<String>,
    pub is_baseline: bool,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub times_seen: u64,
    pub notes: Option<String>,
    pub ai_description: Option<String>,
    pub threat_level: Option<String>,
    pub sightings: Vec<SightingRecord>,
    pub user_notes: Vec<serde_json::Value>,
}

// ===== Threat Watchlist =====

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatchlistEntry {
    pub id: i64,
    pub mac_address: Option<String>,
    pub signature: Option<String>,
    pub threat_type: String,
    pub description: Option<String>,
    pub source: String,
    pub active: bool,
    pub created_at: String,
}

impl Database {
    pub async fn watchlist_add(&self, mac: Option<&str>, sig: Option<&str>,
                                threat_type: &str, description: Option<&str>, source: &str) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO threat_watchlist (mac_address, signature, threat_type, description, source) VALUES (?, ?, ?, ?, ?)"
        ).bind(mac).bind(sig).bind(threat_type).bind(description).bind(source)
        .execute(&self.pool).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn watchlist_remove(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM threat_watchlist WHERE id = ?").bind(id)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn watchlist_list(&self) -> Result<Vec<WatchlistEntry>> {
        let rows = sqlx::query("SELECT * FROM threat_watchlist WHERE active = 1 ORDER BY created_at DESC")
            .fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| WatchlistEntry {
            id: r.get("id"), mac_address: r.get("mac_address"), signature: r.get("signature"),
            threat_type: r.get("threat_type"), description: r.get("description"),
            source: r.get("source"), active: r.get::<i32, _>("active") != 0,
            created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn watchlist_check_mac(&self, mac: &str) -> Result<Option<WatchlistEntry>> {
        let row = sqlx::query("SELECT * FROM threat_watchlist WHERE mac_address = ? AND active = 1 LIMIT 1")
            .bind(mac.to_uppercase())
            .fetch_optional(&self.pool).await?;
        Ok(row.map(|r| WatchlistEntry {
            id: r.get("id"), mac_address: r.get("mac_address"), signature: r.get("signature"),
            threat_type: r.get("threat_type"), description: r.get("description"),
            source: r.get("source"), active: true, created_at: r.get("created_at"),
        }))
    }
}

// ===== SIEM Operations =====

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SiemEvent {
    pub id: i64,
    pub timestamp: String,
    pub source: String,
    pub severity: String,
    pub category: String,
    pub device_mac: Option<String>,
    pub message: String,
    pub raw_data: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Database {
    pub async fn siem_insert(&self, source: &str, severity: &str, category: &str,
                             device_mac: Option<&str>, message: &str, raw_data: Option<&str>,
                             lat: Option<f64>, lon: Option<f64>) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO siem_events (source, severity, category, device_mac, message, raw_data, latitude, longitude)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(source).bind(severity).bind(category).bind(device_mac)
        .bind(message).bind(raw_data).bind(lat).bind(lon)
        .execute(&self.pool).await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn siem_search(&self, query: &str, limit: i64, offset: i64,
                              severity: Option<&str>, category: Option<&str>,
                              since: Option<&str>, until: Option<&str>) -> Result<Vec<SiemEvent>> {
        let mut sql = String::from(
            "SELECT e.id, e.timestamp, e.source, e.severity, e.category, e.device_mac, e.message, e.raw_data, e.latitude, e.longitude
             FROM siem_events e"
        );
        let mut conditions = Vec::new();
        let mut use_fts = false;

        if !query.is_empty() {
            sql = format!(
                "SELECT e.id, e.timestamp, e.source, e.severity, e.category, e.device_mac, e.message, e.raw_data, e.latitude, e.longitude
                 FROM siem_events e JOIN siem_events_fts f ON e.id = f.rowid
                 WHERE siem_events_fts MATCH ?1"
            );
            use_fts = true;
        }

        if let Some(sev) = severity {
            conditions.push(format!("e.severity = '{}'", sev.replace('\'', "''")));
        }
        if let Some(cat) = category {
            conditions.push(format!("e.category = '{}'", cat.replace('\'', "''")));
        }
        if let Some(ts) = since {
            conditions.push(format!("e.timestamp >= '{}'", ts.replace('\'', "''")));
        }
        if let Some(ts) = until {
            conditions.push(format!("e.timestamp <= '{}'", ts.replace('\'', "''")));
        }

        if !conditions.is_empty() {
            let joiner = if use_fts { " AND " } else { " WHERE " };
            sql.push_str(joiner);
            sql.push_str(&conditions.join(" AND "));
        }

        if use_fts {
            sql.push_str(" ORDER BY e.timestamp DESC LIMIT ?2 OFFSET ?3");
        } else {
            sql.push_str(" ORDER BY e.timestamp DESC LIMIT ?1 OFFSET ?2");
        }

        let rows = if use_fts {
            sqlx::query(&sql)
                .bind(query).bind(limit).bind(offset)
                .fetch_all(&self.pool).await?
        } else {
            sqlx::query(&sql)
                .bind(limit).bind(offset)
                .fetch_all(&self.pool).await?
        };

        let mut events = Vec::new();
        for row in rows {
            events.push(SiemEvent {
                id: row.get("id"),
                timestamp: row.get("timestamp"),
                source: row.get("source"),
                severity: row.get("severity"),
                category: row.get("category"),
                device_mac: row.get("device_mac"),
                message: row.get("message"),
                raw_data: row.get("raw_data"),
                latitude: row.get("latitude"),
                longitude: row.get("longitude"),
            });
        }
        Ok(events)
    }

    pub async fn siem_count(&self) -> Result<(i64, i64)> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM siem_events")
            .fetch_one(&self.pool).await?;
        // Estimate size: ~500 bytes per event average
        let page_count: i64 = sqlx::query_scalar("PRAGMA page_count")
            .fetch_one(&self.pool).await.unwrap_or(0);
        let page_size: i64 = sqlx::query_scalar("PRAGMA page_size")
            .fetch_one(&self.pool).await.unwrap_or(4096);
        let db_size = page_count * page_size;
        Ok((count, db_size))
    }

    pub async fn siem_prune(&self, max_bytes: i64) -> Result<u64> {
        let (count, db_size) = self.siem_count().await?;
        if db_size <= max_bytes || count == 0 { return Ok(0); }
        // Delete oldest 10% of events
        let delete_count = (count / 10).max(100);
        let result = sqlx::query(
            "DELETE FROM siem_events WHERE id IN (SELECT id FROM siem_events ORDER BY timestamp ASC LIMIT ?)"
        ).bind(delete_count).execute(&self.pool).await?;
        // Rebuild FTS index after bulk delete
        let _ = sqlx::query("INSERT INTO siem_events_fts(siem_events_fts) VALUES('rebuild')")
            .execute(&self.pool).await;
        Ok(result.rows_affected())
    }

    pub async fn siem_severity_counts(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query("SELECT severity, COUNT(*) as cnt FROM siem_events GROUP BY severity ORDER BY cnt DESC")
            .fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| (r.get::<String, _>("severity"), r.get::<i64, _>("cnt"))).collect())
    }

    pub async fn siem_category_counts(&self) -> Result<Vec<(String, i64)>> {
        let rows = sqlx::query("SELECT category, COUNT(*) as cnt FROM siem_events GROUP BY category ORDER BY cnt DESC")
            .fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| (r.get::<String, _>("category"), r.get::<i64, _>("cnt"))).collect())
    }

    pub async fn siem_recent_sources(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT DISTINCT source FROM siem_events ORDER BY source")
            .fetch_all(&self.pool).await?;
        Ok(rows.iter().map(|r| r.get::<String, _>("source")).collect())
    }
}
