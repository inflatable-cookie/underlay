# 205 - Legacy Migration Framework (End-to-End)

Status: Canonical implementation guide  
Last updated: 2026-03-02

This guide describes how to set up and operate an Underlay migration system from start to finish for:

1. Human operators
2. AI agents executing deterministic migration workflows

It is implementation-oriented and complements roadmap design docs:

1. [025 Universal Legacy Migration Foundation](../roadmaps/g01/025-universal-legacy-migration-foundation.md)
2. [026 Migration Bundles and OCI Distribution](../roadmaps/g01/026-migration-bundles-and-oci-distribution.md)
3. [027 Incremental Decision Memory and AI Reuse](../roadmaps/g01/027-incremental-decision-memory-and-ai-reuse.md)
4. [028 Migration Operations and Hardening](../roadmaps/g01/028-migration-operations-and-hardening.md)

For current Underlay-site project layout policy and Effigy integration, use
[Migration State Layout And Effigy](../usage/migration/000-state-layout-and-effigy.md).
This guide remains the detailed migration-core operations playbook.

## 1. Scope and Guarantees

This framework is designed for migrations where:

1. Legacy schemas and semantics differ significantly from target schemas.
2. Transform decisions can depend on cross-table context.
3. Large media payloads must move with data.
4. Migration passes must be repeatable for demo and pre-production cycles.

Core guarantees:

1. Deterministic stage execution order.
2. Digest-pinned replay for bundle prep/run flows.
3. Decision journaling and indexable decision reuse.
4. Governance, integrity, drift, and verification reporting.

## 2. Core Building Blocks

### 2.1 Runtime traits (`underlay-migration-core`)

Implement these traits in your consuming app:

1. `LegacySource`  
   Extract legacy records into batched JSON payloads (`LegacyRecordBatch`).
2. `MigrationPlugin`  
   Implement `normalize`, `transform`, `materialize`, and optional invalidation/semantic verification hooks.
3. `DecisionResolver`  
   Fingerprint decision input deterministically and resolve outcomes (`rule|ai|human`).
4. `AssetResolver`  
   Resolve media/assets associated with transformed data.
5. `RunStore`  
   Persist checkpoints, stage snapshots, decision journal entries, unresolved queue items, and summaries.

### 2.2 Orchestrator stage graph

Execution order is fixed:

1. `extract`
2. `normalize`
3. `transform`
4. `decide`
5. `materialize`
6. `assets`
7. `verify`

### 2.3 Operational policies

Configure through `PipelinePolicy`:

1. AI threshold policy (`default 0.92` with decision-type overrides).
2. Fail-on-unresolved behavior.
3. Optional declarative verification rules for common migration checks.
4. Integrity policy/evidence, including signature rollout phase:
   1. `observe`
   2. `enforce_preprod`
   3. `enforce_all`

## 3. Project Setup (Consuming App)

### 3.1 Suggested crate layout

```text
your-api/
  crates/
    migration/
      src/
        lib.rs
        source.rs
        plugin.rs
        decision.rs
        assets.rs
        run_store.rs
        runner.rs
```

### 3.2 Add dependencies

Add at minimum:

1. `underlay-migration-core`
2. `underlay-devtools` (CLI workflows)
3. `serde`, `serde_json`, `chrono`, `tokio`, `async-trait`

### 3.3 Minimal orchestrator wiring

```rust
use underlay_migration_core::{
    DecisionReusePolicy, MigrationContext, MigrationOrchestrator, PipelinePolicy, RunMetadata,
};

pub async fn run_migration<R>(
    run_store: &R,
) -> underlay_migration_core::MigrationResult<underlay_migration_core::PipelineRunReport>
where
    R: underlay_migration_core::RunStore,
{
    let source = crate::source::AppLegacySource::new();
    let plugin = crate::plugin::AppMigrationPlugin::new();
    let resolver = crate::decision::AppDecisionResolver::new();
    let assets = crate::assets::AppAssetResolver::new();

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);

    let ctx = MigrationContext::new(
        RunMetadata::new("app-migration-plugin-v1", "schema-v1"),
        PipelinePolicy::default(),
    );

    orchestrator
        .run(&ctx, run_store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
}
```

### 3.4 Declarative verification rules

