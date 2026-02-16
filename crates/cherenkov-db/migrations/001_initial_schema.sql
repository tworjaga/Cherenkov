-- Initial schema for SQLite warm storage tier

-- Main readings table with time-series optimized structure
CREATE TABLE IF NOT EXISTS radiation_readings_warm (
    sensor_id TEXT NOT NULL,
    bucket INTEGER NOT NULL,
    timestamp DATETIME NOT NULL,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    dose_rate REAL NOT NULL,
    uncertainty REAL,
    quality_flag TEXT NOT NULL DEFAULT 'Valid',
    source TEXT NOT NULL,
    cell_id TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (sensor_id, bucket, timestamp)
) WITHOUT ROWID;

-- Index for time-range queries
CREATE INDEX IF NOT EXISTS idx_readings_timestamp 
ON radiation_readings_warm(timestamp);

-- Index for geospatial queries
CREATE INDEX IF NOT EXISTS idx_readings_location 
ON radiation_readings_warm(latitude, longitude);

-- Index for source filtering
CREATE INDEX IF NOT EXISTS idx_readings_source 
ON radiation_readings_warm(source);

-- Sensor metadata table
CREATE TABLE IF NOT EXISTS sensors (
    sensor_id TEXT PRIMARY KEY,
    name TEXT,
    location TEXT,
    source TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    first_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_reading DATETIME,
    latitude REAL,
    longitude REAL,
    metadata TEXT -- JSON blob for additional sensor metadata
);

-- Index for sensor status queries
CREATE INDEX IF NOT EXISTS idx_sensors_status 
ON sensors(status);

-- Index for sensor source queries
CREATE INDEX IF NOT EXISTS idx_sensors_source 
ON sensors(source);

-- Anomalies detected by stream processor
CREATE TABLE IF NOT EXISTS anomalies (
    anomaly_id TEXT PRIMARY KEY,
    sensor_id TEXT NOT NULL,
    severity TEXT NOT NULL,
    z_score REAL NOT NULL,
    detected_at DATETIME NOT NULL,
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_at DATETIME,
    acknowledged_by TEXT,
    notes TEXT,
    
    FOREIGN KEY (sensor_id) REFERENCES sensors(sensor_id)
);

-- Index for unacknowledged anomalies
CREATE INDEX IF NOT EXISTS idx_anomalies_unacknowledged 
ON anomalies(acknowledged, detected_at) 
WHERE acknowledged = FALSE;

-- Index for sensor anomalies
CREATE INDEX IF NOT EXISTS idx_anomalies_sensor 
ON anomalies(sensor_id, detected_at);

-- Aggregation cache for common queries
CREATE TABLE IF NOT EXISTS aggregation_cache (
    cache_key TEXT PRIMARY KEY,
    sensor_ids TEXT NOT NULL, -- JSON array
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    aggregation_level TEXT NOT NULL,
    data TEXT NOT NULL, -- JSON blob of aggregated data
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL
);

-- Index for cache expiration
CREATE INDEX IF NOT EXISTS idx_cache_expires 
ON aggregation_cache(expires_at);

-- Maintenance: Clean up expired cache entries
CREATE TRIGGER IF NOT EXISTS cleanup_expired_cache
AFTER INSERT ON aggregation_cache
BEGIN
    DELETE FROM aggregation_cache WHERE expires_at < datetime('now');
END;

-- Migration tracking
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);

-- Insert initial migration record
INSERT OR IGNORE INTO schema_migrations (version, description) 
VALUES (1, 'Initial schema: readings, sensors, anomalies, cache tables');
