//! AWS SES email adapter.

use async_trait::async_trait;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message as SesMessage};
use aws_sdk_sesv2::Client;

use crate::adapter::EmailAdapter;
use crate::error::{EmailError, EmailResult};
use crate::manager::SesConfig;
use crate::types::{Email, SendResult};

/// AWS SES email adapter.
pub struct SesAdapter {
    client: Client,
    configuration_set: Option<String>,
}

impl SesAdapter {
    /// Create a new SES adapter with the given configuration.
    ///
    /// This uses the default AWS credential chain (environment variables,
    /// IAM role, shared credentials file, etc.).
    pub async fn new(config: &SesConfig) -> EmailResult<Self> {
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_config::Region::new(config.region.clone()))
            .load()
            .await;

        let client = Client::new(&aws_config);

        Ok(Self {
            client,
            configuration_set: config.configuration_set.clone(),
        })
    }

    /// Create a new SES adapter from an existing AWS SDK config.
    pub fn from_aws_config(
        aws_config: &aws_config::SdkConfig,
        configuration_set: Option<String>,
    ) -> Self {
        let client = Client::new(aws_config);
        Self {
            client,
            configuration_set,
        }
    }

    /// Build the SES email content from our Email type.
    fn build_content(email: &Email) -> EmailResult<EmailContent> {
        let subject = Content::builder()
            .data(&email.subject)
            .charset("UTF-8")
            .build()
            .map_err(|e| EmailError::ConfigError(format!("Failed to build subject: {}", e)))?;

        let mut body_builder = Body::builder();

        if let Some(text) = &email.text_body {
            let text_content = Content::builder()
                .data(text)
                .charset("UTF-8")
                .build()
                .map_err(|e| {
                    EmailError::ConfigError(format!("Failed to build text body: {}", e))
                })?;
            body_builder = body_builder.text(text_content);
        }

        if let Some(html) = &email.html_body {
            let html_content = Content::builder()
                .data(html)
                .charset("UTF-8")
                .build()
                .map_err(|e| {
                    EmailError::ConfigError(format!("Failed to build HTML body: {}", e))
                })?;
            body_builder = body_builder.html(html_content);
        }

        let body = body_builder.build();

        let message = SesMessage::builder().subject(subject).body(body).build();

        Ok(EmailContent::builder().simple(message).build())
    }

    /// Build the SES destination from our Email type.
    fn build_destination(email: &Email) -> Destination {
        let mut builder = Destination::builder();

        for addr in &email.to {
            builder = builder.to_addresses(addr.formatted());
        }

        for addr in &email.cc {
            builder = builder.cc_addresses(addr.formatted());
        }

        for addr in &email.bcc {
            builder = builder.bcc_addresses(addr.formatted());
        }

        builder.build()
    }
}

#[async_trait]
impl EmailAdapter for SesAdapter {
    async fn send(&self, email: &Email) -> EmailResult<SendResult> {
        let content = Self::build_content(email)?;
        let destination = Self::build_destination(email);

        let mut request = self
            .client
            .send_email()
            .from_email_address(email.from.formatted())
            .destination(destination)
            .content(content);

        // Add reply-to if present
        if let Some(reply_to) = &email.reply_to {
            request = request.reply_to_addresses(reply_to.formatted());
        }

        // Add configuration set if configured
        if let Some(config_set) = &self.configuration_set {
            request = request.configuration_set_name(config_set);
        }

        match request.send().await {
            Ok(output) => {
                let message_id = output.message_id;
                Ok(SendResult::success(email.id, message_id))
            }
            Err(e) => {
                let error_msg = e.to_string();

                // Categorize SES-specific errors
                if error_msg.contains("MessageRejected") {
                    Err(EmailError::Rejected(error_msg))
                } else if error_msg.contains("Throttling") || error_msg.contains("rate") {
                    Err(EmailError::RateLimited(error_msg))
                } else if error_msg.contains("credentials")
                    || error_msg.contains("AccessDenied")
                    || error_msg.contains("InvalidClientTokenId")
                {
                    Err(EmailError::AuthenticationFailed(error_msg))
                } else {
                    Err(EmailError::TransportError(error_msg))
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "ses"
    }

    async fn health_check(&self) -> EmailResult<()> {
        // Use get_account to verify credentials and connectivity
        self.client
            .get_account()
            .send()
            .await
            .map_err(|e| EmailError::ConnectionFailed(e.to_string()))?;

        Ok(())
    }
}

impl std::fmt::Debug for SesAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SesAdapter")
            .field("client", &"aws_sdk_sesv2::Client<...>")
            .field("configuration_set", &self.configuration_set)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EmailAddress;

    #[test]
    fn test_build_content_text_only() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .build()
            .unwrap();

        let content = SesAdapter::build_content(&email).unwrap();
        assert!(content.simple().is_some());
    }

    #[test]
    fn test_build_content_html_only() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .html_body("<h1>Hello, World!</h1>")
            .build()
            .unwrap();

        let content = SesAdapter::build_content(&email).unwrap();
        assert!(content.simple().is_some());
    }

    #[test]
    fn test_build_content_multipart() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .html_body("<h1>Hello, World!</h1>")
            .build()
            .unwrap();

        let content = SesAdapter::build_content(&email).unwrap();
        assert!(content.simple().is_some());
    }

    #[test]
    fn test_build_destination() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("to1@example.com").unwrap())
            .to(EmailAddress::new("to2@example.com").unwrap())
            .cc(EmailAddress::new("cc@example.com").unwrap())
            .bcc(EmailAddress::new("bcc@example.com").unwrap())
            .subject("Test")
            .text_body("Hello")
            .build()
            .unwrap();

        let destination = SesAdapter::build_destination(&email);
        assert_eq!(destination.to_addresses().len(), 2);
        assert_eq!(destination.cc_addresses().len(), 1);
        assert_eq!(destination.bcc_addresses().len(), 1);
    }
}
