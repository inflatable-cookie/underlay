<script lang="ts" module>
  export type StatVariant = "default" | "success" | "warning" | "danger" | "info";
</script>

<script lang="ts">
  /**
   * StatCard - A card for displaying a key metric with icon and label.
   *
   * Use for dashboard summaries and statistics displays.
   */
  import Card from "./Card.svelte";
  import type { Snippet } from "svelte";

  interface Props {
    /** Card title (displayed in header) */
    title: string;
    /** The main stat value to display */
    value: string | number;
    /** Label text below the value */
    label?: string;
    /** Icon to display (pass as snippet) */
    icon?: Snippet;
    /** Color variant for the icon background */
    variant?: StatVariant;
    /** Whether the stat is loading */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Link to navigate to on click */
    href?: string;
    /** Additional breakdown content below the stat */
    breakdown?: Snippet;
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
    breakdown
  }: Props = $props();
</script>

<Card {title} variant="muted" {href}>
  {#if loading}
    <p class="stat-loading">Loading...</p>
  {:else if error}
    <p class="stat-error">{error}</p>
  {:else}
    <div class="stat-card">
      {#if icon}
        <div class="stat-card__icon stat-card__icon--{variant}">
          {@render icon()}
        </div>
      {/if}
      <div class="stat-card__content">
        <p class="stat-card__value">{value}</p>
        {#if label}
          <p class="stat-card__label">{label}</p>
        {/if}
      </div>
    </div>
    {#if breakdown}
      <div class="stat-card__breakdown">
        {@render breakdown()}
      </div>
    {/if}
  {/if}
</Card>

<style>
  .stat-loading,
  .stat-error {
    margin: 0;
    font-size: 0.875rem;
  }

  .stat-loading {
    color: var(--text-secondary, #6b7280);
  }

  .stat-error {
    color: var(--danger, #ef4444);
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .stat-card__icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    height: 3rem;
    border-radius: 0.5rem;
    flex-shrink: 0;
  }

  .stat-card__icon--default {
    background: var(--bg-muted, #f3f4f6);
    color: var(--text-secondary, #6b7280);
  }

  .stat-card__icon--success {
    background: color-mix(in srgb, var(--success, #22c55e) 15%, transparent);
    color: var(--success, #22c55e);
  }

  .stat-card__icon--warning {
    background: color-mix(in srgb, var(--warning, #f97316) 15%, transparent);
    color: var(--warning, #f97316);
  }

  .stat-card__icon--danger {
    background: color-mix(in srgb, var(--danger, #ef4444) 15%, transparent);
    color: var(--danger, #ef4444);
  }

  .stat-card__icon--info {
    background: color-mix(in srgb, var(--info, #3b82f6) 15%, transparent);
    color: var(--info, #3b82f6);
  }

  .stat-card__content {
    flex: 1;
    min-width: 0;
  }

  .stat-card__value {
    margin: 0;
    font-size: 1.75rem;
    font-weight: 700;
    line-height: 1.2;
    color: var(--text-primary, #111827);
  }

  .stat-card__label {
    margin: 0.125rem 0 0;
    font-size: 0.875rem;
    color: var(--text-secondary, #6b7280);
  }

  .stat-card__breakdown {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    margin-top: 0.75rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border-color, #e5e7eb);
  }
</style>
