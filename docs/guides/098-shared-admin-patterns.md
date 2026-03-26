# 098 - Shared Admin Patterns

Higher-level, pre-composed patterns for common admin interface needs. These build on the core UI kit (090) and autonomous list hooks (097) to eliminate boilerplate across consuming apps.

All components are additive — importing them is optional and they introduce no breaking changes.

## Overview

| Pattern | Location | Purpose |
|---------|----------|---------|
| EmptyState | `components/EmptyState.svelte` | Rich empty state with icon, message, and CTA |
| EntityActionsMenu | `patterns/EntityActionsMenu.svelte` | Dropdown with edit, custom actions, and soft-delete flow |
| Drawer | `components/Drawer.svelte` | Slide-out side panel |
| DetailPageShell | `patterns/DetailPageShell/` | Composable entity detail page with tabs |
| AutonomousList | `patterns/AutonomousList/` | Self-contained list with filters, batch actions, reorder |
| EditableLabel | `@poodle/svelte-primitives` | Click-to-edit text field |
| KeyboardShortcuts | `patterns/keyboard-shortcuts.svelte.ts` | Centralized shortcut registration |
| ErrorBoundary | `components/ErrorBoundary.svelte` | Render error catch with recovery UI |

---

## EmptyState

Rich empty state component replacing plain `<p>` text in lists, tables, and filtered views.

```svelte
<script lang="ts">
  import { EmptyState } from "@decodelabs/underlay/components";
  import InboxIcon from "lucide-svelte/icons/inbox";
</script>

<EmptyState
  icon={InboxIcon}
  title="No outcomes found"
  description="Try adjusting your filters or add a new outcome."
  actionLabel="Add outcome"
  actionHref="/outcomes/new"
/>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `icon` | `Component<{ size?: number }>` | - | Lucide or custom icon |
| `title` | `string` | **required** | Primary message |
| `description` | `string` | - | Secondary text |
| `actionLabel` | `string` | - | CTA button text |
| `actionHref` | `string` | - | CTA link URL |
| `onaction` | `() => void` | - | CTA callback (alternative to href) |
| `variant` | `"default" \| "compact"` | `"default"` | Size variant |
| `children` | `Snippet` | - | Override entire content |
| `class` | `string` | `""` | Additional CSS class |

### Variants

- **default** — Full-size with large icon (40px), generous padding. Use for page-level empty states.
- **compact** — Smaller icon (24px), tighter padding. Use for inline empty states within cards or tabs.

### Overriding Content

Pass `children` to replace the entire default layout:

```svelte
<EmptyState title="unused">
  <div class="my-custom-empty">
    <p>Custom empty content here</p>
  </div>
</EmptyState>
```

---

## EntityActionsMenu

Dropdown menu combining copy-to-clipboard, edit, custom actions, and soft-delete with confirmation dialog. Eliminates the per-entity actions menu boilerplate found in most detail/list pages.

```svelte
<script lang="ts">
  import { EntityActionsMenu, useToasts } from "@decodelabs/underlay/patterns";
  import { gotoWithContext } from "@decodelabs/underlay/client";

  const toastStore = useToasts();

  function handleEdit() {
    gotoWithContext(`/items/${item.id}/edit`, sourceContext);
  }
</script>

<EntityActionsMenu
  toastStore={toastStore}
  copies={[
    { label: "Copy slug", text: item.slug, successMessage: "Copied slug" },
    { label: "Copy ID", text: item.id, successMessage: "Copied ID" }
  ]}
  onEdit={handleEdit}
  deleteConfig={{
    entityLabel: item.title,
    title: "Soft delete item?",
    description: "This will hide the item. You can restore it later.",
    confirmLabel: "Soft delete",
    execute: async () => { await api.softDelete(item.id); }
  }}
  onDeleteSuccess={() => goto("/items")}
/>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `toastStore` | `ToastStore` | context | Toast store (falls back to `useToasts()`) |
| `copies` | `CopyItem[]` | `[]` | Copy-to-clipboard items |
| `editLabel` | `string` | `"Edit"` | Edit action label |
| `onEdit` | `() => void` | - | Edit callback (omit to hide edit action) |
| `deleteConfig` | `DeleteConfig` | - | Soft-delete config (omit to hide delete action) |
| `onDeleteSuccess` | `() => void` | - | Callback after successful delete |
| `customActions` | `DropdownMenuItem[]` | `[]` | Extra actions between edit and delete |
| `trigger` | `Snippet` | - | Custom trigger button |
| `align` | `"start" \| "center" \| "end"` | `"end"` | Dropdown alignment |
| `side` | `"top" \| "right" \| "bottom" \| "left"` | `"bottom"` | Dropdown side |
| `class` | `string` | - | Additional CSS class |

