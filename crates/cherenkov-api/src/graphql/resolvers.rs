use async_graphql::{Object, ID, Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, error};

use cherenkov_db::{RadiationDatabase, AggregationLevel};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn sensors(&self, ctx: &Context<'_>) -> Result<Vec<Sensor>> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let sensors = db.warm.list_sensors().await
            .map_err(|e| async_graphql::Error::new(format!("Database error: {}", e)))?;
        
        let result: Vec<Sensor> = sensors.into_iter()
            .map(|s| Sensor {
                id: ID::from(s.sensor_id.to_string()),
                name: format!("Sensor {}", s.sensor_id),
                latitude: 0.0,
                longitude: 0.0,
                status: "active".to_string(),
                last_reading: None,
            })
            .collect();
        
        Ok(result)
    }
    
    async fn sensor(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Sensor>> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let sensor_id = Uuid::parse_str(&id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;
        
        let latest = db.hot.get_sensor_latest(sensor_id).await
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
                sensor_id: ID::from(p.sensor_id),
                timestamp: DateTime::from_timestamp(p.timestamp, 0)
                    .unwrap_or_else(|| Utc::now()),
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
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let lim = limit.unwrap_or(100) as usize;
        
        let anomalies = db.warm.get_anomalies(since.timestamp(), lim).await
            .map_err(|e| async_graphql::Error::new(format!("Database error: {}", e)))?;
        
        let result: Vec<Anomaly> = anomalies.into_iter()
            .filter(|a| {
                if let Some(ref sev) = severity {
                    sev.contains(&a.severity)
                } else {
                    true
                }
            })
            .map(|a| Anomaly {
                id: ID::from(a.anomaly_id),
                sensor_id: ID::from(a.sensor_id.to_string()),
                severity: a.severity,
                z_score: a.z_score,
                detected_at: DateTime::from_timestamp(a.detected_at, 0)
                    .unwrap_or_else(|| Utc::now()),
            })
            .collect();
        
        Ok(result)
    }
    
    async fn facilities(&self, ctx: &Context<'_>) -> Vec<Facility> {
        vec![]
    }
    
    async fn global_status(&self, ctx: &Context<'_>) -> Result<GlobalStatus> {
        let db = ctx.data::<Arc<RadiationDatabase>>()?;
        
        let health = db.health_check().await;
        let anomaly_count = db.warm.get_anomaly_count(24).await.unwrap_or(0);
        
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
        ctx: &Context<'_>,
        lat: f64,
        lon: f64,
        release_rate: f64,
        duration_hours: u32,
        isotope: Option<String>,
    ) -> Result<PlumeSimulation> {
        use cherenkov_plume::dispersion::{GaussianPlumeModel, WeatherConditions, StabilityClass};
        use cherenkov_plume::ReleaseParameters;
        
        let release = ReleaseParameters {
            latitude: lat,
            longitude: lon,
            altitude_m: 50.0,
            release_rate_bq_s: release_rate,
            duration_hours,
            isotope: isotope.unwrap_or_else(|| "Cs-137".to_string()),
            particle_size_um: 1.0,
        };
        
        let weather = WeatherConditions {
            wind_speed_ms: 5.0,
            wind_direction_deg: 0.0,
            stability_class: StabilityClass::D,
            temperature_k: 288.15,
            pressure_pa: 101325.0,
        };
        
        let model = GaussianPlumeModel::new(weather, release);
        
        // Generate concentration grid
        let mut grid = vec![];
        for x in (0..10000).step_by(500) {
            let x = x as f64;
            let mut row = vec![];
            for y in (-5000..5000).step_by(500) {
                let y = y as f64;
                let dose = model.ground_level_dose_rate(x, y);
                row.push(dose);
            }
            grid.push(row);
        }
        
        Ok(PlumeSimulation {
            lat,
            lon,
            concentration_grid: grid,
            max_dose: model.centerline_dose_rate(1000.0),
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
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

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
        let rx = self.sensor_tx.subscribe();
        let target_id = sensor_id.to_string();

        BroadcastStream::new(rx)
            .filter_map(move |result| {
                let target = target_id.clone();
                async move {
                    match result {
                        Ok(reading) if reading.sensor_id == target => {
                            Some(SensorUpdate {
                                sensor_id: ID::from(reading.sensor_id),
                                timestamp: reading.timestamp,
                                dose_rate: reading.dose_rate,
                                latitude: reading.latitude,
                                longitude: reading.longitude,
                            })
                        }
                        _ => None,
                    }
                }
            })
    }

    /// Subscribe to anomaly alerts, optionally filtered by region
    async fn anomaly_alerts(
        &self,
        region: Option<GeoRegionInput>,
    ) -> impl Stream<Item = AnomalyAlert> {
        let rx = self.anomaly_tx.subscribe();

        BroadcastStream::new(rx)
            .filter_map(move |result| {
                let region_filter = region.clone();
                async move {
                    match result {
                        Ok(event) => {
                            // Apply region filter if provided
                            if let Some(ref reg) = region_filter {
                                // Simple bounding box check
                                if !reg.contains(event.latitude, event.longitude) {
                                    return None;
                                }
                            }
                            Some(AnomalyAlert {
                                anomaly_id: ID::from(event.anomaly_id),
                                sensor_id: ID::from(event.sensor_id),
                                severity: event.severity,
                                z_score: event.z_score,
                                detected_at: event.detected_at,
                                message: event.message,
                            })
                        }
                        _ => None,
                    }
                }
            })
    }

    /// Subscribe to all sensor readings (broadcast)
    async fn all_sensor_updates(&self) -> impl Stream<Item = SensorUpdate> {
        let rx = self.sensor_tx.subscribe();

        BroadcastStream::new(rx)
            .filter_map(|result| async move {
                match result {
                    Ok(reading) => Some(SensorUpdate {
                        sensor_id: ID::from(reading.sensor_id),
                        timestamp: reading.timestamp,
                        dose_rate: reading.dose_rate,
                        latitude: reading.latitude,
                        longitude: reading.longitude,
                    }),
                    _ => None,
                }
            })
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

#[derive(InputObject, Clone)]
pub struct GeoRegionInput {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl GeoRegionInput {
    fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat &&
        lon >= self.min_lon && lon <= self.max_lon
    }
}
