# 205 Legacy Migration Framework Samples

This folder contains TypeScript golden-path scripts referenced by `205-legacy-migration-framework.md`.

## Start here

For full setup execution order (runtime wiring -> bundle -> decision reuse -> operations),
read `migration-system-setup.md` first, then run tasks from this README.

For complete documentation coverage, read `documentation-map.md`.

For operator-to-agent delegation, use `ai-migration-handoff.prompt.md`.
For gate evidence and artifact naming rules, use `migration-evidence-matrix.md`.
For failure diagnosis by gate and script, use `migration-troubleshooting.md`.
For stable CI/agent error parsing, use `migration-error-codes.md`.

## Config precedence

All scripts load configuration in this order:

1. JSON config file (`MIGRATION_CONFIG_FILE`, default `./migration.config.json`)
2. Environment variables
3. Script defaults (for non-required keys only)

Use these files:

1. `migration.config.sample.json` (copy to `migration.config.json`)
2. `migration.env.sample` (optional env fallback template)
3. `config.schema.json` (contract for config keys)
4. `migration.config.md` (key-by-key reference for required/optional usage)

## Scripts

Shared helper:
1. `error_codes.ts`
   - Emits stable `[MIG_*]` failure code prefixes for machine-readable diagnostics.
   - Refer to `migration-error-codes.md` for full catalog and remediations.
2. `migration-error-registry.json`
   - Machine-readable source of truth for all migration error codes and script mappings.
3. `migration-error-registry.schema.json`
   - JSON schema contract for registry shape, code format, categories, and script list constraints.

1. `00_config_lint.ts`
   - Validates `migration.config.json` against `config.schema.json`.
2. `00_doctor.ts`
   - Runs config lint + preflight checks and writes `runtime/migration-doctor.json`.
   - Supports `--output <path>` override for artifact location.
   - Redacts sensitive settings and exits non-zero on blocking failures.
3. `00_preflight.ts`
   - Validates prerequisites, digest format, and writable output paths.
   - Modes: `general`, `reports`, `refresh`.
4. `01_build_publish.ts`
   - Builds and publishes a bundle.
   - Prints `DIGEST_REF` for downstream steps.
5. `02_run_reports.ts`
   - Prepares digest-pinned run input and runs report commands.
   - Expects your orchestrator to produce `run-report.json`.
6. `03_refresh_cycle.ts`
   - Builds/publishes a refresh bundle.
   - Supports decision reuse baseline via `REUSE_FROM_DIGEST_REF`.
   - Runs drift checks with decision index/journal artifacts.
7. `04_evidence_manifest.ts`
   - Verifies required evidence artifacts exist and computes SHA-256 checksums.
   - Emits `underlay.migration.evidence_manifest.v1` JSON contract.
   - Requires `decision_reuse_summary` artifact when `RUN_SCOPE=refresh`.
   - Requires `decision_reuse_summary_lint` artifact when `RUN_SCOPE=refresh`.
   - Fails fast if required artifacts are missing.
8. `05_evidence_verify.ts`
   - Re-hashes artifacts listed in `artifact-manifest.json`.
   - Enforces refresh-only `decision_reuse_summary` entry when `run_scope=refresh`.
   - Enforces refresh-only `decision_reuse_summary_lint` entry when `run_scope=refresh`.
   - Fails fast on tampering, missing files, or required-entry gaps.
9. `10_decision_reuse_summary.ts`
   - Generates refresh-only decision reuse summary from `run-report.json`.
   - Emits `underlay.migration.decision_reuse_summary.v1` with reuse stats, sidecar merge summary, and suppression KPIs.
10. `12_decision_reuse_summary_lint.ts`
   - Validates decision reuse summary against `decision-reuse-summary.schema.json`.
   - Emits `underlay.migration.decision_reuse_summary_lint.v1` and exits non-zero on schema/semantic drift.
11. `13_refresh_drift_gate.ts`
   - Applies threshold-based drift gate for refresh runs.
   - Emits `underlay.migration.refresh_drift_gate.v1` and exits non-zero on gate failure.
12. `14_refresh_drift_gate_lint.ts`
   - Validates refresh drift gate output against `refresh-drift-gate.schema.json`.
   - Emits `underlay.migration.refresh_drift_gate_lint.v1` and exits non-zero on schema/semantic drift.
13. `06_promotion_check.ts`
   - Runs pre-promotion gate checks as one flow.
   - Executes evidence generation + evidence verification + integrity/drift/verify/policy checks.
   - Writes `underlay.migration.promotion_check.v1` decision JSON and exits non-zero on hold.
14. `07_promotion_decision_lint.ts`
   - Validates the promotion decision JSON against `promotion-decision.schema.json`.
   - Emits `underlay.migration.promotion_decision_lint.v1` lint report JSON.
   - Fails fast on contract drift before CI/release handoff.
15. `08_promotion_release_note.ts`
   - Builds JSON + Markdown release summaries from promotion decision, decision lint, and artifact manifest.
   - Captures digest refs, gate statuses, blockers, and artifact checksums.
16. `09_promotion_ci_guard.ts`
   - Applies one canonical CI verdict over promotion decision, decision lint, and release note outputs.
   - Emits `underlay.migration.promotion_ci_guard.v1` and exits non-zero on failed verdict.
