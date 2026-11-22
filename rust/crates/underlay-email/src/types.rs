use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{EmailError, EmailResult};

/// An email address with optional display name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "EmailAddressFields")]
pub struct EmailAddress {
    /// The email address (e.g., "user@example.com").
    pub email: String,
    /// Optional display name (e.g., "John Doe").
    pub name: Option<String>,
}

#[derive(Deserialize)]
struct EmailAddressFields {
    email: String,
    name: Option<String>,
}

impl TryFrom<EmailAddressFields> for EmailAddress {
    type Error = EmailError;

    fn try_from(fields: EmailAddressFields) -> EmailResult<Self> {
        match fields.name {
            Some(name) => Self::with_name(fields.email, name),
            None => Self::new(fields.email),
        }
    }
}

impl EmailAddress {
    /// Create a new email address without a display name.
    pub fn new(email: impl Into<String>) -> EmailResult<Self> {
        let raw = email.into();
        Self::validate_email(&raw)?;
        Ok(Self {
            email: raw.trim().to_string(),
            name: None,
        })
    }

    /// Create a new email address with a display name.
    pub fn with_name(email: impl Into<String>, name: impl Into<String>) -> EmailResult<Self> {
        let raw = email.into();
        Self::validate_email(&raw)?;
        let address = Self {
            email: raw.trim().to_string(),
            name: Some(name.into()),
        };
        address.validate()?;
        Ok(address)
    }

    fn validate(&self) -> EmailResult<()> {
        Self::validate_email(&self.email)?;
        if self.name.as_deref().is_some_and(contains_control) {
            return Err(EmailError::InvalidAddress(
                "display name cannot contain control characters".into(),
            ));
        }
        Ok(())
    }

    /// Basic email validation - checks for @ symbol, non-empty parts, and no
    /// header-injection / control characters.
    fn validate_email(email: &str) -> EmailResult<()> {
        if contains_control(email) {
            return Err(EmailError::InvalidAddress(
                "email cannot contain control characters".into(),
            ));
        }
        let email = email.trim();
        if email.is_empty() {
            return Err(EmailError::InvalidAddress("email cannot be empty".into()));
        }
        if email.chars().any(char::is_whitespace) {
            return Err(EmailError::InvalidAddress(
                "email cannot contain whitespace".into(),
            ));
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
    /// Display names that need quoting use RFC 5322 quoted-string escaping.
    pub fn formatted(&self) -> String {
        match &self.name {
            Some(name) if name.split(' ').all(is_display_name_atom) => {
                format!("{name} <{}>", self.email)
            }
            Some(name) => {
                let mut escaped = String::with_capacity(name.len());
                for character in name.chars() {
                    if matches!(character, '\\' | '"') {
                        escaped.push('\\');
                    }
                    escaped.push(character);
                }
                format!("\"{escaped}\" <{}>", self.email)
            }
            None => self.email.clone(),
        }
    }
}

fn is_display_name_atom(word: &str) -> bool {
    !word.is_empty()
        && word
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-/=?^`{|}~_".contains(&byte))
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

    pub(crate) fn validate_headers(&self) -> EmailResult<()> {
        if contains_control(&self.subject) {
            return Err(EmailError::ConfigError(
                "email subject cannot contain control characters".into(),
            ));
        }
        let mut names = std::collections::HashSet::with_capacity(self.headers.len());
        for (name, value) in &self.headers {
            if name.is_empty()
                || !name
                    .bytes()
                    .all(|byte| byte.is_ascii_graphic() && byte != b':')
            {
                return Err(EmailError::ConfigError("invalid email header name".into()));
            }
            if contains_control(value) {
                return Err(EmailError::ConfigError(
                    "email header value cannot contain control characters".into(),
                ));
            }
            let name = name.to_ascii_lowercase();
            if matches!(
                name.as_str(),
                "from"
                    | "sender"
                    | "to"
                    | "cc"
                    | "bcc"
                    | "reply-to"
                    | "subject"
                    | "message-id"
                    | "mime-version"
                    | "content-type"
                    | "content-transfer-encoding"
            ) {
                return Err(EmailError::ConfigError(format!(
                    "custom email header conflicts with a generated header: {name}"
                )));
            }
            if !names.insert(name) {
                return Err(EmailError::ConfigError(
                    "duplicate email header name".into(),
                ));
            }
        }
        for address in std::iter::once(&self.from)
            .chain(&self.to)
            .chain(&self.reply_to)
            .chain(&self.cc)
            .chain(&self.bcc)
        {
            address.validate()?;
        }
        Ok(())
    }
}

fn contains_control(value: &str) -> bool {
    value.chars().any(char::is_control)
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
        let from = self.from.ok_or(EmailError::MissingField("from address"))?;

        if self.to.is_empty() {
            return Err(EmailError::MissingField("at least one recipient"));
        }

        let subject = self.subject.ok_or(EmailError::MissingField("subject"))?;

        if self.text_body.is_none() && self.html_body.is_none() {
            return Err(EmailError::MissingField("text_body or html_body"));
        }

        let email = Email {
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
        };
        email.validate_headers()?;
        Ok(email)
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
#[path = "tests/types_tests.rs"]
mod tests;
