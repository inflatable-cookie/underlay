<script lang="ts">
  import ListGrid from "../../src/components/ListGrid.svelte";

  interface Props {
    variant?: "default" | "compact";
    minItemWidth?: number | string | null;
    gap?: number | string | null;
    withActions?: boolean;
    withChildren?: boolean;
  }

  let {
    variant = "default",
    minItemWidth = null,
    gap = null,
    withActions = true,
    withChildren = true
  }: Props = $props();
</script>

{#snippet actionsSnippet()}
  <button type="button" data-testid="list-grid-action">Action</button>
{/snippet}

{#snippet childrenSnippet()}
  <div data-testid="list-grid-item-a">A</div>
  <div data-testid="list-grid-item-b">B</div>
{/snippet}

<ListGrid
  {variant}
  {minItemWidth}
  {gap}
  actions={withActions ? actionsSnippet : undefined}
  children={withChildren ? childrenSnippet : undefined}
/>
