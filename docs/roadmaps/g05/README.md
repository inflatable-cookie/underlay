# g05

`g05` is the template-convergence generation for the current consumer family.

## Current State

`g05` opened the next template-system lane after `g03` closeout and is now
complete.

`g03` proved the shared form and list-page shells across Dairy and
`underlay-reference`, but it stopped before the broader detail-page and tab-list
convergence line.

The generation problem is template convergence across the current consumers:

- converge detail routes onto `EntityDetailPage` where the shared shell can
  carry the real behavior
- converge real browse/manage list surfaces onto reusable app-local wrappers
  over `EntityListPage`
- harden the written contract so `underlay-reference` becomes the reference
  implementation style other apps should copy

## Active Lane

- none

## Completed Work

- `g05.001`: Dairy detail pages and tab-list convergence sweep
- `g05.002`: compli-me and contact-patch detail-page convergence sweep
- `g05.003`: underlay-reference template completion and contract-hardening
  sweep

## Batch Cards

If `g05` later enters strict execution posture, keep its cards under
`g05/batch-cards/`.

## Next Task

None. `g05` is complete.
