# Contract: Jobs Events and Operator Systems

Status: active
Owner: repo maintainers
Depends on: `040-storage-blob-and-media-systems.md`

## Purpose

Define the shared async and operator-facing infrastructure contract Underlay
owns across background jobs, scheduled tasks, domain events, audit logs, email,
rate limiting, and security alerts.

This contract covers:

- the background job queue, runner, store, scheduler, dead-letter, and job
  event model
- the outbox/domain-event seam
- audit entry, writer, and query helpers
- email adapter, manager, message, and optional template-engine surfaces
- pluggable rate-limiting backends
- shared security-alert signal evaluation and persistence seams

It does not define app-local admin routes, staff workflows, notification
policy, or operator UX. Those build on this layer and belong elsewhere.

## Sources of Truth

Primary:

- [`rust/crates/underlay-jobs/src/lib.rs`](../../rust/crates/underlay-jobs/src/lib.rs)
- [`rust/crates/underlay-jobs/src/types.rs`](../../rust/crates/underlay-jobs/src/types.rs)
- [`rust/crates/underlay-jobs/src/store.rs`](../../rust/crates/underlay-jobs/src/store.rs)
- [`rust/crates/underlay-jobs/src/runner.rs`](../../rust/crates/underlay-jobs/src/runner.rs)
- [`rust/crates/underlay-jobs/src/registry.rs`](../../rust/crates/underlay-jobs/src/registry.rs)
- [`rust/crates/underlay-jobs/src/scheduler.rs`](../../rust/crates/underlay-jobs/src/scheduler.rs)
- [`rust/crates/underlay-jobs/src/dead_letters.rs`](../../rust/crates/underlay-jobs/src/dead_letters.rs)
- [`rust/crates/underlay-jobs/src/events.rs`](../../rust/crates/underlay-jobs/src/events.rs)
- [`rust/crates/underlay-jobs-postgres/src/lib.rs`](../../rust/crates/underlay-jobs-postgres/src/lib.rs)
- [`rust/crates/underlay-jobs-postgres/src/postgres.rs`](../../rust/crates/underlay-jobs-postgres/src/postgres.rs)
- [`rust/crates/underlay-jobs-postgres/src/postgres_scheduled.rs`](../../rust/crates/underlay-jobs-postgres/src/postgres_scheduled.rs)
- [`rust/crates/underlay-jobs-postgres/src/outbox.rs`](../../rust/crates/underlay-jobs-postgres/src/outbox.rs)
- [`rust/crates/underlay-events/src/lib.rs`](../../rust/crates/underlay-events/src/lib.rs)
- [`rust/crates/underlay-audit/src/lib.rs`](../../rust/crates/underlay-audit/src/lib.rs)
- [`rust/crates/underlay-audit/src/entry.rs`](../../rust/crates/underlay-audit/src/entry.rs)
- [`rust/crates/underlay-audit/src/writer.rs`](../../rust/crates/underlay-audit/src/writer.rs)
- [`rust/crates/underlay-audit/src/query.rs`](../../rust/crates/underlay-audit/src/query.rs)
- [`rust/crates/underlay-email/src/lib.rs`](../../rust/crates/underlay-email/src/lib.rs)
- [`rust/crates/underlay-email/src/adapter.rs`](../../rust/crates/underlay-email/src/adapter.rs)
- [`rust/crates/underlay-email/src/manager.rs`](../../rust/crates/underlay-email/src/manager.rs)
- [`rust/crates/underlay-email/src/types.rs`](../../rust/crates/underlay-email/src/types.rs)
- [`rust/crates/underlay-email/src/templates.rs`](../../rust/crates/underlay-email/src/templates.rs)
- [`rust/crates/underlay-ratelimit/src/lib.rs`](../../rust/crates/underlay-ratelimit/src/lib.rs)
- [`rust/crates/underlay-ratelimit/src/backend.rs`](../../rust/crates/underlay-ratelimit/src/backend.rs)
- [`rust/crates/underlay-ratelimit/src/config.rs`](../../rust/crates/underlay-ratelimit/src/config.rs)
- [`rust/crates/underlay-security-alerts/src/lib.rs`](../../rust/crates/underlay-security-alerts/src/lib.rs)
- [`rust/crates/underlay-security-alerts/src/types.rs`](../../rust/crates/underlay-security-alerts/src/types.rs)
- [`rust/crates/underlay-security-alerts/src/detector.rs`](../../rust/crates/underlay-security-alerts/src/detector.rs)

Supporting:

- [`docs/architecture/010-package-map.md`](../architecture/010-package-map.md)
- [`docs/architecture/020-rust-api-foundation.md`](../architecture/020-rust-api-foundation.md)

If these diverge, the shared code wins.

## Contract Goal

Underlay should provide one reusable operator-facing infrastructure layer with
clear seams:

