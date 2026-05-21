import { describe, expect, it } from "vitest";
import {
	appendPageListParams,
	buildPageListQuery,
} from "../../src/client/page-lists";

describe("client/page-lists", () => {
	it("builds page-shaped query params", () => {
		expect(buildPageListQuery({ page: 3, limit: 25 })).toEqual({
			page: "3",
			limit: "25",
		});
		expect(buildPageListQuery({})).toEqual({});
	});

	it("appends page-shaped query params", () => {
		expect(appendPageListParams("/v1/items", { page: 2, limit: 10 })).toBe(
			"/v1/items?page=2&limit=10"
		);
		expect(
			appendPageListParams("/v1/items?variant=pending", { page: 4, limit: 50 })
		).toBe("/v1/items?variant=pending&page=4&limit=50");
	});

	it("replaces duplicate keys when merging", () => {
		expect(
			appendPageListParams("/v1/items?page=1&limit=10", { page: 2, limit: 20 })
		).toBe("/v1/items?page=2&limit=20");
	});
});
