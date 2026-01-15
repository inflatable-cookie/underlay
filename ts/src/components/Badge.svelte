<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLAttributes } from "svelte/elements";

	/**
	 * Badge component for status indicators, counts, and labels.
	 *
	 * @example
	 * ```svelte
	 * <Badge>Default</Badge>
	 * <Badge variant="success">Active</Badge>
	 * <Badge variant="warning">Pending</Badge>
	 * <Badge variant="danger">Error</Badge>
	 * <Badge variant="info">New</Badge>
	 * <Badge size="sm">Small</Badge>
	 * <Badge pill>Pill Shape</Badge>
	 * ```
	 */

	interface Props extends HTMLAttributes<HTMLSpanElement> {
		/** Visual style of the badge */
		variant?: "default" | "success" | "warning" | "danger" | "info" | "muted";
		/** Size of the badge */
		size?: "sm" | "md" | "lg";
		/** Use pill shape (fully rounded) instead of rounded corners */
		pill?: boolean;
		/** Optional icon to show before the text */
		icon?: string;
		/** Additional CSS class */
		className?: string;
		/** Badge content */
		children?: Snippet;
	}

	let {
		variant = "default",
		size = "md",
		pill = false,
		icon = undefined,
		className = "",
		children,
		...restProps
	}: Props = $props();
</script>

<span
	class="underlay-badge underlay-badge--{variant} underlay-badge--{size} {pill
		? 'underlay-badge--pill'
		: ''} {className}"
	{...restProps}
>
	{#if icon}
		<span class="badge-icon">{icon}</span>
	{/if}
	{@render children?.()}
</span>

<style>
	.underlay-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.25em;
		font-weight: 500;
		line-height: 1;
		white-space: nowrap;
		border-radius: var(--underlay-radius-sm, 0.25rem);
		transition:
			background-color 0.15s,
			color 0.15s;
	}

	/* Sizes */
	.underlay-badge--sm {
		padding: 0.2em 0.5em;
		font-size: 0.7em;
	}

	.underlay-badge--md {
		padding: 0.25em 0.6em;
		font-size: 0.8em;
	}

	.underlay-badge--lg {
		padding: 0.35em 0.75em;
		font-size: 0.9em;
	}

	/* Pill shape */
	.underlay-badge--pill {
		border-radius: var(--underlay-radius-pill, 999px);
	}

	/* Variants */
	.underlay-badge--default {
		background-color: var(--underlay-color-badge-default-bg, rgba(100, 116, 139, 0.15));
		color: var(--underlay-color-badge-default-text, #475569);
	}

	.underlay-badge--success {
		background-color: var(--underlay-color-badge-success-bg, rgba(34, 197, 94, 0.15));
		color: var(--underlay-color-badge-success-text, #15803d);
	}

	.underlay-badge--warning {
		background-color: var(--underlay-color-badge-warning-bg, rgba(245, 158, 11, 0.15));
		color: var(--underlay-color-badge-warning-text, #b45309);
	}

	.underlay-badge--danger {
		background-color: var(--underlay-color-badge-danger-bg, rgba(239, 68, 68, 0.15));
		color: var(--underlay-color-badge-danger-text, #dc2626);
	}

	.underlay-badge--info {
		background-color: var(--underlay-color-badge-info-bg, rgba(59, 130, 246, 0.15));
		color: var(--underlay-color-badge-info-text, #2563eb);
	}

	.underlay-badge--muted {
		background-color: var(--underlay-color-badge-muted-bg, rgba(148, 163, 184, 0.1));
		color: var(--underlay-color-badge-muted-text, #94a3b8);
	}

	/* Dark mode adjustments via CSS variables */
	:global([data-theme="dark"]) .underlay-badge--default {
		background-color: var(--underlay-color-badge-default-bg-dark, rgba(148, 163, 184, 0.2));
		color: var(--underlay-color-badge-default-text-dark, #cbd5e1);
	}

	:global([data-theme="dark"]) .underlay-badge--success {
		background-color: var(--underlay-color-badge-success-bg-dark, rgba(34, 197, 94, 0.2));
		color: var(--underlay-color-badge-success-text-dark, #4ade80);
	}

	:global([data-theme="dark"]) .underlay-badge--warning {
		background-color: var(--underlay-color-badge-warning-bg-dark, rgba(245, 158, 11, 0.2));
		color: var(--underlay-color-badge-warning-text-dark, #fbbf24);
	}

	:global([data-theme="dark"]) .underlay-badge--danger {
		background-color: var(--underlay-color-badge-danger-bg-dark, rgba(239, 68, 68, 0.2));
		color: var(--underlay-color-badge-danger-text-dark, #f87171);
	}

	:global([data-theme="dark"]) .underlay-badge--info {
		background-color: var(--underlay-color-badge-info-bg-dark, rgba(59, 130, 246, 0.2));
		color: var(--underlay-color-badge-info-text-dark, #60a5fa);
	}

	:global([data-theme="dark"]) .underlay-badge--muted {
		background-color: var(--underlay-color-badge-muted-bg-dark, rgba(148, 163, 184, 0.15));
		color: var(--underlay-color-badge-muted-text-dark, #64748b);
	}

	.badge-icon {
		display: inline-flex;
		font-size: 1em;
	}
</style>
