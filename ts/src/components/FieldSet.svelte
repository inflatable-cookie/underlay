<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Optional legend/title for the fieldset */
    legend?: string;
    /** Layout columns: 1, 2, 3, 4, 6 or "auto" for auto-fit */
    columns?: 1 | 2 | 3 | 4 | 6 | "auto";
    /** Span full width in a 2-column form grid */
    full?: boolean;
    /** Additional CSS class */
    class?: string;
    children?: Snippet;
  }

  let {
    legend,
    columns = 1,
    full = false,
    class: className = "",
    children
  }: Props = $props();

  const columnClass = columns === "auto" ? "" : `underlay-fieldset--cols-${columns}`;
  const fullClass = full ? "underlay-fieldset--full" : "";
</script>

<fieldset class={`underlay-fieldset ${columnClass} ${fullClass} ${className}`}>
  {#if legend}
    <legend class="underlay-fieldset__legend">{legend}</legend>
  {/if}
  <div class="underlay-fieldset__fields">
    {@render children?.()}
  </div>
</fieldset>

<style>
  .underlay-fieldset {
    border: none;
    padding: 0;
    margin: 0;
    min-width: 0;
    container-type: inline-size;
  }

  .underlay-fieldset__legend {
    font-weight: 500;
    font-size: 0.65rem;
    color: var(--underlay-color-text-muted, #6b7280);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    opacity: 0.5;
    margin-bottom: var(--underlay-space-3, 0.75rem);
    padding: 0;
  }

  .underlay-fieldset__fields {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
    grid-template-columns: 1fr;
    align-items: start;
  }

  /* Auto-fit columns based on available space */
  .underlay-fieldset:not(.underlay-fieldset--cols-1):not(.underlay-fieldset--cols-2):not(.underlay-fieldset--cols-3):not(.underlay-fieldset--cols-4):not(.underlay-fieldset--cols-6)
    .underlay-fieldset__fields {
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 14rem), 1fr));
  }

  /* Fixed column layouts using container queries */
  @container (min-width: 400px) {
    .underlay-fieldset--cols-2 .underlay-fieldset__fields {
      grid-template-columns: repeat(2, 1fr);
    }

    /* Progressive: 3+ columns collapse to 2 first */
    .underlay-fieldset--cols-3 .underlay-fieldset__fields,
    .underlay-fieldset--cols-4 .underlay-fieldset__fields {
      grid-template-columns: repeat(2, 1fr);
    }

    .underlay-fieldset--cols-6 .underlay-fieldset__fields {
      grid-template-columns: repeat(3, 1fr);
    }
  }

  /* Full column layouts at wider container widths */
  @container (min-width: 600px) {
    .underlay-fieldset--cols-3 .underlay-fieldset__fields {
      grid-template-columns: repeat(3, 1fr);
    }

    .underlay-fieldset--cols-4 .underlay-fieldset__fields {
      grid-template-columns: repeat(4, 1fr);
    }

    .underlay-fieldset--cols-6 .underlay-fieldset__fields {
      grid-template-columns: repeat(6, 1fr);
    }
  }
</style>
