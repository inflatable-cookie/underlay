<script lang="ts" module>
	import { exportRowsToCsv } from "./data-table/csv";

	/**
	 * Column configuration for DataTable.
	 */
	export interface DataTableColumn<T = unknown> {
		/** Unique key for this column (used for data access) */
		key: string;
		/** Display label for column header */
		label: string;
		/** Column width (CSS value, e.g., "200px", "1fr", "auto") */
		width?: string;
		/** Minimum column width (CSS value, e.g., "100px"). Used with flexible widths. */
		minWidth?: string;
		/** Enable sorting for this column */
		sortable?: boolean;
		/** Enable filtering for this column */
		filterable?: boolean;
		/** Filter type: 'text' (default), 'select', or 'date' */
		filterType?: "text" | "select" | "date";
		/** Options for select filter type */
		filterOptions?: Array<{ value: string; label: string } | string>;
		/** Custom formatter for cell value */
		formatter?: (value: unknown, row: T) => string;
		/** Text alignment */
		align?: "left" | "center" | "right";
		/** Hide column on mobile */
		hideOnMobile?: boolean;
		/** Whether this column can be hidden via column visibility toggle */
		hideable?: boolean;
	}

	/**
	 * Row action configuration.
	 */
	export interface DataTableAction<T = unknown> {
		/** Action label */
		label: string;
		/** Action icon (optional) */
		icon?: string;
		/** Link href (for navigation actions) */
		href?: string | ((row: T) => string);
		/** Click handler (for button actions) */
		onClick?: (row: T) => void;
		/** Variant for styling */
		variant?: "default" | "danger" | "primary";
		/** Confirmation message before action */
		confirm?: string;
		/** Condition to show this action */
		show?: (row: T) => boolean;
	}

	/**
	 * Pagination state.
	 */
	export interface DataTablePagination {
		page: number;
		limit: number;
		total: number;
	}

	/**
	 * Sort state.
	 */
	export interface DataTableSort {
		key: string;
		direction: "asc" | "desc";
	}

	/**
	 * Filter state.
	 */
	export type DataTableFilters = Record<string, string>;

	/**
	 * Default items per page options.
	 */
	export const DEFAULT_LIMIT_OPTIONS = [10, 20, 50, 100];

	/**
	 * Export data to CSV format.
	 */
	export function exportToCsv<T extends object>(
		data: T[],
		columns: DataTableColumn<T>[],
		filename = "export.csv"
	): void {
		exportRowsToCsv(data, columns, filename);
	}
</script>

