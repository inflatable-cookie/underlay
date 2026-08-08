# 002 - Underlay Reuse Sweep

This sweep detects and corrects a common drift in Underlay-based apps: teams (or AI agents) rebuilding UI primitives and patterns that already exist in Underlay.

Primary target areas:

1. Form input widgets
2. List controls (filter bars, ordering, pagination)
3. List cards
4. Form dialogs

## Why this sweep exists

Reimplementation causes:

- inconsistent UX and accessibility behavior
- duplicated bug surface and styling drift
- harder migrations when shared components evolve
- slower delivery due to repeated local implementations

The goal is not "zero custom components". The goal is:

- reuse first
- extend shared components when needed
- justify and document true exceptions

## Scope

Run this across all consuming frontends in a project (admin, web, and any additional apps).

Set paths first:

```bash
export UNDERLAY_REPO="/path/to/underlay"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

For Acowtancy these typically map to:

- `underlay`
- `dairy`
- `cream`

---

## Step 1 - Build the canonical reusable surface map

Before finding duplicates, identify what already exists.

```bash
rg -n "^export \{ default as " "$UNDERLAY_REPO/ts/src/components/index.ts"
rg -n "^export \{ default as |^export \{" "$UNDERLAY_REPO/ts/src/patterns/index.ts"
```

Focus on these canonical items for this sweep:

- **Form widgets:** `Field`, `TextInput` (including multiline/textarea usage), `Select`, `Switch`, `DateInput`, `DateTimeInput`, `NumberInput`, `DurationInput`, `MarkdownEditor`, `FormActions`
- **List controls:** `FilterBar`, `OrderBy`, `Pagination`, `DataTable`, `createListController`, `BatchActionBar`, `LogList`, `CopyActionsMenu`
- **List cards:** `ListCard`, `InlineListCard`, `InlineListItem`
- **Form dialogs:** `FormDialog` (patterns), `Dialog`, `AlertDialog`, `FormActions`

---

## Step 2 - Measure current Underlay adoption

```bash
rg -n "@inflatable-cookie/underlay/(components|patterns)" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Record:

- Which feature areas are already using shared components
- Which areas import few/no Underlay components

Low Underlay import density in form-heavy or list-heavy areas is a strong reinvention signal.

---

## Step 3 - Detect likely reinvention (automated pass)

Treat these commands as candidate generation, not final verdicts.

### 3.1 Form input widget reinvention

Find raw inputs/selects/textareas in app code:

```bash
rg -n "<(input|select|textarea)\b" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Find local wrapper components likely duplicating shared widgets:

```bash
rg -n "export let|\$props\(" "$ADMIN_REPO/src" "$WEB_REPO/src" -g "**/*{Input,Select,Textarea,Switch,Field}*.svelte"
```

Find direct `bits-ui` usage in consuming apps (often a sign of bypassing shared primitives):

```bash
rg -n "from \"bits-ui\"|from 'bits-ui'" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Expected outcome:

- raw HTML controls used only where justified (for example hidden/file internals)
- primary forms built from Underlay form components

### 3.2 List controls reinvention

Find list pages with filter/order/pagination concepts:

```bash
rg -n "filter|search|sort|order|page|pagination|limit" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Check if those areas use shared list controls:

```bash
rg -n "FilterBar|OrderBy|Pagination|DataTable|createListController|BatchActionBar|LogList|CopyActionsMenu" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Flag candidates where list functionality exists but shared controls are absent.

### 3.3 List card reinvention

Find custom card-like components:

```bash
rg -n "class=\"[^"]*card|class='[^']*card" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "ListCard|InlineListCard|InlineListItem" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Also inspect local component names:

```bash
rg -n "export let|\$props\(" "$ADMIN_REPO/src" "$WEB_REPO/src" -g "**/*{Card,Tile,Row,Item}*.svelte"
```

Expected outcome:

- list item/card UI favors shared card primitives
- custom cards exist only for truly domain-specific structure

### 3.4 Form dialog reinvention

Find dialog implementations in consuming apps:

```bash
rg -n "FormDialog|Dialog|AlertDialog|BitsDialog|BitsAlertDialog|modal|drawer" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Find suspicious footer/action layouts (missing shared `FormActions` conventions):

