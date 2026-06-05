# g06.050 - Migration-Core Public Model Modularity Audit

## Why

`g06.049` tightened the devtools migration-bundle boundary. The remaining Rust
quality pressure is now more about large public model surfaces than immediate
runtime security.

`underlay-migration-core` is intentionally broad, but files such as pipeline,
decision memory, and verification rules remain large. Before splitting them,
Underlay needs a bounded audit that distinguishes useful public model breadth
from accidental god-file structure.

## Goal

Audit `underlay-migration-core` for modularity and public API shape, then choose
the next safe split or explicitly leave the model broad.

## Scope

In scope:

- inspect `underlay-migration-core` public exports, module sizes, and internal
  coupling
- classify which large modules are coherent public model families and which are
  accidental implementation clusters
- identify small mechanical splits that preserve public imports
- update the Rust public API inventory if migration-core direction changes
- separate known Effigy doctor backlog from new risks

Out of scope:

- changing migration bundle formats
- changing devtools CLI behavior
- changing consumer app migration systems
- broad migration engine redesign
- release execution or publishing

## Acceptance Criteria

- migration-core public model shape is documented from current code
- next split/leave-alone decision is explicit
- any proposed split preserves public re-exports unless classified as breaking
- validation surface for the next code batch is named

## Consumer Upgrade Impact

Expected impact: planning/audit only.

Any later source change must classify consumer impact before landing.

## Current State

`g06.050` is next after `g06.049`.

## Next Task

Execute `g06.050`: migration-core public model modularity audit.
