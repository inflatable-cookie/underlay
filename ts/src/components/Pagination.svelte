<script lang="ts" module>
	import type { PaginationController } from "../patterns/pagination-types";

	/** Pagination state passed to the component (legacy interface) */
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
	import { tick } from "svelte";
	import type { Snippet } from "svelte";
	import type { HTMLAttributes } from "svelte/elements";

	/**
	 * Standalone Pagination component for navigating through pages of content.
	 *
	 * Can be used in two modes:
	 * 1. Controller mode: Pass a PaginationController from createPaginationController/createClientPagination
	 * 2. Props mode: Pass individual page/limit/total props with callbacks
	 *
	 * @example Controller mode (recommended)
	 * ```svelte
	 * <Pagination controller={pagination} />
	 * ```
	 *
	 * @example Props mode (legacy)
	 * ```svelte
	 * <Pagination
	 *   page={1}
	 *   limit={20}
	 *   total={100}
	 *   onPage={(page) => goto(`?page=${page}`)}
	 * />
	 * ```
	 *
	 * @example With limit selector
	 * ```svelte
	 * <Pagination
	 *   controller={pagination}
	 *   showLimitSelector
	 * />
	 * ```
	 */

	interface Props extends HTMLAttributes<HTMLElement> {
		/** Pagination controller from createPaginationController or createClientPagination */
		controller?: PaginationController<unknown>;
		/** Current page (1-based) - ignored if controller is provided */
		page?: number;
		/** Items per page - ignored if controller is provided */
		limit?: number;
		/** Total number of items - ignored if controller is provided */
		total?: number;
		/** Show the items-per-page selector */
		showLimitSelector?: boolean;
		/** Available limit options */
		limitOptions?: number[];
		/** Show "Showing X to Y of Z" info text */
		showInfo?: boolean;
		/** Compact mode (smaller padding, no info text on mobile) */
		compact?: boolean;
		/** Variant: "full" shows page numbers, "simple" shows only prev/next */
		variant?: "full" | "simple";
		/** Additional CSS class */
		className?: string;
		/** Callback when page changes - ignored if controller is provided */
		onPage?: (page: number) => void;
		/** Callback when limit changes - ignored if controller is provided */
		onLimit?: (limit: number) => void;
		/** Element or CSS selector to scroll to when page changes. Set to false to disable. */
		scrollTarget?: HTMLElement | string | false;
		/** Offset in pixels from the scroll target (default: 16) */
		scrollOffset?: number;
	}

	let {
		controller,
		page = 1,
		limit = 20,
		total = 0,
		showLimitSelector = false,
		limitOptions = [30, 50, 100],
		showInfo = true,
		compact = false,
		variant = "full",
		className = "",
		onPage,
		onLimit,
		scrollTarget,
		scrollOffset = 16,
		...restProps
	}: Props = $props();

	/**
	 * Find the nearest scrollable ancestor of an element
	 */
	function getScrollParent(element: HTMLElement): HTMLElement | null {
		let parent = element.parentElement;

		while (parent) {
			const style = getComputedStyle(parent);
			const overflowY = style.overflowY;

			if (overflowY === "auto" || overflowY === "scroll") {
				// Check if it's actually scrollable (has overflow content)
				if (parent.scrollHeight > parent.clientHeight) {
					return parent;
				}
			}

			parent = parent.parentElement;
		}

		return null;
	}

	/**
	 * Scroll to the target element with offset
	 */
	function doScroll() {
		if (scrollTarget === false) return;

		let element: HTMLElement | null = null;

		if (typeof scrollTarget === "string") {
			element = document.querySelector(scrollTarget);
		} else if (scrollTarget instanceof HTMLElement) {
			element = scrollTarget;
		}

		if (!element) return;

		const scrollParent = getScrollParent(element);

		// Only scroll if we found a scrollable container
		if (!scrollParent) return;

		const containerRect = scrollParent.getBoundingClientRect();
		const elementRect = element.getBoundingClientRect();

		// Calculate where element is relative to the container's scroll position
		const relativeTop = elementRect.top - containerRect.top;

		// Only scroll if element is not already near the top of the container
		if (relativeTop < scrollOffset && relativeTop > -scrollOffset) {
			// Already at the top, no need to scroll
			return;
		}

		// Calculate target scroll position
		const targetScroll = scrollParent.scrollTop + relativeTop - scrollOffset;

		// Clamp to valid range
		const maxScroll = scrollParent.scrollHeight - scrollParent.clientHeight;
		const clampedScroll = Math.max(0, Math.min(targetScroll, maxScroll));

		scrollParent.scrollTo({ top: clampedScroll, behavior: "smooth" });
	}

	// Derive values from controller if provided, otherwise use props
	let effectivePage = $derived(controller?.currentPage ?? page);
	let effectiveLimit = $derived(controller?.pageSize ?? limit);
	let effectiveTotal = $derived(controller?.total ?? total);
	let effectiveTotalPages = $derived(controller?.totalPages ?? Math.ceil((effectiveTotal ?? 0) / effectiveLimit));
	let startItem = $derived(controller?.showingFrom ?? ((effectiveTotal ?? 0) === 0 ? 0 : (effectivePage - 1) * effectiveLimit + 1));
	let endItem = $derived(controller?.showingTo ?? Math.min(effectivePage * effectiveLimit, effectiveTotal ?? 0));
	let hasPrev = $derived(controller?.hasPrevPage ?? effectivePage > 1);
	let hasNext = $derived(controller?.hasNextPage ?? effectivePage < (effectiveTotalPages ?? 1));
	let isLoading = $derived(controller?.loading ?? false);

	// Check if controller supports random page access (client-side pagination)
	let supportsGoToPage = $derived(controller && typeof controller.goToPage === 'function');

	async function handlePrev() {
		if (controller) {
			await controller.prevPage();
		} else {
			onPage?.(effectivePage - 1);
		}
		// Wait for Svelte to update DOM, then scroll
		await tick();
		doScroll();
	}

	async function handleNext() {
		if (controller) {
			await controller.nextPage();
		} else {
			onPage?.(effectivePage + 1);
		}
		// Wait for Svelte to update DOM, then scroll
		await tick();
		doScroll();
	}

	async function handleGoToPage(newPage: number) {
		if (controller && supportsGoToPage) {
			await controller.goToPage!(newPage);
		} else if (!controller) {
			onPage?.(newPage);
		}
		// Wait for Svelte to update DOM, then scroll
		await tick();
		doScroll();
	}

	async function handleLimitChange(event: Event) {
		const newLimit = Number((event.target as HTMLSelectElement).value);
		if (controller) {
			await controller.setPageSize(newLimit);
		} else {
			onLimit?.(newLimit);
		}
		// Wait for Svelte to update DOM, then scroll
		await tick();
		doScroll();
	}
