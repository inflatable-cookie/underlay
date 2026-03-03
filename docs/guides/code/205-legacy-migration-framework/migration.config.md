# Migration Config Reference

This document defines configuration keys for the TypeScript migration scripts in this folder.

Config resolution order:

1. JSON config file (`MIGRATION_CONFIG_FILE`, default `./migration.config.json`)
2. Environment variables
3. Script defaults (non-required keys only)

## Key Matrix

| Key | Required | Default | Used by | Notes |
|---|---|---|---|---|
| `SOURCE_SYSTEM` | No | `legacy_site` | `00_preflight.ts`, `01_build_publish.ts`, `03_refresh_cycle.ts` | Logical source identifier stored in bundle metadata. |
| `TARGET_SCHEMA_VERSION` | No | `schema-v1` | `00_preflight.ts`, `01_build_publish.ts`, `03_refresh_cycle.ts` | Must align with migration plugin target schema contract. |
| `MEDIA_DIR` | No | `./legacy-export/media` | `00_preflight.ts`, `01_build_publish.ts`, `03_refresh_cycle.ts` | Preflight warns if missing (allowed for data-only migrations). |
| `BUNDLE_FILE` | No | `./dist/migration-bundle.oci` | `00_preflight.ts`, `01_build_publish.ts`, `03_refresh_cycle.ts` | Build output path; preflight validates writable parent dir. |
| `OCI_REF_TAG` | No | auto timestamped demo/refresh tag | `01_build_publish.ts`, `03_refresh_cycle.ts` | Publish target tag; scripts derive digest-pinned ref from publish output. |
| `BUNDLE_REF` | Yes for `reports` mode | none | `00_preflight.ts` (`reports` mode), `02_run_reports.ts` | Must be digest-pinned (`<repo>@sha256:<64 hex>`). |
| `OUTPUT_DIR` | No | `./runtime/demo-pass` or `./runtime/refresh-pass` | `00_preflight.ts`, `02_run_reports.ts`, `03_refresh_cycle.ts` | Runtime artifact/output directory. |
| `RUN_REPORT` | No | `${OUTPUT_DIR}/run-report.json` | `02_run_reports.ts`, `03_refresh_cycle.ts` | Must exist after orchestrator execution for report steps. |
| `GOVERNANCE_POLICY_FILE` | No | `./runtime/governance-policy.json` | `00_preflight.ts` (optional check), `02_run_reports.ts` | If missing, policy report is skipped. |
| `REUSE_FROM_DIGEST_REF` | Yes for `refresh` mode | none | `00_preflight.ts` (`refresh` mode), `03_refresh_cycle.ts` | Must be digest-pinned baseline for decision reuse. |
| `APP_MIGRATION_RUNNER_CMD` | No | empty | `03_refresh_cycle.ts` | Optional command that runs app orchestrator after `migration run`. |
| `DECISION_INDEX_FILE` | No | `${OUTPUT_DIR}/decision_index.json` | `03_refresh_cycle.ts` | Used for lineage-aware drift checks. |
| `DECISION_JOURNAL_FILE` | No | `${OUTPUT_DIR}/decision_journal.ndjson` | `03_refresh_cycle.ts` | Used for lineage-aware drift checks. |
| `DECISION_REUSE_SUMMARY_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.decision-reuse-summary.json` | `10_decision_reuse_summary.ts`, `04_evidence_manifest.ts` | Refresh-only evidence artifact with sidecar merge stats and AI suppression KPIs. |
| `DECISION_REUSE_SUMMARY_SCHEMA_FILE` | No | `./docs/guides/code/205-legacy-migration-framework/decision-reuse-summary.schema.json` | `12_decision_reuse_summary_lint.ts` | Schema file used to validate decision reuse summary contract. |
| `DECISION_REUSE_SUMMARY_LINT_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.decision-reuse-summary-lint.json` | `12_decision_reuse_summary_lint.ts`, `04_evidence_manifest.ts` | Refresh-only lint artifact for summary schema/semantic checks. |
| `REFRESH_DRIFT_GATE_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.refresh-drift-gate.json` | `13_refresh_drift_gate.ts`, `04_evidence_manifest.ts` | Refresh drift gate output artifact from thresholded drift check. |
| `REFRESH_DRIFT_GATE_SCHEMA_FILE` | No | `./docs/guides/code/205-legacy-migration-framework/refresh-drift-gate.schema.json` | `14_refresh_drift_gate_lint.ts` | Schema file used to validate refresh drift gate contract. |
| `REFRESH_DRIFT_GATE_LINT_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.refresh-drift-gate-lint.json` | `14_refresh_drift_gate_lint.ts`, `04_evidence_manifest.ts` | Refresh drift gate lint artifact for schema/semantic checks. |
| `PROJECT_NAME` | No | `migration` | `04_evidence_manifest.ts` | Lowercase kebab-case project key in evidence manifest naming. |
| `RUN_SCOPE` | No | `demo` | `04_evidence_manifest.ts` | One of `demo`, `refresh`, `pre_production`. |
| `RUN_DATE_UTC` | No | current UTC date (`YYYY-MM-DD`) | `04_evidence_manifest.ts` | Controls manifest filename and metadata. |
| `DOCTOR_REPORT` | No | derived from `RUN_REPORT` | `04_evidence_manifest.ts` | Defaults to `migration-doctor.json` when `RUN_REPORT` ends with `run-report.json`. |
| `VERIFY_ARTIFACT_FILE` | No | `${OUTPUT_DIR}/verification-artifacts/<run_id>.json` | `04_evidence_manifest.ts` | If omitted, script derives from `run_id` in run report. |
| `AUDIT_ARTIFACT_FILE` | No | `${OUTPUT_DIR}/audit-artifacts/<run_id>.json` | `04_evidence_manifest.ts` | If omitted, script derives from `run_id` in run report. |
| `ARTIFACT_MANIFEST_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.artifact-manifest.json` | `04_evidence_manifest.ts`, `05_evidence_verify.ts` | Output path for generated evidence manifest contract, and default verify input path. |
| `PROMOTION_DECISION_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-decision.json` | `06_promotion_check.ts` | Machine-readable promotion decision output. |
| `PROMOTION_DECISION_SCHEMA_FILE` | No | `./docs/guides/code/205-legacy-migration-framework/promotion-decision.schema.json` | `07_promotion_decision_lint.ts` | Schema file used to validate promotion decision artifact contract. |
| `PROMOTION_DECISION_LINT_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-decision-lint.json` | `07_promotion_decision_lint.ts` | Machine-readable decision lint result output. |
| `PROMOTION_RELEASE_NOTE_JSON_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-release-note.json` | `08_promotion_release_note.ts` | Machine-readable release summary for ticket/CI consumption. |
| `PROMOTION_RELEASE_NOTE_MD_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-release-note.md` | `08_promotion_release_note.ts` | Markdown summary for release tickets/human review. |
| `PROMOTION_CI_GUARD_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-ci-guard.json` | `09_promotion_ci_guard.ts` | Canonical CI verdict artifact and failure reasons. |
| `PROMOTION_CI_GUARD_SCHEMA_FILE` | No | `./docs/guides/code/205-legacy-migration-framework/promotion-ci-guard.schema.json` | `11_promotion_ci_guard_lint.ts` | Schema file used to validate CI guard artifact contract. |
| `PROMOTION_CI_GUARD_LINT_FILE` | No | `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.promotion-ci-guard-lint.json` | `11_promotion_ci_guard_lint.ts` | Machine-readable CI guard lint output. |
| `DRIFT_MAX_UNRESOLVED` | No | `0` | `06_promotion_check.ts` | Drift threshold passed to `migration report drift --max-unresolved`. |
| `DRIFT_MAX_GOVERNANCE` | No | `0` | `06_promotion_check.ts` | Drift threshold passed to `migration report drift --max-governance`. |
| `DRIFT_MAX_LINEAGE` | No | `0` | `06_promotion_check.ts` | Drift threshold passed to `migration report drift --max-lineage`. |
| `MIGRATION_CONFIG_FILE` | No | `./migration.config.json` | all scripts (indirect via `config.ts`) | Path to JSON config object. |

