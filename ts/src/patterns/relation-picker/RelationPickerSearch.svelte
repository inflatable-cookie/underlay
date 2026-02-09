<script lang="ts">
  import Search from "lucide-svelte/icons/search";
  import Loader from "lucide-svelte/icons/loader-circle";

  interface Props {
    placeholder: string;
    value: string;
    searching: boolean;
    onInput: (event: Event) => void;
    onKeyDown: (event: KeyboardEvent) => void;
    onInputRef: (input: HTMLInputElement | null) => void;
  }

  let {
    placeholder,
    value,
    searching,
    onInput,
    onKeyDown,
    onInputRef
  }: Props = $props();

  let inputRef: HTMLInputElement | null = null;

  $effect(() => {
    onInputRef(inputRef);
  });
</script>

<div class="relation-picker-dialog__search">
  <Search size="1.1em" class="relation-picker-dialog__search-icon" />
  <input
    bind:this={inputRef}
    type="text"
    class="relation-picker-dialog__search-input"
    {placeholder}
    {value}
    oninput={onInput}
    onkeydown={onKeyDown}
    aria-controls="relation-picker-list"
    aria-autocomplete="list"
  />
  {#if searching}
    <Loader size="1.1em" class="relation-picker-dialog__search-loader" />
  {/if}
</div>

<style>
  .relation-picker-dialog__search {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0 1rem 0.75rem;
    position: relative;
    flex-shrink: 0;
  }

  :global(.relation-picker-dialog__search-icon) {
    position: absolute;
    left: 1.6rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    pointer-events: none;
  }

  .relation-picker-dialog__search-input {
    width: 100%;
    padding: 0.55em 0.7em 0.55em 2.2em;
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
  }

  .relation-picker-dialog__search-input:focus {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-picker-dialog__search-input::placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.relation-picker-dialog__search-loader) {
    position: absolute;
    right: 1.6rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    animation: relation-picker-spin 1s linear infinite;
  }

  @keyframes relation-picker-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
