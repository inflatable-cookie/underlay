<script lang="ts">
  import Search from "lucide-svelte/icons/search";
  import Loader from "lucide-svelte/icons/loader-circle";

  interface Props {
    placeholder: string;
    value: string;
    showLoading: boolean;
    onInput: (event: Event) => void;
    onKeyDown: (event: KeyboardEvent) => void;
    onInputRef: (input: HTMLInputElement | null) => void;
  }

  let {
    placeholder,
    value,
    showLoading,
    onInput,
    onKeyDown,
    onInputRef
  }: Props = $props();

  let inputRef: HTMLInputElement | null = $state(null);

  $effect(() => {
    onInputRef(inputRef);
  });
</script>

<div class="relation-selector-popover__search">
  <Search size="1em" class="relation-selector-popover__search-icon" />
  <input
    bind:this={inputRef}
    type="text"
    class="relation-selector-popover__search-input"
    {placeholder}
    {value}
    oninput={onInput}
    onkeydown={onKeyDown}
    aria-controls="relation-selector-popover-list"
    aria-autocomplete="list"
  />
  {#if showLoading}
    <Loader size="1em" class="relation-selector-popover__search-loader" />
  {/if}
</div>

<style>
  .relation-selector-popover__search {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0 0.75rem 0.5rem;
    position: relative;
    flex-shrink: 0;
  }

  :global(.relation-selector-popover__search-icon) {
    position: absolute;
    left: 1.3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    pointer-events: none;
  }

  .relation-selector-popover__search-input {
    width: 100%;
    padding: 0.45em 0.6em 0.45em 1.9em;
    border-radius: 0.3rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.8rem;
  }

  .relation-selector-popover__search-input:focus {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-popover__search-input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.relation-selector-popover__search-loader) {
    position: absolute;
    right: 1.3rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
