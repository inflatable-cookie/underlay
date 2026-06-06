# g08

`g08` is the residual Rust public policy and tooling boundary hardening
generation.

## Current State

`g06` closed the main Rust platform-contract reset. `g07` closed the TypeScript
runtime and workflow boundary hardening thread. The remaining Rust risk is
residual public policy, option, and tooling model surface that still exposes
direct field construction where builders and accessors would make extension
safer.

Posture: baseline-routing.

## Governing References

- [`docs/architecture/product-guardrails.md`](../../architecture/product-guardrails.md)
- [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md)
- [`docs/contracts/023-release-and-compatibility-rollout.md`](../../contracts/023-release-and-compatibility-rollout.md)
- [`docs/contracts/122-rust-public-api-inventory.md`](../../contracts/122-rust-public-api-inventory.md)

## Goals

- [ ] Retire direct construction for residual Rust policy/config surfaces where
  invariants matter.
- [ ] Keep serialized DTO and report shapes stable unless a card explicitly
  classifies a breaking change.
- [ ] Prove any consumer-affecting changes across the current six-app family.
- [ ] Leave devtools-only raw CLI edges explicit when strings are the correct
  boundary.

## Execution Plan

- [x] `g08.001`: migration-core pipeline and integrity policy field retirement.
- [ ] `g08.002`: devtools bundle/seed option constructor and accessor audit.
- [ ] `g08.003`: migration-core governance/OCI/manifest policy model audit.
- [ ] `g08.004`: residual Rust public config closeout and compatibility proof.

## Acceptance Criteria

- [ ] Public policy/config types expose defaults, constructors, builders, and
  read-only accessors for app-facing use.
- [ ] Internal code does not rely on mutable public fields for policy behavior.
- [ ] Current consumers compile or are listed with explicit upgrade impact.
- [ ] `effigy rust:check`, `effigy qa:docs`, and `effigy qa:northstar` pass for
  completed code batches.

## Consumer Family

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

## Current Queue

- `g08.001` is complete.
- `g08.002` is next.

## Next Task

Continue with `g08.002`.
