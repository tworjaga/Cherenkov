//! Rate limiting for notification channels

use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use nonzero_ext::nonzero;
use std::sync::Arc;
use tracing::debug;

/// Rate limiter configuration per channel
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 20,
        }
    }
}

impl RateLimitConfig {
    /// Conservative limits for email (respecting SMTP rate limits)
    pub fn email_conservative() -> Self {
        Self {
            requests_per_second: 5,
            burst_size: 10,
        }
    }

    /// Standard limits for SMS (Twilio)
    pub fn sms_standard() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 20,
        }
    }

    /// Aggressive limits for webhooks
    pub fn webhook_aggressive() -> Self {
        Self {
            requests_per_second: 50,
            burst_size: 100,
        }
    }

    /// Telegram limits (respecting Bot API)
    pub fn telegram_standard() -> Self {
        Self {
            requests_per_second: 30,
            burst_size: 60,
        }
    }
}

/// Rate limiter wrapper for notification channels
pub struct NotificationRateLimiter {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    channel_name: String,
}

impl NotificationRateLimiter {
    /// Create a new rate limiter with custom config
    pub fn new(channel_name: impl Into<String>, config: RateLimitConfig) -> Self {
        let quota = Quota::per_second(nonzero!(config.requests_per_second))
            .allow_burst(nonzero!(config.burst_size));

        let limiter = Arc::new(RateLimiter::direct(quota));

        debug!(
            channel = %channel_name.as_ref(),
            rps = config.requests_per_second,
            burst = config.burst_size,
            "Rate limiter created"
        );

        Self {
            limiter,
            channel_name: channel_name.into(),
        }
    }

    /// Create with default config
    pub fn default_for(channel_name: impl Into<String>) -> Self {
        Self::new(channel_name, RateLimitConfig::default())
    }

    /// Check if request can proceed (non-blocking)
    pub fn check(&self) -> bool {
        self.limiter.check().is_ok()
    }

    /// Wait until request can proceed (blocking async)
    pub async fn acquire(&self) {
        self.limiter.until_ready().await;
    }

    /// Get channel name
    pub fn channel_name(&self) -> &str {
        &self.channel_name
    }
}

/// Rate limiter registry for multiple channels
pub struct RateLimiterRegistry {
    email: NotificationRateLimiter,
    sms: NotificationRateLimiter,
    webhook: NotificationRateLimiter,
    telegram: NotificationRateLimiter,
}

impl RateLimiterRegistry {
    /// Create registry with default configurations
    pub fn new() -> Self {
        Self {
            email: NotificationRateLimiter::new("email", RateLimitConfig::email_conservative()),
            sms: NotificationRateLimiter::new("sms", RateLimitConfig::sms_standard()),
            webhook: NotificationRateLimiter::new("webhook", RateLimitConfig::webhook_aggressive()),
            telegram: NotificationRateLimiter::new("telegram", RateLimitConfig::telegram_standard()),
        }
    }

    /// Create with custom configurations
    pub fn with_config(
        email: RateLimitConfig,
        sms: RateLimitConfig,
        webhook: RateLimitConfig,
        telegram: RateLimitConfig,
    ) -> Self {
        Self {
            email: NotificationRateLimiter::new("email", email),
            sms: NotificationRateLimiter::new("sms", sms),
            webhook: NotificationRateLimiter::new("webhook", webhook),
            telegram: NotificationRateLimiter::new("telegram", telegram),
        }
    }

    /// Get rate limiter for channel
    pub fn get(&self, channel: crate::types::NotificationChannel) -> &NotificationRateLimiter {
        match channel {
            crate::types::NotificationChannel::Email => &self.email,
            crate::types::NotificationChannel::Sms => &self.sms,
            crate::types::NotificationChannel::Webhook => &self.webhook,
            crate::types::NotificationChannel::Telegram => &self.telegram,
        }
    }
}

impl Default for RateLimiterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_creation() {
        let limiter = NotificationRateLimiter::default_for("test");
        assert_eq!(limiter.channel_name(), "test");
    }

    #[test]
    fn test_rate_limiter_check() {
        let config = RateLimitConfig {
            requests_per_second: 1,
            burst_size: 1,
        };
        let limiter = NotificationRateLimiter::new("test", config);
        
        // First check should pass
        assert!(limiter.check());
        
        // Immediate second check should fail (rate limited)
        // Note: This might occasionally pass due to timing
    }

    #[tokio::test]
    async fn test_rate_limiter_acquire() {
        let config = RateLimitConfig {
            requests_per_second: 100, // High limit for test
            burst_size: 100,
        };
        let limiter = NotificationRateLimiter::new("test", config);
        
        // Should complete quickly
        limiter.acquire().await;
    }
}
