import { beforeEach, describe, expect, it, vi } from "vitest";
import { get } from "svelte/store";

const contextMap = vi.hoisted(() => new Map<symbol, unknown>());
const mockSetContext = vi.hoisted(() => vi.fn((key: symbol, value: unknown) => {
	contextMap.set(key, value);
	return value;
}));
const mockGetContext = vi.hoisted(() => vi.fn((key: symbol) => {
	if (!contextMap.has(key)) throw new Error("no context");
	return contextMap.get(key);
}));

vi.mock("svelte", () => ({
	setContext: mockSetContext,
	getContext: mockGetContext,
}));

describe("nightfire/strategies", () => {
	beforeEach(() => {
		contextMap.clear();
		vi.resetModules();
		mockSetContext.mockClear();
		mockGetContext.mockClear();
	});

	it("handles unconfigured store and context usage", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		const store = strategies.createNightfireStrategiesContext();

		await expect(store.ensure()).resolves.toEqual([]);
		expect(get(store.error)).toContain("not configured");
		expect(strategies.useNightfireStrategies()).toBe(store);
	});

	it("loads, caches, refreshes and invalidates strategies", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		let calls = 0;
		strategies.configureNightfireStrategies({
			fetchStrategies: async () => {
				calls += 1;
				return [
					{ id: `s-${calls}`, cardinality: { mode: "single" }, allowedTypes: [], allowedCategories: [], defaultType: "markdown" },
				];
			},
			cacheTtl: 100000,
		});

		const store = strategies.createNightfireStrategiesContext();
		const [a, b] = await Promise.all([store.ensure(), store.ensure()]);
		expect(calls).toBe(1);
		expect(a).toEqual(b);
		expect(store.findById("s-1")?.id).toBe("s-1");
		expect(await strategies.getStrategy("s-1")).toEqual(
			expect.objectContaining({ id: "s-1" })
		);

		await store.refresh();
		expect(calls).toBe(2);
		expect(store.findById("s-2")?.id).toBe("s-2");

		store.invalidate();
		await store.ensure();
		expect(calls).toBe(3);
	});
});
