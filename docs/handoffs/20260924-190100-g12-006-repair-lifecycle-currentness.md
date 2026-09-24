---
title: g12.006 — Repair lifecycle currentness
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
status: ready-to-launch
owner: Tom
created: 2026-09-24
updated: 2026-09-24
base_required: pushed-main
roadmap: docs/roadmaps/g12/006-repair-lifecycle-currentness.md
queue_dispatch: northstar-queue
queue_approval: "Tom approved the six-repository currentness repair batch in the Northstar Chatterbox on 2026-09-24 ('Go'), following the route in Northstar docs/triage/20260921-083000-portfolio-currentness-debt-after-v4-rollout.md. Queue Spec 006 names this bounded repair for a consumer already left red."
queue:
  capability: mechanical
  skipPRReview: false
---

## What This Thread Was Doing

Clearing the exact lifecycle currentness findings in Underlay, so this
repository has one mechanical lifecycle authority again.

## Why It Matters

The closeout hook audits every state record's task path, so a red repository
blocks closeout for every lane in it. This is the bounded repair Queue Spec 006
names for a consumer already left red; it is not a rewrite of the roadmap.

## Current State

Underlay `main` is clean and synchronized at the planning commit containing
this handoff. The read-only audit recorded the findings in the task card's Evidence
section; re-run it rather than trusting that summary. Required sibling worktree
links: none. Use the automatic mechanical pool; frontier-worker justification:
none.

## Boundaries

Touch only the files the audit reports, plus this task and its handoff. Remove
duplicate status header lines and replace stale terminal-task frontier values
with task-free sequencing intent. Do not renumber or rename task files, do not
rewrite semantic prose, do not touch product code, releases, generation
rollover, or another repository's planning.

## Important Context

The audit is the authority, and it must report `status: ok` at the reviewed
head. A status-looking line whose ownership is ambiguous stays: return it to
Chatterbox instead of guessing. Repository docs QA may also enforce next-action,
number-collision and status-drift checks; the frontier replacement must keep all
of them green.

## Suggested Next Move

Re-run the audit first and confirm the inventory. Then work file by file:
header removals are mechanical, frontier values need task-free sequencing intent
that preserves the history around them, and each edit should leave comments,
contracts and nearby prose untouched.

## Completion Protocol

Prove `audit-currentness` clean, projection verification without drift,
documentation checks, normal QA and `git diff --check`. Commit only the audited
paths, push one non-draft PR and report `ready_for_review` through the
authenticated Queue callback; independent review uses another provider/model
identity. Do not merge, do not refresh an installed skill and do not publish a
release.
