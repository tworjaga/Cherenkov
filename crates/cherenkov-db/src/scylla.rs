use scylla::{Session, SessionBuilder, ExecutionProfile};
use scylla::frame::types::Consistency;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};
use tokio::sync::Semaphore;

pub struct ScyllaStorage {
    session: Arc<Session>,
    write_semaphore: Arc<Semaphore>,
    read_semaphore: Arc<Semaphore>,
}

pub struct ScyllaConfig {
    pub nodes: Vec<String>,
    pub keyspace: String,
    pub local_dc: String,
    pub write_consistency: Consistency,
    pub read_consistency: Consistency,
    pub max_concurrent_writes: usize,
    pub max_concurrent_reads: usize,
    pub connection_timeout: Duration,
}

impl Default for ScyllaConfig {
    fn default() -> Self {
        Self {
            nodes: vec!["127.0.0.1:9042".to_string()],
            keyspace: "cherenkov".to_string(),
            local_dc: "datacenter1".to_string(),
            write_consistency: Consistency::LocalQuorum,
            read_consistency: Consistency::LocalOne,
            max_concurrent_writes: 1000,
            max_concurrent_reads: 500,
            connection_timeout: Duration::from_secs(5),
        }
    }
}

impl ScyllaStorage {
    pub async fn new(config: ScyllaConfig) -> anyhow::Result<Self> {
        let execution_profile = ExecutionProfile::builder()
            .consistency(config.write_consistency)
            .request_timeout(Some(config.connection_timeout))
            .build();

        let session = SessionBuilder::new()
            .known_nodes(&config.nodes)
            .default_execution_profile_handle(execution_profile.into_handle())
            .build()
            .await?;

        session.query(format!("USE {}", config.keyspace), &[]).await?;
        
        info!("Connected to ScyllaDB cluster with {} nodes", config.nodes.len());
        
        Ok(Self {
            session: Arc::new(session),
            write_semaphore: Arc::new(Semaphore::new(config.max_concurrent_writes)),
            read_semaphore: Arc::new(Semaphore::new(config.max_concurrent_reads)),
        })
    }
    
