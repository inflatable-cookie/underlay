use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::errors::CursorError;

/// A cursor for keyset pagination.
///
/// Cursors encode the sort column values and a tiebreaker ID to enable
/// efficient keyset pagination that works consistently at any depth.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cursor {
    /// The cursor values as a JSON map.
    #[serde(flatten)]
    values: HashMap<String, serde_json::Value>,
}

impl Cursor {
    /// Create a new empty cursor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an integer value to the cursor.
    pub fn with_int(mut self, key: impl Into<String>, value: i64) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::Number(value.into()));
        self
    }

    /// Add a string value to the cursor.
    pub fn with_string(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.into()));
        self
    }

    /// Add a UUID value to the cursor (stored as string).
    pub fn with_uuid(mut self, key: impl Into<String>, value: Uuid) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.to_string()));
        self
    }

    /// Add the tiebreaker ID (convenience for common pattern).
    pub fn with_id(self, id: Uuid) -> Self {
        self.with_uuid("id", id)
    }

    /// Add a weight value (convenience for common pattern).
    pub fn with_weight(self, weight: i32) -> Self {
        self.with_int("w", weight as i64)
    }

    /// Add a timestamp value (stored as ISO string).
    pub fn with_timestamp(
        mut self,
        key: impl Into<String>,
        value: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.values
            .insert(key.into(), serde_json::Value::String(value.to_rfc3339()));
        self
    }

    /// Get an integer value from the cursor.
    pub fn get_int(&self, key: &str) -> Result<i64, CursorError> {
        self.values
            .get(key)
            .ok_or_else(|| CursorError::MissingField(key.to_string()))?
            .as_i64()
            .ok_or_else(|| CursorError::InvalidType(format!("{} is not an integer", key)))
    }

    /// Get a string value from the cursor.
    pub fn get_string(&self, key: &str) -> Result<&str, CursorError> {
        self.values
            .get(key)
            .ok_or_else(|| CursorError::MissingField(key.to_string()))?
            .as_str()
            .ok_or_else(|| CursorError::InvalidType(format!("{} is not a string", key)))
    }

    /// Get a UUID value from the cursor.
    pub fn get_uuid(&self, key: &str) -> Result<Uuid, CursorError> {
        let s = self.get_string(key)?;
        Uuid::parse_str(s)
            .map_err(|e| CursorError::InvalidType(format!("{} is not a valid UUID: {}", key, e)))
    }

    /// Get the tiebreaker ID (convenience for common pattern).
    pub fn get_id(&self) -> Result<Uuid, CursorError> {
        self.get_uuid("id")
    }

    /// Get weight value (convenience for common pattern).
    pub fn get_weight(&self) -> Result<i32, CursorError> {
        self.get_int("w").map(|v| v as i32)
    }

    /// Get a timestamp value from the cursor.
    pub fn get_timestamp(&self, key: &str) -> Result<chrono::DateTime<chrono::Utc>, CursorError> {
        let s = self.get_string(key)?;
        chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .map_err(|e| {
                CursorError::InvalidType(format!("{} is not a valid timestamp: {}", key, e))
            })
    }

    /// Encode the cursor to a URL-safe string.
    pub fn encode(&self) -> String {
        let json = serde_json::to_string(&self.values).unwrap_or_default();
        URL_SAFE_NO_PAD.encode(json.as_bytes())
    }

    /// Decode a cursor from a URL-safe string.
    pub fn decode(encoded: &str) -> Result<Self, CursorError> {
        let bytes = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|e| CursorError::DecodeError(e.to_string()))?;

        let json = String::from_utf8(bytes).map_err(|e| CursorError::DecodeError(e.to_string()))?;

        let values: HashMap<String, serde_json::Value> =
            serde_json::from_str(&json).map_err(|e| CursorError::ParseError(e.to_string()))?;

        Ok(Self { values })
    }

    /// Convert the cursor into a typed value.
    pub fn into_value<T: DeserializeOwned>(self) -> Result<T, CursorError> {
        let json = serde_json::Value::Object(self.values.into_iter().collect());
        serde_json::from_value(json).map_err(|e| CursorError::ParseError(e.to_string()))
    }
}
