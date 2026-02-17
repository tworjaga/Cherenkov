use async_graphql::{Object, ID, Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;

use cherenkov_db::{RadiationDatabase, AggregationLevel};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn sensors(&self, _ctx: &Context<'_>) -> Result<Vec<Sensor>> {
        // list_sensors API not available in current database implementation
        // Return empty list for now
        Ok(vec![])
    }
    
    async fn sensor(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Sensor>> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let sensor_id_str = id.to_string();
        
        // Use public API method - takes &str
        let latest = db.get_sensor_latest(&sensor_id_str).await
            .map_err(|e| async_graphql::Error::new(format!("Database error: {}", e)))?;
        
        Ok(latest.map(|r| Sensor {
            id: ID::from(r.sensor_id.to_string()),
            name: format!("Sensor {}", r.sensor_id),
            latitude: r.latitude,
            longitude: r.longitude,
            status: "active".to_string(),
            last_reading: Some(DateTime::from_timestamp(r.timestamp, 0)
                .unwrap_or_else(|| Utc::now())),
        }))
    }
    
    async fn readings(
        &self,
        ctx: &Context<'_>,
        sensor_ids: Vec<ID>,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        aggregation: Option<String>,
    ) -> Result<Vec<Reading>> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let ids: Vec<String> = sensor_ids.iter()
            .map(|id| id.to_string())
            .collect();
        
        let agg = match aggregation.as_deref() {
            Some("1m") => AggregationLevel::OneMinute,
            Some("5m") => AggregationLevel::FiveMinutes,
            Some("1h") => AggregationLevel::OneHour,
            Some("1d") => AggregationLevel::OneDay,
            _ => AggregationLevel::Raw,
        };
        
        let points = db.query_range(
            &ids,
            from.timestamp(),
            to.timestamp(),
            agg,
        ).await.map_err(|e| async_graphql::Error::new(format!("Query error: {}", e)))?;
        
        let readings: Vec<Reading> = points.into_iter()
            .map(|p| Reading {
                id: ID::from(Uuid::new_v4().to_string()),
                sensor_id: ID::from("unknown"),
                timestamp: p.timestamp,
                dose_rate: p.value,
                unit: "microsieverts_per_hour".to_string(),
            })
            .collect();
        
        Ok(readings)
    }
    
    async fn anomalies(
        &self,
        ctx: &Context<'_>,
        severity: Option<Vec<String>>,
        since: DateTime<Utc>,
        limit: Option<i32>,
    ) -> Result<Vec<Anomaly>> {
        let _db = ctx.data::<Arc<RadiationDatabase>>()?;
        let _ = (severity, since, limit);
        
        // Anomaly API not available in current database implementation
        // Return empty list for now
        Ok(vec![])
    }
    
    async fn facilities(&self, _ctx: &Context<'_>) -> Vec<Facility> {
        vec![]
    }
    
    async fn global_status(&self, ctx: &Context<'_>) -> Result<GlobalStatus> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let _health = db.health_check().await;
        // Anomaly count API not available - using placeholder
        let anomaly_count = 0;
        
        let defcon = if anomaly_count > 10 {
            2
        } else if anomaly_count > 5 {
            3
        } else if anomaly_count > 0 {
            4
        } else {
            5
        };
        
        Ok(GlobalStatus {
            defcon_level: defcon,
            status: if defcon <= 3 { "ELEVATED" } else { "NORMAL" }.to_string(),
            active_anomalies: anomaly_count as i32,
            last_updated: Utc::now(),
        })
    }
    
    async fn simulate_plume(
        &self,
        _ctx: &Context<'_>,
        lat: f64,
        lon: f64,
        release_rate: f64,
        duration_hours: u32,
        isotope: Option<String>,
    ) -> Result<PlumeSimulation> {
        // Plume simulation disabled - cherenkov_plume crate not available
        // Return placeholder data
        let _ = (release_rate, duration_hours, isotope);
        
        // Generate placeholder concentration grid
        let mut grid = vec![];
        for _x in (0..10000).step_by(500) {
            let mut row = vec![];
            for _y in (-5000..5000).step_by(500) {
                row.push(0.0);
            }
            grid.push(row);
        }
        
        Ok(PlumeSimulation {
            lat,
            lon,
            concentration_grid: grid,
            max_dose: 0.0,
        })
    }
}


#[derive(SimpleObject)]
pub struct Sensor {
    pub id: ID,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
    pub last_reading: Option<DateTime<Utc>>,
}

#[derive(SimpleObject)]
pub struct Reading {
    pub id: ID,
    pub sensor_id: ID,
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub unit: String,
}

