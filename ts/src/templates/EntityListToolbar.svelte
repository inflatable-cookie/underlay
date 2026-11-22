<script lang="ts">
  import {
    CardToggleGroup,
    FilterToolbar,
    IconButton,
    OrderBy,
    PaginationSummary,
    Pill,
    Select,
    TextInput
  } from "@inflatable-cookie/poodle-svelte";
  import type { CardToggleItem } from "@inflatable-cookie/poodle-svelte";
  import { orderByToSortFields, type QueryParams, type SortField } from "../client/query";
  import { getListFilterValue, isListFilterActive } from "./entity-list-query";
  import type { FilterConfig, ListVariantDefinition, TemplateFilterOption } from "./template.types";

  interface Props {
    uid: string;
    ariaLabel: string;
    queryVariantAriaLabel: string;
    collapsed: boolean;
    currentPage: number;
    totalPages: number;
    totalCount: number;
    currentPageSize: number;
    isRefetching: boolean;
    activeQueryVariant: ListVariantDefinition | null;
    activeAdHocFilterCount: number;
    hasSort: boolean;
    hasActiveEphemeralControls: boolean;
    queryVariantItems: CardToggleItem[];
    currentVariantId: string | null;
    queryVariantColumns: 1 | 2 | 3 | 4;
    filters: FilterConfig[];
    currentQuery: QueryParams;
    effectiveSort: SortField[];
    searchDebounceMs: number;
    getSelectItems: (filter: FilterConfig) => TemplateFilterOption[];
    getSearchValue: (filter: FilterConfig, queryValue: string) => string;
    onSearchChange: (
      filter: FilterConfig,
      value: string,
      debounceMs: number,
      apply: () => void
    ) => void;
    onApplyFilter: (filter: FilterConfig, value: string) => void;
    onVariantChange: (value: string | null) => void;
    onSortChange: (sortFields: SortField[]) => void;
    onClearEphemeral: () => void;
    onRefresh: () => void;
  }

  let {
    uid,
    ariaLabel,
    queryVariantAriaLabel,
    collapsed,
    currentPage,
    totalPages,
    totalCount,
    currentPageSize,
    isRefetching,
    activeQueryVariant,
    activeAdHocFilterCount,
    hasSort,
    hasActiveEphemeralControls,
    queryVariantItems,
    currentVariantId,
    queryVariantColumns,
    filters,
    currentQuery,
    effectiveSort,
    searchDebounceMs,
    getSelectItems,
    getSearchValue,
    onSearchChange,
    onApplyFilter,
    onVariantChange,
    onSortChange,
    onClearEphemeral,
    onRefresh
  }: Props = $props();

  function getVariantPillTone(
    tone: ListVariantDefinition["tone"]
  ): "neutral" | "info" | "success" | "warning" | "danger" {
    switch (tone) {
      case "info":
      case "success":
      case "warning":
      case "danger":
        return tone;
      default:
        return "neutral";
    }
  }

  function getSearchPlaceholder(filter: FilterConfig): string {
    return filter.placeholder ?? `Search ${filter.label.toLowerCase()}...`;
  }

  function getSortOrderValue() {
    return effectiveSort.map((field) => ({
      key: field.field,
      direction: field.direction
    }));
  }
</script>

<FilterToolbar
  {ariaLabel}
  summaryText="Views & Filters"
  size="sm"
  {collapsed}
