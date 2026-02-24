//! Development-only blob storage endpoints for Axum.
//!
//! **WARNING**: These endpoints should NEVER be enabled in production.
//! They provide unauthenticated access to upload and download files.
//!
//! # Usage
//!
//! ```rust,ignore
//! use underlay_blob::{LocalAdapter, LocalConfig, dev_server};
//!
//! // Create the local adapter
//! let config = LocalConfig::new("./dev-uploads", "http://localhost:8080/dev-blobs");
//! let adapter = LocalAdapter::new(config).await?;
//!
//! // Create dev routes with default settings
//! let dev_routes = dev_server::routes(adapter);
//!
//! // Or customise with the builder
//! let dev_routes = dev_server::DevBlobRoutes::new(adapter)
//!     .route_path("/v1/dev-blobs")
//!     .max_upload_size(100 * 1024 * 1024)  // 100MB
//!     .with_cors()
//!     .build();
//!
//! // Merge into your app
//! let app = Router::new()
//!     .merge(dev_routes)
//!     // ... other routes
//! ```

use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::put,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::adapters::LocalAdapter;

/// Default maximum upload size (50 MB).
pub const DEFAULT_MAX_UPLOAD_SIZE: usize = 50 * 1024 * 1024;

/// Default route path for dev blob endpoints.
pub const DEFAULT_ROUTE_PATH: &str = "/dev-blobs";

/// State for dev blob routes.
#[derive(Clone)]
pub struct DevBlobState {
    /// The local adapter for file storage.
    pub adapter: Arc<LocalAdapter>,
}

/// Builder for configuring dev blob routes.
pub struct DevBlobRoutes {
    adapter: Arc<LocalAdapter>,
    route_path: String,
    max_upload_size: usize,
    enable_cors: bool,
}

impl DevBlobRoutes {
    /// Create a new dev blob routes builder.
    pub fn new(adapter: Arc<LocalAdapter>) -> Self {
        Self {
            adapter,
            route_path: DEFAULT_ROUTE_PATH.to_string(),
            max_upload_size: DEFAULT_MAX_UPLOAD_SIZE,
            enable_cors: false,
        }
    }

    /// Set the route path (e.g., "/v1/dev-blobs" or "/dev-uploads").
    ///
    /// The path should NOT include the `/*key` suffix - that's added automatically.
    pub fn route_path(mut self, path: impl Into<String>) -> Self {
        self.route_path = path.into();
        self
    }

    /// Set the maximum upload size in bytes.
    pub fn max_upload_size(mut self, size: usize) -> Self {
        self.max_upload_size = size;
        self
    }

    /// Enable permissive CORS for development.
    ///
    /// This allows any origin, method, and header - suitable for local development
    /// where the frontend may be on a different port.
    pub fn with_cors(mut self) -> Self {
        self.enable_cors = true;
        self
    }

    /// Build the router.
    pub fn build(self) -> Router<()> {
        let state = DevBlobState {
            adapter: self.adapter,
        };

        let route_pattern = format!("{}/{{*key}}", self.route_path.trim_end_matches('/'));

        let router = Router::new()
            .route(&route_pattern, put(upload).get(download))
            .layer(DefaultBodyLimit::max(self.max_upload_size))
            .with_state(state);

        if self.enable_cors {
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::PUT, Method::OPTIONS])
                .allow_headers(Any);

            tracing::info!(
                "Dev blob endpoints enabled at {}/* (CORS enabled, max {}MB)",
                self.route_path,
                self.max_upload_size / 1024 / 1024
            );

            router.layer(cors)
        } else {
            tracing::info!(
                "Dev blob endpoints enabled at {}/* (max {}MB)",
                self.route_path,
                self.max_upload_size / 1024 / 1024
            );

            router
        }
    }
}

/// Create dev blob routes with default settings.
///
/// This is a convenience function that creates routes at `/dev-blobs/*key`
/// with a 50MB upload limit and no CORS.
///
/// For more control, use [`DevBlobRoutes::new`].
pub fn routes(adapter: Arc<LocalAdapter>) -> Router<()> {
    DevBlobRoutes::new(adapter).build()
}

/// Upload a file (PUT /{path}/*key).
///
/// This accepts the raw file bytes and stores them using the LocalAdapter.
pub async fn upload(
    State(state): State<DevBlobState>,
    Path(key): Path<String>,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, String)> {
    let content_type = guess_content_type(&key);

    state
        .adapter
        .write_file(&key, &body, &content_type)
        .await
        .map_err(|e| {
            tracing::error!("Failed to write file {}: {}", key, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to write file: {}", e),
            )
        })?;

    tracing::debug!("Uploaded file: {} ({} bytes)", key, body.len());
    Ok(StatusCode::OK)
}

/// Download a file (GET /{path}/*key).
///
/// This serves files from the local filesystem for development.
pub async fn download(
    State(state): State<DevBlobState>,
    Path(key): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let data = state.adapter.read_file(&key).await.map_err(|e| {
        let status = if e.to_string().contains("NotFound") || e.to_string().contains("not found") {
            StatusCode::NOT_FOUND
        } else {
            tracing::error!("Failed to read file {}: {}", key, e);
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, format!("Failed to read file: {}", e))
    })?;

    let content_type = guess_content_type(&key);

    Ok((StatusCode::OK, [(header::CONTENT_TYPE, content_type)], data).into_response())
}

/// Guess the content type from a file extension.
pub fn guess_content_type(key: &str) -> String {
    let extension = key.rsplit('.').next().map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("avif") => "image/avif",
        Some("ico") => "image/x-icon",
        Some("pdf") => "application/pdf",
        Some("json") => "application/json",
        Some("txt") => "text/plain",
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("otf") => "font/otf",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("mp3") => "audio/mpeg",
        Some("ogg") => "audio/ogg",
        Some("wav") => "audio/wav",
        Some("zip") => "application/zip",
        Some("xml") => "application/xml",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_content_type() {
        assert_eq!(guess_content_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_content_type("photo.JPEG"), "image/jpeg");
        assert_eq!(guess_content_type("doc.pdf"), "application/pdf");
        assert_eq!(guess_content_type("video.mp4"), "video/mp4");
        assert_eq!(guess_content_type("font.woff2"), "font/woff2");
        assert_eq!(
            guess_content_type("unknown.xyz"),
            "application/octet-stream"
        );
        assert_eq!(
            guess_content_type("no-extension"),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_route_path_formatting() {
        // Build a valid adapter instance so the test does not rely on UB.
        let mut base = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        base.push(format!("underlay_blob_dev_server_test_{}", nanos));

        let runtime = tokio::runtime::Runtime::new().expect("runtime should build");
        let adapter = runtime
            .block_on(async {
                LocalAdapter::new(crate::adapters::LocalConfig::new(
                    &base,
                    "http://localhost/dev-blobs",
                ))
                .await
            })
            .expect("adapter should initialize");

        let builder = DevBlobRoutes::new(Arc::new(adapter));

        assert_eq!(builder.route_path, "/dev-blobs");

        let _ = std::fs::remove_dir_all(base);
    }
}
