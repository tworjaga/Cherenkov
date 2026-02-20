//! Main notification service with retry logic and channel orchestration

use crate::{
    email::{EmailConfig, EmailNotifier},
    rate_limiter::{RateLimitConfig, RateLimiterRegistry},
    sms::{SmsConfig, SmsNotifier},
    telegram::{TelegramConfig, TelegramNotifier},
    types::{
        AlertEvent, Notification, NotificationBuilder, NotificationChannel,
        NotificationPriority, NotificationResult, NotificationStatus, Recipient,
    },
    webhook::{WebhookConfig, WebhookNotifier},
};
use anyhow::{Context, Result};
use backoff::{future::retry, ExponentialBackoff};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Notification service configuration
#[derive(Debug, Clone)]
pub struct NotificationServiceConfig {
    pub email: EmailConfig,
    pub sms: SmsConfig,
    pub webhook: WebhookConfig,
    pub telegram: TelegramConfig,
    pub rate_limits: RateLimitConfig,
    pub max_retries: u32,
    pub retry_base_delay_ms: u64,
}

impl NotificationServiceConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            email: EmailConfig::from_env()?,
            sms: SmsConfig::from_env()?,
            webhook: WebhookConfig::from_env()?,
            telegram: TelegramConfig::from_env()?,
            rate_limits: RateLimitConfig::default(),
            max_retries: std::env::var("NOTIFICATION_MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid NOTIFICATION_MAX_RETRIES")?,
            retry_base_delay_ms: std::env::var("NOTIFICATION_RETRY_DELAY_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .context("Invalid NOTIFICATION_RETRY_DELAY_MS")?,
        })
    }
}

/// Notification service with multi-channel support
pub struct NotificationService {
    email: EmailNotifier,
    sms: SmsNotifier,
    webhook: WebhookNotifier,
    telegram: TelegramNotifier,
    rate_limiters: RateLimiterRegistry,
    max_retries: u32,
    retry_base_delay_ms: u64,
    history: Arc<RwLock<HashMap<uuid::Uuid, Vec<NotificationResult>>>>,
}

