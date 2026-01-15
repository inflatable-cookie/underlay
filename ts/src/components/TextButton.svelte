<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    type?: "button" | "submit" | "reset";
    variant?: "default" | "danger";
    href?: string | null;
    class?: string;
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    children?: Snippet;
  }

  let {
    type = "button",
    variant = "default",
    href = null,
    class: className = "",
    onclick,
    disabled = false,
    children,
  }: Props = $props();
</script>

{#if href}
  <a
    {href}
    class={`underlay-text-button underlay-text-button--${variant} ${className}`}
    aria-disabled={disabled || undefined}
  >
    {@render children?.()}
  </a>
{:else}
  <button
    class={`underlay-text-button underlay-text-button--${variant} ${className}`}
    {type}
    {onclick}
    {disabled}
  >
    {@render children?.()}
  </button>
{/if}

<style>
  :global(.underlay-text-button) {
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, var(--underlay-color-text-muted, inherit));
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.85));
    cursor: pointer;
    padding: 0;
    text-decoration: none;
  }

  :global(a.underlay-text-button) {
    display: inline;
  }

  :global(.underlay-text-button:focus-visible) {
    outline: var(--underlay-focus-ring-width, var(--underlay-focus-ring-width, 2px)) solid
      var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    outline-offset: var(--underlay-focus-ring-offset, var(--underlay-focus-ring-offset, 2px));
  }

  :global(.underlay-text-button:hover) {
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    text-decoration: underline;
  }

  :global(.underlay-text-button--danger) {
    color: var(--underlay-color-error, #ef4444);
  }

  :global(.underlay-text-button--danger:hover) {
    color: var(--underlay-color-error, #ef4444);
    text-decoration: underline;
    text-decoration-thickness: 2px;
  }

  :global(.underlay-text-button:disabled),
  :global(.underlay-text-button[aria-disabled="true"]) {
    opacity: 0.6;
    cursor: default;
    text-decoration: none;
  }
</style>
