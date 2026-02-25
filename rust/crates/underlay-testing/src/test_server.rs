//! Test HTTP server utilities
//!
//! Provides an in-memory HTTP server for testing Axum applications
//! without binding to actual network ports.
//!
//! # Example
//!
//! ```ignore
//! use underlay_testing::TestServer;
//! use axum::{Router, routing::get};
//!
//! async fn hello() -> &'static str {
//!     "Hello, World!"
//! }
//!
//! #[tokio::test]
//! async fn test_hello_endpoint() {
//!     let app = Router::new().route("/hello", get(hello));
//!     let server = TestServer::new(app);
//!     
//!     let response = server.get("/hello").send().await;
//!     
//!     assert_eq!(response.status(), 200);
//!     assert_eq!(response.text().await, "Hello, World!");
//! }
//! ```

use axum::body::Body;
use axum::Router;
use http::{Method, Request, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use tower::ServiceExt;

/// A test server that wraps an Axum router for in-memory testing.
///
/// This allows testing HTTP handlers without binding to network ports,
/// making tests faster and more isolated.
pub struct TestServer {
    router: Router,
}

impl TestServer {
    /// Create a new test server with the given router
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    /// Start building a GET request
    pub fn get(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::GET, path)
    }

    /// Start building a POST request
    pub fn post(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::POST, path)
    }

    /// Start building a PUT request
    pub fn put(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::PUT, path)
    }

    /// Start building a PATCH request
    pub fn patch(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::PATCH, path)
    }

    /// Start building a DELETE request
    pub fn delete(&self, path: &str) -> RequestBuilder {
        RequestBuilder::new(self.router.clone(), Method::DELETE, path)
    }
}

/// Builder for constructing test requests
pub struct RequestBuilder {
    router: Router,
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    fn new(router: Router, method: Method, path: &str) -> Self {
        Self {
            router,
            method,
            path: path.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Add a header to the request
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Add an Authorization header with a Bearer token
    pub fn bearer_token(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {token}"))
    }

    /// Authenticate as a specific user by ID.
    ///
    /// This sets a custom header that your auth middleware can use
    /// to identify the user in tests. You'll need to configure your
    /// test middleware to read from `X-Test-User-Id`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = server
    ///     .get("/api/profile")
    ///     .with_user("user-uuid-here")
    ///     .send()
    ///     .await;
    /// ```
    pub fn with_user(self, user_id: &str) -> Self {
        self.header("X-Test-User-Id", user_id)
    }

    /// Authenticate as an admin user.
    ///
    /// This sets headers indicating admin access for tests.
    /// Configure your test middleware to read from `X-Test-User-Id`
    /// and `X-Test-User-Role`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = server
    ///     .get("/api/admin/users")
    ///     .with_admin("admin-uuid-here")
    ///     .send()
    ///     .await;
    /// ```
    pub fn with_admin(self, user_id: &str) -> Self {
        self.header("X-Test-User-Id", user_id)
            .header("X-Test-User-Role", "admin")
    }

    /// Set a custom role for the authenticated user.
    ///
    /// Use with `with_user()` to test role-based access control.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let response = server
    ///     .get("/api/moderator/queue")
    ///     .with_user("user-uuid")
    ///     .with_role("moderator")
    ///     .send()
    ///     .await;
    /// ```
    pub fn with_role(self, role: &str) -> Self {
        self.header("X-Test-User-Role", role)
    }

    /// Add a JSON body to the request
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_vec(body).expect("Failed to serialize JSON body"));
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self
    }

    /// Add a raw body to the request
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    /// Send the request and get the response
    pub async fn send(self) -> TestResponse {
        let mut request_builder = Request::builder().method(self.method).uri(&self.path);

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key, value);
        }

        let body = match self.body {
            Some(bytes) => Body::from(bytes),
            None => Body::empty(),
        };

        let request = request_builder.body(body).expect("Failed to build request");

        let response = self
            .router
            .oneshot(request)
            .await
            .expect("Failed to send request");

        let status = response.status();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("Failed to read response body");

        TestResponse {
            status,
            headers,
            body: body_bytes.to_vec(),
        }
    }
}

/// A test response with helper methods for assertions
pub struct TestResponse {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl TestResponse {
    /// Get the HTTP status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Check if the status is successful (2xx)
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Get the response body as text
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    /// Get the response body as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Parse the response body as JSON
    pub fn json<T: DeserializeOwned>(&self) -> T {
        serde_json::from_slice(&self.body).expect("Failed to parse JSON response")
    }

    /// Try to parse the response body as JSON
    pub fn try_json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Get a header value
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(|s| s.as_str())
    }

    /// Check if a header exists
    pub fn has_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }

    /// Assert the status code equals the expected value
    ///
    /// # Panics
    ///
    /// Panics if the status code doesn't match, including the response body
    /// in the error message for debugging.
    pub fn assert_status(&self, expected: StatusCode) {
        if self.status != expected {
            panic!(
                "Expected status {}, got {}.\nBody: {}",
                expected,
                self.status,
                self.text()
            );
        }
    }

    /// Assert the response is successful (2xx)
    pub fn assert_success(&self) {
        if !self.is_success() {
            panic!(
                "Expected success status, got {}.\nBody: {}",
                self.status,
                self.text()
            );
        }
    }

    /// Assert the status is 200 OK
    pub fn assert_ok(&self) {
        self.assert_status(StatusCode::OK);
    }

    /// Assert the status is 201 Created
    pub fn assert_created(&self) {
        self.assert_status(StatusCode::CREATED);
    }

    /// Assert the status is 204 No Content
    pub fn assert_no_content(&self) {
        self.assert_status(StatusCode::NO_CONTENT);
    }

    /// Assert the status is 400 Bad Request
    pub fn assert_bad_request(&self) {
        self.assert_status(StatusCode::BAD_REQUEST);
    }

    /// Assert the status is 401 Unauthorized
    pub fn assert_unauthorized(&self) {
        self.assert_status(StatusCode::UNAUTHORIZED);
    }

    /// Assert the status is 403 Forbidden
    pub fn assert_forbidden(&self) {
        self.assert_status(StatusCode::FORBIDDEN);
    }

    /// Assert the status is 404 Not Found
    pub fn assert_not_found(&self) {
        self.assert_status(StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
#[path = "tests/test_server_tests.rs"]
mod tests;
