<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Layout columns: 1, 2, 3, 4, 6 or "auto" for auto-fit */
    columns?: 1 | 2 | 3 | 4 | 6 | "auto";
    /** Span full width in a 2-column form grid */
    full?: boolean;
    /** Additional CSS class */
    class?: string;
    children?: Snippet;
  }

  let { columns = 1, full = false, class: className = "", children }: Props = $props();

  const columnClass = $derived(
    columns === "auto"
      ? ""
      : `underlay-fieldset-grid--cols-${columns} underlay-fieldset--cols-${columns}`
  );
  const fullClass = $derived(
    full
      ? "underlay-fieldset-grid--full underlay-fieldset--full"
      : ""
  );
</script>

<div class={`underlay-fieldset-grid underlay-fieldset ${columnClass} ${fullClass} ${className}`}>
  {@render children?.()}
</div>

<style>
  .underlay-fieldset-grid {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
    grid-template-columns: minmax(0, 1fr);
    align-items: start;
    min-width: 0;
    container-type: inline-size;
  }

  /*
   * Optional mobile variants for very small containers:
   * - underlay-fieldset-grid--mobile-2: force two columns.
   * - underlay-fieldset-grid--mobile-2-last-two: first child full width, remaining two side-by-side.
   */
  @container (max-width: 399px) {
    .underlay-fieldset-grid--mobile-2,
    .underlay-fieldset--mobile-2,
    .underlay-fieldset-grid--mobile-2-last-two,
    .underlay-fieldset--mobile-2-last-two {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .underlay-fieldset-grid--mobile-2-first-full > :global(:first-child),
    .underlay-fieldset--mobile-2-first-full > :global(:first-child),
    .underlay-fieldset-grid--mobile-2-last-two > :global(:first-child),
    .underlay-fieldset--mobile-2-last-two > :global(:first-child) {
      grid-column: 1 / -1;
    }
  }

  .underlay-fieldset-grid > :global(*) {
    min-width: 0;
    max-width: 100%;
  }

  .underlay-fieldset-grid.underlay-fieldset-grid--full,
  .underlay-fieldset-grid.underlay-fieldset--full {
    grid-column: 1 / -1;
  }

  /* Auto-fit columns based on available space */
  .underlay-fieldset-grid:not(.underlay-fieldset-grid--cols-1):not(.underlay-fieldset-grid--cols-2):not(.underlay-fieldset-grid--cols-3):not(.underlay-fieldset-grid--cols-4):not(.underlay-fieldset-grid--cols-6) {
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 14rem), 1fr));
  }

  /* Fixed column layouts using container queries */
  @container (min-width: 400px) {
    .underlay-fieldset-grid--cols-2,
    .underlay-fieldset--cols-2 {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    /* Progressive: 3+ columns collapse to 2 first */
    .underlay-fieldset-grid--cols-3,
    .underlay-fieldset-grid--cols-4,
    .underlay-fieldset--cols-3,
    .underlay-fieldset--cols-4 {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .underlay-fieldset-grid--cols-6,
    .underlay-fieldset--cols-6 {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }

  /* Full column layouts at wider container widths */
  @container (min-width: 600px) {
    .underlay-fieldset-grid--cols-3,
    .underlay-fieldset--cols-3 {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }

    .underlay-fieldset-grid--cols-4,
    .underlay-fieldset--cols-4 {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }

    .underlay-fieldset-grid--cols-6,
    .underlay-fieldset--cols-6 {
      grid-template-columns: repeat(6, minmax(0, 1fr));
    }
  }
</style>
