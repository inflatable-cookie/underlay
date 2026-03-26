# 003 - Frontend Consistency Sweep

This sweep verifies that Underlay-based frontends stay consistent across apps (admin/web) in architecture, API usage, state patterns, and UX conventions.

Use this when you want to prevent "same feature, different implementation style" drift.

## Scope

Run across all frontend repos in a project.

Set paths first:

```bash
export UNDERLAY_REPO="/path/to/underlay"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
export CLIENT_REPO="/path/to/myapp-client"
```

Acowtancy mapping:

- `underlay`, `dairy`, `cream`, `cattle-grid`

---

## Step 1 - App shell and route architecture consistency

### 1.1 Verify route group structure

```bash
rg -n "\(app\)|\(auth\)" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- Authenticated and unauthenticated areas are clearly separated where applicable.
- Protected pages are not mixed into public/auth groups by accident.

### 1.2 Check root layout strategy

```bash
rg -n "export const ssr|export const prerender" "$ADMIN_REPO/src/routes/+layout.ts" "$WEB_REPO/src/routes/+layout.ts"
rg -n "configureAuth|ToastHost|createToastStore|configureNightfireStrategies" "$ADMIN_REPO/src/routes/(app)/+layout.svelte"
```

Pass criteria:

- Layout strategy matches intended deployment model (SSR vs SPA).
- Shared runtime setup is centralized in app shell rather than repeated in pages.

---

## Step 2 - API boundary consistency

### 2.1 Prevent raw backend fetch sprawl

```bash
rg -n "fetch\(" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "@cattle-grid|commands" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Classify raw `fetch` hits:

- allowed: auth/bootstrap infrastructure, explicit low-level integration cases
- violation: routine feature/API calls that should use typed client commands

Pass criteria:

- Most feature calls flow through typed client commands.
- Raw fetch usage is rare and justified.

### 2.2 Validate centralized client config

```bash
rg -n "configureCattleGrid\(|resolvePublicApiConfig|PUBLIC_API_URL|PUBLIC_API_VERSION" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- API base/version setup is consistent and centralized.
- Deprecated env aliases (if any) are handled intentionally.

---

## Step 3 - Naming, structure, and import hygiene

### 3.1 File naming conventions

```bash
find "$ADMIN_REPO/src" "$WEB_REPO/src" -name "*.svelte" -o -name "*.ts"
```

Review for:

- Svelte components in expected project style (commonly PascalCase)
- TS utility/module files in expected style (commonly kebab-case)
- route segment names consistent with project conventions

### 3.2 Deep relative import drift

```bash
rg -n "from ['\"]\.\./\.\./\.\./" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "from ['\"]\$lib|from ['\"]@cattle-grid|from ['\"]@decodelabs/underlay" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Alias imports are preferred over deep relative paths.
- Deep relative imports are exceptions, not standard practice.

---

## Step 4 - State management consistency

### 4.1 Token handling policy

```bash
rg -n "authToken|getToken\(|return\s*\{[^}]*token|load\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- Tokens are not passed through page/load data.
- Components use auth store/providers directly per project policy.

### 4.2 URL state for tabs/filters/pagination

```bash
rg -n "historyKey=|\?tab=|page=|limit=|sort=|order=" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Key UI state is URL-backed where expected (tabs, pagination, filters).
- Behavior is consistent between similar pages.

---

## Step 5 - UX pattern consistency (forms, lists, dialogs)

This step complements [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md). Here we focus on consistency between pages, not just reuse.

### 5.1 Form structure consistency

```bash
rg -n "<Form\b|<Field\b|<FormActions\b|SaveSplitButton|submitFormWithIntent" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "<form\b" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Equivalent form flows use shared structure patterns.
- Ad-hoc `<form>` usage is justified and not divergent by default.

### 5.2 List page consistency

```bash
rg -n "FilterBar|OrderBy|Pagination|DataTable|ListCard|BatchActionBar|LogList|CopyActionsMenu" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Similar list pages use similar controls and interaction patterns.
- No feature-specific one-offs without rationale.

### 5.3 Dialog/footer action consistency

```bash
rg -n "FormDialog|Dialog|AlertDialog|FormActions|Button|Cancel" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Form dialogs follow shared footer/action conventions.
- Cancel/submit behavior is predictable and consistent.

---

## Step 6 - Loading/error/empty state consistency

```bash
rg -n "PageLoading|FormError|empty|No .* found|Loading" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Every data-heavy page has explicit loading, error, and empty states.
- Similar pages use the same components/messages style.

---

## Step 7 - Theme/token and styling consistency

### 7.1 Ensure Underlay base styles are used

```bash
rg -n "@decodelabs/underlay/styles" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

### 7.2 Detect hardcoded style drift hotspots

```bash
rg -n "#[0-9a-fA-F]{3,8}|rgb\(|hsl\(|font-family:" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Review context:

- acceptable: theme definition files/layout roots
- flag: random hardcoded values spread across feature components

Pass criteria:

- Design tokens/themes are centralized.
- Feature-level hardcoded visual values are minimized.

---

## Step 8 - Accessibility and interaction baseline consistency

```bash
rg -n "onclick=|on:click=|type=\"button\"|aria-|role=|tabindex=" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Check for:

- non-submit buttons inside forms explicitly use `type="button"`
- controls have labels/ARIA where needed
- custom interactive elements are keyboard accessible

Pass criteria:

- no repeated a11y anti-patterns in one app but not the other

---

## Step 9 - Findings classification

Classify each finding:

- `critical`: architecture violation likely to cause security/data/correctness issues
- `high`: broad user-facing inconsistency or high-maintenance divergence
- `medium`: clear drift that should be normalized soon
- `low`: minor inconsistency/hygiene issue
- `note`: informational alignment opportunity

---

## Report template

```md
### [AREA] [SEVERITY] Consistency finding title

- **Location:** `src/...`
- **Check step:** Step X.Y
- **Expected pattern:**
- **Observed drift:**
- **Recommended normalization:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved / Exception documented
```

Summary section:

```md
## Frontend consistency sweep summary

- Critical: 0
- High: 0
- Medium: 0
- Low: 0
- Notes: 0

## Normalization plan

- Immediate fixes:
- Follow-up refactors:
- Accepted exceptions:
```

---

## Related docs

- [002-underlay-reuse-sweep.md](./002-underlay-reuse-sweep.md)
- [090-ui-kit.md](../guides/090-ui-kit.md)
- [096-form-helpers.md](../guides/096-form-helpers.md)
- [097-autonomous-list-components.md](../guides/097-autonomous-list-components.md)
- [100-frontend-web.md](../guides/100-frontend-web.md)
- [110-admin.md](../guides/110-admin.md)
- [120-configuration.md](../guides/120-configuration.md)
