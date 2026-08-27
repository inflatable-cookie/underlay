# 2026-08-27 20:57:58 - g09.060 Released Dependency Promotion

## Outcome

The operator chose to continue `g09` after the route-retirement closeout.
Promoted the known Contract `023` drift into one ready docs-only roadmap,
`g09.060`.

## Evidence

- Contract `023` calls sibling Cargo `path` and JavaScript `file:` dependencies
  the default and describes Underlay as unpublished.
- Contract `024` requires released Underlay Git tags on both language surfaces
  and rejects committed path/file edges.
- Guides `030`, `040`, `190`, and `200` teach released tags.
- all six consumer roots use tagged Git dependencies; sibling Underlay/Poodle
  checkouts are tooling inputs only.
- Underlay `v0.9.5` proves the synchronized Rust/JavaScript release surface and
  immutable tag path.

## Decision

- released Git tags are the only committed Underlay dependency shape
- private registry posture does not mean unreleased
- hold-back retains an older tag; upgrade moves every declaration to a new tag
- local sibling checkouts stay untracked and non-authoritative
- semantic versions follow releases, not roadmap generation numbers
- no consumer edit, version bump, release mutation, or tooling change enters
  `g09.060`

## Posture

`strict-planned`. The governing contracts and live evidence agree; the drift is
isolated to Contract `023` plus currentness surfaces. Dispatch waits only for
the independently authorised papercuts wave 3 lane because it also edits
Contract `023` links.

## Consumer Upgrade Notes

- Impact class: documentation correction
- Affected consumers: none; all six known roots already conform
- Required actions: none
- Compatibility window: none; committed path/file edges are already unsupported

## Next Task

Gate cleared in Underlay PR12, merge commit `9e26ba9a`. The `g09.060` handoff is
published; launch it and await the worker PR.
