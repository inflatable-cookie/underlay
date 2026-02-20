# 021 – Shared Admin Components

**Status**: Complete
**Created**: 2026-02-14

## Overview

Extract reusable admin interface components and patterns identified from auditing Acowtancy. Eight phases, ordered by dependency and incremental value. All additive — no breaking changes.

### Consuming Apps

| App | Location | Status |
|-----|----------|--------|
| Acowtancy | `../acowtancy` | Active, heaviest consumer |
| Compli-me | `../compli-me` | Active |
| Songsprout | `../songsprout` | Active |
| Loophole Composer | `../loophole/composer` | Out of date |
| Underlay Reference | `../underlay-reference` | Reference |

## Progress Checklist

- [x] Phase 21.1 — EmptyState component
- [x] Phase 21.2 — EntityActionsMenu pattern
- [x] Phase 21.3 — Drawer / Side Panel component
- [x] Phase 21.4 — DetailPageShell pattern
- [x] Phase 21.5 — Autonomous List Builder
- [x] Phase 21.6 — InlineEditableField component
- [x] Phase 21.7 — Keyboard Shortcut Manager
- [x] Phase 21.8 — ErrorBoundary component

---

## Phase 21.1 — EmptyState Component

**Location:** `ts/src/components/EmptyState.svelte`

Rich empty state with icon, title, description, and optional CTA. Replaces plain `<p>` text across DataTable, InlineListCard, and list components.

```svelte
<EmptyState
  icon={InboxIcon}
  title="No outcomes found"
  description="Try adjusting your filters or add a new outcome."
  actionLabel="Add outcome"
  actionHref="/outcomes/new"
/>
```

**Props:**
- `icon?: Component` — Lucide icon or custom
- `title: string` — Primary message
- `description?: string` — Secondary explanatory text
- `actionLabel?: string` — CTA button text
- `actionHref?: string` — CTA link
- `onaction?: () => void` — CTA callback (alternative to href)
- `variant?: "default" | "compact"` — Full-size vs inline
- `children?: Snippet` — Override entire content

**Files:**
- [ ] Create `ts/src/components/EmptyState.svelte`
- [ ] Export from `ts/src/components/index.ts`
- [ ] Update DataTable empty slot default to use EmptyState

---

## Phase 21.2 — EntityActionsMenu Pattern

**Location:** `ts/src/patterns/EntityActionsMenu.svelte`

Extends CopyActionsMenu with built-in soft-delete flow. Eliminates ~80% of per-entity actions menu boilerplate (8+ menus in Acowtancy).

```svelte
<EntityActionsMenu
  {toastStore}
  copies={[
    { label: "Copy slug", text: item.slug, successMessage: "Copied slug" },
    { label: "Copy ID", text: item.id, successMessage: "Copied ID" }
  ]}
  editHref="/items/{item.id}/edit"
  editSourceContext={sourceContext}
  deleteConfig={{
    entityLabel: item.title,
    title: "Soft delete item?",
    description: "This will hide the item. You can restore it later.",
    confirmLabel: "Soft delete",
    execute: async () => { await api.softDelete(item.id); }
  }}
  onDeleteSuccess={onSoftDeleteSuccess}
  redirectHref={backHref}
/>
```

**Handles internally:** CopyActionsMenu composition, edit via `gotoWithContext`, soft-delete state + AlertDialog + auth check + error toast, `onDeleteSuccess` callback or redirect.

**Consumers provide:** Copy items, edit href + source context, delete config, optional custom actions array.

**Files:**
- [ ] Create `ts/src/patterns/EntityActionsMenu.svelte`
- [ ] Export from `ts/src/patterns/index.ts`

---

## Phase 21.3 — Drawer / Side Panel Component

**Location:** `ts/src/components/Drawer.svelte`

Slide-out panel from right/left edge. Based on Acowtancy's ContextPanel, generalized.

```svelte
<Drawer bind:open={panelOpen} title="Filters" position="right" width="28rem">
  <FilterForm ... />
</Drawer>
```

**Props:** `open`, `title?`, `position?` (right|left), `width?`, `overlay?`, `onclose?`, `children`, `headerActions?`, `class?`

**Features:** Focus trap in overlay mode, Escape closes, backdrop click closes, CSS transition (transform 0.25s), responsive (absolute on desktop, fixed overlay on mobile ≤900px).

**Files:**
- [ ] Create `ts/src/components/Drawer.svelte`
- [ ] Export from `ts/src/components/index.ts`

---

## Phase 21.4 — DetailPageShell Pattern

**Location:** `ts/src/patterns/DetailPageShell/`

Composable shell for entity detail pages. Standardizes PageHeader + meta + tabs + actions composition (~10+ detail pages in Acowtancy).

