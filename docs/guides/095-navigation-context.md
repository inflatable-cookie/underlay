# 095 - Navigation Context

This document covers Underlay's navigation context system for contextual back buttons and form redirects. This pattern enables edit forms to know where users came from and provide appropriate "back" navigation.

## Overview

When users navigate to an edit form, they could arrive from multiple places:
- A list view (e.g., `/videos`)
- A detail view (e.g., `/videos/123`)
- A nested context (e.g., `/modules/456/videos`)
- Deep links or external sources

The navigation context system:
- **Tracks navigation history** in sessionStorage with sanity rules
- **Provides contextual back buttons** with appropriate labels
- **Enables form redirects** to return users where they came from
- **Survives page refresh** (sessionStorage-backed)
- **Prevents unbounded growth** with max depth and deduplication

---

## Quick Start

### 1. Navigate with Context

When navigating to an edit page, use `gotoWithContext()` to record where the user came from:

```svelte
<script lang="ts">
  import { gotoWithContext } from "@decodelabs/underlay/client";
  import { Menu } from "@poodle/svelte-primitives";

  export let data;
</script>

<Menu
  items={[{ value: "edit", label: "Edit video" }]}
  triggerAriaLabel="Video actions"
  on:action={() =>
    void gotoWithContext(`/videos/${data.video.videoId}/edit`, {
      label: data.video.title,
      href: `/videos/${data.video.videoId}`,
      type: "detail"
    })
  }
>
  <svelte:fragment slot="trigger">
    <button type="button">Actions</button>
  </svelte:fragment>
</Menu>
```

### 2. Consume Context in Edit/Create Page

In the edit or create page, use `consumeNavigationContext()` to **pop** the context from the stack and get both back button info and return URL:

```svelte
<script lang="ts">
  import { consumeNavigationContext } from "@decodelabs/underlay/runtime";
  import { PageHeader } from "@decodelabs/underlay/runtime";

  export let data;

  const defaultBackHref = `/videos/${data.video.videoId}`;
  const { backInfo, returnTo } = consumeNavigationContext("Back to video", defaultBackHref);
</script>

<PageHeader
  title="Edit Video"
  subtitle={data.video.title}
  backHref={backInfo.href}
  backLabel={backInfo.label}
/>

<!-- Pass returnTo to form for server redirect -->
<VideoForm {returnTo} ... />
```

> **Important:** Use `consumeNavigationContext()` for edit/create pages. This function **pops** the context from the stack, ensuring it's only used once and doesn't persist across multiple navigations.

Navigation seam:
- use `@decodelabs/underlay/client` for SvelteKit navigation wrappers like `gotoWithContext()`, `navigateBack()`, and `navigateOnCancel()`
- use `@decodelabs/underlay/runtime` for framework-agnostic navigation context and page-state helpers like `consumeNavigationContext()`, `getBackButtonInfo()`, and `storePageState()`

### 3. Handle Server Redirect

In the form component, include a hidden `returnTo` field:

```svelte
<!-- VideoForm.svelte -->
<script lang="ts">
  interface Props {
    returnTo?: string;
    // ... other props
  }

  let { returnTo = undefined }: Props = $props();
</script>

{#if returnTo}
  <input type="hidden" name="returnTo" value={returnTo} />
{/if}
```

In the server action, read `returnTo` and use it for the redirect:

```typescript
// +page.server.ts
export const actions: Actions = {
  default: async ({ params, request }) => {
    const formData = await request.formData();
    const returnTo = String(formData.get("returnTo") ?? "").trim() || null;

    // ... validate and save ...

    if (intent === "save-close") {
      // Use returnTo if provided and safe (relative path only)
      const redirectTarget = returnTo && returnTo.startsWith("/")
        ? returnTo
        : `/videos/${params.videoId}`;
      throw redirect(303, redirectTarget);
    }

    return { success: true };
  }
};
```

---

## API Reference

### Types

```typescript
/**
 * Navigation context representing where the user came from.
 */
interface NavigationContext {
  /** Display label for back button (e.g., "Videos", "Module: Intro") */
  label: string;
  /** URL to navigate back to */
  href: string;
  /** Type of page - used for breadcrumb collapse rules */
  type: "list" | "detail" | "edit";
  /** Target URL this context is intended for (used for stale context validation) */
  targetHref?: string;
  /** Optional page state snapshot (tabs, pagination, filters, etc.) */
  state?: Record<string, unknown>;
}

/**
 * Configuration options for the navigation context system.
 */
interface NavigationContextConfig {
  /** Storage key (default: "underlay:nav-context") */
  storageKey?: string;
  /** Maximum breadcrumb depth (default: 3) */
  maxDepth?: number;
}

/**
 * Return type for getBackButtonInfo() and consumeNavigationContext().
 */
interface BackButtonInfo {
  /** Label for the back button */
  label: string;
  /** Href for the back button */
  href: string;
  /** True when derived from stored navigation context (not fallback) */
  isContextual?: boolean;
}
```

### Framework-Agnostic Functions (`@decodelabs/underlay/runtime`)

These functions work in any JavaScript environment with sessionStorage.

#### `pushNavigationContext(context)`

Push a navigation context onto the stack.

```typescript
import { pushNavigationContext } from "@decodelabs/underlay/runtime";

pushNavigationContext({
  label: "Videos",
  href: "/content/videos",
  type: "list"
});
```

**Sanity rules applied:**
- **Max depth**: Stack is trimmed to 3 items (configurable)
- **Same-type collapse**: `list→list` replaces the top item
- **Deduplication**: Same `href` already in stack? Moves to top, doesn't duplicate

#### `popNavigationContext()`

Remove and return the most recent context from the stack.

