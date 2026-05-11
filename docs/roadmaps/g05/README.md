# g05

`g05` is the active Underlay roadmap generation.

## Current State

`g05` opens the next template-system lane after `g03` closeout.

`g03` proved the shared form and list-page shells across Dairy and
`underlay-reference`, but it stopped before the broader detail-page and tab-list
convergence line.

The active problem now is detail-page and child-list normalization:

- move Dairy detail routes onto `EntityDetailPage` where the shared shell can
  carry the real behavior
- move tab child collections onto `EntityList` or reused root-list wrappers
  where that keeps behavior intact
- expand the shared template shape only when repeated real caller families prove
  the current shape is too narrow

## Active Lane

- `g05.001`: Dairy detail pages and tab-list convergence sweep

## Completed Work

- none yet

## Batch Cards

If `g05` later enters strict execution posture, keep its cards under
`g05/batch-cards/`.

## Next Task

Execute `g05.001`: record the Dairy detail-page and tab-list caller inventory,
classify direct migrations vs real shared-template gaps, and start with the
highest-repeat family instead of scattered page rewrites.
