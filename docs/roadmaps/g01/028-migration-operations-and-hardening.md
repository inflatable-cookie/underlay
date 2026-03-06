# 028 - Migration Operations and Hardening

Status: Complete
Owner: Platform (Underlay + consuming apps)
Created: 2026-03-02
Depends on: 025, 026, 027

## Overview

This roadmap covers operational reliability and hardening for migration execution in demo and pre-production lifecycles, including resume/recovery, drift detection, governance, and validation discipline.

## Decision

- [x] Treat resume and recovery behavior as first-class operational features
- [x] Add explicit verification and drift detection stages
- [x] Enforce bundle integrity and decision memory consistency checks
- [x] Add rollout runbook for iterative demo refresh workflows

## Goals

1. Provide resilient operator workflows for long-running migration runs
2. Detect and prevent silent drift between expected and applied outcomes
3. Add governance controls for integrity, auditability, and safety
4. Define validation matrix and completion gates for release readiness

## Non-Goals

1. Building a full managed migration SaaS in Underlay
2. Guaranteeing zero operator intervention for all projects
3. Replacing project-specific release approval processes

---

## Progress Checklist

- [x] Phase 28.1 complete (resume/checkpoint and failure recovery)
- [x] Phase 28.2 complete (verification and drift detection)
- [x] Phase 28.3 complete (security, governance, and integrity controls)
- [x] Phase 28.4 complete (rollout runbook and readiness matrix)

---

## Phase 28.1 - Resume, Checkpoint, and Failure Recovery

### 28.1.1 Harden resume semantics

- [x] Validate checkpoint compatibility before resume
- [x] Support resume from last durable successful boundary
- [x] Emit structured resume diagnostics

### 28.1.2 Add failure recovery playbooks

- [x] Define retry-safe and non-retry-safe failure classes
- [x] Add recovery actions for interrupted data and asset stages
- [x] Document operator commands for resume and cleanup

### Acceptance Criteria (Phase 28.1)

- [x] Interrupted runs resume deterministically
- [x] Recovery paths are documented and test-covered
- [x] Partial failures do not require full rerun by default

---

## Phase 28.2 - Verification Pipeline and Drift Detection

### 28.2.1 Add post-run verification pipeline

- [x] Validate row counts, checksums, and referential integrity
- [x] Validate decision journal/index consistency
- [x] Emit machine-readable verification artifacts

### 28.2.2 Add drift detection workflows

- [x] Detect drift between expected bundle manifest and applied target state
- [x] Detect drift between decision index and journal lineage
- [x] Fail promotion when drift exceeds policy thresholds

### Acceptance Criteria (Phase 28.2)

- [x] Verification artifacts are generated for each run
- [x] Drift categories are explicit and actionable
- [x] Promotion blocks on unresolved critical drift

### Implemented In This Phase Batch

1. `PipelineRunReport` now includes `resume_diagnostics` with:
   - `resume_attempted`
   - `resumed_from_stage`
   - `status`
   - `reason`
2. Resume compatibility failures now emit structured error codes:
   - `plugin_version_mismatch`
   - `target_schema_version_mismatch`
3. Drift detection contract added in migration core:
   - `DriftThresholds`
   - `DriftIssue`
   - `DriftDetectionReport`
   - `detect_drift_from_run(...)`
4. Devtools operator command added:
   - `underlay-devtools migration report drift --input <FILE> [--max-unresolved <N>] [--max-governance <N>]`
5. Deep lineage drift mode added:
   - optional decision index + journal inputs
   - fingerprint-to-decision_id lineage checks
   - bundle digest linkage checks
   - per-category summaries and remediation hints
6. Verification artifact contract added:
   - machine-readable JSON artifact including row counts, checksums, referential-integrity section, and promotion blockers
7. Devtools operator command added:
   - `underlay-devtools migration report verify --input <FILE> [--output-dir <DIR>]`

### Implemented In This Phase Batch (Recovery Hardening)

1. Stage failures now carry structured recovery metadata:
   - `failure_class` (`retry_safe|non_retry_safe`)
   - `recovery_hint`
2. Resume compatibility failures now emit explicit error codes for operator handling:
   - `plugin_version_mismatch`
   - `target_schema_version_mismatch`
3. Recovery advisory contract added for operator summaries:
   - derives actionable advisories from run outcomes and failure semantics
4. Devtools operator command added:
   - `underlay-devtools migration report recovery --input <FILE>`

### Operator Command Examples (Current)

