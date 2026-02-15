use uuid::Uuid;

pub struct TimeRangeQuery {
    pub sensor_ids: Vec<Uuid>,
    pub from_timestamp: i64,
    pub to_timestamp: i64,
    pub aggregation: Aggregation,
}

pub enum Aggregation {
    Raw,
    Minute,
    Hour,
    Day,
}

pub struct SpatialQuery {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
    pub active_only: bool,
}

pub struct AnomalyQuery {
    pub severity: Option<Vec<String>>,
    pub since: i64,
    pub limit: usize,
}

impl TimeRangeQuery {
    pub fn new(sensor_ids: Vec<Uuid>, from: i64, to: i64) -> Self {
        Self {
            sensor_ids,
            from_timestamp: from,
            to_timestamp: to,
            aggregation: Aggregation::Raw,
        }
    }
    
    pub fn with_aggregation(mut self, agg: Aggregation) -> Self {
        self.aggregation = agg;
        self
    }
}

impl SpatialQuery {
    pub fn bounding_box(min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            max_lat,
            min_lon,
            max_lon,
            active_only: true,
        }
    }
}
