# Translation Memo: Background Job Improvements

Status: Draft
Memo: JOBS-TM-001
Owner:
Last updated: 2026-03-11
Related track: `value-tracks/background-job-patterns.md`

## 1) Project problem statement

`underlay-jobs` provides basic job execution with PostgreSQL persistence, but lacks production-grade reliability and observability features found in mature job systems:

**Current gaps**:
1. **No jitter in backoff** - Risk of thundering herd on retries
2. **Limited dead letter handling** - Failed jobs just sit in DB
3. **No built-in observability** - Apps must build their own dashboards
4. **No job orchestration** - Complex workflows require manual state management
5. **No rate limiting** - Risk of overwhelming downstream services

**Evidence from Acowtancy**: AI action jobs manually manage:
- Circuit breaker (separate from job system)
- State transitions between steps
- Progress tracking via database updates
- Custom retry logic

## 2) External evidence summary

### Sidekiq (Ruby)
- **Retry**: 25 attempts over ~21 days with exponential backoff + jitter
- **Dead letter**: "Morgue" for failed jobs with manual retry UI
- **Formula**: `delay = count**4 + 15 + (rand(30) * (count + 1))`

### BullMQ (Node.js)
- **Backoff**: Configurable with optional jitter
- **Observability**: Rich events, Bull Board UI
- **Flows**: Job dependencies and DAGs
- **Progress**: `job.updateProgress(42)`

### Temporal
- **Durable execution**: Survives process crashes
- **Saga**: Compensation patterns for multi-step workflows
- **Query**: Inspect workflow state dynamically

## 3) Recommendation

### Phase 1: Reliability (High Priority)

#### 1.1 Add Jitter to Exponential Backoff

```rust
// rust/crates/underlay-jobs/src/types.rs
pub enum BackoffStrategy {
    None,
    Linear { base_secs: u64 },
    Exponential { 
        base_secs: u64, 
        max_secs: u64,
        jitter: bool, // NEW
    },
}

impl BackoffStrategy {
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        match self {
            Self::Exponential { base_secs, max_secs, jitter } => {
                let base = (*base_secs as u64)
                    .saturating_mul(2u64.saturating_pow(attempt));
                let delay = base.min(*max_secs);
                
                if *jitter {
                    // Add 0-30% random jitter
                    let jitter_factor = rand::random::<f64>() * 0.3;
                    Duration::from_secs((delay as f64 * (1.0 + jitter_factor)) as u64)
                } else {
                    Duration::from_secs(delay)
                }
            }
            // ...
        }
    }
}
```

**Default**: Enable jitter for new jobs, maintain backward compatibility.

#### 1.2 Dead Letter Queue Table

```sql
-- New table for dead letters
CREATE TABLE job_dead_letters (
    id UUID PRIMARY KEY,
    original_job_id UUID NOT NULL,
    job_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    error_message TEXT NOT NULL,
    error_details JSONB,
    failed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    attempts INTEGER NOT NULL,
    retried_at TIMESTAMPTZ,
    retried_job_id UUID,
    archived_at TIMESTAMPTZ  -- For cleanup
);

CREATE INDEX idx_dead_letters_job_type ON job_dead_letters(job_type);
CREATE INDEX idx_dead_letters_failed_at ON job_dead_letters(failed_at);
```

**API**:
```rust
pub trait DeadLetterRepository {
    async fn archive_failed_job(&self, job: &Job, error: &str) -> Result<DeadLetterId>;
    async fn list_dead_letters(&self, filters: DeadLetterFilters) -> Result<Vec<DeadLetter>>;
    async fn retry_dead_letter(&self, id: DeadLetterId) -> Result<JobId>;
    async fn archive_old_dead_letters(&self, before: DateTime<Utc>) -> Result<u64>;
}
```

### Phase 2: Observability (Medium Priority)

#### 2.1 Job Lifecycle Events

```rust
pub enum JobEvent {
    Enqueued { job_id: JobId, job_type: String },
    Claimed { job_id: JobId, worker_id: String },
    Started { job_id: JobId },
    Progress { job_id: JobId, percent: u8 },
    Completed { job_id: JobId, duration_ms: u64 },
    Failed { job_id: JobId, error: String, will_retry: bool },
    DeadLettered { job_id: JobId, dead_letter_id: DeadLetterId },
}

pub trait JobEventHandler: Send + Sync {
    fn on_event(&self, event: JobEvent);
}
```

#### 2.2 Metrics Traits

```rust
pub trait JobMetrics {
    fn record_job_enqueued(&self, job_type: &str);
    fn record_job_completed(&self, job_type: &str, duration: Duration);
    fn record_job_failed(&self, job_type: &str, error_kind: &str);
    fn gauge_queue_depth(&self, status: JobStatus, count: u64);
}
```

### Phase 3: Admin UI (Lower Priority)

#### 3.1 Svelte Components

```svelte
<!-- ts/src/components/JobDashboard.svelte -->
<JobDashboard 
  {jobStats}
  recentJobs={recentJobs}
  deadLetters={deadLetters}
  onRetry={(jobId) => ...}
  onArchive={(jobId) => ...}
/>
```

**Scope**: Read-only monitoring first, actions (retry, cancel) later.

## 4) Tradeoffs the project would accept

| Tradeoff | Rationale |
|----------|-----------|
| **PostgreSQL vs Redis** | Keep PostgreSQL for durability; accept slightly lower throughput |
| **No full workflow engine** | Temporal is overkill for most use cases; keep jobs simple |
| **Manual orchestration for complex cases** | Most jobs are simple; complex cases can use database state |

## 5) What must be true before adoption

- [ ] Jitter prevents thundering herd in production
- [ ] Dead letter table handles high volume
- [ ] Events don't impact job processing performance
- [ ] Backward compatibility maintained

## 6) Required prototype or validation work

**Prototype P-JOBS-001**: Jitter and Dead Letters

1. Add jitter to backoff in test environment
2. Simulate 1000 failing jobs simultaneously
3. Measure retry distribution
4. Create dead letter table and API
5. Test retry from dead letter

## 7) Promotion target

- `roadmap planning` → Add to G01 roadmap if prototype validates

## 8) Sources

| Source | Confidence | Notes |
| --- | --- | --- |
| Sidekiq retry docs | High | Jitter formula proven |
| Acowtancy implementation | High | Real orchestration pain |
| BullMQ events | High | Observability patterns |

## Next Task

Create IDR for Phase 1 implementation:
1. Add jitter to exponential backoff
2. Create dead letter table and repository
3. Add job lifecycle events

## Related

- `value-tracks/background-job-patterns.md` - Full analysis
- `specimen-dossiers/sidekiq.md` - Retry patterns
- `specimen-dossiers/bullmq.md` - Observability
