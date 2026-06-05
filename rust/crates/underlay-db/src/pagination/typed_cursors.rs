use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::cursor::Cursor;

/// Cursor for weight-based pagination (common pattern).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightCursor {
    /// Weight value.
    #[serde(rename = "w")]
    pub weight: i32,
    /// Tiebreaker ID.
    pub id: Uuid,
}

impl WeightCursor {
    /// Create a new weight cursor.
    pub fn new(weight: i32, id: Uuid) -> Self {
        Self { weight, id }
    }

    /// Encode to cursor string.
    pub fn encode(&self) -> String {
        Cursor::new()
            .with_weight(self.weight)
            .with_id(self.id)
            .encode()
    }
}

/// Cursor for timestamp-based pagination (common pattern).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampCursor {
    /// Timestamp value.
    #[serde(rename = "t")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Tiebreaker ID.
    pub id: Uuid,
}

impl TimestampCursor {
    /// Create a new timestamp cursor.
    pub fn new(timestamp: chrono::DateTime<chrono::Utc>, id: Uuid) -> Self {
        Self { timestamp, id }
    }

    /// Encode to cursor string.
    pub fn encode(&self) -> String {
        Cursor::new()
            .with_timestamp("t", self.timestamp)
            .with_id(self.id)
            .encode()
    }
}
