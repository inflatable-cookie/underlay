import { describe, expect, it } from "vitest";
import {
	appendQueryParams,
	buildQueryString,
	createFilterBuilder,
	orderByToSortFields,
	parseQueryParams,
	parseSort,
	queryParamsToFlatRecord,
	serializeSort,
} from "../../src/client/query";

describe("client/query", () => {
	it("converts OrderBy values to sort fields", () => {
		expect(
			orderByToSortFields([
				{ key: "title", direction: "asc" },
				{ key: "created_at", direction: "desc" },
			]),
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
		expect(parseSort("title:asc,")).toEqual([{ field: "title", direction: "asc" }]);
		expect(parseSort(",title:asc,,created_at:desc, ")).toEqual([
			{ field: "title", direction: "asc" },
			{ field: "created_at", direction: "desc" },
		]);
		expect(parseSort(":asc, ,")).toEqual([]);
		expect(parseSort("")).toEqual([]);
	});

	it("builds query strings with variant, sort, filters, and paging", () => {
		const query = buildQueryString({
			variant: "pending",
			sort: [{ field: "title", direction: "asc" }],
			filters: [
				{ field: "status", value: "active" },
				{ field: "weight", operator: "gte", value: "10" },
			],
			page: 2,
			limit: 50,
		});
		const params = new URLSearchParams(query);

		expect(params.get("variant")).toBe("pending");
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
			}),
		).toBe("/v1/items?page=1&limit=10");

		expect(
			appendQueryParams("/v1/items?archived=true", {
				page: 3,
			}),
		).toBe("/v1/items?archived=true&page=3");

		expect(appendQueryParams("/v1/items", {})).toBe("/v1/items");
	});

	it("merges query strings without duplicating keys", () => {
		expect(
			appendQueryParams("/v1/items?variant=all&limit=10", {
				variant: "pending",
				limit: 20,
			}),
		).toBe("/v1/items?variant=pending&limit=20");
	});

	it("preserves #hash fragments when appending or merging query params", () => {
		expect(
			appendQueryParams("/v1/items#section", {
				page: 2,
				limit: 10,
			}),
		).toBe("/v1/items?page=2&limit=10#section");

		expect(
			appendQueryParams("/v1/items?archived=true&limit=5#top", {
				limit: 20,
				page: 3,
			}),
		).toBe("/v1/items?archived=true&limit=20&page=3#top");

		expect(
			appendQueryParams("/v1/items#frag?not-query=1", {
				page: 1,
			}),
		).toBe("/v1/items?page=1#frag?not-query=1");
	});

	it("converts query params with variant to flat records", () => {
		expect(
			queryParamsToFlatRecord({
				variant: "pending",
				filters: [{ field: "status", value: "active" }],
				page: 1,
			}),
		).toEqual({
			variant: "pending",
			"filter[status]": "active",
			page: "1",
		});
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
			"variant=pending&sort=title:asc,created_at:desc&filter[status]=active&filter[weight][gte]=10&page=3&limit=25",
		);
		expect(parseQueryParams(params)).toEqual({
			variant: "pending",
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

	it("supports hyphenated and dotted field names while ignoring malformed filter keys", () => {
		const params = new URLSearchParams(
			"filter[created-at][gte]=2026-01-01&filter[user.email][like]=%40example.com&filter[meta.zip-code]=12345&filter[bad key]=x&filter[]=y&filter[a..b]=z&page=1&limit=20",
		);
		expect(parseQueryParams(params)).toEqual({
			filters: [
				{ field: "created-at", operator: "gte", value: "2026-01-01" },
				{ field: "user.email", operator: "like", value: "@example.com" },
				{ field: "meta.zip-code", value: "12345" },
			],
			page: 1,
			limit: 20,
		});
	});

	it("rejects unknown filter operators and non-numeric pagination values", () => {
		const params = new URLSearchParams(
			"filter[weight][evil]=10&filter[status][eq]=active&page=abc&limit=12abc&sort=,",
		);
		expect(parseQueryParams(params)).toEqual({
			filters: [{ field: "status", operator: "eq", value: "active" }],
		});

		expect(parseQueryParams(new URLSearchParams("page=0&limit=-5"))).toEqual({});
		expect(parseQueryParams(new URLSearchParams("page=1.5&limit=NaN"))).toEqual({});
	});
});
