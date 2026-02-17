use redis::{Client, aio::MultiplexedConnection};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, instrument};
use uuid::Uuid;

use crate::RadiationReading;

#[derive(Debug)]
pub struct RedisCache {
    #[allow(dead_code)]
    client: Client,
    connection: Arc<RwLock<MultiplexedConnection>>,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = Client::open(redis_url)?;
        let connection = client.get_multiplexed_async_connection().await?;
        
        info!("Redis cache initialized");

        Ok(Self {
            client,
            connection: Arc::new(RwLock::new(connection)),
        })
    }

    #[instrument(skip(self, value))]
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(value)?;
        let mut conn = self.connection.write().await;
        
        redis::cmd("SETEX")
            .arg(key)
            .arg(ttl_seconds)
            .arg(serialized)
            .query_async::<_, ()>(&mut *conn)
            .await?;
        
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> anyhow::Result<Option<T>> {
        let mut conn = self.connection.write().await;
        
        let value: Option<String> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut *conn)
            .await?;
        
        match value {
            Some(v) => {
                let deserialized = serde_json::from_str(&v)?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.connection.write().await;
        
        redis::cmd("DEL")
            .arg(key)
            .query_async::<_, ()>(&mut *conn)
            .await?;
        
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn invalidate_sensor(&self, sensor_id: &Uuid) -> anyhow::Result<()> {
        let pattern = format!("sensor:{}:*", sensor_id);
        let mut conn = self.connection.write().await;
        
        // Find and delete all keys matching the sensor pattern
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(&pattern)
            .query_async(&mut *conn)
            .await?;
        
        if !keys.is_empty() {
            redis::cmd("DEL")
                .arg(&keys)
                .query_async::<_, ()>(&mut *conn)
                .await?;
        }
        
        Ok(())
    }

    #[instrument(skip(self, reading))]
    pub async fn set_sensor_latest(
        &self,
        sensor_id: &Uuid,
        reading: &RadiationReading,
        ttl_seconds: u64,
    ) -> anyhow::Result<()> {
        let key = format!("sensor:{}:latest", sensor_id);
        self.set(&key, reading, ttl_seconds).await
    }

    #[instrument(skip(self))]
    pub async fn get_sensor_latest(
        &self,
        sensor_id: &Uuid,
    ) -> anyhow::Result<Option<RadiationReading>> {
        let key = format!("sensor:{}:latest", sensor_id);
        self.get(&key).await
    }

    #[instrument(skip(self, points))]
    pub async fn set_query_result<T: Serialize>(
        &self,
        cache_key: &str,
        points: &T,
        ttl_seconds: u64,
    ) -> anyhow::Result<()> {
        let key = format!("query:{}", cache_key);
        self.set(&key, points, ttl_seconds).await
    }

    #[instrument(skip(self))]
    pub async fn get_query_result<T: DeserializeOwned>(
        &self,
        cache_key: &str,
    ) -> anyhow::Result<Option<T>> {
        let key = format!("query:{}", cache_key);
        self.get(&key).await
    }

    /// Cache sensor metadata
    #[instrument(skip(self, metadata))]
    pub async fn set_sensor_metadata<T: Serialize>(
        &self,
        sensor_id: &Uuid,
        metadata: &T,
    ) -> anyhow::Result<()> {
        let key = format!("sensor:{}:metadata", sensor_id);
        self.set(&key, metadata, 3600).await // 1 hour TTL
    }

    #[instrument(skip(self))]
    pub async fn get_sensor_metadata<T: DeserializeOwned>(
        &self,
        sensor_id: &Uuid,
    ) -> anyhow::Result<Option<T>> {
        let key = format!("sensor:{}:metadata", sensor_id);
        self.get(&key).await
    }

    /// Cache global statistics
    #[instrument(skip(self, stats))]
    pub async fn set_global_stats<T: Serialize>(
        &self,
        stats: &T,
    ) -> anyhow::Result<()> {
        self.set("global:stats", stats, 60).await // 1 minute TTL
    }

    #[instrument(skip(self))]
    pub async fn get_global_stats<T: DeserializeOwned>(
        &self,
    ) -> anyhow::Result<Option<T>> {
        self.get("global:stats").await
    }

    /// Increment counter for rate limiting
    #[instrument(skip(self))]
    pub async fn increment_rate_limit(
        &self,
        key: &str,
        window_seconds: u64,
    ) -> anyhow::Result<u64> {
        let mut conn = self.connection.write().await;
        
        let count: u64 = redis::cmd("INCR")
            .arg(key)
            .query_async(&mut *conn)
            .await?;
        
        // Set expiry on first increment
        if count == 1 {
            redis::cmd("EXPIRE")
                .arg(key)
                .arg(window_seconds)
                .query_async::<_, ()>(&mut *conn)
                .await?;
        }
        
        Ok(count)
    }

    /// Check if rate limit is exceeded
    #[instrument(skip(self))]
    pub async fn check_rate_limit(
        &self,
        key: &str,
        max_requests: u64,
    ) -> anyhow::Result<bool> {
        let count = self.get::<u64>(key).await?.unwrap_or(0);
        Ok(count >= max_requests)
    }

    /// Publish message to channel
    #[instrument(skip(self, message))]
    pub async fn publish<T: Serialize>(
        &self,
        channel: &str,
        message: &T,
    ) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(message)?;
        let mut conn = self.connection.write().await;
        
        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(serialized)
            .query_async::<_, ()>(&mut *conn)
            .await?;
        
        Ok(())
    }

    /// Add to sorted set with score (for time-series data)
    #[instrument(skip(self))]
    pub async fn zadd(
        &self,
        key: &str,
        score: f64,
        member: &str,
    ) -> anyhow::Result<()> {
        let mut conn = self.connection.write().await;
        
        redis::cmd("ZADD")
            .arg(key)
            .arg(score)
            .arg(member)
            .query_async::<_, ()>(&mut *conn)
            .await?;
        
        Ok(())
    }

    /// Get range from sorted set
    #[instrument(skip(self))]
    pub async fn zrange(
        &self,
        key: &str,
        start: i64,
        stop: i64,
    ) -> anyhow::Result<Vec<String>> {
        let mut conn = self.connection.write().await;
        
        let members: Vec<String> = redis::cmd("ZRANGE")
            .arg(key)
            .arg(start)
            .arg(stop)
            .query_async(&mut *conn)
            .await?;
        
        Ok(members)
    }

    /// Health check
    pub async fn health_check(&self) -> bool {
        let mut conn = match self.connection.write().await {
            conn => conn,
        };
        
        match redis::cmd("PING")
            .query_async::<_, String>(&mut *conn)
            .await
        {
            Ok(response) => response == "PONG",
            Err(e) => {
                error!("Redis health check failed: {}", e);
                false
            }
        }
    }

    /// Get connection pool stats
    pub async fn pool_stats(&self) -> anyhow::Result<CacheStats> {
        let mut conn = self.connection.write().await;
        
        let info: String = redis::cmd("INFO")
            .arg("memory")
            .query_async(&mut *conn)
            .await?;
        
        // Parse memory info
        let used_memory = info
            .lines()
            .find(|line| line.starts_with("used_memory:"))
            .and_then(|line| line.split(':').nth(1))
            .and_then(|v| v.trim().parse::<u64>().ok())
            .unwrap_or(0);

        Ok(CacheStats {
            used_memory_bytes: used_memory,
            connected: self.health_check().await,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub used_memory_bytes: u64,
    pub connected: bool,
}
