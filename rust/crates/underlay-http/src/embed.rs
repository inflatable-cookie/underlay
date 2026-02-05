//! Embed metadata proxy utilities.
//!
//! Provides server-side metadata lookup for embed providers that block
//! browser-side requests via CORS (e.g., Audioboom).
//!
//! # Example
//!
//! ```rust,ignore
//! use axum::{routing::post, Router};
//! use underlay_http::embed::{lookup_embed_metadata, EmbedMetaRequest};
//!
//! let app = Router::new()
//!     .route("/api/embed/metadata", post(lookup_embed_metadata));
//! ```

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

// ============================================================================
// Types
// ============================================================================

/// Request to look up embed metadata via server proxy.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedMetaRequest {
    /// Provider name: "audioboom", "youtube", "vimeo"
    pub provider: String,
    /// Media ID
    pub id: String,
    /// Embed type for providers with multiple formats
    /// (e.g., "single" or "playlist" for audioboom)
    pub embed_type: Option<String>,
}

/// Response from embed metadata lookup.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedMetaResponse {
    /// Whether lookup was successful
    pub success: bool,
    /// Media title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Media description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Thumbnail URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// Author/channel name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// Error message if lookup failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl EmbedMetaResponse {
    /// Create a successful response with metadata.
    pub fn success(
        title: Option<String>,
        description: Option<String>,
        duration: Option<i64>,
        thumbnail_url: Option<String>,
        author_name: Option<String>,
    ) -> Self {
        Self {
            success: true,
            title,
            description,
            duration,
            thumbnail_url,
            author_name,
            error: None,
        }
    }

    /// Create a failed response with an error message.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            title: None,
            description: None,
            duration: None,
            thumbnail_url: None,
            author_name: None,
            error: Some(message.into()),
        }
    }
}

// ============================================================================
// Provider Lookups
// ============================================================================

/// Look up metadata for any supported provider.
pub async fn lookup_metadata(req: &EmbedMetaRequest) -> EmbedMetaResponse {
    match req.provider.as_str() {
        "audioboom" => lookup_audioboom(&req.id, req.embed_type.as_deref()).await,
        _ => EmbedMetaResponse::error(format!("Unsupported provider: {}", req.provider)),
    }
}

/// Look up Audioboom metadata.
///
/// Uses the audio_clips API for single posts (includes duration) and
/// the playlists API for playlists (aggregates duration from clips).
pub async fn lookup_audioboom(id: &str, embed_type: Option<&str>) -> EmbedMetaResponse {
    let client = reqwest::Client::new();

    let result = match embed_type {
        Some("playlist") => lookup_audioboom_playlist(&client, id).await,
        _ => lookup_audioboom_single(&client, id).await,
    };

    match result {
        Ok(response) => response,
        Err(e) => EmbedMetaResponse::error(e),
    }
}

async fn lookup_audioboom_single(
    client: &reqwest::Client,
    id: &str,
) -> Result<EmbedMetaResponse, String> {
    // Use audio_clips API (has duration, unlike oEmbed)
    let url = format!("https://api.audioboom.com/audio_clips/{}", id);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let clip = &data["body"]["audio_clip"];

    // Duration is a float, convert to i64 (seconds)
    let duration = clip["duration"].as_f64().map(|d| d.round() as i64);

    Ok(EmbedMetaResponse::success(
        clip["title"].as_str().map(String::from),
        clip["description"].as_str().map(String::from),
        duration,
        clip["channel"]["urls"]["logo_image"]["original"]
            .as_str()
            .map(String::from),
        clip["channel"]["title"].as_str().map(String::from),
    ))
}

async fn lookup_audioboom_playlist(
    client: &reqwest::Client,
    id: &str,
) -> Result<EmbedMetaResponse, String> {
    let url = format!("https://api.audioboom.com/playlists/{}", id);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }

    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let playlist = &data["body"]["playlist"];

    // Aggregate duration from clips
    let mut total_duration: i64 = 0;
    if let Some(memberships) = playlist["memberships"].as_array() {
        for item in memberships {
            if let Some(dur) = item["audio_clip"]["duration"].as_f64() {
                total_duration += dur.round() as i64;
            }
        }
    }

    Ok(EmbedMetaResponse::success(
        playlist["title"].as_str().map(String::from),
        playlist["description"].as_str().map(String::from),
        if total_duration > 0 {
            Some(total_duration)
        } else {
            None
        },
        playlist["mosaic_image"]["original"]
            .as_str()
            .map(String::from),
        None,
    ))
}

// ============================================================================
// Axum Handler
// ============================================================================

/// Axum handler for embed metadata lookup.
///
/// This is a ready-to-use handler that can be mounted in your router.
/// For authenticated endpoints, wrap this with your auth middleware.
///
/// # Example
///
/// ```rust,ignore
/// use axum::{routing::post, Router};
/// use underlay_http::embed::lookup_embed_metadata;
///
/// let app = Router::new()
///     .route("/api/embed/metadata", post(lookup_embed_metadata));
/// ```
pub async fn lookup_embed_metadata(Json(req): Json<EmbedMetaRequest>) -> impl IntoResponse {
    let response = lookup_metadata(&req).await;
    (StatusCode::OK, Json(response))
}
