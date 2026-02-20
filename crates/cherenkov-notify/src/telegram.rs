//! Telegram notification service using teloxide

use crate::types::{Notification, NotificationResult, NotificationStatus, Recipient};
use anyhow::{Context, Result};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, Recipient as TgRecipient};
use tracing::{error, info, warn};

/// Telegram notifier configuration
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub default_chat_id: Option<i64>,
    pub parse_mode: ParseMode,
}

impl TelegramConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            bot_token: std::env::var("TELEGRAM_BOT_TOKEN")
                .context("TELEGRAM_BOT_TOKEN not set")?,
            default_chat_id: std::env::var("TELEGRAM_DEFAULT_CHAT_ID")
                .ok()
                .and_then(|v| v.parse().ok()),
            parse_mode: ParseMode::Html,
        })
    }
}

/// Telegram notifier implementation
pub struct TelegramNotifier {
    config: TelegramConfig,
    bot: Arc<Bot>,
}

impl TelegramNotifier {
    /// Create a new Telegram notifier
    pub fn new(config: TelegramConfig) -> Self {
        let bot = Bot::new(&config.bot_token);

        info!("Telegram notifier initialized");

        Self {
            config,
            bot: Arc::new(bot),
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

        // Check if recipient has Telegram chat ID
        let chat_id = match recipient.telegram_chat_id {
            Some(id) => id,
            None => {
                // Try default chat ID
                match self.config.default_chat_id {
                    Some(id) => id,
                    None => {
                        return NotificationResult {
                            notification_id,
                            channel: crate::types::NotificationChannel::Telegram,
                            recipient_id,
                            status: NotificationStatus::Failed,
                            sent_at: None,
                            delivered_at: None,
                            error_message: Some("Recipient has no Telegram chat ID".to_string()),
                            retry_count: 0,
                        };
                    }
                }
            }
        };

        // Build message
        let message = self.build_message(notification);

        // Send message
        let sent_at = chrono::Utc::now();
        
        match self
            .bot
            .send_message(ChatId(chat_id), message)
            .parse_mode(self.config.parse_mode)
            .await
        {
            Ok(_) => {
                info!(
                    notification_id = %notification_id,
                    recipient_id = %recipient_id,
                    chat_id = %chat_id,
                    "Telegram message sent successfully"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Telegram,
                    recipient_id,
                    status: NotificationStatus::Delivered,
                    sent_at: Some(sent_at),
                    delivered_at: Some(chrono::Utc::now()),
                    error_message: None,
                    retry_count: 0,
                }
            }
            Err(e) => {
                error!(
                    notification_id = %notification_id,
                    recipient_id = %recipient_id,
                    chat_id = %chat_id,
                    error = %e,
                    "Failed to send Telegram message"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Telegram,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: Some(sent_at),
                    delivered_at: None,
                    error_message: Some(format!("Telegram API error: {}", e)),
                    retry_count: 0,
                }
            }
        }
    }

    /// Send notification to multiple recipients
    pub async fn broadcast(
        &self,
        notification: &Notification,
        chat_ids: Vec<i64>,
    ) -> Vec<NotificationResult> {
        let mut results = Vec::new();

        for chat_id in chat_ids {
            let recipient = Recipient::new().with_telegram(chat_id);
            let result = self.send(notification, &recipient).await;
            results.push(result);
        }

        results
    }

    /// Build Telegram message from notification
    fn build_message(&self, notification: &Notification) -> String {
        let priority_emoji = match notification.priority {
            crate::types::NotificationPriority::Critical => "🔴",
            crate::types::NotificationPriority::High => "🟠",
            crate::types::NotificationPriority::Normal => "🔵",
            crate::types::NotificationPriority::Low => "🟢",
        };

        let mut message = format!(
            "<b>{} {}</b>\n\n",
            priority_emoji,
            html_escape(&notification.title)
        );

        message.push_str(&html_escape(&notification.message));
        message.push_str("\n\n");

        // Add metadata
        if !notification.metadata.is_empty() {
            message.push_str("<b>Details:</b>\n");
            for (key, value) in &notification.metadata {
                message.push_str(&format!(
                    "• <i>{}:</i> {}\n",
                    html_escape(key),
                    html_escape(value)
                ));
            }
        }

        // Add footer
        message.push_str(&format!(
            "\n<i>Notification ID: {}</i>",
            notification.id
        ));

        message
    }

    /// Get bot info
    pub async fn get_bot_info(&self) -> Result<teloxide::types::User> {
        self.bot.get_me().await
            .context("Failed to get bot info")
    }
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "<")
        .replace('>', ">")
        .replace('"', """)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NotificationBuilder, NotificationPriority};

    #[test]
    fn test_build_message() {
        let config = TelegramConfig {
            bot_token: "test_token".to_string(),
            default_chat_id: Some(123456),
            parse_mode: ParseMode::Html,
        };

        let notifier = TelegramNotifier::new(config);

        let notification = NotificationBuilder::new("Test Alert", "Test <message>")
            .priority(NotificationPriority::Critical)
            .metadata("sensor_id", "S001")
            .build();

        let message = notifier.build_message(&notification);
        
        assert!(message.contains("🔴"));
        assert!(message.contains("Test <message>")); // HTML escaped
        assert!(message.contains("sensor_id"));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "<script>");
        assert_eq!(html_escape("&test"), "&amp;test");
        assert_eq!(html_escape("\"quote\""), ""quote"");
    }
}