### DeleteConfig

```typescript
interface DeleteConfig {
  entityLabel: string;   // Shown in confirmation dialog
  title: string;         // Dialog title
  description: string;   // Dialog description
  confirmLabel?: string; // Confirm button label (default: "Delete")
  execute: () => Promise<void>; // Async delete function
}
```

### Design Note

`EntityActionsMenu` uses `onEdit` (a callback) rather than `editHref` to avoid coupling the library to SvelteKit's `$app/navigation`. Consumers wire up their own navigation:

```svelte
onEdit={() => gotoWithContext(`/items/${item.id}/edit`, sourceContext)}
```

---

## Drawer

Slide-out side panel from the right or left edge. Responsive — absolute positioned on desktop, fixed overlay on mobile.

```svelte
<script lang="ts">
  import { Drawer } from "@decodelabs/underlay/components";

  let panelOpen = $state(false);
</script>

<button onclick={() => panelOpen = true}>Open filters</button>

<Drawer bind:open={panelOpen} title="Filters" position="right" width="28rem">
  <FilterForm ... />
</Drawer>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `open` | `boolean` | `false` | Open state (bindable) |
| `title` | `string` | - | Panel header title |
| `position` | `"right" \| "left"` | `"right"` | Slide direction |
| `width` | `string` | `"28rem"` | Panel width (CSS value) |
| `overlay` | `boolean \| "auto"` | `"auto"` | Backdrop mode. `"auto"` = backdrop on mobile only |
| `onclose` | `() => void` | - | Close callback |
| `children` | `Snippet` | - | Panel content |
| `headerActions` | `Snippet` | - | Extra header content (before close button) |
| `class` | `string` | `""` | Additional CSS class |

### Behavior

- **Escape** key closes the panel
- Backdrop click closes the panel
- Focus is trapped and restored on close
- CSS transition: `transform 0.25s ease` (respects `prefers-reduced-motion`)
- Desktop (>900px): absolute positioned within parent container
- Mobile (<=900px): fixed overlay with backdrop
- `overlay={true}` forces overlay mode on all screen sizes

---

## DetailPageShell

Composable shell for entity detail pages. Standardizes the PageHeader + metadata + tabs + actions composition pattern. The shell stays in Underlay because it owns the detail-page assembly and lazy-mount behavior, but the tab chrome is now provided by Poodle `Tabs`.

```svelte
<script lang="ts">
  import {
    DetailPageShell,
    DetailMeta,
    DetailMetaId,
    DetailMetaStatus,
    DetailMetaSeparator
  } from "@decodelabs/underlay/patterns";
</script>

<DetailPageShell
  title={module.code}
  subtitle={module.slug}
  breadcrumbs={breadcrumbs}
  backHref={backHref}
  tabs={[
    { value: "details", label: "Details" },
    { value: "sections", label: "Sections", count: sections.length }
  ]}
  activeTab="details"
