<script lang="ts">
  import type { Snippet } from "svelte";
  import RestoreResolutionDialog from "./RestoreResolutionDialog.svelte";

  interface Props {
    open?: boolean;
    mode?: "ready" | "blocked" | null;
    ready?: Snippet;
    blocked?: Snippet;
  }

  let {
    open = $bindable(false),
    mode = null,
    ready = undefined,
    blocked = undefined,
  }: Props = $props();
</script>

{#if mode}
  <RestoreResolutionDialog bind:open>
    {#if mode === "ready"}
      {@render ready?.()}
    {:else if mode === "blocked"}
      {@render blocked?.()}
    {/if}
  </RestoreResolutionDialog>
{/if}
