<script lang="ts">
  import DataTable from "../../src/components/DataTable.svelte";
  import type {
    DataTableAction,
    DataTableColumn,
    DataTableFilters,
    DataTablePagination,
    DataTableSort
  } from "../../src/components/DataTable.svelte";

  type Row = {
    id: string;
    name: string;
    status: string;
  };

  interface Props {
    data?: Row[];
    columns?: DataTableColumn<Row>[];
    actions?: DataTableAction<Row>[];
    pagination?: DataTablePagination | null;
    sort?: DataTableSort | null;
    filters?: DataTableFilters;
    loading?: boolean;
    selectable?: boolean;
    showColumnToggle?: boolean;
    showExport?: boolean;
    showLimitSelector?: boolean;
    onSort?: (sort: DataTableSort) => void;
    onFilter?: (filters: DataTableFilters) => void;
    onPage?: (page: number) => void;
    onLimit?: (limit: number) => void;
    onSelect?: (selected: Row[]) => void;
    onAction?: (event: { action: string; row: Row }) => void;
    onExport?: (event: { data: Row[]; columns: DataTableColumn<Row>[] }) => void;
    onRowClick?: (row: Row) => void;
    withEmptySnippet?: boolean;
    withCellSnippet?: boolean;
    withExtendedRow?: boolean;
  }

  let {
    data = [
      { id: "1", name: "Ada", status: "active" },
      { id: "2", name: "Linus", status: "inactive" }
    ],
    columns = [
      { key: "name", label: "Name", sortable: true, filterable: true, hideable: true },
      { key: "status", label: "Status", sortable: false, filterable: false, hideable: true }
    ],
    actions = [],
    pagination = null,
    sort = null,
    filters = {},
    loading = false,
    selectable = false,
    showColumnToggle = false,
    showExport = false,
    showLimitSelector = true,
    onSort = undefined,
    onFilter = undefined,
    onPage = undefined,
    onLimit = undefined,
    onSelect = undefined,
    onAction = undefined,
    onExport = undefined,
    onRowClick = undefined,
    withEmptySnippet = false,
    withCellSnippet = false,
    withExtendedRow = false
  }: Props = $props();

  let selected = $state<Row[]>([]);
</script>

{#snippet emptySnippet()}
  <p data-testid="data-table-empty">Custom empty</p>
{/snippet}

{#snippet cellSnippet({ column, value })}
  <span data-testid={`cell-${column.key}`}>{value}</span>
{/snippet}

{#snippet extendedRowSnippet({ row })}
  <div data-testid={`extended-${row.id}`}>Extended: {row.name}</div>
{/snippet}

<p data-testid="selected-count">{selected.length}</p>

<DataTable
  {data}
  {columns}
  {actions}
  {pagination}
  {sort}
  {filters}
  {loading}
  {selectable}
  bind:selected
  {showColumnToggle}
  {showExport}
  {showLimitSelector}
  {onSort}
  {onFilter}
  {onPage}
  {onLimit}
  {onSelect}
  {onAction}
  {onExport}
  {onRowClick}
  empty={withEmptySnippet ? emptySnippet : undefined}
  cell={withCellSnippet ? cellSnippet : undefined}
  extendedRow={withExtendedRow ? extendedRowSnippet : undefined}
  extendedRowWhen={(row) => row.status === "active"}
/>
