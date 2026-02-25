# Cross-Project Layout Sweep (2026-02-25)

Applied sweep: `docs/sweeps/028-layout-foundation-and-detail-page-css-sweep.md`

Projects reviewed:
- `compli-me`
- `songsprout`
- `loophole`

## Findings

### Compli-Me (`compli-me/admin`)

Relevant detail-page surfaces found under:
- `src/routes/(app)/compliments/businesses/[businessId]/+page.svelte`
- `src/routes/(app)/compliments/messages/[complimentId]/+page.svelte`
- `src/routes/(app)/compliments/people/[personId]/+page.svelte`

Changes applied:
- Replaced local page wrapper CSS (`display:flex; flex-direction:column; gap`) with shared utility class:
  - `.underlay-page-stack`
- Removed redundant local wrapper styles from the three files above.
- Migrated scheduled-task detail view to shared detail layout structure:
  - `src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
  - replaced local wrapper/grid stack with `.underlay-details-content` + `.span-full`
- Kept feature-specific styles:
  - message body formatting
  - header separator
  - link affordances
  - scheduled-task content presentation + jobs table theming

Validation:
- `bun run check` in `compli-me/admin` passed.

### Songsprout

Initial scans across `songsprout/bloom` and `songsprout/stem` found no admin detail-page layout matches requiring this sweep’s migration pattern (no `underlay-details-content` / shared-card override signatures in target surfaces).

No changes required in this pass.

### Loophole

Project appears currently Rust-focused in the scanned workspace surface (`echo`); no Underlay/Svelte admin detail-page targets identified for this sweep.

No changes required in this pass.

## Shared follow-up completed

Underlay updated with a reusable page-level stack helper:
- `.underlay-page-stack` in `ts/src/styles/base.css`
- documented in `docs/guides/090-ui-kit.md`

This utility is now available for all consuming apps to replace trivial per-page vertical stack wrappers.

## Remaining work

- Continue sweep `028` on any additional admin apps as they are discovered.
- For future migrations, use this order:
  1. Remove structural wrapper CSS first.
  2. Replace with shared utilities/props.
  3. Keep only feature-specific CSS and document exceptions.
