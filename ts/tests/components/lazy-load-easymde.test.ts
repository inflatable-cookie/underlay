import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("easymde", () => ({
	default: { name: "mock-easymde" },
}));

describe("components/lazy-load-easymde", () => {
	afterEach(() => {
		vi.resetModules();
	});

	it("loads and caches the EasyMDE module", async () => {
		const mod = await import("../../src/components/lazy-load-easymde");
		const [a, b] = await Promise.all([mod.lazyLoadEasyMde(), mod.lazyLoadEasyMde()]);
		expect(a).toEqual({ name: "mock-easymde" });
		expect(b).toBe(a);
	});

	it("prefetches only in browser contexts and never throws", async () => {
		const originalWindow = (globalThis as { window?: unknown }).window;
		const mod = await import("../../src/components/lazy-load-easymde");

		(globalThis as { window?: unknown }).window = undefined;
		await expect(mod.prefetchEasyMde()).resolves.toBeUndefined();

		(globalThis as { window?: unknown }).window = {};
		await expect(mod.prefetchEasyMde()).resolves.toBeUndefined();

		(globalThis as { window?: unknown }).window = originalWindow;
	});
});
