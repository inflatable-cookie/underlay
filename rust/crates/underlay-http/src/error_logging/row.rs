/// Database row returned from error_log queries.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ErrorLogRow {
    pub id: uuid::Uuid,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub error_code: String,
    pub message: String,
    pub correlation_id: String,
    pub context: serde_json::Value,
}
