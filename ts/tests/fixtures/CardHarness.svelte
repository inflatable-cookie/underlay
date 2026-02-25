<script lang="ts">
  import Card from "../../src/components/Card.svelte";

  interface Props {
    as?: "section" | "article" | "div";
    className?: string;
    dataTestId?: string;
    title?: string;
  }

  let {
    as = "section",
    className = "",
    dataTestId = "card-root",
    title = "Card Title"
  }: Props = $props();
</script>

<Card {as} class={className} data-testid={dataTestId}>
  {#snippet children()}
    <h3 data-testid="card-title">{title}</h3>
  {/snippet}
</Card>
