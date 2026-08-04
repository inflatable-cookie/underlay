//! Canonical cached-list endpoint: the microcache + singleflight +
//! weak-ETag serving flow every admin list handler used to hand-write.
//!
//! One `CachedListEndpoint` per list surface; `serve` handles the hit
//! path (304 on matching `If-None-Match`, cached JSON otherwise), the
//! stampede guard (singleflight rebuild), and the miss path.

use std::future::Future;
use std::time::Duration;

use axum::http::HeaderMap;
use axum::response::Response;

use crate::caching::{if_none_match_matches, MicroCache, SingleFlight};

/// Serialized list response held in the microcache.
#[derive(Debug, Clone)]
pub struct CachedListResponse {
    pub etag: String,
    pub body: Vec<u8>,
}

/// A cached admin list endpoint: TTL microcache + singleflight +
/// weak-ETag conditional responses.
pub struct CachedListEndpoint {
    cache: MicroCache<CachedListResponse>,
    flight: SingleFlight<()>,
}

impl CachedListEndpoint {
    pub fn new(ttl: Duration, capacity: usize) -> Self {
        Self {
            cache: MicroCache::new(ttl, capacity),
            flight: SingleFlight::new(),
        }
    }

    /// Standard 2s/256 shape used by admin list surfaces.
    pub fn admin_default() -> Self {
        Self::new(Duration::from_secs(2), 256)
    }

    pub fn invalidate(&self, key: &str) {
        self.cache.invalidate(key);
    }

    pub fn invalidate_prefix(&self, prefix: &str) {
        self.cache.invalidate_prefix(prefix);
    }

    /// Serve a list response through the canonical cache flow.
    ///
    /// - `key`: cache key for this query variant.
    /// - `headers`: request headers (for `If-None-Match`).
    /// - `build`: producer of the serialized response on cache miss. Called
    ///   again inline if the stampede guard lost a concurrent rebuild race.
    /// - `on_hit`: called on every cache hit (metrics).
    /// - `render_json(cached)`: full JSON response for a cached body.
    /// - `render_not_modified(etag)`: 304 response for a matching ETag.
    /// - `render_error(err)`: response when `build` fails.
    #[allow(clippy::too_many_arguments)]
    pub async fn serve<E, F, B, H, J, N, X>(
        &self,
        key: &str,
        headers: &HeaderMap,
        build: B,
        on_hit: H,
        render_json: J,
        render_not_modified: N,
        render_error: X,
    ) -> Response
    where
        F: Future<Output = Result<CachedListResponse, E>>,
        B: Fn() -> F,
        H: Fn(),
        J: Fn(Vec<u8>, &str) -> Response,
        N: Fn(&str) -> Response,
        X: Fn(E) -> Response,
    {
        if let Some(cached) = self.cache.get(key) {
            on_hit();
            if if_none_match_matches(headers, &cached.etag) {
                return render_not_modified(&cached.etag);
            }
            return render_json(cached.body, &cached.etag);
        }

        // Stampede guard: concurrent misses for the same key rebuild once.
        self.flight
            .run(key.to_string(), || async {
                if self.cache.get(key).is_some() {
                    return;
                }
                if let Ok(cached) = build().await {
                    self.cache.insert(key, cached);
                }
            })
            .await;

        if let Some(cached) = self.cache.get(key) {
            on_hit();
            if if_none_match_matches(headers, &cached.etag) {
                return render_not_modified(&cached.etag);
            }
            return render_json(cached.body, &cached.etag);
        }

        match build().await {
            Ok(cached) => {
                let etag = cached.etag.clone();
                let body = cached.body.clone();
                self.cache.insert(key, cached);
                if if_none_match_matches(headers, &etag) {
                    return render_not_modified(&etag);
                }
                render_json(body, &etag)
            }
            Err(err) => render_error(err),
        }
    }
}
