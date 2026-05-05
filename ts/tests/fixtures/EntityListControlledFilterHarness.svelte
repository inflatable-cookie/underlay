<script lang="ts">
  import { EntityList } from "../../src/templates";
  import type { QueryParams } from "../../src/client/query";

  interface Props {
    onQuery?: (query: QueryParams) => void;
  }

  let { onQuery }: Props = $props();

  let query = $state<QueryParams>({});

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
</script>

<EntityList
  {dataLoader}
  presentation="table"
  {columns}
  filters={[
    {
      id: "search",
      type: "search",
      label: "Search"
    }
  ]}
  {query}
  onQueryChange={(nextQuery) => (query = nextQuery)}
/>
