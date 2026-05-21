# 110 - Admin Frontend (Admin Pattern)

> **Reference Implementation**: See `acme-admin/` in the `underlay-reference` repository for a complete, working example of the admin frontend patterns described here.

This document covers creating the admin/author SvelteKit frontend following the admin frontend pattern.

For UI implementation, use Poodle as the canonical guide source:
- `Poodle Svelte Developer Guide`
- `Page Shell And Admin Recipes`
- `Form Layout And Field Recipes`
- `Admin Feature Delivery Recipes`
- `Admin App Shell Recipes`

This Underlay page should now be read for admin app structure, retained
workflow/runtime usage, and integration rules rather than generic shared UI
implementation.

Ownership rule:
- use Poodle for visible admin shell, list, detail, form, dialog, and metadata
  composition
- use Underlay for retained workflow shells, runtime/client helpers, Nightfire
  integration, and admin deployment/runtime wiring
- use the pattern files in `docs/patterns/` as full-stack/runtime delivery
  guides rather than as a second shared UI kit

Reference UI implementations now live in the ACME admin app in the separate
`underlay-reference` repository and should be treated as the real examples.

The snippets under [code/110-admin](./code/110-admin)
are now integration-oriented stubs, not the canonical UI recipe surface.

For admin feature work:
- start with [180-admin-workflow-playbook.md](./180-admin-workflow-playbook.md)
- choose the appropriate Underlay full-stack recipe in `docs/patterns/`
- follow the Poodle guides for the visible route composition inside that recipe

For dashboard and overview routes:
- use Poodle `PageHeader` for the page title/subtitle shell
- use host-owned `MetricTile` link cards for summary metrics
- use secondary sections like `LogList` or `NavCard` below that first metrics
  band
- keep overview data loading and command wiring in host code
- if the page is a workflow launcher rather than a true dashboard, prefer
  grouped `NavCard` sections with short intro copy and skip the metric band
  entirely instead of inventing placeholder stats

## Admin Frontend Structure

The admin frontend uses **layout groups** to separate authenticated routes (with sidebar) from unauthenticated routes (login/register with minimal centered layout).

## Static SPA Deployment (Acowtancy-Style)

Acowtancy’s admin frontend (Dairy) is deployed as a **pure SPA**:

- `@sveltejs/adapter-static` with `fallback: "index.html"` and `strict: true`
- `export const ssr = false` at `src/routes/+layout.ts`
- auth handled client-side via an auth store (so `App.Locals` is typically empty)
- CSP headers via Underlay’s server helpers (applied in production only; HMR needs inline scripts)

### Dependencies

Most Poodle and retained Underlay UI surfaces expect these peer dependencies to
be installed in the consuming app:

```bash
bun add bits-ui lucide-svelte
```

### Form Actions Boundary

Prefer Poodle `FormActions` for admin forms. It now owns the responsive
destructive-action treatment that used to keep this surface in Underlay:
inline `danger` slot content on wider containers and optional collapsed
`dangerItems` overflow treatment on narrower containers.

### 1) Adapter + Aliases (`svelte.config.js`)

```js
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "index.html",
      strict: true
    }),
    alias: {
      "@app": "src",
      "@cattle-grid": "../cattle-grid/src"
    },
    prerender: {
      handleUnseenRoutes: "ignore"
    }
  }
};

export default config;
```

### 2) SPA Root Layout (`src/routes/+layout.ts`)

```ts
export const ssr = false;
export const prerender = true;
```

### 3) Error Hook (Optional)

Convert Underlay HTTP errors into SvelteKit error metadata:

```ts
import { type HandleServerError } from "@sveltejs/kit";
import { UnderlayHttpError } from "@decodelabs/underlay/client/errors";

export const handleError: HandleServerError = async ({ error: err }) => {
  if (err instanceof UnderlayHttpError) {
    return { message: err.message, status: err.status, code: err.code };
  }

  return { message: "An unexpected error occurred" };
};
```

### 4) App Shell Runtime Setup (Recommended)

For SPA admin apps, configure auth and shared runtime services in `(app)/+layout.svelte`:

- `configureAuth()` for automatic token refresh in `useAuthenticatedData()`
- toast context (`createToastStore()` + Poodle `ToastHost`)
- Nightfire strategy loading (`configureNightfireStrategies()`)
- optional timezone initialization (`initTimezone()`) if profiles store timezone

