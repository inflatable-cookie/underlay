<script lang="ts">
  import DetailsItem from "../../src/components/DetailsItem.svelte";

  interface Props {
    label?: string;
    value?: string | number | null;
    code?: boolean;
    span?: number | "full";
    muted?: boolean;
    className?: string;
    withChildren?: boolean;
  }

  let {
    label = "Identifier",
    value = "abc-123",
    code = false,
    span = undefined,
    muted = false,
    className = "",
    withChildren = false
  }: Props = $props();
</script>

{#snippet childrenSnippet()}
  <span data-testid="details-item-custom">Custom details</span>
{/snippet}

<DetailsItem
  {label}
  {value}
  {code}
  {span}
  {muted}
  class={className}
  children={withChildren ? childrenSnippet : undefined}
/>
