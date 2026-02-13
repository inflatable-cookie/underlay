# 014 - Accessibility and Keyboard Sweep

This sweep verifies core accessibility behavior across Underlay-based frontends, with emphasis on keyboard usability and assistive technology semantics.

## Problem this sweep targets

Common regressions:

- interactive UI only works with mouse
- focus is lost in dialogs/popovers/dropdowns
- form errors are visible but not announced/accessibly linked
- custom components miss ARIA roles/states
- inconsistent focus ring or hidden focus cues

## Scope

```bash
export UNDERLAY_REPO="/path/to/underlay"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `underlay`, `dairy`, `cream`.

---

## Step 1 - Build interactive surface inventory

Find key interactive constructs:

```bash
rg -n "Dialog|AlertDialog|Popover|Dropdown|Combobox|TabsRoot|TabsList|RelationSelector|Form|Field" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Build a shortlist of high-impact screens:

- auth/login/verification flows
- create/edit forms
- list/filter/sort pages
- modal/dialog-heavy flows

---

## Step 2 - Keyboard accessibility baseline checks

### 2.1 Ensure non-submit buttons are explicit in forms

```bash
rg -n "<button(?![^>]*type=)" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- buttons inside forms use explicit `type="button"` unless submit intended

### 2.2 Ensure keyboard handlers are paired with semantics

```bash
rg -n "on:keydown|onkeydown" "$ADMIN_REPO/src" "$WEB_REPO/src" "$UNDERLAY_REPO/ts/src"
```

Review each hit for:

- element role/semantic appropriateness
- `Enter`/`Space` handling for custom interactive elements
- no keyboard trap without escape path

Pass criteria:

- all custom keyboard behavior preserves expected browser/ARIA patterns

---

## Step 3 - Focus management checks

### 3.1 Dialog/popover focus lifecycle

```bash
rg -n "focus\(|focus-visible|aria-expanded|aria-haspopup|Dialog|Popover" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Manual verification:

1. open dialog/popover via keyboard
2. focus moves inside component
3. `Escape` closes where expected
4. focus returns to trigger after close

Pass criteria:

- no lost focus on open/close transitions
- no trapped focus outside intended modal contexts

### 3.2 Focus indicator visibility

```bash
rg -n "focus-visible|outline:\s*none|outline:\s*0" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- keyboard focus is always visibly indicated
- if default outline is removed, an accessible custom ring is present

---

## Step 4 - ARIA semantics and labeling checks

```bash
rg -n "aria-|role=|aria-labelledby|aria-describedby|aria-expanded|aria-selected|aria-current" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Review targets:

- combobox/listbox/option patterns
- tabs/tablist/tab semantics
- breadcrumbs/current page semantics
- toggle/switch semantics

Pass criteria:

- roles and ARIA attributes match interaction model
- state attributes (`aria-expanded`, `aria-selected`, etc.) stay in sync with UI state

---

## Step 5 - Form accessibility checks

### 5.1 Label and hint/error associations

```bash
rg -n "<Field|aria-invalid|aria-describedby|FormError|fieldErrors" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

Pass criteria:

- every input has an accessible label
- error text is associated to the field via `aria-describedby` when relevant
- invalid state reflected via `aria-invalid`

### 5.2 Required state clarity

```bash
rg -n "required|showRequiredError|NightfireFieldError" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

Pass criteria:

- required fields are clearly indicated
- required validation feedback is visible and screen-reader consumable

---

## Step 6 - Dialog and destructive action UX checks

```bash
rg -n "Dialog|AlertDialog|confirm|danger|delete" "$ADMIN_REPO/src" "$WEB_REPO/src" "$UNDERLAY_REPO/ts/src"
```

Manual checks:

- destructive confirmations are explicit
- cancel action is keyboard reachable and clear
- focus order in dialog footer is logical

Pass criteria:

- destructive flows are hard to trigger accidentally
- dialog interaction is predictable with keyboard only

---

## Step 7 - Runtime keyboard walkthrough

For each high-impact screen, test keyboard-only path:

1. `Tab` through controls in logical order
2. trigger primary/secondary actions via keyboard
3. open and close all modal/popover/tab controls
4. submit invalid form and verify accessible error feedback

Record failures with exact repro steps.

---

## Step 8 - Optional automation checks

If Playwright/Vitest is available, add a11y smoke checks:

- focus return assertions for dialogs/popovers
- tab order checks on critical forms
- axe-core style checks on key pages

These do not replace manual keyboard walkthroughs, but prevent regressions.

---

## Correction playbook

When findings are present:

1. replace ad-hoc controls with Underlay components that already encode semantics
2. fix focus lifecycle (open, trap if modal, close return)
3. add/repair ARIA state attributes and labeling
4. align form error wiring (`Field`, `aria-invalid`, `aria-describedby`)
5. add keyboard regression tests for critical journeys

---

## Severity rubric

- `high`: critical functionality inaccessible via keyboard/screen reader
- `medium`: substantial usability friction or incorrect semantics
- `low`: minor accessibility polish gap
- `note`: hardening opportunity

---

## Findings template

```md
### [SEVERITY] Accessibility gap - <component/page>

- **Location:** `src/...`
- **Interaction affected:**
- **Observed behavior:**
- **Expected accessible behavior:**
- **Who is impacted:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Accessibility sweep summary

- Screens audited: N
- High issues: N
- Medium issues: N
- Low issues: N

## Priority remediation

- Immediate fixes:
- Follow-up fixes:
```

---

## Related docs

- [090-ui-kit.md](../guides/090-ui-kit.md)
- [096-form-helpers.md](../guides/096-form-helpers.md)
- [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md)
- [003-frontend-consistency-sweep.md](./003-frontend-consistency-sweep.md)
