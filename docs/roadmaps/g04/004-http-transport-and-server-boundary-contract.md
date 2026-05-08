# 004 - HTTP Transport And Server Boundary Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.003` settled the primitive layer. The next dependency is the transport
and server boundary that turns those primitives into actual HTTP behavior
across Rust and TypeScript.

## Goals

- define the canonical HTTP transport contract across shared Rust and TS
- cover response helpers, error helpers, query parsing, pagination boundaries,
  cookies, and server-facing client helpers
- make the shared transport surface explicit before auth and runtime contracts
  build on it

## Non-Goals

- auth/session workflow rules beyond the transport seam
- app-specific server adapters
- implementation fixes beyond light authority repair needed to write the
  contract

## Inputs

- [`docs/contracts/010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- `rust/crates/underlay-http/**`
- `rust/crates/underlay-http-client/**`
- `ts/src/client/**`
- `ts/src/server/**`

## Outputs

- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- refreshed contract and roadmap front doors so `g04` now points at the auth
  lane

## Outcome

The transport contract now exists.

It settles:

- the shared Rust and TS HTTP helper boundary
- canonical sort/filter/page/limit query vocabulary
- the distinct `ListResponse<T>` and `Paginated<T>` transport roles
- cookie, context, retry, CORS, caching, and CSP/security-header seams

It also records current drift to assess later, especially the split Rust/TS
query ownership and the still-murky transport ownership of TS pagination
exports.

## Next Task

Execute `g04.005`: write `030-auth-and-session-systems.md`.