<script lang="ts" generics="T extends object">
	import type { Snippet } from "svelte";
	import {
		applySelectAll,
		applySelectRow,
		emitLimitChange,
		emitNextPage,
		getNextFiltersState,
		getNextSortState,
		runRowAction
	} from "./data-table/interactions";
	import {
		toggleHiddenColumn,
	} from "./data-table/state";
	import {
		buildGridColumns,
		getRenderedActionHref,
		getRenderedCellValue,
		getRenderedRowActions
	} from "./data-table/render";
	import Skeleton from "./Skeleton.svelte";
	import Select from "./Select.svelte";
	import ToolbarControls from "./data-table/ToolbarControls.svelte";
	import FilterCell from "./data-table/FilterCell.svelte";
	import RowActionsCell from "./data-table/RowActionsCell.svelte";

	interface Props {
		/** Data rows */
		data?: T[];
		/** Column configurations */
		columns?: DataTableColumn<T>[];
		/** Row actions */
		actions?: DataTableAction<T>[] | ((row: T) => DataTableAction<T>[]);
		/** Pagination state */
		pagination?: DataTablePagination | null;
		/** Current sort state */
		sort?: DataTableSort | null;
		/** Current filter values */
		filters?: DataTableFilters;
		/** Whether data is loading */
		loading?: boolean;
		/** Enable row selection */
		selectable?: boolean;
		/** Currently selected rows */
		selected?: T[];
		/** Message to show when there's no data */
		emptyMessage?: string;
		/** Number of skeleton rows to show when loading */
		loadingRows?: number;
		/** Stick header to top when scrolling */
		stickyHeader?: boolean;
		/** Compact mode (less padding) */
		compact?: boolean;
		/** Striped rows */
		striped?: boolean;
		/** Available items per page options. Set to empty array to hide selector. */
		limitOptions?: number[];
		/** Whether to show the items per page selector */
		showLimitSelector?: boolean;
		/** Whether to show the column visibility toggle */
		showColumnToggle?: boolean;
		/** Whether to show the export to CSV button */
		showExport?: boolean;
		/** Filename for CSV export (without extension) */
		exportFilename?: string;
		/** Callback when sort changes */
		onSort?: (sort: DataTableSort) => void;
		/** Callback when filters change */
		onFilter?: (filters: DataTableFilters) => void;
		/** Callback when page changes */
		onPage?: (page: number) => void;
		/** Callback when limit changes */
		onLimit?: (limit: number) => void;
		/** Callback when selection changes */
		onSelect?: (selected: T[]) => void;
		/** Callback when an action is triggered */
		onAction?: (event: { action: string; row: T }) => void;
		/** Callback when export is triggered */
		onExport?: (event: { data: T[]; columns: DataTableColumn<T>[] }) => void;
		/** Callback when a row is clicked */
		onRowClick?: (row: T) => void;
		/** Snippet for toolbar left area */
		toolbarLeft?: Snippet;
		/** Snippet for toolbar right area */
		toolbarRight?: Snippet;
		/** Snippet for empty state */
		empty?: Snippet;
		/** Snippet for custom cell rendering */
		cell?: Snippet<[{ column: DataTableColumn<T>; row: T; value: string }]>;
		/** Snippet for extended row content */
		extendedRow?: Snippet<[{ row: T }]>;
		/** Condition to show the extended row */
		extendedRowWhen?: (row: T) => boolean;
	}

	let {
		data = [],
		columns = [],
		actions = [],
		pagination = null,
		sort = null,
		filters = {},
		loading = false,
		selectable = false,
		selected = $bindable([]),
		emptyMessage = "No data available",
		loadingRows = 5,
		stickyHeader = false,
		compact = false,
		striped = false,
		limitOptions = DEFAULT_LIMIT_OPTIONS,
		showLimitSelector = true,
		showColumnToggle = false,
		showExport = false,
		exportFilename = "export",
		onSort,
		onFilter,
		onPage,
		onLimit,
		onSelect,
		onAction,
		onExport,
		onRowClick,
		toolbarLeft,
		toolbarRight,
		empty,
		cell,
		extendedRow,
		extendedRowWhen = () => true
	}: Props = $props();

	// Internal state
	let internalFilters = $state<DataTableFilters>({});
	let hiddenColumns = $state<Set<string>>(new Set());
	let showColumnMenu = $state(false);

	// Sync internal filters when prop changes
	$effect(() => {
		internalFilters = { ...filters };
	});

	// Computed - visible columns (excluding hidden ones)
	let visibleColumns = $derived(columns.filter((col) => !hiddenColumns.has(col.key)));
	let hideableColumns = $derived(columns.filter((col) => col.hideable !== false));

	// Computed
	let totalPages = $derived(pagination ? Math.ceil(pagination.total / pagination.limit) : 1);
	let allSelected = $derived(data.length > 0 && selected.length === data.length);
	let someSelected = $derived(selected.length > 0 && selected.length < data.length);

	// Handle sort
	function handleSort(column: DataTableColumn<T>) {
		if (!column.sortable) return;
		const newSort: DataTableSort = getNextSortState(sort, column);
		onSort?.(newSort);
	}

	// Handle filter change
	function handleFilterChange(key: string, value: string) {
		internalFilters = getNextFiltersState(internalFilters, key, value);
		onFilter?.(internalFilters);
	}

	// Handle page change
	function handlePageChange(newPage: number) {
		emitNextPage(newPage, totalPages, onPage);
	}

	// Handle limit change
	function handleLimitChange(newLimit: number) {
		emitLimitChange(newLimit, onLimit, onPage);
	}

	// Handle column visibility toggle
	function toggleColumn(key: string) {
		hiddenColumns = toggleHiddenColumn(hiddenColumns, key);
	}

	// Handle export to CSV
	function handleExport() {
		onExport?.({ data, columns: visibleColumns });
		exportToCsv(data, visibleColumns, `${exportFilename}.csv`);
	}

	// Handle selection
	function handleSelectAll() {
		selected = applySelectAll(data, selected, allSelected, onSelect);
	}

	function handleSelectRow(row: T) {
		selected = applySelectRow(selected, row, onSelect);
	}

	// Handle action click
	function handleActionClick(action: DataTableAction<T>, row: T) {
		runRowAction(action, row, onAction, confirm);
	}

	let gridColumns = $derived(
		buildGridColumns(selectable, visibleColumns, actions)
	);
