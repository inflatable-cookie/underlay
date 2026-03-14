# 031 - Consumer Upgrade and Change Communication

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 030

## Overview

Establish a standing rule for Underlay: any change that alters behavior, public APIs, configuration, migrations, or recommended integration patterns must ship with an explicit consumer upgrade path for downstream apps.

## Decision

- [x] Use `docs/guides/190-upgrade-compatibility.md` as the durable cross-cutting upgrade index.
- [x] Require every new active roadmap to carry a `Consumer Upgrade Impact` section.
- [x] Start classifying changes as `additive`, `deprecation`, or `breaking`.
- [x] Add reusable release-note and upgrade-note templates for roadmap and log batches.
- [x] Expand project-sync guidance so app repos know where to start for each change area.
- [x] Add a lightweight review checklist or guardrail for missing upgrade documentation.

## Phase 31.1 - Contract Definition

- [x] Define the minimum output for every Underlay change batch:
  - impact class
  - required app actions
  - deprecation window or cutover date when relevant
  - validation commands
  - docs updated in the same batch
- [x] Seed that contract into `docs/guides/190-upgrade-compatibility.md`.
- [x] Add the same contract to roadmap/log authoring guidance where it will be hard to miss.

## Phase 31.2 - Upgrade Surfaces

- [x] Add a reusable upgrade-note template for feature batches and release logs.
- [x] Update `docs/guides/200-project-sync.md` to point apps at the right upgrade entrypoints by subsystem.
- [x] Define how package, crate, migration, and docs-only changes should be summarized for consuming apps.

## Phase 31.3 - Enforcement

- [x] Require active roadmap batches to keep their upgrade note references current.
- [x] Require release logs for behavior-changing work to link the upgrade note or compatibility section.
- [x] Decide whether documentation completeness should be checked by policy only or by a repository script.

Policy decision:

- Start with documentation policy and review-checklist enforcement.
- Revisit repository-script enforcement only if repeated misses survive the new roadmap/log guidance.

## Acceptance Criteria

- [x] `docs/guides/190-upgrade-compatibility.md` is the clear first stop for app upgrades.
- [x] Every active roadmap opened from this wave includes consumer upgrade requirements.
- [x] Future behavior changes can be summarized for consuming apps without reconstructing intent from scattered docs or code diffs.

## Validation

Run after roadmap and guide updates land:

```bash
effigy validate
```

## Next Task

`g01.031` is complete. Next take `g01.032` and `g01.034` together as the first implementation batch so the new upgrade-documentation contract is exercised on real feature work.
