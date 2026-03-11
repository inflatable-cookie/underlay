# Implementation Decision Record: Background Job Improvements

## Feature

Name: Job Reliability and Observability Enhancements
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Add jitter to retry backoff, dead letter queue management, and job lifecycle events to `underlay-jobs` for production-grade reliability and observability.

## Research Discovery

### Architecture Target

- Primary doc: `rust/crates/underlay-jobs/src/lib.rs`
- Related docs: `docs/guides/055-background-jobs.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/sidekiq.md` | Exponential backoff with jitter formula | Retry pattern |
| Specimen Dossier | `specimen-dossiers/bullmq.md` | Job events and progress tracking | Observability |
| Specimen Dossier | `specimen-dossiers/temporal.md` | Durable execution patterns | Advanced cases |
| Value Track | `value-tracks/background-job-patterns.md` | 5 repeated patterns across systems | Feature prioritization |
| Translation Memo | `translation-memos/background-job-improvements.md` | Specific implementation recommendations | Blueprint |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| Acowtancy job analysis | `complete` | Custom circuit breaker, manual orchestration | Confirms gaps |
| Acme job analysis | `complete` | Basic retry usage, no progress tracking | Confirms need |
| Sidekiq jitter research | `complete` | `count**4 + 15 + rand(30)` formula | Proven approach |

## Decisions

### Decision 1: Add Jitter to Exponential Backoff

**Decision:** Add optional jitter to `BackoffStrategy::Exponential` to prevent thundering herds.

**Research basis:**
- Sidekiq uses jitter: `rand(30) * (count + 1)`
- Thundering herd is real risk when many jobs fail simultaneously
- Zero-risk change (additive, backward compatible)

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| No jitter (status quo) | Thundering herd risk on mass failures |
| Always-on jitter | Breaking change to existing behavior |
| Fixed jitter amount | Random percentage is more effective |

**Confidence:** `high`

**Risks**
- None - additive feature with default-off for backward compatibility

**Implementation**

```rust
pub enum BackoffStrategy {
    // ... existing variants
    Exponential { 
        base_secs: u64, 
        max_secs: u64,
        jitter: bool, // NEW, default false for compat
    },
}

// Default new jobs get jitter
impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential {
            base_secs: DEFAULT_BACKOFF_BASE_SECS,
            max_secs: DEFAULT_BACKOFF_MAX_SECS,
            jitter: true, // Enable for new jobs
        }
    }
}
```

### Decision 2: Create Dead Letter Queue Table

**Decision:** Add separate `job_dead_letters` table for failed jobs with retry capability.

**Research basis:**
- Sidekiq has "morgue" for inspection and manual retry
- Currently failed jobs just sit in main table with `Failed` status
- Separate table enables better indexing and management

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Status in main table | Harder to query and manage large failure volumes |
| Delete failed jobs | Lose debugging capability |
| External system | Adds infrastructure complexity |

**Confidence:** `high`

**Risks**
- Migration required for existing failed jobs
- Retention policy needed to prevent unbounded growth

**Implementation**

```rust
// New repository trait
#[async_trait]
pub trait DeadLetterRepository {
    async fn archive(&self, job: &Job, error: &str) -> Result<DeadLetterId>;
    async fn list(&self, filters: DeadLetterFilters) -> Result<Vec<DeadLetter>>;
    async fn retry(&self, id: DeadLetterId) -> Result<JobId>;
    async fn archive_old(&self, before: DateTime<Utc>) -> Result<u64>;
}

// PostgreSQL implementation
pub struct PgDeadLetterRepository {
    pool: PgPool,
}
```

### Decision 3: Add Job Lifecycle Events

**Decision:** Add event system for job lifecycle (enqueue, start, complete, fail).

**Research basis:**
- BullMQ has rich event system for observability
- Apps need visibility into job processing
- Events enable metrics, logging, tracing

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Direct metrics integration | Too opinionated, limits flexibility |
| No events (status quo) | Poor observability |
| Tracing only | Events are more flexible for various use cases |

**Confidence:** `medium` (API design needs validation)

**Risks**
- Event handler performance could impact job processing
- Memory usage if handlers are slow

**Implementation**

```rust
pub enum JobEvent {
    Enqueued { job_id: JobId, job_type: String, scheduled_at: Option<DateTime<Utc>> },
    Claimed { job_id: JobId, worker_id: String },
    Started { job_id: JobId, attempt: u32 },
    Progress { job_id: JobId, percent: u8 },
    Completed { job_id: JobId, duration: Duration },
    Failed { job_id: JobId, error: String, will_retry: bool },
    DeadLettered { job_id: JobId, dead_letter_id: DeadLetterId },
}

pub trait JobEventHandler: Send + Sync {
    fn on_event(&self, event: JobEvent);
}
```

### Decision 4: Defer Admin UI (Phase 2)

**Decision:** Build admin UI components after core reliability features are stable.

**Research basis:**
- Sidekiq and Bull Board are valuable but secondary
- Core reliability (jitter, dead letters) is more urgent
- UI can be built on top of events API

**Confidence:** `high`

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Job orchestration/flows | Defer | Most jobs are simple; complex cases use database |
| Rate limiting | Defer | Can be added later, not blocking |
| Progress tracking | Include in events | Basic support via events, full UI later |

## Implementation Notes

### Key locations

- `rust/crates/underlay-jobs/src/types.rs` - BackoffStrategy update
- `rust/crates/underlay-jobs/src/postgres.rs` - Dead letter repository
- `rust/crates/underlay-jobs/src/runner.rs` - Event emission
- New: `rust/crates/underlay-jobs/src/events.rs` - Event types
- New: `rust/crates/underlay-jobs/src/dead_letter.rs` - DLQ repository

### Migration plan

1. Add jitter field (backward compatible)
2. Create dead letter table
3. Migrate existing failed jobs to dead letter table (optional)
4. Add event system
5. Update documentation

### Research references in code

```rust
// Research: translation-memos/background-job-improvements.md
// Based on: specimen-dossiers/sidekiq.md
// Decision: IDR-JOBS-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Event handler performance | Medium | Benchmark with no-op handler vs direct call |
| Dead letter retention policies | Low | Document recommendations, not enforce |

## Validation

- [ ] Jitter prevents thundering herd in test
- [ ] Dead letter table handles 10k+ records
- [ ] Events work with tracing integration
- [ ] Backward compatibility verified
- [ ] Documentation updated

## Related Documents

- Guide: `docs/guides/055-background-jobs.md`
- Translation Memo: `docs/research/translation-memos/background-job-improvements.md`
- Value Track: `docs/research/value-tracks/background-job-patterns.md`
- Dossier: `docs/research/specimen-dossiers/sidekiq.md`

## Next Task

Create implementation roadmap:
1. Add jitter to backoff (0.5 days)
2. Create dead letter table and repository (2 days)
3. Add job lifecycle events (2 days)
4. Migration and testing (1 day)
5. Documentation (0.5 days)

## Handoff Notes for Implementation Thread

**Priority:** Medium-High
**Estimated effort:** 6 days
**Dependencies:** None (extends existing crate)
**Breaking changes:** None (additive)
**Test strategy:** Test in Acme/Acowtancy with simulated failures

**Success criteria:**
- Thundering herd test passes (1000 simultaneous failing jobs)
- Dead letter retry works end-to-end
- Events fire correctly for all job states
