# 098 - Shared Admin Patterns

This page is now a bridge note.

The actual admin UI implementation guidance now lives in Poodle:
- `Page Shell And Admin Recipes` in the Poodle guide set
- `List And Filter Recipes` in the Poodle guide set
- `Dialog And Detail Recipes` in the Poodle guide set

Read this Underlay page as a retained-boundary explanation only.

The recipe layer that sits above this page now follows a split model:
- use [CRUD Admin Interface](../patterns/crud-admin-interface.md) for full-stack delivery order
- use [Nested Entity Management](../patterns/nested-entity-management.md) when parent/child route structure matters
- use [Autonomous Admin List](../patterns/autonomous-admin-list.md), [Reorderable Collections](../patterns/reorderable-collections.md), and [Trash Lifecycle](../patterns/trash-lifecycle.md) when list/runtime workflows are the real problem
- use [097-autonomous-list-components.md](./097-autonomous-list-components.md) when the missing piece is lower list workflow mechanics such as selection-mode state, transform-launch state, or local-vs-loaded reorder sessions
- use the Poodle guides for the visible page/list/detail/dialog composition inside those recipes

Storybook coverage:
- Poodle `PageHeader`
- Underlay `SpaFormShell`
- Poodle `Menu`

AI routing ops pages now compose directly over `createAiRoutingOpsController`
plus Poodle `PageHeader`, `Card`, `Callout`, `PageLoading`, and `DataTable`.
Underlay no longer exports a public `AiRoutingAdmin` shell.

## Overview

| Pattern | Location | Purpose |
|---------|----------|---------|
| EmptyState | `@poodle/svelte` | Rich empty state with message, optional actions, and optional custom visual slot |
| Local actions menu + AlertDialog | app-local wrapper or direct Poodle `Menu` + Poodle `AlertDialog` | Copy-to-clipboard actions with caller-owned destructive confirmation |
| Poodle `PageHeader` + `Tabs` + `MetaBar` | direct composition | Standard entity detail page framing without a shared shell wrapper |
| `SpaFormShell` | `@decodelabs/underlay/patterns` | Retained SPA create/edit workflow shell with submit/result/navigation orchestration |
| EditableLabel | `@poodle/svelte` | Click-to-edit text field |
| KeyboardShortcuts | `patterns/keyboard-shortcuts.svelte.ts` | Centralized shortcut registration |

## Ownership Boundary

- Poodle owns visible admin page composition
- Underlay owns retained workflow shells and runtime helpers
- app code owns entity wording, menus, permission rules, and route orchestration

Do not treat this page as a substitute for the Poodle guides. It exists to
explain what still belongs in Underlay after the UI translation wave.

---

## EmptyState

Use Poodle `EmptyState` directly for page-level and inline empty views.

```svelte
<script lang="ts">
  import { EmptyState } from "@poodle/svelte";
  import InboxIcon from "lucide-svelte/icons/inbox";
</script>

<EmptyState
  title="No outcomes found"
  message="Try adjusting your filters or add a new outcome."
  size="compact"
>
  {#snippet visual()}
    <InboxIcon size={16} />
  {/snippet}
  {#snippet actions()}
    <a href="/outcomes/new">Add outcome</a>
  {/snippet}
</EmptyState>
```

---

## Entity Action Menus

`EntityActionsMenu` and `CopyActionsMenu` are retired. The stable boundary is now app-local action-menu composition over Poodle `Menu`, caller-owned `AlertDialog`, and local clipboard/toast wiring.

```svelte
<script lang="ts">
  import { copyToClipboard } from "@decodelabs/underlay/runtime/feedback";
  import { useToasts } from "@decodelabs/underlay/runtime/feedback";
  import { gotoWithContext } from "@decodelabs/underlay/client/navigation";
  import { AlertDialog, Menu } from "@poodle/svelte";

  const toastStore = useToasts();
  let showDeleteConfirm = $state(false);

  function handleEdit() {
    gotoWithContext(`/items/${item.id}/edit`, sourceContext);
  }

  async function handleDelete() {
    await api.softDelete(item.id);
  }
</script>

<Menu
  items={[
    { value: "edit", label: "Edit" },
    { value: "delete", label: "Soft delete", tone: "danger" },
    { value: "separator-copy", label: "", kind: "separator" },
    { value: "copy-slug", label: "Copy slug" },
    { value: "copy-id", label: "Copy ID" }
  ]}
  triggerAriaLabel="Actions"
  on:action={(event) => {
    if (event.detail.value === "edit") handleEdit();
    if (event.detail.value === "delete") showDeleteConfirm = true;
    if (event.detail.value === "copy-slug") {
      void copyToClipboard(toastStore, item.slug, "Copied slug");
    }
    if (event.detail.value === "copy-id") {
      void copyToClipboard(toastStore, item.id, "Copied ID");
    }
  }}
>
  <svelte:fragment slot="trigger">
    <button type="button">Actions</button>
  </svelte:fragment>
</Menu>

<AlertDialog
  open={showDeleteConfirm}
  title="Soft delete item?"
  description="This will hide the item. You can restore it later."
  confirmLabel="Soft delete"
  tone="danger"
  onConfirm={handleDelete}
  onCancel={() => {
    showDeleteConfirm = false;
  }}
>
  <p><strong>{item.title}</strong></p>
</AlertDialog>
```

