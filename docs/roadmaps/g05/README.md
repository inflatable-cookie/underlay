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

The current generation problem is now the broader Underlay dev-flow contract
layer that sits above the shared runtime and template work:

- new app bootstrap and bring-up
- migration and schema workflow
- testing posture
- template adoption rules
- release and compatibility rollout
- the remaining supporting delivery contracts around config, OpenAPI quality,
  audit posture, and app review

Another explicit `g05` lane is now reopened:

- fleet media-library capability policy across all six consumer apps
- rollout of the missing media family in `songsprout` and `compli-me`, now
  complete

## Active Lane

- `g05.024`: Marking Hub query variant proof and rollout

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
- `g05.009`: Rust runtime contract audit and next contract set
- `g05.010`: new Underlay app bootstrap and bring-up contract
- `g05.011`: migration and schema workflow contract
- `g05.012`: testing posture and shared harness contract
- `g05.013`: consumer template adoption contract
- `g05.014`: release and compatibility rollout contract
- `g05.015`: config and secrets contract
- `g05.016`: OpenAPI quality and declaration contract
- `g05.017`: error-code and operator-audit contract
- `g05.018`: Underlay app review checklist and audit artifact
- `g05.019`: fleet media library capability mandate
- `g05.020`: compli-me and songsprout media family rollout
- `g05.021`: Poodle card toggle query variant control
- `g05.022`: list query variant API contract
- `g05.023`: EntityList query variant integration
- `g05.024`: Marking Hub query variant proof and rollout

## Current Queue

- `g05.024` is promoted as the Acowtancy Marking Hub proof and six-app rollout
- `g05.023` is complete as the EntityList query variant integration
- `g05.022` is complete as the list query variant API contract
- `g05.021` is complete as the Poodle card toggle query variant control
- `g05.020` is complete as the compli-me and songsprout media family rollout
- `g05.019` is complete as the fleet media library capability mandate
- `g05.018` is complete as the Underlay app review checklist and audit artifact
- `g05.017` is complete as the error-code and operator-audit contract
- `g05.016` is complete as the OpenAPI quality and declaration contract
- `g05.015` is complete as the config and secrets contract
- `g05.014` is complete as the release and compatibility rollout contract
- `g05.013` is complete as the consumer template adoption contract
- `g05.012` is complete as the testing posture and shared harness contract
- `g05.011` is complete as the migration and schema workflow contract
- `g05.010` is complete as the new Underlay app bootstrap and bring-up contract
- `g05.009` is complete as the Rust runtime contract audit and next contract set
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

Execute `g05.024`: prove the query-variant contract through Acowtancy Marking
Hub, then inventory follow-on named base-query lists across the six-app
consumer family.