    pub async fn write_reading(&self, reading: &super::RadiationReading) -> anyhow::Result<()> {
        let _permit = self.write_semaphore.acquire().await?;
        
        let query = "
            INSERT INTO radiation_readings 
            (sensor_id, bucket, timestamp, latitude, longitude, dose_rate, uncertainty, quality_flag, source, cell_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        let prepared = self.session.prepare(query).await?;
        
        self.session.execute(&prepared, (
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
    
    pub async fn write_batch(&self, readings: &[super::RadiationReading]) -> anyhow::Result<()> {
        let _permit = self.write_semaphore.acquire().await?;
        
        if readings.is_empty() {
            return Ok(());
        }
        
        let query = "
            INSERT INTO radiation_readings 
            (sensor_id, bucket, timestamp, latitude, longitude, dose_rate, uncertainty, quality_flag, source, cell_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        let prepared = self.session.prepare(query).await?;
        
        let mut batch = scylla::batch::Batch::new(scylla::batch::BatchType::Unlogged);
        
        for _reading in readings {
            batch.append_statement(prepared.clone());
        }
        
        let values: Vec<_> = readings.iter().map(|r| (
            r.sensor_id,
            r.bucket,
            r.timestamp,
            r.latitude,
            r.longitude,
            r.dose_rate_microsieverts,
            r.uncertainty,
            format!("{:?}", r.quality_flag),
            &r.source,
            &r.cell_id,
        )).collect();
        
        self.session.batch(&batch, &values).await?;
        
        Ok(())
    }
    
    pub async fn query_by_time_range(
        &self,
        sensor_id: uuid::Uuid,
        from: i64,
        to: i64,
    ) -> anyhow::Result<Vec<super::RadiationReading>> {
        let _permit = self.read_semaphore.acquire().await?;
        
        let query = "
            SELECT * FROM radiation_readings 
            WHERE sensor_id = ? AND bucket IN ? AND timestamp >= ? AND timestamp <= ?
        ";
        
        let buckets: Vec<i64> = ((from / 3600)..=(to / 3600)).collect();
        
        let prepared = self.session.prepare(query).await?;
        let result = self.session.execute(&prepared, (sensor_id, &buckets, from, to)).await?;
        
        let mut readings = Vec::new();
        let rows = result.rows()?;
        for row in rows {
            let reading = parse_row_to_reading(row)?;
            readings.push(reading);
        }
        
        Ok(readings)
    }
    
    pub async fn query_by_location(
        &self,
        cell_id: &str,
        geohash_prefix: &str,
        from: i64,
        to: i64,
    ) -> anyhow::Result<Vec<super::RadiationReading>> {
        let _permit = self.read_semaphore.acquire().await?;
        
        let query = "
            SELECT * FROM readings_by_location 
            WHERE cell_id = ? AND geohash_4 = ? AND timestamp >= ? AND timestamp <= ?
        ";
        
        let prepared = self.session.prepare(query).await?;
        let result = self.session.execute(&prepared, (cell_id, geohash_prefix, from, to)).await?;
        
        let mut readings = Vec::new();
        let rows = result.rows()?;
        for row in rows {
            let reading = parse_row_to_reading(row)?;
            readings.push(reading);
        }
        
        Ok(readings)
    }
    
    pub async fn get_sensor_latest(
        &self,
        sensor_id: uuid::Uuid,
    ) -> anyhow::Result<Option<super::RadiationReading>> {
        let _permit = self.read_semaphore.acquire().await?;
        
        let query = "
            SELECT * FROM radiation_readings 
            WHERE sensor_id = ? 
            LIMIT 1
        ";
        
        let prepared = self.session.prepare(query).await?;
        let result = self.session.execute(&prepared, (sensor_id,)).await?;
        
        let rows = result.rows()?;
        if let Some(row) = rows.into_iter().next() {
            return Ok(Some(parse_row_to_reading(row)?));
        }
        Ok(None)
    }
    
    pub fn get_session(&self) -> Arc<Session> {
        self.session.clone()
    }
    
    pub async fn health_check(&self) -> bool {
        match self.session.query("SELECT now() FROM system.local", &[]).await {
            Ok(_) => true,
            Err(e) => {
                warn!("ScyllaDB health check failed: {}", e);
                false
            }
        }
    }
}

/// Helper function to parse a Scylla row into RadiationReading
fn parse_row_to_reading(row: scylla::frame::response::result::Row) -> anyhow::Result<super::RadiationReading> {
    use scylla::frame::response::result::CqlValue;
    
    let columns = row.columns;
    if columns.len() < 10 {
        return Err(anyhow::anyhow!("Row has insufficient columns: {}", columns.len()));
    }
    
    let sensor_id = match &columns[0] {
        Some(CqlValue::Uuid(u)) => *u,
        _ => return Err(anyhow::anyhow!("Invalid sensor_id")),
    };
    
    let bucket = match &columns[1] {
        Some(CqlValue::BigInt(b)) => *b,
        _ => return Err(anyhow::anyhow!("Invalid bucket")),
    };
    
    let timestamp = match &columns[2] {
        Some(CqlValue::BigInt(t)) => *t,
        _ => return Err(anyhow::anyhow!("Invalid timestamp")),
    };
    
    let latitude = match &columns[3] {
        Some(CqlValue::Double(l)) => *l,
        _ => return Err(anyhow::anyhow!("Invalid latitude")),
    };
    
    let longitude = match &columns[4] {
        Some(CqlValue::Double(l)) => *l,
        _ => return Err(anyhow::anyhow!("Invalid longitude")),
    };
    
    let dose_rate_microsieverts = match &columns[5] {
        Some(CqlValue::Double(d)) => *d,
        _ => return Err(anyhow::anyhow!("Invalid dose_rate")),
    };
    
    let uncertainty = match &columns[6] {
        Some(CqlValue::Float(u)) => *u,
        Some(CqlValue::Double(u)) => *u as f32,
        _ => return Err(anyhow::anyhow!("Invalid uncertainty")),
    };
    
    let quality_flag = match &columns[7] {
        Some(CqlValue::Text(q)) => match q.as_str() {
            "Valid" => super::QualityFlag::Valid,
            "Suspect" => super::QualityFlag::Suspect,
            _ => super::QualityFlag::Invalid,
        },
        _ => super::QualityFlag::Invalid,
    };
    
    let source = match &columns[8] {
        Some(CqlValue::Text(s)) => s.clone(),
        _ => String::new(),
    };
    
    let cell_id = match &columns[9] {
        Some(CqlValue::Text(c)) => c.clone(),
        _ => String::new(),
    };
    
    Ok(super::RadiationReading {
        sensor_id,
        bucket,
        timestamp,
        latitude,
        longitude,
        dose_rate_microsieverts,
        uncertainty,
        quality_flag,
        source,
        cell_id,
    })
}
