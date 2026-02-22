pub mod scylla;
pub mod schema;
pub mod query;
pub mod sqlite;
pub mod cache;
pub mod storage;

pub use sqlite::{SensorInfo, AnomalyRecord, SensorRecord};


use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::sync::Arc;
use tracing::{info, warn, instrument};
use thiserror::Error;
use backoff::{ExponentialBackoff, future::retry};

use scylla::ScyllaStorage;
use sqlite::SqliteStorage;
use cache::RedisCache;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationReading {
    pub sensor_id: Uuid,
    pub bucket: i64,
    pub timestamp: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub dose_rate_microsieverts: f64,
    pub uncertainty: f32,
    pub quality_flag: QualityFlag,
    pub source: String,
    pub cell_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityFlag {
    Valid,
    Suspect,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub aggregate_id: Uuid,
    pub payload: serde_json::Value,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    AnomalyDetected,
    AlertTriggered,
    IncidentCreated,
    SensorOffline,
    SensorOnline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub count: u64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AggregationLevel {
    Raw,
    OneMinute,
    FiveMinutes,
    OneHour,
    OneDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub location: GeoPoint,
    pub dose_rate: f64,
    pub quality: QualityFlag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("ScyllaDB error: {0}")]
    Scylla(String),
    #[error("SQLite error: {0}")]
    Sqlite(String),
    #[error("Redis error: {0}")]
    Redis(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Query error: {0}")]
    Query(String),
    #[error("Migration error: {0}")]
    Migration(String),
}

/// Unified storage abstraction with hot/warm/cold tiering
pub struct RadiationDatabase {
    hot: Arc<ScyllaStorage>,
    warm: Arc<SqliteStorage>,
    cache: Arc<RedisCache>,
    config: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub hot_retention_days: i64,
    pub warm_retention_days: i64,
    pub enable_cold_archive: bool,
    pub max_retry_attempts: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            hot_retention_days: 7,
            warm_retention_days: 30,
            enable_cold_archive: false,
            max_retry_attempts: 3,
        }
    }
}

impl RadiationDatabase {
    pub async fn new(
        scylla_config: scylla::ScyllaConfig,
        sqlite_path: &str,
        redis_url: &str,
        config: DatabaseConfig,
    ) -> Result<Self, DatabaseError> {
        let hot = Arc::new(
            ScyllaStorage::new(scylla_config)
                .await
                .map_err(|e| DatabaseError::Scylla(e.to_string()))?
        );

        let warm = Arc::new(
            SqliteStorage::new(sqlite_path)
                .await
                .map_err(|e| DatabaseError::Sqlite(e.to_string()))?
        );

        let cache = Arc::new(
            RedisCache::new(redis_url)
                .await
                .map_err(|e| DatabaseError::Redis(e.to_string()))?
        );

        info!("RadiationDatabase initialized with hot/warm/cold tiers");

        Ok(Self {
            hot,
            warm,
            cache,
            config,
        })
    }

    /// Write with automatic tier routing based on timestamp
    #[instrument(skip(self, reading))]
    pub async fn write_reading(&self, reading: &RadiationReading) -> Result<(), DatabaseError> {
        let reading_time = DateTime::from_timestamp(reading.timestamp, 0)
            .ok_or_else(|| DatabaseError::Query("Invalid timestamp".to_string()))?;

        let now = Utc::now();
        let age = now.signed_duration_since(reading_time);

        // Route to appropriate tier
        if age <= Duration::days(self.config.hot_retention_days) {
            // Hot tier: ScyllaDB for real-time queries
            self.write_to_hot(reading).await?;
        } else if age <= Duration::days(self.config.warm_retention_days) {
            // Warm tier: SQLite for analytical queries
            self.write_to_warm(reading).await?;
        } else if self.config.enable_cold_archive {
            // Cold tier: Object storage for historical archive
            // TODO: Implement cold storage tier
            warn!("Cold storage not yet implemented, dropping reading");
        }

        // Invalidate cache for this sensor
        self.cache.invalidate_sensor(&reading.sensor_id).await
            .map_err(|e| DatabaseError::Redis(e.to_string()))?;

        Ok(())
    }

    async fn write_to_hot(&self, reading: &RadiationReading) -> Result<(), DatabaseError> {
        let operation = || async {
            self.hot.write_reading(reading).await
                .map_err(|e| backoff::Error::transient(e.to_string()))
        };

        let backoff = ExponentialBackoff::default();
        
        retry(backoff, operation)
            .await
            .map_err(|e| DatabaseError::Scylla(format!("Failed after retries: {}", e)))
    }

