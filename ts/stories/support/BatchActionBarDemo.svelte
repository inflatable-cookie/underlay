<script lang="ts">
  import BatchActionBar from "../../src/components/BatchActionBar.svelte";

  let selectedCount = $state(3);
</script>

<div class="batch-action-bar-demo">
  <BatchActionBar
    {selectedCount}
    totalCount={18}
    itemLabel="record"
    itemLabelPlural="records"
    registeredActions={[
      { id: "approve", label: "Approve", variant: "default" },
      { id: "flag", label: "Flag", variant: "warning" }
    ]}
    statusOptions={[
      { value: "draft", label: "Draft" },
      { value: "review", label: "In Review" },
      { value: "published", label: "Published" }
    ]}
    showStatusUpdate
    onClearSelection={() => {
      selectedCount = 0;
    }}
    onSelectAll={() => {
      selectedCount = 18;
    }}
    onBatchDelete={() => {
      selectedCount = 0;
    }}
    onBatchStatusUpdate={() => {}}
    onAction={() => {}}
  />
</div>

<style>
  .batch-action-bar-demo {
    min-height: 14rem;
    width: min(70rem, calc(100vw - 2rem));
  }
</style>
