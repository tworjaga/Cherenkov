//! Cherenkov Notification System
//! 
//! Multi-channel notification delivery with support for:
//! - Email (SMTP)
//! - SMS (Twilio)
//! - Webhooks
//! - Telegram Bot

pub mod email;
pub mod sms;
pub mod webhook;
pub mod telegram;
pub mod types;
pub mod service;
pub mod rate_limiter;

pub use types::{
    Notification, NotificationChannel, NotificationPriority, 
    NotificationStatus, NotificationResult
};
pub use service::NotificationService;
pub use email::EmailNotifier;
pub use sms::SmsNotifier;
pub use webhook::WebhookNotifier;
pub use telegram::TelegramNotifier;
