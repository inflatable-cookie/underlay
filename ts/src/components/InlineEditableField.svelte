<script lang="ts">
  import type { ValidationResult } from "./TextInput.svelte";

  interface Props {
    /** Current display value */
    value: string;
    /** Async save callback */
    onSave: (newValue: string) => Promise<void>;
    /** Optional validation */
    validate?: (value: string) => ValidationResult;
    /** Placeholder text when editing */
    placeholder?: string;
    /** Input type */
    inputType?: "text" | "number";
    /** Disabled state */
    disabled?: boolean;
    /** Additional CSS class */
    class?: string;
  }

  let {
    value,
    onSave,
    validate = undefined,
    placeholder = "",
    inputType = "text",
    disabled = false,
    class: className = ""
  }: Props = $props();

  type Mode = "display" | "edit" | "saving";
  let mode = $state<Mode>("display");
  let editValue = $state("");
  let errorMessage = $state<string | null>(null);
  let inputEl = $state<HTMLInputElement | null>(null);

  function enterEdit() {
    if (disabled) return;
    editValue = value;
    errorMessage = null;
    mode = "edit";
    // Focus after DOM update
    requestAnimationFrame(() => {
      inputEl?.focus();
      inputEl?.select();
    });
  }

  function cancelEdit() {
    mode = "display";
    editValue = "";
    errorMessage = null;
  }

  async function saveEdit() {
    if (editValue === value) {
      cancelEdit();
      return;
    }

    if (validate) {
      const result = validate(editValue);
      if (!result.valid) {
        errorMessage = result.message ?? "Invalid value";
        return;
      }
    }

    errorMessage = null;
    mode = "saving";

    try {
      await onSave(editValue);
      mode = "display";
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : "Save failed";
      mode = "edit";
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (mode !== "edit") return;
    if (event.key === "Enter") {
      event.preventDefault();
      saveEdit();
    } else if (event.key === "Escape") {
      event.preventDefault();
      cancelEdit();
    }
  }

  function handleDisplayKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      enterEdit();
    }
  }
</script>

<div class="underlay-inline-editable {className}">
  {#if mode === "display"}
    <div
      class="underlay-inline-editable__display"
      class:underlay-inline-editable__display--disabled={disabled}
      onclick={enterEdit}
      onkeydown={handleDisplayKeydown}
      role="button"
      tabindex={disabled ? -1 : 0}
    >
      <span class="underlay-inline-editable__text">{value || placeholder}</span>
      {#if !disabled}
        <svg class="underlay-inline-editable__pencil" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" /><path d="m15 5 4 4" />
        </svg>
      {/if}
    </div>
  {:else}
    <div class="underlay-inline-editable__edit">
      <div class="underlay-inline-editable__input-row">
        <input
          bind:this={inputEl}
          type={inputType}
          bind:value={editValue}
          onkeydown={handleKeydown}
          {placeholder}
          disabled={mode === "saving"}
          class="underlay-inline-editable__input"
        />
        {#if mode === "saving"}
          <div class="underlay-inline-editable__spinner" aria-label="Saving">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12a9 9 0 1 1-6.219-8.56" />
            </svg>
          </div>
        {:else}
          <button
            class="underlay-inline-editable__btn underlay-inline-editable__btn--save"
            onclick={saveEdit}
            type="button"
            aria-label="Save"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M20 6 9 17l-5-5" />
            </svg>
          </button>
          <button
            class="underlay-inline-editable__btn underlay-inline-editable__btn--cancel"
            onclick={cancelEdit}
            type="button"
            aria-label="Cancel"
          >
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M18 6 6 18" /><path d="m6 6 12 12" />
            </svg>
          </button>
        {/if}
      </div>
      {#if errorMessage}
        <p class="underlay-inline-editable__error" role="alert">{errorMessage}</p>
      {/if}
    </div>
  {/if}
</div>

<style>
  :global(.underlay-inline-editable) {
    display: inline-flex;
    flex-direction: column;
    min-width: 0;
  }

  :global(.underlay-inline-editable__display) {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    cursor: pointer;
    padding: 0.125rem 0.25rem;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    transition: background-color 0.12s ease;
  }

  :global(.underlay-inline-editable__display:hover) {
    background-color: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
  }

  :global(.underlay-inline-editable__display:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 2px;
  }

  :global(.underlay-inline-editable__display--disabled) {
    cursor: default;
    opacity: 0.6;
  }

  :global(.underlay-inline-editable__display--disabled:hover) {
    background-color: transparent;
  }

  :global(.underlay-inline-editable__text) {
    color: var(--underlay-color-text, inherit);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.underlay-inline-editable__pencil) {
    flex-shrink: 0;
    color: var(--underlay-color-text-muted, #64748b);
    opacity: 0;
    transition: opacity 0.12s ease;
  }

  :global(.underlay-inline-editable__display:hover .underlay-inline-editable__pencil),
  :global(.underlay-inline-editable__display:focus-visible .underlay-inline-editable__pencil) {
    opacity: 1;
  }

  :global(.underlay-inline-editable__edit) {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  :global(.underlay-inline-editable__input-row) {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  :global(.underlay-inline-editable__input) {
    flex: 1;
    min-width: 0;
    padding: 0.25rem 0.5rem;
    font-size: inherit;
    font-family: inherit;
    color: var(--underlay-color-text, inherit);
    background-color: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    border: 1px solid var(--underlay-color-border, rgba(148, 163, 184, 0.12));
    border-radius: var(--underlay-radius-sm, 0.25rem);
    outline: none;
  }

  :global(.underlay-inline-editable__input:focus) {
    border-color: var(--underlay-color-primary, #2563eb);
    box-shadow: 0 0 0 2px rgba(37, 99, 235, 0.2);
  }

  :global(.underlay-inline-editable__input:disabled) {
    opacity: 0.6;
  }

  :global(.underlay-inline-editable__btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    background: transparent;
    cursor: pointer;
    flex-shrink: 0;
    transition: background-color 0.12s ease, color 0.12s ease;
  }

  :global(.underlay-inline-editable__btn--save) {
    color: var(--underlay-color-success, #22c55e);
  }

  :global(.underlay-inline-editable__btn--save:hover) {
    background-color: rgba(34, 197, 94, 0.15);
  }

  :global(.underlay-inline-editable__btn--cancel) {
    color: var(--underlay-color-text-muted, #64748b);
  }

  :global(.underlay-inline-editable__btn--cancel:hover) {
    background-color: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
  }

  :global(.underlay-inline-editable__btn:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 2px;
  }

  :global(.underlay-inline-editable__spinner) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    flex-shrink: 0;
    color: var(--underlay-color-text-muted, #64748b);
    animation: underlay-inline-editable-spin 1s linear infinite;
  }

  @keyframes underlay-inline-editable-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  :global(.underlay-inline-editable__error) {
    margin: 0;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    color: var(--underlay-color-error, #ef4444);
  }

  @media (prefers-reduced-motion: reduce) {
    :global(.underlay-inline-editable__spinner) {
      animation: none;
    }
  }
</style>
