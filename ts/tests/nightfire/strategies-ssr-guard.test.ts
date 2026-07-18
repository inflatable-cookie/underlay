import { afterEach, describe, expect, it, vi } from "vitest";
import { configureNightfireStrategies } from "../../src/nightfire/strategies";

describe("nightfire/strategies SSR guard", () => {
	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it("throws when configured during SSR (no window)", () => {
		expect(typeof window).toBe("undefined");
		expect(() =>
			configureNightfireStrategies({
				fetchStrategies: async () => [],
			}),
		).toThrow(/SSR/);
	});

	it("succeeds in the browser", () => {
		vi.stubGlobal("window", {});
		expect(() =>
			configureNightfireStrategies({
				fetchStrategies: async () => [],
			}),
		).not.toThrow();
	});
});