The visible shell itself should follow the Poodle app-shell recipes. This
section is only about the retained runtime services that belong in the layout.

```svelte
<script lang="ts">
  import { configureAuth } from "@decodelabs/underlay/runtime/auth";
  import { configureNightfireStrategies } from "@decodelabs/underlay/nightfire/strategies";
  import { nightfireCommands } from "@cattle-grid";
  import { auth } from "$lib/stores/auth";

  configureAuth({
    getToken: () => auth.getToken(),
    onRefresh: auth.getRefreshHandler()
  });

  configureNightfireStrategies({
    fetchStrategies: async () => {
      const token = auth.getToken();
      if (!token) return [];
      return await nightfireCommands.listStrategies(fetch, token, { includeOptions: true });
    }
  });
</script>
```

### 5) Optimistic Concurrency (`If-Match` / `412`)

For admin edit forms where concurrent writes are possible:

- load detail with `ETag` (for example `getModuleAdminWithEtag`)
- send `If-Match` on update
- on `412 Precondition Failed`, reload latest server state and show a conflict message

This prevents silent overwrite when two admins edit the same record.

```
apps/admin/src/
├── app.html                  # HTML shell
├── app.d.ts                  # TypeScript declarations with Locals
├── hooks.server.ts           # Optional server error/headers hook
├── routes/
│   ├── +layout.svelte        # Minimal root layout (CSS vars, body reset)
│   ├── (app)/                # Authenticated routes WITH sidebar
│   │   ├── +layout.svelte    # App shell with sidebar navigation
│   │   ├── +layout.server.ts # Optional (SSR auth); often omitted in SPA mode
│   │   ├── +page.svelte      # Dashboard
│   │   ├── account/
│   │   ├── content/
│   │   ├── learning/
│   │   ├── logout/
│   │   └── system/
│   └── (auth)/               # Unauthenticated routes (no sidebar)
│       ├── +layout.svelte    # Centered card layout
│       └── login/
│           ├── +page.svelte
│           └── +page.server.ts  # Optional (SSR form actions)
└── lib/
    ├── api/
    │   └── client.ts         # Client factory
    └── components/
```

## Layout Groups Pattern

SvelteKit layout groups (directories wrapped in parentheses) allow different routes to use completely different layouts while sharing the same root.

### Why Layout Groups?

- **Auth routes** (login, register) should show a minimal centered layout without navigation
- **App routes** (dashboard, content, etc.) should show the full admin shell with sidebar
- Both should share the same design tokens and global styles

### Root Layout (Minimal)

The root `+layout.svelte` contains only shared CSS variables and global body styles. It imports the Underlay CSS **before** custom `:root` overrides so the overrides take precedence.

```svelte
<!-- apps/admin/src/routes/+layout.svelte -->
<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";
  // Import Underlay CSS FIRST so :root overrides take precedence
  import "@decodelabs/underlay/styles/tokens.css";
  import "@decodelabs/underlay/styles/forms.css";

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{@render children()}

<style>
  :global(:root) {
    /* App-specific design tokens */
    --admin-color-bg: #2a3037;
    --admin-color-surface: #050608;
    --admin-color-surface-subtle: #16181d;
    --admin-color-surface-card: #101318;
    --admin-color-border-subtle: rgba(255, 255, 255, 0.1);
    --admin-color-text: #f9fafb;
    --admin-color-text-muted: #9ca3af;
    --admin-color-accent: #14b8a6;

    /* Map to Underlay tokens */
    --underlay-color-bg-surface: var(--admin-color-surface);
    --underlay-color-text: var(--admin-color-text);
    --underlay-color-surface-muted: var(--admin-color-surface-card);
    --underlay-color-on-surface: var(--admin-color-text);
    --underlay-color-primary: var(--admin-color-accent);
    --underlay-color-primary-strong: #0f766e;
    --underlay-color-border-subtle: var(--admin-color-border-subtle);
  }

  :global(html) {
    font-size: 112.5%;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--admin-color-bg);
    color: var(--admin-color-text);
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }
</style>
```

### App Shell Layout (Nav + User Menu + Context Panel)

The `(app)/+layout.svelte` hosts the default admin shell:

- Left panel: brand + navigation + user menu
- Main content: scrollable
- Right panel: a pop-in context panel (AI/help/tools/actions) with a collapsed strip on desktop and a slide-in drawer on mobile

Use `Admin App Shell Recipes` in the Poodle guide set
for the visible shell composition. Keep this section as an integration note for
how retained runtime services fit into that shell.

