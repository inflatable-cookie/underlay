# g12.005 — Consume Nightfire v0.2.0; keep facades, not a second copy

Owner: Underlay Nightfire facades (`underlay-nightfire`, TS `./nightfire/*`)
Created: 2026-09-18
Depends on: nightfire `v0.2.0` (tag `1931cfc2d4d77959140c39ee56047b581b11256d`, npm `@inflatable-cookie/nightfire@0.2.0`)
Auto-start next task: no
Execution authority: Tom, 2026-09-18 — “Do the underlay update”, after Acowtancy g05.154 landed.

This is **g12.001 Card 274** as an executable slice, updated for v0.2.0. The 2026-09-18 triage note asked for v0.1.0; that release shipped no schemas and still had `./media`. Do not pin v0.1.0.

Source: `docs/triage/20260918-113055-retire-in-repo-nightfire-copy.md`. Acowtancy Farmyard already consumes `nightfire` `v0.2.0` via `nightfire_upstream`; its remaining bridges are `underlay-media` / `underlay-validation` on this repo’s `v0.9.8` internal copy.

## Outcome

Underlay’s generic Nightfire **implementation** is the released `nightfire` package. `underlay-validation` and `underlay-media` depend on crate `nightfire` at tag `v0.2.0`, not `path = "../underlay-nightfire"`. Historical TS subpaths keep working as facades. The in-repo generic copy is no longer the editable source. Nothing is published.

## Why this is needed

Nightfire is its own repository and release train. This repo still hosts `rust/crates/underlay-nightfire/` and `ts/src/nightfire/` and publishes sixteen `./nightfire/*` subpaths from `@inflatable-cookie/underlay`. That is a duplicate of a released capability. Removing the crate without swapping internals would break this build.

## What v0.2.0 changes for this swap

- Generic schemas and core payloads live in Nightfire. Underlay does not generate them.
- **Breaking:** Nightfire removed `./media` and `useNightfireMedia` / `createNightfireMediaContext`. Replaced by `./media-source` (`registerMediaSource`). There is no alias.
- Underlay `./nightfire/media` (`ts/src/nightfire/media/context.ts`) is **Underlay-specific** (media-library picker context for apps such as Dairy). **Keep that export.** Do not re-export a Nightfire module that no longer exists. Do not make Dairy import `@inflatable-cookie/nightfire/media`.
- Nightfire `./media-locator` remains. Map Underlay `./nightfire/media-locator` to it.

## Work (order)

1. Depend on released Nightfire at tag `v0.2.0` / commit `1931cfc2`. **Transport is HTTPS, not SSH:**
   `nightfire = { git = "https://github.com/inflatable-cookie/nightfire.git", tag = "v0.2.0" }`.
   Nightfire is a public repository; Underlay GitHub Actions has no SSH key to it, and this
   lane does not add credentials or edit `.github/workflows/`. TypeScript uses released npm
   package `@inflatable-cookie/nightfire` `0.2.0` because Bun cannot resolve the annotated Git
   tag. Any Git-based Nightfire pin must use HTTPS. Do not use `ssh://git@github.com/...` in
   Cargo.toml, Cargo.lock, or package.json — that is what blocked PR #34 (Clippy fetch failed:
   `ssh-agent authentication with no usernames succeeding`).
2. Point `underlay-validation` (`nightfire` feature) and `underlay-media` (`nightfire` feature) at that crate. Prove they compile and their Nightfire-typed tests still pass.
3. Turn historical TS `./nightfire/*` subpaths into facades over `@inflatable-cookie/nightfire` **except** `./nightfire/media`, which stays Underlay-owned. Keep `./nightfire` plus editor, renderer, block-editor, block-registration, markdown, editor-registry, render-registry, validator-registry, strategies, media-locator, block-ids, block-versions, utils, validation. Map each to the Nightfire counterpart (`./core` / `./types` as needed). Callers must not have to change import paths in this lane.
4. Turn `underlay-nightfire` into a thin crate-name facade over `nightfire`, or delete the implementation once nothing in this workspace uses the path crate. Card 274: delete internal implementations; retain behaviour-compatible historical names. Public API inventory `docs/contracts/122-rust-public-api-inventory.md` still names `underlay-nightfire` as a deprecation facade — honour that.
5. Point `docs/guides/076-nightfire.md` and `docs/contracts/070-nightfire-and-migration-systems.md` at the Nightfire repository for generic capability; keep Underlay integration (media traversal, validation-to-HTTP, picker context) documented here.
6. Do **not** publish. Do not edit `.github/workflows/`. Do not write into `/Users/tom/Dev/projects/nightfire` or Acowtancy.

## Consumer Upgrade Impact

- Existing `@inflatable-cookie/underlay/nightfire/*` imports keep working, including `./nightfire/media`.
- Rust `underlay-nightfire` crate name remains until a later retirement card.
- Farmyard can drop its `underlay-nightfire` path dependency only after **this repository tags a release** that contains the swap. That Farmyard lane is Acowtancy g05.155, not this task.
- Apps that want Nightfire’s new `registerMediaSource` / core blocks (`download_card`, `rich_text`, `item_list`, `video`) take them from `@inflatable-cookie/nightfire` directly. Not required to finish this lane.

## Acceptance

- `underlay-validation` and `underlay-media` do not `path = "../underlay-nightfire"` for the generic crate.
- Generic Nightfire TS implementation is not edited in-tree as the source of truth; facades re-export the package.
- `./nightfire/media` still exports `createNightfireMediaContext` / `useNightfireMedia`.
- `effigy rust:check` (or workspace check with nightfire features) and the Nightfire-related unit/component tests this repo already runs are green.
- Nothing published.

## Stop Conditions

- A historical subpath cannot facade onto v0.2.0 without changing caller imports — return the subpath; do not break Dairy.
- `underlay-media` extractor cannot type against `nightfire` v0.2.0 `NightfireValue` — return that; do not paper over with a serde round-trip that drops new core blocks.
- Publication or a Nightfire-repo edit would be required.
- SSH git transport would be required for CI — use HTTPS instead; do not add Actions credentials.

## Next Task

Operator publishes an Underlay tag. Acowtancy then bumps Farmyard’s underlay pin and drops the two bridges (g05.155). Do not auto-start. Do not invent a version number.
