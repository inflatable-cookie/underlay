import type {
  TableCellValue,
  TableRow
} from "@inflatable-cookie/poodle-svelte";
import type { PagedListResult } from "./template.types";

/// Page size for user tab lists that show a bounded recent slice (the common
/// consumer posture: fixed limit, no pagination controls).
export const USER_TAB_LIST_FIXED_LIMIT = 10;

/// Page size for user tab lists mounted in server-paginated mode.
export const USER_TAB_LIST_PAGED_LIMIT = 20;

export function buildUserTabRows<TItem extends { id: string }>(
  items: TItem[],
  toCells: (item: TItem) => Record<string, TableCellValue>
): TableRow<TItem>[] {
  return items.map((item) => ({
    id: item.id,
    cells: toCells(item),
    data: item
  }));
}

export function resolveUserTabCount<TItem>(
  result: PagedListResult<TItem> | undefined,
  visibleCount: number
): number {
  const total = result?.total;
  if (typeof total === "number" && Number.isFinite(total) && total >= 0) {
    return total;
  }
  return visibleCount;
}
