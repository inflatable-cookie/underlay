import type {
  DataTableAction,
  DataTableColumn,
  DataTableFilters,
  DataTableSort
} from "../DataTable.svelte";
import {
  getNextPage,
  getPageAfterLimitChange,
  toggleRowSelection,
  toggleSelectAllRows
} from "./pagination-selection";
import { getNextSort, updateFilters } from "./state";

export function getNextSortState(
  currentSort: DataTableSort | null,
  column: DataTableColumn<any>,
): DataTableSort {
  return getNextSort(currentSort, column.key);
}

export function getNextFiltersState(
  currentFilters: DataTableFilters,
  key: string,
  value: string
): DataTableFilters {
  return updateFilters(currentFilters, key, value);
}

export function emitNextPage(
  newPage: number,
  totalPages: number,
  onPage?: (page: number) => void
): void {
  const nextPage = getNextPage(newPage, totalPages);
  if (nextPage == null) return;
  onPage?.(nextPage);
}

export function emitLimitChange(
  newLimit: number,
  onLimit?: (limit: number) => void,
  onPage?: (page: number) => void
): void {
  onLimit?.(newLimit);
  onPage?.(getPageAfterLimitChange());
}

export function applySelectAll<T>(
  data: T[],
  selected: T[],
  allSelected: boolean,
  onSelect?: (selected: T[]) => void
): T[] {
  const nextSelected = toggleSelectAllRows(data, selected, allSelected);
  onSelect?.(nextSelected);
  return nextSelected;
}

export function applySelectRow<T>(
  selected: T[],
  row: T,
  onSelect?: (selected: T[]) => void
): T[] {
  const nextSelected = toggleRowSelection(selected, row);
  onSelect?.(nextSelected);
  return nextSelected;
}

export function runRowAction<T>(
  action: DataTableAction<T>,
  row: T,
  onAction?: (event: { action: string; row: T }) => void,
  confirmFn: (message?: string) => boolean = () => true
): void {
  if (action.disabled || action.separator) return;
  if (action.confirm && !confirmFn(action.confirm)) return;

  action.onClick?.(row);
  onAction?.({ action: action.label, row });
}
