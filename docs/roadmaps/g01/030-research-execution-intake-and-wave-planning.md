# 030 - Research Execution Intake and Wave Planning

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 029

## Overview

Turn the March 11, 2026 research corpus into an executable Underlay roadmap wave. Only schedule work backed by checked-in IDRs and current repo paths, and make consumer upgrade documentation a required dependency for all follow-on batches.

## Source of Truth Review

### Scheduled in this wave

- `docs/research/implementation-decision-records/idr-passkey-client-hooks.md`
- `docs/research/implementation-decision-records/idr-ai-runtime-resilience.md`
- `docs/research/implementation-decision-records/idr-validation-zod-integration.md`
- `docs/research/implementation-decision-records/idr-background-job-improvements.md`
- `docs/research/implementation-decision-records/idr-migration-verification-rules.md`
- `docs/research/implementation-decision-records/idr-nightfire-slash-commands.md`

### Held out of the active wave

- Broad legacy-migration expansion remains unscheduled because `docs/research/master-index.md` and `docs/research/IMPLEMENTATION_DECISION_RECORDS.md` reference `idr-migration-framework-gaps.md`, but the checked-in canonical IDR is narrower: declarative verification rules.
- Structured editor-shell and collaboration work remains unscheduled because the active research index references `idr-content-editor-abstractions.md` and `specimen-dossiers/sanity.md`, and neither artifact exists in this repo state.
- Passkey research is still execution-ready, but its IDR points at stale architecture/doc paths. That drift should be corrected during implementation rather than blocking roadmap opening.

## Scheduling Decision

### Wave 1

- [x] `g01.031` consumer upgrade and change communication contract
- [x] `g01.032` passkey client abstractions
- [x] `g01.034` cross-language validation with Zod

### Wave 2

- [x] `g01.033` AI runtime resilience middleware
- [x] `g01.035` background job reliability and observability
- [x] `g01.036` declarative migration verification rules

### Wave 3

- [x] `g01.037` Nightfire slash command palette

## Intake Completion Checklist

- [x] Compare the research inventory against checked-in source files.
- [x] Open active g01 roadmaps only for IDR-backed work with real artifacts.
- [x] Treat stale or missing research references as backlog, not active commitments.
- [x] Make consumer upgrade documentation an explicit dependency for the new wave.
- [x] Record a batch order that favors contained additive TypeScript work before heavier Rust/runtime surface changes.

## Completion

`g01.030` is complete. The next execution batch should start with `g01.031` and then take `g01.032` plus `g01.034` together as the first meaningful implementation wave.
