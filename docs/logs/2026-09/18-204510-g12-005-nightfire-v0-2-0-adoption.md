# g12.005 Nightfire v0.2.0 Adoption

Date: 2026-09-18
Roadmap: `g12.005`
Impact: deprecation

## Outcome

Underlay now consumes released Nightfire v0.2.0. Generic Rust and
TypeScript/Svelte implementations no longer live in this repository.

## Changes

- pinned Rust crate `nightfire` to Git tag `v0.2.0` / commit `1931cfc2`
- pointed `underlay-media` and `underlay-validation` Nightfire features at the
  released crate
- reduced `underlay-nightfire` to a crate-name re-export facade
- pinned `@inflatable-cookie/nightfire` to npm version `0.2.0`
- replaced fifteen historical TypeScript subpaths with package re-exports
- retained Underlay's `./nightfire/media` picker context and historical media
  editor adapter
- removed duplicate generic source and implementation-owned tests
- added direct facade identity coverage and reconciled the TypeScript quality
  profile inventory
- retargeted the Nightfire contract and guide to standalone ownership

No package or crate was published.

## Consumer Upgrade Notes

Existing imports keep working:

- Rust `underlay-nightfire` re-exports the standalone crate unchanged.
- `@inflatable-cookie/underlay/nightfire/*` re-exports the matching standalone
  package subpath.
- `@inflatable-cookie/underlay/nightfire/media` remains Underlay-owned because
  Nightfire v0.2.0 removed `./media`.

New code should import `nightfire` and
`@inflatable-cookie/nightfire/*` directly. Existing consumers do not need an
import-path change in this release. Facade retirement remains a later
caller-inventory lane.

## Validation

- `effigy rust:check`
- `effigy rust:clippy`
- `cargo test -p underlay-media --features nightfire`
- `cargo test -p underlay-validation --features nightfire`
- `cargo test -p underlay-nightfire`
- focused Nightfire unit tests: 83 passed
- focused Nightfire component tests: 9 passed
- `effigy qa`: 770 unit and 50 component tests passed; docs and Northstar gates
  passed
- `git diff --check`

## Follow-up

Publish an Underlay release tag through the operator-owned release lane.
Acowtancy g05.155 can then drop its remaining Underlay Nightfire bridges.