```svelte
<DetailPageShell
  title={module.code}
  subtitle={module.slug}
  breadcrumbs={breadcrumbs}
  {backHref}
>
  {#snippet meta()}
    <DetailMeta>
      <DetailMetaId value={module.moduleId} />
      <DetailMetaSeparator />
      <DetailMetaStatus value={module.isLive} trueLabel="Live" falseLabel="Draft" />
    </DetailMeta>
  {/snippet}

  {#snippet actions()}
    <ModuleActionsMenu {module} ... />
  {/snippet}

  {#snippet tabs(Tab)}
    <Tab value="details" label="Details">
      <DetailsCard sections={detailsSections} />
    </Tab>
    <Tab value="sections" label="Sections" count={sections.length}>
      <SectionsList moduleId={module.moduleId} />
    </Tab>
  {/snippet}
</DetailPageShell>
```

**Sub-components:** DetailPageShell, DetailMeta, DetailMetaId, DetailMetaStatus, DetailMetaSeparator, DetailTab.

**Handles:** PageHeader with breadcrumbs/back navigation, banner support, tab management with URL sync, lazy content loading, responsive layout.

**Files:**
- [ ] Create `ts/src/patterns/DetailPageShell/` directory with sub-components
- [ ] Create `index.ts` barrel export
- [ ] Export from `ts/src/patterns/index.ts`

---

## Phase 21.5 — Autonomous List Builder

**Location:** `ts/src/patterns/AutonomousList/`

Composable pattern wiring together createListController, useBatchActions, createReorderController, FilterBar, BatchActionBar, Pagination, and loading/error/empty states. Targets 18+ lists in Acowtancy.

```svelte
<AutonomousList
  title="Modules"
  fetcher={fetchModules}
  pagination="server"
  filters={filterConfig}
  batchActions={batchActionConfig}
  reorderable={reorderConfig}
  addHref="/modules/new"
  emptyMessage="No modules found"
>
  {#snippet item(module, { selected, onSelectionChange, selectionMode })}
    <ModuleListCard {module} {selected} {onSelectionChange} {selectionMode} />
  {/snippet}

  {#snippet reorderItem(module)}
    <ListCard title={module.code} subtitle={module.title} compact />
  {/snippet}
</AutonomousList>
```

**Handles internally:** Auth-integrated data fetching, batch selection/actions with confirmation, reorder mode, selection mode + Escape handler, filter bar from config, header actions (select/reorder/add), loading→error→empty→content state machine, ListGrid + Pagination or ReorderableList.

**Consumers provide:** Fetcher function, item/reorder snippets, filter config, batch action config, domain card components.

**Files:**
- [ ] Create `ts/src/patterns/AutonomousList/` directory
- [ ] `AutonomousList.svelte`, `autonomous-list-types.ts`, `autonomous-list-context.svelte.ts`
- [ ] `AutonomousListHeader.svelte`, `AutonomousListContent.svelte`, `AutonomousListFilters.svelte`
- [ ] `index.ts` barrel export
- [ ] Export from `ts/src/patterns/index.ts`

---

## Phase 21.6 — InlineEditableField Component

**Location:** `ts/src/components/InlineEditableField.svelte`

Click-to-edit field: displays as text, becomes input on click/Enter.

```svelte
<InlineEditableField
  value={item.title}
  onSave={async (newValue) => { await api.updateTitle(item.id, newValue); }}
  validate={(v) => v.trim().length > 0 ? { valid: true } : { valid: false, message: "Required" }}
/>
```

**States:** Display (text + pencil on hover) → Edit (input + check/x) → Saving (spinner) → Error (message below).

**Keyboard:** Click/Enter → edit, Enter → save, Escape → cancel.

**Files:**
- [ ] Create `ts/src/components/InlineEditableField.svelte`
- [ ] Export from `ts/src/components/index.ts`

---

## Phase 21.7 — Keyboard Shortcut Manager

**Location:** `ts/src/patterns/keyboard-shortcuts.svelte.ts`

Centralized keyboard shortcut registration with priority, conditional activation, and `mod+` cross-platform prefix.

```svelte
const shortcuts = useKeyboardShortcuts();
shortcuts.register("Escape", () => closePanel());
shortcuts.register("mod+k", () => openSearch());
```

**Features:** `mod+` resolves to Cmd/Ctrl per platform, priority system, `when` guard, auto-cleanup, readable shortcut list.

**Files:**
- [ ] Create `ts/src/patterns/keyboard-shortcuts.svelte.ts`
- [ ] Export from `ts/src/patterns/index.ts`

---

## Phase 21.8 — ErrorBoundary Component

**Location:** `ts/src/components/ErrorBoundary.svelte`

Catches render errors in children, displays recovery UI.

```svelte
<ErrorBoundary>
  {#snippet fallback(error, reset)}
    <EmptyState title="Something went wrong" description={error.message}
      actionLabel="Try again" onaction={reset} />
  {/snippet}
  <AsyncComponent />
</ErrorBoundary>
```

**Default fallback:** Uses EmptyState (21.1) with error message and retry button.

**Files:**
- [ ] Create `ts/src/components/ErrorBoundary.svelte`
- [ ] Export from `ts/src/components/index.ts`

---

## Verification

Each phase:
1. `npx svelte-check` — zero errors in underlay
2. `npx svelte-check` in consuming apps — zero new errors
3. `bash scripts/check-file-length.sh` — all files under limits
4. Existing component usages unaffected (additive only)
