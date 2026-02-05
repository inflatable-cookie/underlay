<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="underlay-page-header__meta-row">
  {@render children()}
</div>

<style>
  .underlay-page-header__meta-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

</style>
