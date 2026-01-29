<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Minimum width for each section before wrapping (default: "200px") */
    minSectionWidth?: string;
    /** Stack sections in a single column instead of auto-fit grid */
    singleColumn?: boolean;
    /** Additional CSS classes */
    class?: string;
    children: Snippet;
  }

  let {
    minSectionWidth = "200px",
    singleColumn = false,
    class: className,
    children
  }: Props = $props();

  const style = `--details-grid-min-section-width: ${minSectionWidth}`;
</script>

<div class="details-grid {className ?? ''}" class:details-grid--single-column={singleColumn} {style}>
  {@render children()}
</div>

<style>
  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--details-grid-min-section-width, 200px), 1fr));
    gap: 1.5rem;
    padding: 1.25rem 1.5rem;
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-radius: var(--underlay-radius-lg, 0.75rem);
  }

  .details-grid--single-column {
    grid-template-columns: 1fr;
  }
</style>
