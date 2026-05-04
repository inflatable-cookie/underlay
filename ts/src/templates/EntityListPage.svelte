<script lang="ts">
  import type { Snippet } from "svelte";
  import {
    PageHeader,
    IconButton,
    Button
  } from "@poodle/svelte";
  import EntityList from "./EntityList.svelte";
  import type { TableColumn, TableRow } from "@poodle/svelte";

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
    confirm?: boolean | { title: string; description: string | ((count: number) => string) };
    dialog?: BatchDialogConfig;
    handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
  }

  interface ReorderConfig {
    enabled: boolean;
    handler: (orderedIds: string[]) => Promise<void>;
  }

  interface Props {
    /** Page title */
    title: string;
    
    /** Back link URL */
    backHref?: string;
    
    /** Back link label */
    backLabel?: string;
    
    /** Data loading function */
    dataLoader: (fetch: typeof window.fetch, token: string | null, query: Record<string, unknown>) => Promise<T[]>;
    
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

    /** External filter values (for URL sync) */
    filterValues?: Record<string, string>;

    /** External sort state (for URL sync) */
    sort?: { field: string; direction: "asc" | "desc" }[];

    /** Called when filters change (parent manages URL sync) */
    onFilterChange?: (id: string, value: string) => void;

    /** Called when sort changes (parent manages URL sync) */
    onSortChange?: (sort: { field: string; direction: "asc" | "desc" }[]) => void;

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
    filterValues,
    sort,
    onFilterChange,
    onSortChange,
    onReorderError
  }: Props = $props();

  // --- State ---

  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let itemCount = $state(0);

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

  function handleCountChange(count: number) {
    itemCount = count;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="entity-list-page">
  <PageHeader
    {title}
    count={itemCount}
    backHref={backHref ?? null}
    backLabel={backLabel}
  >
    {#snippet actions()}
      {#if itemCount > 0 && !reorderMode && batchActions.length > 0}
        <IconButton
          type="button"
          variant="secondary"
          tone={selectionMode ? "danger" : "default"}
          icon={selectionMode ? "x" : "check-square"}
          ariaLabel={selectionMode ? "Cancel selection" : "Select items"}
          tooltip={selectionMode ? "Cancel Selection" : "Select Items"}
          onclick={toggleSelectionMode}
        />
      {/if}
      
      {#if itemCount > 1 && !selectionMode && reorder?.enabled}
        <IconButton
          type="button"
          variant="secondary"
          tone={reorderMode ? "danger" : "default"}
          icon="arrow-up-down"
          ariaLabel={reorderMode ? "Cancel reorder" : "Reorder items"}
          tooltip={reorderMode ? "Cancel Reorder" : "Reorder Items"}
          onclick={toggleReorderMode}
        />
      {/if}
      
      {#if !selectionMode && !reorderMode && onAdd}
        <IconButton
          type="button"
          variant="primary"
          icon="plus"
          ariaLabel={addLabel}
          tooltip={addLabel}
          onclick={onAdd}
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
    {filterValues}
    {sort}
    {onFilterChange}
    {onSortChange}
    {onReorderError}
    selectionMode={selectionMode}
    reorderMode={reorderMode}
    onCountChange={handleCountChange}
  />
</div>

<style>
  .entity-list-page {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }
</style>
