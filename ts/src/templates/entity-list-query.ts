import type { FilterField, QueryParams, SortField } from "../client/query";
import { DEFAULT_PAGE_SIZE } from "../patterns/pagination-types";
import type { FilterConfig } from "./template.types";

export function normalizeListQuery(query: QueryParams): QueryParams {
	return {
		...query,
		variant: query.variant ?? undefined,
		page: Math.max(1, query.page ?? 1),
		limit: Math.max(1, query.limit ?? DEFAULT_PAGE_SIZE),
		filters:
			query.filters?.filter(({ value }) => {
				const normalizedValue = value.trim();
				return normalizedValue !== "" && normalizedValue !== "All";
			}) ?? [],
		sort: query.sort?.filter(({ field }) => field.trim() !== "") ?? [],
	};
}

export function listQueryKey(query: QueryParams, reloadKey?: string | number): string {
	return JSON.stringify([
		query.variant,
		query.page,
		query.limit,
		query.filters?.map(({ field, operator, value }) => [field, operator, value]),
		query.sort?.map(({ field, direction }) => [field, direction]),
		reloadKey,
	]);
}

export function getListFilterValue(query: QueryParams, filter: FilterConfig): string {
	const activeFilter = query.filters?.find((entry) => entry.field === filter.id);
	if (!activeFilter) return "";

	if (filter.type === "search" && activeFilter.operator === "like") {
		return activeFilter.value.replace(/^%|%$/g, "");
	}
	if (filter.type === "select" && activeFilter.value === "All") return "";
	return activeFilter.value;
}

export function updateListFilters(
	query: QueryParams,
	filter: FilterConfig,
	value: string,
): FilterField[] {
	const filters = (query.filters ?? []).filter((entry) => entry.field !== filter.id);
	if (filter.type === "search") {
		const search = value.trim();
		return search
			? [...filters, { field: filter.id, operator: "like", value: `%${search}%` }]
			: filters;
	}
	return value ? [...filters, { field: filter.id, value }] : filters;
}

export function isListFilterActive(query: QueryParams, filter: FilterConfig): boolean {
	if (filter.type === "sort") return Boolean(query.sort?.length);
	const value = getListFilterValue(query, filter);
	return filter.type === "select" ? value !== "" : value.trim() !== "";
}

export function sortFieldsEqual(left: SortField[], right: SortField[]): boolean {
	return (
		left.length === right.length &&
		left.every((field, index) => {
			const other = right[index];
			return field.field === other.field && field.direction === other.direction;
		})
	);
}
