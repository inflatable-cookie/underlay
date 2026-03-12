# Migration Documentation Map (Human + AI)

This map defines the canonical read/execute order for setting up and operating the Underlay migration framework.

Use this as the entrypoint when onboarding a new project team or spinning up an AI migration agent.

## 1. Coverage Summary

Current coverage status: `comprehensive` for foundation, setup, execution, validation, and promotion gates.

Covered areas:

1. architecture + roadmap intent
2. app-side runtime implementation boundaries
3. deterministic bundle/refresh execution paths
4. decision reuse + invalidation + KPI tracking
5. evidence manifests and verification contracts
6. promotion decision + CI guard + release note outputs
7. Effigy task wiring (single-command operator workflows)

## 2. Canonical Read Order

Read in this order:

1. `docs/roadmaps/g01/025-universal-legacy-migration-foundation.md`
2. `docs/roadmaps/g01/026-migration-bundles-and-oci-distribution.md`
3. `docs/roadmaps/g01/027-incremental-decision-memory-and-ai-reuse.md`
4. `docs/roadmaps/g01/028-migration-operations-and-hardening.md`
5. `docs/guides/205-legacy-migration-framework.md`
6. `docs/guides/code/205-legacy-migration-framework/migration-system-setup.md`
7. `docs/guides/code/205-legacy-migration-framework/README.md`
8. `docs/guides/code/205-legacy-migration-framework/migration.config.md`
9. `docs/guides/code/205-legacy-migration-framework/migration-runner-contract.md`
10. `docs/guides/code/205-legacy-migration-framework/migration-evidence-matrix.md`
11. `docs/guides/code/205-legacy-migration-framework/ai-migration-handoff.prompt.md`
12. `docs/guides/code/205-legacy-migration-framework/migration-troubleshooting.md`
13. `docs/guides/code/205-legacy-migration-framework/migration-error-codes.md`

## 3. Role-Based Entry Points

### Human operator

Read:

1. `migration-system-setup.md`
2. `README.md`
3. `migration-runner-contract.md`
4. `migration-evidence-matrix.md`
5. `migration-troubleshooting.md`
6. `migration-error-codes.md`

Run:

1. `effigy migration:doctor`
2. `effigy migration:demo`
3. `effigy migration:refresh:validate`
4. `effigy migration:promote:check`
5. `effigy migration:promote:guard`
6. `effigy migration:error-codes:sync:check`
7. `effigy migration:error-codes:lint`

### AI agent

Read:

1. `ai-migration-handoff.prompt.md`
2. `migration.config.md`
3. `migration-runner-contract.md`
4. `migration-evidence-matrix.md`
5. `migration-troubleshooting.md`
6. `migration-error-codes.md`

Execute:

1. config lint + doctor
2. refresh validation chain
3. evidence manifest + verify
4. error-code registry lint
5. promotion check + CI guard
6. structured run summary output with artifact paths

## 4. Script-to-Document Index

| Script | Primary contract doc |
|---|---|
| `00_config_lint.ts` | `migration.config.md` |
| `00_doctor.ts` | `README.md`, `migration-system-setup.md` |
| `00_preflight.ts` | `README.md`, `migration-system-setup.md` |
| `01_build_publish.ts` | `205-legacy-migration-framework.md` |
| `02_run_reports.ts` | `205-legacy-migration-framework.md`, `migration-runner-contract.md` |
| `03_refresh_cycle.ts` | `205-legacy-migration-framework.md`, `migration-system-setup.md`, `migration-runner-contract.md` |
| `10_decision_reuse_summary.ts` | `205-legacy-migration-framework.md`, `migration-evidence-matrix.md` |
| `12_decision_reuse_summary_lint.ts` | `README.md`, `migration.config.md` |
| `13_refresh_drift_gate.ts` | `README.md`, `migration.config.md` |
| `14_refresh_drift_gate_lint.ts` | `README.md`, `migration.config.md` |
| `04_evidence_manifest.ts` | `migration-evidence-matrix.md` |
| `05_evidence_verify.ts` | `migration-evidence-matrix.md` |
| `06_promotion_check.ts` | `README.md`, `205-legacy-migration-framework.md` |
| `07_promotion_decision_lint.ts` | `README.md`, `migration.config.md` |
| `08_promotion_release_note.ts` | `README.md` |
| `09_promotion_ci_guard.ts` | `README.md`, `205-legacy-migration-framework.md` |
| `11_promotion_ci_guard_lint.ts` | `README.md`, `migration.config.md` |
| `15_error_code_registry_lint.ts` | `migration-error-codes.md`, `README.md` |
| `16_error_code_registry_sync.ts` | `migration-error-codes.md`, `README.md` |

## 5. Definition of Done (Documentation)

Documentation is considered complete for a project migration when:

1. `migration.config.json` is fully populated and passes `00_config_lint.ts`.
2. One `demo` run and one `refresh` run have complete evidence artifacts.
3. Refresh evidence contains:
   1. decision reuse summary + lint
   2. drift gate + drift gate lint
4. Promotion check + CI guard outputs are present and passing.
5. AI handoff prompt is filled with project-specific values (digest refs, policy, unresolved queue owner).

## 6. Known Gaps (Current)

No blocking documentation gaps identified in this framework batch.

Optional future enhancements:

1. add copy-paste CI snippets for non-GitHub CI providers
2. add CI examples for automated code-to-remediation mapping
3. add a machine-readable runbook of code-specific retry policies
