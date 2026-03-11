use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::types::{DeadLetterId, JobId};

#[derive(Debug, Clone)]
pub enum JobEvent {
    Enqueued {
        job_id: JobId,
        job_type: String,
        scheduled_for: Option<DateTime<Utc>>,
    },
    Claimed {
        job_id: JobId,
        job_type: String,
        worker_id: Option<String>,
        attempt: i32,
    },
    Started {
        job_id: JobId,
        job_type: String,
        attempt: i32,
    },
    Completed {
        job_id: JobId,
        job_type: String,
        attempt: i32,
        duration: Option<Duration>,
    },
    Failed {
        job_id: JobId,
        job_type: String,
        error: String,
        attempt: i32,
        will_retry: bool,
        next_retry_delay: Option<Duration>,
    },
    DeadLettered {
        job_id: JobId,
        job_type: String,
        dead_letter_id: DeadLetterId,
    },
}

pub trait JobEventSink: Send + Sync {
    fn on_event(&self, event: JobEvent);
}

#[derive(Clone, Default)]
pub struct JobEventHub {
    sinks: Vec<Arc<dyn JobEventSink>>,
}

impl fmt::Debug for JobEventHub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JobEventHub")
            .field("sink_count", &self.sinks.len())
            .finish()
    }
}

impl JobEventHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sink(mut self, sink: Arc<dyn JobEventSink>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn emit(&self, event: JobEvent) {
        for sink in &self.sinks {
            sink.on_event(event.clone());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sinks.is_empty()
    }
}
