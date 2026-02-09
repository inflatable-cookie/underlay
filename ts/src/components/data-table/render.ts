import type { DataTableAction, DataTableColumn } from "../DataTable.svelte";
import { getRowActionHref, getVisibleRowActions } from "./actions";
import { getCellDisplayValue, getColumnWidthValue } from "./state";

export function getRenderedCellValue<T extends object>(
  row: T,
  column: DataTableColumn<T>
): string {
  return getCellDisplayValue(row, column);
}

export function getRenderedRowActions<T extends object>(
  row: T,
  actions: DataTableAction<T>[] | ((row: T) => DataTableAction<T>[])
): DataTableAction<T>[] {
  return getVisibleRowActions(row, actions) as DataTableAction<T>[];
}

export function getRenderedActionHref<T extends object>(
  action: DataTableAction<T>,
  row: T
): string | undefined {
  return getRowActionHref(action, row);
}

export function buildGridColumns<T extends object>(
  selectable: boolean,
  visibleColumns: DataTableColumn<T>[],
  actions: DataTableAction<T>[] | ((row: T) => DataTableAction<T>[])
): string {
  return [
    selectable ? "40px" : null,
    ...visibleColumns.map((column) => getColumnWidthValue(column)),
    actions.length > 0 || typeof actions === "function" ? "80px" : null
  ]
    .filter(Boolean)
    .join(" ");
}
