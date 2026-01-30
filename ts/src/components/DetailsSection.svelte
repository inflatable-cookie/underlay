<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Legend/title for this section */
    legend?: string;
    /** Additional CSS classes */
    class?: string;
    children: Snippet;
  }

  let {
    legend,
    class: className,
    children
  }: Props = $props();
</script>

<div class="details-section {className ?? ''}"
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
    grid-template-columns: repeat(auto-fill, minmax(var(--underlay-details-section-min-width, 12rem), 1fr));
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
