<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLAttributes } from "svelte/elements";

	/**
	 * ProgressBar component for showing completion progress.
	 *
	 * @example
	 * ```svelte
	 * <ProgressBar value={50} />
	 * <ProgressBar value={75} max={100} variant="success" />
	 * <ProgressBar value={30} showLabel />
	 * <ProgressBar value={60} size="lg" animated />
	 * ```
	 */

	interface Props extends HTMLAttributes<HTMLDivElement> {
		/** Current progress value */
		value: number;
		/** Maximum value (default: 100) */
		max?: number;
		/** Visual style of the progress bar */
		variant?: "default" | "success" | "warning" | "danger" | "info";
		/** Size/height of the progress bar */
		size?: "sm" | "md" | "lg";
		/** Show percentage label */
		showLabel?: boolean;
		/** Enable animation on the progress fill */
		animated?: boolean;
		/** Custom label format function */
		formatLabel?: (value: number, max: number, percentage: number) => string;
		/** Additional CSS class */
		className?: string;
		/** Optional custom label content */
		label?: Snippet<[{ value: number; max: number; percentage: number }]>;
	}

	let {
		value,
		max = 100,
		variant = "default",
		size = "md",
		showLabel = false,
		animated = false,
		formatLabel = undefined,
		className = "",
		label = undefined,
		...restProps
	}: Props = $props();

	const percentage = $derived(Math.min(100, Math.max(0, (value / max) * 100)));
	const displayLabel = $derived(
		formatLabel ? formatLabel(value, max, percentage) : `${Math.round(percentage)}%`
	);
</script>

<div
	class="underlay-progress underlay-progress--{variant} underlay-progress--{size} {className}"
	role="progressbar"
	aria-valuenow={value}
	aria-valuemin={0}
	aria-valuemax={max}
	aria-label={displayLabel}
	{...restProps}
>
	<div class="underlay-progress-track">
		<div
			class="underlay-progress-fill {animated ? 'underlay-progress-fill--animated' : ''}"
			style="width: {percentage}%"
		></div>
	</div>
	{#if showLabel || label}
		<span class="underlay-progress-label">
			{#if label}
				{@render label({ value, max, percentage })}
			{:else}
				{displayLabel}
			{/if}
		</span>
	{/if}
</div>

<style>
	.underlay-progress {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
	}

	.underlay-progress-track {
		flex: 1;
		background-color: var(--underlay-color-progress-track, rgba(100, 116, 139, 0.15));
		border-radius: var(--underlay-radius-pill, 999px);
		overflow: hidden;
	}

	.underlay-progress-fill {
		height: 100%;
		border-radius: var(--underlay-radius-pill, 999px);
		transition: width 0.3s ease;
	}

	.underlay-progress-fill--animated {
		background-image: linear-gradient(
			-45deg,
			rgba(255, 255, 255, 0.15) 25%,
			transparent 25%,
			transparent 50%,
			rgba(255, 255, 255, 0.15) 50%,
			rgba(255, 255, 255, 0.15) 75%,
			transparent 75%,
			transparent
		);
		background-size: 1rem 1rem;
		animation: progress-stripes 1s linear infinite;
	}

	@keyframes progress-stripes {
		from {
			background-position: 1rem 0;
		}
		to {
			background-position: 0 0;
		}
	}

	.underlay-progress-label {
		font-size: 0.875em;
		font-weight: 500;
		color: var(--underlay-color-text-muted, #64748b);
		min-width: 3em;
		text-align: right;
	}

	/* Sizes */
	.underlay-progress--sm .underlay-progress-track {
		height: 0.375rem;
	}

	.underlay-progress--sm .underlay-progress-label {
		font-size: 0.75em;
	}

	.underlay-progress--md .underlay-progress-track {
		height: 0.5rem;
	}

	.underlay-progress--lg .underlay-progress-track {
		height: 0.75rem;
	}

	.underlay-progress--lg .underlay-progress-label {
		font-size: 1em;
	}

	/* Variants */
	.underlay-progress--default .underlay-progress-fill {
		background-color: var(--underlay-color-progress-default, #6366f1);
	}

	.underlay-progress--success .underlay-progress-fill {
		background-color: var(--underlay-color-progress-success, #22c55e);
	}

	.underlay-progress--warning .underlay-progress-fill {
		background-color: var(--underlay-color-progress-warning, #f59e0b);
	}

	.underlay-progress--danger .underlay-progress-fill {
		background-color: var(--underlay-color-progress-danger, #ef4444);
	}

	.underlay-progress--info .underlay-progress-fill {
		background-color: var(--underlay-color-progress-info, #3b82f6);
	}

	/* Dark mode adjustments */
	:global([data-theme="dark"]) .underlay-progress-track {
		background-color: var(--underlay-color-progress-track-dark, rgba(148, 163, 184, 0.2));
	}

	:global([data-theme="dark"]) .underlay-progress-label {
		color: var(--underlay-color-text-muted-dark, #94a3b8);
	}

	:global([data-theme="dark"]) .underlay-progress--default .underlay-progress-fill {
		background-color: var(--underlay-color-progress-default-dark, #818cf8);
	}

	:global([data-theme="dark"]) .underlay-progress--success .underlay-progress-fill {
		background-color: var(--underlay-color-progress-success-dark, #4ade80);
	}

	:global([data-theme="dark"]) .underlay-progress--warning .underlay-progress-fill {
		background-color: var(--underlay-color-progress-warning-dark, #fbbf24);
	}

	:global([data-theme="dark"]) .underlay-progress--danger .underlay-progress-fill {
		background-color: var(--underlay-color-progress-danger-dark, #f87171);
	}

	:global([data-theme="dark"]) .underlay-progress--info .underlay-progress-fill {
		background-color: var(--underlay-color-progress-info-dark, #60a5fa);
	}
</style>
