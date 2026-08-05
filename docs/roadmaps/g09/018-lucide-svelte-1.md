# g09.018 - lucide-svelte 1.0

Status: complete
Completed: 2026-08-04
Owner: repo maintainers

## Purpose

Icon library major 0.575 → 1.0. Icon imports (`lucide-svelte/icons/<name>`)
appear across every admin/front app; check for renames/removals in the 1.0
release notes and codemod where mechanical.

## Evidence

- JS dependency survey 2026-08-03 (lucide-svelte 0.575.0 → 1.0.1)
- Widespread `lucide-svelte/icons/*` imports in admin/front packages and
  underlay templates (`AdminNavList`, etc.)

## Planned Changes

- [x] Review the 1.0 changelog for removed/renamed icons and package
  structure changes; grep the family for every import and classify
  (safe / renamed / removed).
- [x] underlay first (templates + stories); then consumers per repo.
- [x] svelte-check vs baselines; component tests green in underlay.
- [x] Visual spot-check one admin nav per repo class (icon rendering,
  sizes) if feasible; otherwise note as unverified.

## Consumer Upgrade Impact

Impact class: `breaking` where icons were renamed/removed; mechanical
otherwise.

## Validation

- [x] svelte-check no-new-errors; underlay component tests green

## Stop Conditions

If 1.0 removed icons the apps rely on, substitute per app and note in the
commit; do not leave broken references.

## Completion Notes

Completed 2026-08-04. lucide-svelte 1.0.1 everywhere (underlay ae885e30 with peer range widened to ^0.563.0 || ^1.0.0). The only breaking surface: 1.0 dropped two legacy aliases — check-circle -> circle-check, alert-triangle -> triangle-alert — fixed in 2 underlay templates and 5 consumer components (3 AdminNavList clones, DairyNavList, TransformPreviewCard). All ~90 family icon names verified to resolve. Commits: underlay-reference a568b3a, contact-patch 2d79c93, compli-me 0198cc5, songsprout f9fa0e2, acowtancy submodules dairy 40aa91b6 / cream 65d9212 + parent f75d873. svelte-check green everywhere (dairy at exact 64-warning baseline).

## Next Task

`g09.019` jsdom 30.
