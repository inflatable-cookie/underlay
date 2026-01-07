# 002 — Frontend (TS/Svelte) Extraction Roadmap

**Status:** Draft

This roadmap covers the **TypeScript + Svelte** side of the shared architecture.
It complements `docs/roadmap/001-extraction-roadmap.md` (Rust/backend/infra).

Scope includes:
- Shared UI kits (`@acowtancy/froyo`, `@songsprout/petal`)
- Admin app integration patterns (Dairy → Froyo + Underlay)
- Headless UI foundation via Bits UI (`bits-ui`)
- Shared conventions: tokens, component APIs, accessibility, packaging, testing

Non-goals (for this doc):
- Picking a hosting provider
- Product-specific UX decisions that belong in app roadmaps

---

## Inputs audited (current state)

### 1) Froyo (`/Users/betterthanclay/Dev/apps/acowtancy/froyo`)
- SvelteKit + library-style exports in `src/lib/index.ts`.
- Uses `bits-ui` today (notably `Button.Root`), plus `lucide-svelte` and `easymde`.
- Components currently live under `src/lib/shared/*` and `src/lib/nightfire/*`.
  - This does **not** match the intended `src/lib/ui/...` structure described in Ledger Froyo docs.
- Design tokens exist but are currently minimal in `src/lib/styles/tokens.css`.

### 2) Dairy (`/Users/betterthanclay/Dev/apps/acowtancy/dairy`)
- SvelteKit admin app.
- Consumes Froyo for app-level UI (imports of `@froyo/shared/*` and `@froyo/nightfire/*`).
- Consumes Underlay primitives directly where Froyo/Petal add no value (no-op wrappers avoided).
- Uses a local alias to Froyo source (`../froyo/src/lib`) during development.
  - This is convenient, but increases the risk of “source-level coupling” and version drift.

### 3) Petal (`/Users/betterthanclay/Dev/apps/songsprout/petal`)
- Shared UI kit with strong design tokens (`src/lib/styles/tokens.css`).
- Depends on `bits-ui`, but the current component set is mostly custom (e.g. `<button>` in `Button.svelte`).
- Current component surface is small (layout primitives, button/input, status badge).

### Shared toolchain snapshot
- All three are on Svelte 5 + SvelteKit 2.
- Froyo and Petal are very closely aligned in tooling versions.
- Dairy is slightly ahead in SvelteKit/devDependencies.

---

## Key findings (what is truly “shared”)

### A) Shared primitives (present in both ecosystems)
These exist in both Froyo and Petal conceptually and are safe to standardize:
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

## Phase checklist (high-level)

- [x] Phase 1 — Standardize frontend foundations
- [x] Phase 2 — Bits-first primitives and wrappers
- [ ] Phase 3 — Tokens, themes, density
- [ ] Phase 4 — Packaging and consumption boundaries
- [ ] Phase 5 — Extract app-specific UI up into kits
- [ ] Phase 6 — Testing + release discipline

---

## Phase 1 — Standardize frontend foundations (low risk)

- [x] Align Svelte/SvelteKit/TS/ESLint/Vite/Vitest versions within each product family:
  - [x] Acowtancy: Dairy + Froyo
  - [x] Songsprout: Petal + Bloom + Greenhouse
- [x] Standardize SSR config for Bits UI usage:
  - Ensure `ssr.noExternal` consistently includes `bits-ui` (+ any required dependencies like `svelte-toolbelt`).
- [x] Standardize iconography:
  - Confirm `lucide-svelte` as the shared icon set; define import conventions.
  - Convention: prefer per-icon imports (`lucide-svelte/icons/<icon-name>`) for consistent, tree-shakeable usage.
  - Convention: icon-only buttons must include an accessible label (`aria-label`) or use a labelled wrapper.
- [x] Standardize CSS entrypoints for UI kits:
  - Explicitly document that UI kits may import their token CSS at the package entry (`index.ts`).

Acceptance criteria:
- Apps run with no module-resolution hacks beyond local `file:` deps.

---

## Phase 2 — Bits-first primitives and wrappers (medium risk, high payoff)

Goal: make Bits UI the shared interaction foundation, with product-owned styling.

