# g05

`g05` is the shared-surface convergence generation for the current consumer
family.

## Current State

`g05` opened the next template-system lane after `g03` closeout, completed the
cross-consumer entity-template convergence work, and remains the live
generation for the next retained shared-page and workflow-template wave.

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

The next generation problem was the repeated non-entity page family that still
deserved shared ownership:

- cross-app media-library workflow consolidation
- repeated system index shells
- repeated admin dashboard shells
- repeated media upload and detail shells inside that family

The current generation problem is the Rust/runtime contract layer that still
needs to be made explicit before the six APIs can converge on one declared app
server posture:

- app-runtime assembly and router topology
- route-family and access-model rules
- front/shared read API shapes beyond the admin template system

## Active Lane

- `g05.009`: Rust runtime contract audit and next contract set

## Completed Work

- `g05.001`: Dairy detail pages and tab-list convergence sweep
- `g05.002`: compli-me and contact-patch detail-page convergence sweep
- `g05.003`: underlay-reference template completion and contract-hardening
  sweep
- `g05.004`: cross-app media-library template consolidation
- `g05.005`: system index page template proof
- `g05.006`: admin dashboard page template proof
- `g05.007`: media upload page proof, absorbed into `g05.004`
- `g05.008`: media detail workflow page proof, absorbed into `g05.004`

## Current Queue

- `g05.009` is active as the Rust runtime contract audit and next contract set
- `g05.008` is complete as the media detail workflow page proof, absorbed into
  `g05.004`
- `g05.007` is complete as the media upload page proof, absorbed into
  `g05.004`
- `g05.006` is complete as the admin dashboard page template proof
- `g05.005` is complete as the system index page template proof
- `g05.004` is complete as the cross-app media-library template consolidation
- `g05.003` is complete as the underlay-reference template completion and
  contract-hardening sweep
- `g05.002` is complete as the compli-me and contact-patch detail-page
  convergence sweep
- `g05.001` is complete as the Dairy detail pages and tab-list convergence
  sweep

## Batch Cards

If `g05` later enters strict execution posture, keep its cards under
`g05/batch-cards/`.

## Next Task

Write `118`: the front/shared read API shape contract.