```svelte
<!-- apps/admin/src/routes/(app)/+layout.svelte -->
<script lang="ts">
  import Menu from "lucide-svelte/icons/menu";
  import X from "lucide-svelte/icons/x";
  import PanelRight from "lucide-svelte/icons/panel-right";
  import { setContext } from "svelte";
  import { page } from "$app/stores";
  import { ToastHost } from "@poodle/svelte";
  import { UNDERLAY_TOASTS_CONTEXT_KEY, createToastStore } from "@decodelabs/underlay/runtime/feedback";
  import AdminNavList from "$lib/ui/AdminNavList.svelte";
  import AdminUserMenu from "$lib/ui/AdminUserMenu.svelte";

  let { children, data } = $props();

  const toastStore = createToastStore();
  setContext(UNDERLAY_TOASTS_CONTEXT_KEY, toastStore);

  let mobileMenuOpen = $state(false);
  let contextPanelOpen = $state(false);

  const closeMobileMenu = () => {
    mobileMenuOpen = false;
  };

  const toggleContextPanel = () => {
    contextPanelOpen = !contextPanelOpen;
  };

  const closeContextPanel = () => {
    contextPanelOpen = false;
  };

  const currentSection = $derived.by(() => {
    const path = $page.url.pathname;
    if (path.startsWith("/system")) return "system";
    return "overview";
  });
</script>

<div class="admin-app-shell">
  <!-- Mobile header (< 900px) -->
  <header class="admin-mobile-header">
    <a href="/" class="admin-mobile-header__brand">
      <span class="admin-mobile-header__title">Admin</span>
      <span class="admin-mobile-header__env">UI</span>
    </a>
    <div class="admin-mobile-header__actions">
      <button
        type="button"
        class="admin-mobile-header__icon-btn"
        aria-label={contextPanelOpen ? "Close context panel" : "Open context panel"}
        onclick={toggleContextPanel}
      >
        <PanelRight class="admin-mobile-header__icon" />
      </button>
      <button
        type="button"
        class="admin-mobile-header__icon-btn"
        aria-label={mobileMenuOpen ? "Close menu" : "Open menu"}
        aria-expanded={mobileMenuOpen}
        onclick={() => (mobileMenuOpen = !mobileMenuOpen)}
      >
        {#if mobileMenuOpen}
          <X class="admin-mobile-header__icon" />
        {:else}
          <Menu class="admin-mobile-header__icon" />
        {/if}
      </button>
    </div>
  </header>

  {#if mobileMenuOpen}
    <div class="admin-mobile-overlay" role="dialog" aria-modal="true" aria-label="Navigation menu">
      <nav class="admin-mobile-overlay__nav" aria-label="Main">
        <AdminNavList
          currentSection={data?.currentSection ?? currentSection}
          currentPath={$page.url.pathname}
          onNavigate={closeMobileMenu}
          variant="mobile"
        />
        <AdminUserMenu variant="mobile" onNavigate={closeMobileMenu} />
      </nav>
    </div>
  {/if}

  <div class="admin-main" class:admin-main--panel-open={contextPanelOpen}>
    <!-- Left nav -->
    <nav class="admin-nav" aria-label="Main">
      <div class="admin-nav__inner">
        <a href="/" class="admin-nav__brand">
          <span class="admin-nav__title">Admin</span>
          <span class="admin-nav__env">UI</span>
        </a>

        <AdminNavList
          currentSection={data?.currentSection ?? currentSection}
          currentPath={$page.url.pathname}
          variant="desktop"
        />

        <AdminUserMenu variant="desktop" />
      </div>
    </nav>

    <!-- Main content -->
    <div class="admin-content">
      <main class="admin-content__body">
        {@render children()}
      </main>
    </div>

    <!-- Right context panel -->
    <aside
      class="admin-context-panel"
      class:admin-context-panel--open={contextPanelOpen}
      aria-label="Context panel"
    >
      <button
        type="button"
        class="admin-context-panel__toggle"
        aria-label={contextPanelOpen ? "Close context panel" : "Open context panel"}
        onclick={toggleContextPanel}
      >
        <PanelRight class="admin-context-panel__toggle-icon" />
      </button>

      <div class="admin-context-panel__content">
        <div class="admin-context-panel__header">
          <h2 class="admin-context-panel__title">Context</h2>
          <button
            type="button"
            class="admin-context-panel__close"
            aria-label="Close context panel"
            onclick={closeContextPanel}
          >
            <X class="admin-context-panel__close-icon" />
          </button>
        </div>

        <div class="admin-context-panel__body">
          <p class="admin-context-panel__placeholder">Project-specific content goes here.</p>
        </div>
      </div>
    </aside>

    {#if contextPanelOpen}
      <button
        type="button"
        class="admin-context-panel__backdrop"
        aria-label="Close context panel"
        onclick={closeContextPanel}
      ></button>
    {/if}
  </div>

  <ToastHost store={toastStore} />
</div>
```

