<script lang="ts">
  import SegmentedControl from "../../src/components/SegmentedControl.svelte";

  interface Props {
    name?: string;
    value?: string;
    disabled?: boolean;
    ariaLabel?: string;
    options?: Array<{ label: string; value: string }>;
  }

  let {
    name = "mode",
    value = "",
    disabled = false,
    ariaLabel = "Mode",
    options = []
  }: Props = $props();
</script>

<SegmentedControl {name} bind:value {disabled} {ariaLabel} {options} />
<p data-testid="segmented-edge-value">{value}</p>
