<script lang="ts">
  import type { Snippet } from "svelte";
  import FieldHint from "./FieldHint.svelte";

  interface Props {
    label?: string;
    forId?: string;
    hint?: string;
    error?: string;
    /** Grid column span: "full" spans all columns, number specifies exact span */
    span?: "full" | 1 | 2 | 3 | 4 | 5 | 6;
    /** Remove max-width constraint for wide content like editors */
    wide?: boolean;
    children?: Snippet;
  }

  let { label, forId, hint, error, span, wide = false, children }: Props = $props();

  const spanStyle = span === "full"
    ? "grid-column: 1 / -1;"
    : span
      ? `grid-column: span ${span};`
      : "";
</script>

<div class="underlay-field" class:underlay-field--wide={wide} style={spanStyle}>
  {#if label}
    <div class="underlay-field__header">
      <label class="underlay-field__label" for={forId}>
        {label}
      </label>

      {#if hint}
        <FieldHint content={hint} />
      {/if}
    </div>
  {/if}

  {#if error}
    <p class="underlay-field__error">{error}</p>
  {/if}

  <div class="underlay-field__content">
    {@render children?.()}
  </div>
</div>

<style>
  .underlay-field {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
    max-width: var(--underlay-field-max-width, 52rem);
  }

  .underlay-field.underlay-field--wide {
    max-width: none;
  }

  .underlay-field__header {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .underlay-field__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .underlay-field__label {
    font-weight: 600;
    font-size: var(--underlay-font-size-xs, var(--underlay-font-size-xs, 0.75rem));
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }


  .underlay-field__error {
    color: var(--underlay-color-error, var(--underlay-color-error, #ef4444));
    font-size: var(--underlay-font-size-sm, var(--underlay-font-size-sm, 0.8rem));
    margin: 0;
  }
</style>
