export function getNextPage(newPage: number, totalPages: number): number | null {
	if (newPage < 1 || newPage > totalPages) return null;
	return newPage;
}

export function getPageAfterLimitChange(): number {
	return 1;
}

export function toggleSelectAllRows<T>(
	data: T[],
	selected: T[],
	allSelected: boolean
): T[] {
	if (allSelected) {
		return [];
	}
	return [...data];
}

export function toggleRowSelection<T>(selected: T[], row: T): T[] {
	const index = selected.indexOf(row);
	if (index >= 0) {
		return [...selected.slice(0, index), ...selected.slice(index + 1)];
	}
	return [...selected, row];
}