```bash
# Summarize governance/invalidation outcomes
underlay-devtools migration report governance --input ./runtime/run-report.json --limit 10

# Evaluate governance policy compliance (owners, retention, access, redaction)
underlay-devtools migration report policy --input ./runtime/governance-policy.json

# Summarize promotion drift risk against thresholds
underlay-devtools migration report drift \
  --input ./runtime/run-report.json \
  --max-unresolved 0 \
  --max-governance 0

# Summarize recommended recovery actions after failed/partial runs
underlay-devtools migration report recovery --input ./runtime/run-report.json

# Build machine-readable verification artifact + blocker summary
underlay-devtools migration report verify --input ./runtime/run-report.json
```

---

## Phase 28.3 - Security, Governance, and Integrity Controls

### 28.3.1 Add integrity enforcement

- [x] Require digest verification before apply
- [x] Verify all bundle and sidecar checksums
- [x] Fail on signature/integrity policy violations (signing phase rollout)

### 28.3.2 Add governance and audit controls

- [x] Document data sensitivity boundaries and redaction standards
- [x] Define artifact retention and access control policy
- [x] Add audit events for critical migration operations

### Acceptance Criteria (Phase 28.3)

- [x] Integrity violations fail before writes
- [x] Governance controls are documented with ownership
- [x] Critical operations emit audit records

### Implemented In This Phase Batch (Integrity Gate)

1. Explicit integrity policy contract added:
   - `require_digest_verification`
   - `require_sidecar_checksum_verification`
   - `require_signature_verification`
2. Pre-apply integrity gate now blocks materialization when required evidence is missing.
3. Structured integrity blockers include actionable remediation hints.
4. Devtools operator command added:
   - `underlay-devtools migration report integrity --input <FILE>`
5. Audit event contract added for critical operations:
   - bundle pull
   - integrity verify
   - apply/materialize
   - resume
   - cleanup advisory
   - promotion check
6. Devtools operator command added:
   - `underlay-devtools migration report audit --input <FILE|DIR> [--output-dir <DIR>]`
7. Governance policy contract + compliance evaluator added:
   - ownership/contact requirements
   - retention baseline checks for decision/audit/verification artifacts
   - access-control baseline checks with break-glass warnings
   - redaction allow-list and forbidden-field conflict checks
8. Devtools operator command added:
   - `underlay-devtools migration report policy --input <FILE>`
9. Signature enforcement rollout gates implemented in integrity core:
   - phase-aware enforcement (`observe|enforce_preprod|enforce_all`)
   - scope-aware enforcement (`demo|pre_production|production`)
   - effective signature requirement computed before apply
10. Signer evidence fields added and surfaced in integrity artifacts/reports:
   - `signature_verified_at`
   - `signer_identity`
   - `signature_key_id`
11. Integrity gate now rejects incomplete signer evidence when signature verification is required.
12. Test coverage added for observe, pre-prod enforcement, and enforce-all behaviors.
13. Pipeline orchestrator coverage added to enforce signature rollout before materialize:
   - pre-production rollout enforcement failure path
   - incomplete signer evidence failure path

### Data Sensitivity and Redaction Standard (Governance Ownership)

Ownership model:
1. `migration_plugin_owner`: defines decision inputs/outputs and semantic dependency boundaries.
2. `security_owner`: approves redaction policy changes and forbidden-field registry.
3. `release_owner`: approves promotion gates for demo/refresh/pre-prod rollout.

Data classification:
| Class | Description | Examples | Redaction Policy |
|---|---|---|---|
| `public` | Safe for client demo visibility | course name, taxonomy labels, static URLs | no redaction required |
| `internal` | Operational metadata not intended for client exposure | legacy internal IDs, operator notes | redact in demo artifacts unless explicitly allowed |
| `sensitive` | Personal or commercially sensitive data | email, full_name, billing fields | redact by default in exported artifacts |
| `restricted` | Secrets or high-risk identifiers | access tokens, password hashes, signing keys, auth headers | never persisted in decision memory, never included in bundles |

Redaction rules:
1. `restricted` fields are always forbidden in `allowed_redacted_fields`.
2. `sensitive` fields require explicit allow-list entry and owner approval for any non-redacted usage.
3. Decision journal `outcome` payloads must contain only fields required for deterministic replay.
4. AI prompts must exclude `restricted` content and minimize `sensitive` fields to required context.
5. Audit artifacts may reference redaction events but must not include raw restricted values.

Operational controls:
1. Policy updates require dual signoff from `migration_plugin_owner` and `security_owner`.
2. Promotion is blocked if `migration report policy` returns blocking governance issues.
3. Break-glass access must emit an audit record with ticket reference and operator identity.

### Signature Verification Rollout Policy

