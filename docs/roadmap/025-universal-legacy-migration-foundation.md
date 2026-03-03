# 025 - Universal Legacy Migration Foundation

Status: Complete
Owner: Platform (Underlay + consuming apps)
Created: 2026-03-02
Depends on: 010, 016, 020, 023

## Overview

Underlay has reusable database, media, jobs, and AI runtime primitives, but no universal migration framework for moving legacy systems into new Underlay-based applications.

This roadmap defines the shared foundation for deterministic legacy migration execution across consuming projects, with project-specific transformation logic supplied via plugin boundaries instead of app-specific forks.

## Decision

- [x] Build a reusable migration core in Underlay (`underlay-migration-core`)
- [x] Keep project-specific migration semantics in project-owned plugins
- [x] Treat deterministic replay as a first-class requirement
- [x] Keep JSON contracts `snake_case`
- [x] Use UUIDv7 for new persistent migration identifiers

## Problem Statement

Legacy-to-Underlay migration is currently handled ad hoc per project and usually fails to provide:

1. Shared orchestration for cross-table transforms
2. Deterministic replay across demo/test/pre-production environments
3. Resumable execution for long-running migrations
4. Reusable interfaces for rules, AI decisions, and media attachment workflows

## Goals

1. Define stable core interfaces for end-to-end migration orchestration
2. Support table-level and cross-table transforms
3. Guarantee deterministic behavior when inputs and versions are unchanged
4. Provide explicit run checkpoints and resumability hooks
5. Keep architecture app-agnostic and composable

## Non-Goals

1. Shipping per-project legacy connectors in Underlay core
2. Replacing SQL schema migration tools (`sqlx::migrate!` remains separate)
3. Shipping a full UI review console in v1

---

## Architecture Boundaries

### New Core Crate

- `rust/crates/underlay-migration-core`

### Primary Plugin Interfaces

- `LegacySource`: project-owned extractors and source snapshots
- `MigrationPlugin`: transformation graph, mapping logic, and dependency declaration
- `DecisionResolver`: deterministic rule + AI + manual decision orchestration
- `AssetResolver`: media and file reference resolution/mapping
- `RunStore`: checkpoints, ID maps, journals, and resume metadata

### Integration Targets

- `underlay-db`: target persistence and schema-aligned writes
- `underlay-media` / `underlay-blob`: media version/object-key workflows
- `underlay-jobs`: optional asynchronous long-running execution
- `underlay-ai-runtime`: provider-agnostic AI-assisted decisioning

---

## Deterministic Execution Model

### Stage Graph

1. `extract`: source snapshot and chunk emission
2. `normalize`: canonical shape conversion for deterministic transforms
3. `transform`: plugin-defined mappings, merges, and splits
4. `decide`: rules/AI/manual decision resolution
5. `materialize`: target write operations with stable ordering
6. `assets`: media/file copy/attach and integrity checks
7. `verify`: counts, checksums, and referential assertions

### Determinism Controls

- Stable iteration order and explicit sort keys
- Canonical serialization for hashed inputs
- Version-pinned resolver and prompt metadata
- Persisted decision outputs to avoid non-deterministic re-resolution

---

## Progress Checklist

- [x] Phase 25.1 complete (core contracts and crate scaffolding)
- [x] Phase 25.2 complete (pipeline execution and stage contracts)
- [x] Phase 25.3 complete (checkpointing and resume model)
- [x] Phase 25.4 complete (verification framework and deterministic guards)

---

## Phase 25.1 - Core Contracts and Crate Scaffolding

### 25.1.1 Create migration-core crate and module layout

- [x] Add `rust/crates/underlay-migration-core`
- [x] Define top-level modules (`manifest`, `pipeline`, `plugin`, `run_store`, `errors`)
- [x] Wire crate into workspace manifests and package map docs

### 25.1.2 Define trait contracts and lifecycle hooks

- [x] Add initial trait contracts for `LegacySource`, `MigrationPlugin`, `DecisionResolver`, `AssetResolver`, and `RunStore`
- [x] Define execution context structures and deterministic metadata fields
- [x] Add typed error model for stage failure semantics

### Acceptance Criteria (Phase 25.1)

- [x] Public trait interfaces compile and are documented
- [x] Contracts are project-agnostic and do not reference app-specific tables/domains
- [x] Workspace builds with the new crate added

---

## Phase 25.2 - Pipeline Execution and Stage Contracts

### 25.2.1 Implement orchestrator skeleton

