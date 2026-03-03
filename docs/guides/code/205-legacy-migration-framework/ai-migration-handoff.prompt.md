# AI Migration Handoff Prompt Template

Use this template when handing a migration pass to an AI agent.

Goal: zero ambiguity, deterministic execution, and audit-ready outputs.

## Operator Input Block

Fill all fields before running:

```text
run_scope: demo | pre_production | production
project_name: <string>
migration_plugin_version: <string>
target_schema_version: <string>
prompt_version: <string>
decision_reuse_policy: strict | compatible
bundle_ref_digest: <registry/repo@sha256:...>
reuse_from_digest_ref: <registry/repo@sha256:... or empty>
migration_config_file: <absolute or repo-relative path>
governance_policy_file: <absolute or repo-relative path>
output_dir: <absolute or repo-relative path>
run_report_path: <absolute or repo-relative path>
decision_index_path: <absolute or repo-relative path>
decision_journal_path: <absolute or repo-relative path>
app_migration_runner_cmd: <optional command>
confidence_threshold_default: 0.92
```

## Prompt Contract (Copy/Paste)

```text
You are executing an Underlay migration run. Follow all rules exactly.

Rules:
1. Use only digest-pinned bundle refs; never use mutable tags for replay.
2. Do not mutate historical decision journal entries.
3. Reuse decisions only when fingerprint compatibility checks pass.
4. If compatibility fails, invalidate and recompute only affected decisions.
5. Never include restricted secrets in prompts, decision memory, or artifacts.
6. Fail fast on integrity/governance/drift blocking issues.

Execution steps:
1. Validate config and environment prerequisites.
2. Pull or prepare run input from `bundle_ref_digest`.
3. If `reuse_from_digest_ref` is set, load prior decision index and apply reuse flow.
4. Run migration pipeline stages in fixed order:
   extract -> normalize -> transform -> decide -> materialize -> assets -> verify
5. Execute report suite:
   governance, policy, integrity, drift, verify, recovery, audit
6. Generate and verify evidence manifest (`04_evidence_manifest.ts`, `05_evidence_verify.ts`).
7. Generate refresh decision reuse summary (`10_decision_reuse_summary.ts`) and lint it (`12_decision_reuse_summary_lint.ts`) when `run_scope=refresh`.
8. Run unified promotion check (`06_promotion_check.ts`).
9. Validate promotion decision contract (`07_promotion_decision_lint.ts`).
10. Generate promotion release summary (`08_promotion_release_note.ts`).
11. Run canonical CI guard (`09_promotion_ci_guard.ts`).
12. Validate CI guard contract (`11_promotion_ci_guard_lint.ts`).
13. Produce final machine-readable artifact summary.

Effigy shortcut for refresh scope:
1. `effigy migration:refresh:validate` must run the full chain:
   `03_refresh_cycle.ts -> 10_decision_reuse_summary.ts -> 12_decision_reuse_summary_lint.ts -> 13_refresh_drift_gate.ts -> 14_refresh_drift_gate_lint.ts -> 04_evidence_manifest.ts -> 05_evidence_verify.ts`
2. `effigy migration:error-codes:sync:check` must pass before promotion recommendation.
3. `effigy migration:error-codes:lint` must pass before promotion recommendation.

Required final output:
1. Candidate digest and input digest refs used.
2. Reuse stats:
   reused_count, new_count, invalidated_count, unresolved_count
3. Sidecar merge summary (for refresh):
   deduped_count, replaced_count, invalidated_count, corrupted_count
4. AI suppression KPI summary (for refresh):
   candidate_decisions_total, reused_decisions_total, new_ai_calls_total, new_human_required_total, ai_call_suppression_ratio, reuse_ratio, human_queue_ratio
5. Gate summary:
   integrity_gate, governance_gate, drift_gate, verify_gate, recovery_status
6. Artifact paths + checksums:
   run_report, decision_index, decision_journal, verify_artifact, audit_artifact, artifact_manifest, promotion_decision
7. Recommendation:
   promote | hold | rollback
8. If hold/rollback, include exact blocking reasons and next remediation command.
9. Include any observed `MIG_*` failure codes and mapped remediation from `migration-error-codes.md`.

Hard-stop conditions:
1. Non digest-pinned bundle ref
2. Corrupt or unverifiable decision index
3. Signature/integrity requirement unmet for current run scope
4. Drift or governance blockers exceeding policy thresholds
5. Evidence verification failure (missing file or checksum mismatch)
6. Promotion decision lint failure (contract mismatch)
7. Promotion CI guard lint failure (contract or semantic mismatch)
8. Decision reuse summary lint failure for refresh runs
9. Refresh drift gate lint failure for refresh runs
```

## Expected Artifact Checklist

Require these files after each pass:

1. `migration-doctor.json`
2. `run-report.json`
3. `decision_journal.ndjson`
4. `decision_index.json`
5. `decision-reuse-summary.json` (required for refresh scope)
6. `decision-reuse-summary-lint.json` (required for refresh scope)
7. `refresh-drift-gate.json` (required for refresh scope)
8. `refresh-drift-gate-lint.json` (required for refresh scope)
9. verification artifact (`verify` command output)
10. audit artifact (`audit` command output)
11. `artifact-manifest.json` generated from `04_evidence_manifest.ts`

Canonical evidence expectations by scope are defined here:
1. `migration-evidence-matrix.md`

## Handoff Validation

Before accepting AI output, operator verifies:

1. `bundle_ref_digest` and reported digest match exactly.
2. Reuse stats align with expected source delta.
3. No blocked gates are ignored.
4. Artifact paths are real and checksums were provided.
5. Recommendation matches gate results.
6. Evidence file names and locations conform to `migration-evidence-matrix.md`.
