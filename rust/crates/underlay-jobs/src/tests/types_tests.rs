use super::*;
use chrono::Utc;
use serde_json::json;
use std::time::Duration;

#[test]
fn test_backoff_strategy() {
    let strategy = BackoffStrategy::Exponential {
        base: Duration::from_secs(60),
        max: Duration::from_secs(3600),
        jitter: None,
    };

    assert_eq!(strategy.delay_for_attempt(0), Duration::from_secs(60));
    assert_eq!(strategy.delay_for_attempt(1), Duration::from_secs(120));
    assert_eq!(strategy.delay_for_attempt(2), Duration::from_secs(240));
    assert_eq!(strategy.delay_for_attempt(6), Duration::from_secs(3600)); // Capped at max
}

#[test]
fn test_backoff_strategy_with_jitter_stays_bounded_and_spreads() {
    let strategy = BackoffStrategy::Exponential {
        base: Duration::from_millis(100),
        max: Duration::from_millis(1000),
        jitter: Some(BackoffJitter { max_percent: 30 }),
    };

    let delay_a = strategy.delay_for_attempt_with_seed(1, 11);
    let delay_b = strategy.delay_for_attempt_with_seed(1, 29);

    assert!(delay_a >= Duration::from_millis(200));
    assert!(delay_a <= Duration::from_millis(260));
    assert!(delay_b >= Duration::from_millis(200));
    assert!(delay_b <= Duration::from_millis(260));
    assert_ne!(delay_a, delay_b);
}

#[test]
fn test_job_progress() {
    let progress = JobProgress::new(50, 100).with_message("Processing...");
    assert_eq!(progress.percentage(), 50.0);
}

#[test]
fn test_job_status_terminal() {
    assert!(!JobStatus::Pending.is_terminal());
    assert!(!JobStatus::Running.is_terminal());
    assert!(JobStatus::Succeeded.is_terminal());
    assert!(JobStatus::Failed.is_terminal());
    assert!(JobStatus::Cancelled.is_terminal());
}

#[test]
fn job_type_has_required_fields() {
    let job = Job {
        id: underlay_core::Uuid::new_v7(),
        job_type: "test_job".to_string(),
        status: JobStatus::Pending,
        payload: json!({"key": "value"}),
        attempts: 1,
        max_attempts: 3,
        scheduled_for: None,
        priority: 0,
        claimed_at: None,
        claimed_by: None,
        started_at: None,
        finished_at: None,
        heartbeat_at: None,
        progress: None,
        last_error: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    assert!(!job.id.to_string().is_empty());
    assert_eq!(job.job_type, "test_job");
    assert_eq!(job.attempts, 1);
}

#[test]
fn job_handler_error_new() {
    let err = JobHandlerError::new("test message");
    assert_eq!(err.message, "test message");
    assert!(!err.is_permanent);
}

#[test]
fn job_handler_error_permanent() {
    let err = JobHandlerError::permanent("permanent error");
    assert_eq!(err.message, "permanent error");
    assert!(err.is_permanent);
}

#[test]
fn job_handler_error_display() {
    let err = JobHandlerError::new("error message");
    assert_eq!(err.to_string(), "error message");
}

#[test]
fn job_config_presets() {
    let maintenance = JobConfig::maintenance();
    assert_eq!(maintenance.max_attempts(), 1);

    let with_retries = JobConfig::with_retries(3);
    assert_eq!(with_retries.max_attempts(), 3);

    let with_jitter = JobConfig::with_retries_and_jitter(3);
    assert!(matches!(
        with_jitter.backoff(),
        BackoffStrategy::Exponential {
            jitter: Some(_),
            ..
        }
    ));

    let long_running = JobConfig::long_running();
    assert!(long_running.tracks_progress());
    assert_eq!(long_running.timeout_seconds(), Some(3600));
}