For retained integration snippets, see [code/110-admin/README.md](./code/110-admin/README.md).
Use ACME plus the Poodle admin-shell guides for the visible implementation.

### App Layout Server (Auth Check)

The `(app)/+layout.server.ts` handles authentication and redirects unauthenticated users to login.

```typescript
// apps/admin/src/routes/(app)/+layout.server.ts
import type { LayoutServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { authCommands } from "@myorg/client";

export const load: LayoutServerLoad = async ({ locals, url, fetch }) => {
  // Redirect unauthenticated users to login
  if (!locals.isAuthenticated) {
    throw redirect(302, "/login");
  }

  // Determine current section for nav highlighting
  const currentSection =
    url.pathname.startsWith("/learning") ? "learning" :
    url.pathname.startsWith("/content") ? "content" :
    url.pathname.startsWith("/account") ? "account" :
    null;

  // Fetch current user data
  let currentUser = null;
  if (locals.authToken) {
    try {
      currentUser = await authCommands.me(fetch, locals.authToken);
    } catch {
      currentUser = null;
    }
  }

  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken,
    currentUser,
    currentSection,
  };
};
```

### Auth Layout (Centered, No Sidebar)

The `(auth)/+layout.svelte` provides a minimal centered layout for login/register pages. It inherits from the root layout (getting the design tokens) but has no sidebar.

```svelte
<!-- apps/admin/src/routes/(auth)/+layout.svelte -->
<script lang="ts">
  let { children } = $props();
</script>

<div class="auth-layout">
  <div class="auth-layout__card">
    {@render children()}
  </div>
</div>

<style>
  .auth-layout {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    box-sizing: border-box;
  }

  .auth-layout__card {
    width: 100%;
    max-width: 24rem;
  }
</style>
```

## App.d.ts (Locals Pattern)

```typescript
declare module "*.svelte" {
  const component: any;
  export default component;
}

declare global {
  namespace App {
    interface Locals {
      authToken: string | null;
      isAuthenticated: boolean;
    }
  }
}

export {};
```

## Server Hooks

Create `apps/admin/src/hooks.server.ts`:

```typescript
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("myapp_admin_token") ?? null;

  event.locals.authToken = token;
  event.locals.isAuthenticated = token != null;

  return resolve(event);
};
```

## Client Factory

Create `apps/admin/src/lib/api/client.ts`:

```typescript
import { createClient as createApiClient } from "@myorg/client";
import { env } from "$env/dynamic/public";

const baseUrl =
  env.PUBLIC_API_BASE_URL ??
  env.PUBLIC_API_URL ??
  env.VITE_API_URL ??
  "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createAdminClient(
  fetchFn: typeof fetch,
  authToken: string | null | undefined
) {
  return createApiClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken ?? null
  });
}
```

## Admin Detail And CRUD UI

The generic visible admin page patterns are no longer maintained here in full.

Use these as the canonical implementation references instead:

- `Page Shell And Admin Recipes` in the Poodle guide set
- `Admin Feature Delivery Recipes` in the Poodle guide set
- `Admin App Shell Recipes` in the Poodle guide set
- the ACME admin list, detail, edit, and media-detail routes in the separate
  `underlay-reference` repository

What still belongs here:

- admin app structure
- retained runtime setup in layouts
- client and auth integration rules
- when and why `SpaFormShell` still earns shared Underlay ownership

## Complete CRUD Admin Pattern

When implementing a full admin section for a new entity type, follow this comprehensive structure. **Do NOT use dialogs for creating/editing entities** - always use dedicated routes.

### Route Structure

For an entity `Topic` nested under `Bundle`:

```
routes/(app)/learning/bundles/[bundleId]/
├── +page.svelte           # Bundle detail with Topics tab
├── topics/
│   ├── new/
│   │   └── +page.svelte   # Create topic form
│   └── [topicId]/
│       ├── +page.svelte   # Topic detail view
│       └── edit/
│           └── +page.svelte   # Edit topic form
```

