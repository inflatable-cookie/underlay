<script lang="ts" context="module">
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

	/** Array of breadcrumb items */
	export let items: BreadcrumbItem[] = [];

	/** Separator between items */
	export let separator: string = "›";

	/** Additional CSS class */
	export let className: string = "";

	/** Collapse middle items on mobile when there are many items */
	export let collapseOnMobile: boolean = true;

	/** Maximum items to show before collapsing (includes first and last) */
	export let maxItems: number = 4;

	$: shouldCollapse = collapseOnMobile && items.length > maxItems;
	$: visibleItems = shouldCollapse
		? ([items[0], { label: "…", collapsed: true }, ...items.slice(-2)] as InternalBreadcrumbItem[])
		: (items as InternalBreadcrumbItem[]);
</script>

<nav
	class="underlay-breadcrumbs {className}"
	aria-label="Breadcrumb"
	{...$$restProps}
>
	<ol class="breadcrumb-list" class:collapsible={shouldCollapse}>
		{#each visibleItems as item, index}
			<li class="breadcrumb-item" class:collapsed={item.collapsed}>
				{#if item.collapsed}
					<span class="breadcrumb-ellipsis" title="More items">…</span>
				{:else if item.href && index < visibleItems.length - 1}
					<a href={item.href} class="breadcrumb-link">
						{#if item.icon}
							<span class="breadcrumb-icon">{item.icon}</span>
						{/if}
						{item.label}
					</a>
				{:else}
					<span
						class="breadcrumb-current"
						aria-current={index === visibleItems.length - 1 ? "page" : undefined}
					>
						{#if item.icon}
							<span class="breadcrumb-icon">{item.icon}</span>
						{/if}
						{item.label}
					</span>
				{/if}
			</li>

			{#if index < visibleItems.length - 1}
				<li class="breadcrumb-separator" aria-hidden="true">
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

	.breadcrumb-list {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.25rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.breadcrumb-item {
		display: inline-flex;
		align-items: center;
	}

	.breadcrumb-separator {
		color: var(--underlay-color-text-muted, #94a3b8);
		user-select: none;
		padding: 0 0.125rem;
	}

	.breadcrumb-link {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--underlay-color-text-muted, #64748b);
		text-decoration: none;
		transition:
			color 0.15s,
			text-decoration 0.15s;
	}

	.breadcrumb-link:hover {
		color: var(--underlay-color-primary, #2563eb);
		text-decoration: underline;
	}

	.breadcrumb-link:focus-visible {
		outline: 2px solid var(--underlay-color-primary, #2563eb);
		outline-offset: 2px;
		border-radius: 2px;
	}

	.breadcrumb-current {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--underlay-color-text, #1e293b);
		font-weight: 500;
	}

	.breadcrumb-ellipsis {
		color: var(--underlay-color-text-muted, #94a3b8);
		padding: 0 0.25rem;
	}

	.breadcrumb-icon {
		display: inline-flex;
		font-size: 1em;
	}

	/* Mobile: hide collapsed items */
	@media (max-width: 640px) {
		.collapsible .collapsed {
			display: none;
		}
	}

	/* Dark mode */
	:global([data-theme="dark"]) .breadcrumb-link {
		color: var(--underlay-color-text-muted-dark, #94a3b8);
	}

	:global([data-theme="dark"]) .breadcrumb-link:hover {
		color: var(--underlay-color-primary-light, #60a5fa);
	}

	:global([data-theme="dark"]) .breadcrumb-current {
		color: var(--underlay-color-text-dark, #f1f5f9);
	}

	:global([data-theme="dark"]) .breadcrumb-separator,
	:global([data-theme="dark"]) .breadcrumb-ellipsis {
		color: var(--underlay-color-text-muted-dark, #64748b);
	}
</style>
