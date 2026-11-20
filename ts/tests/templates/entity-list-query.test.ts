import { describe, expect, it } from "vitest";
import {
	getListFilterValue,
	isListFilterActive,
	listQueryKey,
	normalizeListQuery,
	updateListFilters,
} from "../../src/templates/entity-list-query";
import type { FilterConfig } from "../../src/templates/template.types";

describe("entity list query", () => {
	it("normalizes empty controls and uses a stable key for equivalent query objects", () => {
		const local = normalizeListQuery({
			page: 0,
			filters: [
				{ field: "state", value: "All" },
				{ field: "search", operator: "like", value: "%Mars%" },
				{ field: "other", value: " " },
			],
			sort: [{ field: "", direction: "desc" }],
		});
		const fromUrl = normalizeListQuery({
			filters: [{ operator: "like", value: "%Mars%", field: "search" }],
			page: 1,
			limit: 30,
		});

		expect(local).toEqual(fromUrl);
		expect(listQueryKey(local)).toBe(listQueryKey(fromUrl));
		expect(listQueryKey(local, 1)).not.toBe(listQueryKey(local, 2));
		expect(listQueryKey({ ...local, variant: "archived" })).not.toBe(listQueryKey(local));
	});

	it("replaces the search condition while preserving other filters", () => {
		const search: FilterConfig = { id: "search", label: "Search", type: "search" };
		const stateFilter = { field: "state", value: "open" };
		const query = { filters: [stateFilter] };

		const filters = updateListFilters(query, search, "  Mars  ");
		expect(filters).toEqual([stateFilter, { field: "search", operator: "like", value: "%Mars%" }]);
		expect(getListFilterValue({ filters }, search)).toBe("Mars");
		expect(isListFilterActive({ filters }, search)).toBe(true);
		expect(updateListFilters({ filters }, search, "  ")).toEqual([stateFilter]);
	});
});
