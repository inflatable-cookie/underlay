import { describe, expect, it } from "vitest";
import {
	appendQueryParams,
	buildQueryString,
	createFilterBuilder,
	orderByToSortFields,
	parseQueryParams,
	parseSort,
	serializeSort,
} from "../../src/client/query";

describe("client/query", () => {
	it("converts OrderBy values to sort fields", () => {
		expect(
			orderByToSortFields([
				{ key: "title", direction: "asc" },
				{ key: "created_at", direction: "desc" },
			])
		).toEqual([
			{ field: "title", direction: "asc" },
			{ field: "created_at", direction: "desc" },
		]);
	});

	it("serializes and parses sort values", () => {
		const serialized = serializeSort([
			{ field: "title", direction: "asc" },
			{ field: "updated_at", direction: "desc" },
		]);
		expect(serialized).toBe("title:asc,updated_at:desc");

		expect(parseSort(serialized)).toEqual([
			{ field: "title", direction: "asc" },
			{ field: "updated_at", direction: "desc" },
		]);
		expect(parseSort("name")).toEqual([{ field: "name", direction: "asc" }]);
		expect(parseSort("")).toEqual([]);
	});

	it("builds query strings with sort, filters, and paging", () => {
		const query = buildQueryString({
			sort: [{ field: "title", direction: "asc" }],
			filters: [
				{ field: "status", value: "active" },
				{ field: "weight", operator: "gte", value: "10" },
			],
			page: 2,
			limit: 50,
		});
		const params = new URLSearchParams(query);

		expect(params.get("sort")).toBe("title:asc");
		expect(params.get("filter[status]")).toBe("active");
		expect(params.get("filter[weight][gte]")).toBe("10");
		expect(params.get("page")).toBe("2");
		expect(params.get("limit")).toBe("50");
	});

	it("appends query strings with the right separator", () => {
		expect(
			appendQueryParams("/v1/items", {
				page: 1,
				limit: 10,
			})
		).toBe("/v1/items?page=1&limit=10");

		expect(
			appendQueryParams("/v1/items?archived=true", {
				page: 3,
			})
		).toBe("/v1/items?archived=true&page=3");

		expect(appendQueryParams("/v1/items", {})).toBe("/v1/items");
	});

	it("merges query strings without duplicating keys", () => {
		expect(
			appendQueryParams("/v1/items?limit=10", {
				limit: 20,
			})
		).toBe("/v1/items?limit=20");
	});

	it("creates filter helpers for all operators", () => {
		const f = createFilterBuilder();
		expect(f.eq("status", "active")).toEqual({
			field: "status",
			operator: "eq",
			value: "active",
		});
		expect(f.ne("status", "deleted").operator).toBe("ne");
		expect(f.gt("weight", "1").operator).toBe("gt");
		expect(f.gte("weight", "1").operator).toBe("gte");
		expect(f.lt("weight", "1").operator).toBe("lt");
		expect(f.lte("weight", "1").operator).toBe("lte");
		expect(f.like("title", "%intro%").operator).toBe("like");
	});

	it("parses query params back to structured values", () => {
		const params = new URLSearchParams(
			"sort=title:asc,created_at:desc&filter[status]=active&filter[weight][gte]=10&page=3&limit=25"
		);
		expect(parseQueryParams(params)).toEqual({
			sort: [
				{ field: "title", direction: "asc" },
				{ field: "created_at", direction: "desc" },
			],
			filters: [
				{ field: "status", value: "active" },
				{ field: "weight", operator: "gte", value: "10" },
			],
			page: 3,
			limit: 25,
		});
	});

	it("returns empty params when no supported keys are present", () => {
		const params = new URLSearchParams("q=search&archived=true");
		expect(parseQueryParams(params)).toEqual({});
	});

	it("ignores malformed filter keys while parsing valid pagination", () => {
		const params = new URLSearchParams(
			"filter[not-valid-key]=x&filter[weight][gte]=10&page=1&limit=20"
		);
		expect(parseQueryParams(params)).toEqual({
			filters: [{ field: "weight", operator: "gte", value: "10" }],
			page: 1,
			limit: 20,
		});
	});
});
