pub const CREATE_KEYSPACE: &str = "
    CREATE KEYSPACE IF NOT EXISTS cherenkov
    WITH REPLICATION = {
        'class': 'NetworkTopologyStrategy',
        'replication_factor': 3
    }
";

pub const CREATE_READINGS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS radiation_readings (
        sensor_id UUID,
        bucket BIGINT,
        timestamp BIGINT,
        latitude DOUBLE,
        longitude DOUBLE,
        dose_rate DOUBLE,
        uncertainty FLOAT,
        quality_flag TEXT,
        source TEXT,
        cell_id TEXT,
        PRIMARY KEY ((sensor_id, bucket), timestamp)
    ) WITH CLUSTERING ORDER BY (timestamp DESC)
    AND compaction = {'class': 'TimeWindowCompactionStrategy', 'compaction_window_unit': 'HOURS', 'compaction_window_size': 1}
";

pub const CREATE_EVENTS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS domain_events (
        event_id TEXT,
        event_type TEXT,
        aggregate_id UUID,
        payload TEXT,
        timestamp BIGINT,
        PRIMARY KEY (event_type, timestamp, event_id)
    ) WITH CLUSTERING ORDER BY (timestamp DESC)
";

pub const CREATE_SENSORS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS sensors (
        sensor_id UUID PRIMARY KEY,
        name TEXT,
        location TEXT,
        source TEXT,
        status TEXT,
        last_reading TIMESTAMP
    )
";

pub const CREATE_MATERIALIZED_VIEW_BY_LOCATION: &str = "
    CREATE MATERIALIZED VIEW IF NOT EXISTS readings_by_location AS
    SELECT * FROM radiation_readings
    PRIMARY KEY ((cell_id), latitude, longitude, timestamp, sensor_id)
";

pub const CREATE_MATERIALIZED_VIEW_BY_TIME: &str = "
    CREATE MATERIALIZED VIEW IF NOT EXISTS readings_by_time AS
    SELECT * FROM radiation_readings
    PRIMARY KEY ((bucket), timestamp, sensor_id)
";
