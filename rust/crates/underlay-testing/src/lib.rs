//! Testing utilities for Underlay applications
//!
//! This crate provides reusable test infrastructure to reduce boilerplate
//! in test files. It offers:
//!
//! - **TestDb** - Isolated PostgreSQL databases using testcontainers
//! - **TestServer** - HTTP test server with request helpers
//! - **Fixtures** - Common test data builders and helpers
//!
//! # Features
//!
//! - `db` - Database testing with testcontainers (requires Docker)
//! - `server` - HTTP/API testing with in-memory server
//! - `full` - All features enabled
//!
//! # Example
//!
//! ```ignore
//! use underlay_testing::{TestDb, TestServer};
//!
//! #[tokio::test]
//! async fn test_list_users() {
//!     let db = TestDb::new().await;
//!     db.load_fixture("users.sql").await;
//!     
//!     let server = TestServer::new(app(db.pool())).await;
//!     let response = server.get("/v1/users").send().await;
//!     
//!     assert_eq!(response.status(), 200);
//!
//!     // Required when UNDERLAY_TEST_DATABASE_URL is set.
//!     db.cleanup().await.unwrap();
//! }
//! ```

mod fixtures;

#[cfg(feature = "db")]
mod test_db;

#[cfg(feature = "server")]
mod test_server;

// Re-exports
pub use fixtures::{AuthFixtures, Fixtures, TimestampFixtures};

#[cfg(feature = "db")]
pub use test_db::TestDb;

#[cfg(feature = "server")]
pub use test_server::{TestResponse, TestServer};
