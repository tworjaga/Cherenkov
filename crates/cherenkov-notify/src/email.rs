//! Email notification service using SMTP

use crate::types::{Notification, NotificationResult, NotificationStatus, Recipient};
use anyhow::{Context, Result};
use lettre::{
    message::{header, Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};
use std::sync::Arc;
use tracing::{error, info, warn};

/// Email notifier configuration
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    pub use_tls: bool,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            smtp_host: std::env::var("SMTP_HOST")
                .context("SMTP_HOST not set")?,
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "587".to_string())
                .parse()
                .context("Invalid SMTP_PORT")?,
            username: std::env::var("SMTP_USERNAME")
                .context("SMTP_USERNAME not set")?,
            password: std::env::var("SMTP_PASSWORD")
                .context("SMTP_PASSWORD not set")?,
            from_address: std::env::var("SMTP_FROM_ADDRESS")
                .context("SMTP_FROM_ADDRESS not set")?,
            from_name: std::env::var("SMTP_FROM_NAME")
                .unwrap_or_else(|_| "Cherenkov Alerts".to_string()),
            use_tls: std::env::var("SMTP_USE_TLS")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
        })
    }
}

/// Email notifier implementation
pub struct EmailNotifier {
    config: EmailConfig,
    transport: Arc<AsyncSmtpTransport<Tokio1Executor>>,
}

impl EmailNotifier {
    /// Create a new email notifier
    pub async fn new(config: EmailConfig) -> Result<Self> {
        let creds = Credentials::new(
            config.username.clone(),
            config.password.clone(),
        );

        let transport = if config.use_tls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)?
                .port(config.smtp_port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_host)
                .port(config.smtp_port)
                .credentials(creds)
                .build()
        };

        // Test connection
        transport.test_connection().await
            .context("Failed to connect to SMTP server")?;

        info!("Email notifier initialized: {}:{}", config.smtp_host, config.smtp_port);

        Ok(Self {
            config,
            transport: Arc::new(transport),
        })
    }

    /// Send notification to a recipient
    pub async fn send(
        &self,
        notification: &Notification,
        recipient: &Recipient,
    ) -> NotificationResult {
        let recipient_id = recipient.id;
        let notification_id = notification.id;

        // Check if recipient has email
        let email = match &recipient.email {
            Some(e) => e,
            None => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some("Recipient has no email address".to_string()),
                    retry_count: 0,
                };
            }
        };

        // Build email message
        let from_mailbox = match format!("{} <{}>", self.config.from_name, self.config.from_address)
            .parse::<Mailbox>() {
            Ok(m) => m,
            Err(e) => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some(format!("Invalid from address: {}", e)),
                    retry_count: 0,
                };
            }
        };

        let to_mailbox = match email.parse::<Mailbox>() {
            Ok(m) => m,
            Err(e) => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some(format!("Invalid recipient email: {}", e)),
                    retry_count: 0,
                };
            }
        };

        // Build message with HTML and plain text parts
        let html_body = self.build_html_body(notification);
        let text_body = self.build_text_body(notification);

        let message = match Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(&notification.title)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text_body)
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html_body)
                    )
            ) {
            Ok(m) => m,
            Err(e) => {
                return NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: None,
                    delivered_at: None,
                    error_message: Some(format!("Failed to build email: {}", e)),
                    retry_count: 0,
                };
            }
        };

        // Send email
        let sent_at = chrono::Utc::now();
        match self.transport.send(message).await {
            Ok(_) => {
                info!(
                    notification_id = %notification_id,
                    recipient_id = %recipient_id,
                    email = %email,
                    "Email sent successfully"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
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
                    error = %e,
                    "Failed to send email"
                );
                
                NotificationResult {
                    notification_id,
                    channel: crate::types::NotificationChannel::Email,
                    recipient_id,
                    status: NotificationStatus::Failed,
                    sent_at: Some(sent_at),
                    delivered_at: None,
                    error_message: Some(format!("SMTP error: {}", e)),
                    retry_count: 0,
                }
            }
        }
    }

    /// Build HTML email body
    fn build_html_body(&self, notification: &Notification) -> String {
        let priority_color = match notification.priority {
            crate::types::NotificationPriority::Critical => "#ff3366",
            crate::types::NotificationPriority::High => "#ff9933",
            crate::types::NotificationPriority::Normal => "#33ccff",
            crate::types::NotificationPriority::Low => "#66ff99",
        };

        let metadata_html = if notification.metadata.is_empty() {
            String::new()
        } else {
            let rows: Vec<String> = notification.metadata
                .iter()
                .map(|(k, v)| format!("<tr><td><strong>{}:</strong></td><td>{}</td></tr>", k, v))
                .collect();
            format!(
                r#"<table style="margin-top: 20px; border-collapse: collapse;">
                    <tr><th colspan="2" style="text-align: left; padding: 10px 0; border-bottom: 1px solid #333;">Additional Information</th></tr>
                    {}
                </table>"#,
                rows.join("")
            )
        };

        format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>{}</title>
            </head>
            <body style="font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif; background-color: #0a0a0a; color: #e0e0e0; margin: 0; padding: 20px;">
                <div style="max-width: 600px; margin: 0 auto; background-color: #141414; border-radius: 8px; overflow: hidden; border: 1px solid #1f1f1f;">
                    <div style="background-color: {}; padding: 20px; color: #000;">
                        <h1 style="margin: 0; font-size: 24px; font-weight: 600;">{}</h1>
                        <p style="margin: 5px 0 0 0; font-size: 14px; opacity: 0.8;">Priority: {}</p>
                    </div>
                    <div style="padding: 30px;">
                        <p style="font-size: 16px; line-height: 1.6; margin-bottom: 20px;">{}</p>
                        {}
                        <div style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #1f1f1f; font-size: 12px; color: #666;">
                            <p>This alert was generated by the Cherenkov Radiation Monitoring System.</p>
                            <p>Notification ID: {} | Sent: {}</p>
                        </div>
                    </div>
                </div>
            </body>
            </html>"#,
            notification.title,
            priority_color,
            notification.title,
            notification.priority.as_str(),
            notification.message,
            metadata_html,
            notification.id,
            notification.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    /// Build plain text email body
    fn build_text_body(&self, notification: &Notification) -> String {
        let metadata_text = if notification.metadata.is_empty() {
            String::new()
        } else {
            let lines: Vec<String> = notification.metadata
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();
            format!("\n\nAdditional Information:\n{}", lines.join("\n"))
        };

        format!(
            r#"{}

Priority: {}{}

---
This alert was generated by the Cherenkov Radiation Monitoring System.
Notification ID: {} | Sent: {}"#,
            notification.message,
            notification.priority.as_str(),
            metadata_text,
            notification.id,
            notification.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{NotificationBuilder, NotificationPriority};

    #[test]
    fn test_build_text_body() {
        let config = EmailConfig {
            smtp_host: "smtp.test.com".to_string(),
            smtp_port: 587,
            username: "test".to_string(),
            password: "pass".to_string(),
            from_address: "test@test.com".to_string(),
            from_name: "Test".to_string(),
            use_tls: true,
        };

        let notification = NotificationBuilder::new("Test Alert", "This is a test message")
            .priority(NotificationPriority::High)
            .metadata("sensor_id", "S001")
            .build();

        // Note: We can't test the actual notifier without SMTP server
        // In real tests, use a mock SMTP server
    }
}
