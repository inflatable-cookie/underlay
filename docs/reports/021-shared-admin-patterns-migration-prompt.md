# Migrate to Shared Admin Patterns (Roadmap 021)

## Context

The Underlay library at `.` has 8 new shared admin patterns (roadmap 021). Full documentation is at:
- **Roadmap**: `docs/roadmap/021-shared-admin-components.md`
- **Guide**: `docs/guides/098-shared-admin-patterns.md`

Read both files thoroughly before starting. The patterns are:

1. **EmptyState** (`@decodelabs/underlay/components`) — Rich empty state replacing plain `<p>` text
2. **EntityActionsMenu** (`@decodelabs/underlay/patterns`) — Dropdown with edit, custom actions, and soft-delete flow
3. **Drawer** (`@decodelabs/underlay/components`) — Slide-out side panel
4. **DetailPageShell** (`@decodelabs/underlay/patterns`) — Composable entity detail page with tabs
5. **AutonomousList** (`@decodelabs/underlay/patterns`) — Self-contained list with filters, batch, reorder
6. **InlineEditableField** (`@decodelabs/underlay/components`) — Click-to-edit text field
7. **KeyboardShortcuts** (`@decodelabs/underlay/patterns`) — Centralized shortcut registration
8. **ErrorBoundary** (`@decodelabs/underlay/components`) — Render error catch with recovery UI

## Your Task

Migrate this project to use these shared patterns where applicable. Work through each pattern in priority order. For each migration:

1. **Read the existing code** — Understand what the current implementation does before changing it
2. **Read the underlay component** — Read the actual source in `ts/src/components/` or `ts/src/patterns/` to understand the exact API
3. **Migrate incrementally** — Change one file at a time, verify with `npx svelte-check` after each change
4. **Preserve behavior** — The migration must be behavior-preserving. Don't change what the UI does, only how it's implemented
5. **Don't force it** — If a component has significant custom logic that doesn't fit the shared pattern, leave it alone

## Migration Priority Order

### Priority 1: EmptyState (quick wins)

Find all plain `<p>` empty state messages in list views and detail page tabs. Replace with:

```svelte
<EmptyState title="No items found" />
```

or with an icon/description/action as appropriate. Don't change InlineListCard `emptyMessage` props — those already work fine.

### Priority 2: EntityActionsMenu (medium effort, high value)

Find entity action menus that follow this pattern:
- Import CopyActionsMenu + AlertDialog
- Define copies array and actions array
- Manage `softDeleteOpen` state
- Have requestSoftDelete/confirmSoftDelete/cancelSoftDelete functions
- Auth token check + API call + toast

Replace with EntityActionsMenu. Key mapping:
- `copies` → `copies` (same format)
- Edit action `gotoWithContext(...)` → `onEdit={() => gotoWithContext(...)}`
- `confirmLabel` on soft delete → `deleteConfig.confirmLabel`
- `onSoftDeleteSuccess` → `onDeleteSuccess`
- Any extra actions between edit and delete → `customActions`
- `trigger` snippet → `trigger`

The `deleteConfig.execute` should contain the async delete logic (auth check + API call). EntityActionsMenu handles the AlertDialog, error toasting, and state internally.

**Important**: EntityActionsMenu does NOT import from `$app/navigation` or use `gotoWithContext` — it takes an `onEdit` callback. The consuming app provides its own navigation logic.

### Priority 3: DetailPageShell (high effort, high value)

Find detail pages that manually compose:
- PageHeader with title, breadcrumbs, backHref
- PageHeaderMeta / PageHeaderMetaRow / PageHeaderMetaItem with ID + StatusBadge
- TabsRoot / TabsList / TabsTrigger / TabsContent

Replace with DetailPageShell + DetailMeta sub-components. This is the highest-effort migration because each page has unique metadata and tab configurations.

### Priority 4: AutonomousList (highest effort, highest value)

Find list components that manually wire up:
- useAuthenticatedData or createListController
- useBatchActions with registerAction
- FilterBar with custom filter components
- BatchActionBar + confirmation dialogs
- Escape key handler for selection mode
- Loading/error/empty state machine

**Note**: Many list components in Acowtancy have been decomposed into sub-files (e.g., `outcomes-list/data-fetch.ts`, `outcomes-list/filters.ts`, `outcomes-list/batch-actions.ts`). AutonomousList may not be a drop-in replacement for these heavily decomposed lists — evaluate whether the AutonomousList abstraction is a good fit or whether the existing decomposition is already clean enough. Simpler lists are better candidates.

### Priority 5: ErrorBoundary (low effort)

Wrap top-level route components or risky async components:

```svelte
<ErrorBoundary>
  <RiskyComponent />
</ErrorBoundary>
```

### Lower Priority (opportunistic)

- **InlineEditableField**: Only if click-to-edit patterns exist
- **KeyboardShortcuts**: Only if there are manual keyboard handlers beyond what AutonomousList already handles
- **Drawer**: Only if there are slide-out panels

## Verification

After each batch of changes:
1. `npx svelte-check` — zero new errors
2. Visual review — ensure the UI looks the same
3. Functional check — ensure behavior is preserved

## Do NOT

- Force-migrate components with significant custom logic that doesn't fit the shared pattern
- Change behavior — this is a refactoring exercise
- Modify anything in the underlay library itself
- Create new files unless absolutely necessary
- Run `npm install` or `bun install` in the reference app directory (cyclic hard link issues)

---

## Recommended Session Plan

Paste the prompt above into each agent session, run from the project root.

| Session | App Path | Priority | Notes |
|---------|----------|----------|-------|
| 1 | `../acowtancy` | Highest | 14 action menus, 38+ detail pages, 18 lists, 14 empty states |
| 2 | `../compli-me` | High | 3 action menus, 8+ detail pages, 9+ lists |
| 3 | `../underlay-reference` | Medium | Reference app, good for showcasing patterns |
| 4 | `../songsprout` | Low | Minimal underlay integration currently |

Skip Loophole Composer — it's marked "out of date" and has zero underlay integration.