>
  {#snippet summary()}
    <span class="underlay-entity-list__toolbar-summary">
      <PaginationSummary
        currentPage={currentPage}
        totalPages={totalPages}
        totalItems={totalCount}
        pageSize={currentPageSize}
      />
      {#if isRefetching}
        <span
          class="underlay-entity-list__refetching"
          role="status"
          aria-live="polite"
        >
          Updating…
        </span>
      {/if}
      {#if activeQueryVariant}
        <Pill
          tone={getVariantPillTone(activeQueryVariant.tone)}
          appearance="badge"
          size="xs"
          typography="inherit"
        >
          {activeQueryVariant.label}
        </Pill>
      {/if}
      {#if activeAdHocFilterCount > 0}
        <Pill tone="neutral" appearance="badge" size="xs" typography="inherit">
          {activeAdHocFilterCount} filter{activeAdHocFilterCount === 1 ? "" : "s"}
        </Pill>
      {/if}
      {#if hasSort}
        <Pill tone="neutral" appearance="badge" size="xs" typography="inherit">
          Sorted
        </Pill>
      {/if}
    </span>
  {/snippet}

  {#snippet actions()}
    {#if hasActiveEphemeralControls}
      <IconButton
        icon="x"
        variant="ghost"
        size="sm"
        ariaLabel="Clear filters and sort"
        tooltip="Clear filters and sort"
        onClick={onClearEphemeral}
      />
    {/if}
    <IconButton
      icon="refresh-cw"
      variant="ghost"
      size="sm"
      ariaLabel="Refresh list"
      tooltip="Refresh"
      onClick={onRefresh}
    />
  {/snippet}

  {#if queryVariantItems.length > 0}
    <div class="underlay-entity-list__query-variants">
      <CardToggleGroup
        items={queryVariantItems}
        value={currentVariantId}
        columns={queryVariantColumns}
        allowDeactivation
        size="xs"
        density="compact"
        ariaLabel={queryVariantAriaLabel}
        onValueChange={onVariantChange}
      />
    </div>
  {/if}

  {#each filters as filter (filter.id)}
    <div
      class="underlay-entity-list__filter-control"
      data-active={isListFilterActive(currentQuery, filter)}
    >
      {#if filter.type === "search"}
        <TextInput
          id={`${uid}-filter-${filter.id}`}
          type="search"
          size="sm"
          value={getSearchValue(filter, getListFilterValue(currentQuery, filter))}
          ariaLabel={filter.label}
          placeholder={getSearchPlaceholder(filter)}
          onValueChange={(value) =>
            onSearchChange(filter, value, searchDebounceMs, () => onApplyFilter(filter, value))}
        />
      {:else if filter.type === "select"}
        <Select
          id={`${uid}-filter-${filter.id}`}
          size="sm"
          disabled={filter.disabled ?? false}
          value={getListFilterValue(currentQuery, filter)}
          options={getSelectItems(filter)}
          loadOptions={filter.searchable ? filter.loadOptions ?? null : null}
          loadKey={filter.loadKey ?? null}
          searchable={filter.searchable ?? false}
          native={false}
          clearable
          placeholder={filter.placeholder ?? filter.label}
          ariaLabel={filter.label}
          onValueChange={(value) => onApplyFilter(filter, value)}
        />
      {:else if filter.type === "sort" && filter.sortFields}
        <OrderBy
          fields={filter.sortFields}
          value={getSortOrderValue()}
          size="sm"
          ariaLabel={filter.label}
          showClearButton={false}
          onChange={(value) => onSortChange(orderByToSortFields(value))}
          compact
        />
      {/if}
    </div>
  {/each}
</FilterToolbar>

<style>
  .underlay-entity-list__toolbar-summary {
    display: inline-flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    min-width: 0;
  }

  .underlay-entity-list__toolbar-summary :global(.poodle-pagination-summary) {
    font-size: 0.875em;
  }

  .underlay-entity-list__refetching {
    font-size: 0.8125em;
    opacity: 0.7;
    font-style: italic;
  }

  .underlay-entity-list__filter-control {
    min-width: 0;
    opacity: 0.68;
    transition:
      opacity 120ms ease,
      filter 120ms ease;
  }

  .underlay-entity-list__filter-control[data-active="true"],
  .underlay-entity-list__filter-control:focus-within,
  .underlay-entity-list__filter-control:hover {
    opacity: 1;
  }

  .underlay-entity-list__filter-control[data-active="false"] :global(.poodle-select__value[data-placeholder="true"]),
  .underlay-entity-list__filter-control[data-active="false"] :global(.poodle-order-by__summary[data-placeholder="true"]) {
    color: var(--poodle-color-text-muted);
  }

  .underlay-entity-list__query-variants {
    display: grid;
    grid-column: 1 / -1;
    margin-bottom: 0.125rem;
  }

  .underlay-entity-list__query-variants :global(.poodle-card) {
    --poodle-recipe-card-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 95%,
      var(--poodle-color-text-primary)
    );
    --poodle-recipe-card-hover-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 92%,
      var(--poodle-color-text-primary)
    );
    --poodle-recipe-card-border: color-mix(
      in srgb,
      var(--poodle-color-border-default) 28%,
      transparent
    );
    --poodle-recipe-card-shadow:
      0 0.375rem 0.875rem color-mix(in srgb, black 12%, transparent),
      inset 0 0 0 0.0625rem color-mix(in srgb, var(--poodle-color-border-default) 18%, transparent);
  }

  :global([data-theme="light"]) .underlay-entity-list__query-variants :global(.poodle-card) {
    --poodle-recipe-card-fill: var(--poodle-color-background-elevated);
    --poodle-recipe-card-hover-fill: color-mix(
      in srgb,
      var(--poodle-color-background-elevated) 92%,
      var(--poodle-color-background-surface)
    );
  }
</style>