`underlay-migration-core` now supports optional declarative verification rules in the existing
`verify` stage. These rules run before `MigrationPlugin::verify_semantics()`, and custom
verification code still runs unchanged after them.

```rust
use underlay_migration_core::{
    standard_verification_rules, MigrationContext, PipelinePolicy, RunMetadata,
    VerificationMetric,
};

let mut policy = PipelinePolicy::default();
policy.verification_rules = vec![
    standard_verification_rules::unique("id"),
    standard_verification_rules::not_null("email"),
    standard_verification_rules::referential_integrity("manager_id", "id"),
    standard_verification_rules::row_count_min(VerificationMetric::TransformRecordCount, 1),
];

let ctx = MigrationContext::new(
    RunMetadata::new("app-migration-plugin-v1", "schema-v1"),
    policy,
);
```

Supported declarative checks in this batch:

1. Row-count expectations over transform, decision, materialize, and asset totals.
2. Not-null checks for transformed record fields using dotted JSON paths.
3. Uniqueness checks for transformed record fields.
4. Referential-integrity checks where one transformed field must resolve against another field in the transformed set.

Mixed-mode rule:

1. Shared declarative rules handle common operator-facing checks.
2. `verify_semantics()` remains the place for migration-specific logic that cannot be expressed declaratively yet.
3. Verification fails if either declarative rules or plugin verification emit `error` severity issues.

### 3.5 Start-to-finish setup checklist

Use this order for new projects to avoid partial setups:

1. Implement `LegacySource`, `MigrationPlugin`, `DecisionResolver`, `AssetResolver`, `RunStore`.
2. Add deterministic fingerprinting, invalidation hooks, and decision reuse policy.
3. Wire orchestrator stage graph and run-store persistence boundaries.
4. Add bundle, preflight, report, and refresh automation (Effigy + CI).
5. Validate one demo pass and one refresh pass before pre-production.

Detailed checklist artifact for human + AI execution:
1. [migration-system-setup.md](./code/205-legacy-migration-framework/migration-system-setup.md)
2. [documentation-map.md](./code/205-legacy-migration-framework/documentation-map.md)
3. [migration-troubleshooting.md](./code/205-legacy-migration-framework/migration-troubleshooting.md)
4. [migration-error-codes.md](./code/205-legacy-migration-framework/migration-error-codes.md)

## 4. Decision Reuse Contracts

### 4.1 Deterministic decision key

`DecisionResolver::fingerprint` must be deterministic across runs for identical semantic input.

Required fingerprint input fields:

1. `canonical_decision_input`
2. `decision_type`
3. `resolver_version`
4. `prompt_version`
5. `target_schema_version`

### 4.2 Invalidation rules

Reuse must be rejected when incompatible:

1. `resolver_version` changed.
2. `prompt_version` changed for AI decisions.
3. `target_schema_version` changed.
4. Plugin invalidation hook indicates dependency drift.

### 4.3 Journal/index artifacts

Persist:

1. `decision_journal.ndjson` (append-only provenance log).
2. `decision_index.json` (fingerprint lookup map to prior decision records).

These are used for incremental passes so only `new`/`changed` fingerprints require fresh decisions.

### 4.4 Sidecar merge policy (`decision_index.json`)

When a refresh pass imports multiple prior sidecar indexes, merge behavior must be deterministic.

Merge ordering:

1. sort candidate indexes by `bundle_created_at` ascending
2. tie-break by lexical `bundle_digest`
3. evaluate compatibility and keep the latest compatible winner

Conflict rules:

1. same fingerprint + same `decision_id`:
   1. dedupe and keep one entry
2. same fingerprint + different `decision_id`:
   1. prefer newest `created_at` if compatibility checks pass
   2. if timestamps tie, prefer lexical max `decision_id`
   3. emit merge conflict audit event
3. same fingerprint + incompatible versions:
   1. reject reuse and mark for recompute (`index_merge_incompatible`)
4. fingerprint without backing journal record:
   1. treat as index corruption and fail integrity in strict mode

### 4.5 Decision reuse metrics and AI-call suppression

Refresh runs should always report reuse metrics from candidate decision population:

1. `candidate_decisions_total`
2. `reused_decisions_total`
3. `new_ai_calls_total`
4. `new_human_required_total`
5. `invalidated_decisions_total`

Derived metrics:

