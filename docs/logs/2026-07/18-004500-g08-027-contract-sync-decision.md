# 2026-07-18 - g08.027 contract-sync decision

## Context

The Rust <-> TS <-> OpenAPI envelope "sync" was a sentence in contract `015`
("reflect changes in all three places") with no codegen and no test. Drift
already existed: `PagedListResponse` was in TS `envelopes.ts` but not the YAML.
The three `contracts/ui/poodle-*.json` audit artifacts were read by no code.

## Decision

The card framed a binary — generate the YAML from utoipa in CI, or delete it as
drift bait. Neither fit:

- **utoipa codegen** is disproportionate. The YAML is five envelope schemas with
  `paths: {}`; apps own their own paths and OpenAPI. A Rust build + generation +
  diff step in CI is heavy machinery for a tiny, rarely-changing surface.
- **Deleting** fights the YAML's documented role. It is referenced across the
  contract system (contracts `010`/`032`/`120`, architecture `015`,
  contract-index) and governed by contract `032` as the shared envelope
  declaration. Deleting cascades doc edits and drops the cross-language
  reference.

Chosen: **keep the YAML, make it honest** — close the drift and add a runnable
drift-check so "kept" stops meaning "drift bait."

## Changes

- Added `PagedListResponse` (`{ data, total, hasMore }`) to the YAML.
- `ts/tests/client/envelope-contract-drift.test.ts`: loads both
  machine-readable surfaces (OpenAPI schema names + TS envelope interfaces) and
  asserts an equal envelope set with aligned `ErrorBody` required fields. Fails
  on the exact drift class that let `PagedListResponse` diverge. No new
  dependency — text-structural parse, no YAML library (keeps g08.024 dep
  hygiene).
- Retired the orphan `poodle-*.json`. They are g01 poodle-adoption snapshots read
  by no live check (`check-poodle-prop-names.ts` does not open them), linked only
  from archival g01 roadmaps and contract `120`. Reclassified in contract `120`
  from "primary machine-readable artifacts" to "historical audit snapshots (not
  authoritative)"; kept the files so the ~13 archival g01 doc links resolve.

## Validation

- `effigy validate` clean: svelte-check 0 errors (2472 files), guardrails,
  component hygiene, 742 unit (was 739; +3 drift tests) + 33 component.
- Drift-check fails-on-disagreement verified by construction (dropping
  `PagedListResponse` from either surface breaks set equality).

## Consumer Upgrade Notes

Impact class **none**. Docs + a new test + an additive YAML schema; no code or
exported surface changed.

## Next

`g08.028` versioning and consumer-pin story.
