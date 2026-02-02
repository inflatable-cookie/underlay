# 002 — Frontend (TS/Svelte) Extraction Roadmap

**Status:** In progress

This roadmap covers the **TypeScript + Svelte** side of the shared architecture.
It complements `docs/roadmap/001-extraction-roadmap.md` (Rust/backend/infra).

Scope includes:
- Shared UI primitives (`@decodelabs/underlay`)
- Acowtancy-specific UI extensions (`@acowtancy/froyo` Nightfire blocks)
- Songsprout-specific UI extensions (`@songsprout/petal`)
- Admin app integration patterns (Dairy → Froyo + Underlay)
- Headless UI foundation via Bits UI (`bits-ui`)
- Shared conventions: tokens, component APIs, accessibility, packaging, testing

Non-goals (for this doc):
- Picking a hosting provider
- Product-specific UX decisions that belong in app roadmaps

---

## Inputs audited (current state)

### 1) Froyo (`/Users/betterthanclay/Dev/apps/acowtancy/froyo`)
- Lightweight package for **Acowtancy-specific** Nightfire blocks (render/editor/validation registrations).
- Depends on `@decodelabs/underlay` for shared primitives and Nightfire engine.
- Does not aim to be a general shared UI kit.

### 2) Dairy (`/Users/betterthanclay/Dev/apps/acowtancy/dairy`)
- SvelteKit admin app.
- Consumes Froyo for Acowtancy Nightfire blocks (registrations via `@acowtancy/froyo/editor|render|validation`).
- Consumes Underlay primitives directly (`@decodelabs/underlay/components`, `@decodelabs/underlay/nightfire`).
- Avoids source-level aliasing; prefers `file:` deps for local development.

### 3) Petal (`/Users/betterthanclay/Dev/apps/songsprout/petal`)
- Lightweight package reserved for Songsprout-specific UI extensions.
- Intentionally minimal until Songsprout has shared cross-app UI extensions to centralize.

### Shared toolchain snapshot
- All three are on Svelte 5 + SvelteKit 2.
- Froyo and Dairy are closely aligned in tooling versions.
- Petal is intentionally minimal (does not currently drive shared UI tooling).

---

## Key findings (what is truly “shared”)

### A) Shared primitives (present in both ecosystems)
These exist across product frontends conceptually and are safe to standardize:
- `Button`
- `TextInput` (+ label/hint/error pattern)
- `Card`
- `Grid`/`Stack` layout primitives
- Status/feedback badge patterns
- Icon system (`lucide-svelte` already common)

### B) Shared foundation (should be common, even if styling differs)
- A11y primitives: focus ring, keyboard interactions, ARIA conventions
- “Headless UI” semantics and interaction models (Bits UI)
- Component API conventions (variants/sizes/density, event patterns)
- CSS token *shape* (semantic tokens) even if actual values differ by brand

### C) Not truly shared
These are product/domain specific and should remain app-owned:
- Nightfire block editors/renderers (Acowtancy-only for now)
- Dairy admin workflows and form shells
- Songsprout content/editor concepts (until they converge on a shared model)

---

## Section checklist (high-level)

- [x] Section 1 — Standardize frontend foundations
- [x] Section 2 — Bits-first primitives and wrappers
- [x] Section 3 — Tokens, themes, density
- [x] Section 4 — Packaging and consumption boundaries
- [x] Section 5 — Extract app-specific UI up into extension packages
- [x] Section 6 — Testing + release discipline

---

## Section 1 — Standardize frontend foundations (low risk)

- [x] Align Svelte/SvelteKit/TS/ESLint/Vite/Vitest versions within each product family:
  - [x] Acowtancy: Dairy + Froyo
  - [x] Songsprout: Bloom + Greenhouse
- [x] Standardize SSR config for Bits UI usage:
  - Ensure `ssr.noExternal` consistently includes `bits-ui` (+ any required dependencies like `svelte-toolbelt`).
- [x] Standardize iconography:
  - Confirm `lucide-svelte` as the shared icon set; define import conventions.
  - Convention: prefer per-icon imports (`lucide-svelte/icons/<icon-name>`) for consistent, tree-shakeable usage.
  - Convention: icon-only buttons must include an accessible label (`aria-label`) or use a labelled wrapper.
- [x] Standardize CSS entrypoints for shared packages:
  - Explicitly document which CSS entrypoints apps must import (tokens/forms/markdown editor) and where.

