use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Semaphore, RwLock};
use tokio::time::timeout;
use futures::stream::{FuturesUnordered, StreamExt};
use tracing::{info, warn, error, instrument, debug};
use dashmap::DashMap;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use cherenkov_db::{RadiationDatabase, RadiationReading, QualityFlag};
use cherenkov_core::{EventBus, CherenkovEvent, NormalizedReading};


/// Configuration for the ingestion pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Maximum number of concurrent source fetches
    pub max_concurrent_sources: usize,
    /// Channel buffer size for backpressure
    pub channel_buffer_size: usize,
    /// Batch size for database writes
    pub batch_size: usize,
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u64,
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker reset timeout in seconds
    pub circuit_breaker_reset_secs: u64,
    /// Dead letter queue max size
    pub dlq_max_size: usize,
    /// Deduplication window in seconds
    pub dedup_window_secs: u64,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sources: 10,
            channel_buffer_size: 10000,
            batch_size: 100,
            batch_timeout_ms: 1000,
            circuit_breaker_threshold: 5,
            circuit_breaker_reset_secs: 30,
            dlq_max_size: 10000,
            dedup_window_secs: 60,
        }
    }
}

/// Circuit breaker for fault tolerance
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_count: Arc<RwLock<u32>>,
    last_failure: Arc<RwLock<Option<Instant>>>,
    threshold: u32,
    reset_timeout: Duration,
    state: Arc<RwLock<CircuitState>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(threshold: u32, reset_timeout_secs: u64) -> Self {
        Self {
            failure_count: Arc::new(RwLock::new(0)),
            last_failure: Arc::new(RwLock::new(None)),
            threshold,
            reset_timeout: Duration::from_secs(reset_timeout_secs),
            state: Arc::new(RwLock::new(CircuitState::Closed)),
        }
    }

    pub async fn can_execute(&self) -> bool {
        let state = *self.state.read().await;
        
        match state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if reset timeout has passed
                let last = *self.last_failure.read().await;
                if let Some(last_failure) = last {
                    if last_failure.elapsed() >= self.reset_timeout {
                        *self.state.write().await = CircuitState::HalfOpen;
                        info!("Circuit breaker transitioning to HalfOpen");
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    pub async fn record_success(&self) {
        let mut state = self.state.write().await;
        if *state == CircuitState::HalfOpen {
            *state = CircuitState::Closed;
            *self.failure_count.write().await = 0;
            info!("Circuit breaker closed after successful test");
        }
    }

    pub async fn record_failure(&self) {
        let mut count = self.failure_count.write().await;
        *count += 1;
        *self.last_failure.write().await = Some(Instant::now());

        if *count >= self.threshold {
            let mut state = self.state.write().await;
            if *state != CircuitState::Open {
                *state = CircuitState::Open;
                warn!("Circuit breaker opened after {} failures", *count);
            }
        }
    }
}

/// Dead letter queue for failed writes
#[derive(Debug)]
pub struct DeadLetterQueue {
    queue: Arc<RwLock<Vec<DeadLetterEntry>>>,
    max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterEntry {
    pub reading: RadiationReading,
    pub error: String,
    pub timestamp: i64,
    pub retry_count: u32,
}

impl DeadLetterQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(Vec::with_capacity(max_size))),
            max_size,
        }
    }

    pub async fn store(&self, reading: RadiationReading, error: String) {
        let entry = DeadLetterEntry {
            reading,
            error,
            timestamp: Utc::now().timestamp(),
            retry_count: 0,
        };

        let mut queue = self.queue.write().await;
        
        if queue.len() >= self.max_size {
            // Remove oldest entry
            queue.remove(0);
            warn!("DLQ at capacity, removed oldest entry");
        }
        
        queue.push(entry);
        metrics::counter!("cherenkov_dlq_entries_total").increment(1);
    }

    pub async fn get_entries(&self) -> Vec<DeadLetterEntry> {
        self.queue.read().await.clone()
    }

    pub async fn remove_entry(&self, index: usize) -> Option<DeadLetterEntry> {
        let mut queue = self.queue.write().await;
        if index < queue.len() {
            Some(queue.remove(index))
        } else {
            None
        }
    }

    pub async fn len(&self) -> usize {
        self.queue.read().await.len()
    }

    pub async fn is_empty(&self) -> bool {
        self.queue.read().await.is_empty()
    }

    /// Replay entries from DLQ
    pub async fn replay<F, Fut>(&self, mut processor: F) -> usize
    where
        F: FnMut(DeadLetterEntry) -> Fut,
        Fut: std::future::Future<Output = Result<(), ()>>,
    {
        let entries = self.get_entries().await;
        let mut replayed = 0;

        for (index, entry) in entries.iter().enumerate() {
            match processor(entry.clone()).await {
                Ok(()) => {
                    self.remove_entry(index).await;
                    replayed += 1;
                }
                Err(()) => {
                    warn!("Failed to replay DLQ entry {}", index);
                }
            }
        }

        replayed
    }
}

