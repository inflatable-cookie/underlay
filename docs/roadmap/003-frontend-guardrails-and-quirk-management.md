# 003 — Frontend Guardrails + Quirk Management Roadmap (Svelte + TS)

**Status:** In progress

This roadmap defines the “hardening pass” that follows Section 2 (shared primitives and wrappers).

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
- [x] Workstream C — Primitives hardening (portals, tokens, a11y)
- [~] Workstream D — TS/Svelte guardrails (lint, patterns, conventions)
- [ ] Workstream E — Performance + perceived speed
- [ ] Workstream F — Lightweight tests + validation discipline

---

## Workstream A — Deterministic workspace + exports

Goal: eliminate “mystery state” in dev caused by linked packages and prebundling.

- [x] Standardize a single supported local-dev topology for Underlay → Froyo → Dairy.
  - Golden path:
    - `pnpm -C apps/acowtancy/dairy dev:refresh`
    - If you add new files/exports in Underlay/Froyo, run `pnpm -C apps/acowtancy/dairy refresh:deps` (this re-links the `file:` deps) and restart.
  - Troubleshooting (only if something is clearly stale):
    - Stop dev server
    - Delete caches: `rm -rf apps/acowtancy/dairy/node_modules/.vite apps/acowtancy/dairy/.svelte-kit`
    - Restart: `pnpm -C apps/acowtancy/dairy dev --force`

- [ ] Underlay: enforce a stable public API surface.
  - Keep `package.json` `exports` accurate and intentional.
  - Prefer shallow imports from `@decodelabs/underlay` / `@decodelabs/underlay/components` rather than arbitrary deep file paths.

- [ ] Underlay/Froyo: decide and implement one of these approaches:
  - (Preferred) Consumers import built output (`dist/`) only.
  - (Acceptable) Source import is allowed but requires explicit Vite config + documented refresh commands.

- [x] Dairy: keep a minimal, explainable Vite config for linked packages.
  - `optimizeDeps.exclude`:
    - Excludes `@decodelabs/underlay/*` entrypoints because Underlay is a local `file:` dep and Vite prebundling can cache stale exports.
  - `ssr.noExternal`:
    - Includes `bits-ui`, `svelte-toolbelt`, `runed` to keep SSR stable.
    - Includes `lucide-svelte` and `easymde` since they’re used by shared components.

Acceptance criteria:
- Starting `pnpm -C apps/acowtancy/dairy dev` reliably reflects changes in Underlay/Froyo with a documented refresh path.

---

## Workstream B — SSR hardening + client-only isolation

Goal: no SSR-only crashes and no “works in dev, breaks in build/SSR”.

- [x] Document SSR-safe coding rules:
  - No `window` / `document` / `navigator` / `localStorage` usage at module scope.
  - Browser APIs must be behind `onMount` or explicit runtime guards.
  - Prefer shared helpers that are SSR-aware (clipboard, toasts, etc.).
  - For portaled UI: assume it renders during SSR; avoid DOM reads/writes until mounted.

- [x] Maintain a small SSR config registry (single source of truth):
  - `apps/acowtancy/dairy/vite.config.ts`
    - `ssr.noExternal`: `bits-ui`, `svelte-toolbelt`, `runed`, `lucide-svelte`, `easymde`
    - `optimizeDeps.exclude`: Underlay entrypoints (local `file:` dependency)
  - Validation after bumps:
    - `pnpm -C libraries/underlay check`
    - `pnpm -C apps/acowtancy/dairy build`

- [ ] Audit “hot paths” for SSR safety:
  - Bits UI overlays/portals
  - Nightfire editor/renderer entrypoints
  - Analytics hooks

Acceptance criteria:
- `pnpm -C apps/acowtancy/dairy build` does not regress due to browser-only imports.

---

## Workstream C — Primitives hardening (portals, tokens, a11y)

Goal: shared components are predictable, themeable, and accessible by default.

### C1) Portaled styling policy