- [x] Add stage runner with explicit stage transitions and lifecycle events
- [x] Enforce deterministic stage ordering
- [x] Surface structured stage telemetry for logs/metrics

### 25.2.2 Add typed input/output envelopes per stage

- [x] Define canonical stage payload types
- [x] Add compatibility checks for stage outputs
- [x] Reject non-canonical payloads in strict mode

### Acceptance Criteria (Phase 25.2)

- [x] Pipeline can execute a no-op plugin end-to-end
- [x] Stage transitions are observable and serializable
- [x] Deterministic ordering rules are enforced

---

## Phase 25.3 - Checkpointing and Resume Model

### 25.3.1 Define run checkpoint schema

- [x] Introduce run ID (UUIDv7), stage checkpoint records, and cursor metadata
- [x] Support partial stage completion markers
- [x] Persist checkpoint provenance and version metadata

### 25.3.2 Add resume policy behavior

- [x] Resume from latest durable checkpoint
- [x] Validate compatibility of resumed run context
- [x] Fail fast when checkpoint metadata is incompatible

### Acceptance Criteria (Phase 25.3)

- [x] Interrupted runs can resume without replaying completed checkpoints
- [x] Resume behavior is deterministic and test-covered
- [x] Incompatible checkpoint reuse fails with actionable diagnostics

---

## Phase 25.4 - Verification Framework and Deterministic Guards

### 25.4.1 Add verification contracts

- [x] Add row-count, checksum, and referential integrity verification interfaces
- [x] Allow plugin-defined semantic assertions
- [x] Emit machine-readable verification summaries

### 25.4.2 Add deterministic guardrail checks

- [x] Detect non-stable iteration inputs
- [x] Detect version drift in resolver context
- [x] Fail run on deterministic contract violations

### Acceptance Criteria (Phase 25.4)

- [x] Verification reports are generated for each run
- [x] Determinism violations are surfaced as hard failures
- [x] Guardrails are documented with troubleshooting guidance

---

## Risks and Mitigations

- Risk: plugin interfaces become too narrow for complex migrations
  - Mitigation: support staged hooks and dependency graph declarations early.
- Risk: deterministic constraints over-limit practical migration workflows
  - Mitigation: define strict/compatible modes with explicit audit traces.
- Risk: overlap/confusion with existing migration SQL workflows
  - Mitigation: document boundary with `underlay-db` and SQL migrations clearly.

## Traceability Matrix (Phase -> File -> Evidence)

| Phase | Primary Implementation Files | Evidence Artifacts |
|---|---|---|
| 25.1 Core contracts | `rust/crates/underlay-migration-core/src/lib.rs`, `rust/crates/underlay-migration-core/src/plugin.rs`, `rust/crates/underlay-migration-core/src/run_store.rs` | crate-level compile + contract tests (`cargo check/test -p underlay-migration-core`) |
| 25.2 Stage orchestration | `rust/crates/underlay-migration-core/src/pipeline.rs` | deterministic stage-order tests + serialized stage transition snapshots |
| 25.3 Checkpoint/resume | `rust/crates/underlay-migration-core/src/run_store.rs`, `rust/crates/underlay-migration-core/src/pipeline.rs` | resume diagnostics in `run-report.json` and resume-path tests |
| 25.4 Verification guards | `rust/crates/underlay-migration-core/src/pipeline.rs`, `docs/guides/205-legacy-migration-framework.md` | verification artifacts + deterministic guardrail failure tests |

Operational guide mapping:
1. [205 - Legacy Migration Framework](../guides/205-legacy-migration-framework.md)
2. [Migration System Setup Playbook](../guides/code/205-legacy-migration-framework/migration-system-setup.md)

## Validation

Run scoped checks while iterating; run broader checks at phase boundaries.

```bash
# Rust (targeted)
cargo check -p underlay-migration-core --all-features
cargo test -p underlay-migration-core --all-features

# Workspace confidence checks at milestone boundaries
cargo test --all-features
bun check
```

## Completion Criteria

Roadmap 025 is complete when:

- [x] A reusable migration-core crate exists with stable trait boundaries
- [x] Stage orchestration, checkpoints, and verification contracts are implemented
- [x] Deterministic run requirements are codified and test-covered
- [x] Documentation links migration-core to existing Underlay data/media/AI foundations

## References

- [Package Map](../architecture/010-package-map.md)
- [Database & Migrations](../guides/050-database.md)
- [Media Library](../guides/077-media-library.md)
- [AI Runtime Routing](../guides/176-ai-runtime-routing.md)
