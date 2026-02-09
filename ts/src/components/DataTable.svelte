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
		getRowActionHref,
		getVisibleRowActions
	} from "./data-table/actions";
	import Skeleton from "./Skeleton.svelte";
	import DropdownMenu from "./DropdownMenu.svelte";
	import Select from "./Select.svelte";
	import TextInput from "./TextInput.svelte";
	import DateInput from "./DateInput.svelte";

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

	// Get cell value from row
	function getCellValue(row: T, column: DataTableColumn<T>): string {
		const keys = column.key.split(".");
		let value: unknown = row;
		for (const key of keys) {
			if (value && typeof value === "object") {
				value = (value as Record<string, unknown>)[key];
			} else {
				value = undefined;
				break;
			}
		}

		if (column.formatter) {
			return column.formatter(value, row);
		}

		if (value == null) return "";
		if (value instanceof Date) return value.toLocaleDateString();
		return String(value);
	}

	// Get actions for a row
	function getRowActions(row: T): DataTableAction<T>[] {
		return getVisibleRowActions(row, actions) as DataTableAction<T>[];
	}

	// Get action href
	function getActionHref(action: DataTableAction<T>, row: T): string | undefined {
		return getRowActionHref(action, row);
	}

	// Handle sort
	function handleSort(column: DataTableColumn<T>) {
		if (!column.sortable) return;

		const newSort: DataTableSort = {
			key: column.key,
			direction: sort?.key === column.key && sort.direction === "asc" ? "desc" : "asc"
		};

		onSort?.(newSort);
	}

	// Handle filter change
	function handleFilterChange(key: string, value: string) {
		internalFilters = { ...internalFilters, [key]: value };
		onFilter?.(internalFilters);
	}

	// Handle page change
	function handlePageChange(newPage: number) {
		if (newPage < 1 || newPage > totalPages) return;
		onPage?.(newPage);
	}

	// Handle limit change
	function handleLimitChange(newLimit: number) {
		onLimit?.(newLimit);
		// Reset to first page when changing limit to avoid being on an invalid page
		onPage?.(1);
	}

	// Handle column visibility toggle
	function toggleColumn(key: string) {
		if (hiddenColumns.has(key)) {
			hiddenColumns.delete(key);
		} else {
			hiddenColumns.add(key);
		}
		hiddenColumns = hiddenColumns; // Trigger reactivity
	}

	// Handle export to CSV
	function handleExport() {
		onExport?.({ data, columns: visibleColumns });
		exportToCsv(data, visibleColumns, `${exportFilename}.csv`);
	}

	// Handle selection
	function handleSelectAll() {
		if (allSelected) {
			selected = [];
		} else {
			selected = [...data];
		}
		onSelect?.(selected);
	}

	function handleSelectRow(row: T) {
		const index = selected.indexOf(row);
		if (index >= 0) {
			selected = [...selected.slice(0, index), ...selected.slice(index + 1)];
		} else {
			selected = [...selected, row];
		}
		onSelect?.(selected);
	}

	// Handle action click
	function handleActionClick(action: DataTableAction<T>, row: T) {
		if (action.confirm) {
			if (!confirm(action.confirm)) return;
		}

		if (action.onClick) {
			action.onClick(row);
		}

		onAction?.({ action: action.label, row });
	}

	// Grid template columns
	function getColumnWidth(col: DataTableColumn<T>): string {
		// If explicit width is set, use it (but wrap in minmax if minWidth is also set)
		if (col.width) {
			if (col.minWidth) {
				return `minmax(${col.minWidth}, ${col.width})`;
			}
			return col.width;
		}
		// Default: flexible column with minimum width
		const minWidth = col.minWidth ?? "100px";
		return `minmax(${minWidth}, 1fr)`;
	}

	let gridColumns = $derived(
		[
			selectable ? "40px" : null,
			...visibleColumns.map(getColumnWidth),
			actions.length > 0 || typeof actions === "function" ? "80px" : null
		]
			.filter(Boolean)
			.join(" ")
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
				{#if showColumnToggle && hideableColumns.length > 0}
					<div class="column-toggle">
						<button
							type="button"
							class="toolbar-button"
							onclick={() => (showColumnMenu = !showColumnMenu)}
							aria-expanded={showColumnMenu}
							aria-haspopup="true"
						>
							<span class="toolbar-icon">☰</span>
							Columns
						</button>
						{#if showColumnMenu}
							<div class="column-menu" role="menu">
								{#each hideableColumns as column}
									<label class="column-menu-item">
										<input
											type="checkbox"
											checked={!hiddenColumns.has(column.key)}
											onchange={() => toggleColumn(column.key)}
										/>
										{column.label}
									</label>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
				{#if showExport}
					<button type="button" class="toolbar-button" onclick={handleExport} disabled={data.length === 0}>
						<span class="toolbar-icon">↓</span>
						Export CSV
					</button>
				{/if}
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
						{#if column.filterable}
							{#if column.filterType === "select" && column.filterOptions}
								<Select
									value={internalFilters[column.key] ?? ""}
									onchange={(value) => handleFilterChange(column.key, value)}
									placeholder="All"
									items={[
										{ value: "", label: "All" },
										...column.filterOptions.map((opt) =>
											typeof opt === "string" ? { value: opt, label: opt } : opt
										)
									]}
								/>
							{:else if column.filterType === "date"}
								<DateInput
									value={internalFilters[column.key] ?? ""}
									onchange={(value) => handleFilterChange(column.key, value)}
								/>
							{:else}
								<TextInput
									search
									placeholder={`Filter ${column.label.toLowerCase()}...`}
									value={internalFilters[column.key] ?? ""}
									debounce={300}
									onchange={(value) => handleFilterChange(column.key, value)}
								/>
							{/if}
						{/if}
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
								{@render cell({ column, row, value: getCellValue(row, column) })}
							{:else}
								{getCellValue(row, column)}
							{/if}
						</div>
					{/each}

					{#if actions.length > 0 || typeof actions === "function"}
						<div class="table-cell actions-cell" role="cell">
							{#if getRowActions(row).length === 1}
								{#each [getRowActions(row)[0]] as action}
									{#if action.href}
										<a href={getActionHref(action, row)} class="action-link">{action.label}</a>
									{:else}
										<button type="button" class="action-button" onclick={() => handleActionClick(action, row)}>
											{action.label}
										</button>
									{/if}
								{/each}
							{:else if getRowActions(row).length > 1}
								<DropdownMenu>
								{#snippet trigger()}
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="16"
										height="16"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
										aria-hidden="true"
									>
										<circle cx="12" cy="12" r="1" />
										<circle cx="12" cy="5" r="1" />
										<circle cx="12" cy="19" r="1" />
									</svg>
								{/snippet}
									{#each getRowActions(row) as action}
										{#if action.href}
											<a href={getActionHref(action, row)} class="menu-item">{action.label}</a>
										{:else}
											<button
												type="button"
												class="menu-item"
												class:danger={action.variant === "danger"}
												onclick={() => handleActionClick(action, row)}
											>
												{action.label}
											</button>
										{/if}
									{/each}
								</DropdownMenu>
							{/if}
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

	.toolbar-button {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		font-size: inherit;
		font-weight: 500;
		color: var(--color-text, #1e293b);
		background: var(--color-surface, #fff);
		border: 1px solid var(--color-border, #e2e8f0);
		border-radius: var(--radius-md, 0.375rem);
		cursor: pointer;
		transition: background-color 0.15s, border-color 0.15s;
	}

	.toolbar-button:hover:not(:disabled) {
		background: var(--dt-row-hover);
		border-color: var(--color-border-hover, #cbd5e1);
	}

	.toolbar-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.toolbar-icon {
		font-size: inherit;
	}

	.column-toggle {
		position: relative;
	}

	.column-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 0.25rem;
		min-width: 12rem;
		background: var(--color-surface, #fff);
		border: 1px solid var(--color-border, #e2e8f0);
		border-radius: var(--radius-md, 0.375rem);
		box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
		z-index: 50;
		max-height: 20rem;
		overflow-y: auto;
	}

	.column-menu-item {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		font-size: inherit;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.column-menu-item:hover {
		background: var(--dt-row-hover);
	}

	.column-menu-item input {
		cursor: pointer;
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

	.action-link,
	.action-button {
		font-size: inherit;
		color: var(--color-primary, #3b82f6);
		text-decoration: none;
		background: none;
		border: none;
		padding: 0.25rem 0.5rem;
		cursor: pointer;
		border-radius: var(--radius-sm, 0.25rem);
	}

	.action-link:hover,
	.action-button:hover {
		background: var(--dt-row-hover);
	}

	.menu-item {
		display: block;
		width: 100%;
		padding: 0.5rem 1rem;
		text-align: left;
		background: none;
		border: none;
		cursor: pointer;
		font: inherit;
		color: inherit;
		text-decoration: none;
	}

	.menu-item:hover {
		background: var(--dt-row-hover);
	}

	.menu-item.danger {
		color: var(--color-danger, #ef4444);
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
