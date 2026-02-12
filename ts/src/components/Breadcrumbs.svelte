<script lang="ts" module>
	/** A single breadcrumb item */
	export interface BreadcrumbItem {
		/** Display label */
		label: string;
		/** Link URL (omit for current/last item) */
		href?: string;
		/** Optional icon */
		icon?: string;
	}

	interface InternalBreadcrumbItem extends BreadcrumbItem {
		collapsed?: boolean;
	}
</script>

<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";

	/**
	 * Breadcrumbs navigation component.
	 *
	 * @example
	 * ```svelte
	 * <Breadcrumbs items={[
	 *   { label: 'Home', href: '/' },
	 *   { label: 'Products', href: '/products' },
	 *   { label: 'Widget' }
	 * ]} />
	 * ```
	 *
	 * @example With custom separator
	 * ```svelte
	 * <Breadcrumbs items={items} separator="/" />
	 * ```
	 */

	interface Props extends HTMLAttributes<HTMLElement> {
		/** Array of breadcrumb items */
		items?: BreadcrumbItem[];
		/** Separator between items */
		separator?: string;
		/** Additional CSS class */
		className?: string;
		/** Collapse middle items on mobile when there are many items */
		collapseOnMobile?: boolean;
		/** Maximum items to show before collapsing (includes first and last) */
		maxItems?: number;
	}

	let {
		items = [],
		separator = "›",
		className = "",
		collapseOnMobile = true,
		maxItems = 4,
		...restProps
	}: Props = $props();

	let shouldCollapse = $derived(collapseOnMobile && items.length > maxItems);
	let visibleItems = $derived<InternalBreadcrumbItem[]>(
		shouldCollapse
			? [items[0], { label: "…", collapsed: true }, ...items.slice(-2)]
			: items
	);
</script>

<nav
	class="underlay-breadcrumbs {className}"
	aria-label="Breadcrumb"
	{...restProps}
>
	<ol class="underlay-breadcrumb-list" class:underlay-collapsible={shouldCollapse}>
		{#each visibleItems as item, index}
			<li class="underlay-breadcrumb-item" class:underlay-collapsed={item.collapsed}>
				{#if item.collapsed}
					<span class="underlay-breadcrumb-ellipsis" title="More items">…</span>
				{:else if item.href && index < visibleItems.length - 1}
					<a href={item.href} class="underlay-breadcrumb-link">
						{#if item.icon}
							<span class="underlay-breadcrumb-icon">{item.icon}</span>
						{/if}
						{item.label}
					</a>
				{:else}
					<span
						class="underlay-breadcrumb-current"
						aria-current={index === visibleItems.length - 1 ? "page" : undefined}
					>
						{#if item.icon}
							<span class="underlay-breadcrumb-icon">{item.icon}</span>
						{/if}
						{item.label}
					</span>
				{/if}
			</li>

			{#if index < visibleItems.length - 1}
				<li class="underlay-breadcrumb-separator" aria-hidden="true">
					{separator}
				</li>
			{/if}
		{/each}
	</ol>
</nav>

<style>
	.underlay-breadcrumbs {
		font-size: var(--underlay-breadcrumb-font-size, 0.875rem);
	}

	.underlay-breadcrumb-list {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.25rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.underlay-breadcrumb-item {
		display: inline-flex;
		align-items: center;
	}

	.underlay-breadcrumb-separator {
		color: var(--underlay-color-text-muted, #94a3b8);
		user-select: none;
		padding: 0 0.125rem;
	}

	.underlay-breadcrumb-link {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--underlay-color-text-muted, #64748b);
		text-decoration: none;
		transition:
			color 0.15s,
			text-decoration 0.15s;
	}

	.underlay-breadcrumb-link:hover {
		color: var(--underlay-color-primary, #2563eb);
		text-decoration: underline;
	}

	.underlay-breadcrumb-link:focus-visible {
		outline: 2px solid var(--underlay-color-primary, #2563eb);
		outline-offset: 2px;
		border-radius: 2px;
	}

	.underlay-breadcrumb-current {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--underlay-color-text, #1e293b);
		font-weight: 500;
	}

	.underlay-breadcrumb-ellipsis {
		color: var(--underlay-color-text-muted, #94a3b8);
		padding: 0 0.25rem;
	}

	.underlay-breadcrumb-icon {
		display: inline-flex;
		font-size: 1em;
	}

	/* Mobile: hide collapsed items */
	@media (max-width: 640px) {
		.underlay-collapsible .underlay-collapsed {
			display: none;
		}
	}

	/* Dark mode */
	:global([data-theme="dark"]) .underlay-breadcrumb-link {
		color: var(--underlay-color-text-muted-dark, #94a3b8);
	}

	:global([data-theme="dark"]) .underlay-breadcrumb-link:hover {
		color: var(--underlay-color-primary-light, #60a5fa);
	}

	:global([data-theme="dark"]) .underlay-breadcrumb-current {
		color: var(--underlay-color-text-dark, #f1f5f9);
	}

	:global([data-theme="dark"]) .underlay-breadcrumb-separator,
	:global([data-theme="dark"]) .underlay-breadcrumb-ellipsis {
		color: var(--underlay-color-text-muted-dark, #64748b);
	}
</style>
