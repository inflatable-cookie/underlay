<script lang="ts">
  import FieldSet from "../../src/components/FieldSet.svelte";

  interface Props {
    legend?: string;
    full?: boolean;
    className?: string;
  }

  let {
    legend = undefined,
    full = false,
    className = ""
  }: Props = $props();
</script>

<FieldSet {legend} {full} class={className}>
  {#snippet children()}
    <div data-testid="fieldset-child">Child</div>
  {/snippet}
</FieldSet>
