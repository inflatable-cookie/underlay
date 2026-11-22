<script lang="ts" generics="T extends object = Record<string, unknown>">
  import {
    DataTable,
    EditableList,
    ListCard,
    ListGrid,
    LogList
  } from "@inflatable-cookie/poodle-svelte";
  import type {
    EditableListItem,
    LogEntry,
    TableColumn,
    TableRow,
    TableRowAction
  } from "@inflatable-cookie/poodle-svelte";
  import type {
    EntityListItemContext,
    EntityListSharedProps,
    TemplateSurface
  } from "./template.types";

  interface Props {
    presentation: "cards" | "table" | "log";
    items: T[];
    itemIds: string[];
    getItemKey: (item: T) => string;
    getItemContext: (item: T) => EntityListItemContext;
    logEmptyMessage: string;
    // Reorder props
    reorderMode: boolean;
    hasReorderController: boolean;
    reorderDirty: boolean;
    reorderSubmitting: boolean;
    reorderError: string | null;
    editableItems: EditableListItem[];
    getReorderOriginalItem: (id: string) => T | undefined;
    isReorderHighlighted: (id: string) => boolean;
    onReorderSave: () => Promise<void>;
    onReorderCancel: () => void;
    onReorderUpdate: (items: EditableListItem[]) => void;
    customReorderContent?: TemplateSurface;
    renderReorderItem?: TemplateSurface;
    onRefetch: () => Promise<void>;
    // Cards props
    renderItem?: TemplateSurface;
    listGridVariant?: "default" | "compact";
    listGridGap?: number | string | null;
    // Table props
    columns?: TableColumn[];
    rowActions?: EntityListSharedProps<T>["rowActions"];
    showRowActions?: boolean;
    renderCell?: TemplateSurface;
    renderExpandedRow?: TemplateSurface;
    expandedRowIds?: string[];
    selectionMode: boolean;
    selectedIds: string[];
    onRowActionSelect?: (row: TableRow<T>, action: TableRowAction) => void;
    onRowToggle: (rowId: string, selected: boolean) => void;
    onToggleAll: (selected: boolean) => void;
    // Log props
    logEntries: LogEntry[];
    actionIcon?: TemplateSurface;
    entryDetails?: TemplateSurface;
    getActionType?: EntityListSharedProps<T>["getActionType"];
    formatAction?: EntityListSharedProps<T>["formatAction"];
    formatResourceType?: EntityListSharedProps<T>["formatResourceType"];
    getActorHref?: EntityListSharedProps<T>["getActorHref"];
    getResourceHref?: EntityListSharedProps<T>["getResourceHref"];
  }

  let {
    presentation,
    items,
    itemIds,
    getItemKey,
    getItemContext,
    logEmptyMessage,
    reorderMode,
    hasReorderController,
    reorderDirty,
    reorderSubmitting,
    reorderError,
    editableItems,
    getReorderOriginalItem,
    isReorderHighlighted,
    onReorderSave,
    onReorderCancel,
    onReorderUpdate,
    customReorderContent,
    renderReorderItem,
    onRefetch,
    renderItem,
    listGridVariant = "default",
    listGridGap = "1rem",
    columns,
    rowActions,
    showRowActions = true,
    renderCell,
    renderExpandedRow,
    expandedRowIds = [],
    selectionMode,
    selectedIds,
    onRowActionSelect,
    onRowToggle,
    onToggleAll,
    logEntries,
    actionIcon,
    entryDetails,
    getActionType,
    formatAction,
    formatResourceType,
    getActorHref,
    getResourceHref
  }: Props = $props();

  const tableColumns = $derived<TableColumn[]>(columns ?? []);

  const tableRows = $derived<TableRow<T>[]>(
    items.map((item) => ({
      id: getItemKey(item),
      cells: tableColumns.reduce(
        (acc, col) => {
          acc[col.id] = String((item as Record<string, unknown>)[col.id] ?? "");
          return acc;
        },
        {} as Record<string, string>
      ),
      data: item
    }))
  );

  function handleRowActionSelect(event: { row: TableRow; action: TableRowAction }) {
    // Poodle's DataTable emits a non-generic TableRow; our row payload is
    // TableRow<T> at runtime.
    onRowActionSelect?.(event.row as TableRow<T>, event.action);
  }
</script>

{#if reorderMode && customReorderContent}
  {@render customReorderContent({
    items,
    cancel: onReorderCancel,
    refetch: onRefetch
  })}
{:else if reorderMode && hasReorderController}
  <EditableList
    items={editableItems}
    embeddedHandle
    dirty={reorderDirty}
    submitting={reorderSubmitting}
    errorMessage={reorderError}
    showWorkflowChrome={false}
    onSubmit={onReorderSave}
    onCancel={onReorderCancel}
    onReorder={onReorderUpdate}
  >
    {#snippet item(reorderEntry)}
      {@const originalItem = getReorderOriginalItem(reorderEntry.id)}
      {#if renderReorderItem && originalItem}
        <div data-reorder-highlighted={isReorderHighlighted(reorderEntry.id)}>
          {@render renderReorderItem(originalItem, {
            ...getItemContext(originalItem),
            highlighted: isReorderHighlighted(reorderEntry.id)
          })}
        </div>
      {:else if renderItem && originalItem}
        {@render renderItem(originalItem, getItemContext(originalItem))}
      {:else}
        <ListCard title={reorderEntry.id} layout="compact" showReorderHandle />
      {/if}
    {/snippet}
  </EditableList>
{:else if presentation === "cards"}
  <ListGrid minItemWidth="var(--underlay-list-grid-min, 20rem)" variant={listGridVariant} gap={listGridGap}>
    {#each items as item (getItemKey(item))}
      {#if renderItem}
        {@render renderItem(item, getItemContext(item))}
      {/if}
    {/each}
  </ListGrid>
{:else if presentation === "log"}
  <LogList
    entries={logEntries}
    variant="audit"
    emptyMessage={logEmptyMessage}
    actionIcon={actionIcon}
    entryDetails={entryDetails}
    {getActionType}
    {formatAction}
    {formatResourceType}
    {getActorHref}
    {getResourceHref}
  />
{:else if presentation === "table"}
  <DataTable
    columns={tableColumns}
    rows={tableRows}
    {expandedRowIds}
    {showRowActions}
    rowActions={rowActions as unknown as TableRowAction[] | ((row: TableRow) => TableRowAction[]) | undefined}
    selectable={selectionMode}
    selectedRowIds={selectedIds}
    emptyMessage="No items found"
    onRowActionSelect={handleRowActionSelect}
    onRowToggle={({ rowId, selected }) => onRowToggle(rowId, selected)}
    onToggleAll={({ selected }) => onToggleAll(selected)}
  >
    {#snippet cell(column, row, value)}
      {#if renderCell}
        {@render renderCell(column, row, value)}
      {:else}
        {value}
      {/if}
    {/snippet}
    {#snippet expandedRow(row)}
      {#if renderExpandedRow}
        {@render renderExpandedRow(row)}
      {/if}
    {/snippet}
  </DataTable>
{/if}

<style>
  [data-reorder-highlighted="true"] {
    outline: 2px solid var(--underlay-color-accent, #14b8a6);
    outline-offset: 0.25rem;
    border-radius: 0.75rem;
  }
</style>
