import type {
  DataTableColumn,
  DataTablePagination
} from "../DataTable.svelte";

export function getVisibleColumns<T extends object>(
  columns: DataTableColumn<T>[],
  hiddenColumns: Set<string>
): DataTableColumn<T>[] {
  return columns.filter((column) => !hiddenColumns.has(column.key));
}

export function getHideableColumns<T extends object>(
  columns: DataTableColumn<T>[]
): DataTableColumn<T>[] {
  return columns.filter((column) => column.hideable !== false);
}

export function getTotalPages(pagination: DataTablePagination | null): number {
  if (!pagination) return 1;
  return Math.ceil(pagination.total / pagination.limit);
}

export function isAllSelected(dataLength: number, selectedLength: number): boolean {
  return dataLength > 0 && selectedLength === dataLength;
}

export function isSomeSelected(dataLength: number, selectedLength: number): boolean {
  return selectedLength > 0 && selectedLength < dataLength;
}
