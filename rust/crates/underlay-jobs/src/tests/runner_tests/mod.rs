use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;

use crate::types::{
    Job, JobConfig, JobFailureOutcome, JobHandler, JobHandlerError, JobId, JobStatus,
};
use crate::{JobEvent, JobEventSink, JobRegistry, JobRunner, JobRunnerConfig, JobStore};

mod batch;
mod dispatch;
mod failures;

#[derive(Debug, Default)]
struct MemStore {
    queue: Mutex<Vec<Job>>,
    successes: Mutex<Vec<JobId>>,
    failures: Mutex<Vec<JobId>>,
    failure_calls: Mutex<Vec<FailureCall>>,
    dead_letters: Mutex<Vec<JobId>>,
}

#[derive(Debug, Clone)]
struct FailureCall {
    job_id: JobId,
    is_permanent: bool,
    max_attempts: u32,
    retry_delay_secs: u64,
}

#[derive(Debug)]
struct MemError;

impl std::fmt::Display for MemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mem error")
    }
}

impl std::error::Error for MemError {}

#[async_trait]
impl JobStore for Arc<MemStore> {
    type Error = MemError;

    async fn fetch_next(&self, allowed_types: &[String]) -> Result<Option<Job>, Self::Error> {
        let mut q = self.queue.lock().unwrap();
        let idx = q
            .iter()
            .position(|j| allowed_types.iter().any(|t| t == &j.job_type));
        Ok(idx.map(|i| q.remove(i)))
    }

    async fn mark_success(&self, job_id: JobId) -> Result<(), Self::Error> {
        self.successes.lock().unwrap().push(job_id);
        Ok(())
    }

    async fn mark_failure(
        &self,
        job: &Job,
        error: JobHandlerError,
        config: &crate::types::JobConfig,
    ) -> Result<JobFailureOutcome, Self::Error> {
        self.failures.lock().unwrap().push(job.id);
        let will_retry = !error.is_permanent && (job.attempts + 1) < config.max_attempts() as i32;
        let dead_letter_id = if will_retry {
            None
        } else {
            let dead_letter_id = underlay_core::Uuid::new_v7();
            self.dead_letters.lock().unwrap().push(dead_letter_id);
            Some(dead_letter_id)
        };
        self.failure_calls.lock().unwrap().push(FailureCall {
            job_id: job.id,
            is_permanent: error.is_permanent,
            max_attempts: config.max_attempts(),
            retry_delay_secs: config.backoff().delay_for_attempt(0).as_secs(),
        });
        Ok(JobFailureOutcome {
            will_retry,
            retry_delay: will_retry.then(|| config.backoff().delay_for_attempt(0)),
            dead_letter_id,
        })
    }
}

#[derive(Debug, Default)]
struct RecordingEventSink {
    events: Mutex<Vec<JobEvent>>,
}

impl RecordingEventSink {
    fn events(&self) -> Vec<JobEvent> {
        self.events.lock().unwrap().clone()
    }
}

impl JobEventSink for RecordingEventSink {
    fn on_event(&self, event: JobEvent) {
        self.events.lock().unwrap().push(event);
    }
}

fn make_test_job(job_type: &str) -> Job {
    let now = Utc::now();
    Job {
        id: underlay_core::Uuid::new_v7(),
        job_type: job_type.to_string(),
        status: JobStatus::Pending,
        payload: json!({"ok": true}),
        attempts: 0,
        max_attempts: 1,
        scheduled_for: None,
        priority: 0,
        claimed_at: None,
        claimed_by: None,
        started_at: None,
        finished_at: None,
        heartbeat_at: None,
        progress: None,
        last_error: None,
        created_at: now,
        updated_at: now,
    }
}

#[derive(Debug)]
struct TestHandler;

#[async_trait]
impl JobHandler for TestHandler {
    fn job_type(&self) -> &'static str {
        "test"
    }

    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        Ok(())
    }
}
