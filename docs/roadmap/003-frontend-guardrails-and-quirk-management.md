# 003 — Frontend Guardrails + Quirk Management Roadmap (Svelte + TS)

**Status:** Draft

This roadmap defines the “hardening pass” that follows Phase 2 (shared primitives and wrappers).

The goal is to make the SvelteKit + TypeScript admin UI (Dairy) and the shared UI packages (Underlay/Froyo) feel **dependable, polished, and low-quirk** while preserving SSR.

It complements:
- `docs/roadmap/002-frontend-extraction-roadmap.md` (overall frontend extraction plan)

Scope includes:
- Dev-time determinism (linked packages, Vite prebundle, exports discipline)
- SSR reliability and client-only isolation
- Shared primitive guardrails (portals, styling, a11y defaults)
- TypeScript + Svelte 5 conventions that reduce churn
- Targeted performance and UX “polish wins”

Non-goals (for this doc):
- Migrating to React or swapping frameworks
- Replacing Bits UI wholesale
- Product-level feature work unrelated to dependability

---

## 1. How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Also tick the *workstream header checkbox* once all of its children are complete.

---

## 2. Workstream Checklist (high-level)

- [ ] Workstream A — Deterministic workspace + exports
- [ ] Workstream B — SSR hardening + client-only isolation
- [ ] Workstream C — Primitives hardening (portals, tokens, a11y)
- [ ] Workstream D — TS/Svelte guardrails (lint, patterns, conventions)
- [ ] Workstream E — Performance + perceived speed
- [ ] Workstream F — Lightweight tests + validation discipline

---

## Workstream A — Deterministic workspace + exports

Goal: eliminate “mystery state” in dev caused by linked packages and prebundling.

- [ ] Standardize a single supported local-dev topology for Underlay → Froyo → Dairy.
  - Document the golden path for “I changed Underlay, how do I see it in Dairy?”.
  - Prefer a solution that works without manual cache spelunking.

- [ ] Underlay: enforce a stable public API surface.
  - Keep `package.json` `exports` accurate and intentional.
  - Prefer shallow imports from `@decodelabs/underlay` / `@decodelabs/underlay/components` rather than arbitrary deep file paths.

- [ ] Underlay/Froyo: decide and implement one of these approaches:
  - (Preferred) Consumers import built output (`dist/`) only.
  - (Acceptable) Source import is allowed but requires explicit Vite config + documented refresh commands.

- [ ] Dairy: keep a minimal, explainable Vite config for linked packages.
  - Maintain a short “why” list for `optimizeDeps.exclude` and `ssr.noExternal`.

Acceptance criteria:
- Starting `pnpm -C dairy dev` reliably reflects changes in Underlay/Froyo with a documented refresh path.

---

## Workstream B — SSR hardening + client-only isolation

Goal: no SSR-only crashes and no “works in dev, breaks in build/SSR”.

- [ ] Document SSR-safe coding rules:
  - No `window` / `document` / `navigator` / `localStorage` usage at module scope.
  - Browser APIs must be behind `onMount` or explicit runtime guards.
  - Prefer shared helpers that are SSR-aware (clipboard, toasts, etc.).

- [ ] Maintain a small SSR config registry (single source of truth):
  - Which dependencies must be `ssr.noExternal` and why.
  - Which dependencies must be excluded from optimizeDeps and why.
  - How to validate after dependency bumps.

- [ ] Audit “hot paths” for SSR safety:
  - Bits UI overlays/portals
  - Nightfire editor/renderer entrypoints
  - Analytics hooks

Acceptance criteria:
- `pnpm -C dairy build` does not regress due to browser-only imports.

---

## Workstream C — Primitives hardening (portals, tokens, a11y)

Goal: shared components are predictable, themeable, and accessible by default.

### C1) Portaled styling policy

- [ ] Adopt a strict rule:
  - Any component that portals (Select, DropdownMenu, Dialog/AlertDialog, Tooltip, Popover) must be styled with:
    - CSS variables (semantic tokens)
    - `:global(...)` selectors for portaled nodes
  - Do not rely on scoped CSS or `v-bind(...)` variables for portaled content.

- [ ] Provide a “token bridge” strategy:
  - Underlay defines semantic token names.
  - Apps (Dairy) and kits (Froyo) provide values.

### C2) API consistency

- [ ] Normalize prop conventions across Underlay primitives:
  - `open` + `bind:open`
  - `showTrigger`, `triggerLabel`, and trigger slots
  - Floating props: `side`, `sideOffset`, `align`, `alignOffset`, `avoidCollisions`, `collisionPadding`
  - Destructive actions: `destructive` / `danger` naming consistency

- [ ] Froyo adapters: forward all relevant props without surprises.

### C3) Accessibility defaults

- [ ] Icon-only triggers require accessible names (`aria-label`), enforced by API.
- [ ] Focus is visible and consistent across primitives.
- [ ] Overlays return focus correctly on close.

Acceptance criteria:
- A new page can be built using only primitives without custom a11y fixes.

---

## Workstream D — TS/Svelte guardrails (lint, patterns, conventions)

Goal: prevent regressions and reduce “paper-cut” churn.

- [ ] Add lint rules (or equivalent guardrails) that enforce:
  - no `window.alert` / `window.confirm`
  - no raw `navigator.clipboard` usage outside shared clipboard helper
  - no module-scope browser API usage

- [ ] Standardize Svelte 5 typing patterns used in this repo constellation:
  - `$props()` usage conventions
  - `bind:this` element typing
  - form value typing conventions (avoid `FormDataEntryValue` surprises)

- [ ] Standardize “error feedback” conventions:
  - toast for non-blocking errors
  - alert dialog only for destructive confirmation
  - avoid throwing users into broken states

Acceptance criteria:
- New code naturally follows the preferred patterns (pit-of-success).

---

## Workstream E — Performance + perceived speed

Goal: keep the admin feeling fast and stable.

- [ ] Establish a small baseline measurement checklist:
  - initial SSR page load on key routes
  - navigation between list pages
  - opening menus/dialogs

- [ ] Prioritize perceived speed improvements:
  - consistent skeleton/loading patterns
  - avoid layout shift on hydration

- [ ] Handle heavy pages explicitly:
  - Nightfire editor routes should be lazily loaded or guarded where appropriate.
  - Large lists should paginate/filter before rendering massive DOM.

Acceptance criteria:
- Admin remains responsive on realistic data sizes.

---

## Workstream F — Lightweight tests + validation discipline

Goal: catch regressions early without building a huge test suite.

- [ ] Standardize a minimal validation pipeline for frontend changes:
  - `pnpm -C underlay check`
  - `pnpm -C dairy build`
  - (Optional when relevant) `pnpm -C froyo check`

- [ ] Add smoke tests only where they meaningfully reduce risk:
  - SSR smoke (route renders / build)
  - primitive render sanity

Acceptance criteria:
- When validation passes, production regressions are rare and explainable.

---

## Definition of Done

- Dev iteration loop is deterministic (documented refresh path works consistently).
- SSR build is stable; client-only code is isolated by convention and enforcement.
- Portaled primitives are styled via tokens and global selectors.
- Guardrails prevent reintroducing known quirks.
- Admin UX feels polished and predictable (menus/dialogs/toasts/clipboard are consistent).
