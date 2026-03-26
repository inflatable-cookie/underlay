<script lang="ts">
  import type { Snippet } from "svelte";
  import { Callout } from "@poodle/svelte-primitives";
  import type { BannerVariant } from "./banner";

  interface Props {
    /** Visual style variant */
    variant?: BannerVariant;
    /** Main message to display */
    message: string;
    /** Optional additional content/actions */
    children?: Snippet;
  }

  let {
    variant = "warning",
    message,
    children
  }: Props = $props();
</script>

{#if children}
  <Callout tone={variant === "error" ? "danger" : variant} message={message} announceMode="polite">
    <svelte:fragment slot="actions">
      <div class="underlay-banner__actions">
        {@render children()}
      </div>
    </svelte:fragment>
  </Callout>
{:else}
  <Callout tone={variant === "error" ? "danger" : variant} message={message} announceMode="polite" />
{/if}

<style>
  .underlay-banner__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
