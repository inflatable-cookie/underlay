# 2026-04-10 02:45 - g02.004 Songsprout gate open

## Summary

Opened `g02.004` as the next bounded downstream consumer gate after the
completed `compli-me/admin` rollout.

## Why this mattered

`g02.003` closed the first downstream generalization check. The next step
needed to stay bounded and evidence-led rather than resuming broad multi-app
execution by habit.

## Changes

- created `g02.004` as the Songsprout rollout gate
- scoped the gate to `greenhouse` and `bloom`
- left `stem` and Rust route work explicitly out of scope
- recorded the initial Songsprout family split so the next execution starts
  from a real route inventory instead of a generic “Songsprout” label

## Validation

- local roadmap/front-door review in
  `~/Dev/projects/underlay/docs/roadmaps/g02`

## Next Task

Execute `g02.004` Batch 4.1: inventory the active `songsprout/greenhouse` and
`songsprout/bloom` route families against the frozen proof-app pattern set,
then classify each family as direct rollout, local exception, or deferred.
