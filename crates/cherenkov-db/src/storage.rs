use chrono::{DateTime, Utc};
use std::path::Path;
use tracing::{info, error, instrument};
use uuid::Uuid;

use crate::RadiationReading;

/// Cold storage for historical archive using Parquet format
/// This is a stub implementation - full S3 integration requires
/// the `cold-storage` feature flag and AWS SDK
#[derive(Debug)]
pub struct ColdStorage {
    local_path: String,
    enabled: bool,
}

impl ColdStorage {
    pub fn new(local_path: &str) -> anyhow::Result<Self> {
        // Ensure directory exists
        std::fs::create_dir_all(local_path)?;
        
        info!("Cold storage initialized at {}", local_path);

        Ok(Self {
            local_path: local_path.to_string(),
            enabled: true,
        })
    }

    pub fn disabled() -> Self {
        Self {
            local_path: String::new(),
            enabled: false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[instrument(skip(self, readings))]
    pub async fn archive_readings(
        &self,
        readings: &[RadiationReading],
    ) -> anyhow::Result<String> {
        if !self.enabled {
            return Err(anyhow::anyhow!("Cold storage is disabled"));
        }

        if readings.is_empty() {
            return Ok(String::new());
        }

        // Generate filename based on date range
        let min_timestamp = readings.iter()
            .map(|r| r.timestamp)
            .min()
            .unwrap_or(0);
        
        let max_timestamp = readings.iter()
            .map(|r| r.timestamp)
            .max()
            .unwrap_or(0);

        let filename = format!(
            "readings_{}_{}_{}.parquet",
            min_timestamp,
            max_timestamp,
            Uuid::new_v4()
        );

        let filepath = Path::new(&self.local_path).join(&filename);

        // For now, write as JSON (Parquet requires arrow/parquet crate)
        // In production, this would use Parquet format
        let json_data = serde_json::to_string_pretty(readings)?;
        tokio::fs::write(&filepath, json_data).await?;

        info!("Archived {} readings to {}", readings.len(), filepath.display());

        Ok(filepath.to_string_lossy().to_string())
    }

    #[instrument(skip(self))]
    pub async fn query_range(
        &self,
        _sensor_ids: &[String],
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
    ) -> anyhow::Result<Vec<RadiationReading>> {
        if !self.enabled {
            return Ok(vec![]);
        }

        // TODO: Implement query against archived Parquet files
        // This would require:
        // 1. Listing relevant Parquet files based on date range
        // 2. Reading and filtering data
        // 3. Returning matching readings
        
        info!("Cold storage query not yet implemented");
        Ok(vec![])
    }

    #[instrument(skip(self))]
    pub async fn list_archives(
        &self,
    ) -> anyhow::Result<Vec<ArchiveInfo>> {
        if !self.enabled {
            return Ok(vec![]);
        }

        let mut archives = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.local_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            let path = entry.path();
            
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                // Parse filename for timestamps
                let parts: Vec<&str> = filename.split('_').collect();
                if parts.len() >= 3 {
                    if let (Ok(start), Ok(end)) = (
                        parts[1].parse::<i64>(),
                        parts[2].parse::<i64>()
                    ) {
                        archives.push(ArchiveInfo {
                            filename: filename.to_string(),
                            start_timestamp: start,
                            end_timestamp: end,
                            size_bytes: metadata.len(),
                            created: metadata.created()
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs() as i64)
                                .unwrap_or(0),
                        });
                    }
                }
            }
        }

        // Sort by start timestamp
        archives.sort_by(|a, b| a.start_timestamp.cmp(&b.start_timestamp));

        Ok(archives)
    }

    #[instrument(skip(self))]
    pub async fn delete_old_archives(
        &self,
        before: DateTime<Utc>,
    ) -> anyhow::Result<u64> {
        if !self.enabled {
            return Ok(0);
        }

        let before_timestamp = before.timestamp();
        let archives = self.list_archives().await?;
        let mut deleted = 0u64;

        for archive in archives {
            if archive.end_timestamp < before_timestamp {
                let path = Path::new(&self.local_path)
                    .join(format!("{}.parquet", archive.filename));
                
                if let Err(e) = tokio::fs::remove_file(&path).await {
                    error!("Failed to delete archive {}: {}", path.display(), e);
                } else {
                    deleted += 1;
                    info!("Deleted old archive: {}", path.display());
                }
            }
        }

        Ok(deleted)
    }

    pub async fn health_check(&self) -> bool {
        if !self.enabled {
            return true; // Disabled is considered healthy
        }

        match tokio::fs::metadata(&self.local_path).await {
            Ok(_) => true,
            Err(e) => {
                error!("Cold storage health check failed: {}", e);
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchiveInfo {
    pub filename: String,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub size_bytes: u64,
    pub created: i64,
}

/// Configuration for cold storage
#[derive(Debug, Clone)]
pub struct ColdStorageConfig {
    pub local_path: String,
    pub s3_bucket: Option<String>,
    pub s3_region: Option<String>,
    pub s3_endpoint: Option<String>,
    pub compression: CompressionType,
    pub retention_days: u32,
}

impl Default for ColdStorageConfig {
    fn default() -> Self {
        Self {
            local_path: "./data/cold".to_string(),
            s3_bucket: None,
            s3_region: None,
            s3_endpoint: None,
            compression: CompressionType::Zstd,
            retention_days: 365,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    None,
    Snappy,
    Gzip,
    Zstd,
    Lz4,
}

impl CompressionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompressionType::None => "NONE",
            CompressionType::Snappy => "SNAPPY",
            CompressionType::Gzip => "GZIP",
            CompressionType::Zstd => "ZSTD",
            CompressionType::Lz4 => "LZ4",
        }
    }
}

/// Future S3-backed cold storage implementation
#[cfg(feature = "cold-storage")]
pub mod s3 {
    use super::*;
    use aws_sdk_s3::Client as S3Client;
    use aws_sdk_s3::primitives::ByteStream;

    pub struct S3ColdStorage {
        client: S3Client,
        bucket: String,
        prefix: String,
    }

    impl S3ColdStorage {
        pub async fn new(config: &ColdStorageConfig) -> anyhow::Result<Self> {
            let aws_config = aws_config::load_from_env().await;
            let client = S3Client::new(&aws_config);
            
            let bucket = config.s3_bucket.clone()
                .ok_or_else(|| anyhow::anyhow!("S3 bucket not configured"))?;

            Ok(Self {
                client,
                bucket,
                prefix: "radiation_readings/".to_string(),
            })
        }

        pub async fn upload_archive(
            &self,
            data: Vec<u8>,
            key: &str,
        ) -> anyhow::Result<()> {
            let stream = ByteStream::from(data);
            
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(format!("{}{}", self.prefix, key))
                .body(stream)
                .send()
                .await?;

            Ok(())
        }

        pub async fn download_archive(
            &self,
            key: &str,
        ) -> anyhow::Result<Vec<u8>> {
            let response = self.client
                .get_object()
                .bucket(&self.bucket)
                .key(format!("{}{}", self.prefix, key))
                .send()
                .await?;

            let data = response.body.collect().await?.into_bytes();
            Ok(data.to_vec())
        }
    }
}