1. `ai_call_suppression_ratio = 1 - (new_ai_calls_total / candidate_decisions_total)`
2. `reuse_ratio = reused_decisions_total / candidate_decisions_total`
3. `human_queue_ratio = new_human_required_total / candidate_decisions_total`

Default operational targets for stable refresh cycles:

1. `ai_call_suppression_ratio >= 0.85`
2. `reuse_ratio >= 0.80`
3. `human_queue_ratio <= 0.05`

Escalation rules:

1. suppression target miss for 2 consecutive refresh passes requires plugin compatibility review
2. invalidation spike above `0.20` requires semantic dependency audit before promotion
3. release note must include KPI deltas compared with previous accepted refresh

## 5. Bundle Lifecycle (OCI)

Use `underlay-devtools` CLI for bundle packaging/distribution:

```bash
# Build
underlay-devtools migration bundle build \
  --output ./dist/migration-bundle.oci \
  --source-system legacy_site \
  --target-schema-version 2026_03_demo \
  --media-dir ./legacy-export/media

# Publish
underlay-devtools migration bundle publish \
  --bundle ./dist/migration-bundle.oci \
  --oci-ref registry.example.com/underlay/site-migration:demo-2026-03-02

# Pull
underlay-devtools migration bundle pull \
  --oci-ref registry.example.com/underlay/site-migration:demo-2026-03-02 \
  --output ./runtime/pulled-bundle
```

For run preparation, always use digest-pinned refs:

```bash
underlay-devtools migration run \
  --bundle registry.example.com/underlay/site-migration@sha256:<bundle_digest> \
  --output ./runtime/run-input
```

## 6. Reporting and Promotion Gates

Generate machine-readable evidence from `run-report.json`:

```bash
underlay-devtools migration report governance --input ./runtime/run-report.json --limit 20
underlay-devtools migration report policy --input ./runtime/governance-policy.json
underlay-devtools migration report integrity --input ./runtime/run-report.json
underlay-devtools migration report drift --input ./runtime/run-report.json --max-unresolved 0 --max-governance 0
underlay-devtools migration report verify --input ./runtime/run-report.json --output-dir ./runtime
underlay-devtools migration report recovery --input ./runtime/run-report.json
underlay-devtools migration report audit --input ./runtime --output-dir ./runtime
```

Promotion gate baseline:

1. Verification passed.
2. Drift has no blocking issues.
3. Integrity gate passed.
4. Governance policy has no blocking errors.
5. Audit artifact exists for critical operations.
6. Evidence manifest exists with checksums for required artifacts.
7. Evidence verification passes against the manifest (no missing/tampered files).
8. Promotion decision artifact reports `recommendation=promote`.

## 7. Governance Policy Template

Use `snake_case` JSON contracts.

```json
{
  "policy_id": "migration-governance-v1",
  "owners": [
    {
      "domain": "migration",
      "owner": "platform-team",
      "contact": "platform@example.com"
    }
  ],
  "retention": {
    "rules": [
      { "artifact": "decision_journal", "min_days": 365 },
      { "artifact": "decision_index", "min_days": 365 },
      { "artifact": "audit_artifact", "min_days": 365 },
      { "artifact": "verification_artifact", "min_days": 365 }
    ]
  },
  "access_control": {
    "rules": [
      {
        "artifact": "decision_journal",
        "allowed_roles": ["migration_admin"],
        "break_glass_role": "migration_break_glass"
      },
      {
        "artifact": "decision_index",
        "allowed_roles": ["migration_admin"],
        "break_glass_role": "migration_break_glass"
      },
      {
        "artifact": "audit_artifact",
        "allowed_roles": ["migration_auditor"],
        "break_glass_role": "migration_break_glass"
      },
      {
        "artifact": "verification_artifact",
        "allowed_roles": ["migration_auditor"],
        "break_glass_role": "migration_break_glass"
      }
    ]
  },
  "redaction": {
    "allowed_redacted_fields": ["email", "full_name"],
    "forbidden_redacted_fields": ["fingerprint"]
  }
}
```

## 8. Signature Rollout Configuration

Integrity policy supports three rollout phases:

1. `observe`
2. `enforce_preprod`
3. `enforce_all`

Effective signature enforcement is computed from:

1. Explicit `require_signature_verification`
2. Rollout phase
3. Run scope (`demo|pre_production|production`)

When signatures are required, evidence must include:

