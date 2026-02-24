import { describe, expect, it, vi } from "vitest";

describe("patterns/synced-selection.svelte.ts", () => {
	it("initializes once, syncs non-null values, and resets", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useSyncedSelection } = await import("../../src/patterns/synced-selection.svelte");
		const selection = useSyncedSelection<string>();

		expect(selection.value).toBeNull();
		expect(selection.hasInitialized).toBe(false);

		selection.initializeFrom("alpha");
		expect(selection.value).toBe("alpha");
		expect(selection.hasInitialized).toBe(true);

		selection.initializeFrom("beta");
		expect(selection.value).toBe("alpha");

		selection.syncFrom(undefined);
		expect(selection.value).toBe("alpha");

		selection.syncFrom("gamma");
		expect(selection.value).toBe("gamma");

		selection.value = "delta";
		expect(selection.value).toBe("delta");

		selection.reset();
		expect(selection.value).toBeNull();
		expect(selection.hasInitialized).toBe(false);
	});

	it("supports non-null initial values", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { useSyncedSelection } = await import("../../src/patterns/synced-selection.svelte");
		const selection = useSyncedSelection<string>("seed");

		expect(selection.value).toBe("seed");
		expect(selection.hasInitialized).toBe(true);
	});
});
