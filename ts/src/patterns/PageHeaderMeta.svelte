<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="underlay-page-header__meta-group">
  {@render children()}
</div>

<style>
  .underlay-page-header__meta-group {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }
</style>