1. `signature_verified=true`
2. `signature_verified_at`
3. `signer_identity`
4. `signature_key_id`

## 9. AI Agent Operating Protocol

Use this section as the default prompt contract for AI agents running migrations.

### 9.1 Non-negotiable rules

1. Never use mutable tags for replay; only digest-pinned refs.
2. Never alter historical decision journal entries.
3. Never reuse decisions when compatibility checks fail.
4. Never include restricted secrets in decision memory or prompts.

### 9.2 Agent run sequence

1. Pull accepted baseline digest + decision index.
2. Build/run current pass and compute fingerprints.
3. Reuse compatible decisions.
4. Resolve only misses (`rule -> cached -> ai -> unresolved queue`).
5. Emit run report + integrity/governance/drift/verify/audit artifacts.
6. Fail promotion if any blocking gate fails.

### 9.3 Required agent output payload

Every automated pass should publish:

1. Candidate bundle digest.
2. Reuse stats (`reused`, `new`, `invalidated`, `unresolved`).
3. Sidecar merge summary (`deduped`, `replaced`, `invalidated`, `corrupted`) for refresh runs.
4. AI suppression KPI summary (`ai_call_suppression_ratio`, `reuse_ratio`, `human_queue_ratio`) for refresh runs.
5. Blocking/non-blocking gate summary.
6. Artifact paths and checksums.
7. Explicit recommendation (`promote|hold|rollback`).

## 10. Human Operator Runbook

### 10.1 First demo pass

1. Build and publish bundle.
2. Run digest-pinned prep.
3. Execute orchestrator in demo DB.
4. Review all report commands.
5. Share demo with client feedback loop.

### 10.2 Refresh pass

1. Build new bundle from latest legacy snapshot.
2. Reuse previous decision memory.
3. Confirm only changed/new fingerprints create new decisions.
4. Re-run gates.

### 10.3 Pre-production promotion

1. Freeze candidate digest.
2. Replay exact digest in pre-production.
3. Approve only if all gates pass.
4. Retain artifacts per governance policy.

## 11. Troubleshooting Matrix

1. `migration run requires digest-pinned --bundle <ref@sha256:...>`  
   Cause: tag-only bundle ref.  
   Fix: resolve/pin bundle digest and retry.
2. `integrity gate failed: ... signature_verification_required`  
   Cause: rollout phase/scope requires signature evidence.  
   Fix: verify signature and supply signature evidence fields.
3. `integrity gate failed: ... signature_evidence_incomplete`  
   Cause: signature marked true but signer metadata missing.  
   Fix: populate `signature_verified_at`, `signer_identity`, `signature_key_id`.
4. Drift blocking on lineage mismatch  
   Cause: decision index and journal out of sync or wrong expected digest.  
   Fix: rebuild index from valid journal/bundle and rerun drift check.

## 12. Validation Checklist

Before marking migration release-ready:

1. `cargo test -p underlay-migration-core --all-features`
2. `cargo test -p underlay-devtools --all-features`
3. Demo replay passes with digest pinning.
4. Refresh replay reuses expected decision set.
5. Pre-production replay passes all promotion gates.

### 12.1 Declarative verification guidance

Use declarative rules when you want repeatable, reviewable checks for common conditions:

1. unique identifiers after transform
2. required fields after normalization/transform
3. self-contained referential checks within transformed records
4. minimum or exact row-count expectations for operator signoff

Keep these out of scope for this roadmap:

1. CDC cutover integrations
2. heavyweight external validation suites
3. replacing migration-specific `verify_semantics()` logic wholesale

## 13. Related Guides

1. [050 Database & Migrations](./050-database.md)
2. [077 Media Library](./077-media-library.md)
3. [176 AI Runtime Routing](./176-ai-runtime-routing.md)
4. [185 Recipe Map and Testing Matrix](./185-recipe-map-and-testing-matrix.md)

## 14. Companion Artifacts

Use these runnable samples as starting points:

