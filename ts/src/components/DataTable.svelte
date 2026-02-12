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
	import { buildGridColumns } from "./data-table/render";
	import {
		getHideableColumns,
		getTotalPages,
		getVisibleColumns,
		isAllSelected,
		isSomeSelected
	} from "./data-table/view";
	import TableBody from "./data-table/TableBody.svelte";
	import TableHeader from "./data-table/TableHeader.svelte";
	import TableToolbar from "./data-table/TableToolbar.svelte";
	import PaginationFooter from "./data-table/PaginationFooter.svelte";

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
	let visibleColumns = $derived(getVisibleColumns(columns, hiddenColumns));
	let hideableColumns = $derived(getHideableColumns(columns));

	// Computed
	let hasActions = $derived(actions.length > 0 || typeof actions === "function");
	let totalPages = $derived(getTotalPages(pagination));
	let allSelected = $derived(isAllSelected(data.length, selected.length));
	let someSelected = $derived(isSomeSelected(data.length, selected.length));

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
	class:underlay-compact={compact}
	class:underlay-striped={striped}
	class:underlay-sticky-header={stickyHeader}
	style:--grid-columns={gridColumns}
>
	<!-- Toolbar -->
	{#if showColumnToggle || showExport}
		<TableToolbar
			{showColumnToggle}
			{hideableColumns}
			{hiddenColumns}
			{showColumnMenu}
			{showExport}
			dataLength={data.length}
			{toolbarLeft}
			{toolbarRight}
			onToggleColumnMenu={() => (showColumnMenu = !showColumnMenu)}
			onToggleColumn={toggleColumn}
			onExport={handleExport}
		/>
	{/if}

	<TableHeader
		{stickyHeader}
		{selectable}
		{allSelected}
		{someSelected}
		{visibleColumns}
		{hasActions}
		{sort}
		{internalFilters}
		onSort={handleSort}
		onSelectAll={handleSelectAll}
		onFilterChange={handleFilterChange}
	/>

	<TableBody
		{data}
		{actions}
		{loading}
		{loadingRows}
		{selectable}
		{visibleColumns}
		{hasActions}
		{selected}
		{emptyMessage}
		{onRowClick}
		onSelectRow={handleSelectRow}
		onActionClick={handleActionClick}
		{empty}
		{cell}
		{extendedRow}
		{extendedRowWhen}
	/>

	<!-- Pagination -->
	{#if pagination && (totalPages > 1 || showLimitSelector)}
		<PaginationFooter
			page={pagination.page}
			limit={pagination.limit}
			total={pagination.total}
			{totalPages}
			{showLimitSelector}
			{limitOptions}
			onLimitChange={handleLimitChange}
			onPageChange={handlePageChange}
		/>
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

</style>
