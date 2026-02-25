export type DataTableRowId = string | number;
export type DataTableRowIdGetter<T = unknown> = (row: T) => DataTableRowId;