1. [governance-policy.sample.json](./code/205-legacy-migration-framework/governance-policy.sample.json)
2. [decision_journal.sample.ndjson](./code/205-legacy-migration-framework/decision_journal.sample.ndjson)
3. [decision_index.sample.json](./code/205-legacy-migration-framework/decision_index.sample.json)
4. [decision-reuse-summary.sample.json](./code/205-legacy-migration-framework/decision-reuse-summary.sample.json)
5. [decision-reuse-summary.schema.json](./code/205-legacy-migration-framework/decision-reuse-summary.schema.json)
6. [run-report.sample.json](./code/205-legacy-migration-framework/run-report.sample.json)
7. [rust_scaffold.sample.rs](./code/205-legacy-migration-framework/rust_scaffold.sample.rs)
8. [README.md](./code/205-legacy-migration-framework/README.md)
9. [00_config_lint.ts](./code/205-legacy-migration-framework/00_config_lint.ts)
10. [00_doctor.ts](./code/205-legacy-migration-framework/00_doctor.ts)
11. [00_preflight.ts](./code/205-legacy-migration-framework/00_preflight.ts)
12. [01_build_publish.ts](./code/205-legacy-migration-framework/01_build_publish.ts)
13. [02_run_reports.ts](./code/205-legacy-migration-framework/02_run_reports.ts)
14. [03_refresh_cycle.ts](./code/205-legacy-migration-framework/03_refresh_cycle.ts)
15. [effigy.migration.sample.toml](./code/205-legacy-migration-framework/effigy.migration.sample.toml)
16. [config.ts](./code/205-legacy-migration-framework/config.ts)
17. [config.schema.json](./code/205-legacy-migration-framework/config.schema.json)
18. [migration.config.sample.json](./code/205-legacy-migration-framework/migration.config.sample.json)
19. [migration.env.sample](./code/205-legacy-migration-framework/migration.env.sample)
20. [migration.config.md](./code/205-legacy-migration-framework/migration.config.md)
21. [migration-doctor.schema.json](./code/205-legacy-migration-framework/migration-doctor.schema.json)
22. [migration-system-setup.md](./code/205-legacy-migration-framework/migration-system-setup.md)
23. [ai-migration-handoff.prompt.md](./code/205-legacy-migration-framework/ai-migration-handoff.prompt.md)
24. [migration-evidence-matrix.md](./code/205-legacy-migration-framework/migration-evidence-matrix.md)
25. [10_decision_reuse_summary.ts](./code/205-legacy-migration-framework/10_decision_reuse_summary.ts)
26. [12_decision_reuse_summary_lint.ts](./code/205-legacy-migration-framework/12_decision_reuse_summary_lint.ts)
27. [13_refresh_drift_gate.ts](./code/205-legacy-migration-framework/13_refresh_drift_gate.ts)
28. [14_refresh_drift_gate_lint.ts](./code/205-legacy-migration-framework/14_refresh_drift_gate_lint.ts)
29. [refresh-drift-gate.schema.json](./code/205-legacy-migration-framework/refresh-drift-gate.schema.json)
30. [refresh-drift-gate.sample.json](./code/205-legacy-migration-framework/refresh-drift-gate.sample.json)
31. [decision-reuse-summary-lint.schema.json](./code/205-legacy-migration-framework/decision-reuse-summary-lint.schema.json)
32. [04_evidence_manifest.ts](./code/205-legacy-migration-framework/04_evidence_manifest.ts)
33. [05_evidence_verify.ts](./code/205-legacy-migration-framework/05_evidence_verify.ts)
34. [06_promotion_check.ts](./code/205-legacy-migration-framework/06_promotion_check.ts)
35. [07_promotion_decision_lint.ts](./code/205-legacy-migration-framework/07_promotion_decision_lint.ts)
36. [promotion-decision.schema.json](./code/205-legacy-migration-framework/promotion-decision.schema.json)
37. [promotion-decision-lint.schema.json](./code/205-legacy-migration-framework/promotion-decision-lint.schema.json)
38. [promotion-decision.sample.json](./code/205-legacy-migration-framework/promotion-decision.sample.json)
39. [08_promotion_release_note.ts](./code/205-legacy-migration-framework/08_promotion_release_note.ts)
40. [09_promotion_ci_guard.ts](./code/205-legacy-migration-framework/09_promotion_ci_guard.ts)
41. [promotion-ci-guard.schema.json](./code/205-legacy-migration-framework/promotion-ci-guard.schema.json)
42. [11_promotion_ci_guard_lint.ts](./code/205-legacy-migration-framework/11_promotion_ci_guard_lint.ts)
43. [error_codes.ts](./code/205-legacy-migration-framework/error_codes.ts)
44. [error_registry_shared.ts](./code/205-legacy-migration-framework/error_registry_shared.ts)
45. [migration-error-codes.md](./code/205-legacy-migration-framework/migration-error-codes.md)
46. [migration-error-registry.json](./code/205-legacy-migration-framework/migration-error-registry.json)
47. [15_error_code_registry_lint.ts](./code/205-legacy-migration-framework/15_error_code_registry_lint.ts)
48. [16_error_code_registry_sync.ts](./code/205-legacy-migration-framework/16_error_code_registry_sync.ts)
49. [migration-error-registry.schema.json](./code/205-legacy-migration-framework/migration-error-registry.schema.json)

