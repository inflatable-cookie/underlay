<script lang="ts">
  import DetailItem from "../../src/components/DetailItem.svelte";

  interface Props {
    label?: string;
    value?: string | number | boolean | null;
    code?: boolean;
    capitalize?: boolean;
    className?: string;
    withChildren?: boolean;
  }

  let {
    label = "Status",
    value = "active",
    code = false,
    capitalize = false,
    className = "",
    withChildren = false
  }: Props = $props();
</script>

{#snippet childrenSnippet()}
  <span data-testid="detail-item-custom">Custom value</span>
{/snippet}

<DetailItem
  {label}
  {value}
  {code}
  {capitalize}
  class={className}
  children={withChildren ? childrenSnippet : undefined}
/>
