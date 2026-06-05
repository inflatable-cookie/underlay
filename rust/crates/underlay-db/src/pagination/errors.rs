/// Error type for cursor operations.
#[derive(Debug, Clone)]
pub enum CursorError {
    /// Base64 decoding failed.
    DecodeError(String),
    /// JSON parsing failed.
    ParseError(String),
    /// Missing required field in cursor.
    MissingField(String),
    /// Invalid field type in cursor.
    InvalidType(String),
}

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorError::DecodeError(msg) => write!(f, "Cursor decode error: {}", msg),
            CursorError::ParseError(msg) => write!(f, "Cursor parse error: {}", msg),
            CursorError::MissingField(field) => write!(f, "Cursor missing field: {}", field),
            CursorError::InvalidType(msg) => write!(f, "Cursor invalid type: {}", msg),
        }
    }
}

impl std::error::Error for CursorError {}
