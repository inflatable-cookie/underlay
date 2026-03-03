<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    density?: "compact" | "tight";
    separated?: boolean;
    children?: Snippet;
  }

  let {
    class: className = "",
    density = "compact",
    separated = true,
    children
  }: Props = $props();
</script>

<div
  class={`underlay-compact-grouped-bar underlay-compact-grouped-bar--${density} ${className}`}
  class:underlay-compact-grouped-bar--separated={separated}
>
  {@render children?.()}
</div>

<style>
  .underlay-compact-grouped-bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.45rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.24));
    border-radius: var(--underlay-radius-sm, 0.45rem);
    background: var(--underlay-color-surface-muted, rgba(15, 23, 42, 0.16));
  }

  .underlay-compact-grouped-bar--tight {
    gap: 0.25rem;
    padding: 0.25rem 0.35rem;
  }

  .underlay-compact-grouped-bar--separated :global(> .underlay-compact-grouped-bar-group + .underlay-compact-grouped-bar-group) {
    border-left: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.22));
    padding-left: 0.4rem;
    margin-left: 0.05rem;
  }
</style>
