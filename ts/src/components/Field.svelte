<script lang="ts">
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
        <button
          type="button"
          class="underlay-field__hint"
          aria-label={hint}
          title={hint}
        >
          ?
        </button>
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

  .underlay-field__hint {
    border: 1px solid
      var(--underlay-color-border-subtle, var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.35)));
    border-radius: var(--underlay-radius-pill, var(--underlay-radius-pill, 999px));
    padding: 0;
    margin: 0;
    background: transparent;

    width: 1.1rem;
    height: 1.1rem;
    display: inline-flex;
    align-items: center;
    justify-content: center;

    cursor: help;
    color: var(--underlay-color-text-subtle, var(--underlay-color-text-subtle, #6b7280));
    font-size: 0.75em;
    font-weight: 700;
    line-height: 1;
  }

  .underlay-field__hint:hover {
    color: var(--underlay-color-text, var(--underlay-color-text, #e5e7eb));
    border-color: var(--underlay-color-border-strong, var(--underlay-color-border-strong, rgba(148, 163, 184, 0.6)));
    background: var(--underlay-color-hover-bg, var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.08)));
  }

  .underlay-field__error {
    color: var(--underlay-color-error, var(--underlay-color-error, #ef4444));
    font-size: var(--underlay-font-size-sm, var(--underlay-font-size-sm, 0.8rem));
    margin: 0;
  }
</style>