#[derive(SimpleObject)]
pub struct Anomaly {
    pub id: ID,
    pub sensor_id: ID,
    pub severity: String,
    pub z_score: f64,
    pub detected_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct Facility {
    pub id: ID,
    pub name: String,
    pub facility_type: String,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
}

#[derive(SimpleObject)]
pub struct PlumeSimulation {
    pub lat: f64,
    pub lon: f64,
    pub concentration_grid: Vec<Vec<f64>>,
    pub max_dose: f64,
}

#[derive(SimpleObject)]
pub struct GlobalStatus {
    pub defcon_level: i32,
    pub status: String,
    pub active_anomalies: i32,
    pub last_updated: DateTime<Utc>,
}

use async_graphql::SimpleObject;
use async_graphql::Subscription;
use futures_util::stream::Stream;
use tokio::sync::broadcast;

/// Subscription root for real-time updates
pub struct SubscriptionRoot {
    sensor_tx: broadcast::Sender<SensorReading>,
    anomaly_tx: broadcast::Sender<AnomalyEvent>,
}

impl SubscriptionRoot {
    pub fn new() -> Self {
        let (sensor_tx, _) = broadcast::channel(1000);
        let (anomaly_tx, _) = broadcast::channel(100);
        Self {
            sensor_tx,
            anomaly_tx,
        }
    }

    pub fn get_sensor_sender(&self) -> broadcast::Sender<SensorReading> {
        self.sensor_tx.clone()
    }

    pub fn get_anomaly_sender(&self) -> broadcast::Sender<AnomalyEvent> {
        self.anomaly_tx.clone()
    }
}

#[derive(Clone, Debug)]
pub struct SensorReading {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug)]
pub struct AnomalyEvent {
    pub anomaly_id: String,
    pub sensor_id: String,
    pub severity: String,
    pub z_score: f64,
    pub detected_at: DateTime<Utc>,
    pub message: String,
    pub latitude: f64,
    pub longitude: f64,
}


#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to real-time sensor updates for a specific sensor
    async fn sensor_updates(
        &self,
        sensor_id: ID,
    ) -> impl Stream<Item = SensorUpdate> {
        let mut rx = self.sensor_tx.subscribe();
        let target_id = sensor_id.to_string();

        async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(reading) if reading.sensor_id == target_id => {
                        yield SensorUpdate {
                            sensor_id: ID::from(reading.sensor_id),
                            timestamp: reading.timestamp,
                            dose_rate: reading.dose_rate,
                            latitude: reading.latitude,
                            longitude: reading.longitude,
                        };
                    }
                    Ok(_) => continue,
                    Err(_) => break,
                }
            }
        }
    }

    /// Subscribe to anomaly alerts, optionally filtered by region
    async fn anomaly_alerts(
        &self,
        min_lat: Option<f64>,
        max_lat: Option<f64>,
        min_lon: Option<f64>,
        max_lon: Option<f64>,
    ) -> impl Stream<Item = AnomalyAlert> {
        let mut rx = self.anomaly_tx.subscribe();

        async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        // Apply region filter if provided
                        let in_bounds = match (min_lat, max_lat, min_lon, max_lon) {
                            (Some(min_lat), Some(max_lat), Some(min_lon), Some(max_lon)) => {
                                event.latitude >= min_lat && event.latitude <= max_lat &&
                                event.longitude >= min_lon && event.longitude <= max_lon
                            }
                            _ => true,
                        };
                        
                        if in_bounds {
                            yield AnomalyAlert {
                                anomaly_id: ID::from(event.anomaly_id),
                                sensor_id: ID::from(event.sensor_id),
                                severity: event.severity,
                                z_score: event.z_score,
                                detected_at: event.detected_at,
                                message: event.message,
                            };
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    /// Subscribe to all sensor readings (broadcast)
    async fn all_sensor_updates(&self) -> impl Stream<Item = SensorUpdate> {
        let mut rx = self.sensor_tx.subscribe();

        async_stream::stream! {
            loop {
                match rx.recv().await {
                    Ok(reading) => {
                        yield SensorUpdate {
                            sensor_id: ID::from(reading.sensor_id),
                            timestamp: reading.timestamp,
                            dose_rate: reading.dose_rate,
                            latitude: reading.latitude,
                            longitude: reading.longitude,
                        };
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct SensorUpdate {
    pub sensor_id: ID,
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(SimpleObject, Clone)]
pub struct AnomalyAlert {
    pub anomaly_id: ID,
    pub sensor_id: ID,
    pub severity: String,
    pub z_score: f64,
    pub detected_at: DateTime<Utc>,
    pub message: String,
}
