use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0,
    })
}

#[derive(Deserialize)]
pub struct ReadingsQuery {
    pub sensor_id: String,
    pub from: String,
    pub to: String,
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct ReadingsResponse {
    pub sensor_id: String,
    pub readings: Vec<Reading>,
}

#[derive(Serialize)]
pub struct Reading {
    pub timestamp: String,
    pub dose_rate: f64,
    pub unit: String,
}

pub async fn get_readings(
    Query(params): Query<ReadingsQuery>,
) -> Json<ReadingsResponse> {
    Json(ReadingsResponse {
        sensor_id: params.sensor_id,
        readings: vec![],
    })
}

pub async fn get_sensor(
    Path(id): Path<String>,
) -> Json<HashMap<String, String>> {
    let mut sensor = HashMap::new();
    sensor.insert("id".to_string(), id);
    sensor.insert("status".to_string(), "active".to_string());
    Json(sensor)
}
