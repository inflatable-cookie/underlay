# g08.027 - Contract-Sync Decision

Status: done
Owner: repo maintainers
Started: 2026-07-18
Completed: 2026-07-18

## Purpose

Make the Rust <-> TS contract sync real or remove it. `contracts/openapi/underlay.openapi.yaml`
is ~40 lines of hand-written envelope schemas with `paths: {}`; the sync
mechanism is a sentence in `015` ("reflect changes in all three places"), with
no codegen and no test asserting Rust `dto.rs` <-> TS `envelopes.ts` <-> YAML
agreement. Drift already exists: `PagedListResponse` is in TS but not the YAML or
the doc. utoipa is a workspace dep but apps own their own OpenAPI. The three
`contracts/ui/poodle-*.json` files are referenced by nothing.

## Evidence

- `contracts/openapi/underlay.openapi.yaml`
- `rust/crates/underlay-core/src/dto.rs:27`, `ts/src/client/envelopes.ts`
- unused `contracts/ui/poodle-*.json`

## Governing References

- [032 OpenAPI quality and declaration](../../contracts/032-openapi-quality-and-declaration.md)
- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)

## Decision

Neither pole of the framed binary fit. **Full utoipa codegen** is
disproportionate: the YAML holds five rarely-changing envelope schemas with
`paths: {}` (apps own their own paths/utoipa), so a Rust build + generation step
in CI is heavy machinery for a tiny stable surface. **Deleting** the YAML fights
its documented role — it is referenced across the contract system (contracts
`010`/`032`/`120`, architecture `015`, contract-index) and governed by contract
`032` as the shared envelope declaration; deleting would cascade edits and lose
the cross-language reference.

Chosen third path: **keep the YAML but make it honest** — close the drift and add
a runnable drift-check so "kept" no longer means "drift bait."

## Changes

- [x] Added `PagedListResponse` (`{ data, total, hasMore }`) to the YAML,
  closing the known drift (it was in TS `envelopes.ts` only).
- [x] Added `ts/tests/client/envelope-contract-drift.test.ts`: parses both
  machine-readable surfaces (the OpenAPI YAML schemas and the TS envelope
  interfaces) and asserts they declare the same envelope set with aligned
  required fields. Fails the moment an envelope is added/renamed on one surface
  but not the other — the exact class of drift `PagedListResponse` was. Runs in
  the existing vitest CI; no new dependency (text-structural, no YAML parser).
- [x] Retired the orphan `poodle-*.json`: read by no live check (only linked from
  archival `g01` roadmaps + contract `120`). Reclassified in contract `120` from
  "primary machine-readable artifacts" to "historical audit snapshots (g01
  poodle-adoption, not authoritative)"; files retained so the ~13 archival g01
  doc links stay intact.

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [x] Drift-check fails when envelopes disagree: verified by construction
  (removing `PagedListResponse` from either surface breaks the set-equality
  assertion). 3 drift tests pass; unit suite 739 -> 742.
- [x] `effigy validate` clean (svelte-check 0 errors, guardrails, 742 unit + 33
  component).

## Stop Conditions

Decision card; surface the generate-vs-delete trade-off before implementing.

## Next Task

`g08.028` versioning and consumer-pin story.