- async work is durable and observable
- scheduled work is declarative and overlap-aware
- domain events have a reliable append/process path
- audit and security signals are retained as first-class operator evidence
- email and rate limiting are pluggable service seams, not app rewrites

The goal is a shared control plane, not an app-specific admin stack.

## Shared Boundary

### Job system spine

`underlay-jobs` is the async infrastructure anchor.

Core pieces:

- `Job`
- `JobStatus`
- `JobConfig`
- `BackoffStrategy`
- `BackoffJitter`
- `JobProgress`
- `JobHandler`
- `JobHandlerError`
- `JobStore`
- `JobRegistry`
- `JobRunner`
- `JobRunnerConfig`

Rules:

- the shared queue model is durable, typed by `job_type`, and payload-driven
- handlers register by stable job-type string
- retries, timeout, overlap policy, progress tracking, and priority are part of
  the shared job contract
- `JobRunner` owns fetch/dispatch/retry completion semantics, not business
  logic
- polling mode is the generic baseline; notification mode is the production
  recommendation when supported

### Job persistence and Postgres implementation

Underlay owns a production-oriented Postgres implementation, not just traits.
The stable job contract lives in `underlay-jobs`; concrete Postgres runtime code
lives in `underlay-jobs-postgres`.

Core pieces:

- `JobRepository`
- `RepoError`
- `claim_batch()`
- `mark_running()`
- `mark_succeeded()`
- `mark_failed()`
- job SQL migration artifacts exposed from the crate

Rules:

- the shared Postgres path uses durable queue records with explicit status
  transitions
- claim semantics rely on `FOR UPDATE SKIP LOCKED`
- retry and failure storage semantics belong in the shared repo implementation,
  not in each app’s handlers
- LISTEN/NOTIFY is an optimization layer over a durable queue, not the source
  of truth

### Scheduled tasks

Recurring work is a first-class extension of the shared job system.
Pure scheduler configuration remains part of the job contract. The
Postgres-bound scheduler runtime lives in `underlay-jobs-postgres`.

Core pieces:

- `ScheduledTask`
- `ScheduledTaskDefinition`
- `ScheduledTaskRepository`
- `Scheduler`
- `SchedulerConfig`
- `PgJobNotifier`
- `JOB_NOTIFY_CHANNEL`

Rules:

- scheduled tasks are code-registered definitions persisted into durable task
  rows
- cron schedule parsing and due-task evaluation belong to the shared scheduler
- overlap prevention is explicit per task
- scheduled tasks create jobs; they are not a separate execution model
- notification support improves responsiveness but must still tolerate missed
  notifications through fallback polling

### Dead-letter and failure handling

Failed async work is first-class operator data.

Core pieces:

- `DeadLetter`
- `DeadLetterFilters`
- `DeadLetterStore`
- `JobFailureOutcome`
- `PgDeadLetterRepository`

Rules:

- exhausted or permanently failed jobs become dead-letter records
- retrying dead letters produces new job ids
- archiving old dead letters is a supported maintenance operation
- operator inspection and remediation depend on this retained failure history

### Job event stream

Underlay owns an internal event sink seam around job lifecycle changes.

Core pieces:

- `JobEvent`
- `JobEventSink`
- `JobEventHub`

Rules:

- job events are internal operator/observability hooks, not a replacement for
  durable job state
- sinks must tolerate fan-out semantics from the shared runner/repo path
- event payloads reflect lifecycle transitions like enqueue, claim, start,
  fail, complete, and dead-lettering

### Domain events and outbox

Underlay owns a shared event-append and reliable outbox-processing seam.

Core pieces:

- `DomainEvent`
- `NewDomainEvent`
- `DomainEventWriter`
- `OutboxEvent`
- `OutboxConfig`
- `OutboxNotifier`
- `OutboxProcessor`
- `DOMAIN_EVENT_NOTIFY_CHANNEL`

Rules:

- `underlay-events` is the append/schema seam, not the full processing stack
- apps append domain events through the shared writer boundary
- reliable asynchronous processing uses the outbox pattern, not in-request
  best-effort dispatch
- the Postgres jobs adapter owns the durable claim/process/mark-processed model
  and LISTEN/NOTIFY wake-up path
- LISTEN/NOTIFY is a wake-up mechanism layered on top of durable outbox rows
- outbox handlers mark events processed only after successful handling

### Audit log contract

`underlay-audit` owns the reusable audit-entry and query seam over app-owned
tables.

Core pieces:

- `AuditAction`
- `AuditEntry`
- `AuditLogRow`
- `AuditTable`
- `append_audit_log_to_table()`
- `append_audit_log_to_table_async()`
- `AuditLogFilters`
- `list_audit_logs_from_table()`
- `get_audit_log_by_id_from_table()`
- `count_audit_logs_from_table()`

Rules:

- audit logs use app-owned schema/table locations but share one retained
  entry/query model
- typed table config is the API boundary; raw string table-name helpers are not
  retained
