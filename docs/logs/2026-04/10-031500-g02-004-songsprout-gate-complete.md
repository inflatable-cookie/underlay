# 2026-04-10 03:15 - g02.004 Songsprout gate complete

## Summary

Completed `g02.004` by normalizing the bounded Songsprout direct-rollout
families and recording what generalized cleanly versus what stayed app-local.

## Why this mattered

Songsprout was the first downstream gate after `compli-me` that mixed an
richer operator surface (`greenhouse`) with an artist-facing app (`bloom`).
The gate needed to prove whether the proof-app pattern freeze still held when
the workflow content became more domain-specific.

## Changes

- normalized the direct-rollout Songsprout routes:
  - `greenhouse` overview and ops
  - `bloom` overview, security, and workflow browse routes
- left the catalogue and artist-detail family explicitly app-local
- recorded the narrower generalization result in `g02.004`
- selected `loophole/composer` as the next downstream consumer gate

## Validation

- `bun x svelte-check --tsconfig ./tsconfig.json` in
  `/Users/betterthanclay/Dev/projects/songsprout/bloom`
- `bun x svelte-check --tsconfig ./tsconfig.json` in
  `/Users/betterthanclay/Dev/projects/songsprout/greenhouse`
  - result: `0 errors, 4 warnings`
  - the warnings are pre-existing state-capture warnings in catalogue/programs
    routes outside the normalized direct-rollout slice

## Next Task

Open the next bounded downstream consumer gate for `loophole/composer`, using
the completed Songsprout result as the next generalization baseline instead of
reopening proof-app family selection.