/// Deduplication tracker
#[derive(Debug)]
pub struct Deduplicator {
    seen: DashMap<String, i64>, // sensor_id -> last_timestamp
    window_secs: u64,
}

impl Deduplicator {
    pub fn new(window_secs: u64) -> Self {
        Self {
            seen: DashMap::new(),
            window_secs,
        }
    }

    pub fn is_duplicate(&self, sensor_id: &str, timestamp: i64) -> bool {
        let now = Utc::now().timestamp();
        
        if let Some(last_seen) = self.seen.get(sensor_id) {
            // Check if within deduplication window
            if now - *last_seen < self.window_secs as i64 {
                // Check if timestamp is same or older
                if timestamp <= *last_seen {
                    return true;
                }
            }
        }
        
        false
    }

    pub fn record(&self, sensor_id: String, timestamp: i64) {
        self.seen.insert(sensor_id, timestamp);
        
        // Cleanup old entries periodically (simplified)
        if self.seen.len() > 10000 {
            let now = Utc::now().timestamp();
            self.seen.retain(|_, last_seen| now - *last_seen < self.window_secs as i64);
        }
    }

    pub fn clear(&self) {
        self.seen.clear();
    }
}

/// Ingestion pipeline with resilience patterns
pub struct IngestionPipeline {
    config: PipelineConfig,
    db: Arc<RadiationDatabase>,
    event_bus: Arc<EventBus>,
    circuit_breaker: CircuitBreaker,
    dlq: DeadLetterQueue,
    deduplicator: Deduplicator,
    backpressure: Arc<Semaphore>,
}


impl IngestionPipeline {
    pub fn new(
        config: PipelineConfig,
        db: Arc<RadiationDatabase>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        let circuit_breaker = CircuitBreaker::new(
            config.circuit_breaker_threshold,
            config.circuit_breaker_reset_secs,
        );
        
        let dlq = DeadLetterQueue::new(config.dlq_max_size);
        let deduplicator = Deduplicator::new(config.dedup_window_secs);
        let backpressure = Arc::new(Semaphore::new(config.channel_buffer_size));

        Self {
            config,
            db,
            event_bus,
            circuit_breaker,
            dlq,
            deduplicator,
            backpressure,
        }
    }


    /// Run the ingestion pipeline with multiple sources
    #[instrument(skip(self, sources))]
    pub async fn run(&self, sources: Vec<Box<dyn DataSource>>) -> anyhow::Result<()> {
        info!("Starting ingestion pipeline with {} sources", sources.len());

        let (tx, mut rx) = mpsc::channel::<RadiationReading>(self.config.channel_buffer_size);
        
        // Spawn source tasks
        let mut source_handles = FuturesUnordered::new();
        
        for mut source in sources {
            let tx = tx.clone();
            let permit = self.backpressure.clone().acquire_owned().await?;
            
            let handle = tokio::spawn(async move {
                let _permit = permit; // Hold permit until task completes
                Self::run_source(&mut *source, tx).await
            });
            
            source_handles.push(handle);
        }

        // Spawn batch writer task
        let db = self.db.clone();
        let event_bus = self.event_bus.clone();
        let circuit_breaker = &self.circuit_breaker;
        let dlq = &self.dlq;
        let batch_size = self.config.batch_size;
        let batch_timeout = Duration::from_millis(self.config.batch_timeout_ms);

        let writer_handle = tokio::spawn(async move {

            let mut batch = Vec::with_capacity(batch_size);
            let mut last_write = Instant::now();

            loop {
                let timeout_result = timeout(batch_timeout, rx.recv()).await;
                
                match timeout_result {
                    Ok(Some(reading)) => {
                        // Check deduplication
                        if self.deduplicator.is_duplicate(
                            &reading.sensor_id.to_string(),
                            reading.timestamp
                        ) {
                            metrics::counter!("cherenkov_ingest_deduplicated_total").increment(1);
                            continue;
                        }
                        
                        self.deduplicator.record(
                            reading.sensor_id.to_string(),
                            reading.timestamp
                        );
                        
                        batch.push(reading);

                        if batch.len() >= batch_size {
                            Self::write_batch(&db, &event_bus, circuit_breaker, dlq, &mut batch).await;
                            last_write = Instant::now();
                        }

                    }
                    Ok(None) => {
                        // Channel closed
                        if !batch.is_empty() {
                            Self::write_batch(&db, &event_bus, circuit_breaker, dlq, &mut batch).await;
                        }
                        break;
                    }
                    Err(_) => {
                        // Timeout - flush batch
                        if !batch.is_empty() {
                            Self::write_batch(&db, &event_bus, circuit_breaker, dlq, &mut batch).await;
                            last_write = Instant::now();
                        }
                    }
                }

                // Periodic flush if batch has been sitting too long
                if !batch.is_empty() && last_write.elapsed() >= batch_timeout {
                    Self::write_batch(&db, &event_bus, circuit_breaker, dlq, &mut batch).await;
                    last_write = Instant::now();
                }

            }
        });

        // Monitor source tasks
        while let Some(result) = source_handles.next().await {
            match result {
                Ok(Ok(())) => debug!("Source completed successfully"),
                Ok(Err(e)) => {
                    warn!("Source error: {}", e);
                    metrics::counter!("cherenkov_ingest_source_errors_total").increment(1);
                }
                Err(e) => {
                    error!("Source task panicked: {}", e);
                    metrics::counter!("cherenkov_ingest_panics_total").increment(1);
                }
            }
        }

        // Wait for writer to finish
        drop(tx); // Close channel
        writer_handle.await?;

        info!("Ingestion pipeline completed");
        Ok(())
    }

