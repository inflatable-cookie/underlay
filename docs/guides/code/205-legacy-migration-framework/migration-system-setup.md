# Migration System Setup Playbook (Human + AI)

This playbook defines the complete setup sequence for a consuming Underlay app.

Use it as an execution checklist, not just reference material.

## 1. Outcomes

A setup is complete only when all outcomes are true:

1. Deterministic migration runner is implemented in app code.
2. Bundle build/replay works with digest-pinned refs.
3. Decision reuse works across at least two passes.
4. Report gates (integrity, governance, drift, verify, audit) are enforced.
5. Effigy tasks and CI automation produce repeatable artifacts.

## 2. Inputs

Prepare before implementation:

1. Legacy access credentials and extract strategy.
2. Target schema and plugin versioning policy.
3. Decision taxonomy (`decision_type` list and required outcomes).
4. Media/export strategy and storage location.
5. Governance ownership and retention policy.

## 3. Phase Plan

### Phase A - Runtime wiring

Deliverables:

1. `LegacySource` implementation with stable source identity.
2. `MigrationPlugin` implementation with deterministic normalize/transform/materialize hooks.
3. `DecisionResolver` implementation with canonical fingerprinting.
4. `AssetResolver` implementation for media transfer.
5. `RunStore` implementation for checkpoints, journal, unresolved queue, and summaries.

Acceptance criteria:

1. Stage order is fixed (`extract -> normalize -> transform -> decide -> materialize -> assets -> verify`).
2. Two identical runs against identical input produce identical fingerprints.
3. `RunStore` recovery can resume after interruption without journal loss.

### Phase B - Bundle and distribution

Deliverables:

1. Bundle build command and OCI publish/pull flow.
2. Digest pinning contract for all replay commands.
3. Media shard integrity checks.

Acceptance criteria:

1. Pulling by digest reproduces exact replay input.
2. Tag-only replay is blocked.
3. Bundle metadata records source system and schema version.

### Phase C - Decision reuse and AI boundaries

Deliverables:

1. `decision_journal.ndjson` emission.
2. `decision_index.json` sidecar build/lookup.
3. Invalidation policy for resolver/prompt/schema changes.
4. AI threshold and unresolved-queue routing policy.

Acceptance criteria:

1. Unchanged entities reuse prior decisions with zero new AI calls.
2. Changed entities only recompute dependent decisions.
3. Human override persists and wins over prior AI outcomes.
4. Sidecar index merges resolve conflicts deterministically with audit evidence.
5. Refresh runs emit suppression KPIs (`ai_call_suppression_ratio`, `reuse_ratio`, `human_queue_ratio`).

### Phase D - Operations and promotion gates

Deliverables:

1. Governance, integrity, drift, verify, and audit report generation.
2. Effigy task chain for preflight, demo pass, and refresh pass.
3. CI artifact upload for doctor/report outputs.
4. Single-command refresh validation gate (`migration:refresh:validate`).
5. Error-code registry drift gate (`migration:error-codes:lint`).
6. Error-code registry sync-check gate (`migration:error-codes:sync:check`).

Acceptance criteria:

1. Demo pass and refresh pass both complete with machine-readable artifacts.
2. Blocking gate violations fail promotion.
3. Pre-production replay from frozen digest matches expected outcomes.
4. Error-code registry lint passes with zero registry/script drift.
5. Error-code registry sync-check confirms generated output is up to date.

## 4. Evidence Checklist

Track these artifacts per pass:

1. Bundle digest ref.
2. `run-report.json`.
3. `decision_journal.ndjson`.
4. `decision_index.json`.
5. `decision-reuse-summary.json` (required for refresh scope).
6. `decision-reuse-summary-lint.json` (required for refresh scope).
7. `refresh-drift-gate.json` (required for refresh scope).
8. `refresh-drift-gate-lint.json` (required for refresh scope).
9. Governance policy report output.
10. Integrity, drift, verify, recovery, and audit report outputs.
11. `migration-doctor.json`.
12. `artifact-manifest.json` from `04_evidence_manifest.ts`.
13. evidence verification pass from `05_evidence_verify.ts`.
14. promotion decision lint pass from `07_promotion_decision_lint.ts`.
15. promotion release note outputs (`08_promotion_release_note.ts`) in JSON and Markdown.
16. promotion CI guard pass from `09_promotion_ci_guard.ts`.
17. promotion CI guard lint pass from `11_promotion_ci_guard_lint.ts`.
18. refresh sidecar merge report (`deduped/replaced/invalidated/corrupted` counts).
19. refresh AI suppression KPI summary (`candidate_decisions_total`, `new_ai_calls_total`, `ai_call_suppression_ratio`).
20. error-code registry lint pass from `15_error_code_registry_lint.ts`.
21. error-code registry sync-check pass from `16_error_code_registry_sync.ts --check`.

Canonical naming, directory layout, and gate-specific evidence minimums:
1. `migration-evidence-matrix.md`

## 5. AI Agent Execution Contract

Use this checklist when delegating to an AI agent:

1. Inputs must include digest refs, config path, and expected run scope (`demo|pre_production|production`).
2. Agent must fail fast on missing digest pinning or invalid index integrity.
3. Agent output must include:
   1. summary of reused/new/invalidated/unresolved decisions
   2. sidecar merge summary (deduped/replaced/invalidated/corrupted)
   3. suppression KPI summary for refresh passes
   4. list of generated artifacts and paths
   5. gate result (`pass|fail`) and promote recommendation
4. Agent must never rewrite historical journal entries.

## 6. Human Handoff Checklist

Before handing a migration run to stakeholders:

1. Confirm frozen digest for the reviewed demo/pre-production candidate.
2. Confirm refresh delta scope (new records, changed records, invalidations).
3. Confirm unresolved queue ownership and SLA.
4. Confirm report artifacts are archived under retention policy.

## 7. Command Baseline

Use direct scripts:

```bash
MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_doctor.ts --output ./runtime/migration-doctor.json

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/01_build_publish.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/02_run_reports.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/03_refresh_cycle.ts
```

Use Effigy:

```bash
effigy migration:doctor
effigy migration:demo
effigy migration:refresh
effigy migration:refresh:validate
effigy migration:validate
```
