use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main configuration for Cherenkov services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// Data source configuration
    pub sources: SourcesConfig,
    
    /// API server configuration
    pub api: ApiConfig,
    
    /// Stream processing configuration
    pub stream: StreamConfig,
    
    /// Ingestion configuration
    pub ingest: IngestConfig,
    
    /// Observability configuration
    pub observability: ObservabilityConfig,
}

/// Database connection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// ScyllaDB connection URI
    pub scylla_uri: String,
    
    /// SQLite database path
    pub sqlite_path: String,
    
    /// Redis connection URI
    pub redis_uri: String,
    
    /// Object storage bucket for cold data
    pub cold_storage_bucket: Option<String>,
    
    /// Connection pool sizes
    pub pool_size: usize,
}

/// Data source settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcesConfig {
    /// Safecast source
    pub safecast: SourceSettings,
    
    /// uRADMonitor source
    pub uradmonitor: SourceSettings,
    
    /// EPA RadNet source
    pub epa_radnet: Option<SourceSettings>,
    
    /// OpenAQ source
    pub openaq: Option<SourceSettings>,
    
    /// Open-Meteo source
    pub openmeteo: Option<SourceSettings>,
}

/// Individual source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSettings {
    /// Whether source is enabled
    pub enabled: bool,
    
    /// Poll interval in seconds
    pub interval_sec: u64,
    
    /// API key if required
    pub api_key: Option<String>,
    
    /// Additional parameters
    #[serde(default)]
    pub params: HashMap<String, String>,
}

/// API server settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address
    pub bind_addr: String,
    
    /// Port to listen on
    pub port: u16,
    
    /// JWT secret for authentication
    pub jwt_secret: String,
    
    /// Rate limit requests per minute
    pub rate_limit_per_minute: u32,
    
    /// Enable GraphQL playground
    pub enable_playground: bool,
}

/// Stream processing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Anomaly detection sensitivity (z-score threshold)
    pub anomaly_threshold: f64,
    
    /// Window size in seconds for anomaly detection
    pub window_size_sec: u64,
    
    /// Enable cross-sensor correlation
    pub enable_correlation: bool,
    
    /// Correlation radius in kilometers
    pub correlation_radius_km: f64,
}

/// Ingestion pipeline settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestConfig {
    /// Batch size for database writes
    pub batch_size: usize,
    
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u64,
    
    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
    
    /// Dead letter queue retry interval
    pub dlq_retry_interval_sec: u64,
    
    /// Maximum concurrent fetches
    pub max_concurrent_fetches: usize,
}

/// Observability settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Log level
    pub log_level: String,
    
    /// Enable JSON logging
    pub json_logs: bool,
    
    /// Metrics export port
    pub metrics_port: u16,
    
    /// Distributed tracing endpoint
    pub tracing_endpoint: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                scylla_uri: "127.0.0.1:9042".to_string(),
                sqlite_path: "./data/cherenkov.db".to_string(),
                redis_uri: "redis://127.0.0.1:6379".to_string(),
                cold_storage_bucket: None,
                pool_size: 10,
            },
            sources: SourcesConfig {
                safecast: SourceSettings {
                    enabled: true,
                    interval_sec: 60,
                    api_key: None,
                    params: HashMap::new(),
                },
                uradmonitor: SourceSettings {
                    enabled: true,
                    interval_sec: 30,
                    api_key: None,
                    params: HashMap::new(),
                },
                epa_radnet: None,
                openaq: None,
                openmeteo: None,
            },
            api: ApiConfig {
                bind_addr: "0.0.0.0".to_string(),
                port: 8080,
                jwt_secret: "cherenkov-dev-secret".to_string(),
                rate_limit_per_minute: 60,
                enable_playground: true,
            },
            stream: StreamConfig {
                anomaly_threshold: 3.0,
                window_size_sec: 300,
                enable_correlation: true,
                correlation_radius_km: 50.0,
            },
            ingest: IngestConfig {
                batch_size: 100,
                batch_timeout_ms: 5000,
                circuit_breaker_threshold: 5,
                dlq_retry_interval_sec: 60,
                max_concurrent_fetches: 10,
            },
            observability: ObservabilityConfig {
                log_level: "info".to_string(),
                json_logs: true,
                metrics_port: 9090,
                tracing_endpoint: None,
            },
        }
    }
}

impl Config {
    /// Load configuration from YAML file
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        // Override with environment variables
        if let Ok(uri) = std::env::var("SCYLLA_URI") {
            config.database.scylla_uri = uri;
        }
        if let Ok(path) = std::env::var("SQLITE_PATH") {
            config.database.sqlite_path = path;
        }
        if let Ok(secret) = std::env::var("JWT_SECRET") {
            config.api.jwt_secret = secret;
        }
        if let Ok(port) = std::env::var("API_PORT") {
            if let Ok(p) = port.parse() {
                config.api.port = p;
            }
        }
        
        config
    }
    
    /// Merge file config with environment overrides
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let mut config = Self::from_file(path)?;
        
        // Environment overrides
        if let Ok(uri) = std::env::var("SCYLLA_URI") {
            config.database.scylla_uri = uri;
        }
        if let Ok(secret) = std::env::var("JWT_SECRET") {
            config.api.jwt_secret = secret;
        }
        
        Ok(config)
    }
}
