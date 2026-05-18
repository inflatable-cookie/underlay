<script lang="ts">
  import {
    PageHeader,
    IconButton
  } from "@poodle/svelte";
  import { getBackButtonInfo } from "../patterns/navigation";
  import { default as EntityList } from "./EntityList.svelte";
  import { default as EntityReorderControls } from "./EntityReorderControls.svelte";
  import type {
    TableColumn,
    TableRow,
    TableRowAction,
    LogEntry,
    LogActionType,
    LogActor
  } from "@poodle/svelte";
  import type {
    BatchActionConfig,
    EntityListCapabilitiesLoader,
    EntityListDataLoader,
    EntityListSharedProps,
    FilterConfig,
    ListVariantDefinition,
    LogActionFormatter,
    LogActionTypeResolver,
    LogActorHrefResolver,
    LogEntryMapper,
    LogResourceHrefResolver,
    LogResourceTypeFormatter,
    PagedListResult,
    ReorderActionState,
    ReorderConfig,
    TableRowActionFactory,
    TemplateSurface
  } from "./template.types";

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

    /** Explicit contextual-back state override when the caller resolved back info already. */
    backIsContextual?: boolean;

    /** Resolve contextual back navigation from shared Underlay navigation state. */
    resolveBackContext?: boolean;
    
    /** Data loading function. Must return paged results for the current query state. */
    dataLoader: EntityListDataLoader<T>;

    /** Optional external reload signal for non-query data refreshes. */
    reloadKey?: string | number;
    
    /** Unique identifier field (default: "id") */
    idField?: string;
    
    /** Presentation mode */
    presentation: "cards" | "table" | "log";
    
    /** For cards: render snippet for each item (receives item + selection context) */
    renderItem?: TemplateSurface;

    /** For reorder mode: render snippet for each item in reorder posture */
    renderReorderItem?: TemplateSurface;
    
    /** For table: column definitions */
    columns?: TableColumn[];
    
    /** For table: row actions */
    rowActions?: TableRowActionFactory<T>;

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
    toLogEntries?: LogEntryMapper<T>;

    /** For log presentation: custom action icon snippet. */
    actionIcon?: TemplateSurface;

    /** For log presentation: custom entry details snippet. */
    entryDetails?: TemplateSurface;

    /** For log presentation: derive action type semantics. */
    getActionType?: LogActionTypeResolver;

    /** For log presentation: format action labels. */
    formatAction?: LogActionFormatter;

    /** For log presentation: format resource labels. */
    formatResourceType?: LogResourceTypeFormatter;

    /** For log presentation: derive actor hrefs. */
    getActorHref?: LogActorHrefResolver;

    /** For log presentation: derive resource hrefs. */
    getResourceHref?: LogResourceHrefResolver;
    
    /** Declarative filter configuration */
    filters?: FilterConfig[];

    /** Named baseline query variants rendered above filters. */
    queryVariants?: ListVariantDefinition[];

    /** Fallback variant used when query state does not name one explicitly. */
    defaultVariantId?: string;

    /** Optional loader for API-published list variants and filter definitions. */
    capabilitiesLoader?: EntityListCapabilitiesLoader;
    
    /** Batch action configuration */
    batchActions?: BatchActionConfig[];
    
    /** Reorder configuration */
    reorder?: ReorderConfig<T>;

    /** Optional custom reorder surface for non-flat reorder workflows */
    customReorderContent?: TemplateSurface;
    
    /** Add button handler */
    onAdd?: () => void;
    
    /** Add button label */
    addLabel?: string;
    
    /** Optional callback when data changes */
    onDataChange?: () => void;

    /** Optional callback when selected item IDs change */
    onSelectedIdsChange?: (ids: string[]) => void;
    
    /** Additional actions in the header before built-in list controls. */
    headerLeadingActions?: TemplateSurface;

    /** Additional actions in the header after built-in list controls. */
    headerActions?: TemplateSurface;

    /** Optional content rendered between the header and list body. */
    beforeList?: TemplateSurface;

    /** Query state (filters, sort, page, limit) */
    query?: EntityListSharedProps<T>["query"];

    /** Called when query changes (parent manages URL sync) */
    onQueryChange?: EntityListSharedProps<T>["onQueryChange"];

    /** Custom reorder error handler for conflict recovery */
    onReorderError?: EntityListSharedProps<T>["onReorderError"];
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
    backIsContextual = false,
    resolveBackContext = true,
    dataLoader,
    reloadKey,
    idField = "id",
    presentation,
    renderItem,
    renderReorderItem,
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
    queryVariants = [],
    defaultVariantId,
    capabilitiesLoader,
    batchActions = [],
    reorder,
    customReorderContent,
    onAdd,
    addLabel = "Add",
    onDataChange,
    onSelectedIdsChange,
    headerLeadingActions,
    headerActions,
    beforeList,
    query,
    onQueryChange,
    onReorderError
  }: Props = $props();

  let resolvedBackInfo = $state<{ href: string; label: string; contextual: boolean } | null>(null);

  // --- State ---

  let selectionMode = $state(false);
  let reorderMode = $state(false);
  let selectedIds = $state<string[]>([]);
  let visibleItemCount = $state(0);
  let reorderAvailable = $state(false);
  let reorderActionState = $state<ReorderActionState | null>(null);
  const headerActionSizeRole = $derived(headerLevel >= 3 ? "chrome" : "control");

  $effect(() => {
    if (!backHref) {
      resolvedBackInfo = null;
      return;
    }

    const fallbackLabel = backLabel ?? "Back";
    if (!resolveBackContext) {
      resolvedBackInfo = {
        href: backHref,
        label: fallbackLabel,
        contextual: backIsContextual
      };
      return;
    }

    const contextualBackInfo = getBackButtonInfo(fallbackLabel, backHref);
    resolvedBackInfo = {
      href: contextualBackInfo.href,
      label: contextualBackInfo.label,
      contextual: Boolean(contextualBackInfo.isContextual || backIsContextual)
    };
  });

  // Mode flags are managed internally by EntityList
  // We track them here only for header button state

  function toggleSelectionMode() {
    if (reorderMode) reorderMode = false;
    selectionMode = !selectionMode;
  }

  function toggleReorderMode() {
    if (selectionMode) selectionMode = false;
    if (reorderActionState) {
      if (reorderMode) {
        reorderActionState.cancel();
      } else {
        void reorderActionState.enter();
      }
      return;
    }
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

  function handleSelectedIdsChange(ids: string[]) {
    selectedIds = ids;
    onSelectedIdsChange?.(ids);
  }

  function handleReorderAvailabilityChange(enabled: boolean) {
    reorderAvailable = enabled;
    if (!enabled && reorderMode) {
      reorderMode = false;
    }
  }

  function handleReorderActionStateChange(state: ReorderActionState) {
    reorderActionState = state;
  }

  async function enterReorderMode() {
    if (selectionMode) selectionMode = false;
    if (reorderActionState) {
      await reorderActionState.enter();
    } else {
      toggleReorderMode();
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
    backHref={resolvedBackInfo?.href ?? null}
    backLabel={resolvedBackInfo?.label ?? backLabel}
    backIsContextual={resolvedBackInfo?.contextual ?? false}
  >
    {#snippet actions()}
      {#if headerLeadingActions}
        {@render headerLeadingActions({
          selectionMode,
          reorderMode,
          visibleItemCount,
          selectedIds,
          selectedCount: selectedIds.length
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
          onClick={toggleSelectionMode}
        />
      {/if}
      
      <EntityReorderControls
        active={reorderMode}
        available={reorderAvailable}
        dirty={reorderActionState?.dirty ?? false}
        saving={reorderActionState?.saving ?? false}
        disabled={selectionMode}
        sizeRole={headerActionSizeRole}
        onEnter={enterReorderMode}
        onSave={reorderActionState?.save}
        onCancel={() => {
          if (reorderActionState) {
            reorderActionState.cancel();
          } else {
            toggleReorderMode();
          }
        }}
      />
      
      {#if onAdd}
        <IconButton
          type="button"
          variant="primary"
          sizeRole={headerActionSizeRole}
          icon="plus"
          ariaLabel={addLabel}
          tooltip={addLabel}
          disabled={selectionMode || reorderMode}
          onClick={onAdd}
        />
      {/if}
      
      {#if headerActions}
        {@render headerActions({
          selectionMode,
          reorderMode,
          visibleItemCount,
          selectedIds,
          selectedCount: selectedIds.length
        })}
      {/if}
    {/snippet}
  </PageHeader>

  {#if beforeList}
    <div class="underlay-entity-list-page__before-list">
      {@render beforeList({
        selectionMode,
        reorderMode,
        visibleItemCount,
        selectedIds,
        selectedCount: selectedIds.length
      })}
    </div>
  {/if}

  <EntityList
    {dataLoader}
    {reloadKey}
    {idField}
    {presentation}
    {renderItem}
    {renderReorderItem}
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
    {queryVariants}
    {defaultVariantId}
    {capabilitiesLoader}
    {batchActions}
    {reorder}
    {customReorderContent}
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
    onSelectedIdsChange={handleSelectedIdsChange}
    onVisibleCountChange={handleVisibleCountChange}
    onReorderAvailabilityChange={handleReorderAvailabilityChange}
    onReorderActionStateChange={handleReorderActionStateChange}
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
