<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import X from "lucide-svelte/icons/x";

  interface Props extends Omit<HTMLInputAttributes, "value" | "oninput" | "onchange" | "type"> {
    type?: string;
    value?: string;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    inputRef?: HTMLInputElement | null;
    oninput?: (value: string) => void;
    onchange?: (value: string) => void;
    /** Debounce delay in ms. When set, onchange fires after delay instead of on blur. */
    debounce?: number;
    /** Show a clear button when input has content. */
    search?: boolean;
  }

  let {
    type = "text",
    value = $bindable(""),
    autocomplete = "off",
    inputRef = $bindable(null),
    oninput,
    onchange,
    debounce,
    search = false,
    class: className,
    ...restProps
  }: Props = $props();

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  const showClearButton = $derived(search && value.length > 0);

  function handleInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement | null;
    const next = target ? target.value : value;
    value = next;
    oninput?.(next);

    // If debounce is enabled, fire onchange after delay
    if (debounce && debounce > 0 && onchange) {
      if (debounceTimer) {
        clearTimeout(debounceTimer);
      }
      debounceTimer = setTimeout(() => {
        onchange(next);
      }, debounce);
    }
  }

  function handleChange() {
    // Only fire onchange on blur if debounce is not enabled
    if (!debounce || debounce <= 0) {
      onchange?.(value);
    }
  }

  function handleClear() {
    // Clear debounce timer and fire onchange immediately with empty value
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    value = "";
    onchange?.("");
    inputRef?.focus();
  }
</script>

{#if search}
  <div class="underlay-input-wrapper">
    <input
      {...restProps}
      class={`underlay-input underlay-input--search ${className ?? ""}`}
      {type}
      {autocomplete}
      bind:this={inputRef}
      bind:value
      oninput={handleInput}
      onchange={handleChange}
    />
    {#if showClearButton}
      <button
        type="button"
        class="underlay-input-clear"
        aria-label="Clear"
        onclick={handleClear}
      >
        <X size="1em" strokeWidth={2.5} />
      </button>
    {/if}
  </div>
{:else}
  <input
    {...restProps}
    class={`underlay-input ${className ?? ""}`}
    {type}
    {autocomplete}
    bind:this={inputRef}
    bind:value
    oninput={handleInput}
    onchange={handleChange}
  />
{/if}

<style>
  .underlay-input-wrapper {
    position: relative;
    width: 100%;
  }

  .underlay-input {
    width: 100%;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, var(--underlay-field-padding-block, 0.55em))
      var(--underlay-field-padding-inline, var(--underlay-field-padding-inline, 0.7em));
    border-radius: var(--underlay-radius-sm, var(--underlay-radius-sm, 0.35rem));
    border: none;
    background: var(
      --underlay-color-field-bg,
      var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    font-size: var(--underlay-font-size-md, var(--underlay-font-size-md, 0.85rem));
  }

  .underlay-input--search {
    padding-right: 2.2em;
  }

  .underlay-input:focus,
  .underlay-input:focus-visible {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  .underlay-input-clear {
    position: absolute;
    right: 0.5em;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.15em;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 0.2rem;
    opacity: 0.7;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  .underlay-input-clear:hover {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-input-clear:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }
</style>
