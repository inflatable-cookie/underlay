<script lang="ts" module>
  export type StatVariant = "default" | "success" | "warning" | "danger" | "info";
</script>

<script lang="ts">
  /**
   * StatCard - A card for displaying a key metric with icon and label.
   *
   * Use for dashboard summaries and statistics displays.
   *
   * Supports two modes:
   * - Default: Full card with title header
   * - Compact: Inline badge style without card wrapper
   */
  import Card from "./Card.svelte";
  import type { Snippet } from "svelte";

  interface Props {
    /** Card title (displayed in header for default mode, hidden in compact) */
    title?: string;
    /** The main stat value to display */
    value: string | number;
    /** Label text below/beside the value */
    label?: string;
    /** Icon to display (pass as snippet) */
    icon?: Snippet;
    /** Color variant for styling */
    variant?: StatVariant;
    /** Whether the stat is loading */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Link to navigate to on click */
    href?: string;
    /** Additional breakdown content below the stat */
    breakdown?: Snippet;
    /** Use compact inline style instead of full card */
    compact?: boolean;
  }

  let {
    title,
    value,
    label,
    icon,
    variant = "default",
    loading = false,
    error = null,
    href,
    breakdown,
    compact = false
  }: Props = $props();
</script>

{#if compact}
  <div class="underlay-stat-compact underlay-stat-compact--{variant}">
    {#if loading}
      <span class="underlay-stat-compact__loading">...</span>
    {:else if error}
      <span class="underlay-stat-compact__error">!</span>
    {:else}
      {#if icon}
        <span class="underlay-stat-compact__icon">
          {@render icon()}
        </span>
      {/if}
      <span class="underlay-stat-compact__value">{value}</span>
      {#if label}
        <span class="underlay-stat-compact__label">{label}</span>
      {/if}
    {/if}
  </div>
{:else}
  <Card title={title ?? ""} variant="muted" {href}>
    {#if loading}
      <p class="underlay-stat-loading">Loading...</p>
    {:else if error}
      <p class="underlay-stat-error">{error}</p>
    {:else}
      <div class="underlay-stat-card">
        {#if icon}
          <div class="underlay-stat-card__icon underlay-stat-card__icon--{variant}">
            {@render icon()}
          </div>
        {/if}
        <div class="underlay-stat-card__content">
          <p class="underlay-stat-card__value">{value}</p>
          {#if label}
            <p class="underlay-stat-card__label">{label}</p>
          {/if}
        </div>
      </div>
      {#if breakdown}
        <div class="underlay-stat-card__breakdown">
          {@render breakdown()}
        </div>
      {/if}
    {/if}
  </Card>
{/if}

<style>
  /* Compact variant styles */
  .underlay-stat-compact {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-muted, rgba(15, 23, 42, 0.5));
    border-radius: 0.5rem;
    flex: 1;
    min-width: 150px;
  }

  .underlay-stat-compact--success {
    color: var(--underlay-color-success, #10b981);
  }

  .underlay-stat-compact--warning {
    color: var(--underlay-color-warning, #f97316);
  }

  .underlay-stat-compact--danger {
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-stat-compact--info {
    color: var(--underlay-color-primary, #3b82f6);
  }

  .underlay-stat-compact__icon {
    display: flex;
    align-items: center;
    flex-shrink: 0;
  }

  .underlay-stat-compact__value {
    font-size: 1.5rem;
    font-weight: 600;
  }

  .underlay-stat-compact__label {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #6b7280);
  }

  .underlay-stat-compact__loading {
    color: var(--underlay-color-text-muted, #6b7280);
  }

  .underlay-stat-compact__error {
    color: var(--underlay-color-danger, #ef4444);
  }

  /* Default card variant styles */
  .underlay-stat-loading,
  .underlay-stat-error {
    margin: 0;
    font-size: 0.875rem;
  }

  .underlay-stat-loading {
    color: var(--underlay-color-text-muted, #6b7280);
  }

  .underlay-stat-error {
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .underlay-stat-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border-radius: 0.5rem;
    flex-shrink: 0;
  }

  .underlay-stat-card__icon--default {
    background: var(--underlay-color-surface-muted, #e5e7eb);
    color: var(--underlay-color-text-muted, #6b7280);
  }

  .underlay-stat-card__icon--success {
    background: color-mix(in srgb, var(--underlay-color-success, #22c55e) 15%, transparent);
    color: var(--underlay-color-success, #22c55e);
  }

  .underlay-stat-card__icon--warning {
    background: color-mix(in srgb, var(--underlay-color-warning, #f97316) 15%, transparent);
    color: var(--underlay-color-warning, #f97316);
  }

  .underlay-stat-card__icon--danger {
    background: color-mix(in srgb, var(--underlay-color-danger, #ef4444) 15%, transparent);
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-stat-card__icon--info {
    background: color-mix(in srgb, var(--underlay-color-primary, #3b82f6) 15%, transparent);
    color: var(--underlay-color-primary, #3b82f6);
  }

  .underlay-stat-card__content {
    flex: 1;
    min-width: 0;
  }

  .underlay-stat-card__value {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
    line-height: 1.2;
    color: var(--underlay-color-text, inherit);
  }

  .underlay-stat-card__label {
    margin: 0.125rem 0 0;
    font-size: 0.875rem;
    color: var(--underlay-color-text-muted, #6b7280);
  }

  .underlay-stat-card__breakdown {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
  }
</style>
