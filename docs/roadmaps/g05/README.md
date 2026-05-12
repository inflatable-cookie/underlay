# g05

`g05` is the template-convergence generation for the current consumer family.

## Current State

`g05` opened the next template-system lane after `g03` closeout, completed the
cross-consumer entity-template convergence work, and is now reopened for the
next retained shared-page wave.

`g03` proved the shared form and list-page shells across Dairy and
`underlay-reference`, but it stopped before the broader detail-page and tab-list
convergence line.

The first generation problem was template convergence across the current
consumers:

- converge detail routes onto `EntityDetailPage` where the shared shell can
  carry the real behavior
- converge real browse/manage list surfaces onto reusable app-local wrappers
  over `EntityListPage`
- harden the written contract so `underlay-reference` becomes the reference
  implementation style other apps should copy

The next generation problem is the repeated non-entity page family that still
deserves shared ownership:

- cross-app media-library workflow consolidation
- repeated system index shells
- repeated admin dashboard shells
- repeated media upload shells
- possible retained media-detail workflow shell

## Active Lane

- `g05.004`: cross-app media-library template consolidation

## Completed Work

- `g05.001`: Dairy detail pages and tab-list convergence sweep
- `g05.002`: compli-me and contact-patch detail-page convergence sweep
- `g05.003`: underlay-reference template completion and contract-hardening
  sweep

## Current Queue

- `g05.004`: cross-app media-library template consolidation
- `g05.005`: system index page template proof
- `g05.006`: admin dashboard page template proof
- `g05.007`: media upload page template proof
- `g05.008`: media detail workflow template proof

## Batch Cards

If `g05` later enters strict execution posture, keep its cards under
`g05/batch-cards/`.

## Next Task

Execute `g05.004`: audit the four admin media-library implementations and freeze
the retained shared media-template surface before any extraction begins.
