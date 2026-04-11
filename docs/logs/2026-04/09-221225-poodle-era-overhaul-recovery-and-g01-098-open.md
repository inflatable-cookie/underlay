# Poodle-Era Overhaul Recovery And g01.098 Open

Date: 2026-04-09
Roadmap: `g01.098`

## Summary

- recovered the Underlay planning/currentness surfaces so they stop claiming
  there is no active roadmap
- opened `g01.098` as the live planning/control lane for the current
  shared-surface overhaul
- recorded the older Poodle-contraction line as lineage rather than the active
  queue

## Why this batch mattered

Underlay’s docs shape is still strong, but the live queue had drifted behind
reality. The repo was still presenting the Poodle-contraction line as the last
meaningful work, even though the real active work now spans a broader consumer
normalization wave across Underlay and several consuming apps. This batch makes
the current problem explicit before more implementation outruns the planning
surfaces again.

## Evidence

- `docs/roadmaps/README.md` no longer claims there is no active roadmap
- `docs/roadmaps/g01/README.md` now points at `g01.098` as the live lane
- `g01.098` defines the current problem as shared-surface normalization across:
  - `acowtancy`
  - `compli-me`
  - `contact-patch`
  - `underlay-reference`
  - `loophole/composer`
  - `songsprout`
- the existing dirty file `docs/guides/110-admin.md` was left untouched; this
  batch only changed disjoint planning/currentness surfaces

## Validation

- `effigy qa:docs --repo /Users/betterthanclay/Dev/projects/underlay`
- `effigy qa:northstar --repo /Users/betterthanclay/Dev/projects/underlay`

## Next Task

Execute `g01.098` Batch 98.2: audit the live normalization posture across
Underlay and the current consumer family, then compile the first bounded
follow-on wave from that evidence.
