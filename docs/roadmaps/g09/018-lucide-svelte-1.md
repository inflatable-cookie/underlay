# g09.018 - lucide-svelte 1.0

Status: ready
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

- [ ] Review the 1.0 changelog for removed/renamed icons and package
  structure changes; grep the family for every import and classify
  (safe / renamed / removed).
- [ ] underlay first (templates + stories); then consumers per repo.
- [ ] svelte-check vs baselines; component tests green in underlay.
- [ ] Visual spot-check one admin nav per repo class (icon rendering,
  sizes) if feasible; otherwise note as unverified.

## Consumer Upgrade Impact

Impact class: `breaking` where icons were renamed/removed; mechanical
otherwise.

## Validation

- [ ] svelte-check no-new-errors; underlay component tests green

## Stop Conditions

If 1.0 removed icons the apps rely on, substitute per app and note in the
commit; do not leave broken references.

## Next Task

`g09.019` jsdom 30.
