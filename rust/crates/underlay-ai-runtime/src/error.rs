use serde::{Deserialize, Serialize};
use underlay_core::ErrorCode;

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

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[error("{kind:?}: {message}")]
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

impl ErrorCode for AiRuntimeError {
    fn code(&self) -> &str {
        match self.kind {
            AiErrorKind::Auth => "ai.runtime.auth",
            AiErrorKind::RateLimit => "ai.runtime.rate_limit",
            AiErrorKind::Timeout => "ai.runtime.timeout",
            AiErrorKind::Provider => "ai.runtime.provider",
            AiErrorKind::CircuitOpen => "ai.runtime.circuit_open",
            AiErrorKind::Validation => "ai.runtime.validation",
            AiErrorKind::Unknown => "ai.runtime.unknown",
        }
    }
}
