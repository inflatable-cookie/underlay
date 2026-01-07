<script lang="ts">
  export let as: "section" | "article" | "div" = "section";
</script>

<svelte:element
  this={as}
  {...$$restProps}
  class={`underlay-card ${$$restProps.class ?? ""}`}
>
  <slot />
</svelte:element>

<style>
  .underlay-card {
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
