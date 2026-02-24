//! Shared HTTP client for Underlay crates.
//!
//! Provides a pre-configured [`reqwest::Client`] with sensible defaults
//! (rustls TLS, JSON support, user-agent) so that consumers don't each
//! configure their own.
//!
//! # Example
//!
//! ```rust
//! use underlay_http_client::HttpClient;
//!
//! let client = HttpClient::new();
//! let inner: &reqwest::Client = client.inner();
//! ```

pub use reqwest;

/// A pre-configured HTTP client wrapping [`reqwest::Client`].
#[derive(Debug, Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    /// Create a new HTTP client with default settings.
    pub fn new() -> Self {
        let inner = reqwest::Client::builder()
            .user_agent(concat!("underlay/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");

        Self { inner }
    }

    /// Create a new HTTP client with a custom user-agent.
    pub fn with_user_agent(user_agent: impl Into<String>) -> Self {
        let inner = reqwest::Client::builder()
            .user_agent(user_agent.into())
            .build()
            .expect("failed to build HTTP client");

        Self { inner }
    }

    /// Get a reference to the underlying [`reqwest::Client`].
    pub fn inner(&self) -> &reqwest::Client {
        &self.inner
    }

    /// Consume the wrapper and return the underlying [`reqwest::Client`].
    pub fn into_inner(self) -> reqwest::Client {
        self.inner
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for HttpClient {
    type Target = reqwest::Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::HttpClient;

    #[test]
    fn default_client_builds_requests() {
        let client = HttpClient::new();
        client
            .inner()
            .get("https://example.com")
            .build()
            .expect("request should build");
    }

    #[test]
    fn custom_user_agent_client_builds_requests() {
        let client = HttpClient::with_user_agent("test-agent/1.0");
        client
            .inner()
            .get("https://example.com")
            .build()
            .expect("request should build");
    }

    #[test]
    fn default_matches_new() {
        HttpClient::default()
            .inner()
            .get("https://example.com")
            .build()
            .expect("request should build");
        HttpClient::new()
            .inner()
            .get("https://example.com")
            .build()
            .expect("request should build");
    }

    #[test]
    fn deref_and_inner_reference_same_client() {
        let client = HttpClient::with_user_agent("deref-test/1.0");
        assert!(std::ptr::eq(client.inner(), &*client));
    }

    #[test]
    fn into_inner_exposes_reqwest_client() {
        let client = HttpClient::with_user_agent("deref-test/1.0");

        client
            .get("https://example.com")
            .build()
            .expect("request should build via deref");

        let inner = client.into_inner();
        inner
            .get("https://example.com")
            .build()
            .expect("request should build");
    }
}
