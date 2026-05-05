<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    PageHeader,
    IconButton
  } from "@poodle/svelte";
  import EntityList from "./EntityList.svelte";
  import type { TableColumn, TableRow } from "@poodle/svelte";
  import type { QueryParams } from "../client/query";

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
    content: Snippet<[BatchDialogContext]>;
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
    
    /** Back link URL */
    backHref?: string;
    
    /** Back link label */
    backLabel?: string;
    
    /** Data loading function. Must return paged results for the current query state. */
    dataLoader: (fetch: typeof window.fetch, token: string | null, query: QueryParams) => Promise<PagedListResult<T>>;
    
    /** Unique identifier field (default: "id") */
    idField?: string;
    
    /** Presentation mode */
    presentation: "cards" | "table";
    
    /** For cards: render snippet for each item (receives item + selection context) */
    renderItem?: Snippet<[T, { selectionMode: boolean; selected: boolean; onToggle: (selected: boolean) => void; refetch: () => Promise<void> }]>;
    
    /** For table: column definitions */
    columns?: TableColumn[];
    
    /** For table: row actions */
    rowActions?: (row: TableRow<T>) => { value: string; label: string }[];
    
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
    
    /** Additional actions in the header */
    headerActions?: Snippet;

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
    backHref,
    backLabel,
    dataLoader,
    idField = "id",
    presentation,
    renderItem,
    columns,
    rowActions,
    filters = [],
    batchActions = [],
    reorder,
    onAdd,
    addLabel = "Add",
    onDataChange,
    headerActions,
    query,
    onQueryChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let visibleItemCount = $state(0);
  let totalItemCount = $state(0);
  let reorderAvailable = $state(false);

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

  function handleTotalCountChange(count: number) {
    totalItemCount = count;
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
    {title}
    count={totalItemCount}
    backHref={backHref ?? null}
    backLabel={backLabel}
  >
    {#snippet actions()}
      {#if visibleItemCount > 0 && !reorderMode && batchActions.length > 0}
        <IconButton
          type="button"
          variant="secondary"
          tone={selectionMode ? "danger" : "default"}
          icon={selectionMode ? "x" : "check-square"}
          ariaLabel={selectionMode ? "Cancel selection" : "Select items"}
          tooltip={selectionMode ? "Cancel Selection" : "Select Items"}
          on:click={toggleSelectionMode}
        />
      {/if}
      
      {#if reorderAvailable && !selectionMode}
        <IconButton
          type="button"
          variant="secondary"
          tone={reorderMode ? "danger" : "default"}
          icon="arrow-up-down"
          ariaLabel={reorderMode ? "Cancel reorder" : "Reorder items"}
          tooltip={reorderMode ? "Cancel Reorder" : "Reorder Items"}
          on:click={toggleReorderMode}
        />
      {/if}
      
      {#if !selectionMode && !reorderMode && onAdd}
        <IconButton
          type="button"
          variant="primary"
          icon="plus"
          ariaLabel={addLabel}
          tooltip={addLabel}
          on:click={onAdd}
        />
      {/if}
      
      {#if headerActions}
        {@render headerActions()}
      {/if}
    {/snippet}
  </PageHeader>

  <EntityList
    {dataLoader}
    {idField}
    {presentation}
    {renderItem}
    {columns}
    {rowActions}
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
    onTotalCountChange={handleTotalCountChange}
    onReorderAvailabilityChange={handleReorderAvailabilityChange}
  />
</div>

<style>
  .underlay-entity-list-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
