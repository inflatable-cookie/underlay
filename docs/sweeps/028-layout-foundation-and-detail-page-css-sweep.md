# 028 - Layout Foundation and Detail Page CSS Sweep

Use this sweep to keep layout behavior centralized in Underlay primitives and utility classes, and to prevent per-page CSS drift in admin apps.

## Scope

- Underlay shared layout primitives:
  - `styles/base.css`
  - `InlineListCard`
  - `ContentCard`
  - `DetailsCard`
  - `ContainerGrid`
- Consuming admin apps (for example Dairy) detail tabs and list-card surfaces.

## Step 1 - Verify Underlay Layout Baseline

Confirm all of the following exist in Underlay:

- Global box sizing:
  - `*, *::before, *::after { box-sizing: border-box; }`
- `.underlay-details-content` is a responsive 2-column grid with `align-items: stretch`.
- `.underlay-details-content > * { min-width: 0; }` exists to prevent overflow.
- Direct child cards (`InlineListCard`, `ContentCard`, `DetailsCard`, `Card`) default to full-width in detail grids.
- `.span-full` rows are supported and child cards inside `.span-full` also default to full-width.

## Step 2 - Verify Component Capabilities

Check that common “no custom CSS” use cases are supported by props:

- `InlineListCard` supports:
  - `fullWidth`
  - `stretch`
- `ContentCard` supports:
  - `fullWidth`
  - `stretch`

## Step 3 - Scan for Layout-Hack CSS in Consumers

Run these checks in each consuming app:

```bash
rg -n ":global\\(\\.underlay-|underlay-inline-list-card|underlay-content-card|details-content|bundle-details-grid|__left|__right|max-width:\\s*none|height:\\s*100%|align-items:\\s*stretch" src
```

```bash
rg -n "underlay-details-content" src/routes src/lib
```

Manual review:

- Any remaining matches should be intentional visual customization (not generic layout plumbing).
- If the CSS only exists to force width/stretch/alignment for shared cards, remove it and use shared props/utilities.

## Step 4 - Detail Page Structure Check

For each admin detail page:

- Use `.underlay-details-content` as the main details container.
- Prefer direct `DetailsCard` / `InlineListCard` / `ContentCard` composition over local wrapper grids.
- Use `.span-full` for full-width rows beneath side-by-side content.

## Step 5 - Pass Criteria

Pass when:

- No page-level CSS overrides shared card internals for basic layout.
- No ad-hoc “left/right details card” wrapper classes exist solely for width/stretch.
- Detail pages render with shared primitives and utility classes only.
- Custom CSS is limited to genuinely custom visuals (media preview frame, bespoke interactive widgets, etc).

## Step 6 - Validation

Run narrow checks in changed repos:

```bash
# Underlay
bun run check
```

```bash
# Consumer app (example: Dairy)
bun run check
```

## Step 7 - Rollout Notes

When applying to other Underlay apps:

- Migrate one detail surface family at a time (for example Learning first, then Content, then System).
- Keep a short exception log for intentional custom styling that remains.
- Avoid introducing compatibility aliases or temporary layout classes; keep the API and CSS surface clean.