Objective:
1. Transition `require_signature_verification` from optional to mandatory for promotion-grade runs.

Rollout phases:
| Phase | Target Date | Policy | Required Scope |
|---|---|---|---|
| `signing_observe` | through 2026-04-15 | `require_signature_verification=false` allowed; signatures collected where available | demo environments |
| `signing_enforce_preprod` | 2026-04-16 to 2026-05-31 | `require_signature_verification=true` required for pre-production promotion candidates | pre-production only |
| `signing_enforce_all` | from 2026-06-01 | `require_signature_verification=true` required for all demo refreshes and pre-production runs | all managed environments |

Gate behavior:
1. If phase policy requires signatures and signature evidence is missing/invalid, integrity gate must fail before materialize/apply.
2. Break-glass override may be used only in incident recovery with explicit `security_owner` approval and mandatory audit event.
3. Promotion tickets must record bundle digest, signer identity, signature verification timestamp, and verification result.

---

## Phase 28.4 - Rollout Runbook and Readiness Matrix

### 28.4.1 Add operator runbook for iterative demo refreshes

- [x] Document first pass demo build and publish
- [x] Document refresh pass using decision reuse from prior digest
- [x] Document pre-production promotion with frozen digest

### 28.4.2 Add validation matrix and readiness gates

- [x] Define scenario matrix (new data, changed data, assets, AI thresholds, resume)
- [x] Define required checks per milestone
- [x] Define release completion gates

### Acceptance Criteria (Phase 28.4)

- [x] End-to-end runbook supports demo -> refresh -> pre-prod promotion
- [x] Validation matrix is explicit and repeatable
- [x] Release readiness criteria are objective and documented

---

## Operator Runbook (Demo -> Refresh -> Pre-Prod)

### Milestone A - First Demo Bundle Build

1. Freeze migration plugin and schema versions for the demo candidate.
2. Build and publish the initial OCI bundle from current legacy extract.
3. Run digest-pinned replay in demo environment and capture run artifacts.
4. Review governance, integrity, drift, and verification outputs before client review.

```bash
# Build and publish first-pass bundle
underlay-devtools migration bundle build \
  --output ./dist/bundle-demo.oci \
  --source-system legacy_site \
  --target-schema-version 2026_03_demo \
  --media-dir ./legacy-export/media

underlay-devtools migration bundle publish \
  --bundle ./dist/bundle-demo.oci \
  --oci-ref registry.example.com/underlay/site-migration:demo-2026-03-02

# Pull by tag (or pre-resolve digest) and run digest-pinned replay
underlay-devtools migration run \
  --bundle registry.example.com/underlay/site-migration@sha256:<demo_digest> \
  --output ./runtime/demo-pass

# Evaluate run artifacts
underlay-devtools migration report governance --input ./runtime/demo-pass/run-report.json --limit 20
underlay-devtools migration report policy --input ./runtime/governance-policy.json
underlay-devtools migration report integrity --input ./runtime/demo-pass/run-report.json
underlay-devtools migration report drift --input ./runtime/demo-pass/run-report.json --max-unresolved 0 --max-governance 0
underlay-devtools migration report verify --input ./runtime/demo-pass/run-report.json --output-dir ./runtime/demo-pass
underlay-devtools migration report audit --input ./runtime/demo-pass --output-dir ./runtime/demo-pass
```

### Milestone B - Incremental Refresh Pass

1. Pull prior accepted bundle digest and decision sidecar index from demo baseline.
2. Re-extract current legacy state and classify entities as `unchanged`, `changed`, or `new`.
3. Run refresh with `--reuse-from <bundle@digest>` and append only new/changed decision outcomes.
4. Re-run reporting; only changed/new fingerprints should produce fresh AI or unresolved entries.
5. Publish refreshed bundle and merged decision index sidecar.

```bash
# Refresh run with deterministic decision reuse
underlay-devtools migration run \
  --bundle registry.example.com/underlay/site-migration@sha256:<refresh_input_digest> \
  --output ./runtime/refresh-pass \
  --reuse-from registry.example.com/underlay/site-migration@sha256:<accepted_demo_digest>

# Re-check lineage and promotion drift using prior decision memory artifacts
underlay-devtools migration report drift \
  --input ./runtime/refresh-pass/run-report.json \
  --max-unresolved 0 \
  --max-governance 0 \
  --max-lineage 0 \
  --decision-index ./runtime/refresh-pass/decision_index.json \
  --decision-journal ./runtime/refresh-pass/decision_journal.ndjson \
  --expected-bundle-digest sha256:<refresh_input_digest>
```