```typescript
import { popNavigationContext } from "@decodelabs/underlay/runtime";

const context = popNavigationContext();
if (context) {
  console.log(`Returning to: ${context.label}`);
  navigateTo(context.href);
}
```

**Returns:** `NavigationContext | null`

#### `peekNavigationContext()`

Read the most recent context without removing it.

```typescript
import { peekNavigationContext } from "@decodelabs/underlay/runtime";

const context = peekNavigationContext();
console.log(`User came from: ${context?.label ?? "unknown"}`);
```

**Returns:** `NavigationContext | null`

#### `getNavigationContextStack()`

Get the full navigation context stack (for debugging or breadcrumbs).

```typescript
import { getNavigationContextStack } from "@decodelabs/underlay/runtime";

const stack = getNavigationContextStack();
// [
//   { label: "Learning", href: "/learning", type: "list" },
//   { label: "Pathways", href: "/learning/pathways", type: "list" },
//   { label: "FA 2025", href: "/learning/pathways/abc", type: "detail" }
// ]
```

**Returns:** `NavigationContext[]`

#### `clearNavigationContext()`

Clear all navigation context.

```typescript
import { clearNavigationContext } from "@decodelabs/underlay/runtime";

clearNavigationContext();
```

#### `getReturnUrl(fallbackHref)`

Get the URL for form submission redirects. **Peeks** at the context without consuming it.

If the stored context includes a `targetHref` and it doesn't match the current URL pathname, the context is ignored and the fallback is used.

> **Note:** For edit/create pages, prefer `consumeNavigationContext()` which pops the context.

```typescript
import { getReturnUrl } from "@decodelabs/underlay/runtime";

const returnTo = getReturnUrl(`/items/${itemId}`);
// If context exists: "/content/videos"
// If no context:     "/items/123"
```

**Parameters:**
- `fallbackHref` - URL to return if no context exists

**Returns:** `string`

#### `getBackButtonInfo(fallbackLabel, fallbackHref)`

Get contextual back button label and href. **Peeks** at the context without consuming it.

If the stored context includes a `targetHref` and it doesn't match the current URL pathname, the context is ignored and the fallbacks are used.

> **Note:** For edit/create pages, prefer `consumeNavigationContext()` which pops the context. Use `getBackButtonInfo()` for detail pages that display a back button but don't have forms.

```typescript
import { getBackButtonInfo } from "@decodelabs/underlay/runtime";

const { label, href } = getBackButtonInfo("Back to item", `/items/${itemId}`);
// If context exists: { label: "Back to Videos", href: "/content/videos" }
// If no context:     { label: "Back to item", href: "/items/123" }
```

**Parameters:**
- `fallbackLabel` - Label to use if no context exists
- `fallbackHref` - URL to use if no context exists

**Returns:** `BackButtonInfo`

#### `consumeNavigationContext(fallbackLabel, fallbackHref)` ⭐ Recommended

**Pops** the navigation context and returns both back button info and return URL. This is the recommended function for edit/create pages because it ensures the context is consumed (removed from the stack) when used.

**Stale context detection:** If the context has a `targetHref` that doesn't match the current URL pathname, the context is considered stale and discarded. This prevents showing incorrect "Back to X" labels when users navigate to edit pages via bookmarks, direct links, or from pages that don't push context.

```typescript
import { consumeNavigationContext } from "@decodelabs/underlay/runtime";

const { backInfo, returnTo } = consumeNavigationContext("Back to videos", "/content/videos");
// backInfo: { label: "Back to Module: FA1", href: "/learning/modules/abc" }
// returnTo: "/learning/modules/abc"
```

**Parameters:**
- `fallbackLabel` - Label to use if no context exists or context is stale
- `fallbackHref` - URL to use if no context exists or context is stale

**Returns:** `{ backInfo: BackButtonInfo, returnTo: string }`

**Why use this instead of separate calls?**
- Ensures context is consumed only once
- Validates context is intended for the current page (prevents stale context bugs)
- Prevents stale context from persisting across multiple page navigations
- Provides both back button info and form redirect URL in one call

#### `computeBackInfo(backInfo, fallback?)` ⭐ Recommended for Dynamic Fallbacks

