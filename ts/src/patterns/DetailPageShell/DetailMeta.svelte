<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="underlay-detail-meta">
  <div class="underlay-detail-meta__row">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .underlay-detail-meta {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .underlay-detail-meta__row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
</style>
