<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Grid content */
    children: Snippet;
    /** Breakpoint at which to collapse to single column (default: 700px) */
    breakpoint?: number;
    /** Gap between grid items (default: 1.5rem) */
    gap?: string;
    /** Whether items should stretch to fill the row height (default: false) */
    stretch?: boolean;
    /** Additional class for the container */
    class?: string;
  }

  let {
    children,
    breakpoint = 700,
    gap = "1.5rem",
    stretch = false,
    class: className = ""
  }: Props = $props();
</script>

<div class="container-grid-wrapper {className}" class:container-grid-wrapper--stretch={stretch} style="--grid-gap: {gap}; --grid-breakpoint: {breakpoint}px;">
  <div class="container-grid">
    {@render children()}
  </div>
</div>

<style>
  .container-grid-wrapper {
    container-type: inline-size;
  }

  .container-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--grid-gap, 1.5rem);
    align-items: start;
  }

  .container-grid-wrapper--stretch .container-grid {
    align-items: stretch;
  }

  /* Remove default margins/max-width from components when in grid */
  .container-grid :global(.details-grid) {
    margin-top: 0;
  }

  .container-grid :global(.inline-list-card) {
    margin-top: 0;
    max-width: none;
  }

  @container (max-width: 700px) {
    .container-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
