use std::{
    collections::HashMap,
    future::Future,
    sync::{Mutex, MutexGuard},
    time::{Duration, Instant},
};

use axum::http::{HeaderMap, HeaderValue};
use sha2::{Digest, Sha256};
use tokio::sync::oneshot;

pub const CACHE_CONTROL_ADMIN_REVALIDATE: &str = "private, no-cache, must-revalidate";
pub const CACHE_CONTROL_NO_STORE: &str = "no-store";

#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

/// Small in-process TTL cache for hot read paths.
///
/// This is intentionally simple and should be used as an opt-in microcache,
/// not as a general distributed cache replacement.
pub struct MicroCache<V> {
    ttl: Duration,
    max_entries: usize,
    entries: Mutex<HashMap<String, CacheEntry<V>>>,
}

impl<V: Clone> MicroCache<V> {
    pub fn new(ttl: Duration, max_entries: usize) -> Self {
        Self {
            ttl,
            max_entries: max_entries.max(1),
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<V> {
        let now = Instant::now();
        let mut guard = self.lock_entries();

        match guard.get(key) {
            Some(entry) if entry.expires_at > now => Some(entry.value.clone()),
            Some(_) => {
                guard.remove(key);
                None
            }
            None => None,
        }
    }

    pub fn insert(&self, key: impl Into<String>, value: V) {
        let now = Instant::now();
        let mut guard = self.lock_entries();

        // Opportunistic cleanup before insertion.
        guard.retain(|_, entry| entry.expires_at > now);

        if guard.len() >= self.max_entries {
            // Simple bounded behavior: evict an arbitrary key.
            if let Some(first_key) = guard.keys().next().cloned() {
                guard.remove(&first_key);
            }
        }

        guard.insert(
            key.into(),
            CacheEntry {
                value,
                expires_at: now + self.ttl,
            },
        );
    }

    pub fn invalidate(&self, key: &str) {
        let mut guard = self.lock_entries();
        guard.remove(key);
    }

    pub fn invalidate_prefix(&self, prefix: &str) {
        let mut guard = self.lock_entries();
        guard.retain(|k, _| !k.starts_with(prefix));
    }

    pub fn clear(&self) {
        let mut guard = self.lock_entries();
        guard.clear();
    }

    fn lock_entries(&self) -> MutexGuard<'_, HashMap<String, CacheEntry<V>>> {
        self.entries
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    #[cfg(test)]
    pub(crate) fn test_poison_entries_lock(&self) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = self
                .entries
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            panic!("poison microcache lock");
        }));
    }
}

/// In-process keyed single-flight coordinator for async read paths.
///
/// For a given key, only one caller executes the loader; concurrent callers
/// await the same result from the leader.
pub struct SingleFlight<V> {
    inflight: Mutex<HashMap<String, Vec<oneshot::Sender<V>>>>,
}

struct InflightGuard<'a, V> {
    inflight: &'a Mutex<HashMap<String, Vec<oneshot::Sender<V>>>>,
    key: String,
    active: bool,
}

impl<V> Drop for InflightGuard<'_, V> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut guard = self
            .inflight
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        // Dropping the removed Vec<oneshot::Sender<V>> wakes any waiters with
        // RecvError so they can recompute instead of hanging indefinitely.
        guard.remove(&self.key);
    }
}

impl<V: Clone> Default for SingleFlight<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone> SingleFlight<V> {
    pub fn new() -> Self {
        Self {
            inflight: Mutex::new(HashMap::new()),
        }
    }

    fn lock_inflight(&self) -> MutexGuard<'_, HashMap<String, Vec<oneshot::Sender<V>>>> {
        self.inflight
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub async fn run<F, Fut>(&self, key: impl Into<String>, loader: F) -> V
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = V>,
    {
        let key = key.into();
        let mut loader = Some(loader);

        loop {
            let waiter = {
                let mut guard = self.lock_inflight();
                if let Some(waiters) = guard.get_mut(&key) {
                    let (tx, rx) = oneshot::channel();
                    waiters.push(tx);
                    Some(rx)
                } else {
                    guard.insert(key.clone(), Vec::new());
                    None
                }
            };

            if let Some(rx) = waiter {
                if let Ok(value) = rx.await {
                    return value;
                }
                // Leader was cancelled or panicked; re-enter the loop so
                // exactly one woken waiter becomes the new leader and the rest
                // re-attach instead of thundering-herding or clobbering state.
                continue;
            }

            let mut cleanup = InflightGuard {
                inflight: &self.inflight,
                key: key.clone(),
                active: true,
            };
            let value = loader
                .take()
                .expect("singleflight loader should only be consumed once")(
            )
            .await;
            cleanup.active = false;
            let waiters = {
                let mut guard = self.lock_inflight();
                guard.remove(&key).unwrap_or_default()
            };
            for tx in waiters {
                let _ = tx.send(value.clone());
            }
            return value;
        }
    }

    #[cfg(test)]
    pub(crate) async fn test_insert_inflight_key(&self, key: &str) {
        let mut guard = self.lock_inflight();
        guard.insert(key.to_string(), Vec::new());
    }

    #[cfg(test)]
    pub(crate) async fn test_remove_inflight_key(&self, key: &str) {
        let mut guard = self.lock_inflight();
        guard.remove(key);
    }
}

fn strip_weak_prefix(etag: &str) -> &str {
    etag.strip_prefix("W/").unwrap_or(etag)
}

fn is_weak_etag(etag: &str) -> bool {
    etag.starts_with("W/")
}

/// Builds a weak ETag from raw payload bytes.
pub fn weak_etag_for_bytes(payload: &[u8]) -> String {
    let digest = Sha256::digest(payload);
    // Truncate for compact header values; still stable enough for validators.
    let short = &digest[..16];
    format!("W/\"{}\"", hex::encode(short))
}

/// Matches `If-None-Match` header values against a current ETag using RFC 9110
/// weak comparison.
pub fn if_none_match_matches(headers: &HeaderMap, current_etag: &str) -> bool {
    let current_opaque = strip_weak_prefix(current_etag.trim());

    headers
        .get_all(axum::http::header::IF_NONE_MATCH)
        .iter()
        .filter_map(|raw| raw.to_str().ok())
        .flat_map(|raw_str| raw_str.split(','))
        .map(str::trim)
        .any(|candidate| candidate == "*" || strip_weak_prefix(candidate) == current_opaque)
}

/// Matches `If-Match` header values against a current ETag using RFC 9110
/// strong comparison (weak ETags never match).
pub fn if_match_matches(headers: &HeaderMap, current_etag: &str) -> bool {
    let current = current_etag.trim();

    headers
        .get_all(axum::http::header::IF_MATCH)
        .iter()
        .filter_map(|raw| raw.to_str().ok())
        .flat_map(|raw_str| raw_str.split(','))
        .map(str::trim)
        .any(|candidate| {
            if candidate == "*" {
                return true;
            }
            !is_weak_etag(candidate) && !is_weak_etag(current) && candidate == current
        })
}

pub fn etag_header_value(etag: &str) -> Option<HeaderValue> {
    HeaderValue::from_str(etag).ok()
}
