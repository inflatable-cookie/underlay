import { beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	goto: vi.fn(async () => {}),
	page: { url: new URL("http://localhost/modules") },
}));

vi.mock("$app/navigation", () => ({
	goto: mocks.goto,
}));

vi.mock("$app/state", () => ({
	page: mocks.page,
}));

import { createPageListQueryState } from "../../src/patterns/page-list-query.svelte";

describe("patterns/page-list-query.svelte.ts (compiled)", () => {
	beforeEach(() => {
		mocks.goto.mockClear();
		mocks.page.url = new URL("http://localhost/modules");
	});

	it("preserves unrelated URL params while updating and clearing list query params", () => {
		mocks.page.url = new URL(
			"http://localhost/modules?tab=active&redirectTo=%2Fhome&page=2&filter[status]=draft&sort=title:asc",
		);
		const state = createPageListQueryState({ mode: "url", pageSize: 30 });

		expect(state.query.page).toBe(2);
		expect(state.query.filters).toEqual([{ field: "status", value: "draft" }]);

		state.setQuery({ page: 1, limit: 30, filters: [{ field: "role", value: "admin" }] });

		expect(mocks.goto).toHaveBeenCalledTimes(1);
		const [urlString, options] = mocks.goto.mock.calls[0];
		const nextUrl = new URL(urlString);
		expect(nextUrl.searchParams.get("tab")).toBe("active");
		expect(nextUrl.searchParams.get("redirectTo")).toBe("/home");
		expect(nextUrl.searchParams.get("page")).toBe("1");
		expect(nextUrl.searchParams.get("limit")).toBe("30");
		expect(nextUrl.searchParams.get("filter[role]")).toBe("admin");
		expect(nextUrl.searchParams.get("filter[status]")).toBeNull();
		expect(nextUrl.searchParams.get("sort")).toBeNull();
		expect(options).toEqual({ replaceState: true, keepFocus: true });
	});

	it("swallows rejected goto promises without unhandled rejection", async () => {
		mocks.goto.mockRejectedValueOnce(new Error("navigation aborted"));
		const state = createPageListQueryState({ mode: "url", pageSize: 30 });

		expect(() => state.setQuery({ page: 2, limit: 30 })).not.toThrow();
		await new Promise((resolve) => setTimeout(resolve, 0));
	});
});
