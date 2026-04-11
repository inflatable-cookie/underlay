# 2026-04-10 00:35:00 BST - g02.002 Recipe Lock And Mismatch Freeze

Roadmap: `g02.002`

## Summary

Executed `g02.002` Batch 2.2.

Verified that the current Underlay mixed recipe layer and the Poodle guide
layer already cover the frozen proof-app families coherently enough to stop
treating “more proof-family execution” as the default next move.

## Why this batch mattered

The risk after Batch 2.1 was that the proof-app lane would keep expanding
because the docs still felt incomplete. Batch 2.2 tested that assumption
directly.

The result is that the recipe spine is broadly coherent already. The remaining
issues are narrower:

- still-open proof-app edits that belong to the existing wave
- surgical staging discipline because `underlay` and `poodle` both have
  unrelated dirt
- the downstream rollout decision, which now matters more than more proof-lane
  family selection

## Changes

- completed `g02.002` Batch 2.2
- added an explicit coverage check mapping the frozen proof-app families to the
  current Poodle and Underlay recipe surfaces
- recorded the remaining mismatches as bounded follow-on work instead of
  implied continued execution

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`

## Consumer Upgrade Notes

None. This batch is planning-control work only.

## Next Task

Execute `g02.002` Batch 2.3: choose the first non-proof consumer family for
downstream rollout, leave the rest explicit but pending, and name one bounded
follow-on wave.