- the shared contract is the row shape, write helpers, filters, and query
  semantics, not a fixed shared migration-owned table
- audit entries capture actor, action, resource identity, details, correlation
  id, and IP address
- shared audit actions provide a common baseline but allow custom extensions
- audit is retained operator evidence, not just debug logging

### Email infrastructure

`underlay-email` owns a pluggable outbound email service seam.

Core pieces:

- `EmailAddress`
- `Email`
- `EmailBuilder`
- `SendResult`
- `EmailAdapter`
- `EmailManager`
- `EmailManagerConfig`
- `AdapterType`
- `SmtpConfig`
- `SesConfig`
- `DevCaptureConfig`
- optional `EmailTemplateEngine`
- optional `EmailContext`

Rules:

- email sending is adapter-based
- the manager owns default-from composition and high-level send flow
- message composition is explicit and typed
- template rendering is optional convenience, not a required part of the core
  email contract
- if used, template rendering belongs to the shared email system rather than
  leaking Tera-specific concerns into unrelated packages
- local development delivery should use SMTP against the Effigy-provided Mailpit
  service
- DB-backed email capture and `/system/emails` admin/API surfaces are deprecated
  and must not be added to new Underlay apps
- legacy `dev_capture` config aliases may map to SMTP during compatibility
  cleanup, but they must not imply a retained captured-email store

### Rate limiting

`underlay-ratelimit` owns the generic rate-limit backend contract.

Core pieces:

- `RateLimitBackend`
- `RateLimitConfig`
- `RateLimitResult`
- `InMemoryBackend`

Rules:

- rate limiting is backend-pluggable
- the shared contract covers check, increment, reset, and combined
  check-and-increment behavior
- the in-memory backend is a convenience implementation, not the implied
  production backend for every app
- rate limits are reusable infrastructure, not auth-only logic

### Security alerts

`underlay-security-alerts` owns a thin shared login-security signal layer.

Core pieces:

- `SecurityAlertConfig`
- `SecurityAlertType`
- `LoginAttemptSignalCounts`
- `SecurityAlertEventInput`
- `LoginAttemptsTable`
- `SecurityAlertEventsTable`
- `SecurityAlertTables`
- `evaluate_alerts()`
- `load_ip_signal_counts_from_table()`
- `has_recent_alert_in_table()`
- `insert_alert_event_into_table()`

Rules:

- shared security alerts are signal evaluation and deduped persistence helpers,
  not a full notification product
- consuming apps own how login attempts are recorded and how alerts are sent
- typed table config is the API boundary; raw string table-name helpers are not
  retained
- cooldown and threshold logic are shared config, so alerting behavior is
  portable across apps

## Invariants

- async work is durable state first, notification second
- scheduled tasks are a job producer, not a separate execution universe
- dead letters and audit rows are retained operator evidence
- domain events must have a reliable append/process path
- email, rate limiting, and security alerts remain seam-based and backend
  pluggable
- shared operator infrastructure must stay app-agnostic even when it enables
  rich admin/system pages downstream

## Extension Points

Allowed:

- app-local job handlers registered through the shared registry
- custom `JobStore`, `DomainEventWriter`, `RateLimitBackend`, and `EmailAdapter`
  implementations
- app-specific audit table names and custom audit actions
- app-local notification policy for dead letters, alerts, and email
- provider/backend expansion beyond current SMTP/SES/dev-capture and in-memory
  implementations

Not allowed:

- collapsing queue semantics into fire-and-forget in-process tasks
- treating LISTEN/NOTIFY as durable queue state
- moving operator/admin page behavior back down into the shared async crates
- replacing retained audit or dead-letter evidence with transient logs only

## Known Drift And Assessment Hooks

Current drift worth assessing later:

- `underlay-events` and `underlay-jobs-postgres::outbox` both define parts of
  the domain event/outbox story, so the ownership line between the pure event
  contract and the processor/runtime path should be checked
- audit and security-alert raw-string table wrappers were removed after the
  six-consumer typed-table rollout
- `SchedulerConfig` is exported from `underlay-jobs` while the actual scheduler
  runtime lives in `underlay-jobs-postgres`, which keeps dependencies clean but
  still deserves future config-vs-runtime assessment
- email templating is optional and shared, but later assessment should confirm
  whether it still earns retained Underlay ownership versus app-local message
  composition

These are assessment hooks, not reasons to widen the contract.

## Assessment Questions

Use this contract to judge later implementation work:

- does a proposed async/operator feature belong in the shared control plane or
  in app-local admin workflows
- are durable state, notifications, and handler execution still cleanly
  separated
- do audit, dead-letter, and security-alert seams preserve operator evidence
  well enough to support system pages
- are email and rate-limit backends still truly pluggable
- does the event/outbox story remain coherent across `underlay-events` and
  `underlay-jobs`

## Next Task

Execute `g04.008`: write `070-nightfire-and-migration-systems.md`.
