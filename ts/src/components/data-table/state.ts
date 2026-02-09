interface ColumnLike<T> {
	key: string;
	width?: string;
	minWidth?: string;
	formatter?: (value: unknown, row: T) => string;
}

interface SortLike {
	key: string;
	direction: "asc" | "desc";
}

export function getCellDisplayValue<T extends object>(row: T, column: ColumnLike<T>): string {
	const keys = column.key.split(".");
	let value: unknown = row;
	for (const key of keys) {
		if (value && typeof value === "object") {
			value = (value as Record<string, unknown>)[key];
		} else {
			value = undefined;
			break;
		}
	}

	if (column.formatter) {
		return column.formatter(value, row);
	}

	if (value == null) return "";
	if (value instanceof Date) return value.toLocaleDateString();
	return String(value);
}

export function getNextSort(
	currentSort: SortLike | null | undefined,
	columnKey: string
): SortLike {
	return {
		key: columnKey,
		direction:
			currentSort?.key === columnKey && currentSort.direction === "asc" ? "desc" : "asc"
	};
}

export function updateFilters(
	currentFilters: Record<string, string>,
	key: string,
	value: string
): Record<string, string> {
	return { ...currentFilters, [key]: value };
}

export function toggleHiddenColumn(
	currentHiddenColumns: Set<string>,
	key: string
): Set<string> {
	const next = new Set(currentHiddenColumns);
	if (next.has(key)) {
		next.delete(key);
	} else {
		next.add(key);
	}
	return next;
}

export function getColumnWidthValue<T>(column: ColumnLike<T>): string {
	if (column.width) {
		if (column.minWidth) {
			return `minmax(${column.minWidth}, ${column.width})`;
		}
		return column.width;
	}
	const minWidth = column.minWidth ?? "100px";
	return `minmax(${minWidth}, 1fr)`;
}
