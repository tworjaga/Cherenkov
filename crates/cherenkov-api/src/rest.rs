use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, debug, error};
use uuid::Uuid;

use cherenkov_db::{RadiationDatabase, AggregationLevel};
use crate::auth::AuthState;
use crate::websocket::WebSocketState;

/// REST API router - uses same state type as main app
pub fn create_router() -> Router<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)> {
    Router::new()
        .route("/sensors", get(list_sensors))
        .route("/sensors/:id", get(get_sensor))
        .route("/sensors/:id/readings", get(get_sensor_readings))
        .route("/sensors/nearby", get(get_nearby_sensors))
        .route("/status", get(get_global_status))
        .route("/anomalies", get(list_anomalies))
        .route("/alerts/:id/acknowledge", get(acknowledge_alert))
}

/// List all sensors
async fn list_sensors(
    State((_, _db, _)): State<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>,
) -> Json<Vec<SensorResponse>> {
    debug!("Listing all sensors");
    
    // Query from database using public API
    // For now return empty list - would need to add list_sensors to RadiationDatabase
    let sensors: Vec<SensorResponse> = vec![];
    
    Json(sensors)
}

/// Get sensor by ID
async fn get_sensor(
    State((_, db, _)): State<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>,
    Path(id): Path<String>,
) -> Result<Json<SensorResponse>, StatusCode> {
    debug!("Getting sensor: {}", id);
    
    // Try to get latest reading for this sensor using public API
    let latest = match db.get_sensor_latest(&id).await {
        Ok(Some(r)) => r,
        _ => {
            return Err(StatusCode::NOT_FOUND);
        }
    };
    
    let response = SensorResponse {
        id: id.clone(),
        name: format!("Sensor {}", id),
        latitude: latest.latitude,
        longitude: latest.longitude,
        status: "active".to_string(),
        last_reading: DateTime::from_timestamp(latest.timestamp, 0),
    };
    
    Ok(Json(response))
}

/// Get sensor readings with time range
async fn get_sensor_readings(
    State((_, db, _)): State<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>,
    Path(id): Path<String>,
    Query(params): Query<ReadingsQuery>,
) -> Result<Json<Vec<ReadingResponse>>, StatusCode> {
    debug!("Getting readings for sensor: {}", id);
    
    let sensor_id = Uuid::parse_str(&id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let from = params.from.timestamp();
    let to = params.to.timestamp();
    
    let aggregation = params.aggregation
        .map(|a| match a.as_str() {
            "1m" => AggregationLevel::OneMinute,
            "5m" => AggregationLevel::FiveMinutes,
            "1h" => AggregationLevel::OneHour,
            "1d" => AggregationLevel::OneDay,
            _ => AggregationLevel::Raw,
        })
        .unwrap_or(AggregationLevel::Raw);
    
    let readings = match db.query_range(
        &[sensor_id.to_string()],
        from,
        to,
        aggregation,
    ).await {
        Ok(points) => points.into_iter().map(|p| ReadingResponse {
            id: Uuid::new_v4().to_string(),
            sensor_id: id.clone(),
            timestamp: p.timestamp,
            dose_rate: p.value,
            unit: "microsieverts_per_hour".to_string(),
        }).collect(),
        Err(e) => {
            error!("Failed to query readings: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(readings))
}

/// Get nearby sensors
async fn get_nearby_sensors(
    State((_, db, _)): State<(Arc<WebSocketState>, Arc<RadiationDatabase>, Arc<AuthState>)>,
    Query(params): Query<NearbyQuery>,
) -> Json<Vec<SensorResponse>> {
    debug!("Finding sensors near {}, {}", params.lat, params.lon);
    
    // Query geo-spatial
    let center = cherenkov_db::GeoPoint {
        latitude: params.lat,
        longitude: params.lon,
    };
    
    let time_window = cherenkov_db::TimeRange {
        start: Utc::now() - chrono::Duration::hours(1),
        end: Utc::now(),
    };
    
    let readings = match db.query_geo(
        center,
        params.radius_km,
        time_window,
    ).await {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to query geo: {}", e);
            vec![]
        }
    };
    
    let sensors: Vec<SensorResponse> = readings.into_iter()
        .map(|r| SensorResponse {
            id: r.sensor_id.to_string(),
            name: format!("Sensor {}", r.sensor_id),
            latitude: r.location.latitude,
            longitude: r.location.longitude,
            status: "active".to_string(),
            last_reading: Some(r.timestamp),
        })
        .collect();
    
    Json(sensors)
}

/// Get global status (DEFCON indicator)
async fn get_global_status() -> Json<GlobalStatusResponse> {
    // TODO: Calculate actual DEFCON level based on anomaly data
    Json(GlobalStatusResponse {
        defcon_level: 5,
        status: "NORMAL".to_string(),
        active_anomalies: 0,
        total_sensors: 0,
        last_updated: Utc::now(),
    })
}

/// List anomalies
async fn list_anomalies(
    Query(params): Query<AnomaliesQuery>,
) -> Json<Vec<AnomalyResponse>> {
    debug!("Listing anomalies with severity: {:?}", params.severity);
    
    // TODO: Query from database
    Json(vec![])
}

/// Acknowledge alert
async fn acknowledge_alert(
    Path(id): Path<String>,
) -> Result<Json<AckResponse>, StatusCode> {
    info!("Acknowledging alert: {}", id);
    
    // TODO: Store acknowledgment in database
    
    Ok(Json(AckResponse {
        alert_id: id,
        acknowledged_at: Utc::now(),
        status: "acknowledged".to_string(),
    }))
}

use axum::http::StatusCode;

// Request/Response types

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ReadingsQuery {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub aggregation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NearbyQuery {
    pub lat: f64,
    pub lon: f64,
    pub radius_km: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AnomaliesQuery {
    pub severity: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SensorResponse {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub status: String,
    pub last_reading: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct ReadingResponse {
    pub id: String,
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub dose_rate: f64,
    pub unit: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct GlobalStatusResponse {
    pub defcon_level: i32,
    pub status: String,
    pub active_anomalies: i32,
    pub total_sensors: i32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct AnomalyResponse {
    pub id: String,
    pub sensor_id: String,
    pub severity: String,
    pub z_score: f64,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct AckResponse {
    pub alert_id: String,
    pub acknowledged_at: DateTime<Utc>,
    pub status: String,
}