- [x] Adopt a strict rule:
  - Any component that portals (Select, DropdownMenu, Dialog/AlertDialog, Popover) must be styled with:
    - CSS variables (semantic tokens)
    - class-based selectors for portaled nodes
  - Do not rely on scoped variables that won’t inherit into `document.body`.

- [x] Provide a “token bridge” strategy:
  - Underlay defines semantic token names:
    - `--underlay-color-popover-bg`
    - `--underlay-color-menu-bg`
    - `--underlay-color-dialog-bg`
    - `--underlay-color-overlay-backdrop`
    - `--underlay-shadow-menu`
    - `--underlay-shadow-dialog`
  - Apps (Dairy) and kits (Froyo) provide values (ideally on `:root` so portaled nodes inherit).

### C2) API consistency

- [x] Normalize portaled styling tokens across primitives.
- [x] Normalize prop conventions across Underlay primitives:
  - `open` + `bind:open`
  - `showTrigger`, `triggerLabel`, `triggerAriaLabel`, and trigger slots
  - Trigger `type="button"` by default for form safety
  - Floating props: `side`, `sideOffset`, `align`, `alignOffset`, `avoidCollisions`, `collisionPadding`
  - Destructive actions: `destructive` / `danger` naming consistency
  - Progress:
    - [x] DropdownMenu: `triggerAriaLabel`, `triggerType`, floating props, `contentClassName`
    - [x] Select: `open` + `bind:open`, `triggerType`, `triggerAriaLabel`, `contentClassName`, focus return, `--underlay-color-menu-bg`, `--underlay-shadow-menu`
    - [x] Popover: `triggerAriaLabel`, `triggerType`, `contentClassName`, `--underlay-color-popover-bg`
    - [x] Dialog/AlertDialog: `triggerAriaLabel`, `triggerType`, `contentClassName`, `overlayClassName`, focus return, `--underlay-color-dialog-bg`, `--underlay-color-overlay-backdrop`

- [x] Froyo adapters: forward all relevant props without surprises.
  - Froyo currently imports Underlay primitives directly (no overlay wrappers to keep in sync).

### C3) Accessibility defaults

- [x] Icon-only triggers require accessible names (`aria-label`), enforced by API.
  - [x] DropdownMenu trigger uses `aria-label` (defaults to "Open menu").
  - [x] Dialog/AlertDialog expose optional `triggerAriaLabel`.
  - [x] Popover trigger exposes `triggerAriaLabel`.
  - [x] Select trigger uses `aria-label={placeholder}` by default.
- [x] Focus is visible and consistent across primitives.
- [x] Overlays return focus correctly on close.
  - Enforced in Underlay wrappers via `returnFocusOnClose` + `bind:ref` on trigger.

Acceptance criteria:
- A new page can be built using only primitives without custom a11y fixes.

---

## Workstream D — TS/Svelte guardrails (lint, patterns, conventions)

Goal: prevent regressions and reduce “paper-cut” churn.

- [~] Add lint rules (or equivalent guardrails) that enforce:
  - [x] no `window.alert` / `window.confirm`
  - [x] no raw `navigator.clipboard` usage outside shared clipboard helper
  - [x] no module-scope browser API usage (enforced via `dairy/guardrails.mjs`)

- [~] Standardize Svelte 5 typing patterns used in this repo constellation:
  - Prefer a named `Props` type over inline object casts.
    - Example: `type Props = { data: PageData }; let { data } = $props() as Props;`
  - Always type `bind:this` targets (nullable) to avoid `any`.
    - Example: `let formEl: HTMLFormElement | null = null;`
  - When reading `FormData`, narrow `FormDataEntryValue` explicitly.
    - Example: `const v = fd.get("name"); if (typeof v !== "string") return fail(...);`

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

- [~] Standardize a minimal validation pipeline for frontend changes:
  - Full stack (recommended): `pnpm -C apps/acowtancy/dairy validate:stack`
  - Or run individually:
    - `pnpm -C underlay validate`
    - `pnpm -C froyo validate` (optional when relevant)
    - `pnpm -C dairy validate`

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
