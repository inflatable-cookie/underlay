<script lang="ts">
  import { EntityList } from "../../src/templates";

  const rows = [
    { id: "1", name: "Alpha" },
    { id: "2", name: "Beta" },
    { id: "3", name: "Gamma" },
    { id: "4", name: "Delta" },
    { id: "5", name: "Epsilon" }
  ];

  async function dataLoader() {
    return {
      data: rows
    };
  }
</script>

<EntityList
  {dataLoader}
  presentation="table"
  columns={[{ id: "name", label: "Name" }]}
  filters={[
    {
      id: "search",
      type: "search",
      label: "Search"
    }
  ]}
/>
