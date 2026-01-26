use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{EmailError, EmailResult};

/// An email address with optional display name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress {
    /// The email address (e.g., "user@example.com").
    pub email: String,
    /// Optional display name (e.g., "John Doe").
    pub name: Option<String>,
}

impl EmailAddress {
    /// Create a new email address without a display name.
    pub fn new(email: impl Into<String>) -> EmailResult<Self> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Self { email, name: None })
    }

    /// Create a new email address with a display name.
    pub fn with_name(email: impl Into<String>, name: impl Into<String>) -> EmailResult<Self> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Self {
            email,
            name: Some(name.into()),
        })
    }

    /// Basic email validation - checks for @ symbol and non-empty parts.
    fn validate(email: &str) -> EmailResult<()> {
        let email = email.trim();
        if email.is_empty() {
            return Err(EmailError::InvalidAddress("email cannot be empty".into()));
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err(EmailError::InvalidAddress(format!(
                "invalid email format: {}",
                email
            )));
        }

        let (local, domain) = (parts[0], parts[1]);
        if local.is_empty() || domain.is_empty() {
            return Err(EmailError::InvalidAddress(format!(
                "invalid email format: {}",
                email
            )));
        }

        if !domain.contains('.') {
            return Err(EmailError::InvalidAddress(format!(
                "invalid domain in email: {}",
                email
            )));
        }

        Ok(())
    }

    /// Returns the email address as a formatted string.
    /// If a name is present, returns "Name <email>", otherwise just the email.
    pub fn formatted(&self) -> String {
        match &self.name {
            Some(name) => format!("{} <{}>", name, self.email),
            None => self.email.clone(),
        }
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

/// An email message to be sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Unique identifier for this email.
    pub id: Uuid,
    /// Recipients (To field).
    pub to: Vec<EmailAddress>,
    /// Sender (From field).
    pub from: EmailAddress,
    /// Reply-To address (optional).
    pub reply_to: Option<EmailAddress>,
    /// CC recipients (optional).
    pub cc: Vec<EmailAddress>,
    /// BCC recipients (optional).
    pub bcc: Vec<EmailAddress>,
    /// Email subject.
    pub subject: String,
    /// Plain text body (optional but recommended).
    pub text_body: Option<String>,
    /// HTML body (optional).
    pub html_body: Option<String>,
    /// Custom headers.
    pub headers: HashMap<String, String>,
    /// When the email was created.
    pub created_at: DateTime<Utc>,
}

impl Email {
    /// Create a new email builder.
    pub fn builder() -> EmailBuilder {
        EmailBuilder::new()
    }
}

/// Builder for constructing Email instances.
#[derive(Debug, Default)]
pub struct EmailBuilder {
    to: Vec<EmailAddress>,
    from: Option<EmailAddress>,
    reply_to: Option<EmailAddress>,
    cc: Vec<EmailAddress>,
    bcc: Vec<EmailAddress>,
    subject: Option<String>,
    text_body: Option<String>,
    html_body: Option<String>,
    headers: HashMap<String, String>,
}

impl EmailBuilder {
    /// Create a new email builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a recipient.
    pub fn to(mut self, address: EmailAddress) -> Self {
        self.to.push(address);
        self
    }

    /// Add multiple recipients.
    pub fn to_many(mut self, addresses: impl IntoIterator<Item = EmailAddress>) -> Self {
        self.to.extend(addresses);
        self
    }

    /// Set the sender.
    pub fn from(mut self, address: EmailAddress) -> Self {
        self.from = Some(address);
        self
    }

    /// Set the reply-to address.
    pub fn reply_to(mut self, address: EmailAddress) -> Self {
        self.reply_to = Some(address);
        self
    }

    /// Add a CC recipient.
    pub fn cc(mut self, address: EmailAddress) -> Self {
        self.cc.push(address);
        self
    }

    /// Add a BCC recipient.
    pub fn bcc(mut self, address: EmailAddress) -> Self {
        self.bcc.push(address);
        self
    }

    /// Set the subject.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Set the plain text body.
    pub fn text_body(mut self, body: impl Into<String>) -> Self {
        self.text_body = Some(body.into());
        self
    }

    /// Set the HTML body.
    pub fn html_body(mut self, body: impl Into<String>) -> Self {
        self.html_body = Some(body.into());
        self
    }

    /// Add a custom header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Build the email.
    pub fn build(self) -> EmailResult<Email> {
        let from = self
            .from
            .ok_or(EmailError::MissingField("from address"))?;

        if self.to.is_empty() {
            return Err(EmailError::MissingField("at least one recipient"));
        }

        let subject = self.subject.ok_or(EmailError::MissingField("subject"))?;

        if self.text_body.is_none() && self.html_body.is_none() {
            return Err(EmailError::MissingField("text_body or html_body"));
        }

        Ok(Email {
            id: Uuid::now_v7(),
            to: self.to,
            from,
            reply_to: self.reply_to,
            cc: self.cc,
            bcc: self.bcc,
            subject,
            text_body: self.text_body,
            html_body: self.html_body,
            headers: self.headers,
            created_at: Utc::now(),
        })
    }
}

/// Result of sending an email.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    /// The email ID.
    pub email_id: Uuid,
    /// Whether the send was successful.
    pub success: bool,
    /// Message ID from the email provider (if available).
    pub message_id: Option<String>,
    /// Error message if the send failed.
    pub error: Option<String>,
    /// When the email was sent (or attempted).
    pub sent_at: DateTime<Utc>,
}

impl SendResult {
    /// Create a successful send result.
    pub fn success(email_id: Uuid, message_id: Option<String>) -> Self {
        Self {
            email_id,
            success: true,
            message_id,
            error: None,
            sent_at: Utc::now(),
        }
    }

    /// Create a failed send result.
    pub fn failure(email_id: Uuid, error: impl Into<String>) -> Self {
        Self {
            email_id,
            success: false,
            message_id: None,
            error: Some(error.into()),
            sent_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address_new() {
        let addr = EmailAddress::new("user@example.com").unwrap();
        assert_eq!(addr.email, "user@example.com");
        assert!(addr.name.is_none());
    }

    #[test]
    fn test_email_address_with_name() {
        let addr = EmailAddress::with_name("user@example.com", "John Doe").unwrap();
        assert_eq!(addr.email, "user@example.com");
        assert_eq!(addr.name, Some("John Doe".to_string()));
        assert_eq!(addr.formatted(), "John Doe <user@example.com>");
    }

    #[test]
    fn test_email_address_invalid() {
        assert!(EmailAddress::new("").is_err());
        assert!(EmailAddress::new("invalid").is_err());
        assert!(EmailAddress::new("missing@domain").is_err());
        assert!(EmailAddress::new("@example.com").is_err());
        assert!(EmailAddress::new("user@").is_err());
    }

    #[test]
    fn test_email_builder() {
        let email = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test Subject")
            .text_body("Hello, World!")
            .build()
            .unwrap();

        assert_eq!(email.from.email, "sender@example.com");
        assert_eq!(email.to.len(), 1);
        assert_eq!(email.to[0].email, "recipient@example.com");
        assert_eq!(email.subject, "Test Subject");
        assert_eq!(email.text_body, Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_email_builder_missing_from() {
        let result = Email::builder()
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test")
            .text_body("Body")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_email_builder_missing_to() {
        let result = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .subject("Test")
            .text_body("Body")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_email_builder_missing_body() {
        let result = Email::builder()
            .from(EmailAddress::new("sender@example.com").unwrap())
            .to(EmailAddress::new("recipient@example.com").unwrap())
            .subject("Test")
            .build();

        assert!(result.is_err());
    }
}
