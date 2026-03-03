<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    title?: string;
    class?: string;
    onchange?: (event: Event) => void;
    children?: Snippet;
  }

  let {
    checked = $bindable(false),
    disabled = false,
    title,
    class: className = "",
    onchange,
    children
  }: Props = $props();

  function handleChange(event: Event) {
    onchange?.(event);
  }
</script>

<label class={`underlay-checkbox-chip ${className}`} {title}>
  <input type="checkbox" bind:checked {disabled} onchange={handleChange} />
  <span class="underlay-checkbox-chip__label">
    {@render children?.()}
  </span>
</label>

<style>
  .underlay-checkbox-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.76rem;
    color: var(--underlay-color-text-muted, #94a3b8);
    padding: 0.2rem 0.4rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: 0.45rem;
  }

  .underlay-checkbox-chip input {
    margin: 0;
  }

  .underlay-checkbox-chip__label {
    line-height: 1.2;
  }

  .underlay-checkbox-chip:has(input:disabled) {
    opacity: 0.6;
  }
</style>
