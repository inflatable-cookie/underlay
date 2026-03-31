# g01.094 - UI Guides Poodle Translation Wave

Status: Complete

## Summary

Translate the main UI guides from old Underlay-first shared-component language
to a Poodle-first implementation story.

## Scope

This first pass covers the top-layer UI guide family:

- `062-auth-ui-components.md`
- `090-ui-kit.md`
- `096-form-helpers.md`
- `097-autonomous-list-components.md`
- `098-shared-admin-patterns.md`
- `100-frontend-web.md`
- `110-admin.md`
- `176-ai-runtime-routing.md`
- `docs/guides/README.md`

## Progress

- first-pass Poodle implementation guides added for auth, page-shell/admin,
  media library/upload, and admin feature delivery
- admin shell layout guidance added in Poodle so Underlay no longer needs to
  teach visible admin-shell implementation
- strongest Poodle recipes are now tied directly to ACME reference
  implementations instead of staying abstract
- `090-ui-kit.md` collapsed to a true boundary page and the dead `090-ui-kit`
  example surface was retired
- `100` and `110` code directories are now explicitly marked as
  integration-oriented stubs rather than canonical UI examples
- Underlay guide index and the strongest UI-heavy guides now point to Poodle as
  the canonical implementation home
- retained Underlay docs are being narrowed to boundary/runtime/full-stack
  guidance rather than generic UI implementation

## Closeout Assessment

This wave is complete as a translation wave.

What is now true:

- Poodle owns the canonical reusable UI implementation guides.
- The strongest reusable implementation families have Poodle recipe coverage:
  auth, page shells, admin delivery, admin shell layout, and media/upload.
- The strongest recipe families are tied directly to ACME reference
  implementations instead of abstract pseudo-code only.
- Underlay no longer presents `090-ui-kit.md` or `code/090-ui-kit/` as an
  active generic UI teaching surface.
- `code/100-frontend-web/` and `code/110-admin/` are now explicitly marked as
  retained integration stubs rather than canonical UI examples.

What still remains for a follow-on prune wave:

- some retained integration docs still contain large embedded Poodle-facing
  example sections because they have not yet been split further:
  - `077-media-library.md`
  - `100-frontend-web.md`
  - `110-admin.md`
- those pages still earn retained existence overall, but their remaining
  implementation-heavy sections should be trimmed or moved in a separate pass
  rather than treated as part of the main translation line

## Decisions

- Poodle is the default implementation layer for shared visuals.
- Underlay references stay only where the contract is still genuinely
  workflow/runtime-owned.
- Examples should prefer direct Poodle composition and current snippet-based
  APIs rather than retired wrapper patterns or legacy slot syntax.

## Consumer Upgrade Impact

Documentation-only in this first pass.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

Open a focused follow-on prune wave on the remaining long-tail integration
docs, especially `077-media-library.md`, `100-frontend-web.md`, and
`110-admin.md`, and split their retained runtime/full-stack guidance from any
remaining generic Poodle implementation examples.
