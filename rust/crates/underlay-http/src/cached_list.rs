//! Canonical cached-list endpoint: the microcache + singleflight +
//! weak-ETag serving flow every admin list handler used to hand-write.
//!
//! One `CachedListEndpoint` per list surface; `serve` handles the hit
//! path (304 on matching `If-None-Match`, cached JSON otherwise), the
//! stampede guard (singleflight rebuild), and the miss path.

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use axum::body::{Body, Bytes};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;

use crate::caching::{if_none_match_matches, MicroCache, SingleFlight};

/// Serialized list response held in the microcache.
#[derive(Debug, Clone)]
pub struct CachedListResponse {
    pub etag: String,
    pub body: Vec<u8>,
}

/// Coalesced error response shared across concurrent single-flight callers so
/// a failing `build()` is never re-executed by waiting requests.
#[derive(Debug, Clone)]
struct CoalescedErrorResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl CoalescedErrorResponse {
    async fn from_response(response: Response) -> Self {
        let (parts, body) = response.into_parts();
        let body = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();
        Self {
            status: parts.status,
            headers: parts.headers,
            body,
        }
    }

    fn into_response(self) -> Response {
        let mut response = Response::new(Body::from(self.body));
        *response.status_mut() = self.status;
        *response.headers_mut() = self.headers;
        response
    }
}

/// A cached admin list endpoint: TTL microcache + singleflight +
/// weak-ETag conditional responses.
pub struct CachedListEndpoint {
    cache: MicroCache<CachedListResponse>,
    flight: SingleFlight<Result<CachedListResponse, CoalescedErrorResponse>>,
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
    /// - `build`: producer of the serialized response on cache miss. Runs once
    ///   per concurrent stampede; both successes and errors are coalesced across
    ///   waiting callers.
    /// - `on_hit`: called on cache hits (including coalesced waiters), not on
    ///   the leader's cache miss.
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

        // Stampede guard: concurrent misses for the same key rebuild once and
        // share either the cached payload or the rendered error response.
        let built_fresh = AtomicBool::new(false);
        let outcome = self
            .flight
            .run(key.to_string(), || async {
                if let Some(cached) = self.cache.get(key) {
                    return Ok(cached);
                }
                built_fresh.store(true, Ordering::Relaxed);
                match build().await {
                    Ok(cached) => {
                        self.cache.insert(key, cached.clone());
                        Ok(cached)
                    }
                    Err(err) => Err(CoalescedErrorResponse::from_response(render_error(err)).await),
                }
            })
            .await;

        match outcome {
            Ok(cached) => {
                if !built_fresh.load(Ordering::Relaxed) {
                    on_hit();
                }
                if if_none_match_matches(headers, &cached.etag) {
                    return render_not_modified(&cached.etag);
                }
                render_json(cached.body, &cached.etag)
            }
            Err(err_response) => err_response.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use std::time::Duration;

    use axum::body::Body;
    use axum::http::{HeaderMap, HeaderValue, StatusCode};
    use axum::response::Response;

    use super::{CachedListEndpoint, CachedListResponse};

    fn render_json(body: Vec<u8>, etag: &str) -> Response {
        let mut resp = Response::new(Body::from(body));
        *resp.status_mut() = StatusCode::OK;
        resp.headers_mut().insert(
            axum::http::header::ETAG,
            HeaderValue::from_str(etag).expect("valid etag"),
        );
        resp
    }

    fn render_not_modified(etag: &str) -> Response {
        let mut resp = Response::new(Body::empty());
        *resp.status_mut() = StatusCode::NOT_MODIFIED;
        resp.headers_mut().insert(
            axum::http::header::ETAG,
            HeaderValue::from_str(etag).expect("valid etag"),
        );
        resp
    }

    fn render_error(err: &'static str) -> Response {
        let mut resp = Response::new(Body::from(err));
        *resp.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
        resp
    }

    #[tokio::test]
    async fn leader_cache_miss_does_not_fire_on_hit_and_subsequent_hit_does() {
        let endpoint = CachedListEndpoint::new(Duration::from_secs(5), 16);
        let headers = HeaderMap::new();
        let builds = Arc::new(AtomicUsize::new(0));
        let hits = Arc::new(AtomicUsize::new(0));

        let resp1 = endpoint
            .serve(
                "list:page=1",
                &headers,
                || {
                    let builds = Arc::clone(&builds);
                    async move {
                        builds.fetch_add(1, Ordering::SeqCst);
                        Ok::<_, &'static str>(CachedListResponse {
                            etag: "W/\"v1\"".to_string(),
                            body: b"[1,2,3]".to_vec(),
                        })
                    }
                },
                || {
                    hits.fetch_add(1, Ordering::SeqCst);
                },
                render_json,
                render_not_modified,
                render_error,
            )
            .await;

        assert_eq!(resp1.status(), StatusCode::OK);
        assert_eq!(builds.load(Ordering::SeqCst), 1);
        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "leader cache miss must not increment hit metrics"
        );

        let resp2 = endpoint
            .serve(
                "list:page=1",
                &headers,
                || {
                    let builds = Arc::clone(&builds);
                    async move {
                        builds.fetch_add(1, Ordering::SeqCst);
                        Ok::<_, &'static str>(CachedListResponse {
                            etag: "W/\"v2\"".to_string(),
                            body: b"[4,5,6]".to_vec(),
                        })
                    }
                },
                || {
                    hits.fetch_add(1, Ordering::SeqCst);
                },
                render_json,
                render_not_modified,
                render_error,
            )
            .await;

        assert_eq!(resp2.status(), StatusCode::OK);
        assert_eq!(builds.load(Ordering::SeqCst), 1);
        assert_eq!(
            hits.load(Ordering::SeqCst),
            1,
            "subsequent cached request must increment hit metrics"
        );
    }

    #[tokio::test]
    async fn concurrent_error_coalesces_without_reexecuting_build() {
        let endpoint = Arc::new(CachedListEndpoint::new(Duration::from_secs(5), 16));
        let builds = Arc::new(AtomicUsize::new(0));
        let hits = Arc::new(AtomicUsize::new(0));

        let mut tasks = Vec::new();
        for _ in 0..8 {
            let endpoint = Arc::clone(&endpoint);
            let builds = Arc::clone(&builds);
            let hits = Arc::clone(&hits);
            tasks.push(tokio::spawn(async move {
                let headers = HeaderMap::new();
                let resp = endpoint
                    .serve(
                        "list:failing",
                        &headers,
                        || {
                            let builds = Arc::clone(&builds);
                            async move {
                                builds.fetch_add(1, Ordering::SeqCst);
                                tokio::time::sleep(Duration::from_millis(25)).await;
                                Err::<CachedListResponse, _>("upstream down")
                            }
                        },
                        || {
                            hits.fetch_add(1, Ordering::SeqCst);
                        },
                        render_json,
                        render_not_modified,
                        render_error,
                    )
                    .await;
                let status = resp.status();
                let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
                    .await
                    .expect("read error body");
                (status, body)
            }));
        }

        for task in tasks {
            let (status, body) = task.await.expect("task should join");
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(&body[..], b"upstream down");
        }

        assert_eq!(
            builds.load(Ordering::SeqCst),
            1,
            "concurrent callers must coalesce on error without re-running build()"
        );
        assert_eq!(
            hits.load(Ordering::SeqCst),
            0,
            "error responses must never fire on_hit()"
        );
    }
}
