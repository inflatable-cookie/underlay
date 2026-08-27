//! Request context extraction for Axum handlers
//!
//! Provides a unified way to extract common request metadata like request ID,
//! client IP, user agent, and user identity from incoming requests.
//!
//! # Example
//!
//! ```ignore
//! use underlay_http::context::RequestContext;
//! use axum::Json;
//!
//! async fn my_handler(ctx: RequestContext) -> Json<String> {
//!     tracing::info!(
//!         request_id = %ctx.request_id(),
//!         ip = ?ctx.ip_address(),
//!         "Processing request"
//!     );
//!     Json("ok".to_string())
//! }
//! ```

pub mod headers;

mod error;
mod extractors;
mod model;
mod parse;
#[cfg(feature = "tracing")]
mod tracing;

#[cfg(test)]
use axum::http::HeaderMap;
pub use error::ContextError;
pub use model::{AuthenticatedContext, AuthenticatedUser, RequestContext};
#[cfg(test)]
use parse::{extract_request_id, resolve_ip_address};
pub use parse::{resolve_client_ip, TrustedProxyConfig};
#[cfg(feature = "tracing")]
pub use tracing::make_request_span;
#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
#[path = "../tests/context_tests/mod.rs"]
mod tests;
