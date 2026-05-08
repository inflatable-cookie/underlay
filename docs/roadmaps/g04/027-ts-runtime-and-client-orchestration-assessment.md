# 027 - TS Runtime And Client Orchestration Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.026` repaired the TS AI and suggestion authority drift enough for the
next assessment wave to proceed honestly.

The next system family in the contract order is the retained TS runtime and
client orchestration layer, anchored by `090`.

## Goals

- assess the live TS runtime and client orchestration implementation against
  `090`
- separate true contract failures from compatibility-barrel residue or older
  packaging drift
- identify the smallest honest repair set for the retained TS runtime/client
  boundary
- leave explicit findings and a bounded next lane instead of broad TS churn

## Non-Goals

- redesigning app-local browser workflows in the same batch
- re-opening the AI/suggestion lane without a new contract failure
- skipping ahead to shared pattern assessment before the runtime/client
  boundary is clear

## Inputs

- [docs/contracts/090-ts-runtime-and-client-orchestration.md](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md)
- `ts/src/runtime/**`
- selected `ts/src/client/**`

## Exit Criteria

- the live TS runtime and client orchestration implementation is reviewed
  against `090`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the later shared-pattern assessment can proceed without ambiguity about the
  retained runtime/client boundary

## Findings

### 1. `client/route-protection.ts` does not match its own exact-match contract

Severity: high

The route-protection docs promise exact matches for plain public paths like
`/login`, with wildcard support only for entries ending in `/*`. The live
implementation uses `startsWith()` for all non-wildcard paths, so `/login`
also treats `/login-help` or `/login-anything` as public.

Evidence:

- [ts/src/client/route-protection.ts](/Users/tom/Dev/projects/underlay/ts/src/client/route-protection.ts:1)
- [docs/contracts/090-ts-runtime-and-client-orchestration.md](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md:152)

Impact:

- route-protection behavior is broader than callers are told
- apps can accidentally leave unintended paths unprotected when they expect
  exact-match semantics

### 2. `runtime/data.ts` is still a broad mixed barrel rather than one clear orchestration domain

Severity: medium

The contract already suspected this, and the code confirms it. `runtime/data`
is a public retained entrypoint, but it is really a compatibility barrel over a
large grab bag of selection history, reorder, batch, list, and pagination
controllers.

Evidence:

- [ts/src/runtime/data.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/data.ts:1)
- [docs/contracts/090-ts-runtime-and-client-orchestration.md](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md:193)

Impact:

- the public runtime domain is harder to explain as one coherent retained
  orchestration surface
- the runtime-vs-pattern authority split is still doing a lot of historical
  compatibility work rather than expressing a crisp domain

### 3. `client/types.ts` is wider than the “small shared helper layer” story

Severity: medium

The file mixes lower transport shapes, auth-facing shared types, and
soft-delete restore-blocker result guards. That is useful, but it is broader
than the current contract phrasing suggests.

Evidence:

- [ts/src/client/types.ts](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts:1)
- [ts/src/client/soft-delete.ts](/Users/tom/Dev/projects/underlay/ts/src/client/soft-delete.ts:1)
- [docs/contracts/090-ts-runtime-and-client-orchestration.md](/Users/tom/Dev/projects/underlay/docs/contracts/090-ts-runtime-and-client-orchestration.md:205)

Impact:

- `client/types.ts` still behaves like a broad browser-side holding area
- later contract work will have to decide whether some of those types belong in
  lower transport authority, auth authority, or an explicit soft-delete slice

### 4. The rest of the retained runtime/client surface is broadly aligned as compatibility/orchestration

Severity: low

Most runtime modules are exactly what `090` says they are: curated public
barrels over pattern-owned logic. The auth command/store layer, SvelteKit auth
integration, and context-aware navigation glue all look materially aligned with
the contract.

Evidence:

- [ts/src/runtime/auth.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/auth.ts:1)
- [ts/src/runtime/browser.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/browser.ts:1)
- [ts/src/runtime/navigation.ts](/Users/tom/Dev/projects/underlay/ts/src/runtime/navigation.ts:1)
- [ts/src/client/auth.ts](/Users/tom/Dev/projects/underlay/ts/src/client/auth.ts:1)
- [ts/src/client/navigation.ts](/Users/tom/Dev/projects/underlay/ts/src/client/navigation.ts:1)
- [ts/src/client/sveltekit.ts](/Users/tom/Dev/projects/underlay/ts/src/client/sveltekit.ts:1)

Impact:

- the next lane should be a bounded repair, not a broad TS runtime rewrite

## Assessment Result

The next real lane is a bounded runtime/client repair:

- fix route-protection matching so plain public paths really are exact matches
- tighten the `090` authority story around `runtime/data` and `client/types`
  so the public surface is described honestly
- leave the rest of the runtime/client layer in place unless later pattern
  assessment finds a stronger reason to split it further

## Next Task

Execute `g04.028`: repair route-protection semantics and tighten runtime/client
authority wording.
