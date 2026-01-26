//! SMTP email adapter using lettre.

use async_trait::async_trait;
use lettre::message::{header::ContentType, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use crate::adapter::EmailAdapter;
use crate::error::{EmailError, EmailResult};
use crate::manager::{SmtpConfig, TlsMode};
use crate::types::{Email, EmailAddress, SendResult};

/// SMTP email adapter using the lettre library.
pub struct SmtpAdapter {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpAdapter {
    /// Create a new SMTP adapter with the given configuration.
    pub fn new(config: &SmtpConfig) -> EmailResult<Self> {
        let builder = match config.tls_mode {
            TlsMode::Required => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                    .map_err(|e| EmailError::ConfigError(format!("SMTP relay error: {}", e)))?
            }
            TlsMode::Opportunistic => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                    .map_err(|e| EmailError::ConfigError(format!("SMTP relay error: {}", e)))?
            }
            TlsMode::None => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host),
        };

        let mut builder = builder.port(config.port);

        // Add credentials if provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            let credentials = Credentials::new(username.clone(), password.clone());
            builder = builder.credentials(credentials);
        }

        let transport = builder.build();

        Ok(Self { transport })
    }

    /// Convert an EmailAddress to a lettre Mailbox.
    fn to_mailbox(addr: &EmailAddress) -> EmailResult<Mailbox> {
        let email = addr
            .email
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("{}: {}", addr.email, e)))?;

        Ok(Mailbox::new(addr.name.clone(), email))
    }

    /// Build a lettre Message from our Email type.
    fn build_message(email: &Email) -> EmailResult<Message> {
        let from = Self::to_mailbox(&email.from)?;

        // Build the To header
        let to_mailboxes: Vec<Mailbox> = email
            .to
            .iter()
            .map(Self::to_mailbox)
            .collect::<EmailResult<Vec<_>>>()?;

        let mut builder = Message::builder()
            .from(from)
            .subject(&email.subject)
            .message_id(Some(format!("<{}@underlay>", email.id)));

        // Add recipients
        for mailbox in to_mailboxes {
            builder = builder.to(mailbox);
        }

        // Add CC recipients
        for addr in &email.cc {
            builder = builder.cc(Self::to_mailbox(addr)?);
        }

        // Add BCC recipients
        for addr in &email.bcc {
            builder = builder.bcc(Self::to_mailbox(addr)?);
        }

        // Add Reply-To if present
        if let Some(reply_to) = &email.reply_to {
            builder = builder.reply_to(Self::to_mailbox(reply_to)?);
        }

        // Build the message body
        let message = match (&email.text_body, &email.html_body) {
            (Some(text), Some(html)) => {
                // Multipart message with both text and HTML
                builder
                    .multipart(
                        MultiPart::alternative()
                            .singlepart(
                                SinglePart::builder()
                                    .header(ContentType::TEXT_PLAIN)
                                    .body(text.clone()),
                            )
                            .singlepart(
                                SinglePart::builder()
                                    .header(ContentType::TEXT_HTML)
                                    .body(html.clone()),
                            ),
                    )
                    .map_err(|e| EmailError::ConfigError(format!("Failed to build message: {}", e)))?
            }
            (Some(text), None) => {
                // Text only
                builder
                    .header(ContentType::TEXT_PLAIN)
                    .body(text.clone())
                    .map_err(|e| EmailError::ConfigError(format!("Failed to build message: {}", e)))?
            }
            (None, Some(html)) => {
                // HTML only
                builder
                    .header(ContentType::TEXT_HTML)
                    .body(html.clone())
                    .map_err(|e| EmailError::ConfigError(format!("Failed to build message: {}", e)))?
            }
            (None, None) => {
                return Err(EmailError::MissingField("text_body or html_body"));
            }
        };

        Ok(message)
    }
}

#[async_trait]
impl EmailAdapter for SmtpAdapter {
    async fn send(&self, email: &Email) -> EmailResult<SendResult> {
        let message = Self::build_message(email)?;

        match self.transport.send(message).await {
            Ok(response) => {
                // response.message() returns an iterator of response lines
                let message_id = response.message().next().map(|s| s.to_string());
                Ok(SendResult::success(email.id, message_id))
            }
            Err(e) => {
                let error_msg = e.to_string();

                // Categorize the error
                if error_msg.contains("authentication") || error_msg.contains("AUTH") {
                    Err(EmailError::AuthenticationFailed(error_msg))
                } else if error_msg.contains("connection")
                    || error_msg.contains("connect")
                    || error_msg.contains("timeout")
                {
                    Err(EmailError::ConnectionFailed(error_msg))
                } else if error_msg.contains("rejected") || error_msg.contains("550") {
                    Err(EmailError::Rejected(error_msg))
                } else {
                    Err(EmailError::TransportError(error_msg))
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "smtp"
    }

    async fn health_check(&self) -> EmailResult<()> {
        // Test the connection by checking if we can connect
        self.transport
            .test_connection()
            .await
            .map_err(|e| EmailError::ConnectionFailed(e.to_string()))?;
        Ok(())
    }
}

impl std::fmt::Debug for SmtpAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmtpAdapter")
            .field("transport", &"AsyncSmtpTransport<...>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smtp_config_builder() {
        let config = SmtpConfig {
            host: "smtp.example.com".to_string(),
            port: 587,
            username: Some("user".to_string()),
            password: Some("pass".to_string()),
            tls_mode: TlsMode::Required,
        };

        assert_eq!(config.host, "smtp.example.com");
        assert_eq!(config.port, 587);
    }

    #[test]
    fn test_to_mailbox() {
        let addr = EmailAddress::new("user@example.com").unwrap();
        let mailbox = SmtpAdapter::to_mailbox(&addr).unwrap();
        assert_eq!(mailbox.email.to_string(), "user@example.com");
        assert!(mailbox.name.is_none());

        let addr_with_name = EmailAddress::with_name("user@example.com", "John Doe").unwrap();
        let mailbox_with_name = SmtpAdapter::to_mailbox(&addr_with_name).unwrap();
        assert_eq!(mailbox_with_name.name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_build_message_text_only() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .build()
            .unwrap();

        let message = SmtpAdapter::build_message(&email).unwrap();
        // Message builds successfully
        assert!(message.headers().get_raw("Subject").is_some());
    }

    #[test]
    fn test_build_message_multipart() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .html_body("<h1>Hello, World!</h1>")
            .build()
            .unwrap();

        let message = SmtpAdapter::build_message(&email).unwrap();
        assert!(message.headers().get_raw("Subject").is_some());
    }
}
