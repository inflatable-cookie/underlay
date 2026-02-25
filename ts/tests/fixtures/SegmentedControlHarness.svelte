<script lang="ts">
  import SegmentedControl from "../../src/components/SegmentedControl.svelte";

  interface Props {
    name?: string;
    value?: string;
    disabled?: boolean;
    ariaLabel?: string;
  }

  let {
    name = undefined,
    value = "one",
    disabled = false,
    ariaLabel = "Mode"
  }: Props = $props();

  const options = [
    { label: "One", value: "one" },
    { label: "Two", value: "two" },
    { label: "Three", value: "three" }
  ];
</script>

<SegmentedControl {name} bind:value {disabled} {ariaLabel} {options} />
<p data-testid="segmented-value">{value}</p>
