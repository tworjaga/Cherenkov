//! Webhook notification service for HTTP callbacks

use crate::types::{Notification, NotificationResult, NotificationStatus, Recipient};
use anyhow::{Context, Result};
use reqwest::{Client, Method};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

/// Webhook notifier configuration
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub default_headers: Vec<(String, String)>,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            default_headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("User-Agent".to_string(), "Cherenkov-Notifier/1.0".to_string()),
            ],
        }
    }
}

impl WebhookConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            timeout_seconds: std::env::var("WEBHOOK_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .context("Invalid WEBHOOK_TIMEOUT_SECONDS")?,
            max_retries: std::env::var("WEBHOOK_MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid WEBHOOK_MAX_RETRIES")?,
            default_headers: Self::default().default_headers,
        })
    }
}

/// Webhook notifier implementation
pub struct WebhookNotifier {
    config: WebhookConfig,
    client: Arc<Client>,
}

impl WebhookNotifier {
    /// Create a new webhook notifier
    pub fn new(config: WebhookConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build HTTP client");

        info!("Webhook notifier initialized with {}s timeout", config.timeout_seconds);

        Self {
            config,
            client: Arc::new(client),
        }
    }

    /// Send notification to a recipient
    pub async fn send(
        &self,
        notification: &Notification,
        recipient: &Recipient,
    ) -> NotificationResult {
        let recipient_id = recipient.id;
        let notification_id = notification.id;

        // Check if recipient has webhook URL
        let webhook_url = match &recipient.webhook_url {
            Some(url) => url,
            None => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Webhook,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some("Recipient has no webhook URL".to_string()),
                    retry_count: 0,
                };
            }
        };

        // Build payload
        let payload = self.build_payload(notification, recipient);

        // Send webhook
        let sent_at = chrono::Utc::now();
        
        let mut request = self
            .client
            .request(Method::POST, webhook_url)
            .json(&payload);

        // Add default headers
        for (key, value) in &self.config.default_headers {
            request = request.header(key, value);
        }

        // Add any custom headers from notification metadata
        for (key, value) in &notification.metadata {
            if key.starts_with("webhook_header_") {
                let header_name = key.trim_start_matches("webhook_header_");
                request = request.header(header_name, value);
            }
        }

        match request.send().await {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    info!(
                        notification_id = %notification_id,
                        recipient_id = %recipient_id,
                        url = %webhook_url,
                        status = %status,
                        "Webhook delivered successfully"
                    );
                    
                    NotificationResult {
                        notification_id,
                        channel: crate::types::NotificationChannel::Webhook,
                        recipient_id,
                        status: NotificationStatus::Delivered,
                        sent_at: Some(sent_at),
                        delivered_at: Some(chrono::Utc::now()),
                        error_message: None,
                        retry_count: 0,
                    }
                } else {
                    let error_body = response.text().await
                        .unwrap_or_else(|_| "No response body".to_string());
                    
                    warn!(
                        notification_id = %notification_id,
                        recipient_id = %recipient_id,
                        status = %status,
                        body = %error_body,
                        "Webhook returned error status"
                    );
                    
                    NotificationResult {
                        notification_id,
                        channel: crate::types::NotificationChannel::Webhook,
                        recipient_id,
                        status: NotificationStatus::Failed,
                        sent_at: Some(sent_at),
                        delivered_at: None,
                        error_message: Some(format!("HTTP {}: {}", status, error_body)),
                        retry_count: 0,
                    }
                }
            }
            Err(e) => {
                error!(
                    notification_id = %notification_id,
                    recipient_id = %recipient_id,
                    error = %e,
                    "Failed to send webhook"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Webhook,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: Some(sent_at),
                    delivered_at: None,
                    error_message: Some(format!("Request failed: {}", e)),
                    retry_count: 0,
                }
            }
        }
    }

    /// Build webhook payload
    fn build_payload(&self, notification: &Notification, recipient: &Recipient) -> serde_json::Value {
        json!({
            "notification_id": notification.id.to_string(),
            "title": notification.title,
            "message": notification.message,
            "priority": notification.priority.as_str(),
            "channels": notification.channels.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
            "recipient": {
                "id": recipient.id.to_string(),
                "name": recipient.name,
            },
            "metadata": notification.metadata,
            "timestamp": notification.created_at.to_rfc3339(),
            "event_type": "alert",
            "version": "1.0"
        })
    }

    /// Send webhook with custom payload (for advanced use cases)
    pub async fn send_custom(
        &self,
        webhook_url: &str,
        payload: serde_json::Value,
        headers: Option<Vec<(String, String)>>,
    ) -> Result<NotificationResult> {
        let sent_at = chrono::Utc::now();
        
        let mut request = self
            .client
            .request(Method::POST, webhook_url)
            .json(&payload);

        // Add default headers
        for (key, value) in &self.config.default_headers {
            request = request.header(key, value);
        }

        // Add custom headers
        if let Some(custom_headers) = headers {
            for (key, value) in custom_headers {
                request = request.header(key, value);
            }
        }

        let response = request.send().await
            .context("Failed to send custom webhook")?;

        let status = response.status();
        
        if status.is_success() {
            Ok(NotificationResult {
                notification_id: uuid::Uuid::new_v4(),
                channel: crate::types::NotificationChannel::Webhook,
                recipient_id: uuid::Uuid::new_v4(),
                status: NotificationStatus::Delivered,
                sent_at: Some(sent_at),
                delivered_at: Some(chrono::Utc::now()),
                error_message: None,
                retry_count: 0,
            })
        } else {
            let error_body = response.text().await
                .unwrap_or_else(|_| "No response body".to_string());
            
            Err(anyhow::anyhow!("Webhook failed: HTTP {} - {}", status, error_body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NotificationBuilder, NotificationPriority, Recipient};

    #[test]
    fn test_build_payload() {
        let config = WebhookConfig::default();
        let notifier = WebhookNotifier::new(config);

        let recipient = Recipient::new()
            .with_email("test@example.com");

        let notification = NotificationBuilder::new("Test Alert", "Test message")
            .priority(NotificationPriority::High)
            .metadata("sensor_id", "S001")
            .build();

        let payload = notifier.build_payload(&notification, &recipient);
        
        assert_eq!(payload["title"], "Test Alert");
        assert_eq!(payload["priority"], "high");
        assert_eq!(payload["event_type"], "alert");
    }
}
