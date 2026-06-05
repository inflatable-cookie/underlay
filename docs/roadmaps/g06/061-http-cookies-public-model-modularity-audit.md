# g06.061 - HTTP Cookies Public Model Modularity Audit

## Why

After `g06.060`, `underlay-http/src/cookies.rs` is the next high-value HTTP
helper surface to inspect before any further split.

Cookies are app-facing auth and CSRF infrastructure, so the next move should
classify the public model and consumer usage before changing file shape.

## Goal

Classify the HTTP cookies public model surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-http/src/cookies.rs` by helper family
- classify stable app-facing cookie config, typed fields, setters, clearers,
  and extractors
- scan the six-consumer family for direct cookie imports and helper usage
- decide whether the next batch should split cookies, tighten public contracts,
  or defer behind a broader HTTP boundary checkpoint
- update the Rust public API inventory if the cookie boundary needs tighter
  wording

Out of scope:

- changing cookie construction semantics
- changing SameSite/Secure/domain/path/name validation behavior
- changing auth or CSRF cookie names, paths, or clear behavior
- changing query, pagination, CORS, or error logging behavior

## Acceptance Criteria

- public cookie surface is grouped by stable contract family
- consumer import paths and helper usage are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking cookie contract change
must be promoted into an explicit follow-up card before execution.

## Current State

`g06.061` is next after `g06.060`.

## Next Task

Execute `g06.061`: HTTP cookies public model modularity audit.