>
  {#snippet meta()}
    <DetailMeta>
      <DetailMetaId value={module.moduleId} />
      <DetailMetaSeparator />
      <DetailMetaStatus value={module.isLive} trueLabel="Live" falseLabel="Draft" />
    </DetailMeta>
  {/snippet}

  {#snippet actions()}
    <ModuleActionsMenu {module} />
  {/snippet}

  {#snippet tabContent(tab)}
    {#if tab === "details"}
      <DetailsCard sections={detailsSections} />
    {:else if tab === "sections"}
      <SectionsList moduleId={module.moduleId} />
    {/if}
  {/snippet}
</DetailPageShell>
```

### DetailPageShell Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | - | Primary heading text |
| `section` | `string` | - | Section name (renders as prominent h1) |
| `subtitle` | `string` | - | Subtitle text (e.g., slug) |
| `breadcrumbs` | `BreadcrumbItem[]` | - | Breadcrumb trail |
| `level` | `PageHeaderLevel` | - | Heading level (1-4) |
| `backHref` | `string \| null` | - | Back button URL |
| `backLabel` | `string` | - | Back button label |
| `backIsContextual` | `boolean` | - | Whether back came from navigation context |
| `bannerMessage` | `string` | - | Banner message below header |
| `bannerVariant` | `BannerVariant` | - | Banner variant (warning, error, info) |
| `titleSuffix` | `Snippet` | - | Inline content after title |
| `meta` | `Snippet` | - | Metadata row (use DetailMeta sub-components) |
| `actions` | `Snippet` | - | Actions menu snippet |
| `tabs` | `TabConfig[]` | - | Tab definitions (value, label, count) |
| `activeTab` | `string` | first tab | Current active tab (bindable) |
| `tabsHistoryKey` | `string` | `"tab"` | URL query key for tab sync |
| `tabContent` | `Snippet<[string]>` | - | Tab content renderer (receives tab value) |
| `children` | `Snippet` | - | Content when no tabs defined |
| `class` | `string` | - | Additional CSS class |

### Sub-Components

| Component | Purpose |
|-----------|---------|
| `DetailMeta` | Wraps metadata items in a detail-page metadata row |
| `DetailMetaItem` | Generic metadata value with optional label inside DetailMeta |
| `DetailMetaId` | Displays an ID with Code formatting and copy support |
| `DetailMetaStatus` | Boolean status badge (value, trueLabel, falseLabel, variant) |
| `DetailMetaSeparator` | Visual separator between metadata items |

### Without Tabs

For detail pages without tabs, use `children` instead of `tabs`/`tabContent`:

```svelte
<DetailPageShell title={item.title} {breadcrumbs}>
  {#snippet meta()}...{/snippet}
  {#snippet actions()}...{/snippet}

  <DetailsCard sections={sections} />
</DetailPageShell>
```

---

## AutonomousList

Self-contained list component that wires together `createListController`, `useBatchActions`, `createReorderController`, FilterBar, BatchActionBar, and loading/error/empty states. Targets the 18+ list views found in typical admin apps.

```svelte
<script lang="ts">
  import { AutonomousList } from "@decodelabs/underlay/patterns";
  import type { BatchAction } from "@decodelabs/underlay/patterns";
  import { learningCommands } from "@cattle-grid";

  async function fetchModules(fetchFn: typeof fetch, token: string, filters: Record<string, unknown>) {
    return await learningCommands.getModules(fetchFn, token, filters);
  }

  const batchActions: BatchAction<string>[] = [{
    id: "delete",
    label: "Delete",
    variant: "danger",
    confirm: {
      title: "Delete Modules",
      description: (count) => `Delete ${count} module(s)?`,
      confirmLabel: "Delete"
    },
    execute: async (ids) => {
      for (const id of ids) await api.softDelete(id);
      return { success: true, affected: ids.length };
    }
  }];
</script>

<AutonomousList
  title="Modules"
  fetcher={fetchModules}
  filters={[
    { key: "search", label: "Search", type: "text", placeholder: "Search modules..." },
    { key: "status", label: "Status", type: "select", options: [
      { value: "live", label: "Live" },
      { value: "draft", label: "Draft" }
    ]}
  ]}
  {batchActions}
  reorderable={{
    execute: async (orderedIds, fetchFn, token) => {
      await api.reorderModules(orderedIds, fetchFn, token);
    }
  }}
  addHref="/modules/new"
  emptyMessage="No modules found"
>
  {#snippet item(module, { selected, onSelectionChange, selectionMode })}
    <ModuleListCard {module} {selected} {onSelectionChange} {selectionMode} />
  {/snippet}

  {#snippet reorderItem(module)}
    <ListCard title={module.code} subtitle={module.title} variant="compact" />
  {/snippet}
</AutonomousList>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | **required** | Page/section title |
| `level` | `PageHeaderLevel` | - | Heading level |
| `breadcrumbs` | `BreadcrumbItem[]` | - | Breadcrumb trail |
| `fetcher` | `(fetch, token, filters) => Promise<T[]>` | **required** | Data fetcher function |
| `idField` | `string` | `"id"` | Field name containing the item ID |
| `filters` | `ListFilterField[]` | `[]` | Filter field definitions |
| `batchActions` | `BatchAction<string>[]` | `[]` | Batch action definitions |
| `reorderable` | `ListReorderConfig` | - | Reorder configuration (omit for non-reorderable) |
| `addHref` | `string` | - | URL for "Add" button |
| `addLabel` | `string` | `"Add"` | Label for "Add" button |
| `emptyMessage` | `string` | `"No items found"` | Message when list is empty |
| `emptyIcon` | `Component<{ size?: number }>` | - | Icon for empty state |
| `item` | `Snippet<[T, ListItemContext]>` | - | Item card renderer |
| `reorderItem` | `Snippet<[T]>` | - | Reorder mode item renderer |
| `class` | `string` | `""` | Additional CSS class |

### ListFilterField

```typescript
interface ListFilterField {
  key: string;          // Unique filter key (sent to fetcher)
  label: string;        // Display label
  type: "text" | "select";
  placeholder?: string; // For text inputs
  options?: Array<{ value: string; label: string }>; // For selects
  includeAll?: boolean; // Show "All" option (default: true)
  allLabel?: string;    // Custom "All" label
  defaultValue?: string;
  debounce?: number;    // For text inputs (default: 400ms)
}
```

### ListReorderConfig

```typescript
interface ListReorderConfig {
  execute: (orderedIds: string[], fetchFn: typeof fetch, token: string) => Promise<void>;
  condition?: (filters: Record<string, unknown>) => boolean; // When reordering is available
}
```

### ListItemContext

The `item` snippet receives the data item and a context object:

```typescript
interface ListItemContext {
  selected: boolean;
  onSelectionChange: (selected: boolean) => void;
  selectionMode: boolean;
}
```

### State Management

`AutonomousList` creates its state internally via `createAutonomousListState()`, which composes:

- `createListController` — data fetching, loading/error states, filters
- `useBatchActions` — selection and batch operations
- `createReorderController` — drag-and-drop reorder (created lazily on mode enter)

The `createAutonomousListState` function is also exported for advanced use cases where you need direct access to the composed state outside the component.

### What It Handles

- Auth-integrated data fetching (via `configureAuth()`)
- Filter bar with text inputs and native select dropdowns
- Filter pill display with clear individual / clear all
- Selection mode toggle + Escape key handler
- Batch action bar + confirmation dialogs
- Reorder mode with drag-and-drop
- Loading / Error / Empty / Content state machine
- ListGrid rendering with item snippets

### Comparison with Manual Approach

The AutonomousList replaces the manual pattern documented in guide 097 ("Building an Autonomous List Component"). Where the manual approach requires ~100 lines of boilerplate per list, AutonomousList reduces this to ~20 lines of configuration.

---

## EditableLabel

Click-to-edit field that displays as text and becomes an input on activation.

```svelte
<script lang="ts">
  import { EditableLabel } from "@poodle/svelte-primitives";
</script>

<EditableLabel
  value={item.title}
  activationMode="enterOrSpace"
  showEditIcon
  on:commit={async (event) => {
    await api.updateTitle(item.id, event.detail.value);
  }}
/>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string` | **required** | Current display value |
| `activationMode` | `"doubleClick" \| "enterOrSpace" \| "programmatic"` | `"doubleClick"` | How edit mode is entered |
| `showEditIcon` | `boolean` | `false` | Show pencil icon on hover/focus |
| `placeholder` | `string` | `""` | Placeholder text |
| `disabled` | `boolean` | `false` | Disabled state |
| `emptyText` | `string \| null` | `null` | Text shown when the current value is empty |
| `maxLength` | `number \| null` | `null` | Input max length |
| `variant` | `"default" \| "flush"` | `"default"` | Inline visual treatment |

### States

1. **Display** — Shows text with optional pencil icon on hover/focus.
2. **Edit** — Shows inline input.
3. **Commit** — Emits `commit` on Enter or blur.
4. **Cancel** — Emits `cancel` on Escape.

### Keyboard

| Key | Action |
|-----|--------|
| Double-click / Click / Enter / Space | Enter edit mode depending on `activationMode` |
| Enter | Commit |
| Escape | Cancel and revert |

Keep validation, save state, and optimistic update policy in the host. If the
save can fail, handle that in the parent after `commit` rather than expecting a
shared inline-edit wrapper to own the whole async workflow.

---

## Keyboard Shortcut Manager

Centralized keyboard shortcut registration with priority, conditional activation, and cross-platform `mod+` prefix.

```svelte
<script lang="ts">
  import { createKeyboardShortcuts } from "@decodelabs/underlay/patterns";

  const shortcuts = createKeyboardShortcuts();

  shortcuts.register("Escape", () => closePanel());
  shortcuts.register("mod+k", () => openSearch(), {
    description: "Open search",
    priority: 10
  });

  // Conditional shortcut — only active when panel is open
  shortcuts.register("mod+s", () => saveChanges(), {
    description: "Save changes",
    when: () => panelOpen,
    priority: 5
  });
</script>

<svelte:window onkeydown={shortcuts.handleKeydown} />
```

### API

```typescript
interface KeyboardShortcutManager {
  register(
    key: string,
    handler: () => void,
    options?: ShortcutOptions
  ): () => void; // returns unregister function

  unregister(key: string): void;
  readonly shortcuts: RegisteredShortcut[];
  handleKeydown: (event: KeyboardEvent) => void;
}

interface ShortcutOptions {
  description?: string;
  when?: () => boolean; // conditional activation
  priority?: number;    // higher wins on conflict (default: 0)
}

interface RegisteredShortcut {
  key: string;
  description: string;
  priority: number;
}
```

### Key Patterns

| Pattern | Meaning |
|---------|---------|
| `"Escape"` | Escape key |
| `"mod+k"` | Cmd+K on Mac, Ctrl+K on Windows/Linux |
| `"mod+shift+s"` | Cmd+Shift+S on Mac, Ctrl+Shift+S on Windows/Linux |
| `"ctrl+enter"` | Ctrl+Enter on all platforms |

### Priority System

When multiple shortcuts match the same key combination, the one with the highest `priority` wins. This allows inner components to override outer shortcuts:

```svelte
// App-level (lower priority)
shortcuts.register("Escape", () => closeSearch(), { priority: 0 });

// Dialog-level (higher priority, wins when dialog is open)
shortcuts.register("Escape", () => closeDialog(), {
  priority: 10,
  when: () => dialogOpen
});
```

### Auto-Cleanup

The `register` function returns an unregister function. Call it when the component unmounts:

```svelte
<script>
  import { onDestroy } from "svelte";

  const unregister = shortcuts.register("mod+s", save);
  onDestroy(unregister);
</script>
```

---

## ErrorBoundary

Catches render errors in child components and displays recovery UI. Uses Svelte 5's `<svelte:boundary>` element.

```svelte
<script lang="ts">
  import { ErrorBoundary, EmptyState } from "@decodelabs/underlay/components";
</script>

<ErrorBoundary>
  <AsyncComponent />
</ErrorBoundary>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `children` | `Snippet` | **required** | Normal content |
| `fallback` | `Snippet<[Error, () => void]>` | EmptyState | Custom error UI |
| `onError` | `(error: Error) => void` | - | Error reporting callback |

### Default Fallback

When no `fallback` snippet is provided, ErrorBoundary renders an EmptyState with the error message and a "Try again" button that resets the boundary.

### Custom Fallback

```svelte
<ErrorBoundary onError={(e) => reportError(e)}>
  {#snippet fallback(error, reset)}
    <div class="my-error-ui">
      <p>Error: {error.message}</p>
      <button onclick={reset}>Retry</button>
    </div>
  {/snippet}

  <RiskyComponent />
</ErrorBoundary>
```

### Error Reporting

Use `onError` to send errors to your logging service:

```svelte
<ErrorBoundary onError={(error) => {
  console.error("Component error:", error);
  errorTracker.capture(error);
}}>
  <Dashboard />
</ErrorBoundary>
```

---

## Import Paths

All components and patterns are available from their respective barrel exports:

```typescript
// Components (primitives)
import {
  EmptyState,
  ErrorBoundary,
  Drawer
} from "@decodelabs/underlay/components";

// Patterns (composed, stateful)
import {
  EntityActionsMenu,
  DetailPageShell,
  DetailMeta,
  DetailMetaId,
  DetailMetaStatus,
  DetailMetaSeparator,
  AutonomousList,
  createAutonomousListState,
  createKeyboardShortcuts,
  type AutonomousListProps,
  type ListFilterField,
  type ListReorderConfig,
  type ListItemContext,
  type AutonomousListState,
  type KeyboardShortcutManager,
  type ShortcutOptions,
  type RegisteredShortcut
} from "@decodelabs/underlay/patterns";
```

---

## CSS Custom Properties

All components use the standard Underlay design token CSS custom properties for theming:

| Token | Default | Usage |
|-------|---------|-------|
| `--underlay-color-surface` | `#1a1a2e` | Panel backgrounds (Drawer) |
| `--underlay-color-text` | `inherit` | Primary text |
| `--underlay-color-text-muted` | `#64748b` | Secondary text, icons |
| `--underlay-color-border` | `rgba(148, 163, 184, 0.12)` | Borders, separators |
| `--underlay-color-field-bg` | `rgba(148, 163, 184, 0.18)` | Input backgrounds, hover states |
| `--underlay-color-primary` | `#2563eb` | Focus rings, active states |
| `--underlay-color-success` | `#22c55e` | Success affordances |
| `--underlay-color-error` | `#ef4444` | Error messages |
| `--underlay-radius-sm` | `0.25rem` | Small border radius |
| `--underlay-radius-md` | `0.5rem` | Medium border radius |
| `--underlay-font-size-sm` | `0.875rem` | Small text |
| `--underlay-font-size-xs` | `0.75rem` | Extra-small text (errors) |