    async fn write_to_warm(&self, reading: &RadiationReading) -> Result<(), DatabaseError> {
        let operation = || async {
            self.warm.write_reading(reading).await
                .map_err(|e| backoff::Error::transient(e.to_string()))
        };

        let backoff = ExponentialBackoff::default();
        
        retry(backoff, operation)
            .await
            .map_err(|e| DatabaseError::Sqlite(format!("Failed after retries: {}", e)))
    }

    /// Time-range query with aggregation across all tiers
    #[instrument(skip(self, sensor_ids))]
    pub async fn query_range(
        &self,
        sensor_ids: &[String],
        start_ts: i64,
        end_ts: i64,
        aggregation: AggregationLevel,
    ) -> Result<Vec<TimeSeriesPoint>, DatabaseError> {
        let start = DateTime::from_timestamp(start_ts, 0)
            .unwrap_or_else(|| Utc::now());
        let end = DateTime::from_timestamp(end_ts, 0)
            .unwrap_or_else(|| Utc::now());

        // Check cache first
        let cache_key = format!("query_range:{:?}:{:?}:{:?}", sensor_ids, start, end);
        if let Some(cached) = self.cache.get_query_result(&cache_key).await
            .map_err(|e| DatabaseError::Redis(e.to_string()))? 
        {
            return Ok(cached);
        }

        let now = Utc::now();
        let mut all_points = Vec::new();

        // Query hot tier (last 7 days)
        let hot_cutoff = now - Duration::days(self.config.hot_retention_days);
        if end > hot_cutoff {
            let hot_start = if start > hot_cutoff { start } else { hot_cutoff };
            let hot_points = self.query_hot_range(sensor_ids, hot_start, end, aggregation).await?;
            all_points.extend(hot_points);
        }

        // Query warm tier (7-30 days)
        let warm_cutoff = now - Duration::days(self.config.warm_retention_days);
        if start < hot_cutoff && end > warm_cutoff {
            let warm_start = if start > warm_cutoff { start } else { warm_cutoff };
            let warm_end = if end < hot_cutoff { end } else { hot_cutoff };
            let warm_points = self.query_warm_range(sensor_ids, warm_start, warm_end, aggregation).await?;
            all_points.extend(warm_points);
        }

        // Sort and aggregate results
        all_points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Cache result
        self.cache.set_query_result(&cache_key, &all_points, 300).await
            .map_err(|e| DatabaseError::Redis(e.to_string()))?;

        Ok(all_points)
    }

    async fn query_hot_range(
        &self,
        sensor_ids: &[String],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        _aggregation: AggregationLevel,
    ) -> Result<Vec<TimeSeriesPoint>, DatabaseError> {
        let mut points = Vec::new();

        for sensor_id in sensor_ids {
            let uuid = Uuid::parse_str(sensor_id)
                .map_err(|e| DatabaseError::Query(format!("Invalid UUID: {}", e)))?;

            let readings = self.hot.query_by_time_range(
                uuid,
                start.timestamp(),
                end.timestamp(),
            ).await.map_err(|e| DatabaseError::Scylla(e.to_string()))?;

            for reading in readings {
                points.push(TimeSeriesPoint {
                    timestamp: DateTime::from_timestamp(reading.timestamp, 0)
                        .unwrap_or_else(|| Utc::now()),
                    value: reading.dose_rate_microsieverts,
                    count: 1,
                    min: reading.dose_rate_microsieverts,
                    max: reading.dose_rate_microsieverts,
                    avg: reading.dose_rate_microsieverts,
                });
            }
        }

        Ok(points)
    }

