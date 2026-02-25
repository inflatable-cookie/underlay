<script lang="ts">
  import DetailsCard from "../../src/components/DetailsCard.svelte";

  interface Props {
    className?: string;
    text?: string;
  }

  let {
    className = "",
    text = "Card detail"
  }: Props = $props();
</script>

<DetailsCard class={className}>
  <p data-testid="details-card-content">{text}</p>
</DetailsCard>
