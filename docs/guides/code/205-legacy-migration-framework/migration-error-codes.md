# Migration Error Code Catalog

This catalog defines stable failure codes for migration setup, refresh validation, evidence checks, and promotion gates.

Use these codes in CI parsing, runbooks, and AI-agent remediation logic.

Machine-readable source of truth:
1. `migration-error-registry.json`
2. `migration-error-registry.schema.json`

Drift lint command:
1. `bun run ./docs/guides/code/205-legacy-migration-framework/15_error_code_registry_lint.ts`
2. `bun run ./docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts --check`

## 1. Code Prefixes

1. `MIG_CLI_*` argument/CLI contract failures
2. `MIG_CFG_*` configuration and environment contract failures
3. `MIG_EVID_*` evidence and integrity contract failures
4. `MIG_REFRESH_*` refresh-specific lint/drift contract failures
5. `MIG_PROMO_*` promotion and CI guard contract failures

## 2. Codes

| Code | Script(s) | Meaning | Typical remediation |
|---|---|---|---|
| `MIG_CLI_001` | `04_evidence_manifest.ts`, `05_evidence_verify.ts`, `06_promotion_check.ts`, `07_promotion_decision_lint.ts`, `09_promotion_ci_guard.ts`, `11_promotion_ci_guard_lint.ts`, `12_decision_reuse_summary_lint.ts`, `14_refresh_drift_gate_lint.ts`, `00_preflight.ts` | Unknown/unsupported CLI argument | Re-run with documented flags only |
| `MIG_CLI_002` | `00_doctor.ts` | `--output` missing/non-empty value required | Supply valid output file path |
| `MIG_CFG_001` | `00_config_lint.ts` | Config file missing | Create/fix `migration.config.json` path |
| `MIG_CFG_002` | `00_config_lint.ts` | Config schema file missing | Fix `MIGRATION_CONFIG_SCHEMA_FILE` path |
| `MIG_CFG_003` | `00_config_lint.ts` | Config is not JSON object | Fix root JSON shape |
| `MIG_CFG_004` | `00_config_lint.ts` | Schema root is not object | Fix schema contract file |
| `MIG_CFG_005` | `00_config_lint.ts` | Config key/value validation failed | Fix unknown keys/type/pattern violations |
| `MIG_CFG_006` | `00_preflight.ts`, `06_promotion_check.ts` | Required tool missing in `PATH` | Install tool and verify PATH |
| `MIG_CFG_007` | `00_preflight.ts` | Required source/schema values empty | Set `SOURCE_SYSTEM` and `TARGET_SCHEMA_VERSION` |
| `MIG_CFG_008` | `04_evidence_manifest.ts`, `06_promotion_check.ts` | Invalid `RUN_SCOPE` | Use `demo`, `refresh`, or `pre_production` |
| `MIG_CFG_009` | `04_evidence_manifest.ts` | Invalid `PROJECT_NAME` format | Use lowercase kebab-case |
| `MIG_CFG_010` | `04_evidence_manifest.ts` | Invalid `RUN_DATE_UTC` format | Use `YYYY-MM-DD` |
| `MIG_CFG_011` | `06_promotion_check.ts` | Invalid unsigned-integer threshold value | Fix drift threshold numeric fields |
| `MIG_CFG_012` | `00_doctor.ts` | Doctor check set failed | Inspect doctor report check failures |
| `MIG_EVID_001` | `04_evidence_manifest.ts` | `run-report.json` missing `run_id` | Fix run-report contract output |
| `MIG_EVID_002` | `04_evidence_manifest.ts` | Required artifact missing | Generate missing artifact before manifest |
| `MIG_EVID_003` | `04_evidence_manifest.ts` | Required artifact path is not file | Correct artifact path/type |
| `MIG_EVID_004` | `05_evidence_verify.ts` | Required file missing | Restore/generate referenced file |
| `MIG_EVID_005` | `05_evidence_verify.ts` | Path is not file | Correct manifest path entry |
| `MIG_EVID_006` | `05_evidence_verify.ts` | Manifest JSON shape invalid | Regenerate manifest |
| `MIG_EVID_007` | `05_evidence_verify.ts` | Unsupported manifest schema id | Use canonical manifest generator |
| `MIG_EVID_008` | `05_evidence_verify.ts` | Unsupported manifest schema version | Regenerate with current script |
| `MIG_EVID_009` | `05_evidence_verify.ts` | Manifest missing `output_dir` | Regenerate manifest |
| `MIG_EVID_010` | `05_evidence_verify.ts` | Manifest missing `artifacts` list | Regenerate manifest |
| `MIG_EVID_011` | `05_evidence_verify.ts` | Evidence verify failure summary | Inspect mismatches and rebuild manifest |
| `MIG_EVID_012` | `05_evidence_verify.ts` | Artifact checksum/entry mismatch detail | Regenerate artifact + manifest pair |
| `MIG_REFRESH_001` | `12_decision_reuse_summary_lint.ts` | Decision reuse summary lint failed | Fix summary contract/semantics then rerun |
| `MIG_REFRESH_002` | `14_refresh_drift_gate_lint.ts` | Refresh drift gate lint failed | Fix drift gate contract/semantics then rerun |
| `MIG_PROMO_001` | `06_promotion_check.ts` | Promotion blocker detected | Resolve blocking gate failures |
| `MIG_PROMO_002` | `07_promotion_decision_lint.ts` | Promotion decision lint failed | Fix decision artifact/schema drift |
| `MIG_PROMO_003` | `09_promotion_ci_guard.ts` | Promotion CI guard failed | Resolve reason list and regenerate inputs |
| `MIG_PROMO_004` | `11_promotion_ci_guard_lint.ts` | Promotion CI guard lint failed | Fix guard artifact/schema drift |

## 3. Consumption Guidance

For CI:

1. parse stderr/stdout for `\[MIG_[A-Z]+_[0-9]{3}\]`
2. map code to this table
3. display the remediation line directly in job summary
4. run `16_error_code_registry_sync.ts --check` and `15_error_code_registry_lint.ts` in PR validation to block drift, uncataloged codes, and `TODO` placeholders

For AI agents:

1. treat `MIG_EVID_*` and `MIG_PROMO_*` as hard-stop blockers
2. allow auto-remediation retries for `MIG_CLI_*` and some `MIG_CFG_*` only
3. always include final failure code list in handoff output
