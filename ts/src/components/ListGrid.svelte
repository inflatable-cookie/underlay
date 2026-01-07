<script lang="ts">
  export let minItemWidthPx: number | null = null;
  export let gapPx: number | null = null;

  $: style = [
    minItemWidthPx == null ? null : `--underlay-list-grid-min: ${minItemWidthPx}px;`,
    gapPx == null ? null : `--underlay-list-grid-gap: ${gapPx}px;`
  ]
    .filter(Boolean)
    .join(" ");
</script>

<div class="underlay-list-grid" style={style || undefined}>
  <slot />
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
