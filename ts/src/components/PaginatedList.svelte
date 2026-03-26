<script lang="ts" module>
	import type { PaginationController } from "../patterns/pagination-types";
</script>

<script lang="ts" generics="T">
	import type { Snippet } from "svelte";
	import { Callout } from "@poodle/svelte-primitives";
	import PageLoading from "./PageLoading.svelte";
	import Pagination from "./Pagination.svelte";

	/**
	 * A wrapper component that combines list rendering with pagination,
	 * loading states, error handling, and empty states.
	 *
	 * @example Basic usage
	 * ```svelte
	 * <PaginatedList controller={pagination}>
	 *   {#snippet items(item)}
	 *     <ListCard title={item.name} />
	 *   {/snippet}
	 * </PaginatedList>
	 * ```
	 *
	 * @example With custom empty state
	 * ```svelte
	 * <PaginatedList controller={pagination}>
	 *   {#snippet items(item)}
	 *     <ListCard title={item.name} />
	 *   {/snippet}
	 *   {#snippet empty()}
	 *     <EmptyState icon={FileQuestion} message="No documents found" />
	 *   {/snippet}
	 * </PaginatedList>
	 * ```
	 *
	 * @example With grid layout
	 * ```svelte
	 * <PaginatedList controller={pagination} layout="grid">
	 *   {#snippet items(item)}
	 *     <ListCard title={item.name} />
	 *   {/snippet}
	 * </PaginatedList>
	 * ```
	 */

	interface Props {
		/** Pagination controller from createPaginationController or createClientPagination */
		controller: PaginationController<T>;
		/** Render each item */
		items: Snippet<[item: T, index: number]>;
		/** Custom empty state content (optional) */
		empty?: Snippet;
		/** Custom loading content (optional) */
		loading?: Snippet;
		/** Custom error content (optional) */
		error?: Snippet<[message: string]>;
		/** Layout mode for items */
		layout?: "list" | "grid";
		/** Grid minimum item width in rem (only for grid layout) */
		gridMinWidth?: number;
		/** Gap between items in rem */
		gap?: number;
		/** Loading message (used if loading snippet not provided) */
		loadingMessage?: string;
		/** Empty message (used if empty snippet not provided) */
		emptyMessage?: string;
		/** Show pagination controls */
		showPagination?: boolean;
		/** Pagination variant */
		paginationVariant?: "full" | "simple";
		/** Show page size selector in pagination */
		showLimitSelector?: boolean;
		/** Additional CSS class for the container */
		className?: string;
	}

	let {
		controller,
		items,
		empty,
		loading,
		error,
		layout = "list",
		gridMinWidth = 20,
		gap = 1,
		loadingMessage = "Loading...",
		emptyMessage = "No items found.",
		showPagination = true,
		paginationVariant = "simple",
		showLimitSelector = false,
		className = ""
	}: Props = $props();

	const hasItems = $derived(controller.items.length > 0);
	const showPaginationControls = $derived(
		showPagination && hasItems && (controller.hasNextPage || controller.hasPrevPage || controller.currentPage > 1)
	);
</script>

{#if controller.loading}
	{#if loading}
		{@render loading()}
	{:else}
		<PageLoading message={loadingMessage} />
	{/if}
{:else if controller.error}
	{#if error}
		{@render error(controller.error)}
	{:else}
		<Callout tone="danger" message={controller.error} announceMode="polite" />
	{/if}
{:else if !hasItems}
	{#if empty}
		{@render empty()}
	{:else}
		<p class="underlay-paginated-list__empty">{emptyMessage}</p>
	{/if}
{:else}
	<div
		class="underlay-paginated-list {className}"
		class:underlay-paginated-list--grid={layout === "grid"}
		style:--paginated-list-gap="{gap}rem"
		style:--paginated-list-grid-min="{gridMinWidth}rem"
	>
		{#each controller.items as item, index}
			{@render items(item, index)}
		{/each}
	</div>

	{#if showPaginationControls}
		<Pagination
			{controller}
			variant={paginationVariant}
			{showLimitSelector}
		/>
	{/if}
{/if}

<style>
	.underlay-paginated-list {
		display: flex;
		flex-direction: column;
		gap: var(--paginated-list-gap, 1rem);
	}

	.underlay-paginated-list--grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(var(--paginated-list-grid-min, 20rem), 1fr));
	}

	.underlay-paginated-list__empty {
		color: var(--underlay-color-text-muted, #64748b);
		text-align: center;
		padding: var(--underlay-space-8, 2rem);
		margin: 0;
	}
</style>
