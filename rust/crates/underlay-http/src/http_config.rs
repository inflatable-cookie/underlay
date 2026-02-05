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
//! println!("Binding to {}:{}", config.bind_addr, config.port);
//! println!("Public URL: http://{}:{}", config.public_host, config.port);
//! ```

use std::env;

use underlay_observability::Environment;

/// HTTP server configuration.
///
/// Separates the socket bind address (which must be an IP) from the public
/// hostname used in URLs (which can be "localhost" or a domain name).
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    /// Address to bind the server socket to (e.g., "127.0.0.1", "0.0.0.0").
    /// Must be a valid IP address.
    pub bind_addr: String,

    /// Port to listen on.
    pub port: u16,

    /// Public hostname for constructing URLs (e.g., "localhost", "api.example.com").
    ///
    /// Used for things like:
    /// - Blob storage URLs that need to be accessible from browsers
    /// - OAuth callback URLs
    /// - Email links
    ///
    /// Defaults to "localhost" for local/dev/test environments.
    pub public_host: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = HttpServerConfig::default();
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.public_host, "localhost");
    }

    #[test]
    fn test_socket_addr() {
        let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
        assert_eq!(config.socket_addr(), "0.0.0.0:8080");
    }

    #[test]
    fn test_base_url() {
        let config = HttpServerConfig::new("0.0.0.0", 8080, "api.example.com");
        assert_eq!(config.http_base_url(), "http://api.example.com:8080");
        assert_eq!(config.https_base_url(), "https://api.example.com:8080");
    }

    #[test]
    fn test_local_defaults() {
        // Clear env vars to test defaults
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("PUBLIC_HOST");

        let config = HttpServerConfig::from_env(Environment::Local);
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 3000);
        // public_host defaults to bind_addr
        assert_eq!(config.public_host, "127.0.0.1");
    }
}
