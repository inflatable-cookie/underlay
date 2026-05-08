import { describe, expect, it } from "vitest";
import {
	DEFAULT_PAGE_SIZE,
	MAX_PAGE_SIZE,
	appendPaginationParams,
	buildPaginationQuery,
	type PaginatedResponse,
	type PaginationController,
	type PaginationParams,
} from "../../src/patterns/pagination-types";

describe("patterns/pagination-types", () => {
	it("exposes pagination constants", () => {
		expect(DEFAULT_PAGE_SIZE).toBe(30);
		expect(MAX_PAGE_SIZE).toBe(100);
	});

	it("builds query params with only explicit and non-default values", () => {
		const full: PaginationParams = {
			limit: 25,
			cursor: "abc123",
			direction: "backward",
			includeTotal: false,
		};
		expect(buildPaginationQuery(full)).toEqual({
			limit: "25",
			cursor: "abc123",
			direction: "backward",
			includeTotal: "false",
		});

		const defaultsOmitted: PaginationParams = {
			limit: undefined,
			cursor: null,
			direction: "forward",
			includeTotal: true,
		};
		expect(buildPaginationQuery(defaultsOmitted)).toEqual({});
	});

	it("appends query params correctly with and without existing query strings", () => {
		const params: PaginationParams = {
			limit: 10,
			cursor: "next+cursor",
			direction: "backward",
			includeTotal: false,
		};
		expect(appendPaginationParams("/api/items", params)).toBe(
			"/api/items?limit=10&cursor=next%2Bcursor&direction=backward&includeTotal=false"
		);
		expect(appendPaginationParams("/api/items?sort=name", params)).toBe(
			"/api/items?sort=name&limit=10&cursor=next%2Bcursor&direction=backward&includeTotal=false"
		);
		expect(appendPaginationParams("/api/items", {})).toBe("/api/items");
	});

	it("merges query params without duplicating keys", () => {
		expect(
			appendPaginationParams("/api/items?limit=5", { limit: 10 })
		).toBe("/api/items?limit=10");
	});

	it("supports type-level usage for response and controller contracts", async () => {
		const response: PaginatedResponse<number> = {
			data: [1, 2, 3],
			nextCursor: "n1",
			prevCursor: null,
			hasMore: true,
			total: 30,
		};
		expect(response.data[0]).toBe(1);

		let page = 1;
		const controller: PaginationController<number> = {
			get items() {
				return response.data;
			},
			get currentPage() {
				return page;
			},
			get pageSize() {
				return 3;
			},
			get hasNextPage() {
				return true;
			},
			get hasPrevPage() {
				return page > 1;
			},
			get total() {
				return response.total;
			},
			get loading() {
				return false;
			},
			get error() {
				return null;
			},
			get showingFrom() {
				return (page - 1) * 3 + 1;
			},
			get showingTo() {
				return page * 3;
			},
			get totalPages() {
				return 10;
			},
			nextPage() {
				page += 1;
			},
			prevPage() {
				page -= 1;
			},
			goToPage(next: number) {
				page = next;
			},
			setPageSize() {},
			async refresh() {},
			async reset() {
				page = 1;
			},
		};

		controller.nextPage();
		expect(controller.currentPage).toBe(2);
		controller.prevPage();
		expect(controller.currentPage).toBe(1);
		controller.goToPage?.(4);
		expect(controller.currentPage).toBe(4);
		await controller.reset();
		expect(controller.currentPage).toBe(1);
	});
});
