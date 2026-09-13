---
kind: northstar-handoff
title: "g12.002 — Adopt the Effigy-hosted lifecycle hook"
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
owner: Tom
created: 2026-09-13
updated: 2026-09-13
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom approved one Queue task per Northstar project on 2026-09-13; Underlay must retain its legitimate parallel-generation mode."
queue:
  capability: general
  skipPRReview: false
  dependsOn: [73d569cd-82c1-427d-b636-d75117bbe350]
---

## What This Thread Was Doing

The operator approved the Effigy-hosted lifecycle rollout across every Northstar
project. Underlay was held back because it legitimately runs `g11` and `g12` in
parallel while the original lifecycle schema represented one active generation.

## Why It Matters

Underlay needs deterministic hook-owned lifecycle state without publishing a
false single-generation view or coupling Queue to Northstar document structure.

## Current State

- Canonical task: [`g12.002`](../roadmaps/g12/002-adopt-effigy-hosted-lifecycle-hook.md).
- Queue prerequisite `73d569cd-82c1-427d-b636-d75117bbe350` owns Northstar's
  strict plural contract and must complete first.
- Underlay's committed roadmap mode has active generations `g11` and `g12`.
- Existing product sequencing remains authoritative.
- Queue origin is routing metadata only. Do not notify it at dispatch or
  closeout; escalate there only if the worker or coordinator needs operator input.

## Boundaries

Follow the task's two owned configuration paths, accepted plural contract,
acceptance oracle, and stop conditions exactly. Do not modify product code,
product runway decisions, Queue or Effigy source, CI/release surfaces, or any
Paseo thread/workspace.

## Important Context

The plural configuration records Underlay's already-authorized roadmap mode; it
does not grant parallel planning authority. The Queue origin is an escalation
route, not a dispatch or closeout notification target.

## Suggested Next Move

After the Queue dependency closes, add the exact portable Queue manifest and a
projection declaration covering both active generations. Validate, open a PR,
obtain independent exact-head review, merge, then let the required hook publish
terminal state and consume this handoff.

## Completion Protocol

The task completes only when Queue records accepted implementation and review,
merges the PR, and the Effigy-hosted `task.closeout` hook publishes the terminal
lifecycle record and all declared projections. No agent-authored closeout commit
is permitted.
