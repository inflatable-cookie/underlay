# 013 - Background Jobs and Scheduler Reliability Sweep

This sweep verifies reliability of Underlay-style background job systems: queue semantics, retries, scheduling correctness, and failure recovery.

## Problem this sweep targets

Common regressions:

- duplicate execution from overlap/race conditions
- jobs stuck in `claimed`/`running` with no recovery
- retries that loop forever or retry too aggressively
- scheduled tasks drifting from intended cadence
- weak idempotency causing repeated side effects after retries

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export JOBS_REPO="/path/to/myapp-jobs"   # if separate; otherwise same backend repo
```

Acowtancy mapping: `farmyard` API + `farmyard/crates/jobs` worker.

---

## Step 1 - Queue model and state machine sanity

### 1.1 Verify job table state model

```bash
rg -n "CREATE TABLE IF NOT EXISTS platform\.job|status IN \('pending', 'claimed', 'running', 'succeeded', 'failed', 'cancelled'\)|attempts|max_attempts|scheduled_for|claimed_at|heartbeat_at|error_history" "$API_REPO/migrations" -g "*.sql"
```

Pass criteria:

- state machine is explicit and finite
- retry and failure fields (`attempts`, `max_attempts`, `last_error`, `error_history`) exist
- claim/running liveness fields exist (`claimed_at`, `heartbeat_at`)

### 1.2 Verify indexes for claim/recovery paths

```bash
rg -n "idx_.*job_claimable|idx_.*job_running|status.*pending|heartbeat_at" "$API_REPO/migrations" -g "*.sql"
```

Pass criteria:

- claim and stalled-job queries are indexed

---

## Step 2 - Scheduler/task definition reliability

### 2.1 Verify scheduled task schema includes reliability knobs

```bash
rg -n "CREATE TABLE IF NOT EXISTS platform\.scheduled_task|allow_overlap|max_attempts|timeout_seconds|enabled|last_scheduled_at|last_completed_at" "$API_REPO/migrations" -g "*.sql"
```

### 2.2 Verify worker registers scheduled tasks deterministically

```bash
rg -n "ScheduledTaskDefinition|register_scheduled_tasks|schedule:\s*\"" "$API_REPO/crates/jobs/src" -g "*.rs"
```

Pass criteria:

- scheduled tasks are centrally registered
- each task has explicit schedule and retry policy
- overlap behavior is explicit (`allow_overlap` or equivalent policy)

---

## Step 3 - Claim, retry, and cancellation behavior

### 3.1 Verify admin/platform mutation semantics

```bash
rg -n "cancel_job|retry_job|trigger_scheduled_task|scheduled_task" "$API_REPO/crates/api/src/routes/admin/platform" -g "*.rs"
rg -n "status IN \('pending', 'claimed'\)|status = 'failed'|attempts = 0|claimed_at = NULL|heartbeat_at = NULL" "$API_REPO/crates/db/src/platform" -g "*.rs"
```

Pass criteria:

- cancel only affects valid pre-execution states
- retry resets runtime fields safely
- trigger path creates job with bounded attempts and defined priority/schedule

---

## Step 4 - Stalled job recovery and dead letter behavior

```bash
rg -n "recover_abandoned_jobs|heartbeat|stalled|archive_completed_jobs|purge_job_history|job_history" "$API_REPO/crates/jobs/src" "$API_REPO/crates/db/src" "$API_REPO/migrations" -g "*.rs" -g "*.sql"
```

Review for:

- periodic recovery task exists and is scheduled
- completed/failed job archival policy is explicit
- retention/purge strategy for history is explicit

Pass criteria:

- stuck jobs have automatic recovery path
- historical failures are queryable for diagnosis

---

## Step 5 - Idempotency of job handlers

For each side-effecting job type, verify idempotency strategy.

Search helpers:

```bash
rg -n "register\(tasks::|job_type:\s*\"" "$API_REPO/crates/jobs/src" -g "*.rs"
rg -n "ON CONFLICT|upsert|already exists|dedupe|idempot|unique" "$API_REPO/crates" -g "*.rs" -g "*.sql"
```

Manual review checklist per job:

- rerun after partial failure does not duplicate external effects
- write paths are guarded by unique keys/version checks where needed
- retries are safe even if previous attempt actually completed but acknowledgement failed

Pass criteria:

- all critical job handlers have documented idempotency behavior

---

## Step 6 - Notification vs polling fallback reliability

```bash
rg -n "pg_notify\('underlay_job_notify'|run_with_notifier|poll_interval|PgJobNotifier|OutboxNotifier" "$API_REPO/crates" "$API_REPO/migrations" -g "*.rs" -g "*.sql"
```

Pass criteria:

- LISTEN/NOTIFY path exists for low-latency wake-up
- polling fallback exists and is bounded
- worker continues if notifications degrade (or fails fast intentionally with clear signal)

---

## Step 7 - Runtime drills (recommended)

Run controlled drills in non-prod:

1. enqueue representative job and verify success path timing
2. inject deterministic failure and verify retry behavior respects `max_attempts`
3. simulate worker interruption and verify recovery of claimed/running jobs
4. manually trigger scheduled task and verify task + job metadata updates

Capture:

- mean enqueue-to-start latency
- retry intervals and final state
- time to recover abandoned jobs

---

## Step 8 - Operational SQL checks

Use quick health queries (adjust schema names if needed):

```sql
-- Pending backlog by type
SELECT job_type, COUNT(*)
FROM platform.job
WHERE status = 'pending'
GROUP BY job_type
ORDER BY COUNT(*) DESC;

-- Stale claimed/running jobs (example threshold: 10 minutes)
SELECT id, job_type, status, claimed_at, heartbeat_at, attempts, max_attempts
FROM platform.job
WHERE status IN ('claimed', 'running')
  AND COALESCE(heartbeat_at, claimed_at) < NOW() - INTERVAL '10 minutes'
ORDER BY COALESCE(heartbeat_at, claimed_at) ASC;

-- Repeated failures
SELECT job_type, COUNT(*) AS failed_count
FROM platform.job_history
WHERE status = 'failed'
GROUP BY job_type
ORDER BY failed_count DESC;
```

---

## Correction playbook

When findings are present:

1. tighten state transitions and guarded updates in queue actions
2. add or fix recovery tasks for stale claimed/running jobs
3. bound retries and improve backoff policies
4. make high-impact handlers idempotent (or explicitly deduplicated)
5. improve notification/poll fallback and operational alerts

---

## Severity rubric

- `critical`: duplicate or lost execution with data integrity/business impact
- `high`: stalled queue/recovery failure likely to cause operational incident
- `medium`: retry/scheduling inefficiency causing degraded reliability
- `low`: observability/hygiene gap with low direct impact
- `note`: hardening opportunity

---

## Findings template

```md
### [SEVERITY] Job reliability gap - <job_type/area>

- **Location:** `crates/...` and/or `migrations/...`
- **Current behavior:**
- **Expected reliability behavior:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Jobs/scheduler reliability sweep summary

- Job types audited: N
- Idempotency gaps: N
- Recovery gaps: N
- Retry policy gaps: N
- Scheduling/overlap gaps: N
```

---

## Related docs

- [012-observability-and-audit-sweep.md](./012-observability-and-audit-sweep.md)
- [007-error-diagnostics-and-logging-sweep.md](./007-error-diagnostics-and-logging-sweep.md)
- [120-configuration.md](../guides/120-configuration.md)
- [068-security.md](../guides/068-security.md)
