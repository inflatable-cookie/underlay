# Contract: Error Codes and Operator Audit

Status: active
Owner: repo maintainers
Depends on: `010-foundation-primitives-and-envelopes.md`, `029-non-resource-workflow-action-route-grammar.md`, `030-auth-and-session-systems.md`, `060-jobs-events-and-operator-systems.md`

## Purpose

Define the shared posture for stable domain error codes and operator-facing
audit evidence in normal Underlay apps.

This contract covers:

- error-code naming and stability by domain
- when admin mutations and workflow actions must emit audit evidence
- when async/operator systems must emit durable event or audit evidence
- how to distinguish required operator evidence from optional debug logging

It does not redefine the transport error envelope. That stays in `010`. It does
not redefine jobs or audit primitives themselves. That stays in `060`.

## Sources of Truth

Shared error and audit surfaces:

- [`docs/contracts/010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- [`docs/contracts/029-non-resource-workflow-action-route-grammar.md`](/Users/tom/Dev/projects/underlay/docs/contracts/029-non-resource-workflow-action-route-grammar.md)
- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- [`docs/contracts/060-jobs-events-and-operator-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/060-jobs-events-and-operator-systems.md)

Supporting guidance and evidence:

- [`docs/guides/078-error-logging.md`](/Users/tom/Dev/projects/underlay/docs/guides/078-error-logging.md)
- [`docs/guides/080-audit-logging.md`](/Users/tom/Dev/projects/underlay/docs/guides/080-audit-logging.md)
- [`docs/guides/081-auth-security-alerting.md`](/Users/tom/Dev/projects/underlay/docs/guides/081-auth-security-alerting.md)
- [`rust/crates/underlay-http/src/errors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/errors.rs)
- [`rust/crates/underlay-http/src/error_logging.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-http/src/error_logging.rs)
- [`rust/crates/underlay-audit/src/entry.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-audit/src/entry.rs)
- [`rust/crates/underlay-jobs/src/events.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-jobs/src/events.rs)
- [`rust/crates/underlay-security-alerts/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-security-alerts/src/lib.rs)

If these diverge, the contract plus the retained shared crate posture win.

## Contract Goal

Underlay should make operator evidence predictable.

A normal app team should not have to guess:

- how stable an error code must be
- whether a workflow action should create an audit entry
- whether operator-facing lifecycle evidence belongs in logs only or in durable
  audit/event systems
- which admin mutations are too sensitive to leave unaudited

The goal is one declared operator-traceability posture instead of six local
habits.

## Scope Boundary

In scope:

- public API error-code posture
- domain error-code namespaces
- admin mutation audit requirements
- workflow action audit requirements
- durable operator evidence for jobs, security alerts, and sensitive state
  changes

Out of scope:

- DTO envelope structure
- app-specific authorization policy
- analytics and product telemetry
- low-level tracing or debug-log conventions

## Shared Boundary

### Stable error-code rule

Error codes are part of the public compatibility surface.

Rules:

- every public transport error code must be stable and machine-readable
- codes should be namespaced by domain, not by route file
- changing an existing public error code is a contract change
- prefer specific domain codes over vague generic buckets when the condition is
  meaningful to callers or operators

Examples of acceptable domain posture:

- `auth.invalid_credentials`
- `auth.forbidden`
- `infra.db_error`
- `content.unknown_block`
- `media.not_found`

### Error-code grammar rule

Preferred error-code grammar is:

- `<domain>.<condition>`

Rules:

- keep domains short and recognizable:
  - `auth`
  - `infra`
  - `content`
  - `media`
  - `jobs`
- avoid route-shaped or UI-shaped codes
- do not encode HTTP status into the code itself

### Transport versus operator detail rule

Public error codes should stay stable and compact even when richer operator
detail exists.

Rules:

- transport error codes identify the category of failure
- operator logs and error records may keep richer context:
  - SQLSTATE
  - hints
  - correlation IDs
  - upstream details
- do not leak unstable internal exception wording into the public code surface

### Always-audit rule

These actions should normally emit durable audit evidence:

- authentication and session security mutations
- role or permission changes
- operator/admin mutations that create, update, delete, restore, or purge
  privileged resources
- configuration changes
- sensitive data export or reveal actions
- manual moderation, suspension, revoke, claim, or release actions with
  operator consequence

Rules:

- treat durable audit evidence as the default for privileged mutation surfaces
- use the shared audit seam rather than ad hoc local tables when the shared seam
  fits

### Usually-audit rule

These actions often need audit evidence, but the app may classify them case by
case:

- standard resource creation and update in admin surfaces
- workflow actions like `complete`, `skip`, or `reorder`
- restore and soft-delete operations on non-sensitive content

Rules:

- if the action is operator-visible, changes durable state, and is likely to be
  reviewed later, prefer auditing it
- if the action is routine and low-risk, a narrower event or job-history record
  may be enough instead of a full audit log entry

### Usually-not-audit rule

These actions normally do not need durable audit entries:

- read-only access to non-sensitive records
- local validation failures
- background polling loops with no state change
- noisy repeated capability checks or status reads

Rules:

- do not fill audit logs with low-value read noise
- use standard application logs or metrics instead when no durable operator
  trace is needed

### Workflow-action rule

Workflow routes covered by `029` should be reviewed for audit posture
explicitly.

Rules:

- `restore`, `purge`, `revoke`, `claim`, and `release` should normally be
  audited
- `reorder`, `complete`, and `skip` should be audited when they change
  meaningful operator-visible state or moderation history
- do not leave workflow actions unaudited by accident just because they are not
  plain CRUD endpoints

### Jobs and operator-event rule

Async and operator systems need durable evidence even when they do not produce a
user-facing audit row.

Rules:

- job lifecycle evidence belongs in durable job state and job events
- dead letters are operator evidence, not just debugging artifacts
- security alerts should produce durable alert-event evidence
- outbox/domain-event processing should keep durable processing state rather
  than relying on transient logs

### Correlation rule

When durable audit evidence is emitted, include the best available correlation
context.

Preferred context:

- actor id
- action
- resource type and id
- correlation/request id
- IP address when relevant
- concise details payload

Rules:

- correlation context should help operators link audit rows, error logs, and
  request history
- do not omit resource identity on privileged mutations without a clear reason

## Audit Classification

When reviewing a route or workflow, classify it as one of:

- `required`
  - must emit durable audit evidence
- `recommended`
  - should emit durable evidence unless the app has a clear lower-risk reason
- `not required`
  - standard logs/metrics are enough

Rules:

- make the classification explicit during convergence or review work
- do not rely on “obviously sensitive” as the only rationale

## What Good Looks Like

Good outcomes:

- public error codes are stable and domain-shaped
- operators can reconstruct privileged mutations from durable evidence
- job, alert, and dead-letter surfaces keep their own durable operator history
- audit logs stay high-signal instead of becoming read-noise dumps

Bad outcomes:

- codes drift by route or by individual developer taste
- privileged admin actions leave no durable trace
- workflow actions are skipped because they are “not CRUD”
- transient logs are the only evidence for sensitive state change

## Next Task

Use this contract when adding new admin mutations, workflow routes, security
surfaces, or domain error families, and when auditing whether an app's operator
evidence posture is strong enough.
