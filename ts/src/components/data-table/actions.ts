interface RowAction<T> {
	href?: string | ((row: T) => string);
	show?: (row: T) => boolean;
}

export function getVisibleRowActions<T>(
	row: T,
	actions: RowAction<T>[] | ((row: T) => RowAction<T>[])
): RowAction<T>[] {
	const rowActions = typeof actions === "function" ? actions(row) : actions;
	return rowActions.filter((action) => !action.show || action.show(row));
}

export function getRowActionHref<T>(action: RowAction<T>, row: T): string | undefined {
	if (!action.href) return undefined;
	return typeof action.href === "function" ? action.href(row) : action.href;
}
