//! Optional email templating convenience layer using Tera.
//!
//! This module is not the core email contract. The primary shared boundary is
//! still the adapter/message/manager seam in `underlay-email`. This helper is a
//! retained convenience when apps want shared HTML rendering.
//!
//! # Example
//!
//! ```rust,ignore
//! use underlay_email::EmailTemplateEngine;
//! use std::path::Path;
//!
//! let engine = EmailTemplateEngine::new(Path::new("templates/emails"))?;
//!
//! let mut context = EmailContext::default();
//! context.set("code", "123456");
//! context.set("expiry_minutes", 10);
//!
//! let html = engine.render("auth/email_totp.html", &context)?;
//! ```

use std::path::Path;

use crate::error::{EmailError, EmailResult};

/// Email template engine using Tera.
///
/// Loads templates from a directory at startup and provides methods for
/// rendering emails with dynamic context data.
pub struct EmailTemplateEngine {
    tera: tera::Tera,
}

impl EmailTemplateEngine {
    /// Create a new template engine loading templates from the given directory.
    ///
    /// The directory should contain HTML templates organized by category:
    /// ```text
    /// templates/
    /// ├── base.html
    /// ├── auth/
    /// │   ├── email_totp.html
    /// │   └── password_changed.html
    /// └── transactional/
    ///     └── welcome.html
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the template directory cannot be read or if
    /// templates contain syntax errors.
    pub fn new(template_dir: &Path) -> EmailResult<Self> {
        let glob_pattern = format!("{}/**/*.html", template_dir.display());

        let tera = tera::Tera::new(&glob_pattern).map_err(|e| {
            EmailError::TemplateError(format!(
                "Failed to load templates from {}: {}",
                template_dir.display(),
                e
            ))
        })?;

        Ok(Self { tera })
    }

    /// Create a template engine from a Tera instance.
    ///
    /// Useful for testing or when templates are embedded in the binary.
    pub fn from_tera(tera: tera::Tera) -> Self {
        Self { tera }
    }

    /// Render a template with the given context.
    ///
    /// # Arguments
    ///
    /// * `template_name` - The template path relative to the templates directory
    ///   (e.g., "auth/email_totp.html")
    /// * `context` - The context variables to pass to the template
    ///
    /// # Errors
    ///
    /// Returns an error if the template cannot be found or rendering fails.
    pub fn render(&self, template_name: &str, context: &EmailContext) -> EmailResult<String> {
        self.tera
            .render(template_name, &context.inner)
            .map_err(|e| {
                EmailError::TemplateError(format!("Failed to render {}: {}", template_name, e))
            })
    }

    /// Get the list of loaded template names.
    ///
    /// Useful for debugging and validation.
    pub fn template_names(&self) -> Vec<&str> {
        self.tera.get_template_names().collect()
    }

    /// Check if a template exists.
    pub fn has_template(&self, name: &str) -> bool {
        self.tera.get_template_names().any(|n| n == name)
    }
}

/// Context for email template rendering.
///
/// Wraps Tera's Context type with email-specific helper methods.
#[derive(Debug, Clone, Default)]
pub struct EmailContext {
    inner: tera::Context,
}

impl EmailContext {
    /// Create a new empty context.
    pub fn new() -> Self {
        Self {
            inner: tera::Context::new(),
        }
    }

    /// Create a context with common app-level variables pre-set.
    ///
    /// Sets the following variables:
    /// - `app_name` - The application name
    /// - `app_url` - The base URL of the application
    /// - `support_email` - Support email address
    /// - `current_year` - The current year (for copyright notices)
    pub fn with_app_info(app_name: &str, app_url: &str, support_email: &str) -> Self {
        let mut ctx = Self::new();
        ctx.set("app_name", app_name);
        ctx.set("app_url", app_url);
        ctx.set("support_email", support_email);
        ctx.set("current_year", chrono::Utc::now().year());
        ctx
    }

    /// Set a variable in the context.
    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) {
        self.inner.insert(key, &value);
    }

    /// Get the inner Tera context for advanced use cases.
    pub fn inner(&self) -> &tera::Context {
        &self.inner
    }

    /// Get a mutable reference to the inner Tera context.
    pub fn inner_mut(&mut self) -> &mut tera::Context {
        &mut self.inner
    }
}

impl From<tera::Context> for EmailContext {
    fn from(inner: tera::Context) -> Self {
        Self { inner }
    }
}

use chrono::Datelike;

#[cfg(test)]
#[path = "tests/templates_tests.rs"]
mod tests;