Config loading precedence used by all TypeScript scripts:

1. `MIGRATION_CONFIG_FILE` JSON (default `./migration.config.json`)
2. Environment variables
3. Script defaults (non-required keys only)

Config source of truth for humans and agents:

1. Read `migration.config.md` before changing keys or automation wiring.

Golden-path execution:

```bash
# Optional: bootstrap local config from sample
cp ./docs/guides/code/205-legacy-migration-framework/migration.config.sample.json ./migration.config.json

# Optional but recommended: config lint + preflight checks
MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_config_lint.ts

MIGRATION_CONFIG_FILE=./migration.config.json \
MIGRATION_CONFIG_SCHEMA_FILE=./docs/guides/code/205-legacy-migration-framework/config.schema.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_doctor.ts \
    --output ./runtime/migration-doctor.json

MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode general

# Step 1: build + publish and capture digest ref
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/01_build_publish.ts

# Step 2: prepare run input and report gates
BUNDLE_REF="registry.example.com/underlay/site-migration@sha256:<digest>" \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode reports

# Step 3: run reports
BUNDLE_REF="registry.example.com/underlay/site-migration@sha256:<digest>" \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/02_run_reports.ts

# Step 4: refresh cycle with reuse baseline
REUSE_FROM_DIGEST_REF="registry.example.com/underlay/site-migration@sha256:<prior_digest>" \
APP_MIGRATION_RUNNER_CMD="cargo run -p myapp-migration -- run" \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/03_refresh_cycle.ts

# Step 5: refresh decision reuse summary
RUN_SCOPE=refresh \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/10_decision_reuse_summary.ts

# Step 6: refresh decision reuse summary lint
RUN_SCOPE=refresh \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/12_decision_reuse_summary_lint.ts

# Step 7: refresh drift gate
RUN_SCOPE=refresh \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/13_refresh_drift_gate.ts

# Step 8: refresh drift gate lint
RUN_SCOPE=refresh \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/14_refresh_drift_gate_lint.ts

# Step 9: evidence manifest (mandatory before promotion gates)
RUN_SCOPE=demo \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts

# Step 10: evidence verification (mandatory before promotion gates)
RUN_SCOPE=demo \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts

# Step 11: unified promotion gate check (writes promotion decision artifact)
RUN_SCOPE=demo \
PROJECT_NAME=acowtancy \
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/06_promotion_check.ts

# Step 12: promotion decision schema lint
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/07_promotion_decision_lint.ts

# Step 13: promotion release summary (json + markdown)
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/08_promotion_release_note.ts

# Step 14: canonical CI verdict
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/09_promotion_ci_guard.ts

# Step 15: CI guard schema + semantic lint
MIGRATION_CONFIG_FILE=./migration.config.json \
  bun run ./docs/guides/code/205-legacy-migration-framework/11_promotion_ci_guard_lint.ts
```

Effigy integration (task catalog):