Acceptance criteria:
- Apps run with no module-resolution hacks beyond local `file:` deps.
- Apps do not depend on Underlay/Froyo/Petal deep-import paths.

---

## Section 2 — Bits-first primitives and wrappers (medium risk, high payoff)

Goal: make Bits UI the shared interaction foundation, with product-owned styling.

Note on wrappers/adapters:
- Avoid no-op component wrappers (a `.svelte` file that only re-exports an Underlay component).
- If an extension package (Froyo/Petal) does not add API policy or defaults, apps should import primitives directly from Underlay.

- [x] Froyo: broaden Bits integration beyond `Button`:
  - Select, switch/toggle, dialogs/menus where relevant.
  - Ensure wrappers expose a consistent Froyo API (variant/size/density) while delegating interaction to Bits.
- [x] Underlay: adopt Bits UI for primitives where it provides clear accessibility/interaction wins.
  - [x] Button
  - [x] dropdown/select
  - [x] dialog/modal
  - Keep Underlay tokens as the styling layer.
- [x] Define a “component contract” template (per component type):
  - Props naming (`variant`, `size`, `density`, `disabled`, `loading`)
  - Events/callbacks
  - Required ARIA hooks

### Component contract template

Use this template for any new shared primitive in Underlay (and any adapter in Froyo/Petal where needed).

**1) Purpose (1–2 sentences)**
- What interaction does it standardize?
- What accessibility / correctness risks does it eliminate?

**2) Public API**
- `variant`: semantic styling variants (`primary`, `secondary`, `subtle`, `danger`, …)
- `size`: sizing (`sm`, `md`, `lg`)
- `density`: optional (`comfortable`, `compact`) when relevant
- `disabled`: always supported
- `loading`: when an action can be in-flight (buttons, forms)
- `open` / `bind:open`: for overlays
- Floating props for portaled UI:
  - `side`, `sideOffset`, `align`, `alignOffset`, `avoidCollisions`, `collisionPadding`
- `name`/`required`: for form controls where a native input exists

**3) Slots (when applicable)**
- default slot: main content
- `trigger` slot: for trigger-only components
- `footer` slot: for dialogs

**4) Events/callbacks**
- Prefer explicit callbacks for menu items / confirm flows (`onSelect`, `onConfirm`).
- For inputs, use `bind:value` and emit `change`/`input` only if needed for compatibility.

**5) Accessibility requirements**
- Icon-only triggers must have an accessible name (`aria-label`) or wrap with a labelled component.
- Overlays:
  - Escape closes
  - click outside closes (unless explicitly disabled)
  - focus trap (unless explicitly disabled)
  - focus returns to trigger on close
- Menus/selects:
  - keyboard navigation works (up/down/enter/esc)

**6) Styling rules (especially for portals)**
- Portaled content must be styled using CSS variables + global selectors.
- Do not rely on scoped CSS variables (`v-bind(...)`) for portaled nodes.
- Prefer semantic tokens (bg/surface/text/border) over hard-coded colors.

**7) Packaging rules**
- Export from a stable barrel (`@decodelabs/underlay` and/or `@decodelabs/underlay/components`).
- Avoid deep imports unless they are explicitly supported by `package.json` `exports`.

**8) Test/verification**
- Minimum: `bun -C <pkg> check` and an SSR build in at least one consuming app.

Acceptance criteria:
- Underlay has at least one Bits-backed component in each category: button, input/select, overlay (dialog/menu).

---

## Section 3 — Tokens, themes, density (medium risk)

- [x] Underlay: adopt a single canonical token namespace (`--underlay-*`) and remove legacy aliases.
- [x] Underlay: implement density switching (comfortable vs compact).
  - [x] Support `data-underlay-density="compact"` attribute overrides
  - [x] Ensure core inputs/cards/actions use density variables
- [x] Define a shared *semantic token shape* (not values):
  - background/surface/text/border/accent/success/warn/error
  - spacing scale and typography scale

### Semantic token shape

These token names are the shared *shape* we standardize across products. Values remain product-owned.