Compute back button info with a data-dependent fallback, while always respecting contextual navigation. This helper ensures that contextual back links (from the user's actual navigation path) take precedence over hardcoded fallbacks.

**Use this when:** Your fallback depends on data that loads asynchronously (e.g., the entity being edited).

```typescript
import { computeBackInfo, consumeNavigationContext } from "@decodelabs/underlay/runtime";

// In page initialization
const { backInfo, returnTo } = consumeNavigationContext("Back to module", defaultBackHref);

// In reactive context (Svelte $derived, Vue computed, etc.)
const computedBackInfo = $derived(
  computeBackInfo(backInfo, module ? {
    href: `/learning/modules/${module.moduleId}`,
    label: `Back to ${module.code}`
  } : undefined)
);
```

**Parameters:**
- `backInfo` - The BackButtonInfo from `consumeNavigationContext()`
- `fallback` - Optional object with `href` and `label` for when no contextual navigation exists

**Returns:** `BackButtonInfo`

**Why use this?**

Without `computeBackInfo`, you might write:

```typescript
// ❌ BAD - Easy to forget isContextual check
const computedBackInfo = $derived(
  module
    ? { href: `/modules/${module.id}`, label: "Back to module", isContextual: false }
    : backInfo
);
```

This ignores contextual navigation! If the user arrived from a different page (e.g., a list with filters), they'll lose that context.

With `computeBackInfo`:

```typescript
// ✅ GOOD - Always respects contextual navigation
const computedBackInfo = $derived(
  computeBackInfo(backInfo, module ? {
    href: `/modules/${module.id}`,
    label: "Back to module"
  } : undefined)
);
```

The helper ensures:
1. Contextual navigation (from the user's actual path) is always used when available
2. The data-dependent fallback is only used when there's no contextual navigation
3. The original `backInfo` defaults are used if neither contextual nor fallback is available

#### `deriveParentPath(currentPath)`

Derive a sensible parent URL from a path.

```typescript
import { deriveParentPath } from "@decodelabs/underlay/runtime";

deriveParentPath("/content/videos/123/edit");  // "/content/videos/123"
deriveParentPath("/content/videos/123");       // "/content/videos"
deriveParentPath("/content/videos");           // "/content"
deriveParentPath("/");                         // "/"
```

**Parameters:**
- `currentPath` - The current URL path

**Returns:** `string`

#### `configureNavigationContext(options)`

Configure the navigation context system. Call early in app initialization if needed.

```typescript
import { configureNavigationContext } from "@decodelabs/underlay/runtime";

configureNavigationContext({
  storageKey: "myapp:nav-context",
  maxDepth: 5
});
```

**Parameters:**
- `options.storageKey` - Storage key (default: `"underlay:nav-context"`)
- `options.maxDepth` - Maximum stack depth (default: `3`)

---

### SvelteKit Functions (`@decodelabs/underlay/client`)

These functions integrate with SvelteKit's navigation.

#### `gotoWithContext(targetHref, context, options?)`

Navigate to a URL while pushing context onto the stack.

The `targetHref` is automatically stored with the context, allowing `consumeNavigationContext()` to validate that the context is intended for the current page. This prevents stale context from being used when users navigate via bookmarks, direct links, or from pages that don't push context.

If the context includes a `state` object, it will be stored in sessionStorage keyed by the context's `href` pathname. This state can be restored using `initPageState()` when the user navigates back.

```typescript
import { gotoWithContext } from "@decodelabs/underlay/client";

// From a list page
await gotoWithContext(`/items/${id}/edit`, {
  label: "Items",
  href: "/items",
  type: "list"
});

// From a detail page with state to restore
await gotoWithContext(`/items/${id}/edit`, {
  label: item.name,
  href: `/items/${id}`,
  type: "detail",
  state: { activeTab: "details", currentPage: 2 }
});
```

**Parameters:**
- `targetHref` - URL to navigate to
- `context` - NavigationContext to push (including optional `state`)
- `options` - Optional SvelteKit goto options

**Returns:** `Promise<void>`

#### `navigateBack(fallbackHref?)`

Navigate back using the context stack.

```typescript
import { navigateBack } from "@decodelabs/underlay/client";

// Uses context if available, otherwise derives parent URL
navigateBack();

// With explicit fallback
navigateBack(`/items/${itemId}`);
```

**Parameters:**
- `fallbackHref` - Optional fallback URL if no context exists

**Returns:** `string` (the href navigated to)

#### `navigateOnCancel(cancelHref?)`

Legacy cancel button navigation. Navigates to the provided href, derives a parent URL, or uses browser history.

```typescript
import { navigateOnCancel } from "@decodelabs/underlay/client";

function handleCancel() {
  navigateOnCancel(); // Derives parent from current URL
}

function handleCancelWithFallback() {
  navigateOnCancel("/items"); // Uses provided URL
}
```

**Parameters:**
- `cancelHref` - Optional explicit URL to navigate to

> **Note:** Prefer `navigateBack()` for context-aware navigation. `navigateOnCancel()` is provided for backwards compatibility.

---

## Page State Restoration

When users navigate away from a page (e.g., to edit a nested item), then navigate back, they expect the page to be in the same state they left it - same tab selected, same pagination page, same filters applied.

The navigation context system supports **page state restoration** to enable this. State is stored in sessionStorage, keyed by pathname, and restored when the user navigates back.

### How It Works

1. **When navigating away**: Pass a `state` object in the context to `gotoWithContext()`
2. **State is stored**: Keyed by the source page's pathname in sessionStorage
3. **When navigating back**: Call `initPageState()` in `onMount` to restore state
4. **State is consumed**: Removed from storage after restoration to prevent stale state

### Quick Start

#### 1. Save State When Navigating Away

```typescript
import { gotoWithContext } from "@decodelabs/underlay/client";

// Current page state
let activeTab = $state("details");
let currentPage = $state(1);

// When navigating to edit page, save current state
void gotoWithContext(`/items/${id}/edit`, {
  label: "Items",
  href: "/items",
  type: "list",
  state: { activeTab, currentPage }  // State to restore on return
});
```

#### 2. Restore State on Return

```typescript
import { onMount } from "svelte";
import { initPageState } from "@decodelabs/underlay/client";

let activeTab = $state("details");
let currentPage = $state(1);

onMount(() => {
  // Restore state if returning via back navigation
  const restored = initPageState({
    activeTab: "details",  // Default values
    currentPage: 1
  });
  
  activeTab = restored.activeTab;
  currentPage = restored.currentPage;
});
```

### API Reference

#### State Storage Functions (`@decodelabs/underlay/runtime`)

##### `storePageState(pathname, state)`

Store state for a specific pathname. Called automatically by `gotoWithContext()` when state is provided, but can be called manually.

```typescript
import { storePageState } from "@decodelabs/underlay/runtime";

storePageState("/learning/modules/123", {
  activeTab: "syllabus",
  expandedSections: ["section-a", "section-b"]
});
```

**Parameters:**
- `pathname` - The pathname to associate state with
- `state` - Object containing state values

##### `retrievePageState<T>(pathname)`

Retrieve stored state for a pathname without consuming it.

```typescript
import { retrievePageState } from "@decodelabs/underlay/runtime";

const state = retrievePageState<{ activeTab: string }>("/learning/modules/123");
if (state) {
  console.log(`Active tab: ${state.activeTab}`);
}
```

**Parameters:**
- `pathname` - The pathname to retrieve state for

**Returns:** `T | null`

##### `consumePageState<T>(pathname?)`

Retrieve and **remove** stored state. The state is deleted from storage after retrieval.

```typescript
import { consumePageState } from "@decodelabs/underlay/runtime";

// Uses current pathname by default
const state = consumePageState<{ activeTab: string }>();
```

**Parameters:**
- `pathname` - Optional pathname (defaults to `window.location.pathname`)

**Returns:** `T | null`

##### `clearPageStates()`

Clear all stored page states.

```typescript
import { clearPageStates } from "@decodelabs/underlay/runtime";

clearPageStates();
```

#### SvelteKit Helper Functions (`@decodelabs/underlay/client`)

##### `initPageState<T>(defaults)`

Initialize page state from storage, merged with defaults. This is the primary way to restore state in Svelte components.

```typescript
import { initPageState } from "@decodelabs/underlay/client";

onMount(() => {
  const restored = initPageState({
    activeTab: "details",
    currentPage: 1,
    filters: {}
  });
  
  // Apply restored values
  activeTab = restored.activeTab;
  currentPage = restored.currentPage;
  filters = restored.filters;
});
```

**Parameters:**
- `defaults` - Object with default values for each state property

**Returns:** `T` - Merged object with restored values overriding defaults

**Key behavior:**
- Only restores properties that exist in `defaults` (type-safe)
- Returns defaults if no stored state exists
- Consumes (removes) state after retrieval

##### `capturePageState<T>(stateValues)`

Type-safe helper to create state objects. Simply returns the input (passthrough function for type safety).

```typescript
import { capturePageState } from "@decodelabs/underlay/client";

void gotoWithContext(`/items/${id}/edit`, {
  label: "Items",
  href: "/items",
  type: "list",
  state: capturePageState({ activeTab, currentPage, filters })
});
```

##### Updated `gotoWithContext()`

The `gotoWithContext()` function now accepts an optional `state` property in the context:

```typescript
await gotoWithContext(targetHref, {
  label: "Module",
  href: `/learning/modules/${moduleId}`,
  type: "detail",
  state: { activeTab: "syllabus" }  // Optional state to restore on return
});
```

When `state` is provided, it's stored in sessionStorage keyed by the context's `href` pathname.

### Complete Example

Here's a complete example of a Module detail page with tabbed content that preserves tab selection:

```svelte
<!-- /learning/modules/[moduleId]/+page.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { PageHeader } from "@decodelabs/underlay/runtime";
  import LocalActionsMenu from "$lib/components/LocalActionsMenu.svelte";
  import { Tabs, type TabItem } from "@poodle/svelte-primitives";
  import { gotoWithContext, initPageState } from "@decodelabs/underlay/client";

  let { data } = $props();
  
  let activeTab = $state("details");
  const tabItems: TabItem[] = [
    { value: "details", label: "Details" },
    { value: "syllabus", label: "Syllabus" }
  ];

  onMount(() => {
    // Restore tab selection if returning via back navigation
    const restored = initPageState({ activeTab: "details" });
    activeTab = restored.activeTab;
  });
</script>

<PageHeader title={data.module.code} subtitle={data.module.title}>
  {#snippet actions()}
    <LocalActionsMenu
      actions={[
        {
          label: "Edit module",
          onSelect: () =>
            void gotoWithContext(`/learning/modules/${data.module.moduleId}/edit`, {
              label: `${data.module.code}: ${data.module.title}`,
              href: `/learning/modules/${data.module.moduleId}`,
              type: "detail",
              state: { activeTab }  // Save current tab
            })
        }
      ]}
    />
  {/snippet}
</PageHeader>

<Tabs bind:value={activeTab} items={tabItems} variant="card" size="sm" ariaLabel="Module sections" let:activeValue>
  {#if activeValue === "details"}
    <!-- Details content -->
  {/if}

  {#if activeValue === "syllabus"}
    <!-- Syllabus content with nested edit links that also save activeTab -->
    {#each data.syllabus.sections as section}
      <LocalActionsMenu
        actions={[
          {
            label: "Edit section",
            onSelect: () =>
              void gotoWithContext(`/learning/modules/${data.module.moduleId}/sections/${section.sectionId}/edit`, {
                label: `Section ${section.label}`,
                href: `/learning/modules/${data.module.moduleId}`,
                type: "detail",
                state: { activeTab }  // User will return to Syllabus tab
              })
          }
        ]}
      />
    {/each}
  {/if}
</Tabs>
```

### What State to Save

Good candidates for state restoration:

| State Type | Example | Restore? |
|------------|---------|----------|
| Active tab | `activeTab: "syllabus"` | ✅ Yes |
| Pagination page | `currentPage: 3` | ✅ Yes |
| Filter selections | `filters: { status: "active" }` | ✅ Yes |
| Sort order | `sortBy: "date"` | ✅ Yes |
| Expanded/collapsed sections | `expanded: ["a", "b"]` | ✅ Yes |
| Search query | `search: "example"` | ⚠️ Maybe |
| Scroll position | `scrollY: 450` | ⚠️ Maybe |
| Form input values | `draft: { title: "..." }` | ❌ No (use form state) |
| Sensitive data | passwords, tokens | ❌ Never |

### Design Decisions

1. **State stored separately from context stack**
   - The context stack is push/pop based for navigation history
   - State needs to persist until consumed, independent of stack operations

2. **State keyed by pathname**
   - Each page has its own state shape
   - Query parameters are stripped (pathname only)
   - Prevents state collision between different pages

3. **State is consumed on retrieval**
   - Prevents stale state from persisting indefinitely
   - Each navigation cycle gets fresh state

4. **Defaults always required**
   - Ensures components work without stored state
   - Type-safe merging - only known keys are restored

5. **sessionStorage (not localStorage)**
   - State cleared when browser/tab closes
   - Different tabs have independent state
   - Appropriate lifetime for navigation state

### Troubleshooting

#### State not being restored

**Possible causes:**
- `initPageState()` not called in `onMount`
- State was already consumed (only works once per navigation)
- Pathname mismatch (query params are stripped)
- User navigated via bookmark/direct link (no state was saved)

**Debug:**
```typescript
import { retrievePageState } from "@decodelabs/underlay/runtime";

// Check if state exists (without consuming)
console.log("Stored state:", retrievePageState(window.location.pathname));
```

#### State being restored unexpectedly

**Possible causes:**
- State from a previous session still in sessionStorage
- State saved but not consumed on a previous visit

**Solution:**
```typescript
import { clearPageStates } from "@decodelabs/underlay/runtime";

// Clear all states (e.g., on logout)
clearPageStates();
```

#### Wrong state being restored

**Possible causes:**
- Pathname collision (unlikely with unique IDs in paths)
- State shape changed between saves (add defaults for new fields)

**Solution:**
```typescript
// Always provide complete defaults
const restored = initPageState({
  activeTab: "details",
  currentPage: 1,
  newField: "default"  // New fields get defaults if not in stored state
});
```

---

## Complete Example

### List Page (Videos)

```svelte
<!-- /content/videos/+page.svelte -->
<script lang="ts">
  import type { PageData } from "./$types";
  import { PageHeader } from "@decodelabs/underlay/runtime";
  import { Grid } from "@poodle/svelte-primitives";
  import { VideoListCard } from "$lib/cards";

  export let data: PageData;
</script>

<PageHeader title="Videos" backHref="/content" backLabel="Back to content" />

<Grid columns="repeat(auto-fit, minmax(min(22.5rem, 100%), 1fr))" gap="lg">
  {#each data.videos as video}
    <VideoListCard {video} />
  {/each}
</Grid>
```

### List Card Component

```svelte
<!-- $lib/cards/VideoListCard.svelte -->
<script lang="ts">
  import LocalActionsMenu from "$lib/components/LocalActionsMenu.svelte";
  import { ListCard } from "@poodle/svelte-primitives";
  import { gotoWithContext } from "@decodelabs/underlay/client";
  import Video from "lucide-svelte/icons/video";

  interface Props {
    video: {
      videoId: string;
      title: string;
      duration: number;
    };
  }

  let { video }: Props = $props();

  const videoHref = $derived(`/content/videos/${video.videoId}`);
</script>

<ListCard href={videoHref} title={video.title}>
  <svelte:fragment slot="leading">
    <Video size={24} />
  </svelte:fragment>

  <svelte:fragment slot="actions">
    <LocalActionsMenu
      triggerLabel="Actions"
      actions={[
        {
          label: "Edit video",
          onSelect: () =>
            void gotoWithContext(`${videoHref}/edit`, {
              label: "Videos",
              href: "/content/videos",
              type: "list"
            })
        }
      ]}
    />
  </svelte:fragment>

  <svelte:fragment slot="footer">{video.duration}s</svelte:fragment>
</ListCard>
```

### Detail Page

```svelte
<!-- /content/videos/[videoId]/+page.svelte -->
<script lang="ts">
  import type { PageData } from "./$types";
  import { PageHeader } from "@decodelabs/underlay/runtime";
  import LocalActionsMenu from "$lib/components/LocalActionsMenu.svelte";
  import { gotoWithContext } from "@decodelabs/underlay/client";

  export let data: PageData;
</script>

<PageHeader
  title="Video"
  subtitle={data.video.title}
  backHref="/content/videos"
  backLabel="Back to videos"
>
  {#snippet actions()}
    <LocalActionsMenu
      actions={[
        {
          label: "Edit video",
          onSelect: () =>
            void gotoWithContext(`/content/videos/${data.video.videoId}/edit`, {
              label: data.video.title,
              href: `/content/videos/${data.video.videoId}`,
              type: "detail"
            })
        }
      ]}
    />
  {/snippet}
</PageHeader>
```

### Edit Page

```svelte
<!-- /content/videos/[videoId]/edit/+page.svelte -->
<script lang="ts">
  import type { PageData, ActionData } from "./$types";
  import { getBackButtonInfo, getReturnUrl } from "@decodelabs/underlay/runtime";
  import CrudFormShell from "$lib/forms/CrudFormShell.svelte";
  import VideoForm from "$lib/forms/VideoForm.svelte";

  export let data: PageData;
  export let form: ActionData | null = null;

  const defaultBackHref = `/content/videos/${data.video.videoId}`;
  const backInfo = getBackButtonInfo("Back to video", defaultBackHref);
  const returnTo = getReturnUrl(defaultBackHref);
</script>

<CrudFormShell
  title="Edit Video"
  subtitle={data.video.title}
  backHref={backInfo.href}
  backLabel={backInfo.label}
  success={form?.success === true}
  error={form?.success === false ? form?.error : null}
>
  <VideoForm
    mode="edit"
    values={data.video}
    errors={form?.fieldErrors ?? null}
    {returnTo}
  />
</CrudFormShell>
```

### Form Component

```svelte
<!-- $lib/forms/VideoForm.svelte -->
<script lang="ts">
  import { Field, TextInput, FormActions } from "@poodle/svelte-primitives";
  import { Button, SplitButton } from "@poodle/svelte-primitives";
  import { navigateOnCancel } from "@decodelabs/underlay/client";

  interface Props {
    mode?: "create" | "edit";
    values?: { title?: string; url?: string };
    errors?: Record<string, string> | null;
    cancelHref?: string;
    returnTo?: string;
  }

  let {
    mode = "edit",
    values = {},
    errors = null,
    cancelHref = undefined,
    returnTo = undefined
  }: Props = $props();

  function handleCancel() {
    navigateOnCancel(cancelHref);
  }

  let intent = $state<"save" | "save-close">("save-close");
  let actionBarElement = $state<HTMLDivElement | null>(null);
  const editIntentItems = [
    { value: "save", label: "Save changes" },
    { value: "save-close", label: "Save & close" }
  ];

  function submitWithIntent(nextIntent: "save" | "save-close") {
    intent = nextIntent;
    actionBarElement?.closest("form")?.requestSubmit();
  }
</script>

<Field label="Title" error={errors?.title}>
  <TextInput name="title" value={values.title ?? ""} required />
</Field>

<Field label="URL" error={errors?.url}>
  <TextInput name="url" value={values.url ?? ""} required />
</Field>

<FormActions>
  <div bind:this={actionBarElement}>
    <Button type="button" variant="ghost" on:click={handleCancel}>Cancel</Button>

    <input type="hidden" name="intent" value={intent} />
    {#if returnTo}
      <input type="hidden" name="returnTo" value={returnTo} />
    {/if}

    {#if mode === "create"}
      <Button type="submit" variant="primary">Create video</Button>
    {:else}
      <SplitButton
        type="submit"
        variant="primary"
        items={editIntentItems}
        on:click={() => submitWithIntent(intent)}
        on:action={(event) => submitWithIntent(event.detail.value as "save" | "save-close")}
      >
        {intent === "save" ? "Save changes" : "Save & close"}
      </SplitButton>
    {/if}
  </div>
</FormActions>
```

### Server Action

```typescript
// /content/videos/[videoId]/edit/+page.server.ts
import type { Actions, PageServerLoad } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { videoCommands } from "@myapp/client";

export const load: PageServerLoad = async ({ params, fetch }) => {
  const video = await videoCommands.getVideo(params.videoId, fetch);
  if (!video) throw error(404, "Video not found");
  return { video };
};

export const actions: Actions = {
  default: async ({ params, request, fetch }) => {
    const formData = await request.formData();

    const title = String(formData.get("title") ?? "").trim();
    const url = String(formData.get("url") ?? "").trim();
    const intent = String(formData.get("intent") ?? "save-close");
    const returnTo = String(formData.get("returnTo") ?? "").trim() || null;

    // Validate
    const fieldErrors: Record<string, string> = {};
    if (!title) fieldErrors.title = "Title is required";
    if (!url) fieldErrors.url = "URL is required";

    if (Object.keys(fieldErrors).length > 0) {
      return fail(400, {
        success: false,
        error: "Validation failed",
        fieldErrors,
        values: { title, url }
      });
    }

    // Save
    try {
      await videoCommands.updateVideo(params.videoId, { title, url }, fetch);
    } catch (e) {
      return fail(400, {
        success: false,
        error: e instanceof Error ? e.message : "Failed to update video",
        fieldErrors: null,
        values: { title, url }
      });
    }

    // Redirect on save-close
    if (intent === "save-close") {
      // Use returnTo if safe (relative path only), otherwise fall back
      const redirectTarget = returnTo && returnTo.startsWith("/")
        ? returnTo
        : `/content/videos/${params.videoId}`;
      throw redirect(303, redirectTarget);
    }

    return { success: true, values: { title, url } };
  }
};
```

---

## Create Page Pattern

Create pages follow a similar pattern to edit pages, but with a key difference: the default back/redirect destination is typically the list page rather than a detail page (since the entity doesn't exist yet).

### Create Page Component

```svelte
<!-- /content/videos/new/+page.svelte -->
<script lang="ts">
  import type { ActionData } from "./$types";
  import { getBackButtonInfo, getReturnUrl } from "@decodelabs/underlay/runtime";
  import CrudFormShell from "$lib/forms/CrudFormShell.svelte";
  import VideoForm from "$lib/forms/VideoForm.svelte";

  export let form: ActionData | null = null;

  // For create pages, the default destination is the list page
  const defaultBackHref = "/content/videos";
  const backInfo = getBackButtonInfo("Back to videos", defaultBackHref);
  const returnTo = getReturnUrl(defaultBackHref);
</script>

<CrudFormShell
  title="New Video"
  backHref={backInfo.href}
  backLabel={backInfo.label}
  success={form?.success === true}
  error={form?.success === false ? form?.error : null}
>
  <VideoForm
    mode="create"
    values={form?.values ?? {}}
    errors={form?.fieldErrors ?? null}
    cancelHref={backInfo.href}
    {returnTo}
  />
</CrudFormShell>
```

### Create Page Server Action

```typescript
// /content/videos/new/+page.server.ts
import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { videoCommands } from "@myapp/client";

export const actions: Actions = {
  default: async ({ request, fetch }) => {
    const formData = await request.formData();

    const title = String(formData.get("title") ?? "").trim();
    const url = String(formData.get("url") ?? "").trim();
    const intent = String(formData.get("intent") ?? "save-close");
    const returnTo = String(formData.get("returnTo") ?? "").trim() || null;

    // Validate
    const fieldErrors: Record<string, string> = {};
    if (!title) fieldErrors.title = "Title is required";
    if (!url) fieldErrors.url = "URL is required";

    if (Object.keys(fieldErrors).length > 0) {
      return fail(400, {
        success: false,
        error: "Validation failed",
        fieldErrors,
        values: { title, url, intent }
      });
    }

    // Create
    let created: { videoId: string };
    try {
      created = await videoCommands.createVideo({ title, url }, fetch);
    } catch (e) {
      return fail(400, {
        success: false,
        error: e instanceof Error ? e.message : "Failed to create video",
        fieldErrors: null,
        values: { title, url, intent }
      });
    }

    // Redirect based on intent
    if (intent === "save-close") {
      // Use returnTo if safe, otherwise go to list
      const redirectTarget =
        returnTo && returnTo.startsWith("/")
          ? returnTo
          : "/content/videos";
      throw redirect(303, redirectTarget);
    }

    // For plain "save", redirect to the edit page of the newly created item
    throw redirect(303, `/content/videos/${created.videoId}`);
  }
};
```

### Key Differences from Edit Pages

| Aspect | Edit Page | Create Page |
|--------|-----------|-------------|
| Default back destination | Detail page (`/items/123`) | List page (`/items`) |
| Save & close redirect | Context or detail page | Context or list page |
| Plain save redirect | Stay on edit page | Go to new item's edit page |
| `cancelHref` | `backInfo.href` | `backInfo.href` |

---

## Inline Form Pattern

For pages where the form is defined inline (not in a separate component), include the hidden fields directly:

```svelte
<!-- /assessment/questions/new/+page.svelte -->
<script lang="ts">
  import type { ActionData } from "./$types";
  import { getBackButtonInfo, getReturnUrl } from "@decodelabs/underlay/runtime";
  import { navigateOnCancel } from "@decodelabs/underlay/client";
  import CrudFormShell from "$lib/forms/CrudFormShell.svelte";
  import { Field, TextInput, FormActions } from "@poodle/svelte-primitives";
  import { Button, SplitButton } from "@poodle/svelte-primitives";

  export let form: ActionData | null = null;

  let intent: "save" | "save-close" = "save-close";
  let actionBarElement = $state<HTMLDivElement | null>(null);
  const createIntentItems = [
    { value: "save", label: "Create & continue" },
    { value: "save-close", label: "Create & close" }
  ];

  const defaultBackHref = "/assessment/questions";
  const backInfo = getBackButtonInfo("Back to questions", defaultBackHref);
  const returnTo = getReturnUrl(defaultBackHref);

  function handleCancel() {
    navigateOnCancel(backInfo.href);
  }

  function submitWithIntent(nextIntent: "save" | "save-close") {
    intent = nextIntent;
    actionBarElement?.closest("form")?.requestSubmit();
  }
</script>

<CrudFormShell
  title="Create Question"
  backHref={backInfo.href}
  backLabel={backInfo.label}
  method="post"
  success={form?.success === true}
  error={form?.success === false ? form?.error : null}
>
  <Field label="Title" error={form?.fieldErrors?.title}>
    <TextInput name="title" value={form?.values?.title ?? ""} required />
  </Field>

  <!-- More fields... -->

  <FormActions>
    <div bind:this={actionBarElement}>
      <Button type="button" variant="ghost" on:click={handleCancel}>Cancel</Button>

      <input type="hidden" name="intent" value={intent} />
      {#if returnTo}
        <input type="hidden" name="returnTo" value={returnTo} />
      {/if}

      <SplitButton
        type="submit"
        variant="primary"
        items={createIntentItems}
        on:click={() => submitWithIntent(intent)}
        on:action={(event) => submitWithIntent(event.detail.value as "save" | "save-close")}
      >
        {intent === "save" ? "Create & continue" : "Create & close"}
      </SplitButton>
    </div>
  </FormActions>
</CrudFormShell>
```

---

## Rollout Checklist

When adding navigation context support to an existing page, follow this checklist:

### For Pages with Separate Form Components

**1. Update the page component (`+page.svelte`):**
- [ ] Import `consumeNavigationContext` (and `computeBackInfo` if using async data) from `@decodelabs/underlay/runtime`
- [ ] Define `defaultBackHref` constant with the fallback destination
- [ ] Call `const { backInfo, returnTo } = consumeNavigationContext(label, defaultBackHref)`
- [ ] If fallback depends on async data (e.g., entity name), use `computeBackInfo()`:
  ```typescript
  const computedBackInfo = $derived(
    computeBackInfo(backInfo, module ? {
      href: `/modules/${module.id}`,
      label: `Back to ${module.code}`
    } : undefined)
  );
  ```
- [ ] Pass `computedBackInfo.href` and `computedBackInfo.label` to `CrudFormShell` or `PageHeader`
- [ ] Pass `returnTo` and `cancelHref={computedBackInfo.href}` to the form component

**2. Update the form component:**
- [ ] Add `returnTo?: string` prop
- [ ] Add `cancelHref?: string` prop (if not already present)
- [ ] Import `navigateOnCancel` from `@decodelabs/underlay/client`
- [ ] Update `handleCancel()` to use `navigateOnCancel(cancelHref)`
- [ ] Add hidden input: `{#if returnTo}<input type="hidden" name="returnTo" value={returnTo} />{/if}`

**3. Update the server action (`+page.server.ts`):**
- [ ] Extract `returnTo`: `const returnTo = String(formData.get("returnTo") ?? "").trim() || null;`
- [ ] Update redirect logic: `const target = returnTo && returnTo.startsWith("/") ? returnTo : defaultPath;`

### For Pages with Inline Forms

**1. Update the page component:**
- [ ] Import `consumeNavigationContext` from `@decodelabs/underlay/runtime`
- [ ] Import `navigateOnCancel` from `@decodelabs/underlay/client`
- [ ] Define `defaultBackHref` and call `const { backInfo, returnTo } = consumeNavigationContext(...)`
- [ ] Add hidden `returnTo` input directly in the form
- [ ] Update cancel handler to use `navigateOnCancel(backInfo.href)`

**2. Update the server action:**
- [ ] Same as above

### For Source Pages (List/Detail)

**Update list cards or action menus:**
- [ ] Import `gotoWithContext` from `@decodelabs/underlay/client`
- [ ] Replace `goto(editHref)` with `gotoWithContext(editHref, { label, href, type })`
- [ ] Use `type: "list"` for list pages, `type: "detail"` for detail pages

---

## Breadcrumb Sanity Rules

The navigation context stack applies these rules to prevent unbounded growth:

### 1. Maximum Depth (default: 3)

The stack is trimmed to keep only the most recent items:

```
Push: List A → Detail A → Detail B → Detail C → Detail D
Stack after: [Detail B, Detail C, Detail D]  // Only 3 items kept
```

### 2. Same-Type Collapse

Consecutive list pages collapse into one:

```
Push: List A (list) → List B (list)
Stack after: [List B]  // List A replaced by List B
```

This makes sense because navigating from one list to another list means the first list is no longer relevant as a "back" destination. Detail pages are kept so they can form a breadcrumb trail (up to the max depth).

### 3. Deduplication

If the same `href` already exists in the stack, it moves to the top:

```
Stack: [List A, Detail A, Detail B]
Push: Detail A
Stack after: [List A, Detail B, Detail A]  // Detail A moved to top
```

---

## Security Considerations

### Validate returnTo on Server

Always validate that `returnTo` is a safe relative path:

```typescript
// GOOD - validate returnTo is a relative path
const redirectTarget = returnTo && returnTo.startsWith("/")
  ? returnTo
  : `/default/path`;

// BAD - allows open redirect attacks
const redirectTarget = returnTo ?? `/default/path`;
```

**Why?** Without validation, an attacker could craft a link like:
```
/edit?returnTo=https://evil.com
```

By requiring `returnTo.startsWith("/")`, we ensure it's always a relative path within the same origin.

### Don't Trust Client State

The server action should always have a sensible default redirect. The `returnTo` field is a hint for better UX, not a requirement:

```typescript
// Always have a safe fallback
const redirectTarget = returnTo && returnTo.startsWith("/")
  ? returnTo
  : `/items/${params.itemId}`;  // Safe default
```

---

## Best Practices

1. **Always provide fallbacks** - Both `getBackButtonInfo()` and `getReturnUrl()` require fallback values
2. **Use `computeBackInfo()` for data-dependent fallbacks** - Don't manually check `isContextual`; let the helper handle it
3. **Use descriptive labels** - "FA 2025" is better than "Pathway" for context labels
4. **Set type correctly** - Use `"list"` for collection pages, `"detail"` for item pages
5. **Validate returnTo server-side** - Only accept relative paths
6. **Don't over-navigate** - Only use `gotoWithContext()` when going to forms/edit pages
7. **Test refresh behavior** - Context survives refresh (sessionStorage), verify this works
8. **Save meaningful state** - Tabs, pagination, filters are good; avoid transient UI state
9. **Always provide defaults for state** - Use `initPageState({ tab: "default" })` not `consumePageState()`
10. **Don't store sensitive data in state** - State is stored in sessionStorage (visible to client-side code)

---

## Troubleshooting

### Back button shows default label instead of context

**Possible causes:**
- Navigation didn't use `gotoWithContext()` - check the source page
- User navigated to the edit page via bookmark or direct link (no context was pushed)
- Context was for a different page (targetHref validation discarded it as stale)
- Different browser tab - sessionStorage is per-tab

**Debug:**
```svelte
<script>
  import { getNavigationContextStack } from "@decodelabs/underlay/runtime";
  console.log("Nav context:", getNavigationContextStack());
</script>
```

### Stale context appearing (wrong "Back to X" label)

**This should be fixed automatically.** When `gotoWithContext()` is used, the target URL is stored with the context. When `consumeNavigationContext()` is called, it validates that the context was intended for the current page.

**If you still see stale context:**
- Ensure you're using `gotoWithContext()` (not `pushNavigationContext()` directly) for navigation
- Ensure edit/create pages use `consumeNavigationContext()` (not `getBackButtonInfo()` + `getReturnUrl()`)
- Check that the stored targetHref matches the current pathname exactly

**Debug:**
```svelte
<script>
  import { getNavigationContextStack } from "@decodelabs/underlay/runtime";
  const stack = getNavigationContextStack();
  if (stack.length > 0) {
    console.log("Top context targetHref:", stack[stack.length - 1].targetHref);
    console.log("Current pathname:", window.location.pathname);
  }
</script>
```

### Form redirects to wrong place

**Possible causes:**
- `returnTo` hidden field not included in form
- Server action not reading `returnTo` from formData
- `returnTo` validation rejecting the value (doesn't start with `/`)

**Debug:**
```typescript
// In server action
console.log("returnTo value:", formData.get("returnTo"));
```

### Context stack grows too large

This shouldn't happen due to built-in sanity rules, but if it does:

```typescript
import { clearNavigationContext } from "@decodelabs/underlay/runtime";

// Reset on logout or major navigation events
clearNavigationContext();
```

---

## Next Steps

- [090-ui-kit](./090-ui-kit.md) - UI components including PageHeader
- [100-frontend-web](./100-frontend-web.md) - Frontend routing patterns
- [110-admin](./110-admin.md) - Admin interface patterns