Expected refresh behavior:
1. Unchanged fingerprints reuse prior decisions with zero additional AI calls.
2. New fingerprints produce new journal entries and optional unresolved items.
3. Changed fingerprints invalidate only dependent decisions, not the full run.

### Milestone C - Pre-Production Promotion

1. Select one accepted refresh bundle digest and freeze it as promotion candidate.
2. Replay the exact digest in pre-production and compare run outputs to demo/refresh baselines.
3. Require explicit approval when all release gates are green.
4. Promote only immutable digest references; do not promote mutable tags.

Promotion gate:
1. `verify_passed=true`
2. `drift.blocking_issue_count=0`
3. `integrity_gate.passed=true`
4. `governance` has no blocking issues
5. Audit artifact present for bundle pull, integrity verify, apply, and promotion checks
6. Promotion decision contract lint passed (`07_promotion_decision_lint.ts`)
7. Promotion CI guard passed (`09_promotion_ci_guard.ts`)
8. Promotion CI guard lint passed (`11_promotion_ci_guard_lint.ts`)

### Promotion Artifact Chain (Required for Gate Eligibility)

The promotion chain is considered complete only when all artifacts exist and agree:

1. `underlay.migration.promotion_check.v1` decision artifact (`06_promotion_check.ts`)
2. `underlay.migration.promotion_decision_lint.v1` lint artifact (`07_promotion_decision_lint.ts`)
3. `underlay.migration.promotion_release_note.v1` release summary (`08_promotion_release_note.ts`)
4. `underlay.migration.promotion_ci_guard.v1` CI verdict (`09_promotion_ci_guard.ts`)
5. `underlay.migration.promotion_ci_guard_lint.v1` CI guard lint (`11_promotion_ci_guard_lint.ts`)

Hard requirements:
1. `promotion_check.recommendation=promote`
2. decision lint `status=passed` and `error_count=0`
3. CI guard `status=passed`
4. CI guard lint `status=passed` and `error_count=0`
5. release note recommendation and lint status must match source artifacts

If any requirement fails:
1. promotion is blocked
2. remediation task must be linked in release notes
3. rerun must regenerate the full artifact chain

### Failure Classes and Recovery Actions

| Failure Class | Typical Causes | Retry Safe | Recovery Action |
|---|---|---|---|
| `extract_io_failure` | source DB timeout, network reset | yes | rerun current stage or resume from checkpoint |
| `normalize_contract_error` | invalid source payload shape | no | fix plugin canonicalization; rerun from normalize |
| `decision_dependency_drift` | changed semantic dependency invalidates prior decision | yes | invalidate affected fingerprints, rerun decide stage |
| `asset_integrity_mismatch` | media shard checksum mismatch | no | rebuild/pull bundle, re-verify hashes before assets stage |
| `governance_policy_block` | retention/access/redaction violation | no | update policy/data handling, rerun reports and promotion checks |
| `signature_evidence_missing` | required signer fields absent in enforced phase | no | verify signature evidence and rerun integrity + promotion chain |
| `promotion_contract_drift` | decision/guard artifact schema mismatch | no | regenerate artifacts using current contract scripts |

Recovery rule:
1. retry-safe failures may resume from last durable checkpoint.
2. non-retry-safe failures require explicit remediation + fresh run from affected stage boundary.
3. every recovery action must emit audit events and updated release notes.

### CI and Effigy Gate Sequence (Canonical)

Use this sequence in automation to avoid partial promotion checks:

1. `effigy migration:promote:check`
2. `effigy migration:promote:guard`

`migration:promote:guard` must execute both:
1. `09_promotion_ci_guard.ts`
2. `11_promotion_ci_guard_lint.ts`

Evidence upload set for promotion jobs must include:
1. `*.promotion-decision.json`
2. `*.promotion-decision-lint.json`
3. `*.promotion-release-note.json`
4. `*.promotion-release-note.md`
5. `*.promotion-ci-guard.json`
6. `*.promotion-ci-guard-lint.json`
7. `*.artifact-manifest.json`

## Validation Matrix (Operational)

