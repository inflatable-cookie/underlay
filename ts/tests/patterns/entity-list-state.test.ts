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

const { createEntityListState } = await import(
	"../../src/patterns/entity-list-state.svelte"
);

function createState(overrides: Record<string, unknown> = {}) {
	return createEntityListState({
		queryMode: () => "url",
		title: () => "Modules",
		pageSize: 30,
		...overrides,
	});
}

describe("createEntityListState", () => {
	beforeEach(() => {
		mocks.goto.mockClear();
		mocks.page.url = new URL("http://localhost/modules");
	});

	it("exposes the base query state with default limit", () => {
		mocks.page.url = new URL("http://localhost/modules?page=2");
		const state = createState();

		expect(state.query.page).toBe(2);
		expect(state.query.limit).toBe(30);
	});

	it("wires setQuery through to url mode writes", () => {
		const state = createState();
		state.setQuery({ page: 3 });

		expect(mocks.goto).toHaveBeenCalledTimes(1);
		expect(mocks.goto.mock.calls[0][0]).toContain("page=3");
	});

	it("keeps setQuery local in local mode", () => {
		const state = createState({ queryMode: () => "local" });
		state.setQuery({ page: 2 });

		expect(mocks.goto).not.toHaveBeenCalled();
		expect(state.query.page).toBe(2);
	});

	it("builds reloadKey from scope and refresh version", () => {
		const state = createState({ reloadScope: () => "modules:all" });

		expect(state.reloadKey).toBe("modules:all:0");
		state.refresh();
		expect(state.refreshVersion).toBe(1);
		expect(state.reloadKey).toBe("modules:all:1");
	});

	it("falls back to the bare version when no reloadScope is given", () => {
		const state = createState();
		state.refresh();

		expect(state.reloadKey).toBe("1");
	});

	it("derives sourceContext from the current url when none is provided", () => {
		mocks.page.url = new URL("http://localhost/modules?page=2");
		const state = createState();

		expect(state.sourceContext).toEqual({
			label: "Modules",
			href: "/modules?page=2",
			type: "list",
		});
	});

	it("prefers a provided sourceContext", () => {
		const provided = { label: "Pathways", href: "/pathways", type: "list" as const };
		const state = createState({ sourceContext: () => provided });

		expect(state.sourceContext).toBe(provided);
	});

	it("filterValue returns undefined for missing fields", () => {
		const state = createState();
		expect(state.filterValue("search")).toBeUndefined();
	});

	it("filterValue reads plain filter values", () => {
		mocks.page.url = new URL("http://localhost/modules?filter[status]=active");
		const state = createState();

		expect(state.filterValue("status")).toBe("active");
	});

	it("filterValue strips like wildcards when asked", () => {
		const state = createState({ queryMode: () => "local" });
		state.setQuery({
			page: 1,
			filters: [{ field: "search", operator: "like", value: "%intro%" }],
		});

		expect(state.filterValue("search", { stripLike: true })).toBe("intro");
		expect(state.filterValue("search")).toBe("%intro%");
	});

	it("filterValue treats empty strings and emptyValues sentinels as undefined", () => {
		const state = createState({ queryMode: () => "local" });
		state.setQuery({
			page: 1,
			filters: [
				{ field: "subjectId", value: "All" },
				{ field: "title", value: "" },
			],
		});

		expect(state.filterValue("subjectId", { emptyValues: ["All"] })).toBeUndefined();
		expect(state.filterValue("subjectId")).toBe("All");
		expect(state.filterValue("title")).toBeUndefined();
	});

	it("backHrefProps returns props in url mode", () => {
		const state = createState();
		expect(state.backHrefProps("/learning", "Learning")).toEqual({
			backHref: "/learning",
			backLabel: "Learning",
		});
	});

	it("backHrefProps returns nothing in local mode", () => {
		const state = createState({ queryMode: () => "local" });
		expect(state.backHrefProps("/learning", "Learning")).toEqual({});
	});
});
