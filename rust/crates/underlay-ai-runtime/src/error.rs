use serde::{Deserialize, Serialize};

use crate::{default_fallback_error_kinds, default_retriable_error_kinds};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AiErrorKind {
    Auth,
    RateLimit,
    Timeout,
    Provider,
    CircuitOpen,
    Validation,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRuntimeError {
    pub kind: AiErrorKind,
    pub message: String,
}

impl AiRuntimeError {
    pub fn new(kind: AiErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn is_retriable(&self) -> bool {
        default_retriable_error_kinds().contains(&self.kind)
    }

    pub fn allows_fallback(&self) -> bool {
        default_fallback_error_kinds().contains(&self.kind)
    }
}
