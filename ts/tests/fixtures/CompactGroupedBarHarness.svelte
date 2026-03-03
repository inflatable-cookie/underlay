<script lang="ts">
  import Button from "../../src/components/Button.svelte";
  import CompactGroupedBar from "../../src/components/CompactGroupedBar.svelte";
  import CompactGroupedBarGroup from "../../src/components/CompactGroupedBarGroup.svelte";
  import SegmentedControl from "../../src/components/SegmentedControl.svelte";

  let rangeValue = "last_24h";
  let outcomeValue = "all";
</script>

<CompactGroupedBar density="tight" separated>
  <CompactGroupedBarGroup label="Status" priority={40}>
    <Button type="button" size="sm" variant="subtle">Pinned</Button>
  </CompactGroupedBarGroup>

  <CompactGroupedBarGroup label="Range" priority={10} joined>
    <SegmentedControl
      ariaLabel="Range"
      size="xs"
      equalWidth={false}
      bind:value={rangeValue}
      options={[
        { label: "1h", value: "last_1h" },
        { label: "24h", value: "last_24h" }
      ]}
    />
  </CompactGroupedBarGroup>

  <CompactGroupedBarGroup label="Outcome" priority={15} joined>
    <SegmentedControl
      ariaLabel="Outcome"
      size="xs"
      equalWidth={false}
      bind:value={outcomeValue}
      options={[
        { label: "All", value: "all" },
        { label: "Failed", value: "failed" }
      ]}
    />
  </CompactGroupedBarGroup>
</CompactGroupedBar>
