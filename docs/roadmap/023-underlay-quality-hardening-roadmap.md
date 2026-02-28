# 023 – Underlay Quality Hardening Roadmap

Status: Complete

## Overview

This roadmap turns the latest full-repo scan into an execution plan focused on reliability, type-safety, and maintainability. It prioritizes high-leverage work that improves consumer safety and catches regressions earlier in CI.

## Objectives

- Reduce public TypeScript `any` exposure in exported APIs.
- Raise confidence in critical TS and Rust paths with enforceable coverage gates.
- Remove panic-prone behavior in Rust runtime paths where failures can be propagated.
- Reduce duplicated tests to speed iteration and avoid drift.

## Decision

- [x] Adopt this as the next active roadmap.
- [x] Prioritize shared-library safety over app-specific features.
- [x] Execute in small PRs with validation scoped to changed areas.

## Progress Checklist

- [x] Phase 23.1 complete (TypeScript type-safety hardening)
- [x] Phase 23.2 complete (TypeScript coverage gating and targeted tests)
- [x] Phase 23.3 complete (Rust panic-path hardening)
- [x] Phase 23.4 complete (Test suite dedup and CI signal quality)

---

## Phase 23.1 – TypeScript Public API Type-Safety

### 23.1.1 Replace broad `any` declarations in component exports

- [x] Audit `ts/src/components/index.d.ts` and replace `declare const X: any` with typed component exports where possible.
- [x] For entries that cannot yet be fully typed, move to `unknown` plus explicit narrowed helper types.
- [x] Ensure exported types remain app-agnostic and reusable across consumers.

### 23.1.2 Tighten wildcard `.svelte` module declarations

- [x] Reduce broad fallback typing in `ts/src/svelte.d.ts`.
- [x] Keep named exports needed by TS entrypoints, but remove unnecessary global permissiveness.
- [x] Add focused tests/typechecks for modules relying on `<script module>` exports.

### Acceptance Criteria (Phase 23.1)

- [x] Public TS exports no longer rely on blanket `any` for core components.
- [x] `bun run check:types` passes without widening declarations to bypass errors.
- [x] No regressions in `bun check` and export checks.

---

## Phase 23.2 – TypeScript Coverage Gates and Hotspot Tests

### 23.2.1 Add enforceable coverage thresholds

- [x] Add phased coverage thresholds in `vitest.config.ts` (global + key module group targets).
- [x] Start with realistic floor above current baseline and ratchet upward each milestone.
- [x] Fail CI when thresholds are not met.

### 23.2.2 Cover highest-risk zero-coverage logic modules

- [x] Add tests for `ts/src/patterns/forms.ts`.
- [x] Add tests for `ts/src/patterns/storage.ts`.
- [x] Add tests for `ts/src/nightfire/editor/value-updates.ts`.
- [x] Continue with next pure-logic modules before UI-heavy component rendering tests.

### Acceptance Criteria (Phase 23.2)

- [x] Coverage thresholds enforced in CI.
- [x] Overall TS coverage increases from current baseline.
- [x] Critical logic modules above zero coverage with regression tests in place.

---

## Phase 23.3 – Rust Panic-Path Hardening

### 23.3.1 Remove panic-on-construction patterns in runtime code

- [x] Replace infallible-`expect` constructors with `Result`-returning constructors where appropriate (for example in `underlay-http-client`).
- [x] Avoid panics in request-id/header conversion paths (for example in `underlay-observability`).

### 23.3.2 Harden concurrent utility failure behavior

- [x] Replace panic path in single-flight wait handling with explicit error/fallback behavior where feasible.
- [x] Add targeted tests that exercise leader-drop/error scenarios.

### Acceptance Criteria (Phase 23.3)

- [x] No `expect`/`unwrap` in targeted runtime hot paths without explicit invariants documented.
- [x] `cargo test --all-features` passes.
- [x] Behavior under failure is explicit and test-covered.

---

## Phase 23.4 – Test Suite Dedup and Signal Quality

### 23.4.1 Consolidate duplicated HTTP client test suites

- [x] Merge overlapping coverage between `ts/tests/client/http.test.ts` and `ts/tests/client/http-refactored.test.ts`.
- [x] Keep one canonical suite + shared helpers in `ts/tests/utils/http-mocks.ts`.
- [x] Preserve coverage while reducing runtime and maintenance overhead.

### 23.4.2 Improve CI signal and reporting discipline

- [x] Keep validation commands aligned with `AGENTS.md` guidance.
- [x] Ensure failures are attributable (coverage gate, typecheck, unit tests) with clear output.
- [x] Document any intentional exclusions with rationale.

Intentional coverage exclusions currently enforced in `vitest.config.ts`:

- `ts/src/**/*.d.ts`: declaration surfaces are typechecked via `tsc`; runtime coverage is not meaningful.
- `ts/src/**/index.ts`: barrel files primarily re-export modules and create noisy, low-signal coverage gaps.
- `ts/src/tools/**`: CLI/guardrail tooling is validated through dedicated script execution, not library unit tests.

### Acceptance Criteria (Phase 23.4)

- [x] Single canonical HTTP client suite, no overlapping redundant cases.
- [x] CI remains green with equal or better defect detection.
- [x] Test runtime for TS suite decreases or stays stable while coverage is preserved.

### Verification Evidence (2026-02-24)

- [x] Added component-test hygiene guardrail: `ts/scripts/check-component-test-hygiene.ts`.
- [x] Added shared component setup hook: `ts/tests/setup/vitest-component.setup.ts`.
- [x] Wired hygiene guardrail into `package.json` `validate`.
- [x] Full verification command: `cargo test --all-features`.
- [x] Full verification command: `bun validate`.
- [x] Full verification command: `bun run test:components`.
- [x] Cross-repo Effigy validation closure: `docs/reports/2026-02-28-underlay-effigy-cross-repo-validation.md`.

---

## Milestone Sequence

1. Phase 23.1 (type safety): unblock stronger consumer guarantees.
2. Phase 23.2 (coverage gates): make regressions visible and enforce minimum quality.
3. Phase 23.3 (Rust panic hardening): improve runtime resilience.
4. Phase 23.4 (dedup): reduce maintenance cost after quality baselines are in place.

## Validation

Run checks scoped to changed areas during implementation:

```bash
# TypeScript/Svelte
bun check
bun run check:types
bun run check:exports
bun run test:run

# Rust
cargo test --all-features
# or targeted crates while iterating
cargo test -p <crate> --all-features
cargo check -p <crate> --all-features
```
