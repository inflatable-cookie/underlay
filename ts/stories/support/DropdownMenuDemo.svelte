<script lang="ts">
  import DropdownMenu from "../../src/components/DropdownMenu.svelte";
</script>

<DropdownMenu
  triggerLabel="Row actions"
  items={[
    { label: "Edit" },
    { label: "Duplicate" },
    { separator: true },
    { label: "Archive", disabled: true },
    { label: "Delete", destructive: true }
  ]}
/>