impl NotificationService {
    /// Create a new notification service
    pub fn new(config: NotificationServiceConfig) -> Self {
        Self {
            email: EmailNotifier::new(config.email),
            sms: SmsNotifier::new(config.sms),
            webhook: WebhookNotifier::new(config.webhook),
            telegram: TelegramNotifier::new(config.telegram),
            rate_limiters: RateLimiterRegistry::new(),
            max_retries: config.max_retries,
            retry_base_delay_ms: config.retry_base_delay_ms,
            history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Send notification to a single recipient through specified channels
    pub async fn send(
        &self,
        notification: &Notification,
        recipient: &Recipient,
    ) -> Vec<NotificationResult> {
        let mut results = Vec::new();

        for channel in &notification.channels {
            // Check rate limit
            let rate_limiter = self.rate_limiters.get(*channel);
            rate_limiter.acquire().await;

            // Send with retry logic
            let result = self.send_with_retry(notification, recipient, *channel).await;
            results.push(result);
        }

        // Store history
        self.store_history(notification.id, results.clone()).await;

        results
    }

    /// Send notification to multiple recipients
    pub async fn broadcast(
        &self,
        notification: &Notification,
        recipients: &[Recipient],
    ) -> Vec<NotificationResult> {
        let mut all_results = Vec::new();

        for recipient in recipients {
            let results = self.send(notification, recipient).await;
            all_results.extend(results);
        }

        all_results
    }

    /// Send alert event notification
    pub async fn send_alert(
        &self,
        alert: &AlertEvent,
        recipients: &[Recipient],
    ) -> Vec<NotificationResult> {
        let notification = NotificationBuilder::from_alert(alert).build();
        self.broadcast(&notification, recipients).await
    }

    /// Send with retry logic using exponential backoff
    async fn send_with_retry(
        &self,
        notification: &Notification,
        recipient: &Recipient,
        channel: NotificationChannel,
    ) -> NotificationResult {
        let operation = || async {
            let result = self.send_single(notification, recipient, channel).await;
            
            if result.status == NotificationStatus::Delivered {
                Ok(result)
            } else {
                Err(backoff::Error::transient(anyhow::anyhow!(
                    "Failed to send: {:?}",
                    result.error_message
                )))
            }
        };

        let backoff = ExponentialBackoff {
            max_elapsed_time: Some(std::time::Duration::from_secs(60)),
            ..Default::default()
        };

        match retry(backoff, operation).await {
            Ok(result) => result,
            Err(e) => {
                warn!(
                    notification_id = %notification.id,
                    recipient_id = %recipient.id,
                    channel = %channel.as_str(),
                    error = %e,
                    "All retries exhausted"
                );

                NotificationResult {
                    notification_id: notification.id,
                    channel,
                    recipient_id: recipient.id,
                    status: NotificationStatus::Failed,
                    sent_at: Some(chrono::Utc::now()),
                    delivered_at: None,
                    error_message: Some(format!("Max retries exceeded: {}", e)),
                    retry_count: self.max_retries,
                }
            }
        }
    }

    /// Send single notification without retry
    async fn send_single(
        &self,
        notification: &Notification,
        recipient: &Recipient,
        channel: NotificationChannel,
    ) -> NotificationResult {
        match channel {
            NotificationChannel::Email => self.email.send(notification, recipient).await,
            NotificationChannel::Sms => self.sms.send(notification, recipient).await,
            NotificationChannel::Webhook => self.webhook.send(notification, recipient).await,
            NotificationChannel::Telegram => self.telegram.send(notification, recipient).await,
        }
    }

    /// Store notification history
    async fn store_history(&self, notification_id: uuid::Uuid, results: Vec<NotificationResult>) {
        let mut history = self.history.write().await;
        history.insert(notification_id, results);
    }

    /// Get notification history
    pub async fn get_history(&self, notification_id: uuid::Uuid) -> Option<Vec<NotificationResult>> {
        let history = self.history.read().await;
        history.get(&notification_id).cloned()
    }

    /// Get delivery statistics
    pub async fn get_stats(&self) -> NotificationStats {
        let history = self.history.read().await;
        
        let total = history.len();
        let mut delivered = 0;
        let mut failed = 0;
        let mut pending = 0;

        for results in history.values() {
            for result in results {
                match result.status {
                    NotificationStatus::Delivered => delivered += 1,
                    NotificationStatus::Failed => failed += 1,
                    NotificationStatus::Pending => pending += 1,
                }
            }
        }

        NotificationStats {
            total_notifications: total,
            delivered,
            failed,
            pending,
        }
    }

    /// Clear old history (call periodically)
    pub async fn clear_history(&self, older_than: chrono::DateTime<chrono::Utc>) {
        let mut history = self.history.write().await;
        history.retain(|_, results| {
            results.iter().any(|r| {
                r.sent_at.map(|t| t > older_than).unwrap_or(true)
            })
        });
    }
}

/// Notification statistics
#[derive(Debug, Clone)]
pub struct NotificationStats {
    pub total_notifications: usize,
    pub delivered: usize,
    pub failed: usize,
    pub pending: usize,
}

impl NotificationStats {
    pub fn success_rate(&self) -> f64 {
        let total = self.delivered + self.failed;
        if total == 0 {
            0.0
        } else {
            self.delivered as f64 / total as f64 * 100.0
        }
    }
}

/// Builder for notification service
pub struct NotificationServiceBuilder {
    email_config: Option<EmailConfig>,
    sms_config: Option<SmsConfig>,
    webhook_config: Option<WebhookConfig>,
    telegram_config: Option<TelegramConfig>,
    max_retries: u32,
}

impl NotificationServiceBuilder {
    pub fn new() -> Self {
        Self {
            email_config: None,
            sms_config: None,
            webhook_config: None,
            telegram_config: None,
            max_retries: 3,
        }
    }

    pub fn with_email(mut self, config: EmailConfig) -> Self {
        self.email_config = Some(config);
        self
    }

    pub fn with_sms(mut self, config: SmsConfig) -> Self {
        self.sms_config = Some(config);
        self
    }

    pub fn with_webhook(mut self, config: WebhookConfig) -> Self {
        self.webhook_config = Some(config);
        self
    }

    pub fn with_telegram(mut self, config: TelegramConfig) -> Self {
        self.telegram_config = Some(config);
        self
    }

    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn build(self) -> Result<NotificationService> {
        let config = NotificationServiceConfig {
            email: self.email_config.context("Email config required")?,
            sms: self.sms_config.context("SMS config required")?,
            webhook: self.webhook_config.context("Webhook config required")?,
            telegram: self.telegram_config.context("Telegram config required")?,
            rate_limits: RateLimitConfig::default(),
            max_retries: self.max_retries,
            retry_base_delay_ms: 1000,
        };

        Ok(NotificationService::new(config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_stats() {
        let stats = NotificationStats {
            total_notifications: 10,
            delivered: 8,
            failed: 2,
            pending: 0,
        };

        assert_eq!(stats.success_rate(), 80.0);
    }

    #[test]
    fn test_notification_stats_empty() {
        let stats = NotificationStats {
            total_notifications: 0,
            delivered: 0,
            failed: 0,
            pending: 0,
        };

        assert_eq!(stats.success_rate(), 0.0);
    }
}
