# g06.159 - TypeScript Source God-File Audit

## Why

`g06.158` cleared the comment-ratio warning family. `effigy doctor` now passes
with only one warning family left: `scan.god-files`.

The remaining findings mix source files and test files. The source files are
the meaningful reference-grade risk because they may hide multiple
responsibilities behind broad public surfaces.

## Goal

Audit the remaining TypeScript source god-files and decide which ones need
bounded splits before the TypeScript structural lane can close.

## Scope

In scope:

- inspect current `scan.god-files` detail report
- classify source findings by responsibility boundaries
- decide which source files need split cards
- defer or justify test-only large-file warnings
- record final doctor state

Out of scope:

- splitting files in this decision batch
- changing public APIs
- changing consumer apps
- Rust cleanup

## Acceptance Criteria

- every source god-file warning is classified
- every test god-file warning is classified or deferred with rationale
- required splits are represented as bounded follow-up cards
- `effigy doctor` state is recorded

## Consumer Upgrade Impact

Expected impact: none.

This is an audit batch. Any later source split must classify impact separately.

## Current State

`g06.159` is ready.

## Next Task

Execute `g06.159`: TypeScript source god-file audit.
