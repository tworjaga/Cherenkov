use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn, error};

use cherenkov_ingest::{
    sources::{
        EpaRadnetSource, NasaFirmsSource, NoaaGfsSource, OpenAqSource, OpenMeteoSource,
        SafecastSource, UradmonitorSource
    },
    pipeline::{IngestionPipeline, PipelineConfig, DataSource},
    sources_extra,
};
use cherenkov_db::{RadiationDatabase, DatabaseConfig, scylla::ScyllaConfig};
use cherenkov_observability::init_observability;
use cherenkov_core::EventBus;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_observability();
    
    info!("Starting Cherenkov Ingest Daemon v{}", env!("CARGO_PKG_VERSION"));
    
    // Initialize database
    let scylla_config = ScyllaConfig::default();
    let db = Arc::new(
        RadiationDatabase::new(
            scylla_config,
            "./data/cherenkov_warm.db",
            "redis://127.0.0.1:6379",
            DatabaseConfig::default(),
        ).await?
    );
    
    // Run migrations
    db.run_migrations().await?;
    
    // Initialize EventBus for inter-crate communication
    let event_bus = Arc::new(EventBus::new(10000));
    info!("EventBus initialized for publishing NewReading events");
    
    // Create ingestion pipeline
    let config = PipelineConfig {
        max_concurrent_sources: 10,
        channel_buffer_size: 10000,
        batch_size: 100,
        batch_timeout_ms: 1000,
        circuit_breaker_threshold: 5,
        circuit_breaker_reset_secs: 30,
        dlq_max_size: 10000,
        dedup_window_secs: 60,
    };
    
    let pipeline = Arc::new(IngestionPipeline::new(config, db.clone(), event_bus.clone()));

    
    // Create data sources
    let sources = create_sources();
    
    // Start pipeline
    let pipeline_clone = pipeline.clone();
    let pipeline_handle = tokio::spawn(async move {
        if let Err(e) = pipeline_clone.run(sources).await {
            error!("Pipeline error: {}", e);
        }
    });
    
    // Start health check server
    let health_handle = tokio::spawn(health_check_server(db.clone()));
    
    // Start DLQ replayer
    let dlq_handle = tokio::spawn(dlq_replayer(pipeline.clone()));
    
    // Start EventBus metrics reporter
    let metrics_handle = tokio::spawn(eventbus_metrics_reporter(event_bus.clone()));
    
    // Wait for all tasks
    tokio::select! {
        _ = pipeline_handle => warn!("Pipeline exited"),
        _ = health_handle => warn!("Health server exited"),
        _ = dlq_handle => warn!("DLQ replayer exited"),
        _ = metrics_handle => warn!("EventBus metrics exited"),
        _ = tokio::signal::ctrl_c() => info!("Shutdown signal received"),
    }
    
    info!("Cherenkov Ingest Daemon shutting down");
    Ok(())

}

fn create_sources() -> Vec<Box<dyn DataSource + Send>> {
    vec![
        Box::new(SafecastSource::new()),
        Box::new(UradmonitorSource::new()),
        Box::new(EpaRadnetSource::new()),
        Box::new(OpenAqSource::new()),
        Box::new(OpenMeteoSource::new()),
        Box::new(NasaFirmsSource::new(
            std::env::var("NASA_FIRMS_API_KEY").unwrap_or_default()
        )),
        Box::new(NoaaGfsSource::new()),
        Box::new(sources_extra::IaeaPrisSource::new()),
    ]
}


async fn health_check_server(db: Arc<RadiationDatabase>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        let health = db.health_check().await;
        if !health.is_healthy() {
            warn!("Database health check failed: hot={}, warm={}, cache={}", 
                health.hot, health.warm, health.cache);
        }
    }
}

async fn dlq_replayer(pipeline: Arc<IngestionPipeline>) {
    let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
    
    loop {
        interval.tick().await;
        
        let replayed = pipeline.replay_dlq().await;
        if replayed > 0 {
            info!("Replayed {} entries from DLQ", replayed);
        }
    }
}

async fn eventbus_metrics_reporter(event_bus: Arc<EventBus>) {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        let subscriber_count = event_bus.subscriber_count();
        info!("EventBus metrics: {} active subscribers", subscriber_count);
    }
}