## Mode-Specific Requirements

### `00_preflight.ts --mode general`

Required keys:

1. none (all have defaults)

Validated:

1. tool availability (`bun`, `underlay-devtools`)
2. writable paths for `BUNDLE_FILE` and `OUTPUT_DIR`

### `00_preflight.ts --mode reports`

Required keys:

1. `BUNDLE_REF`

Validated:

1. `BUNDLE_REF` digest format
2. general-mode checks

### `00_preflight.ts --mode refresh`

Required keys:

1. `REUSE_FROM_DIGEST_REF`

Validated:

1. `REUSE_FROM_DIGEST_REF` digest format
2. general-mode checks

### `04_evidence_manifest.ts`

Required keys:

1. none (defaults available)

Requires files to exist:

1. `migration-doctor.json`
2. `run-report.json`
3. `decision_index.json`
4. `decision_journal.ndjson`
5. verify artifact JSON
6. audit artifact JSON
7. `decision-reuse-summary.json` when `RUN_SCOPE=refresh`
8. `decision-reuse-summary-lint.json` when `RUN_SCOPE=refresh`
9. `refresh-drift-gate.json` when `RUN_SCOPE=refresh`
10. `refresh-drift-gate-lint.json` when `RUN_SCOPE=refresh`

Failure behavior:

