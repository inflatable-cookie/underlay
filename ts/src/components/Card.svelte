<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    as?: "section" | "article" | "div";
    class?: string;
    children?: Snippet;
    [key: string]: unknown;
  }

  let { as = "section", class: className = "", children, ...restProps }: Props = $props();
</script>

<svelte:element
  this={as}
  {...restProps}
  class={`underlay-card ${className}`}
>
  {@render children?.()}
</svelte:element>

<style>
  .underlay-card {
    container-type: inline-size;
    border-radius: var(--underlay-radius-md, var(--underlay-radius-md, 0.75rem));
    padding: var(--underlay-card-padding, var(--underlay-density-gap, 1rem));
    background-color: var(
      --underlay-color-surface-muted,
      var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02))
    );
    color: var(--underlay-color-text, var(--underlay-color-text, inherit));
    box-shadow: var(--underlay-shadow-card, var(--underlay-shadow-card, none));
    border: 1px solid
      var(
        --underlay-color-border-subtle,
        var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25))
      );
  }
</style>
