# g06.154 - TypeScript Structural Closeout Audit

## Why

After `g06.153`, the high-severity god-file backlog is cleared, but
`effigy doctor` still fails on standing TypeScript structural scans:

- attention markers
- comment ratio
- warning-level god-files

Before continuing into smaller warning-level splits, audit the remaining doctor
surface and choose the next reference-grade cleanup lane.

## Goal

Audit the remaining TypeScript doctor findings and produce the next bounded
cleanup card.

## Scope

In scope:

- inspect current doctor reports
- classify remaining attention-marker findings
- classify remaining comment-ratio findings
- classify warning-level god-file findings by source/test and shared-surface
  risk
- decide whether the next batch should clean doctor errors, split warning
  god-files, or close this structural lane

Out of scope:

- changing runtime behavior
- changing consumer apps
- fixing all remaining warnings in one batch

## Acceptance Criteria

- artifact records current doctor state
- artifact records whether high-severity god-files remain
- artifact proposes the next bounded cleanup card or closeout decision
- validation commands are recorded

## Consumer Upgrade Impact

Expected impact: none.

This is structural audit work.

## Current State

`g06.154` is complete.

Artifact:

- [`154-typescript-structural-closeout-audit-artifact.md`](154-typescript-structural-closeout-audit-artifact.md)

## Next Task

Execute `g06.155`: TypeScript doctor error cleanup.
