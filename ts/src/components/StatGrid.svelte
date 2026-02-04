<script lang="ts">
  /**
   * StatGrid - A responsive grid layout for StatCard components.
   *
   * Provides a consistent grid layout for dashboard statistics.
   */
  import type { Snippet } from "svelte";

  interface Props {
    /** Grid column configuration */
    columns?: 2 | 3 | 4 | 5;
    /** Minimum item width before wrapping */
    minItemWidth?: number;
    /** Grid children */
    children: Snippet;
  }

  let {
    columns = 4,
    minItemWidth = 250,
    children
  }: Props = $props();
</script>

<div
  class="stat-grid"
  style:--min-item-width="{minItemWidth}px"
  style:--max-columns={columns}
>
  {@render children()}
</div>

<style>
  .stat-grid {
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(min(var(--min-item-width), 100%), 1fr)
    );
    gap: 1rem;
  }

  @container (min-width: 800px) {
    .stat-grid {
      grid-template-columns: repeat(var(--max-columns), 1fr);
    }
  }
</style>