- **Surfaces:** `--*-color-bg-surface`, `--*-color-surface-muted`
- **Text:** `--*-color-text`, `--*-color-text-muted`, `--*-color-text-subtle`
- **Borders:** `--*-color-border-subtle`, `--*-color-border-strong`
- **Accents:** `--*-color-primary`, `--*-color-primary-strong`, `--*-color-on-primary`
- **Status:** `--*-color-success`, `--*-color-warning`, `--*-color-error`
- **Spacing:** `--*-space-{1..n}`
- **Radii:** `--*-radius-sm/md/lg`, `--*-radius-pill`
- **Shadows:** `--*-shadow-sm/md/...`
- **Density:** `--*-density-gap` plus optional `[data-*-density="compact"]` overrides

Acceptance criteria:
- Apps can change density via `data-underlay-density` without ad-hoc component CSS.

---

## Section 4 — Packaging and consumption boundaries (high leverage)

- [x] Underlay: export a stable public API surface.
  - CSS entrypoints are explicitly exported (tokens/forms/markdown editor).
  - Deep imports are not part of the supported surface.
- [x] Froyo: export explicit entrypoints (`@acowtancy/froyo/editor|render|validation`).
- [x] Petal: keep a minimal, explicit package surface until it has real cross-app Songsprout extensions.
- [x] Apps: avoid source-level aliasing; prefer package entrypoints + `file:` deps for local dev.

Acceptance criteria:
- Apps can upgrade Underlay/Froyo/Petal by bumping a single dependency each.

---

## Section 5 — Extract app-specific UI up into extension packages (selective)

- [x] Acowtancy: move Acowtancy-specific Nightfire block implementations out of Underlay and into Froyo.
- [x] Acowtancy: audit Dairy for UI-only components that are truly generic and move them into Underlay (not Froyo).
  - [x] Extract `FilterBar` into Underlay patterns (`@decodelabs/underlay/patterns`).
  - [x] Extract `FormShell` into Underlay patterns (keep SvelteKit `enhance` in a Dairy wrapper).
  - [x] Extract `FormError` into Underlay components (`@decodelabs/underlay/components`).
  - [x] Extract confirmation dialog usage into `ConfirmAction` (`@decodelabs/underlay/components`).
  - [x] Extract list-page header/back-link into `PageHeader` (`@decodelabs/underlay/patterns`).
  - [x] Extract repeated “copy slug/id + actions” dropdown into `CopyActionsMenu` (`@decodelabs/underlay/patterns`).
  - [x] Add a first-class `actions` slot to `ListCard` so per-card menus can live inside the card without adding extra grid height.
  - [x] Extract small shared helpers into Underlay patterns (e.g. `copyToClipboard()`, `requestSubmitById()`).
- [x] Songsprout: only add Petal exports when there is a concrete cross-app Songsprout extension; avoid using Petal as a general primitives kit.

Remaining candidates (optional / if needed):
- Generic “accent pill”/badge component for the repeated `kind-pill`/`activity-pill`/`module-code` patterns in list cards.
- A generic list-card “meta row” helper if we want to standardize the multi-line `<span>` meta layout.

Acceptance criteria:
- Reduced duplication of generic UI patterns across apps.
- Generic patterns live in Underlay; product-specific ones live in Froyo/Petal or the app.

---

## Section 6 — Testing + release discipline (ongoing)

- [x] Define minimum validation expectations for shared packages.
  - Always run: `bun -C libraries/underlay check`.
  - For any Nightfire or styling change: also run at least one consuming app build:
    - Acowtancy: `bun -C apps/acowtancy/dairy build` and/or `bun -C apps/acowtancy/cream build`.
  - For changes affecting block save/validation: ensure a server action path imports the relevant validators (`@acowtancy/froyo/validation`).

- [x] Define a versioning policy for shared packages.
  - Treat Underlay/Froyo/Petal entrypoints as a public API surface.
  - Breaking changes require a major bump.
  - Additive/backwards-compatible changes require a minor bump.
  - Bugfixes require a patch bump.
  - Avoid deep-import paths; changes to internals should not require consumer updates.

Acceptance criteria:
- Shared packages can evolve without breaking apps unexpectedly.

---

## Reference docs

- Froyo:
  - `acowtancy/ledger/architecture/froyo/010-froyo-architecture.md`
  - `acowtancy/ledger/architecture/froyo/020-froyo-components-and-design-system.md`
- Dairy:
  - `acowtancy/ledger/architecture/dairy/010-dairy-architecture.md`
- Petal:
  - `songsprout/trellis/docs/architecture/petal/010-petal-architecture.md`
