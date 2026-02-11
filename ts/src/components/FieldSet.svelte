<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Optional legend/title for the fieldset */
    legend?: string;
    /** Span full width in a parent grid layout */
    full?: boolean;
    /** Additional CSS class */
    class?: string;
    children?: Snippet;
  }

  let { legend, full = false, class: className = "", children }: Props = $props();

  const fullClass = $derived(full ? "underlay-fieldset--full" : "");
</script>

<fieldset class={`underlay-fieldset ${fullClass} ${className}`}>
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
  }

  .underlay-fieldset.underlay-fieldset--full {
    grid-column: 1 / -1;
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
    grid-template-columns: minmax(0, 1fr);
    align-items: start;
    min-width: 0;
  }

  .underlay-fieldset__fields > :global(*) {
    min-width: 0;
    max-width: 100%;
  }
</style>