### File Checklist

When creating a new admin entity, create these files in order:

1. **`$lib/cards/EntityListCard.svelte`** - List card for displaying items in a grid
2. **`$lib/forms/learning/EntityForm.svelte`** - Reusable form component
3. **`routes/.../EntityTabContent.svelte`** - Tab content with list view (navigation only, NO dialogs)
4. **`routes/.../new/+page.svelte`** - Create page using SpaFormShell
5. **`routes/.../[entityId]/+page.svelte`** - Detail view page
6. **`routes/.../[entityId]/edit/+page.svelte`** - Edit page using SpaFormShell

### Form Component Pattern

Forms should NOT contain `<form>` elements or submission logic. They render fields and use hidden inputs for values:

```svelte
<script lang="ts">
  import { FormLayout } from "@poodle/svelte";
  import {
    AlertDialog,
    Button,
    Field,
    FieldSet,
    FormActions,
    SplitButton,
    Switch,
    TextInput
  } from "@poodle/svelte";
  import { navigateOnCancel } from "@decodelabs/underlay/client/navigation";

  interface Props {
    mode?: "create" | "edit";
    values?: { name?: string; isLive?: boolean; };
    intent?: "save" | "save-close";
    errors?: Record<string, string> | null;
    cancelHref?: string;
    returnTo?: string;
    submitting?: boolean;
  }

  let {
    mode = "create",
    values = {},
    intent = $bindable("save-close"),
    errors = null,
    cancelHref = undefined,
    returnTo = undefined,
    submitting = false
  }: Props = $props();

  let nameValue = $state(values.name ?? "");
  let isLiveValue = $state(values.isLive ?? false);
  let showDeleteConfirm = $state(false);
  let actionBarElement = $state<HTMLDivElement | null>(null);
  const editIntentItems = [
    { value: "save", label: "Save changes" },
    { value: "save-close", label: "Save & close" }
  ];

  function handleCancel() {
    navigateOnCancel(cancelHref);
  }

  function handleDeleteConfirm() {
    const form = document.getElementById('entity-delete-form') as HTMLFormElement | null;
    form?.requestSubmit();
  }

  function submitWithIntent(nextIntent: "save" | "save-close") {
    intent = nextIntent;
    actionBarElement?.closest("form")?.requestSubmit();
  }

  const dangerItems = $derived(mode === "edit"
    ? [
        { label: "Cancel", onSelect: handleCancel, destructive: false },
        { label: "Soft delete entity", onSelect: () => { /* open dialog */ }, destructive: true }
      ]
    : [
        { label: "Cancel", onSelect: handleCancel, destructive: false }
      ]);
</script>

<FieldSet legend="Details">
  <FormLayout columns={1}>
    <Field label="Name" error={errors?.name} required>
      <TextInput name="name" bind:value={nameValue} required />
    </Field>
  </FormLayout>
</FieldSet>

<FieldSet legend="Status">
  <FormLayout columns={1}>
    <Field label="Visibility">
      <input type="hidden" name="isLive" value={isLiveValue ? "true" : "false"} />
      <Switch leftLabel="Draft" rightLabel="Live" bind:checked={isLiveValue} />
    </Field>
  </FormLayout>
</FieldSet>

<FormActions align="start" {dangerItems}>
  <div bind:this={actionBarElement}>
    <svelte:fragment slot="danger">
      <Button type="button" variant="ghost" onClick={handleCancel} disabled={submitting}>
        Cancel
      </Button>
      {#if mode === "edit"}
        <Button type="button" variant="ghost" tone="danger" onClick={() => (showDeleteConfirm = true)}>
          Soft delete entity
        </Button>
      {/if}
    </svelte:fragment>

    <input type="hidden" name="intent" value={intent} />
    {#if returnTo}
      <input type="hidden" name="returnTo" value={returnTo} />
    {/if}

    {#if mode === "create"}
      <Button type="submit" variant="primary" disabled={submitting}>Create entity</Button>
    {:else}
      <SplitButton
        type="submit"
        variant="primary"
        items={editIntentItems}
        disabled={submitting}
        on:click={() => submitWithIntent(intent)}
        on:action={(event) => submitWithIntent(event.detail.value as "save" | "save-close")}
      >
        {intent === "save" ? "Save changes" : "Save & close"}
      </SplitButton>
    {/if}
  </div>
</FormActions>

Use direct Poodle `FormActions` here. It now owns the responsive danger-action
contract, so admin forms no longer need a separate Underlay wrapper for the
inline-danger versus collapsed-menu behavior. When the action row should read
as a footer under stacked fields, set `showTopBorder`.

<AlertDialog
  open={showDeleteConfirm}
  title="Soft delete entity"
  description="Are you sure you want to move this entity to trash?"
  confirmLabel="Soft delete"
  tone="danger"
  onConfirm={handleDeleteConfirm}
  onCancel={() => {
    showDeleteConfirm = false;
  }}
/>
```

