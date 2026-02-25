<script lang="ts">
  import OpsCard from "../../src/patterns/OpsCard.svelte";

  interface Props {
    title?: string;
    class?: string;
    withBody?: boolean;
  }

  let {
    title = "Card title",
    class: className = "",
    withBody = true
  }: Props = $props();
</script>

{#snippet bodySnippet()}
  <p data-testid="ops-card-body">Body content</p>
{/snippet}

<OpsCard {title} class={className} children={withBody ? bodySnippet : undefined} />