</script>

{#if (effectiveTotalPages ?? 1) > 1 || showLimitSelector}
	<nav
		class="underlay-pagination {className}"
		class:underlay-compact={compact}
		class:underlay-loading={isLoading}
		aria-label="Pagination"
		{...restProps}
	>
		{#if showInfo && (effectiveTotal ?? 0) > 0}
			<div class="underlay-pagination-info">
				{#if effectiveTotal !== null}
					Showing {startItem} to {endItem} of {effectiveTotal.toLocaleString()}
				{:else}
					Showing {startItem} to {endItem}
				{/if}
			</div>
		{/if}

		<div class="underlay-pagination-controls-wrapper">
			{#if showLimitSelector && limitOptions.length > 0}
				<div class="underlay-limit-selector">
					<label for="pagination-limit">Show</label>
					<select
						id="pagination-limit"
						value={effectiveLimit}
						onchange={handleLimitChange}
						disabled={isLoading}
					>
						{#each limitOptions as option}
							<option value={option}>{option}</option>
						{/each}
					</select>
					<span>per page</span>
				</div>
			{/if}

			{#if (effectiveTotalPages ?? 1) > 1 || hasPrev || hasNext}
				<div class="underlay-pagination-controls">
					{#if variant === "full" && supportsGoToPage}
						<button
							type="button"
							class="underlay-pagination-button"
							disabled={!hasPrev || isLoading}
							onclick={() => handleGoToPage(1)}
							aria-label="First page"
						>
							««
						</button>
					{/if}
					<button
						type="button"
						class="underlay-pagination-button"
						disabled={!hasPrev || isLoading}
						onclick={handlePrev}
						aria-label="Previous page"
					>
						«{#if variant === "simple"}&nbsp;Prev{/if}
					</button>

					{#if variant === "full" && effectiveTotalPages !== null}
						<span class="underlay-pagination-page">
							Page {effectivePage} of {effectiveTotalPages}
						</span>
					{:else if variant === "simple"}
						<span class="underlay-pagination-page">
							{startItem}–{endItem}{#if effectiveTotal !== null} of {effectiveTotal.toLocaleString()}{/if}
						</span>
					{/if}

					<button
						type="button"
						class="underlay-pagination-button"
						disabled={!hasNext || isLoading}
						onclick={handleNext}
						aria-label="Next page"
					>
						{#if variant === "simple"}Next&nbsp;{/if}»
					</button>
					{#if variant === "full" && supportsGoToPage && effectiveTotalPages !== null}
						<button
							type="button"
							class="underlay-pagination-button"
							disabled={!hasNext || isLoading}
							onclick={() => handleGoToPage(effectiveTotalPages!)}
							aria-label="Last page"
						>
							»»
						</button>
					{/if}
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
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		background: var(--underlay-color-surface-muted, var(--underlay-color-bg-surface, transparent));
		border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
	}

	.underlay-pagination.underlay-compact {
		padding: 0.5rem 0.75rem;
		gap: 0.75rem;
	}

	.underlay-pagination.underlay-loading {
		opacity: 0.7;
		pointer-events: none;
	}

	.underlay-pagination-info {
		color: var(--underlay-color-text-muted, #9ca3af);
	}

	.underlay-pagination-controls-wrapper {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.underlay-limit-selector {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		color: var(--underlay-color-text-muted, #9ca3af);
	}

	.underlay-limit-selector select {
		padding: 0.25rem 0.5rem;
		font-size: 0.875rem;
		border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
		border-radius: var(--underlay-radius-sm, 0.25rem);
		background: var(--underlay-color-button-neutral-bg, rgba(255, 255, 255, 0.05));
		color: var(--underlay-color-text, inherit);
		cursor: pointer;
	}

	.underlay-limit-selector select:focus {
		outline: 2px solid var(--underlay-color-primary, #14b8a6);
		outline-offset: 1px;
	}

	.underlay-pagination-controls {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.underlay-pagination-button {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 2rem;
		padding: 0.375rem 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--underlay-color-text, inherit);
		background: var(--underlay-color-button-neutral-bg, rgba(255, 255, 255, 0.05));
		border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
		border-radius: var(--underlay-radius-sm, 0.25rem);
		cursor: pointer;
		transition:
			background-color 0.15s,
			border-color 0.15s,
			color 0.15s;
	}

	.underlay-pagination-button:hover:not(:disabled) {
		background: var(--underlay-color-button-neutral-hover, rgba(255, 255, 255, 0.1));
		border-color: var(--underlay-color-border-strong, rgba(148, 163, 184, 0.4));
	}

	.underlay-pagination-button:focus-visible {
		outline: 2px solid var(--underlay-color-primary, #14b8a6);
		outline-offset: 1px;
	}

	.underlay-pagination-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.underlay-pagination-page {
		padding: 0 0.5rem;
		color: var(--underlay-color-text-muted, #9ca3af);
		white-space: nowrap;
	}

	/* Mobile: stack layout */
	@media (max-width: 640px) {
		.underlay-pagination {
			flex-direction: column;
			align-items: stretch;
		}

		.underlay-pagination.underlay-compact .underlay-pagination-info {
			display: none;
		}

		.underlay-pagination-controls-wrapper {
			justify-content: center;
			flex-wrap: wrap;
		}
	}
</style>