    async fn run_source(source: &mut dyn DataSource, tx: mpsc::Sender<RadiationReading>) -> anyhow::Result<()> {
        loop {
            match source.fetch().await {
                Ok(readings) => {
                    for reading in readings {
                        if tx.send(reading).await.is_err() {
                            return Ok(()); // Channel closed
                        }
                    }
                    metrics::counter!("cherenkov_ingest_readings_total", "source" => source.name()).increment(readings.len() as u64);
                }
                Err(e) => {
                    warn!("Source {} fetch failed: {}", source.name(), e);
                    metrics::counter!("cherenkov_ingest_fetch_errors_total", "source" => source.name()).increment(1);
                }
            }
            
            tokio::time::sleep(source.poll_interval()).await;
        }
    }

    async fn write_batch(
        db: &Arc<RadiationDatabase>,
        event_bus: &Arc<EventBus>,
        circuit_breaker: &CircuitBreaker,
        dlq: &DeadLetterQueue,
        batch: &mut Vec<RadiationReading>,
    ) {

        if batch.is_empty() {
            return;
        }

        // Check circuit breaker
        if !circuit_breaker.can_execute().await {
            warn!("Circuit breaker open, storing batch to DLQ");
            for reading in batch.drain(..) {
                dlq.store(reading, "Circuit breaker open".to_string()).await;
            }
            return;
        }

        // Attempt write with retry
        let mut attempts = 0;
        let max_attempts = 3;
        let mut failed_readings = Vec::new();

        for reading in batch.drain(..) {
            let mut success = false;
            
            while attempts < max_attempts && !success {
                match db.write_reading(&reading).await {
                    Ok(()) => {
                        success = true;
                        circuit_breaker.record_success().await;
                        
                        // Publish event to EventBus for downstream consumers
                        let event = CherenkovEvent::NewReading(NormalizedReading {
                            sensor_id: reading.sensor_id,
                            timestamp: chrono::DateTime::from_timestamp(reading.timestamp, 0)
                                .unwrap_or_else(|| chrono::Utc::now()),
                            latitude: reading.latitude,
                            longitude: reading.longitude,
                            dose_rate_microsieverts: reading.dose_rate_microsieverts,
                            uncertainty: reading.uncertainty,
                            quality_flag: match reading.quality_flag {
                                QualityFlag::Valid => cherenkov_core::QualityFlag::Valid,
                                QualityFlag::Suspect => cherenkov_core::QualityFlag::Suspect,
                                QualityFlag::Invalid => cherenkov_core::QualityFlag::Invalid,
                            },
                            source: reading.source.clone(),
                            cell_id: reading.cell_id.clone(),
                        });
                        
                        if let Err(e) = event_bus.publish(event).await {
                            warn!("Failed to publish event to EventBus: {}", e);
                        } else {
                            metrics::counter!("cherenkov_ingest_events_published_total").increment(1);
                        }
                    }
                    Err(e) => {
                        attempts += 1;
                        if attempts >= max_attempts {
                            failed_readings.push((reading, e.to_string()));
                            circuit_breaker.record_failure().await;
                        } else {
                            tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
                        }
                    }
                }
            }
            
            attempts = 0; // Reset for next reading
        }


        // Store failures to DLQ
        for (reading, error) in failed_readings {
            dlq.store(reading, error).await;
        }

        metrics::histogram!("cherenkov_ingest_batch_size").record(batch.len() as f64);
    }

    /// Get pipeline statistics
    pub async fn stats(&self) -> PipelineStats {
        PipelineStats {
            dlq_size: self.dlq.len().await,
            circuit_breaker_open: !self.circuit_breaker.can_execute().await,
            dedup_cache_size: self.deduplicator.seen.len(),
        }
    }

    /// Replay dead letter queue
    pub async fn replay_dlq(&self) -> usize {
        let db = self.db.clone();
        
        self.dlq.replay(|entry| async move {
            match db.write_reading(&entry.reading).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    warn!("DLQ replay failed: {}", e);
                    Err(())
                }
            }
        }).await
    }
}

#[derive(Debug, Clone)]
pub struct PipelineStats {
    pub dlq_size: usize,
    pub circuit_breaker_open: bool,
    pub dedup_cache_size: usize,
}

/// Trait for data sources
#[async_trait::async_trait]
pub trait DataSource: Send {
    async fn fetch(&mut self) -> anyhow::Result<Vec<RadiationReading>>;
    fn name(&self) -> String;
    fn poll_interval(&self) -> Duration;
}
