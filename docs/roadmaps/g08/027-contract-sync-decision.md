# g08.027 - Contract-Sync Decision

Status: planned
Owner: repo maintainers
Started:
Completed:

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

## Planned Changes

- [ ] Decide: generate the envelope YAML from utoipa in CI and diff-check against
  `envelopes.ts`, or delete the hand-maintained YAML as drift bait.
- [ ] Add `PagedListResponse` to whichever surface survives.
- [ ] Retire or wire up the orphan `poodle-*.json` audit artifacts.

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [ ] if kept: a drift-check task fails when envelopes disagree
- [ ] `effigy validate`

## Stop Conditions

Decision card; surface the generate-vs-delete trade-off before implementing.

## Next Task

`g08.028` versioning and consumer-pin story.
