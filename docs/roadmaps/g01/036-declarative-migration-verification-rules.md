# 036 - Declarative Migration Verification Rules

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add reusable declarative verification rules to `underlay-migration-core` so migrations can express common checks more readably while preserving existing custom verification hooks.

## Research Basis

- `docs/research/implementation-decision-records/idr-migration-verification-rules.md`
- `docs/research/translation-memos/migration-framework-enhancements.md`
- `docs/research/value-tracks/legacy-migration-patterns.md`
- `docs/research/specimen-dossiers/dbt.md`
- `docs/research/specimen-dossiers/airbyte.md`
- `docs/research/specimen-dossiers/debezium.md`
- `docs/guides/205-legacy-migration-framework.md`

## Likely Implementation Surface

- `rust/crates/underlay-migration-core/src/`
- verification policy and orchestration modules
- `docs/guides/205-legacy-migration-framework.md`

## Phase 36.1 - Rule Engine Core

- [x] Add a generic `VerificationRule` model and evaluation engine.
- [x] Support common expectation types without blocking future custom extensions.
- [x] Benchmark representative rule execution against the current custom-verification path.

## Phase 36.2 - Standard Rule Library

- [x] Add shared helpers for uniqueness, nullability, row-count, and referential-integrity checks.
- [x] Integrate declarative rules into the existing verification stage without removing custom code.
- [x] Ensure failures produce readable, migration-operator-friendly output.

## Phase 36.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update the migration framework guide with declarative examples and mixed-mode verification guidance.
- [x] Keep CDC and broader migration-framework expansion explicitly out of scope for this roadmap.

## Deferred

- Built-in CDC integrations.
- Great Expectations or other heavyweight external validation integrations.
- A wider migration-framework roadmap until the research corpus has a canonical IDR for it.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing custom verification code must keep working unchanged after upgrade.
- Any new configuration or policy fields must be documented with default behavior and an example migration.
- If verification output format changes materially, document the before/after expectation for operators and CI consumers.

## Validation

```bash
cargo check -p underlay-migration-core --all-features
cargo test -p underlay-migration-core --all-features
effigy validate --repo .
```

## Next Task

Roadmap complete on 2026-03-11. Next broad batch: take `g01.037` for the Nightfire slash-command palette, or close the remaining `In progress` roadmaps if you want the current wave fully settled first.