Note on wrappers/adapters:
- Avoid no-op component wrappers (a `.svelte` file that only re-exports an Underlay component).
- If a kit (Froyo/Petal) does not add API policy or defaults, apps should import primitives directly from Underlay.

- [x] Froyo: broaden Bits integration beyond `Button`:
  - Select, switch/toggle, dialogs/menus where relevant.
  - Ensure wrappers expose a consistent Froyo API (variant/size/density) while delegating interaction to Bits.
- [x] Petal: adopt Bits UI for components where it provides clear accessibility/interaction wins.
  - [x] Button
  - [x] dropdown/select
  - [x] dialog/modal
  - Keep Petal tokens as the styling layer.
- [x] Define a “component contract” template (per component type):
  - Props naming (`variant`, `size`, `density`, `disabled`, `loading`)
  - Events/callbacks
  - Required ARIA hooks

### Component contract template

Use this template for any new shared primitive in Underlay (and any adapter in Froyo/Petal).

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
- Minimum: `pnpm -C <pkg> check` and an SSR build in at least one consuming app.

Acceptance criteria:
- Both UI kits have at least one Bits-backed component in each category: button, input/select, overlay (dialog/menu).

---

## Phase 3 — Tokens, themes, density (medium risk)

- [ ] Froyo: expand tokens to match the intended scope (colors/spacing/typography/radii/shadows) and remove hard-coded values.
  - [x] Add baseline semantic tokens (text-muted/subtle, border, status colors, spacing, radii, shadows)
  - [ ] Remove remaining hard-coded literals in Froyo shared components
- [ ] Froyo: implement density switching (comfortable vs compact) per Ledger Froyo docs.
  - [x] Support `data-underlay-density="compact"` attribute overrides
  - [x] Ensure core inputs/cards/actions use density variables
- [ ] Petal: formalize density variants (artist vs admin) if needed.
- [x] Define a shared *semantic token shape* (not values):
  - background/surface/text/border/accent/success/warn/error
  - spacing scale and typography scale

### Semantic token shape

These token names are the shared *shape* we standardize across kits. Values remain product-owned.

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
- Both kits can switch density without ad-hoc CSS in apps.

---

## Phase 4 — Packaging and consumption boundaries (high leverage)

- [ ] Convert both UI kits into true library packages with predictable exports.
  - Ensure CSS token files are included correctly.
  - Ensure type generation and exports are stable.
- [ ] Remove source-level aliasing from apps where feasible.
  - Dairy should consume Froyo via package entry (still using `file:` locally is fine).
- [ ] Add “public API” discipline:
  - Only export stable components from `src/lib/index.ts` / `src/lib/index.ts`.
  - Keep internals unexported.

Acceptance criteria:
- Apps can upgrade UI kits by bumping a single dependency.

---

## Phase 5 — Extract app-specific UI up into kits (selective)

- [ ] Dairy → Froyo:
  - Identify UI-only components currently living in Dairy (e.g. filter bars, list shells) and move them into Froyo if they are generic.
  - Keep admin workflow logic in Dairy.
- [ ] Bloom/Greenhouse → Petal:
  - Identify repeated form and layout patterns and promote to Petal.

Acceptance criteria:
- Reduced duplication of generic UI patterns across apps.

---

## Phase 6 — Testing + release discipline (ongoing)

- [ ] Add minimum test coverage expectations for UI kits:
  - component smoke tests
  - accessibility checks for Bits-backed components (at least keyboard + focus)
- [ ] Define a versioning policy for UI kits:
  - breaking changes require a major bump
  - deprecations tracked in changelog notes

Acceptance criteria:
- UI kits can evolve without breaking apps unexpectedly.

---

## Reference docs

- Froyo:
  - `acowtancy/ledger/docs/architecture/froyo/010-froyo-architecture.md`
  - `acowtancy/ledger/docs/architecture/froyo/020-froyo-components-and-design-system.md`
- Dairy:
  - `acowtancy/ledger/docs/architecture/dairy/010-dairy-architecture.md`
- Petal:
  - `songsprout/trellis/docs/architecture/petal/010-petal-architecture.md`
