<script lang="ts">
  import type { Snippet } from "svelte";

  import type { PassKeyStartPayload } from "./types";

  import Button from "../Button.svelte";

  interface Props {
    label?: string;
    variant?: "primary" | "secondary" | "subtle";
    disabled?: boolean;
    loading?: boolean;
    class?: string;
    children?: Snippet;
    onStart?: (payload: PassKeyStartPayload) => void;
  }

  let {
    label = "Use a passkey",
    variant = "subtle",
    disabled = false,
    loading = false,
    class: className = "",
    children,
    onStart,
  }: Props = $props();

  function handleClick(event: MouseEvent) {
    if (disabled || loading) return;
    event.preventDefault();
    onStart?.({ source: "button" });
  }
</script>

<Button
  type="button"
  {variant}
  class={`underlay-passkey-button ${className}`}
  disabled={disabled || loading}
  aria-busy={loading}
  onclick={handleClick}
>
  {#if children}
    {@render children()}
  {:else}
    {label}
  {/if}
</Button>

<style>
  :global(.underlay-passkey-button) {
    gap: 0.65em;
  }
</style>
