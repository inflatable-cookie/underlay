<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    message: string;
    empty?: Snippet;
  }

  let {
    message,
    empty
  }: Props = $props();
</script>

<div class="empty-state" role="row">
  {#if empty}
    {@render empty()}
  {:else}
    <p>{message}</p>
  {/if}
</div>

<style>
  .empty-state {
    padding: 3rem;
    text-align: center;
    color: var(--color-text-muted, #64748b);
    grid-column: 1 / -1;
  }
</style>
