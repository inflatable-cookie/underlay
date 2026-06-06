# g06.158 - TypeScript Comment-Ratio Cleanup

## Why

`g06.157` removed the attention-marker warning family. `effigy doctor` now
passes with two warning families left. The next cleanup target is
comment-heavy TypeScript source files.

The goal is not to strip useful API documentation blindly. The goal is to move
source files toward implementation-first readability and leave examples to
tests or guides.

## Goal

Reduce TypeScript comment-ratio warnings by trimming redundant in-source
examples and repeated explanatory comments without changing runtime behavior.

## Scope

In scope:

- inspect the six current `scan.comment-ratio` findings
- trim comments where they duplicate names, types, tests, or guides
- preserve public exports and behavior
- run focused tests for touched behavior
- run `effigy doctor`

Out of scope:

- source god-file splitting
- changing public APIs
- moving documentation into new guide files unless needed for clarity
- consumer-app changes

## Acceptance Criteria

- comment-ratio warnings are reduced or each retained warning is justified
- touched tests pass
- `effigy doctor` exits successfully
- artifact records remaining doctor warning state

## Consumer Upgrade Impact

Expected impact: none.

This should be comment-only cleanup.

## Current State

`g06.158` is ready.

## Next Task

Execute `g06.158`: TypeScript comment-ratio cleanup.
