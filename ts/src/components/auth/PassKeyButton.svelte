<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@poodle/svelte-primitives";

  import type { PassKeyStartPayload } from "./types";

  interface Props {
    label?: string;
    variant?: "primary" | "secondary" | "ghost";
    disabled?: boolean;
    loading?: boolean;
    class?: string;
    children?: Snippet;
    onStart?: (payload: PassKeyStartPayload) => void;
  }

  let {
    label = "Use a passkey",
    variant = "secondary",
    disabled = false,
    loading = false,
    class: className = "",
    children,
    onStart,
  }: Props = $props();

  function handleClick(event: CustomEvent<MouseEvent>) {
    if (disabled || loading) return;
    event.detail.preventDefault();
    onStart?.({ source: "button" });
  }
</script>

<Button
  type="button"
  {variant}
  className={`underlay-passkey-button ${className}`}
  disabled={disabled || loading}
  loading={loading}
  on:click={handleClick}
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