---

## Drawer

Use Poodle `Drawer` directly for slide-out side panels. Underlay no longer
ships a second drawer surface.
- Focus is trapped and restored on close
- CSS transition: `transform 0.25s ease` (respects `prefers-reduced-motion`)
- Desktop (>900px): absolute positioned within parent container
- Mobile (<=900px): fixed overlay with backdrop
- `overlay={true}` forces overlay mode on all screen sizes

---

## Detail Headers

`DetailPageShell` is retired from the public Underlay surface.

The stable boundary is now direct composition:
- Poodle `PageHeader` for title, back link, actions, banner, and breadcrumbs
- Poodle `Tabs` for top-level detail sections
- Poodle `MetaBar`, `MetaItem`, `Code`, and `Pill` for the compact metadata
  row when useful

```svelte
<script lang="ts">
  import { PageHeader } from "@poodle/svelte";
  import { Breadcrumbs, Code, MetaBar, MetaItem, Pill, Tabs } from "@poodle/svelte";
</script>

<section class="detail-page">
  <div class="detail-page__header">
    <PageHeader title={module.code} subtitle={module.slug} backHref={backHref}>
      {#snippet breadcrumbs()}
        <Breadcrumbs items={breadcrumbs} />
      {/snippet}
      {#snippet actions()}
        <ModuleActionsMenu {module} />
      {/snippet}
    </PageHeader>

    <MetaBar ariaLabel="Module metadata">
      <MetaItem label="ID">
        <Code inline source={module.moduleId} showCopyButton />
      </MetaItem>
      <Pill tone={module.isLive ? "success" : "danger"} appearance="badge" size="lg">
        {module.isLive ? "Live" : "Draft"}
      </Pill>
    </MetaBar>
  </div>

  <Tabs
    value={activeTab}
    items={[
      { value: "details", label: "Details" },
      { value: "sections", label: "Sections", count: sections.length }
    ]}
    variant="card"
    size="sm"
    ariaLabel="Detail sections"
  >
    <!-- caller-owned tab content -->
  </Tabs>
</section>
```

---

## AutonomousList

`AutonomousList` is retired from the public Underlay surface.

The shell no longer has live consumer-app callers, and its generic building
blocks now live in Poodle:
- `PageHeader`
- `FilterToolbar`
- `PageLoading`
- `EmptyState`
- `ListCard`
- `Pagination`
- `BulkActionBar`
- `EditableList`
- `AlertDialog`

For new list pages, compose directly over those Poodle surfaces plus
Underlay’s lower-level state helpers such as `createListController`,
`createPaginationController`, `useBatchSelection`, `useBatchActions`, and
`createReorderController` now sit most naturally under
`@decodelabs/underlay/runtime/data`.

For hybrid wrappers that still need extra workflow state above those basics,
prefer the retained lower helper set from `@decodelabs/underlay/runtime/data`:

- `createSelectionModeController(...)`
- `buildSelectionTransformState(...)`
- `createLocalReorderSession(...)`
- `createLoadedReorderSession(...)`

Use the local reorder session when the visible constrained list is already the
full reorder set. Use the loaded reorder session when browse mode is paged or
cursor-backed and reorder needs a separate full-set fetch.

Reference:
- [097-autonomous-list-components.md](./097-autonomous-list-components.md)
- [List workflow helpers recipe](./code/097-autonomous-list-components/list-workflow-helpers.ts)

---

## SPA Create/Edit Pages

`SpaFormShell` remains a retained Underlay structural shell.

Keep it when the page needs:
- SPA submit interception
- shared loading/result lifecycle
- success/error/field-error framing
- redirect and navigation handoff
- optional `prepare(formData)` transformation before submit

Do not treat it like stale wrapper residue around Poodle visuals. The visual
pieces inside it should stay Poodle-owned, but the shell still earns shared
Underlay ownership because the remaining value is workflow orchestration across
the broad create/edit route family.

---

## EditableLabel

Click-to-edit field that displays as text and becomes an input on activation.

```svelte
<script lang="ts">
  import { EditableLabel } from "@poodle/svelte";
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

## Import Paths

Use the retained Underlay `patterns` barrel for workflow/shell surfaces and
Poodle directly for low-level primitives:

```typescript
// Underlay patterns (composed, stateful)
import {
  ForgotPasswordFlow,
  LoginPage,
  PasswordRequirements,
  SpaFormShell
} from "@decodelabs/underlay/patterns";

import { createKeyboardShortcuts } from "@decodelabs/underlay/runtime/browser";
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
