use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row, QueryBuilder};
use chrono::{DateTime, Utc, NaiveDateTime};
use std::path::Path;
use tracing::{info, error, instrument};
use uuid::Uuid;

use crate::{RadiationReading, QualityFlag, TimeSeriesPoint, AggregationLevel, GeoPoint, SensorReading, TimeRange};

#[derive(sqlx::FromRow)]
struct AggregatedRow {
    time_bucket: String,
    avg_dose: f64,
    min_dose: f64,
    max_dose: f64,
    reading_count: i64,
}

#[derive(Debug)]
pub struct SqliteStorage {
    pool: Pool<Sqlite>,
}

impl SqliteStorage {
    pub async fn new(database_path: &str) -> anyhow::Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(database_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(30))
            .connect(&format!("sqlite:{}", database_path))
            .await?;

        // Enable WAL mode for better concurrent performance
        sqlx::query("PRAGMA journal_mode = WAL")
            .execute(&pool)
            .await?;
        
        sqlx::query("PRAGMA synchronous = NORMAL")
            .execute(&pool)
            .await?;

        info!("SQLite warm storage initialized at {}", database_path);

        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> anyhow::Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;

        info!("SQLite migrations completed");
        Ok(())
    }

    #[instrument(skip(self, reading))]
    pub async fn write_reading(&self, reading: &RadiationReading) -> anyhow::Result<()> {
        let timestamp = DateTime::from_timestamp(reading.timestamp, 0)
            .unwrap_or_else(|| Utc::now());

        sqlx::query(
            r#"
            INSERT INTO radiation_readings_warm (
                sensor_id, bucket, timestamp, latitude, longitude,
                dose_rate, uncertainty, quality_flag, source, cell_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(sensor_id, bucket, timestamp) DO UPDATE SET
                dose_rate = excluded.dose_rate,
                uncertainty = excluded.uncertainty,
                quality_flag = excluded.quality_flag
            "#
        )
        .bind(reading.sensor_id.to_string())
        .bind(reading.bucket)
        .bind(timestamp.naive_utc())
        .bind(reading.latitude)
        .bind(reading.longitude)
        .bind(reading.dose_rate_microsieverts)
        .bind(reading.uncertainty)
        .bind(format!("{:?}", reading.quality_flag))
        .bind(&reading.source)
        .bind(&reading.cell_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(skip(self, sensor_ids))]
    pub async fn query_range(
        &self,
        sensor_ids: &[String],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        aggregation: AggregationLevel,
    ) -> anyhow::Result<Vec<TimeSeriesPoint>> {
        let start_naive = start.naive_utc();
        let end_naive = end.naive_utc();

        let points = match aggregation {
            AggregationLevel::Raw => {
                self.query_raw(sensor_ids, start_naive, end_naive).await?
            }
            AggregationLevel::OneMinute => {
                self.query_aggregated(sensor_ids, start_naive, end_naive, "minute").await?
            }
            AggregationLevel::FiveMinutes => {
                self.query_aggregated(sensor_ids, start_naive, end_naive, "5 minute").await?
            }
            AggregationLevel::OneHour => {
                self.query_aggregated(sensor_ids, start_naive, end_naive, "hour").await?
            }
            AggregationLevel::OneDay => {
                self.query_aggregated(sensor_ids, start_naive, end_naive, "day").await?
            }
        };

        Ok(points)
    }

    async fn query_raw(
        &self,
        sensor_ids: &[String],
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> anyhow::Result<Vec<TimeSeriesPoint>> {
        let mut query_builder = QueryBuilder::new(
            "SELECT timestamp, dose_rate FROM radiation_readings_warm 
             WHERE timestamp >= "
        );
        
        query_builder.push_bind(start);
        query_builder.push(" AND timestamp <= ");
        query_builder.push_bind(end);
        query_builder.push(" AND sensor_id IN (");

        let mut separated = query_builder.separated(", ");
        for sensor_id in sensor_ids {
            separated.push_bind(sensor_id);
        }
        separated.push_unseparated(") ");
        query_builder.push("ORDER BY timestamp ASC");

        let query = query_builder.build();
        let rows = query.fetch_all(&self.pool).await?;

        let points: Vec<TimeSeriesPoint> = rows
            .into_iter()
            .map(|row| {
                let timestamp: NaiveDateTime = row.get(0);
                let dose_rate: f64 = row.get(1);
                
                TimeSeriesPoint {
                    timestamp: DateTime::from_naive_utc_and_offset(timestamp, Utc),
                    value: dose_rate,
                    count: 1,
                    min: dose_rate,
                    max: dose_rate,
                    avg: dose_rate,
                }
            })
            .collect();

        Ok(points)
    }

    async fn query_aggregated(
        &self,
        sensor_ids: &[String],
        start: NaiveDateTime,
        end: NaiveDateTime,
        interval: &str,
    ) -> anyhow::Result<Vec<TimeSeriesPoint>> {
        let time_bucket = match interval {
            "minute" => "strftime('%Y-%m-%d %H:%M:00', timestamp)",
            "hour" => "strftime('%Y-%m-%d %H:00:00', timestamp)",
            "day" => "strftime('%Y-%m-%d 00:00:00', timestamp)",
            _ => "timestamp",
        };

        let query = format!(
            r#"
            SELECT 
                datetime({}) as time_bucket,
                AVG(dose_rate) as avg_dose,
                MIN(dose_rate) as min_dose,
                MAX(dose_rate) as max_dose,
                COUNT(*) as reading_count
            FROM radiation_readings_warm
            WHERE timestamp >= ? AND timestamp <= ?
            AND sensor_id IN ({})
            GROUP BY time_bucket
            ORDER BY time_bucket ASC
            "#,
            time_bucket,
            sensor_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query = sqlx::query_as::<_, AggregatedRow>(&query)
            .bind(start)
            .bind(end);

        for sensor_id in sensor_ids {
            query = query.bind(sensor_id);
        }

        let rows = query.fetch_all(&self.pool).await?;

        let points: Vec<TimeSeriesPoint> = rows
            .into_iter()
            .map(|row| {
                let timestamp = DateTime::parse_from_str(
                    &row.time_bucket,
                    "%Y-%m-%d %H:%M:%S"
                ).unwrap_or_else(|_| Utc::now().into()).with_timezone(&Utc);

                TimeSeriesPoint {
                    timestamp,
                    value: row.avg_dose,
                    count: row.reading_count as u64,
                    min: row.min_dose,
                    max: row.max_dose,
                    avg: row.avg_dose,
                }
            })
            .collect();

        Ok(points)
    }

    #[instrument(skip(self))]
    pub async fn query_geo(
        &self,
        center: GeoPoint,
        radius_km: f64,
        time_window: TimeRange,
    ) -> anyhow::Result<Vec<SensorReading>> {
        // Calculate bounding box for efficient filtering
        let lat_delta = radius_km / 111.0; // 1 degree lat ~ 111 km
        let lon_delta = radius_km / (111.0 * center.latitude.to_radians().cos());

        let min_lat = center.latitude - lat_delta;
        let max_lat = center.latitude + lat_delta;
        let min_lon = center.longitude - lon_delta;
        let max_lon = center.longitude + lon_delta;

        let start_naive = time_window.start.naive_utc();
        let end_naive = time_window.end.naive_utc();

        let rows = sqlx::query(
            r#"
            SELECT DISTINCT ON (sensor_id)
                sensor_id, timestamp, latitude, longitude, dose_rate, quality_flag
            FROM radiation_readings_warm
            WHERE timestamp >= ? AND timestamp <= ?
            AND latitude BETWEEN ? AND ?
            AND longitude BETWEEN ? AND ?
            ORDER BY sensor_id, timestamp DESC
            "#
        )
        .bind(start_naive)
        .bind(end_naive)
        .bind(min_lat)
        .bind(max_lat)
        .bind(min_lon)
        .bind(max_lon)
        .fetch_all(&self.pool)
        .await?;

        let mut results = Vec::new();

        for row in rows {
            let sensor_id_str: String = row.get(0);
            let sensor_id = Uuid::parse_str(&sensor_id_str)?;
            let timestamp: NaiveDateTime = row.get(1);
            let latitude: f64 = row.get(2);
            let longitude: f64 = row.get(3);
            let dose_rate: f64 = row.get(4);
            let quality_str: String = row.get(5);

            // Filter by actual haversine distance
            let distance = haversine_distance(
                center.latitude, center.longitude,
                latitude, longitude,
            );

            if distance <= radius_km {
                let quality = match quality_str.as_str() {
                    "Valid" => QualityFlag::Valid,
                    "Suspect" => QualityFlag::Suspect,
                    _ => QualityFlag::Invalid,
                };

                results.push(SensorReading {
                    sensor_id,
                    timestamp: DateTime::from_naive_utc_and_offset(timestamp, Utc),
                    location: GeoPoint { latitude, longitude },
                    dose_rate,
                    quality,
                });
            }
        }

        Ok(results)
    }

    pub async fn get_sensor_latest(
        &self,
        sensor_id: &Uuid,
    ) -> anyhow::Result<Option<RadiationReading>> {
        let row = sqlx::query(
            r#"
            SELECT * FROM radiation_readings_warm
            WHERE sensor_id = ?
            ORDER BY timestamp DESC
            LIMIT 1
            "#
        )
        .bind(sensor_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let reading = self.row_to_reading(row).await?;
                Ok(Some(reading))
            }
            None => Ok(None),
        }
    }

    async fn row_to_reading(&self, row: sqlx::sqlite::SqliteRow) -> anyhow::Result<RadiationReading> {
        let sensor_id_str: String = row.get("sensor_id");
        let sensor_id = Uuid::parse_str(&sensor_id_str)?;
        
        let timestamp: NaiveDateTime = row.get("timestamp");
        let timestamp_secs = timestamp.and_utc().timestamp();
        
        let quality_str: String = row.get("quality_flag");
        let quality = match quality_str.as_str() {
            "Valid" => QualityFlag::Valid,
            "Suspect" => QualityFlag::Suspect,
            _ => QualityFlag::Invalid,
        };

        Ok(RadiationReading {
            sensor_id,
            bucket: row.get("bucket"),
            timestamp: timestamp_secs,
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            dose_rate_microsieverts: row.get("dose_rate"),
            uncertainty: row.get("uncertainty"),
            quality_flag: quality,
            source: row.get("source"),
            cell_id: row.get("cell_id"),
        })
    }

    pub async fn health_check(&self) -> bool {
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => true,
            Err(e) => {
                error!("SQLite health check failed: {}", e);
                false
            }
        }
    }

    /// Archive old data to cold storage
    pub async fn archive_old_data(&self, before: DateTime<Utc>) -> anyhow::Result<u64> {
        let before_naive = before.naive_utc();
        
        let result = sqlx::query(
            r#"
            DELETE FROM radiation_readings_warm
            WHERE timestamp < ?
            "#
        )
        .bind(before_naive)
        .execute(&self.pool)
        .await?;

        let deleted = result.rows_affected();
        info!("Archived {} old readings from warm storage", deleted);
        
        // Vacuum to reclaim space
        sqlx::query("VACUUM").execute(&self.pool).await?;
        
        Ok(deleted)
    }

    /// List all sensors
    pub async fn list_sensors(&self) -> anyhow::Result<Vec<SensorInfo>> {
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT sensor_id, source 
            FROM radiation_readings_warm
            ORDER BY sensor_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let sensors: Vec<SensorInfo> = rows
            .into_iter()
            .map(|row| {
                let sensor_id_str: String = row.get(0);
                let sensor_id = Uuid::parse_str(&sensor_id_str).unwrap_or_else(|_| Uuid::new_v4());
                SensorInfo {
                    sensor_id,
                    source: row.get(1),
                }
            })
            .collect();

        Ok(sensors)
    }

    /// Get anomalies since a given timestamp
    pub async fn get_anomalies(
        &self,
        since: i64,
        limit: usize,
    ) -> anyhow::Result<Vec<AnomalyRecord>> {
        let since_naive = DateTime::from_timestamp(since, 0)
            .unwrap_or_else(|| Utc::now())
            .naive_utc();

        let rows = sqlx::query(
            r#"
            SELECT anomaly_id, sensor_id, severity, z_score, detected_at
            FROM anomalies
            WHERE detected_at >= ?
            ORDER BY detected_at DESC
            LIMIT ?
            "#
        )
        .bind(since_naive)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let anomalies: Vec<AnomalyRecord> = rows
            .into_iter()
            .map(|row| {
                let sensor_id_str: String = row.get(1);
                AnomalyRecord {
                    anomaly_id: row.get(0),
                    sensor_id: Uuid::parse_str(&sensor_id_str).unwrap_or_else(|_| Uuid::new_v4()),
                    severity: row.get(2),
                    z_score: row.get(3),
                    detected_at: row.get::<NaiveDateTime, _>(4).and_utc().timestamp(),
                }
            })
            .collect();

        Ok(anomalies)
    }

    /// Get count of anomalies in last N hours
    pub async fn get_anomaly_count(&self, hours: i64) -> anyhow::Result<i64> {
        let since = Utc::now() - chrono::Duration::hours(hours);
        
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM anomalies
            WHERE detected_at >= ?
            "#
        )
        .bind(since.naive_utc())
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
    }

    /// Store domain event for audit trail
    pub async fn store_event(&self, event: &crate::DomainEvent) -> anyhow::Result<()> {
        let timestamp = DateTime::from_timestamp(event.timestamp, 0)
            .unwrap_or_else(|| Utc::now());

        sqlx::query(
            r#"
            INSERT INTO domain_events (
                event_id, event_type, aggregate_id, payload, timestamp
            ) VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(event_id) DO NOTHING
            "#
        )
        .bind(&event.event_id)
        .bind(format!("{:?}", event.event_type))
        .bind(event.aggregate_id.to_string())
        .bind(event.payload.to_string())
        .bind(timestamp.naive_utc())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// List all sensors with their latest location and timestamp
    pub async fn list_sensors_with_location(&self) -> anyhow::Result<Vec<SensorRecord>> {
        let rows = sqlx::query(
            r#"
            SELECT 
                r.sensor_id,
                r.source,
                r.latitude,
                r.longitude,
                r.timestamp
            FROM radiation_readings_warm r
            INNER JOIN (
                SELECT sensor_id, MAX(timestamp) as max_ts
                FROM radiation_readings_warm
                GROUP BY sensor_id
            ) latest ON r.sensor_id = latest.sensor_id AND r.timestamp = latest.max_ts
            ORDER BY r.sensor_id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let sensors: Vec<SensorRecord> = rows
            .into_iter()
            .map(|row| {
                let sensor_id_str: String = row.get(0);
                let sensor_id = Uuid::parse_str(&sensor_id_str).unwrap_or_else(|_| Uuid::new_v4());
                let timestamp_naive: NaiveDateTime = row.get(4);
                
                SensorRecord {
                    sensor_id,
                    source: row.get(1),
                    latitude: row.get(2),
                    longitude: row.get(3),
                    timestamp: timestamp_naive.and_utc().timestamp(),
                }
            })
            .collect();

        Ok(sensors)
    }
}


/// Sensor information
#[derive(Debug, Clone)]
pub struct SensorInfo {
    pub sensor_id: Uuid,
    pub source: String,
}

/// Anomaly record from database
#[derive(Debug, Clone)]
pub struct AnomalyRecord {
    pub anomaly_id: String,
    pub sensor_id: Uuid,
    pub severity: String,
    pub z_score: f64,
    pub detected_at: i64,
}

/// Sensor record with location information for GraphQL resolvers
#[derive(Debug, Clone)]
pub struct SensorRecord {
    pub sensor_id: Uuid,
    pub source: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: i64,
}


/// Calculate haversine distance between two points in kilometers
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth's radius in km

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    R * c
}
