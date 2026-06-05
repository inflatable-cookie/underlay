# g06.002 — Typed Safety Primitives

## Why

`g06.001` classified the Rust public API surface and identified the first
platform-contract migration gate: construction-boundary types for values that
must be validated before SQL, filesystem, HTTP header, or registry IO.

Raw-string APIs remain useful compatibility surfaces, but new Underlay code
should be able to carry already-validated values.

## Goal

Add the first typed safety primitive layer without breaking current consumers.

## Scope

In scope:

- SQL identifier and qualified table-name types in `underlay-db`
- blob object-key type in `underlay-blob`
- cookie name/path/domain types in `underlay-http`
- digest-pinned migration bundle reference type in `underlay-devtools`
- focused tests for each constructor and compatibility bridge

Out of scope:

- removing raw-string APIs
- changing `BlobAdapter`
- changing `AuthCookieConfig` field types
- forcing consumer migrations before additive helpers exist

## Contract References

- `122`: Rust public API inventory
- `020`: HTTP transport and server boundary
- `021`: database migration and schema workflow
- `023`: release and compatibility rollout
- `040`: storage, blob, and media systems
- `070`: Nightfire and migration systems
- `120`: tooling, testing, and contract artifacts

## Consumer Upgrade Impact

Impact classification: `additive`.

Expected consumer effect:

- existing code should keep compiling
- consumers can opt into typed constructors incrementally
- future deprecations of raw-string construction paths require separate caller
  proof and upgrade notes

## Acceptance Criteria

- `underlay-db` exports `SqlIdentifier` and `QualifiedTableName`
- `underlay-blob` exports `BlobObjectKey`
- `underlay-http` exports `CookieName`, `CookiePath`, and `CookieDomain`
- `underlay-devtools` exports `MigrationBundleRef`
- existing raw-string public APIs still compile
- focused tests cover valid and invalid values for each type family
- workspace clippy passes with warnings denied

## Current State

`g06.002` is complete.

Landed:

- `underlay-db::SqlIdentifier`
- `underlay-db::QualifiedTableName`
- `underlay-blob::BlobObjectKey`
- `underlay-http::CookieName`
- `underlay-http::CookiePath`
- `underlay-http::CookieDomain`
- `underlay-devtools::MigrationBundleRef`

The raw-string public APIs remain available. This milestone is additive.

## Next Task

Execute `g06.003`: auth/session contract reset and refresh-rotation rollout
proof.
