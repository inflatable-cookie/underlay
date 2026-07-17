use super::super::{AiErrorKind, AiRuntimeError};
use underlay_core::ErrorCode;

#[test]
fn ai_runtime_error_composes_with_std_error_traits() {
    // Regression for g08.015: AiRuntimeError previously implemented neither
    // Display nor std::error::Error, so it could not be used with `?`,
    // `anyhow`, or `Box<dyn Error>`.
    fn fallible() -> Result<(), Box<dyn std::error::Error>> {
        Err(AiRuntimeError::new(AiErrorKind::Timeout, "upstream timed out"))?;
        Ok(())
    }

    let err = fallible().unwrap_err();
    assert!(err.to_string().contains("upstream timed out"));
    // Display carries the kind too.
    assert!(err.to_string().contains("Timeout"));
}

#[test]
fn ai_runtime_error_exposes_stable_code() {
    let err = AiRuntimeError::new(AiErrorKind::RateLimit, "slow down");
    assert_eq!(ErrorCode::code(&err), "ai.runtime.rate_limit");
    assert_eq!(
        ErrorCode::code(&AiRuntimeError::new(AiErrorKind::Provider, "x")),
        "ai.runtime.provider"
    );
}
