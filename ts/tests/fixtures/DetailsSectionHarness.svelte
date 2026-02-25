<script lang="ts">
  import DetailsSection from "../../src/components/DetailsSection.svelte";

  interface Props {
    legend?: string;
    className?: string;
  }

  let {
    legend = undefined,
    className = ""
  }: Props = $props();
</script>

<DetailsSection {legend} class={className}>
  <div data-testid="details-section-item-a">A</div>
  <div data-testid="details-section-item-b">B</div>
</DetailsSection>