    async fn query_warm_range(
        &self,
        sensor_ids: &[String],
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        aggregation: AggregationLevel,
    ) -> Result<Vec<TimeSeriesPoint>, DatabaseError> {
        self.warm.query_range(sensor_ids, start, end, aggregation).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))
    }

    /// Spatial query: sensors within radius
    #[instrument(skip(self))]
    pub async fn query_geo(
        &self,
        center: GeoPoint,
        radius_km: f64,
        time_window: TimeRange,
    ) -> Result<Vec<SensorReading>, DatabaseError> {
        // Use geohash for efficient spatial indexing
        let coord = geohash::Coord { x: center.longitude, y: center.latitude };
        let geohash_prefix = geohash::encode(coord, 4)
            .map_err(|e| DatabaseError::Query(format!("Geohash error: {:?}", e)))?;

        // Query hot tier by location
        let hot_readings = self.hot.query_by_location(
            &geohash_prefix,
            &geohash_prefix,
            time_window.start.timestamp(),
            time_window.end.timestamp(),
        ).await.map_err(|e| DatabaseError::Scylla(e.to_string()))?;

        let mut results: Vec<SensorReading> = hot_readings.into_iter()
            .filter(|r| {
                let distance = haversine_distance(
                    center.latitude,
                    center.longitude,
                    r.latitude,
                    r.longitude,
                );
                distance <= radius_km
            })
            .map(|r| SensorReading {
                sensor_id: r.sensor_id,
                timestamp: DateTime::from_timestamp(r.timestamp, 0).unwrap_or_else(|| Utc::now()),
                location: GeoPoint {
                    latitude: r.latitude,
                    longitude: r.longitude,
                },
                dose_rate: r.dose_rate_microsieverts,
                quality: r.quality_flag,
            })
            .collect();

        // Also query warm tier
        let warm_readings = self.warm.query_geo(center, radius_km, time_window).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))?;
        
        results.extend(warm_readings);

        Ok(results)
    }

    /// Get latest reading for a sensor
    pub async fn get_sensor_latest(
        &self,
        sensor_id: &str,
    ) -> Result<Option<RadiationReading>, DatabaseError> {
        let uuid = Uuid::parse_str(sensor_id)
            .map_err(|e| DatabaseError::Query(format!("Invalid UUID: {}", e)))?;

        // Check cache first
        if let Some(cached) = self.cache.get_sensor_latest(&uuid).await
            .map_err(|e| DatabaseError::Redis(e.to_string()))? 
        {
            return Ok(Some(cached));
        }

        // Query hot tier
        if let Some(reading) = self.hot.get_sensor_latest(uuid).await
            .map_err(|e| DatabaseError::Scylla(e.to_string()))? 
        {
            // Cache the result
            self.cache.set_sensor_latest(&uuid, &reading, 60).await
                .map_err(|e| DatabaseError::Redis(e.to_string()))?;
            return Ok(Some(reading));
        }

        // Fall back to warm tier
        if let Some(reading) = self.warm.get_sensor_latest(&uuid).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))? 
        {
            return Ok(Some(reading));
        }

        Ok(None)
    }

    /// Store domain event (for anomaly detection, alerts, etc.)
    #[instrument(skip(self, event))]
    pub async fn store_event(&self, event: &DomainEvent) -> Result<(), DatabaseError> {
        // Store events in warm tier (SQLite) for audit trail
        self.warm.store_event(event).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))?;
        
        info!("Stored domain event: {} ({:?})", event.event_id, event.event_type);
        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), DatabaseError> {
        self.warm.run_migrations().await
            .map_err(|e| DatabaseError::Migration(e.to_string()))?;
        
        info!("Database migrations completed successfully");
        Ok(())
    }


    /// Health check for all tiers
    pub async fn health_check(&self) -> DatabaseHealth {
        DatabaseHealth {
            hot: self.hot.health_check().await,
            warm: self.warm.health_check().await,
            cache: self.cache.health_check().await,
        }
    }

    /// Get anomalies since a given timestamp
    #[instrument(skip(self))]
    pub async fn get_anomalies(
        &self,
        since: i64,
        limit: usize,
    ) -> Result<Vec<AnomalyRecord>, DatabaseError> {
        self.warm.get_anomalies(since, limit).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))
    }

    /// Get count of anomalies in last N hours
    #[instrument(skip(self))]
    pub async fn get_anomaly_count(&self, hours: i64) -> Result<i64, DatabaseError> {
        self.warm.get_anomaly_count(hours).await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))
    }

    /// List all sensors with their latest location and timestamp
    #[instrument(skip(self))]
    pub async fn list_sensors(&self) -> Result<Vec<SensorRecord>, DatabaseError> {
        self.warm.list_sensors_with_location().await
            .map_err(|e| DatabaseError::Sqlite(e.to_string()))
    }
}


#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub hot: bool,
    pub warm: bool,
    pub cache: bool,
}

impl DatabaseHealth {
    pub fn is_healthy(&self) -> bool {
        self.hot && self.warm && self.cache
    }
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
