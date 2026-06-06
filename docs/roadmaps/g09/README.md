# g09

`g09` is the Effigy doctor warning closeout generation.

## Current State

`g08` closed the residual Rust public policy and tooling boundary hardening
lane. The next visible quality debt is structural, not API-contract drift:
`effigy doctor` reports comment-ratio and god-file warnings.

Posture: baseline-routing.

## Governing References

- [`docs/architecture/product-guardrails.md`](../../architecture/product-guardrails.md)
- [`docs/contracts/001-working-rules.md`](../../contracts/001-working-rules.md)
- [`docs/contracts/122-rust-public-api-inventory.md`](../../contracts/122-rust-public-api-inventory.md)

## Goals

- [ ] Clear or explicitly classify current Effigy doctor warning findings.
- [ ] Keep structural cleanups behavior-preserving.
- [ ] Avoid moving app-local behavior into Underlay while splitting files.
- [ ] Preserve public crate and package exports.

## Execution Plan

- [x] `g09.001`: doctor warning triage and Rust structural cleanup.
- [x] `g09.002`: TypeScript auth test god-file split batch.
- [x] `g09.003`: slugify test god-file split batch.
- [x] `g09.004`: forms test god-file split batch.
- [x] `g09.005`: i18n test god-file split batch.
- [x] `g09.006`: SvelteKit test god-file split batch.
- [x] `g09.007`: CSP test god-file split batch.
- [ ] `g09.008`: Nightfire test god-file split batch.
- [ ] `g09.009`: OAuth Rust file cleanup and doctor warning closeout decision.

## Acceptance Criteria

- [ ] `effigy doctor` warnings are reduced or explicitly retained with rationale.
- [ ] Rust and TypeScript validation gates pass for touched surfaces.
- [ ] Roadmap front doors agree on the active generation.
- [ ] No public import path changes without an explicit compatibility note.

## Current Queue

- `g09.001` is complete.
- `g09.002` is complete.
- `g09.003` is complete.
- `g09.004` is complete.
- `g09.005` is complete.
- `g09.006` is complete.
- `g09.007` is complete.
- `g09.008` is next.

## Next Task

Execute `g09.008`.
