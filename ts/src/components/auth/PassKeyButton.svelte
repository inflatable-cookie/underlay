<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { PassKeyStartPayload } from "./types";

  import Button from "../Button.svelte";

  const dispatch = createEventDispatcher<{ start: PassKeyStartPayload }>();

  export let label: string = "Use a passkey";
  export let variant: "primary" | "secondary" | "subtle" = "subtle";
  export let disabled: boolean = false;
  export let loading: boolean = false;
  export let className: string = "";

  function handleClick(event: CustomEvent<MouseEvent>) {
    if (disabled || loading) return;
    event.detail.preventDefault();
    dispatch("start", { source: "button" });
  }
</script>

<Button
  type="button"
  {variant}
  className={`underlay-passkey-button ${className}`}
  disabled={disabled || loading}
  aria-busy={loading}
  on:click={handleClick}
>
  <slot>{label}</slot>
</Button>

<style>
  :global(.underlay-passkey-button) {
    gap: 0.65em;
  }
</style>
