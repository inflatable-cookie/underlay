//! HTTP server configuration utilities.
//!
//! Provides standardized configuration for HTTP servers with sensible defaults
//! for different environments.
//!
//! # Example
//!
//! ```rust
//! use underlay_http::HttpServerConfig;
//! use underlay_observability::Environment;
//!
//! // Load from environment with defaults
//! let config = HttpServerConfig::from_env(Environment::Local);
//!
//! // Use in server setup
//! println!("Binding to {}", config.socket_addr());
//! println!("Public URL: {}", config.http_base_url());
//! ```

use std::env;
use std::net::IpAddr;

use thiserror::Error;

use underlay_observability::Environment;

#[derive(Debug, Error)]
pub enum HttpServerConfigError {
    #[error("invalid bind address `{value}`: expected an IP address")]
    InvalidBindAddr { value: String },
    #[error("invalid port `{value}`: expected a number between 0 and 65535")]
    InvalidPort { value: String },
    #[error("invalid public host `{value}`: {reason}")]
    InvalidPublicHost { value: String, reason: &'static str },
}

/// HTTP server configuration.
///
/// Separates the socket bind address (which must be an IP) from the public
/// hostname used in URLs (which can be "localhost" or a domain name).
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    /// Address to bind the server socket to (e.g., "127.0.0.1", "0.0.0.0").
    /// Must be a valid IP address.
    bind_addr: String,

    /// Port to listen on.
    port: u16,

    /// Public hostname for constructing URLs (e.g., "localhost", "api.example.com").
    ///
    /// Used for things like:
    /// - Blob storage URLs that need to be accessible from browsers
    /// - OAuth callback URLs
    /// - Email links
    ///
    /// Defaults to "localhost" for local/dev/test environments.
    public_host: String,
}

impl HttpServerConfig {
    /// Create a new HTTP server config.
    pub fn new(bind_addr: impl Into<String>, port: u16, public_host: impl Into<String>) -> Self {
        Self {
            bind_addr: bind_addr.into(),
            port,
            public_host: public_host.into(),
        }
    }

    /// Create a new validated HTTP server config.
    pub fn try_new(
        bind_addr: impl Into<String>,
        port: u16,
        public_host: impl Into<String>,
    ) -> Result<Self, HttpServerConfigError> {
        let bind_addr = bind_addr.into();
        validate_bind_addr(&bind_addr)?;

        let public_host = public_host.into();
        validate_public_host(&public_host)?;

        Ok(Self {
            bind_addr,
            port,
            public_host,
        })
    }

    /// Load configuration from environment variables with sensible defaults.
    ///
    /// # Environment Variables
    ///
    /// - `HOST` - Bind address (default: "127.0.0.1" for local, "0.0.0.0" otherwise)
    /// - `PORT` - Port number (default: 3000)
    /// - `PUBLIC_HOST` - Public hostname for URLs (default: "localhost" for local/dev/test)
    ///
    /// # Arguments
    ///
    /// * `env` - The current environment, used for determining defaults
    pub fn from_env(env: Environment) -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|raw| raw.parse::<u16>().ok())
            .unwrap_or(3000);

        let bind_addr = env::var("HOST").unwrap_or_else(|_| {
            // In local/test without explicit PORT, bind only to localhost
            // Otherwise bind publicly (needed for containers, proxies, etc.)
            let should_bind_publicly =
                !matches!(env, Environment::Local | Environment::Test) || env::var("PORT").is_ok();

            if should_bind_publicly {
                "0.0.0.0".to_string()
            } else {
                "127.0.0.1".to_string()
            }
        });

        // Public host defaults to HOST value, which itself defaults appropriately
        // For local dev, set HOST=localhost in .env for browser-compatible URLs
        let public_host = env::var("PUBLIC_HOST").unwrap_or_else(|_| bind_addr.clone());

        Self {
            bind_addr,
            port,
            public_host,
        }
    }

    /// Load and validate configuration from environment variables.
    pub fn try_from_env(env: Environment) -> Result<Self, HttpServerConfigError> {
        let port = match env::var("PORT") {
            Ok(raw) => raw
                .parse::<u16>()
                .map_err(|_| HttpServerConfigError::InvalidPort { value: raw })?,
            Err(_) => 3000,
        };

        let bind_addr = env::var("HOST").unwrap_or_else(|_| {
            let should_bind_publicly =
                !matches!(env, Environment::Local | Environment::Test) || env::var("PORT").is_ok();

            if should_bind_publicly {
                "0.0.0.0".to_string()
            } else {
                "127.0.0.1".to_string()
            }
        });

        let public_host = env::var("PUBLIC_HOST").unwrap_or_else(|_| bind_addr.clone());

        Self::try_new(bind_addr, port, public_host)
    }

    /// Get the socket address string for binding (e.g., "127.0.0.1:3000").
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.bind_addr, self.port)
    }

    /// Get the base URL using the public host (e.g., "http://localhost:3000").
    ///
    /// Note: Does not include a trailing slash.
    pub fn base_url(&self, scheme: &str) -> String {
        format!("{}://{}:{}", scheme, self.public_host, self.port)
    }

    /// Get the HTTP base URL (http://...).
    pub fn http_base_url(&self) -> String {
        self.base_url("http")
    }

    /// Get the HTTPS base URL (https://...).
    pub fn https_base_url(&self) -> String {
        self.base_url("https")
    }

    pub fn bind_addr(&self) -> &str {
        &self.bind_addr
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn public_host(&self) -> &str {
        &self.public_host
    }
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1".to_string(),
            port: 3000,
            public_host: "localhost".to_string(),
        }
    }
}

fn validate_bind_addr(value: &str) -> Result<(), HttpServerConfigError> {
    value
        .parse::<IpAddr>()
        .map(|_| ())
        .map_err(|_| HttpServerConfigError::InvalidBindAddr {
            value: value.to_string(),
        })
}

fn validate_public_host(value: &str) -> Result<(), HttpServerConfigError> {
    if value.is_empty() {
        return Err(HttpServerConfigError::InvalidPublicHost {
            value: value.to_string(),
            reason: "host must not be empty",
        });
    }

    if value.contains("://") {
        return Err(HttpServerConfigError::InvalidPublicHost {
            value: value.to_string(),
            reason: "host must not include a URL scheme",
        });
    }

    if value
        .chars()
        .any(|ch| ch.is_control() || ch.is_whitespace() || matches!(ch, '/' | '?' | '#' | '@'))
    {
        return Err(HttpServerConfigError::InvalidPublicHost {
            value: value.to_string(),
            reason: "host must not include path, query, fragment, userinfo, or whitespace",
        });
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/http_config_tests.rs"]
mod tests;
