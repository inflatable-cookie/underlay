use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, Instant},
};

use axum::http::{HeaderMap, HeaderValue};
use sha2::{Digest, Sha256};

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
        let mut guard = self.entries.lock().expect("microcache mutex poisoned");

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
        let mut guard = self.entries.lock().expect("microcache mutex poisoned");

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
        let mut guard = self.entries.lock().expect("microcache mutex poisoned");
        guard.remove(key);
    }

    pub fn invalidate_prefix(&self, prefix: &str) {
        let mut guard = self.entries.lock().expect("microcache mutex poisoned");
        guard.retain(|k, _| !k.starts_with(prefix));
    }

    pub fn clear(&self) {
        let mut guard = self.entries.lock().expect("microcache mutex poisoned");
        guard.clear();
    }
}

/// Builds a weak ETag from raw payload bytes.
pub fn weak_etag_for_bytes(payload: &[u8]) -> String {
    let digest = Sha256::digest(payload);
    // Truncate for compact header values; still stable enough for validators.
    let short = &digest[..16];
    format!("W/\"{}\"", hex::encode(short))
}

/// Matches `If-None-Match` header values against a current ETag.
pub fn if_none_match_matches(headers: &HeaderMap, current_etag: &str) -> bool {
    let Some(raw) = headers.get(axum::http::header::IF_NONE_MATCH) else {
        return false;
    };

    let Ok(raw_str) = raw.to_str() else {
        return false;
    };

    raw_str
        .split(',')
        .map(|part| part.trim())
        .any(|candidate| candidate == "*" || candidate == current_etag)
}

pub fn etag_header_value(etag: &str) -> Option<HeaderValue> {
    HeaderValue::from_str(etag).ok()
}