```bash
rg -n "type=\"submit\"|Cancel|onCancel|onConfirm" "$ADMIN_REPO/src" "$WEB_REPO/src" -g "**/*{Dialog,Modal}*.svelte"
```

Expected outcome:

- form dialogs use `FormDialog` (preferred) or shared `Dialog` + `Form` + `FormActions`
- no ad-hoc divergent footer conventions

---

## Step 4 - Triage rubric (is this a real violation?)

For each candidate, classify as:

### A) Must migrate to shared component

- local component duplicates existing Underlay behavior with minor style differences
- reinvention creates inconsistent keyboard/accessibility behavior
- repeated implementation across multiple pages

### B) Extend Underlay, then migrate

- requirement is broadly reusable
- local implementation exists only because shared component lacks one capability
- change should benefit multiple apps

### C) Allowed exception (document)

- unique domain workflow not representable by existing shared components
- replacement would materially reduce clarity/UX
- exception has an ADR-lite note and test coverage

---

## Step 5 - Correction playbooks

### 5.1 Form input widgets

Replace:

- raw `<input>/<select>/<textarea>` wrappers

With:

- `Field` + `TextInput`/`Select`/`Switch`

Checklist:

1. Keep existing field names/ids stable.
2. Move label/hint/error responsibilities into `Field`.
3. Preserve validation wiring via existing form data/action state.
4. Remove redundant local style wrappers if no longer needed.

### 5.2 List controls

Replace:

- ad-hoc filter rows and ordering bars

With:

- `FilterBar`, `OrderBy`, `Pagination`, or `DataTable` where tabular

Checklist:

1. Normalize filter state shape.
2. Use shared control components for rendering.
3. Keep query param behavior unchanged unless intentionally refactoring.
4. Add/adjust tests for filter persistence and pagination behavior.

### 5.3 List cards

Replace:

- repeated custom card markup in list/grid views

With:

- `ListCard` (or `InlineListCard` + `InlineListItem` for compact inline lists)

Checklist:

1. Map title/subtitle/meta/actions to existing `ListCard` slots/props.
2. Keep route links and actions behavior unchanged.
3. Preserve selection/batch mode behavior where present.

### 5.4 Form dialogs

Replace:

- custom modal/form combinations and inconsistent footers

With:

- `FormDialog` pattern, or shared `Dialog` + `Form` + `FormActions`

Checklist:

1. Keep dialog open/close state API stable for callers.
2. Use shared submit/cancel action layout.
3. Ensure keyboard/escape behavior matches shared dialog behavior.
4. Verify loading/error states are surfaced consistently.

---

## Step 6 - Prevent regression

After remediation, add guardrails:

1. Update app-level `AGENTS.md` with explicit "reuse Underlay first" rules.
2. Add PR checklist item: "Did this recreate existing Underlay component/pattern?"
3. Add lightweight lint/search checks in CI for known reinvention hotspots.

Suggested CI searches:

```bash
rg -n "from \"bits-ui\"|from 'bits-ui'" src
rg -n "<(input|select|textarea)\b" src/routes src/lib/forms
```

Use allowlists where legitimate exceptions exist.

---

## Reporting template

```md
### [AREA] [SEVERITY] Reinvention finding title

- **Location:** `src/...`
- **Existing shared alternative:** `@inflatable-cookie/underlay/components|patterns` item
- **Why this is duplicate:**
- **Classification:** Must migrate / Extend then migrate / Allowed exception
- **Remediation plan:**
- **Owner:**
- **Target date:**
```

Summary section:

```md
## Reuse sweep summary

- Candidates found: N
- Must migrate: N
- Extend then migrate: N
- Allowed exceptions: N

## Follow-up

- Shared component enhancements needed:
- App-level migrations queued:
```

---

## Related docs

- [090-ui-kit.md](../guides/090-ui-kit.md)
- [096-form-helpers.md](../guides/096-form-helpers.md)
- [097-autonomous-list-components.md](../guides/097-autonomous-list-components.md)
- [100-frontend-web.md](../guides/100-frontend-web.md)
- [110-admin.md](../guides/110-admin.md)
- [000-index.md](../patterns/000-index.md)
