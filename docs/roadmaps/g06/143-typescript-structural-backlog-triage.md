# g06.143 - TypeScript Structural Backlog Triage

## Why

`g06.142` cleared Rust from the Effigy doctor structural reports. The remaining
doctor backlog is TypeScript-only:

- attention markers
- comment-ratio findings
- god-files

Before splitting TS files, the queue needs a focused triage so the next batch is
chosen from behavior and public surface risk rather than file size alone.

## Goal

Classify the remaining TypeScript structural backlog and choose the next safe
repair batch.

## Scope

In scope:

- inspect current `scan.god-files`, `scan.attention-markers`, and
  `scan.comment-ratio` reports
- classify TS findings by source/test, public surface, and consumer impact
- identify the next safest TypeScript audit/split target
- update roadmap evidence with the chosen next batch

Out of scope:

- Rust remediation
- consumer app updates unless a TS public surface change is required
- broad package redesign

## Acceptance Criteria

- remaining TS doctor findings are summarized
- next TS target is selected from evidence
- consumer impact is classified
- next card is queued

## Consumer Upgrade Impact

Expected impact: none for triage.

If the selected next batch requires public TS API changes, stop and classify the
six consumer app updates before implementation.

## Current State

`g06.143` is complete.

Artifact:

- [143 artifact](./143-typescript-structural-backlog-triage-artifact.md)

## Next Task

Execute `g06.144`: HTTP client tests modularity audit.
