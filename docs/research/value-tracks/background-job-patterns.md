# Value Track: Background Job Patterns

Status: Draft
Track: JOBS-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: jobs, queues, reliability, observability

## 1) Problem statement

Underlay provides `underlay-jobs` with PostgreSQL persistence, but production deployments need more than basic job execution:

- **Reliability**: Retry storms, poison pills, dead letter handling
- **Observability**: Queue depth, job duration, failure rates
- **Orchestration**: Job dependencies, multi-step workflows
- **Rate limiting**: Preventing downstream overload
- **Progress tracking**: Long-running job visibility

Research shows mature job systems (Sidekiq, BullMQ, Temporal) have solved these patterns differently.

## 2) Why this track matters

**For Underlay:**
- `underlay-jobs` is a core foundation crate
- Apps need reliable background processing
- Current implementation lacks advanced observability

**For consuming apps:**
- Every app needs retry/backoff strategies
- Debugging job failures is painful without visibility
- Complex workflows require orchestration

## 3) Cross-specimen comparison

| Aspect | Sidekiq | BullMQ | Temporal | Underlay Current |
|--------|---------|--------|----------|------------------|
| **Storage** | Redis | Redis | PostgreSQL/Cassandra | PostgreSQL |
| **Retry** | 25× exponential+jitter | Configurable backoff | Activity retry policies | 3 strategies |
| **Dead Letter** | Morgue | Failed queue | Workflow history | `Failed` status |
| **Observability** | Excellent Web UI | Bull Board + events | Query API | Limited |
| **Orchestration** | Batches (Pro) | Flows (DAGs) | Workflows + sagas | Not built-in |
| **Progress** | Not built-in | `updateProgress()` | Query state | `JobProgress` type |
| **Rate Limiting** | Pro feature | Built-in | Task queues | Not built-in |
| **Complexity** | Medium | Medium | High | Lower |

## 4) Repeated patterns

### Pattern 1: Exponential Backoff with Jitter

**Finding**: All mature systems use exponential backoff with jitter to prevent thundering herds.

**Sidekiq**:
```ruby
# delay = count**4 + 15 + (rand(30) * (count + 1))
# 25 retries over ~21 days
```

**BullMQ**:
```typescript
backoff: {
  type: 'exponential',
  delay: 1000 // Base delay
}
```

**Underlay**:
```rust
pub enum BackoffStrategy {
    None,
    Linear { base_secs: u64 },
    Exponential { base_secs: u64, max_secs: u64 },
}
```

**Gap**: Underlay's exponential backoff exists but lacks jitter.

### Pattern 2: Dead Letter Queue

**Finding**: Failed jobs must be retained for inspection, not just marked failed.

**Sidekiq**: Separate "morgue" queue, manual retry UI
**BullMQ**: Failed jobs in queue with retry capability
**Temporal**: Workflow history retained indefinitely
**Underlay**: `Failed` status, no built-in retry UI

**Gap**: No built-in dead letter management or retry UI.

### Pattern 3: Queue Observability

**Finding**: Essential metrics for production:
- Queue depth (waiting, active, scheduled, dead)
- Job duration (p50, p95, p99)
- Failure rate (per job type)
- Worker throughput

**Sidekiq**: Real-time Web UI with all metrics
**BullMQ**: Events + Bull Board UI
**Temporal**: Query API + Web UI
**Underlay**: App must implement

**Gap**: No built-in observability dashboard.

### Pattern 4: Job Orchestration

**Finding**: Complex workflows need job dependencies.

**Sidekiq**: Batches (Pro) - track group completion
**BullMQ**: Flows - full DAG with dependencies
**Temporal**: Child workflows + sagas
**Underlay**: Not built-in

**Example use case** (Acowtancy):
```
AI Content Generation:
1. Assemble context (job 1)
2. Call AI runtime (job 2, depends on 1)
3. Parse response (job 3, depends on 2)
4. Create suggestions (job 4, depends on 3)
```

**Current approach**: Manual state management in database.

### Pattern 5: Rate Limiting

**Finding**: Prevent job workers from overwhelming downstream services.

**Sidekiq Pro**: `sidekiq-throttled` gem
**BullMQ**: Built-in rate limiters
**Temporal**: Task queue rate limiting
**Underlay**: Not built-in

**Gap**: No built-in rate limiting.

## 5) Underlay Analysis

### Current State (`underlay-jobs`)

**Strengths**:
- PostgreSQL persistence (durable, transactional)
- Multiple backoff strategies
- Scheduled tasks (cron)
- Outbox pattern support
- Type-safe Rust API

**Gaps**:
1. **No jitter in backoff** - Risk of thundering herd
2. **Limited observability** - No built-in UI or metrics
3. **No job orchestration** - Manual dependency management
4. **No rate limiting** - Apps must implement throttling
5. **No dead letter management** - Failed jobs just sit in DB

### Real-world usage (Acme reference)

```rust
// Job handler with basic retry
#[async_trait]
impl JobHandler for GenerateThumbnailHandler {
    fn config(&self) -> JobConfig {
        JobConfig {
            max_attempts: 3,
            timeout_seconds: Some(60),
            ..Default::default()
        }
    }
}
```

**Finding**: Simple retry config, no progress reporting, no orchestration.

## 6) Project implications

### Recommended direction

**Phase 1: Reliability improvements** (high priority)

1. **Add jitter to exponential backoff**
   ```rust
   pub enum BackoffStrategy {
       Exponential { 
           base_secs: u64, 
           max_secs: u64,
           jitter: bool // NEW
       },
   }
   ```

2. **Dead letter queue management**
   - Separate table/index for failed jobs
   - Retry API (manual and automated)
   - Retention policies

**Phase 2: Observability** (medium priority)

3. **Metrics and events**
   - Job lifecycle events (enqueue, start, complete, fail)
   - Duration histograms
   - Queue depth gauges

4. **Optional admin UI**
   - Svelte component for job monitoring
   - Retry failed jobs
   - View job details and logs

**Phase 3: Orchestration** (lower priority, investigate)

5. **Job dependencies**
   - Parent-child relationships
   - DAG execution
   - Failure propagation

**Explicitly NOT recommended**:
- Full workflow engine (Temporal-style) - too complex
- Redis backend - PostgreSQL is intentional choice

### Risks to avoid

- **Over-engineering**: Most jobs are simple; don't add complexity
- **Feature bloat**: Keep core crate focused
- **Breaking changes**: Maintain backward compatibility

## 7) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Sidekiq retry docs | Official | High | Jitter formula |
| BullMQ flows | Official | High | Orchestration patterns |
| Temporal sagas | Official | High | Compensation patterns |
| Acme implementation | Production | High | Real usage patterns |

## 8) Decision state

- `continue research` → Validate jitter impact, prototype observability
- `promote to architecture work` → After Phase 1 validated

## Next Task

Create translation memo with specific recommendations:
1. Add jitter to backoff
2. Dead letter queue table design
3. Event system for observability
4. Admin UI component scope

## Related

- `specimen-dossiers/sidekiq.md` - Retry and reliability
- `specimen-dossiers/bullmq.md` - Observability and flows
- `specimen-dossiers/temporal.md` - Durable execution
- `source-hubs/background-job-patterns.md` - Source hierarchy
