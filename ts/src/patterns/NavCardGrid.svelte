<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children: Snippet;
  }

  let {
    children
  }: Props = $props();
</script>

<div class="underlay-nav-card-grid">
  {@render children()}
</div>

<style>
  .underlay-nav-card-grid {
    display: grid;
    gap: var(--underlay-space-4, 1rem);
    grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
  }

  @media (max-width: 480px) {
    .underlay-nav-card-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
