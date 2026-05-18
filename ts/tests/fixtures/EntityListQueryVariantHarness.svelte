<script lang="ts">
  import { EntityList } from "../../src/templates";
  import type { QueryParams } from "../../src/client/query";

  interface Props {
    onQuery?: (query: QueryParams) => void;
    useCapabilities?: boolean;
  }

  let { onQuery, useCapabilities = false }: Props = $props();

  let query = $state<QueryParams>({ page: 3 });

  const columns = [
    { id: "name", label: "Name" }
  ];

  const rows = [
    { id: "1", name: "Alpha" },
    { id: "2", name: "Beta" }
  ];

  async function dataLoader(_fetch: typeof fetch, _token: string | null, currentQuery: QueryParams) {
    onQuery?.(currentQuery);
    return {
      data: rows,
      total: rows.length
    };
  }

  async function capabilitiesLoader() {
    return {
      defaultVariantId: "marked",
      variants: [
        { id: "pending", label: "Pending", count: 4 },
        { id: "marked", label: "Marked", count: 12 },
        { id: "all", label: "All", count: 16 }
      ],
      filters: []
    };
  }
</script>

<EntityList
  {dataLoader}
  presentation="table"
  {columns}
  queryVariants={[
    { id: "pending", label: "Pending", count: 4, isDefault: true },
    { id: "marked", label: "Marked", count: 12 },
    { id: "all", label: "All", count: 16 }
  ]}
  capabilitiesLoader={useCapabilities ? capabilitiesLoader : undefined}
  {query}
  onQueryChange={(nextQuery) => (query = nextQuery)}
/>
