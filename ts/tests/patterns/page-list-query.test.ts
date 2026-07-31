// @vitest-environment jsdom
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

(globalThis as any).$state = <T>(initial: T) => initial;
(globalThis as any).$derived = <T>(fn: () => T) => fn();
// Re-evaluate on every property access so derived values stay fresh after
// setQuery calls (real Svelte reactivity without the compiler).
(globalThis as any).$derived.by = (fn: () => unknown) =>
	new Proxy({}, { get: (_, prop) => (fn() as Record<PropertyKey, unknown>)[prop] });

const { createPageListQueryState } = await import(
	"../../src/patterns/page-list-query.svelte"
);

describe("createPageListQueryState", () => {
	beforeEach(() => {
		mocks.goto.mockClear();
		mocks.page.url = new URL("http://localhost/modules");
	});

	it("url mode reads query from the address and applies default limit", () => {
		mocks.page.url = new URL("http://localhost/modules?page=2&filter[status]=active");
		const state = createPageListQueryState({ mode: "url", pageSize: 30 });

		expect(state.query.page).toBe(2);
		expect(state.query.limit).toBe(30);
		expect(state.query.filters).toEqual([{ field: "status", value: "active" }]);
	});

	it("url mode writes query changes via goto with replaceState", () => {
		const state = createPageListQueryState({ mode: "url", pageSize: 30 });
		state.setQuery({ page: 3, limit: 10 });

		expect(mocks.goto).toHaveBeenCalledTimes(1);
		const [url, options] = mocks.goto.mock.calls[0];
		expect(url).toContain("page=3");
		expect(url).toContain("limit=10");
		expect(options).toEqual({ replaceState: true, keepFocus: true });
	});

	it("local mode keeps state in memory with default limit", () => {
		const state = createPageListQueryState({ mode: "local", pageSize: 50 });

		expect(state.query.page).toBe(1);
		expect(state.query.limit).toBe(50);

		state.setQuery({ page: 2 });
		expect(state.query.page).toBe(2);
		expect(state.query.limit).toBe(50);
		expect(mocks.goto).not.toHaveBeenCalled();
	});

	it("local mode preserves explicit limits", () => {
		const state = createPageListQueryState({
			mode: "local",
			pageSize: 50,
			initialQuery: { page: 1, limit: 10 },
		});

		expect(state.query.limit).toBe(10);
	});

	it("refresh increments the refresh version", () => {
		const state = createPageListQueryState({ mode: "local" });
		expect(state.refreshVersion).toBe(0);
		state.refresh();
		expect(state.refreshVersion).toBe(1);
		state.refresh();
		expect(state.refreshVersion).toBe(2);
	});
});
