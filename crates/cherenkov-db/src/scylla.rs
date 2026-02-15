use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use tracing::{info, error};

pub struct ScyllaStorage {
    session: Arc<Session>,
}

impl ScyllaStorage {
    pub async fn new(uri: &str, keyspace: &str) -> anyhow::Result<Self> {
        let session = SessionBuilder::new()
            .known_node(uri)
            .build()
            .await?;
        
        session.query(format!("USE {}", keyspace), &[]).await?;
        
        info!("Connected to ScyllaDB at {}", uri);
        
        Ok(Self {
            session: Arc::new(session),
        })
    }
    
    pub async fn write_reading(&self, reading: &super::RadiationReading) -> anyhow::Result<()> {
        let query = "
            INSERT INTO radiation_readings 
            (sensor_id, bucket, timestamp, latitude, longitude, dose_rate, uncertainty, quality_flag, source, cell_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        self.session.query(query, (
            reading.sensor_id,
            reading.bucket,
            reading.timestamp,
            reading.latitude,
            reading.longitude,
            reading.dose_rate_microsieverts,
            reading.uncertainty,
            format!("{:?}", reading.quality_flag),
            &reading.source,
            &reading.cell_id,
        )).await?;
        
        Ok(())
    }
    
    pub async fn query_by_time_range(
        &self,
        sensor_id: uuid::Uuid,
        from: i64,
        to: i64,
    ) -> anyhow::Result<Vec<super::RadiationReading>> {
        let query = "
            SELECT * FROM radiation_readings 
            WHERE sensor_id = ? AND bucket IN ? AND timestamp >= ? AND timestamp <= ?
        ";
        
        let result = self.session.query(query, (sensor_id, vec![from / 3600, to / 3600], from, to)).await?;
        
        Ok(vec![])
    }
}
