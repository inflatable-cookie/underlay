# g06.063 - HTTP Error Logging Public Model Modularity Audit

## Why

After `g06.062`, `underlay-http/src/error_logging.rs` is the next large HTTP
production helper surface.

It is feature-gated and DB-facing, so the next move should classify the public
model and consumer usage before changing file shape.

## Goal

Classify the HTTP error logging public model surface and decide the safest
next structural batch.

## Scope

In scope:

- inspect `underlay-http/src/error_logging.rs` by helper family
- classify stable app-facing error log rows, filters, sink, middleware, and DB
  helper contracts
- scan the six-consumer family for feature usage and direct imports
- decide whether the next batch should split error logging, isolate DB-facing
  helpers, or defer behind a broader HTTP operational-surface checkpoint
- update the Rust public API inventory if the error logging boundary needs
  tighter wording

Out of scope:

- changing error log table schema or SQL behavior
- changing middleware request/response behavior
- changing feature flags
- changing query, cookies, pagination, CORS, or response helpers

## Acceptance Criteria

- public error logging surface is grouped by stable contract family
- consumer import paths and feature usage are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking error logging contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.063` is complete.

Artifact:

- [063 artifact](./063-http-error-logging-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.064`: HTTP error logging internal split.