17. `11_promotion_ci_guard_lint.ts`
   - Validates promotion CI guard output against `promotion-ci-guard.schema.json`.
   - Emits `underlay.migration.promotion_ci_guard_lint.v1` and exits non-zero on schema/semantic drift.
18. `15_error_code_registry_lint.ts`
   - Validates `migration-error-registry.json` against `migration-error-registry.schema.json` and script usage in both directions.
   - Fails if registry has unused codes, scripts emit uncataloged codes, or metadata uses `TODO` placeholders.
19. `16_error_code_registry_sync.ts`
   - Regenerates `migration-error-registry.json` from script code usage while preserving existing metadata and canonical order.
   - Supports `--check` mode for CI drift detection without writing.

## Effigy integration

Use `effigy.migration.sample.toml` as a starter catalog.

```bash
cp docs/guides/code/205-legacy-migration-framework/effigy.migration.sample.toml ./effigy.toml
cp docs/guides/code/205-legacy-migration-framework/migration.config.sample.json ./migration.config.json

effigy tasks
```

Note: this sample is root-oriented. It assumes:

1. `effigy.toml` is at repo root.
2. scripts remain at `docs/guides/code/205-legacy-migration-framework/`.
3. config file is `./migration.config.json`.

## Typical usage (direct)

```bash
MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_config_lint.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_doctor.ts \
    --output ./runtime/migration-doctor.json

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode general

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/01_build_publish.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/02_run_reports.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/03_refresh_cycle.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=refresh \
  bun run ./docs/guides/code/205-legacy-migration-framework/10_decision_reuse_summary.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=refresh \
  bun run ./docs/guides/code/205-legacy-migration-framework/12_decision_reuse_summary_lint.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=refresh \
  bun run ./docs/guides/code/205-legacy-migration-framework/13_refresh_drift_gate.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=refresh \
  bun run ./docs/guides/code/205-legacy-migration-framework/14_refresh_drift_gate_lint.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=demo \
  bun run ./docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=demo \
  bun run ./docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=demo \
  bun run ./docs/guides/code/205-legacy-migration-framework/06_promotion_check.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/07_promotion_decision_lint.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/08_promotion_release_note.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/09_promotion_ci_guard.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/11_promotion_ci_guard_lint.ts

bun run ./docs/guides/code/205-legacy-migration-framework/15_error_code_registry_lint.ts

bun run ./docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts
bun run ./docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts --check
```

## Typical usage (Effigy)

```bash
effigy migration:config:lint
effigy migration:doctor
effigy migration:preflight
effigy migration:bundle
effigy migration:preflight:reports
effigy migration:reports
effigy migration:refresh
effigy migration:reuse:summary
effigy migration:reuse:summary:lint
effigy migration:refresh:drift-gate
effigy migration:refresh:drift-gate:lint
effigy migration:refresh:validate
effigy migration:evidence
effigy migration:evidence:verify
effigy migration:promote:check
effigy migration:promote:guard
effigy migration:error-codes:sync
effigy migration:error-codes:sync:check
effigy migration:error-codes:lint
effigy migration:validate
```

Promotion rule:

1. `migration:promote:check` then `migration:promote:guard` must pass before promotion gate review.

## CI artifact upload (GitHub Actions)

```yaml
- name: Promotion gate check
  run: |
    effigy migration:promote:check

- name: Promotion CI guard
  run: |
    effigy migration:promote:guard

- name: Migration error code registry lint
  run: |
    effigy migration:error-codes:sync:check
    effigy migration:error-codes:lint

# Optional explicit PR quality gate for registry metadata:
# fails on sync drift, uncataloged codes, or TODO placeholders.
- name: Migration error registry quality gate
  run: |
    bun run ./docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts --check
    bun run ./docs/guides/code/205-legacy-migration-framework/15_error_code_registry_lint.ts

- name: Refresh validation gate
  run: |
    effigy migration:refresh:validate

- name: Migration doctor
  run: |
    MIGRATION_CONFIG_FILE=./migration.config.json \
    MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
    bun run ./docs/guides/code/205-legacy-migration-framework/00_doctor.ts \
      --output ./runtime/migration-doctor.json

- name: Upload migration doctor artifact
  uses: actions/upload-artifact@v4
  with:
    name: migration-doctor
    path: ./runtime/migration-doctor.json

- name: Upload promotion artifacts
  uses: actions/upload-artifact@v4
  with:
    name: migration-promotion-artifacts
    path: |
      ./runtime/demo-pass/metadata/*.promotion-decision.json
      ./runtime/demo-pass/metadata/*.promotion-decision-lint.json
      ./runtime/demo-pass/metadata/*.promotion-release-note.json
      ./runtime/demo-pass/metadata/*.promotion-release-note.md
      ./runtime/demo-pass/metadata/*.promotion-ci-guard.json
      ./runtime/demo-pass/metadata/*.promotion-ci-guard-lint.json
      ./runtime/*/metadata/*.decision-reuse-summary.json
      ./runtime/*/metadata/*.decision-reuse-summary-lint.json
      ./runtime/*/metadata/*.refresh-drift-gate.json
      ./runtime/*/metadata/*.refresh-drift-gate-lint.json
      ./runtime/demo-pass/metadata/*.artifact-manifest.json
```
