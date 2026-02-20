//! Notification types and data structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Notification priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl NotificationPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationPriority::Low => "low",
            NotificationPriority::Normal => "normal",
            NotificationPriority::High => "high",
            NotificationPriority::Critical => "critical",
        }
    }
}

/// Notification delivery channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum NotificationChannel {
    Email,
    Sms,
    Webhook,
    Telegram,
}

impl NotificationChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            NotificationChannel::Email => "email",
            NotificationChannel::Sms => "sms",
            NotificationChannel::Webhook => "webhook",
            NotificationChannel::Telegram => "telegram",
        }
    }
}

/// Notification delivery status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Retrying,
}

/// Core notification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub channels: Vec<NotificationChannel>,
    pub recipients: Vec<Recipient>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub scheduled_for: Option<DateTime<Utc>>,
}

impl Notification {
    pub fn builder(title: impl Into<String>, message: impl Into<String>) -> NotificationBuilder {
        NotificationBuilder::new(title, message)
    }
}

/// Builder for constructing notifications
pub struct NotificationBuilder {
    title: String,
    message: String,
    priority: NotificationPriority,
    channels: Vec<NotificationChannel>,
    recipients: Vec<Recipient>,
    metadata: HashMap<String, String>,
    scheduled_for: Option<DateTime<Utc>>,
}

impl NotificationBuilder {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            priority: NotificationPriority::Normal,
            channels: Vec::new(),
            recipients: Vec::new(),
            metadata: HashMap::new(),
            scheduled_for: None,
        }
    }

    pub fn priority(mut self, priority: NotificationPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn channel(mut self, channel: NotificationChannel) -> Self {
        self.channels.push(channel);
        self
    }

    pub fn channels(mut self, channels: Vec<NotificationChannel>) -> Self {
        self.channels = channels;
        self
    }

    pub fn recipient(mut self, recipient: Recipient) -> Self {
        self.recipients.push(recipient);
        self
    }

    pub fn recipients(mut self, recipients: Vec<Recipient>) -> Self {
        self.recipients = recipients;
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn scheduled_for(mut self, time: DateTime<Utc>) -> Self {
        self.scheduled_for = Some(time);
        self
    }

    pub fn build(self) -> Notification {
        Notification {
            id: Uuid::new_v4(),
            title: self.title,
            message: self.message,
            priority: self.priority,
            channels: self.channels,
            recipients: self.recipients,
            metadata: self.metadata,
            created_at: Utc::now(),
            scheduled_for: self.scheduled_for,
        }
    }
}

/// Notification recipient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipient {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub telegram_chat_id: Option<i64>,
    pub webhook_url: Option<String>,
    pub preferences: RecipientPreferences,
}

impl Recipient {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: None,
            email: None,
            phone: None,
            telegram_chat_id: None,
            webhook_url: None,
            preferences: RecipientPreferences::default(),
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn with_telegram(mut self, chat_id: i64) -> Self {
        self.telegram_chat_id = Some(chat_id);
        self
    }

    pub fn with_webhook(mut self, url: impl Into<String>) -> Self {
        self.webhook_url = Some(url.into());
        self
    }
}

impl Default for Recipient {
    fn default() -> Self {
        Self::new()
    }
}

/// Recipient notification preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientPreferences {
    pub quiet_hours_start: Option<u8>, // Hour (0-23)
    pub quiet_hours_end: Option<u8>,
    pub min_priority: NotificationPriority,
    pub enabled_channels: Vec<NotificationChannel>,
}

impl Default for RecipientPreferences {
    fn default() -> Self {
        Self {
            quiet_hours_start: None,
            quiet_hours_end: None,
            min_priority: NotificationPriority::Low,
            enabled_channels: vec![
                NotificationChannel::Email,
                NotificationChannel::Telegram,
            ],
        }
    }
}

/// Result of a notification delivery attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    pub notification_id: Uuid,
    pub channel: NotificationChannel,
    pub recipient_id: Uuid,
    pub status: NotificationStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: u32,
}

/// Template for notification messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub subject_template: String,
    pub body_template: String,
    pub channels: Vec<NotificationChannel>,
}

/// Alert event data for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub alert_id: Uuid,
    pub alert_type: String,
    pub severity: String,
    pub location: Option<String>,
    pub sensor_id: Option<String>,
    pub reading_value: Option<f64>,
    pub threshold_value: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}
