<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Legend/title for this section */
    legend?: string;
    /** Minimum item width before wrapping (default: "12rem") */
    minItemWidth?: string;
    /** Additional CSS classes */
    class?: string;
    children: Snippet;
  }

  let {
    legend,
    minItemWidth = "10rem",
    class: className,
    children
  }: Props = $props();

  const style = `--details-section-min-width: ${minItemWidth}`;
</script>

<div class="details-section {className ?? ''}" {style}>
  {#if legend}
    <h4 class="details-section__legend">{legend}</h4>
  {/if}
  <div class="details-section__items">
    {@render children()}
  </div>
</div>

<style>
  .details-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .details-section__legend {
    font-weight: 500;
    font-size: 0.6rem;
    color: var(--underlay-color-text-muted, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.6;
    margin: 0;
    padding: 0;
  }

  .details-section__items {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--details-section-min-width, 10rem), 1fr));
    gap: 1rem 1.5rem;
  }

  /* Utility classes for spanning columns */
  .details-section__items :global(.span-full) {
    grid-column: 1 / -1;
  }

  .details-section__items :global(.span-2) {
    grid-column: span 2;
  }

  /* TabsRoot (description tab groups) always span 2 columns */
  .details-section__items :global(.underlay-tabs) {
    grid-column: span 2;
  }
</style>
