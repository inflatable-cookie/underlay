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

	it("returns null when strategy context is missing", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		expect(strategies.useNightfireStrategies()).toBeNull();
	});

	it("returns null when context lookup succeeds but value is undefined", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		mockGetContext.mockImplementationOnce(() => undefined);
		expect(strategies.useNightfireStrategies()).toBeNull();
	});

	it("handles strategy fetch failures and stale cache re-fetch", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		let calls = 0;
		strategies.configureNightfireStrategies({
			fetchStrategies: async () => {
				calls += 1;
				if (calls === 1) {
					throw "boom";
				}
				return [
					{ id: `ok-${calls}`, cardinality: { mode: "single" }, allowedTypes: [], allowedCategories: [], defaultType: "markdown" },
				];
			},
			cacheTtl: -1,
		});

		const store = strategies.createNightfireStrategiesContext();
		await expect(store.ensure()).resolves.toEqual([]);
		expect(get(store.error)).toBe("Failed to load strategies");

		const loaded = await store.ensure();
		expect(loaded[0]?.id).toBe("ok-2");
		expect(get(store.error)).toBeNull();

		await store.ensure();
		expect(calls).toBe(3);
	});

	it("uses default cache ttl and preserves Error messages", async () => {
		const strategies = await import("../../src/nightfire/strategies");
		let calls = 0;
		strategies.configureNightfireStrategies({
			fetchStrategies: async () => {
				calls += 1;
				if (calls === 1) {
					throw new Error("explicit boom");
				}
				return [{ id: "s-default-ttl", cardinality: { mode: "single" }, allowedTypes: [], allowedCategories: [], defaultType: "markdown" }];
			},
		});

		const store = strategies.createNightfireStrategiesContext();
		await expect(store.ensure()).resolves.toEqual([]);
		expect(get(store.error)).toBe("explicit boom");

		await expect(store.ensure()).resolves.toEqual([
			{ id: "s-default-ttl", cardinality: { mode: "single" }, allowedTypes: [], allowedCategories: [], defaultType: "markdown" },
		]);
		expect(calls).toBe(2);

		// Cached result should be reused under default TTL.
		await store.ensure();
		expect(calls).toBe(2);
		expect(store.findById("missing-id")).toBeNull();
	});
});
