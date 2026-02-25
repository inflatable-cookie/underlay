<script lang="ts">
  import DetailList from "../../src/components/DetailList.svelte";

  interface Props {
    title?: string;
    className?: string;
    withChildren?: boolean;
  }

  let {
    title = undefined,
    className = "",
    withChildren = true
  }: Props = $props();
</script>

{#snippet childrenSnippet()}
  <div data-testid="detail-list-item-a">Item A</div>
  <div data-testid="detail-list-item-b">Item B</div>
{/snippet}

<DetailList title={title} class={className} children={withChildren ? childrenSnippet : undefined} />
