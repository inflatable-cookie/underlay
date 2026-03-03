# Migration Evidence Matrix

This file standardizes artifact naming, output layout, and minimum evidence required
for migration gate decisions.

Use this as the canonical evidence contract for `demo`, `refresh`, and `pre_production` runs.

Implementation command:

```bash
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts
```

## 1. Directory Layout

Use a single run root per pass:

```text
runtime/
  migrations/
    <run_scope>/
      <run_date_utc>/
        <project_name>/
          artifacts/
          reports/
          metadata/
```

Example:

```text
runtime/migrations/demo/2026-03-02/acowtancy/
```

## 2. Naming Convention

File naming format:

```text
<project_name>.<run_scope>.<run_date_utc>.<artifact_name>.json
```

For NDJSON:

```text
<project_name>.<run_scope>.<run_date_utc>.<artifact_name>.ndjson
```

Required variable formats:

1. `project_name`: lowercase kebab-case
2. `run_scope`: `demo|refresh|pre_production`
3. `run_date_utc`: `YYYY-MM-DD`

## 3. Required Core Artifacts (All Scopes)

Minimum files required for every run:

1. `migration-doctor` output
2. `run-report` output
3. `decision_index` output
4. `decision_journal` output
5. `verify` output artifact
6. `audit` output artifact

Recommended paths inside run root:

1. `metadata/<project>.<scope>.<date>.migration-doctor.json`
2. `metadata/<project>.<scope>.<date>.run-report.json`
3. `artifacts/<project>.<scope>.<date>.decision_index.json`
4. `artifacts/<project>.<scope>.<date>.decision_journal.ndjson`
5. `reports/<project>.<scope>.<date>.verify.json`
6. `reports/<project>.<scope>.<date>.audit.json`

## 4. Gate Matrix by Scope

| Scope | Required Gates | Minimum Evidence to Pass |
|---|---|---|
| `demo` | integrity, governance, drift, verify, audit, evidence_verify | all core artifacts present + `drift.blocking_issue_count=0` + evidence verification pass |
| `refresh` | integrity, governance, drift(lineage), verify, audit, reuse-delta, evidence_verify | all core artifacts present + decision reuse stats (`reused/new/invalidated/unresolved`) + sidecar merge report + AI suppression KPI summary + evidence verification pass |
| `pre_production` | integrity (signature phase-aware), governance, drift, verify, audit, recovery, evidence_verify | all core artifacts present + signature evidence fields + `verify_passed=true` + evidence verification pass |

## 5. Scope-Specific Extra Evidence

### `demo`

Must include:

1. Candidate bundle digest used for replay.
2. Governance policy evaluation output.
3. Promotion recommendation (`promote|hold|rollback`).

### `refresh`

Must include:

1. `reuse_from_digest_ref` baseline digest.
2. Reuse stats summary:
   1. `reused_count`
   2. `new_count`
   3. `invalidated_count`
   4. `unresolved_count`
3. Lineage drift inputs (`decision_index` + `decision_journal`) and result summary.
4. Sidecar merge report:
   1. `deduped_count`
   2. `replaced_count`
   3. `invalidated_count`
   4. `corrupted_count`
   5. recommended file: `metadata/<project>.<scope>.<date>.decision-reuse-summary.json`
5. Reuse summary lint report:
   1. `status=passed`
   2. `error_count=0`
   3. recommended file: `metadata/<project>.<scope>.<date>.decision-reuse-summary-lint.json`
6. Drift gate report:
   1. `status=passed`
   2. `exit_code=0`
   3. recommended file: `metadata/<project>.<scope>.<date>.refresh-drift-gate.json`
7. Drift gate lint report:
   1. `status=passed`
   2. `error_count=0`
   3. recommended file: `metadata/<project>.<scope>.<date>.refresh-drift-gate-lint.json`
8. AI suppression KPI summary:
   1. `candidate_decisions_total`
   2. `new_ai_calls_total`
   3. `reused_decisions_total`
   4. `ai_call_suppression_ratio`
   5. `reuse_ratio`
   6. `human_queue_ratio`
9. KPI target evaluation for stable projects:
   1. `ai_call_suppression_ratio >= 0.85`
   2. `reuse_ratio >= 0.80`
   3. `human_queue_ratio <= 0.05`

### `pre_production`

Must include:

1. Frozen promotion candidate digest.
2. Signature evidence (when required by phase/scope):
   1. `signature_verified=true`
   2. `signature_verified_at`
   3. `signer_identity`
   4. `signature_key_id`
3. Recovery report summary (`resume_attempted`, status, reason).

## 6. Integrity and Checksum Recording

For each required artifact, record:

1. absolute/relative path
2. SHA-256 checksum
3. generation timestamp (UTC)

Canonical manifest name:

1. `metadata/<project>.<scope>.<date>.artifact-manifest.json`

Required manifest shape:

```json
{
  "schema": "underlay.migration.evidence_manifest.v1",
  "schema_version": 1,
  "project_name": "acowtancy",
  "run_scope": "refresh",
  "run_date_utc": "2026-03-02",
  "artifacts": [
    {
      "artifact_name": "run-report",
      "path": "metadata/acowtancy.refresh.2026-03-02.run-report.json",
      "sha256": "..."
    }
  ]
}
```

## 7. Operator Acceptance Rules

A run is not gate-eligible if any are true:

1. missing required core artifact
2. missing checksum entry in artifact manifest
3. digest refs not pinned (`@sha256:` absent)
4. blocking integrity/governance/drift issues present
5. evidence verification failed (`05_evidence_verify.ts` non-zero exit)
6. refresh scope missing sidecar merge, summary/drift lint, or suppression KPI evidence

## 8. Agent Output Contract Mapping

When AI agents report completion, output must map directly to this matrix:

1. gate outcomes by name
2. artifact list with paths + checksums
3. reuse stats (for `refresh`)
4. sidecar merge report + suppression KPI summary (for `refresh`)
5. reuse summary lint result (for `refresh`)
6. drift gate + drift gate lint results (for `refresh`)
7. signature evidence (for `pre_production` when required)
8. recommendation and blocking reasons

## 9. Promotion Decision Artifact

Promotion gate runs should emit:

1. `metadata/<project>.<scope>.<date>.promotion-decision.json`
2. schema: `underlay.migration.promotion_check.v1`
3. minimum fields:
   1. `recommendation` (`promote|hold`)
   2. `can_promote` (boolean)
   3. `checks[]` with per-check status
   4. `blocking_reasons[]`
4. lint requirement:
   1. `07_promotion_decision_lint.ts` must pass against `promotion-decision.schema.json`
5. lint artifact:
   1. `metadata/<project>.<scope>.<date>.promotion-decision-lint.json`
   2. schema: `underlay.migration.promotion_decision_lint.v1`
6. CI guard artifact:
   1. `metadata/<project>.<scope>.<date>.promotion-ci-guard.json`
   2. schema: `underlay.migration.promotion_ci_guard.v1`
7. CI guard lint artifact:
   1. `metadata/<project>.<scope>.<date>.promotion-ci-guard-lint.json`
   2. schema: `underlay.migration.promotion_ci_guard_lint.v1`
