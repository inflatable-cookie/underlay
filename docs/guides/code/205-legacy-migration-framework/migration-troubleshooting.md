# Migration Troubleshooting Matrix

Use this guide when any migration setup, refresh validation, or promotion gate fails.

It is designed for both human operators and AI agents.

Canonical code reference:
1. `migration-error-codes.md`

## 1. First 5 Diagnostic Commands

Run these commands in order before deeper investigation:

```bash
MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_doctor.ts \
    --output ./runtime/migration-doctor.json

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode refresh

effigy migration:refresh:validate

MIGRATION_CONFIG_FILE=./migration.config.json \
RUN_SCOPE=refresh \
  bun run ./docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/09_promotion_ci_guard.ts
```

Registry drift check:

```bash
bun run ./docs/guides/code/205-legacy-migration-framework/15_error_code_registry_lint.ts
bun run ./docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts --check
```

## 2. Failure Matrix

| Failing step | Primary code(s) | Typical error signal | Likely cause | Remediation |
|---|---|---|---|---|
| `00_config_lint.ts` | `MIG_CFG_001..005` | `unknown key`, `must be string`, `pattern` mismatch | Invalid `migration.config.json` keys/values | Align config with `config.schema.json`; re-run config lint first |
| `00_doctor.ts` | `MIG_CLI_002`, `MIG_CFG_012` | `doctor checks failed` | Missing config, missing required tools, unwritable output path | Fix tool availability and filesystem permissions; confirm all required config keys are present |
| `00_preflight.ts --mode refresh` | `MIG_CLI_001`, `MIG_CFG_006..007` | missing digest/pin/path checks | Refresh prerequisites not satisfied | Ensure `REUSE_FROM_DIGEST_REF` is digest-pinned and required refresh artifacts are configured |
| `03_refresh_cycle.ts` | upstream runner-specific | refresh run exits non-zero | Upstream migration runner failure or invalid reuse input | Inspect run logs and `run-report.json`; verify decision index/journal refs and digest inputs |
| `10_decision_reuse_summary.ts` | runner-contract specific | missing `run-report.json` fields | Runner output missing reuse counters/sections | Update migration runner output contract to include expected decision reuse metrics |
| `12_decision_reuse_summary_lint.ts` | `MIG_REFRESH_001` | schema/semantic lint failed | Summary JSON contract drift or invalid KPI values | Regenerate summary from valid report; fix field names/types and KPI math |
| `13_refresh_drift_gate.ts` | gate-status specific | `status=failed` with drift threshold breach | Drift or invalidation exceeded configured threshold | Review changed entities and invalidation spikes; adjust plugin logic or threshold policy with explicit approval |
| `14_refresh_drift_gate_lint.ts` | `MIG_REFRESH_002` | gate lint failed (`status/exit_code` mismatch, schema errors) | Invalid gate artifact content or incompatible schema | Regenerate drift gate output and ensure schema version matches configured gate schema |
| `04_evidence_manifest.ts` | `MIG_CLI_001`, `MIG_CFG_008..010`, `MIG_EVID_001..003` | missing required artifact entries | Refresh evidence set incomplete | Ensure refresh includes summary + summary lint + drift gate + drift gate lint artifacts |
| `05_evidence_verify.ts` | `MIG_CLI_001`, `MIG_EVID_004..012` | checksum mismatch / missing file | Tampered or moved artifact, incorrect manifest paths | Rebuild manifest from current artifact set; keep artifact paths immutable after manifest generation |
| `06_promotion_check.ts` | `MIG_CLI_001`, `MIG_CFG_006/008/011`, `MIG_PROMO_001` | promotion decision `hold` | One or more blocking gates failed | Resolve blocking gate outputs and regenerate promotion decision artifact |
| `07_promotion_decision_lint.ts` | `MIG_CLI_001`, `MIG_PROMO_002` | decision schema lint failed | Promotion decision contract drift | Recreate promotion decision JSON from canonical script output and re-run lint |
| `09_promotion_ci_guard.ts` | `MIG_CLI_001`, `MIG_PROMO_003` | `verdict=fail` | Decision lint failed, release note missing, or blockers present | Regenerate decision/lint/release-note artifacts and ensure blocker-free promotion status |
| `11_promotion_ci_guard_lint.ts` | `MIG_CLI_001`, `MIG_PROMO_004` | CI guard lint failed | CI guard artifact contract drift | Re-run CI guard generator and validate against `promotion-ci-guard.schema.json` |

## 3. Refresh-Specific Triage

When refresh validation fails, inspect in this order:

1. `run-report.json` decision reuse counters and invalidation values.
2. `decision-reuse-summary.json` and `decision-reuse-summary-lint.json`.
3. `refresh-drift-gate.json` and `refresh-drift-gate-lint.json`.
4. `artifact-manifest.json` entries for all refresh-required artifacts.
5. `05_evidence_verify.ts` output for integrity mismatches.

## 4. Promotion-Specific Triage

When promotion guard fails, inspect in this order:

1. promotion decision (`*.promotion-decision.json`) for blocking statuses.
2. promotion decision lint (`*.promotion-decision-lint.json`) for contract failures.
3. release note outputs (`*.promotion-release-note.json`, `*.promotion-release-note.md`).
4. CI guard output (`*.promotion-ci-guard.json`) and lint output (`*.promotion-ci-guard-lint.json`).

## 5. Escalation Rules

Escalate to plugin/ruleset review when any applies:

1. `ai_call_suppression_ratio` misses target for 2 consecutive refresh passes.
2. invalidation spike exceeds `0.20` in refresh pass.
3. evidence verification fails repeatedly after artifact regeneration.

Escalate to platform/security review when any applies:

1. digest-pinned replay contract cannot be enforced.
2. repeated artifact integrity mismatches indicate potential tampering.
3. governance policy report blocks promotion due to signature phase requirements.
