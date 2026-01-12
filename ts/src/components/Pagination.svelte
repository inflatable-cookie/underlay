<script lang="ts" context="module">
	/** Pagination state passed to the component */
	export interface PaginationState {
		/** Current page (1-based) */
		page: number;
		/** Items per page */
		limit: number;
		/** Total number of items */
		total: number;
	}
</script>

<script lang="ts">
	import { createEventDispatcher } from "svelte";

	/**
	 * Standalone Pagination component for navigating through pages of content.
	 *
	 * @example Basic usage
	 * ```svelte
	 * <Pagination
	 *   page={1}
	 *   limit={20}
	 *   total={100}
	 *   on:page={(e) => goto(`?page=${e.detail}`)}
	 * />
	 * ```
	 *
	 * @example With limit selector
	 * ```svelte
	 * <Pagination
	 *   page={currentPage}
	 *   limit={itemsPerPage}
	 *   total={totalItems}
	 *   showLimitSelector
	 *   on:page={handlePageChange}
	 *   on:limit={handleLimitChange}
	 * />
	 * ```
	 */

	const dispatch = createEventDispatcher<{
		page: number;
		limit: number;
	}>();

	/** Current page (1-based) */
	export let page: number = 1;

	/** Items per page */
	export let limit: number = 20;

	/** Total number of items */
	export let total: number = 0;

	/** Show the items-per-page selector */
	export let showLimitSelector: boolean = false;

	/** Available limit options */
	export let limitOptions: number[] = [10, 20, 50, 100];

	/** Show "Showing X to Y of Z" info text */
	export let showInfo: boolean = true;

	/** Compact mode (smaller padding, no info text on mobile) */
	export let compact: boolean = false;

	/** Additional CSS class */
	export let className: string = "";

	$: totalPages = Math.ceil(total / limit);
	$: startItem = total === 0 ? 0 : (page - 1) * limit + 1;
	$: endItem = Math.min(page * limit, total);

	function handlePageChange(newPage: number) {
		if (newPage >= 1 && newPage <= totalPages && newPage !== page) {
			dispatch("page", newPage);
		}
	}

	function handleLimitChange(event: Event) {
		const newLimit = Number((event.target as HTMLSelectElement).value);
		dispatch("limit", newLimit);
	}
</script>

{#if totalPages > 1 || showLimitSelector}
	<nav
		class="underlay-pagination {className}"
		class:compact
		aria-label="Pagination"
		{...$$restProps}
	>
		{#if showInfo && total > 0}
			<div class="pagination-info">
				Showing {startItem} to {endItem} of {total}
			</div>
		{/if}

		<div class="pagination-controls-wrapper">
			{#if showLimitSelector && limitOptions.length > 0}
				<div class="limit-selector">
					<label for="pagination-limit">Show</label>
					<select
						id="pagination-limit"
						value={limit}
						on:change={handleLimitChange}
					>
						{#each limitOptions as option}
							<option value={option}>{option}</option>
						{/each}
					</select>
					<span>per page</span>
				</div>
			{/if}

			{#if totalPages > 1}
				<div class="pagination-controls">
					<button
						type="button"
						class="pagination-button"
						disabled={page <= 1}
						on:click={() => handlePageChange(1)}
						aria-label="First page"
					>
						««
					</button>
					<button
						type="button"
						class="pagination-button"
						disabled={page <= 1}
						on:click={() => handlePageChange(page - 1)}
						aria-label="Previous page"
					>
						«
					</button>

					<span class="pagination-page">
						Page {page} of {totalPages}
					</span>

					<button
						type="button"
						class="pagination-button"
						disabled={page >= totalPages}
						on:click={() => handlePageChange(page + 1)}
						aria-label="Next page"
					>
						»
					</button>
					<button
						type="button"
						class="pagination-button"
						disabled={page >= totalPages}
						on:click={() => handlePageChange(totalPages)}
						aria-label="Last page"
					>
						»»
					</button>
				</div>
			{/if}
		</div>
	</nav>
{/if}

<style>
	.underlay-pagination {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		background: var(--underlay-pagination-bg, var(--color-surface-subtle, #f8fafc));
		border-top: 1px solid var(--underlay-pagination-border, var(--color-border, #e2e8f0));
	}

	.underlay-pagination.compact {
		padding: 0.5rem 0.75rem;
		gap: 0.75rem;
	}

	.pagination-info {
		color: var(--underlay-color-text-muted, #64748b);
	}

	.pagination-controls-wrapper {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.limit-selector {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: var(--underlay-color-text-muted, #64748b);
	}

	.limit-selector select {
		padding: 0.25rem 0.5rem;
		font-size: 0.875rem;
		border: 1px solid var(--color-border, #e2e8f0);
		border-radius: var(--radius-sm, 0.25rem);
		background: var(--color-surface, #fff);
		color: var(--underlay-color-text, #1e293b);
		cursor: pointer;
	}

	.limit-selector select:focus {
		outline: 2px solid var(--underlay-color-primary, #2563eb);
		outline-offset: 1px;
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.pagination-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 2rem;
		padding: 0.375rem 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--underlay-color-text, #1e293b);
		background: var(--color-surface, #fff);
		border: 1px solid var(--color-border, #e2e8f0);
		border-radius: var(--radius-sm, 0.25rem);
		cursor: pointer;
		transition:
			background-color 0.15s,
			border-color 0.15s,
			color 0.15s;
	}

	.pagination-button:hover:not(:disabled) {
		background: var(--color-surface-hover, #f1f5f9);
		border-color: var(--color-border-hover, #cbd5e1);
	}

	.pagination-button:focus-visible {
		outline: 2px solid var(--underlay-color-primary, #2563eb);
		outline-offset: 1px;
	}

	.pagination-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-page {
		padding: 0 0.5rem;
		color: var(--underlay-color-text-muted, #64748b);
		white-space: nowrap;
	}

	/* Mobile: stack layout */
	@media (max-width: 640px) {
		.underlay-pagination {
			flex-direction: column;
			align-items: stretch;
		}

		.underlay-pagination.compact .pagination-info {
			display: none;
		}

		.pagination-controls-wrapper {
			justify-content: center;
			flex-wrap: wrap;
		}
	}

	/* Dark mode */
	:global([data-theme="dark"]) .underlay-pagination {
		background: var(--underlay-pagination-bg-dark, rgba(30, 41, 59, 0.5));
		border-color: var(--underlay-pagination-border-dark, rgba(148, 163, 184, 0.2));
	}

	:global([data-theme="dark"]) .pagination-button {
		background: var(--color-surface-dark, #1e293b);
		border-color: var(--color-border-dark, #334155);
		color: var(--underlay-color-text-dark, #f1f5f9);
	}

	:global([data-theme="dark"]) .pagination-button:hover:not(:disabled) {
		background: var(--color-surface-hover-dark, #334155);
	}

	:global([data-theme="dark"]) .limit-selector select {
		background: var(--color-surface-dark, #1e293b);
		border-color: var(--color-border-dark, #334155);
		color: var(--underlay-color-text-dark, #f1f5f9);
	}
</style>