</script>

<div class="underlay-data-table-wrapper">
<div
	class="underlay-data-table"
	class:compact
	class:striped
	class:sticky-header={stickyHeader}
	style:--grid-columns={gridColumns}
>
	<!-- Toolbar -->
	{#if showColumnToggle || showExport}
		<div class="table-toolbar">
			<div class="toolbar-left">
				{@render toolbarLeft?.()}
			</div>
			<div class="toolbar-right">
				<ToolbarControls
					{showColumnToggle}
					{hideableColumns}
					{hiddenColumns}
					{showColumnMenu}
					{showExport}
					dataLength={data.length}
					onToggleColumnMenu={() => (showColumnMenu = !showColumnMenu)}
					onToggleColumn={toggleColumn}
					onExport={handleExport}
				/>
				{@render toolbarRight?.()}
			</div>
		</div>
	{/if}

	<!-- Header -->
	<div class="table-header" role="rowgroup">
		<div class="table-row header-row" role="row">
			{#if selectable}
				<div class="table-cell checkbox-cell" role="columnheader">
					<input
						type="checkbox"
						checked={allSelected}
						indeterminate={someSelected}
						onchange={handleSelectAll}
						aria-label="Select all rows"
					/>
				</div>
			{/if}

			{#each visibleColumns as column}
				<div
					class="table-cell header-cell"
					class:sortable={column.sortable}
					class:hide-mobile={column.hideOnMobile}
					class:align-center={column.align === "center"}
					class:align-right={column.align === "right"}
					role="columnheader"
					aria-sort={sort?.key === column.key ? (sort.direction === "asc" ? "ascending" : "descending") : undefined}
				>
					{#if column.sortable}
						<button type="button" class="sort-button" onclick={() => handleSort(column)}>
							<span>{column.label}</span>
							<span class="sort-icon" class:active={sort?.key === column.key}>
								{#if sort?.key === column.key}
									{sort.direction === "asc" ? "↑" : "↓"}
								{:else}
									↕
								{/if}
							</span>
						</button>
					{:else}
						{column.label}
					{/if}
				</div>
			{/each}

			{#if actions.length > 0 || typeof actions === "function"}
				<div class="table-cell header-cell actions-header" role="columnheader">
					<span class="sr-only">Actions</span>
				</div>
			{/if}
		</div>

		<!-- Filter row -->
		{#if visibleColumns.some((c) => c.filterable)}
			<div class="table-row filter-row" role="row">
				{#if selectable}
					<div class="table-cell" role="cell"></div>
				{/if}

				{#each visibleColumns as column}
					<div class="table-cell filter-cell" class:hide-mobile={column.hideOnMobile} role="cell">
						<FilterCell
							{column}
							value={internalFilters[column.key] ?? ""}
							onChange={(value) => handleFilterChange(column.key, value)}
						/>
					</div>
				{/each}

				{#if actions.length > 0 || typeof actions === "function"}
					<div class="table-cell" role="cell"></div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Body -->
	<div class="table-body" role="rowgroup">
		{#if loading}
			{#each Array(loadingRows) as _, i}
				<div class="table-row" role="row">
					{#if selectable}
						<div class="table-cell checkbox-cell" role="cell">
							<Skeleton variant="button" width="16px" height="16px" />
						</div>
					{/if}
					{#each visibleColumns as column}
						<div class="table-cell" class:hide-mobile={column.hideOnMobile} role="cell">
							<Skeleton variant="text" />
						</div>
					{/each}
					{#if actions.length > 0 || typeof actions === "function"}
						<div class="table-cell actions-cell" role="cell">
							<Skeleton variant="button" width="24px" height="24px" />
						</div>
					{/if}
				</div>
			{/each}
		{:else if data.length === 0}
			<div class="empty-state" role="row">
				{#if empty}
					{@render empty()}
				{:else}
					<p>{emptyMessage}</p>
				{/if}
			</div>
		{:else}
			{#each data as row, rowIndex}
				{@const rowActions = getRenderedRowActions(row, actions)}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<div
					class="table-row"
					class:selected={selected.includes(row)}
					class:has-extended={!!extendedRow && extendedRowWhen(row)}
					class:clickable={!!onRowClick}
					role="row"
					tabindex={onRowClick ? 0 : undefined}
					onclick={() => onRowClick?.(row)}
				>
					{#if selectable}
						<div class="table-cell checkbox-cell" role="cell">
							<input
								type="checkbox"
								checked={selected.includes(row)}
								onchange={() => handleSelectRow(row)}
								aria-label={`Select row ${rowIndex + 1}`}
							/>
						</div>
					{/if}

					{#each visibleColumns as column}
						<div
							class="table-cell"
							class:hide-mobile={column.hideOnMobile}
							class:align-center={column.align === "center"}
							class:align-right={column.align === "right"}
							role="cell"
						>
							{#if cell}
								{@render cell({ column, row, value: getRenderedCellValue(row, column) })}
							{:else}
								{getRenderedCellValue(row, column)}
							{/if}
						</div>
					{/each}

					{#if actions.length > 0 || typeof actions === "function"}
						<div class="table-cell actions-cell" role="cell">
							<RowActionsCell
								{row}
								{rowActions}
								getActionHref={getRenderedActionHref}
								onActionClick={handleActionClick}
							/>
						</div>
					{/if}
				</div>
				{#if extendedRow && extendedRowWhen(row)}
					<div class="table-row table-row--extended" role="row">
						<div class="table-cell table-cell--extended" role="cell">
							{@render extendedRow({ row })}
						</div>
					</div>
				{/if}
			{/each}
		{/if}
	</div>

	<!-- Pagination -->
	{#if pagination && (totalPages > 1 || showLimitSelector)}
		<div class="table-footer">
			<div class="pagination-info">
				Showing {(pagination.page - 1) * pagination.limit + 1} to {Math.min(pagination.page * pagination.limit, pagination.total)}
				of {pagination.total}
			</div>

			<div class="pagination-right">
				{#if showLimitSelector && limitOptions.length > 0}
					<div class="limit-selector">
						<span>Show</span>
						<Select
							value={String(pagination.limit)}
							onchange={(value) => handleLimitChange(Number(value))}
							items={limitOptions.map((opt) => ({ value: String(opt), label: String(opt) }))}
						/>
						<span>per page</span>
					</div>
				{/if}

				{#if totalPages > 1}
					<div class="pagination-controls">
						<button
							type="button"
							class="pagination-button"
							disabled={pagination.page <= 1}
							onclick={() => handlePageChange(1)}
							aria-label="First page"
						>
							««
						</button>
						<button
							type="button"
							class="pagination-button"
							disabled={pagination.page <= 1}
							onclick={() => handlePageChange(pagination.page - 1)}
							aria-label="Previous page"
						>
							«
						</button>

						<span class="pagination-page">
							Page {pagination.page} of {totalPages}
						</span>

						<button
							type="button"
							class="pagination-button"
							disabled={pagination.page >= totalPages}
							onclick={() => handlePageChange(pagination.page + 1)}
							aria-label="Next page"
						>
							»
						</button>
						<button
							type="button"
							class="pagination-button"
							disabled={pagination.page >= totalPages}
							onclick={() => handlePageChange(totalPages)}
							aria-label="Last page"
						>
							»»
						</button>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
</div>

<style>
	.underlay-data-table-wrapper {
		overflow-x: auto;
		border-radius: var(--radius-lg, 0.5rem);
	}

	.underlay-data-table {
		--dt-border: var(--underlay-table-border, 1px solid var(--color-border, #e2e8f0));
		--dt-header-bg: var(--underlay-table-header-bg, var(--color-surface-subtle, #f8fafc));
		--dt-row-hover: var(--underlay-table-row-hover, var(--color-surface-hover, #f1f5f9));
		--dt-row-selected: var(--underlay-table-row-selected, var(--color-primary-subtle, #eff6ff));
		--dt-stripe: var(--underlay-table-stripe, var(--color-surface-subtle, #f8fafc));
		--dt-gap: var(--underlay-table-gap, 0.75rem);
		--dt-gap-compact: var(--underlay-table-gap-compact, 0.5rem);

		display: grid;
		grid-template-columns: var(--grid-columns);
		border: var(--dt-border);
		border-radius: var(--radius-lg, 0.5rem);
		font-size: 0.8rem;
		min-width: fit-content;
	}

	.table-toolbar {
		grid-column: 1 / -1;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		background: var(--dt-header-bg);
		border-bottom: var(--dt-border);
		gap: 0.5rem;
	}

	.toolbar-left,
	.toolbar-right {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.table-header,
	.table-body {
		display: contents;
	}

	.table-row {
		display: contents;
	}

	/* Row styling applied to cells since rows use display:contents */
	.header-row > .table-cell {
		background: var(--dt-header-bg);
		font-weight: 600;
		border-bottom: var(--dt-border);
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--underlay-color-text-muted, var(--color-text-muted, #64748b));
	}

	.filter-row > .table-cell {
		background: var(--dt-header-bg);
		border-bottom: var(--dt-border);
	}

	/* Data rows - border on cells */
	.table-body > .table-row > .table-cell {
		border-bottom: var(--dt-border);
	}

	.table-body > .table-row.has-extended > .table-cell {
		border-bottom: none;
	}

	.table-body > .table-row:last-child > .table-cell,
	.table-body > .table-row:last-of-type > .table-cell,
	.table-body > .table-row--extended:last-child > .table-cell {
		border-bottom: none;
	}

	/* Hover state for data rows using :has() */
	.table-body > .table-row:hover > .table-cell {
		background: var(--dt-row-hover);
	}

	/* Clickable row style */
	.table-body > .table-row.clickable {
		cursor: pointer;
	}

	.table-body > .table-row.selected > .table-cell {
		background: var(--dt-row-selected);
	}

	/* Striped rows */
	.striped .table-body > .table-row:nth-child(even) > .table-cell {
		background: var(--dt-stripe);
	}

	.striped .table-body > .table-row:nth-child(even):hover > .table-cell {
		background: var(--dt-row-hover);
	}

	.table-cell {
		padding: var(--dt-gap);
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		display: flex;
		align-items: center;
	}

	.table-row--extended > .table-cell {
		grid-column: 1 / -1;
	}

	.table-cell--extended {
		white-space: normal;
		align-items: flex-start;
	}

	/* Ensure common elements are vertically centered even when nested content affects layout */
	.table-cell > :global(*) {
		align-self: center;
	}

	.compact .table-cell {
		padding: var(--dt-gap-compact);
	}

	.checkbox-cell {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.header-cell {
		font-weight: 600;
	}

	.align-center {
		text-align: center;
		justify-content: center;
	}

	.align-right {
		text-align: right;
		justify-content: flex-end;
	}

	.sort-button {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		background: none;
		border: none;
		padding: 0;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		color: inherit;
	}

	.sort-button:hover {
		color: var(--color-primary, #3b82f6);
	}

	.sort-icon {
		opacity: 0.4;
		font-size: 0.75em;
	}

	.sort-icon.active {
		opacity: 1;
		color: var(--color-primary, #3b82f6);
	}

	.filter-cell :global(.underlay-input),
	.filter-cell :global(.underlay-input-wrapper) {
		width: 100%;
		font-size: inherit;
	}

	.filter-cell :global(.underlay-input) {
		padding: 0.25rem 0.5rem;
	}

	.filter-cell :global(.underlay-select-trigger) {
		min-width: 0;
		padding: 0.25rem 0.5rem;
		font-size: inherit;
	}

	.actions-cell {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 0.5rem;
	}

	.empty-state {
		padding: 3rem;
		text-align: center;
		color: var(--color-text-muted, #64748b);
		grid-column: 1 / -1;
	}

	.table-footer {
		grid-column: 1 / -1;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--dt-gap);
		background: var(--dt-header-bg);
		border-top: var(--dt-border);
	}

	.pagination-info {
		color: var(--color-text-muted, #64748b);
	}

	.pagination-right {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	.limit-selector {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--color-text-muted, #64748b);
	}

	.limit-selector :global(.underlay-select-trigger) {
		min-width: 4.5rem;
		padding: 0.25rem 0.5rem;
		font-size: inherit;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.pagination-button {
		padding: 0.25rem 0.5rem;
		border: 1px solid var(--color-border, #e2e8f0);
		border-radius: var(--radius-sm, 0.25rem);
		background: var(--color-surface, #fff);
		cursor: pointer;
		font-size: inherit;
	}

	.pagination-button:hover:not(:disabled) {
		background: var(--dt-row-hover);
	}

	.pagination-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-page {
		padding: 0 0.5rem;
	}

	.sticky-header .header-row {
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	@media (max-width: 900px) {
		.hide-mobile {
			display: none;
		}

		.table-footer {
			flex-direction: column;
			gap: 0.75rem;
			align-items: stretch;
		}

		.pagination-right {
			flex-direction: column;
			gap: 0.75rem;
		}

		.limit-selector {
			justify-content: center;
		}

		.pagination-controls {
			justify-content: center;
		}
	}
</style>