### Create/Edit Page Pattern

Both create and edit pages use `SpaFormShell`:

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { entityCommands } from "@client";
  import EntityForm from "$lib/forms/learning/EntityForm.svelte";
  import {
    SpaFormShell,
    consumeNavigationContext,
    submitFormWithIntent,
    type SpaFormResult
  } from "@decodelabs/underlay/patterns";

  const { backInfo, returnTo } = consumeNavigationContext("Back", defaultBackHref);

  let intent = $state<"save" | "save-close">("save-close");
  let success = $state<boolean | null>(null);
  let error = $state<string | null>(null);
  let fieldErrors = $state<Record<string, string> | null>(null);

  async function handleSubmit(formData: FormData): Promise<SpaFormResult> {
    const formIntent = String(formData.get("intent") ?? "save-close");

    // Handle delete intent (edit mode only)
    if (formIntent === "delete") {
      await entityCommands.softDelete(entityId, fetch, token);
      return { success: true, redirectTo: listUrl };
    }

    // Validate and save
    const name = String(formData.get("name") ?? "").trim();
    if (!name) {
      return { success: false, fieldErrors: { name: "Required" } };
    }

    await entityCommands.create({ name }, fetch, token);

    if (formIntent === "save-close") {
      return { success: true, redirectTo: listUrl };
    }
    return { success: true, redirectTo: editUrl };
  }

  function handleResult(result: SpaFormResult) {
    success = result.success;
    error = result.error ?? null;
    fieldErrors = result.fieldErrors ?? null;
  }

  function handleDelete() {
    submitFormWithIntent("delete");
  }
</script>

<SpaFormShell
  title="New Entity"
  backHref={backInfo.href}
  backLabel={backInfo.label}
  success={success === true}
  error={success === false && !fieldErrors ? error : null}
  {fieldErrors}
  onSubmit={handleSubmit}
  onResult={handleResult}
  navigate={goto}
>
  <EntityForm mode="create" {errors} cancelHref={backInfo.href} {returnTo} bind:intent />
</SpaFormShell>

<!-- For edit pages: hidden delete form -->
<form id="entity-delete-form" style="display: none;" onsubmit={(e) => { e.preventDefault(); handleDelete(); }}>
  <input type="hidden" name="intent" value="delete" />
</form>
```

Boundary note:
- keep `SpaFormShell` for save/save-close/delete workflow orchestration,
  navigation, and field-error/result wiring
- pass a banner error only when there is a real top-level route failure; if
  `fieldErrors` are present, suppress the banner error instead of repeating the
  same problem twice
- let Poodle own the visual shell pieces inside it, especially callouts and
  card framing, instead of reintroducing app-local status wrappers
- use Poodle `MetaBar` and `MetaItem` for both detail-page and edit-header
  metadata rows, and compose copyable values with Poodle `Code` and
  `showCopyButton` when needed

For navigation-context behavior inside tabbed detail routes, keep the current
rule explicit:

- when a detail page has tabs, `sourceContext.href` should include the current
  tab so back navigation returns to the correct panel

For visible tab/list/header examples, use the Poodle guides and ACME route
family above instead of the older embedded examples that used to live here.

## Key Points

1. **CSS Import Order**: Import Underlay CSS in the root layout BEFORE custom `:root` styles so your overrides take precedence.

2. **No `@` Reset Needed**: Since all routes inherit from the minimal root layout, you don't need `+layout@.svelte` to break out of a parent layout. The layout groups naturally separate the concerns.

3. **Auth Logic in (app)**: Only the `(app)` layout group has auth checking. The `(auth)` group is accessible without authentication.

4. **Shared Design Tokens**: Both layout groups inherit from root, so they share the same design tokens and styling.

## Next Steps

- [120-configuration.md](./120-configuration.md)