1. hard-fails if any required artifact is missing
2. hard-fails if run report lacks `run_id`

### `05_evidence_verify.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--input <artifact-manifest.json>` (optional override)
2. or `ARTIFACT_MANIFEST_FILE`
3. or default `${OUTPUT_DIR}/metadata/<project>.<scope>.<date>.artifact-manifest.json`

Failure behavior:

1. hard-fails on missing manifest file
2. hard-fails on schema/version mismatch
3. hard-fails on checksum mismatch or missing artifact file
4. hard-fails if required artifact entries are missing
5. hard-fails for refresh manifests missing `decision_reuse_summary`
6. hard-fails for refresh manifests missing `decision_reuse_summary_lint`
7. hard-fails for refresh manifests missing `refresh_drift_gate`
8. hard-fails for refresh manifests missing `refresh_drift_gate_lint`

### `10_decision_reuse_summary.ts`

Required keys:

1. `REUSE_FROM_DIGEST_REF`

Inputs:

1. `--input <run-report.json>` (optional override)
2. `--output <decision-reuse-summary.json>` (optional override)

Outputs:

1. `underlay.migration.decision_reuse_summary.v1` summary JSON
2. refresh reuse stats, sidecar merge summary, and suppression KPI metrics

Failure behavior:

1. hard-fails when `RUN_SCOPE` is not `refresh`
2. hard-fails when `REUSE_FROM_DIGEST_REF` is missing or not digest-pinned
3. hard-fails when `run-report.json` is missing or invalid

### `06_promotion_check.ts`

Required keys:

1. none (defaults available)

Runs required checks:

1. evidence manifest generation (`04_evidence_manifest.ts`)
2. evidence verification (`05_evidence_verify.ts`)
3. integrity report
4. drift report (with configured thresholds)
5. verify report
6. policy report (required; fails if governance policy file missing)

Outputs:

1. `underlay.migration.promotion_check.v1` decision file
2. recommendation: `promote|hold`
3. non-zero exit when recommendation is `hold`

### `07_promotion_decision_lint.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--input <promotion-decision.json>` (optional override)
2. `--schema <promotion-decision.schema.json>` (optional override)
3. `--output <promotion-decision-lint.json>` (optional override)
4. or `PROMOTION_DECISION_FILE` + `PROMOTION_DECISION_SCHEMA_FILE` + `PROMOTION_DECISION_LINT_FILE`

Failure behavior:

1. hard-fails if decision file missing
2. hard-fails if schema file missing
3. hard-fails on schema contract mismatch

Output:

1. `underlay.migration.promotion_decision_lint.v1` lint report JSON

### `08_promotion_release_note.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--decision <promotion-decision.json>` (optional override)
2. `--lint <promotion-decision-lint.json>` (optional override)
3. `--manifest <artifact-manifest.json>` (optional override)
4. `--output-json <promotion-release-note.json>` (optional override)
5. `--output-md <promotion-release-note.md>` (optional override)

Outputs:

