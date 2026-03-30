import { afterEach, describe, expect, it, vi } from "vitest";

describe("nightfire/markup/lazy-load-easymde", () => {
	afterEach(() => {
		vi.resetModules();
	});

	it("loads and caches the EasyMDE module", async () => {
		vi.doMock("easymde", () => ({
			default: { name: "mock-easymde" },
		}));
		const mod = await import("../../src/nightfire/markup/lazy-load-easymde");
		const [a, b] = await Promise.all([mod.lazyLoadEasyMde(), mod.lazyLoadEasyMde()]);
		const c = await mod.lazyLoadEasyMde();
		expect(a).toEqual({ name: "mock-easymde" });
		expect(b).toBe(a);
		expect(c).toBe(a);
	});

	it("prefetches only in browser contexts and never throws", async () => {
		vi.doMock("easymde", () => {
			throw new Error("mock import failure");
		});
		const originalWindow = (globalThis as { window?: unknown }).window;
		const mod = await import("../../src/nightfire/markup/lazy-load-easymde");

		(globalThis as { window?: unknown }).window = undefined;
		await expect(mod.prefetchEasyMde()).resolves.toBeUndefined();

		(globalThis as { window?: unknown }).window = {};
		await expect(mod.prefetchEasyMde()).resolves.toBeUndefined();

		(globalThis as { window?: unknown }).window = originalWindow;
	});

	it("returns the module object when no default export exists", async () => {
		vi.doMock("easymde", () => ({
			default: undefined,
			name: "module-only-export",
		}));
		const mod = await import("../../src/nightfire/markup/lazy-load-easymde");
		await expect(mod.lazyLoadEasyMde()).resolves.toMatchObject({
			name: "module-only-export",
		});
	});
});