```toml
[catalog]
alias = "migration"

[tasks."migration:config:lint"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json MIGRATION_CONFIG_SCHEMA_FILE={repo}/docs/guides/code/205-legacy-migration-framework/config.schema.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/00_config_lint.ts"

[tasks."migration:doctor"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json MIGRATION_CONFIG_SCHEMA_FILE={repo}/docs/guides/code/205-legacy-migration-framework/config.schema.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/00_doctor.ts"

[tasks."migration:preflight"]
run = [
  { task = "migration:config:lint" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode general" }
]

[tasks."migration:bundle"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/01_build_publish.ts {args}"

[tasks."migration:preflight:reports"]
run = [
  { task = "migration:config:lint" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode reports" }
]

[tasks."migration:reports"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/02_run_reports.ts {args}"

[tasks."migration:preflight:refresh"]
run = [
  { task = "migration:config:lint" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/00_preflight.ts --mode refresh" }
]

[tasks."migration:refresh:run"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/03_refresh_cycle.ts {args}"

[tasks."migration:reuse:summary"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/10_decision_reuse_summary.ts {args}"

[tasks."migration:reuse:summary:lint"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/12_decision_reuse_summary_lint.ts {args}"

[tasks."migration:refresh:drift-gate"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/13_refresh_drift_gate.ts {args}"

[tasks."migration:refresh:drift-gate:lint"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/14_refresh_drift_gate_lint.ts {args}"

[tasks."migration:refresh:validate"]
run = [
  { task = "migration:preflight:refresh" },
  { task = "migration:refresh:run" },
  { task = "migration:reuse:summary" },
  { task = "migration:reuse:summary:lint" },
  { task = "migration:refresh:drift-gate" },
  { task = "migration:refresh:drift-gate:lint" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=refresh bun run {repo}/docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts" }
]

[tasks."migration:evidence"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts {args}"

[tasks."migration:evidence:verify"]
run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts {args}"

[tasks."migration:promote:check"]
run = [
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/06_promotion_check.ts {args}" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/07_promotion_decision_lint.ts" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/08_promotion_release_note.ts" }
]

[tasks."migration:promote:guard"]
run = [
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/09_promotion_ci_guard.ts {args}" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json bun run {repo}/docs/guides/code/205-legacy-migration-framework/11_promotion_ci_guard_lint.ts" }
]

[tasks."migration:refresh"]
run = [
  { task = "migration:refresh:validate" }
]

[tasks."migration:error-codes:lint"]
run = "bun run {repo}/docs/guides/code/205-legacy-migration-framework/15_error_code_registry_lint.ts {args}"

[tasks."migration:error-codes:sync"]
run = "bun run {repo}/docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts {args}"

[tasks."migration:error-codes:sync:check"]
run = "bun run {repo}/docs/guides/code/205-legacy-migration-framework/16_error_code_registry_sync.ts --check {args}"

[tasks."migration:demo"]
run = [
  { task = "migration:preflight" },
  { task = "migration:bundle" },
  { task = "migration:preflight:reports" },
  { task = "migration:reports" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=demo bun run {repo}/docs/guides/code/205-legacy-migration-framework/04_evidence_manifest.ts" },
  { run = "MIGRATION_CONFIG_FILE={repo}/migration.config.json RUN_SCOPE=demo bun run {repo}/docs/guides/code/205-legacy-migration-framework/05_evidence_verify.ts" }
]

[tasks."migration:validate"]
run = [
  { task = "migration:error-codes:sync:check" },
  { task = "migration:error-codes:lint" },
  { task = "migration:promote:check" },
  { task = "migration:promote:guard" }
]
```

Run via Effigy:

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

CI artifact example (GitHub Actions):

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
      ./runtime/*/metadata/*.promotion-decision.json
      ./runtime/*/metadata/*.promotion-decision-lint.json
      ./runtime/*/metadata/*.promotion-release-note.json
      ./runtime/*/metadata/*.promotion-release-note.md
      ./runtime/*/metadata/*.promotion-ci-guard.json
      ./runtime/*/metadata/*.promotion-ci-guard-lint.json
      ./runtime/*/metadata/*.decision-reuse-summary.json
      ./runtime/*/metadata/*.decision-reuse-summary-lint.json
      ./runtime/*/metadata/*.refresh-drift-gate.json
      ./runtime/*/metadata/*.refresh-drift-gate-lint.json
      ./runtime/*/metadata/*.artifact-manifest.json
```

Effigy TOML schema mapping used in this guide:

1. `[catalog]` with required `alias` for task ownership/routing.
2. `[tasks."<name>"]` with `run = "<command>"` for direct command tasks.
3. `[tasks."<name>"]` with `run = [{ task = "..." }, ...]` for chained flows.
4. Interpolation tokens:
   1. `{repo}` for resolved catalog root.
   2. `{args}` for passthrough args.

Reference schema/docs source:
1. `../effigy/README.md`
2. `../effigy/docs/guides/022-manifest-cookbook.md`
