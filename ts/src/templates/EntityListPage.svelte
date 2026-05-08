<script lang="ts">
  import {
    PageHeader,
    IconButton
  } from "@poodle/svelte";
  import EntityList from "./EntityList.svelte";
  import type {
    TableColumn,
    TableRow,
    TableRowAction,
    TableCellValue,
    LogEntry,
    LogActionType,
    LogActor
  } from "@poodle/svelte";
  import type { QueryParams } from "../client/query";

  // Cross-package Svelte Snippet identity is brittle in linked local workspaces.
  // Keep the shared template boundary permissive so consumers can pass local snippets cleanly.
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type TemplateSurface = any;

  // --- Types ---

  interface FilterConfig {
    id: string;
    type: "search" | "select" | "date" | "number" | "sort";
    label: string;
    options?: { value: string; label: string }[];
    loadOptions?: () => Promise<{ value: string; label: string }[]>;
    placeholder?: string;
    sortFields?: { key: string; label: string; defaultDirection?: "asc" | "desc" }[];
  }

  interface BatchDialogContext {
    ids: string[];
    onSubmit: (values: Record<string, unknown>) => void;
    onCancel: () => void;
  }

  interface BatchDialogConfig {
    title: string;
    content: TemplateSurface;
  }

  interface BatchActionConfig {
    id: string;
    label: string;
    tone?: "default" | "danger" | "warning";
    icon?: string;
    confirm?: boolean | {
      title: string;
      description: string | ((count: number) => string);
      confirmLabel?: string;
      cancelLabel?: string;
    };
    dialog?: BatchDialogConfig;
    handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
  }

  interface ReorderConfig {
    enabled: boolean;
    handler: (orderedIds: string[]) => Promise<void>;
  }

  interface PagedListResult<TItem> {
    data: TItem[];
    /** Total matching items across all pages. Falls back to visible count if omitted. */
    total?: number | null;
    hasMore?: boolean;
  }

  interface Props {
    /** Page title */
    title: string;

    /** Hide the title while keeping header actions/chrome. */
    hideTitle?: boolean;

    /** Optional section label above the title */
    section?: string;

    /** Optional subtitle below the title */
    subtitle?: string;

    /** Optional eyebrow above the section/title */
    eyebrow?: string;

    /** Heading level for nested composition */
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    
    /** Back link URL */
    backHref?: string;
    
    /** Back link label */
    backLabel?: string;
    
    /** Data loading function. Must return paged results for the current query state. */
    dataLoader: (fetch: typeof window.fetch, token: string | null, query: QueryParams) => Promise<PagedListResult<T>>;
    
    /** Unique identifier field (default: "id") */
    idField?: string;
    
    /** Presentation mode */
    presentation: "cards" | "table" | "log";
    
    /** For cards: render snippet for each item (receives item + selection context) */
    renderItem?: TemplateSurface;
    
    /** For table: column definitions */
    columns?: TableColumn[];
    
    /** For table: row actions */
    rowActions?: (row: TableRow<T>) => { value: string; label: string }[];

    /** For table: whether to show the actions column */
    showRowActions?: boolean;

    /** For table: custom cell rendering */
    renderCell?: TemplateSurface;

    /** For table: expanded row rendering */
    renderExpandedRow?: TemplateSurface;

    /** For table: externally controlled expanded rows */
    expandedRowIds?: string[];

    /** For table: row action selection handler */
    onRowActionSelect?: (row: TableRow<T>, action: TableRowAction) => void;

    /** For log presentation: map loaded items into Poodle log entries. */
    toLogEntries?: (items: T[]) => LogEntry[];

    /** For log presentation: custom action icon snippet. */
    actionIcon?: TemplateSurface;

    /** For log presentation: custom entry details snippet. */
    entryDetails?: TemplateSurface;

    /** For log presentation: derive action type semantics. */
    getActionType?: (action: string) => LogActionType;

    /** For log presentation: format action labels. */
    formatAction?: (action: string) => string;

    /** For log presentation: format resource labels. */
    formatResourceType?: (resourceType: string) => string;

    /** For log presentation: derive actor hrefs. */
    getActorHref?: (actor: LogActor) => string;

    /** For log presentation: derive resource hrefs. */
    getResourceHref?: (resourceType: string, resourceId: string, action: string) => string | null;
    
    /** Declarative filter configuration */
    filters?: FilterConfig[];
    
    /** Batch action configuration */
    batchActions?: BatchActionConfig[];
    
    /** Reorder configuration */
    reorder?: ReorderConfig;
    
    /** Add button handler */
    onAdd?: () => void;
    
    /** Add button label */
    addLabel?: string;
    
    /** Optional callback when data changes */
    onDataChange?: () => void;
    
    /** Additional actions in the header before built-in list controls. */
    headerLeadingActions?: TemplateSurface;

    /** Additional actions in the header after built-in list controls. */
    headerActions?: TemplateSurface;

    /** Optional content rendered between the header and list body. */
    beforeList?: TemplateSurface;

    /** Query state (filters, sort, page, limit) */
    query?: QueryParams;

    /** Called when query changes (parent manages URL sync) */
    onQueryChange?: (query: QueryParams) => void;

    /** Custom reorder error handler for conflict recovery */
    onReorderError?: (error: unknown) => Promise<string | void> | string | void;
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  type T = any;

  // --- Props ---

  let {
    title,
    hideTitle = false,
    section,
    subtitle,
    eyebrow,
    headerLevel = 2,
    backHref,
    backLabel,
    dataLoader,
    idField = "id",
    presentation,
    renderItem,
    columns,
    rowActions,
    showRowActions = true,
    renderCell,
    renderExpandedRow,
    expandedRowIds,
    onRowActionSelect,
    toLogEntries,
    actionIcon,
    entryDetails,
    getActionType,
    formatAction,
    formatResourceType,
    getActorHref,
    getResourceHref,
    filters = [],
    batchActions = [],
    reorder,
    onAdd,
    addLabel = "Add",
    onDataChange,
    headerLeadingActions,
    headerActions,
    beforeList,
    query,
    onQueryChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let visibleItemCount = $state(0);
  let reorderAvailable = $state(false);
  const headerActionSizeRole = $derived(headerLevel >= 3 ? "chrome" : "control");

  // Mode flags are managed internally by EntityList
  // We track them here only for header button state

  function toggleSelectionMode() {
    if (reorderMode) reorderMode = false;
    selectionMode = !selectionMode;
  }

  function toggleReorderMode() {
    if (selectionMode) selectionMode = false;
    reorderMode = !reorderMode;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      if (selectionMode) {
        selectionMode = false;
      } else if (reorderMode) {
        reorderMode = false;
      }
    }
  }

  function handleVisibleCountChange(count: number) {
    visibleItemCount = count;
  }

  function handleSelectionModeChange(enabled: boolean) {
    selectionMode = enabled;
  }

  function handleReorderModeChange(enabled: boolean) {
    reorderMode = enabled;
  }

  function handleReorderAvailabilityChange(enabled: boolean) {
    reorderAvailable = enabled;
    if (!enabled && reorderMode) {
      reorderMode = false;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="underlay-entity-list-page">
  <PageHeader
    title={hideTitle ? null : title}
    {section}
    {subtitle}
    {eyebrow}
    level={headerLevel}
    backHref={backHref ?? null}
    backLabel={backLabel}
  >
    {#snippet actions()}
      {#if headerLeadingActions}
        {@render headerLeadingActions({
          selectionMode,
          reorderMode,
          visibleItemCount
        })}
      {/if}

      {#if (visibleItemCount > 0 || selectionMode) && batchActions.length > 0}
        <IconButton
          type="button"
          variant="secondary"
          sizeRole={headerActionSizeRole}
          tone={selectionMode ? "danger" : "default"}
          icon={selectionMode ? "x" : "check-square"}
          ariaLabel={selectionMode ? "Cancel selection" : "Select items"}
          tooltip={selectionMode ? "Cancel Selection" : "Select Items"}
          disabled={reorderMode}
          on:click={toggleSelectionMode}
        />
      {/if}
      
      {#if reorderAvailable || reorderMode}
        <IconButton
          type="button"
          variant="secondary"
          sizeRole={headerActionSizeRole}
          tone={reorderMode ? "danger" : "default"}
          icon="arrow-up-down"
          ariaLabel={reorderMode ? "Cancel reorder" : "Reorder items"}
          tooltip={reorderMode ? "Cancel Reorder" : "Reorder Items"}
          disabled={selectionMode}
          on:click={toggleReorderMode}
        />
      {/if}
      
      {#if onAdd}
        <IconButton
          type="button"
          variant="primary"
          sizeRole={headerActionSizeRole}
          icon="plus"
          ariaLabel={addLabel}
          tooltip={addLabel}
          disabled={selectionMode || reorderMode}
          on:click={onAdd}
        />
      {/if}
      
      {#if headerActions}
        {@render headerActions({
          selectionMode,
          reorderMode,
          visibleItemCount
        })}
      {/if}
    {/snippet}
  </PageHeader>

  {#if beforeList}
    <div class="underlay-entity-list-page__before-list">
      {@render beforeList({
        selectionMode,
        reorderMode,
        visibleItemCount
      })}
    </div>
  {/if}

  <EntityList
    {dataLoader}
    {idField}
    {presentation}
    {renderItem}
    {columns}
    {rowActions}
    {showRowActions}
    {renderCell}
    {renderExpandedRow}
    {expandedRowIds}
    {onRowActionSelect}
    {toLogEntries}
    {actionIcon}
    {entryDetails}
    {getActionType}
    {formatAction}
    {formatResourceType}
    {getActorHref}
    {getResourceHref}
    {filters}
    {batchActions}
    {reorder}
    {onAdd}
    {addLabel}
    {onDataChange}
    {query}
    {onQueryChange}
    {onReorderError}
    selectionMode={selectionMode}
    reorderMode={reorderMode}
    onSelectionModeChange={handleSelectionModeChange}
    onReorderModeChange={handleReorderModeChange}
    onVisibleCountChange={handleVisibleCountChange}
    onReorderAvailabilityChange={handleReorderAvailabilityChange}
  />
</div>

<style>
  .underlay-entity-list-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-entity-list-page__before-list {
    display: contents;
  }
</style>