1. `underlay.migration.promotion_release_note.v1` JSON summary
2. Markdown release summary with recommendation, gate statuses, blockers, and checksums

### `09_promotion_ci_guard.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--decision <promotion-decision.json>` (optional override)
2. `--lint <promotion-decision-lint.json>` (optional override)
3. `--release-note <promotion-release-note.json>` (optional override)
4. `--output <promotion-ci-guard.json>` (optional override)

Outputs:

1. `underlay.migration.promotion_ci_guard.v1` verdict JSON
2. non-zero exit when verdict is `failed`

### `11_promotion_ci_guard_lint.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--input <promotion-ci-guard.json>` (optional override)
2. `--schema <promotion-ci-guard.schema.json>` (optional override)
3. `--output <promotion-ci-guard-lint.json>` (optional override)
4. or `PROMOTION_CI_GUARD_FILE` + `PROMOTION_CI_GUARD_SCHEMA_FILE` + `PROMOTION_CI_GUARD_LINT_FILE`

Failure behavior:

1. hard-fails if CI guard file missing
2. hard-fails if schema file missing
3. hard-fails on schema contract mismatch
4. hard-fails on semantic mismatch (`reason_count` vs `reasons[]`, status/reason consistency)

Output:

1. `underlay.migration.promotion_ci_guard_lint.v1` lint report JSON

### `12_decision_reuse_summary_lint.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--input <decision-reuse-summary.json>` (optional override)
2. `--schema <decision-reuse-summary.schema.json>` (optional override)
3. `--output <decision-reuse-summary-lint.json>` (optional override)
4. or `DECISION_REUSE_SUMMARY_FILE` + `DECISION_REUSE_SUMMARY_SCHEMA_FILE` + `DECISION_REUSE_SUMMARY_LINT_FILE`

Failure behavior:

1. hard-fails if summary file missing
2. hard-fails if schema file missing
3. hard-fails on schema contract mismatch
4. hard-fails on semantic mismatch between counters and derived KPI ratios

Output:

1. `underlay.migration.decision_reuse_summary_lint.v1` lint report JSON

### `13_refresh_drift_gate.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--output <refresh-drift-gate.json>` (optional override)
2. `RUN_REPORT`
3. `DECISION_INDEX_FILE`
4. `DECISION_JOURNAL_FILE`
5. drift thresholds:
   1. `DRIFT_MAX_UNRESOLVED`
   2. `DRIFT_MAX_GOVERNANCE`
   3. `DRIFT_MAX_LINEAGE`

Failure behavior:

1. hard-fails when `RUN_SCOPE` is not `refresh`
2. hard-fails when `underlay-devtools` is missing from PATH
3. hard-fails when drift report exits non-zero against configured thresholds

Output:

1. `underlay.migration.refresh_drift_gate.v1` drift gate result JSON

### `14_refresh_drift_gate_lint.ts`

Required keys:

1. none (defaults available)

Inputs:

1. `--input <refresh-drift-gate.json>` (optional override)
2. `--schema <refresh-drift-gate.schema.json>` (optional override)
3. `--output <refresh-drift-gate-lint.json>` (optional override)
4. or `REFRESH_DRIFT_GATE_FILE` + `REFRESH_DRIFT_GATE_SCHEMA_FILE` + `REFRESH_DRIFT_GATE_LINT_FILE`

Failure behavior:

1. hard-fails if drift gate file missing
2. hard-fails if schema file missing
3. hard-fails on schema contract mismatch
4. hard-fails on semantic mismatch (`status` vs `exit_code`)

Output:

1. `underlay.migration.refresh_drift_gate_lint.v1` lint report JSON

## Safe Update Rules

1. Keep keys `UPPER_SNAKE_CASE` to align with env mapping.
2. Prefer updating `migration.config.json` over embedding long env lines in task commands.
3. When introducing a new key, update all three artifacts together:
   1. `config.ts`
   2. `config.schema.json`
   3. this `migration.config.md`
4. For digest refs, always use immutable `@sha256:` references, never tags.

## Examples

1. Base config: `migration.config.sample.json`
2. Env fallback template: `migration.env.sample`
3. Effigy task mapping: `effigy.migration.sample.toml`
