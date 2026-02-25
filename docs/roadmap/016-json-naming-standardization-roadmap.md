# 016 – JSON Naming Standardization (snake_case)

Status: Complete

## Overview

This roadmap standardizes JSON naming across all Underlay-based systems on a single convention: `snake_case`.

Scope includes:

1. Underlay shared libraries and docs
2. `underlay-reference` (`acme-*`)
3. `acowtancy` (all active apps)
4. `compli-me`

Target outcome:

1. All exported JSON fields are `snake_case`
2. All persisted JSON payloads are `snake_case`
3. No mixed-case contract drift between services

## Decision

- [x] Canonical exported JSON naming is `snake_case`
- [x] Legacy `camelCase` response compatibility windows are documented where needed

## Progress Checklist

- [x] Phase 16.1 complete
- [x] Phase 16.2 complete
- [x] Phase 16.3 complete
- [x] Phase 16.4 complete
- [x] Phase 16.5 complete
- [x] Phase 16.6 complete
- [x] Validation plan complete
- [x] Success metrics achieved

## Active Remaining Work

- [x] Validate critical `acowtancy` admin pages and integrations after DB reset.
- [x] Remove compatibility adapters after migration cutover.
- [x] Run naming guardrail scripts in all repos (executed 2026-02-25; see `docs/reports/2026-02-25-cross-repo-auth-json-verification.md`).
- [x] Sample key API responses and confirm `snake_case` field names (Songsprout + Acowtancy runtime samples captured; see `docs/reports/2026-02-25-cross-repo-auth-json-verification.md`).
- [x] Confirm error logs and job payload records use `snake_case` keys (Songsprout + Acowtancy `platform.error_log`/job payload samples captured; see verification report).
- [x] Resolve Acowtancy camelCase findings in `farmyard/crates/nightfire/src/lib.rs` and `farmyard/crates/api/src/routes/admin/learning/outcomes/questions.rs` (route DTOs migrated, `nightfire` allowlisted as external-contract exception).
- [x] Resolve Songsprout camelCase query fields in `nursery/crates/api/src/handlers.rs` (`includeTotal` now compatibility alias on canonical `include_total`).

## Problem Statement

Current API and export payloads are inconsistent across projects (`camelCase` and `snake_case` mixed). This causes:

1. Client and backend contract confusion
2. Serialization/deserialization edge-case bugs
3. Migration friction between apps sharing Underlay patterns
4. Higher maintenance cost for seeds, fixtures, and job payloads

## Canonical Rules

1. API request/response JSON fields: `snake_case`
2. Background job payload JSON: `snake_case`
3. Error context JSON persisted by middleware: `snake_case`
4. Seed/export/import JSON files: `snake_case`
5. Rust DTO structs: snake_case fields by default (avoid per-field rename unless mapping external contracts)
6. TypeScript app domain models may remain camelCase internally only if transformed at the API boundary in one place

## Non-Goals

1. Refactor unrelated business logic
2. Rebuild UI state conventions end-to-end
3. Big-bang break production clients without a migration window

## Phase 16.1 – Underlay Contract Policy and Guardrails

### Tasks

- [x] Add a single JSON naming policy guide in Underlay docs (`snake_case` canonical).
- [x] Add Rust examples showing preferred serde defaults for `snake_case`.
- [x] Add lint/check scripts to flag `rename_all = "camelCase"` usage in internal DTOs.
- [x] Define approved exception list for third-party external contracts.

### Implementation Tickets

- [x] `underlay/docs/guides/` – add `JSON naming policy` guide
- [x] `underlay/docs/guides/070-api-handlers.md` – reference naming policy
- [x] `underlay/docs/guides/200-project-sync.md` – add upgrade checklist section
- [x] `underlay/scripts/` – add `check-json-naming.sh` guardrail script

### Acceptance Criteria

- [x] One explicit naming policy is published and referenced by core guides.
- [x] CI/lint path exists to detect new camelCase DTO drift.

## Phase 16.2 – Underlay Runtime and Shared Types Audit

### Tasks

- [x] Audit `underlay-http`, shared error envelopes, auth/session payloads, and common DTOs.
- [x] Remove/replace internal `camelCase` serde directives where contracts are internal.
- [x] Keep exception annotations explicit for external protocol integrations.
- [x] Add tests for representative serialization/deserialization contracts.

