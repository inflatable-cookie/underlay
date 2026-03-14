# 040 - Smart Skeletons

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add a higher-level `DataSkeleton` component so consuming apps can render consistent loading layouts for list, grid, table, and detail views without manually composing low-level `Skeleton` blocks every time.

## Research Basis

- `docs/roadmaps/backlog/smart-skeletons.md`
- `docs/guides/100-frontend-web.md`
- `ts/src/components/Skeleton.svelte`

## Likely Implementation Surface

- `ts/src/components/Skeleton.svelte`
- new smart-skeleton component/module(s)
- `ts/tests/components/`
- `docs/guides/100-frontend-web.md`

## Phase 40.1 - Built-In Layout Presets

- [x] Add a `DataSkeleton` component with built-in list, grid, table, and detail layouts.
- [x] Keep the first release intentionally small and app-agnostic instead of trying to model every layout shape.
- [x] Preserve the existing `Skeleton` API and visual language.

## Phase 40.2 - Reusable Preset Registry

- [x] Add a lightweight registry so apps can name reusable `DataSkeleton` presets without moving app-specific markup into Underlay.
- [x] Keep custom presets declarative and limited to built-in layout configuration in the first batch.
- [x] Add focused component tests for built-in and registered presets.

## Phase 40.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/100-frontend-web.md` with examples for common loading layouts.
- [x] Document the boundary that arbitrary custom skeleton markup remains app-owned for now.

## Deferred

- Container-based auto column detection.
- Arbitrary slot-driven skeleton layout DSL.
- Storybook or Histoire catalog work.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing `Skeleton` usage should remain valid and unchanged.
- Registered custom presets should only configure shared built-in layouts, not inject arbitrary app-specific fragments into Underlay.
- Upgrade guidance must make clear when to use `DataSkeleton` versus composing `Skeleton` manually.

## Validation

```bash
bun x vitest --config vitest.component.config.ts run ts/tests/components/data-skeleton.component.test.ts ts/tests/patterns/skeleton.component.test.ts
effigy validate
```

## Completion

Current active roadmap set is complete. Promote the next backlog item into `g01` only when the next reusable batch is ready for execution.