| Scenario | Input Delta | Expected Result | Required Evidence |
|---|---|---|---|
| Deterministic replay | Same bundle digest, same versions | Identical outcomes, zero net drift | Run reports + verification artifact checksum parity |
| Incremental new records | Additional legacy entities only | New decisions only for new fingerprints | Decision journal delta count equals new fingerprint count |
| Incremental changed records | Field changes in semantic dependency set | Targeted invalidation and recompute | Invalidation report with bounded impacted fingerprints |
| Prompt version bump | `prompt_version` changed for AI decision type | Affected AI decisions invalidated and recomputed | Invalidation reasons include `prompt_version_mismatch` |
| Resume after interruption | Partial run + resume checkpoint | Same final outputs as uninterrupted run | Resume diagnostics + matching verification outcome |
| Corrupted sidecar index | Tampered index payload | Hard fail before apply | Integrity gate blocker + non-zero exit |
| Human override precedence | Prior human override exists | Human outcome reused over prior AI | Provenance chain shows human winner |
| Media-heavy replay | Large multi-shard assets | Deterministic shard integrity and asset mapping | Bundle media manifest + checksum verification |

## Milestone Gates

### Gate for Demo Signoff

1. Bundle digest published and recorded in release notes.
2. Governance, integrity, drift, and verification reports generated and stored.
3. Any unresolved queue items triaged with explicit disposition (`ai_retry|human_required|deferred`).

### Gate for Refresh Acceptance

1. Decision reuse ratio meets project baseline target (default >= `0.80` for stable projects).
2. New AI calls are limited to changed/new fingerprints.
3. Human overrides from prior pass remain authoritative where fingerprints remain compatible.

### Gate for Pre-Production Promotion

1. Promotion candidate is digest-pinned and immutable.
2. All blocking checks pass with zero critical drift.
3. Audit artifacts are retained per policy and linked to release ticket.
4. Evidence manifest checksum verification passes with zero mismatches.
5. Promotion decision artifact (`underlay.migration.promotion_check.v1`) reports `recommendation=promote`.
6. Promotion decision lint artifact (`underlay.migration.promotion_decision_lint.v1`) reports `status=passed`.
7. Promotion CI guard lint artifact (`underlay.migration.promotion_ci_guard_lint.v1`) reports `status=passed`.

## Traceability Matrix (Phase -> File -> Evidence)

| Phase | Primary Implementation Files | Evidence Artifacts |
|---|---|---|
| 28.1 Resume/recovery | `rust/crates/underlay-migration-core/src/pipeline.rs`, `rust/crates/underlay-devtools/src/migration_report.rs` | `migration report recovery` output + resume diagnostics in `run-report.json` |
| 28.2 Verify/drift detection | `rust/crates/underlay-devtools/src/migration_report.rs`, `docs/guides/code/205-legacy-migration-framework/run-report.sample.json` | verify artifact, drift summary, and lineage mismatch blockers |
| 28.3 Integrity/governance/audit | `docs/guides/code/205-legacy-migration-framework/governance-policy.sample.json`, `rust/crates/underlay-devtools/src/migration_report.rs` | `migration report integrity|policy|audit` outputs |
| 28.4 Runbook/readiness | `docs/guides/205-legacy-migration-framework.md`, `docs/guides/code/205-legacy-migration-framework/migration-system-setup.md` | demo/refresh/pre-prod gate checklist completion and archived artifacts |

Runbook and handoff mapping:
1. [205 - Legacy Migration Framework](../../guides/205-legacy-migration-framework.md)
2. [Migration System Setup Playbook](../../guides/code/205-legacy-migration-framework/migration-system-setup.md)
3. [AI Migration Handoff Prompt Template](../../guides/code/205-legacy-migration-framework/ai-migration-handoff.prompt.md)
4. [Migration Evidence Matrix](../../guides/code/205-legacy-migration-framework/migration-evidence-matrix.md)

## Risks and Mitigations

- Risk: operational complexity slows adoption
  - Mitigation: phase rollout with prescriptive runbooks and CLI ergonomics.
- Risk: integrity and governance controls are bypassed in emergencies
  - Mitigation: hard guardrails in apply path plus explicit break-glass audit events.
- Risk: drift checks produce noisy false positives
  - Mitigation: define severity classes and policy thresholds with clear defaults.

## Validation

```bash
# Planned targeted checks by changed area
cargo check -p underlay-migration-core --all-features
cargo test -p underlay-migration-core --all-features
cargo test -p underlay-devtools --all-features

# Milestone boundary confidence
cargo test --all-features
bun check
```

## Completion Criteria

Roadmap 028 is complete when:

- [x] Resume and recovery behavior is deterministic and documented
- [x] Verification and drift detection guard release promotions
- [x] Integrity and governance controls are enforced in operator workflows
- [x] Validation matrix and runbooks support repeatable demo refresh cycles

## References

- [Package Map](../../architecture/010-package-map.md)
- [Database & Migrations](../../guides/050-database.md)
- [Media Library](../../guides/077-media-library.md)
- [AI Runtime Routing](../../guides/176-ai-runtime-routing.md)