### Acceptance Criteria

- [x] Underlay-shared runtime contracts serialize to `snake_case` by default.
- [x] Tests fail on accidental naming regressions.

## Phase 16.3 – `underlay-reference` Migration (`acme-*`)

### Tasks

- [x] Sweep `acme-api` DTOs, route payloads, and job payload types to `snake_case`.
- [x] Update `acme-client` generated/manual types and command adapters.
- [x] Update `acme-admin` and `acme-front` boundary mappers where required.
- [x] Update dev seeds/fixtures/export files to `snake_case`.
- [x] Verify post-reset DB seeds and local workflows remain stable.

### Implementation Tickets

- [x] `acme-api/crates/api/src/dto/`
- [x] `acme-api/crates/jobs/src/`
- [x] `acme-client/src/`
- [x] `acme-admin/src/lib/api/` and route loaders/actions
- [x] `acme-front/src/lib/api/` and route loaders/actions

### Acceptance Criteria

- [x] `acme-*` API responses/requests are consistently `snake_case`.
- [x] Admin/front continue functioning after seed/schema refresh.

## Phase 16.4 – `acowtancy` Migration

### Tasks

- [x] Sweep API DTOs and handlers for mixed JSON naming.
- [x] Update app clients and boundary adapters.
- [x] Update seed/export/import and test fixtures.
- [x] Validate critical admin pages and integration flows after DB reset (tracked in Active Remaining Work).

### Acceptance Criteria

- [x] `acowtancy` API contracts are `snake_case` end-to-end (final confirmation tracked in Active Remaining Work).
- [x] No mixed-case payloads in captured logs or exported JSON (validation tracked in Active Remaining Work).

## Phase 16.5 – `compli-me` Migration

### Tasks

- [x] Sweep API DTOs and handlers for mixed JSON naming.
- [x] Update API clients and front/admin boundary adapters.
- [x] Update seed/export/import and fixture payloads.
- [x] Validate scheduled jobs and task payload handling after DB reset.

### Acceptance Criteria

- [x] `compli-me` API contracts and job payloads are `snake_case`.
- [x] No missing-field regressions from naming mismatch.

## Phase 16.6 – Compatibility Windows and Legacy Removal

### Tasks

- [x] Identify endpoints requiring temporary dual-read compatibility (`camelCase` + `snake_case`).
- [x] Implement short-lived compatibility adapters at the boundary.
- [x] Publish sunset dates for compatibility mode.
- [x] Remove compatibility adapters after migration cutover (tracked in Active Remaining Work).

### Acceptance Criteria

- [x] Legacy compatibility is explicit and time-boxed.
- [x] Final adapter removal and cutoff confirmation tracked in Active Remaining Work.

Compatibility inventory is tracked in `docs/roadmap/016-compatibility-adapters.csv`.
Sunset guardrail is enforced by `scripts/check-compatibility-sunset.sh`.

## Validation Plan

- [x] Run naming guardrail scripts in all repos (tracked in Active Remaining Work).
- [x] Run Rust checks/tests for touched crates in each project.
- [x] Run frontend type checks (`bun check`) for touched apps.
- [x] Sample key API responses and confirm field names are `snake_case` (completed; tracked in verification report).
- [x] Confirm error logs and job payload records use `snake_case` keys (completed; tracked in verification report).

## Success Metrics

- [x] 0 newly introduced internal `camelCase` DTO naming directives in migration scope.
- [x] 0 mixed-case fields in sampled API payloads across the 3 app families.
- [x] 0 naming-mismatch runtime errors in smoke tests after DB reset and seed load.

Closure note (2026-02-25):
- Compatibility inventory entries have been reconciled to current module layouts and all remaining `sort_key_dual_read` adapters are marked removed in `docs/roadmap/016-compatibility-adapters.csv`.
- Cross-repo naming guardrails, runtime samples, and closure sweep checks are green.
- [x] Documentation and guardrails are in place to prevent regression.

## Execution Notes

1. Complete Underlay phases first (16.1–16.2), then migrate app repos.
2. Keep migrations incremental and reversible by domain.
3. Prefer boundary transformations over deep internal naming churn in frontend state layers.
4. Track every temporary compatibility adapter as a checklist item with an owner and removal target.
