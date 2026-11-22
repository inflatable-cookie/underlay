import { describe, expect, it } from "vitest";
import {
	appendPageListParams,
	buildPageListQuery,
	normalizePageListResponse,
} from "@inflatable-cookie/underlay/client/page-lists";

describe("client/page-lists", () => {
	it.each([true, false])("normalizes the Rust page-list wire shape with has_more=%s", (hasMore) => {
		const data = [{ id: "item-1" }];
		const wire = { data, total: 4, has_more: hasMore };

		expect(normalizePageListResponse(wire)).toEqual({ data, total: 4, hasMore });
		expect(wire.has_more).toBe(hasMore);
	});
	it("builds page-shaped query params", () => {
		expect(buildPageListQuery({ page: 3, limit: 25 })).toEqual({
			page: "3",
			limit: "25",
		});
		expect(buildPageListQuery({})).toEqual({});
	});

	it("appends page-shaped query params", () => {
		expect(appendPageListParams("/v1/items", { page: 2, limit: 10 })).toBe(
			"/v1/items?page=2&limit=10",
		);
		expect(appendPageListParams("/v1/items?variant=pending", { page: 4, limit: 50 })).toBe(
			"/v1/items?variant=pending&page=4&limit=50",
		);
	});

	it("replaces duplicate keys when merging", () => {
		expect(appendPageListParams("/v1/items?page=1&limit=10", { page: 2, limit: 20 })).toBe(
			"/v1/items?page=2&limit=20",
		);
	});

	it("preserves hash fragments", () => {
		expect(appendPageListParams("/v1/items#section", { page: 2 })).toBe("/v1/items?page=2#section");
		expect(appendPageListParams("/v1/items?limit=10#section", { page: 2 })).toBe(
			"/v1/items?limit=10&page=2#section",
		);
		expect(appendPageListParams("/v1/items#section", {})).toBe("/v1/items#section");
	});
});
