<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    minItemWidthPx?: number | null;
    gapPx?: number | null;
    children?: Snippet;
  }

  let {
    minItemWidthPx = null,
    gapPx = null,
    children
  }: Props = $props();

  let style = $derived(
    [
      minItemWidthPx == null ? null : `--underlay-list-grid-min: ${minItemWidthPx}px;`,
      gapPx == null ? null : `--underlay-list-grid-gap: ${gapPx}px;`
    ]
      .filter(Boolean)
      .join(" ")
  );
</script>

<div class="underlay-list-grid" style={style || undefined}>
  {#if children}
    {@render children()}
  {/if}
</div>

<style>
  .underlay-list-grid {
    display: grid;
    grid-template-columns: repeat(
      auto-fit,
      minmax(
        var(--underlay-list-grid-min, var(--underlay-list-grid-min, 360px)),
        1fr
      )
    );
    gap: var(--underlay-list-grid-gap, var(--underlay-list-grid-gap, 14px));
    align-items: stretch;
  }
</style>
