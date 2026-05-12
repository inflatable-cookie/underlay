# Contract: Front and Shared Read API Shapes

Status: planned
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `025-rust-app-runtime-assembly-and-router-topology.md`, `026-route-families-and-access-model.md`, `115-admin-resource-api-shapes.md`, `116-canonical-collection-routes-and-query-profiles.md`

## Purpose

Extend the API-shape contract beyond admin template pages so normal front,
public, and shared read surfaces are also declared.

This contract should define:

- front list/detail read envelopes
- canonical route families for front/shared reads
- bounded helper/read endpoint rules
- when a read surface should mirror the canonical admin route family

## Expected Sources of Truth

- front/shared read routes in the current consumer APIs
- current TS client command families
- `115` and `116` as the admin-side precedent

## Planned Scope

- front page-shaped collection and detail reads
- public/shared read helpers
- boundary between full read resources and bounded lookup helpers

## Out of Scope

- admin CRUD page shapes already covered by `115`
- auth/account workflow endpoints
- workflow mutation endpoints

## Next Task

Write the full contract from `g05.009` audit evidence.
