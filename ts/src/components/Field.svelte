<script lang="ts">
  import FieldHint from "./FieldHint.svelte";

  export let label: string | undefined = undefined;
  export let forId: string | undefined = undefined;
  export let hint: string | undefined = undefined;
  export let error: string | undefined = undefined;
</script>

<div class="underlay-field">
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

  <slot />
</div>

<style>
  .underlay-field {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .underlay-field__header {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-1, var(--underlay-space-1, 0.25rem));
  }

  .underlay-field__label {
    font-weight: 600;
    font-size: var(--underlay-font-size-xs, var(--underlay-font-size-xs, 0.75rem));
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, #9ca3af));
  }


  .underlay-field__error {
    color: var(--underlay-color-error, var(--underlay-color-error, #ef4444));
    font-size: var(--underlay-font-size-sm, var(--underlay-font-size-sm, 0.8rem));
    margin: 0;
  }
</style>
