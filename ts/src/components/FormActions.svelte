<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    align?: "start" | "end";
    children?: Snippet;
    danger?: Snippet;
  }

  let { align = "start", children, danger }: Props = $props();
</script>

<div class={`underlay-form-actions underlay-form-actions--${align}`}>
  {@render children?.()}
  {#if danger}
    <div class="underlay-form-actions__danger">
      {@render danger()}
    </div>
  {/if}
</div>

<style>
  .underlay-form-actions {
    margin-top: calc(
      var(--underlay-density-gap, 0.75rem) * 1.5
    );
    display: flex;
    flex-wrap: wrap;
    gap: var(--underlay-form-actions-gap, 1.5rem);
    align-items: center;
  }

  .underlay-form-actions--start {
    justify-content: flex-start;
  }

  .underlay-form-actions--end {
    justify-content: flex-end;
  }

  .underlay-form-actions__danger {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: var(--underlay-form-actions-gap, 1.5rem);
  }
</style>
