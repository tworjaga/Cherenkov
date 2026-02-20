//! SMS notification service using Twilio

use crate::types::{Notification, NotificationResult, NotificationStatus, Recipient};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, warn};

/// SMS notifier configuration
#[derive(Debug, Clone)]
pub struct SmsConfig {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub api_url: String,
}

impl SmsConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            account_sid: std::env::var("TWILIO_ACCOUNT_SID")
                .context("TWILIO_ACCOUNT_SID not set")?,
            auth_token: std::env::var("TWILIO_AUTH_TOKEN")
                .context("TWILIO_AUTH_TOKEN not set")?,
            from_number: std::env::var("TWILIO_FROM_NUMBER")
                .context("TWILIO_FROM_NUMBER not set")?,
            api_url: std::env::var("TWILIO_API_URL")
                .unwrap_or_else(|_| "https://api.twilio.com/2010-04-01".to_string()),
        })
    }
}

/// SMS notifier implementation
pub struct SmsNotifier {
    config: SmsConfig,
    client: Arc<Client>,
}

impl SmsNotifier {
    /// Create a new SMS notifier
    pub fn new(config: SmsConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        info!("SMS notifier initialized for account: {}", 
            &config.account_sid[..8.min(config.account_sid.len())]);

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

        // Check if recipient has phone number
        let phone = match &recipient.phone {
            Some(p) => p,
            None => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Sms,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some("Recipient has no phone number".to_string()),
                    retry_count: 0,
                };
            }
        };

        // Build message
        let message = self.build_message(notification);

        // Send via Twilio API
        let url = format!(
            "{}/Accounts/{}/Messages.json",
            self.config.api_url, self.config.account_sid
        );

        let sent_at = chrono::Utc::now();
        
        let result = self
            .client
            .post(&url)
            .basic_auth(&self.config.account_sid, Some(&self.config.auth_token))
            .form(&[
                ("To", phone.as_str()),
                ("From", &self.config.from_number),
                ("Body", &message),
            ])
            .send()
            .await;

        match result {
            Ok(response) => {
                let status = response.status();
                
                if status.is_success() {
                    info!(
                        notification_id = %notification_id,
                        recipient_id = %recipient_id,
                        phone = %phone,
                        "SMS sent successfully"
                    );
                    
                    NotificationResult {
                        notification_id,
                        channel: crate::types::NotificationChannel::Sms,
                        recipient_id,
                        status: NotificationStatus::Delivered,
                        sent_at: Some(sent_at),
                        delivered_at: Some(chrono::Utc::now()),
                        error_message: None,
                        retry_count: 0,
                    }
                } else {
                    let error_text = response.text().await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    
                    error!(
                        notification_id = %notification_id,
                        recipient_id = %recipient_id,
                        status = %status,
                        error = %error_text,
                        "Failed to send SMS"
                    );
                    
                    NotificationResult {
                        notification_id,
                        channel: crate::types::NotificationChannel::Sms,
                        recipient_id,
                        status: NotificationStatus::Failed,
                        sent_at: Some(sent_at),
                        delivered_at: None,
                        error_message: Some(format!("Twilio error {}: {}", status, error_text)),
                        retry_count: 0,
                    }
                }
            }
            Err(e) => {
                error!(
                    notification_id = %notification_id,
                    recipient_id = %recipient_id,
                    error = %e,
                    "Failed to send SMS request"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Sms,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: Some(sent_at),
                    delivered_at: None,
                    error_message: Some(format!("Request error: {}", e)),
                    retry_count: 0,
                }
            }
        }
    }

    /// Build SMS message from notification
    fn build_message(&self, notification: &Notification) -> String {
        // SMS has 160 char limit per segment, keep it concise
        let priority_emoji = match notification.priority {
            crate::types::NotificationPriority::Critical => "🔴",
            crate::types::NotificationPriority::High => "🟠",
            crate::types::NotificationPriority::Normal => "🔵",
            crate::types::NotificationPriority::Low => "🟢",
        };

        let mut message = format!(
            "{} {}: {}",
            priority_emoji,
            notification.title,
            notification.message
        );

        // Truncate if too long (keep under 320 chars for 2 segments)
        if message.len() > 300 {
            message.truncate(297);
            message.push_str("...");
        }

        message
    }

    /// Validate phone number format (E.164)
    pub fn validate_phone(phone: &str) -> bool {
        // E.164 format: +[country code][number], 10-15 digits total
        let re = regex::Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
        re.is_match(phone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NotificationBuilder, NotificationPriority};

    #[test]
    fn test_build_message() {
        let config = SmsConfig {
            account_sid: "test_account".to_string(),
            auth_token: "test_token".to_string(),
            from_number: "+1234567890".to_string(),
            api_url: "https://api.twilio.com/2010-04-01".to_string(),
        };

        let notifier = SmsNotifier::new(config);

        let notification = NotificationBuilder::new("Test Alert", "This is a test message")
            .priority(NotificationPriority::Critical)
            .build();

        let message = notifier.build_message(&notification);
        assert!(message.contains("🔴"));
        assert!(message.contains("Test Alert"));
    }

    #[test]
    fn test_validate_phone() {
        assert!(SmsNotifier::validate_phone("+1234567890"));
        assert!(SmsNotifier::validate_phone("+441234567890"));
        assert!(!SmsNotifier::validate_phone("1234567890"));
        assert!(!SmsNotifier::validate_phone("+123"));
    }
}
