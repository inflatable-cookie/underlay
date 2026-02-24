import { beforeEach, describe, expect, it, vi } from "vitest";

const contextMap = vi.hoisted(() => new Map<symbol, unknown>());
const mockSetContext = vi.hoisted(() => vi.fn((key: symbol, value: unknown) => {
	contextMap.set(key, value);
	return value;
}));
const mockGetContext = vi.hoisted(() => vi.fn((key: symbol) => {
	if (!contextMap.has(key)) throw new Error("missing");
	return contextMap.get(key);
}));

vi.mock("svelte", () => ({
	setContext: mockSetContext,
	getContext: mockGetContext,
}));

describe("nightfire/media/context", () => {
	beforeEach(() => {
		contextMap.clear();
		vi.resetModules();
	});

	it("creates and consumes media context", async () => {
		const mod = await import("../../src/nightfire/media/context");
		const ctx = { pickMedia: vi.fn(async () => null) };
		mod.createNightfireMediaContext(ctx);
		expect(mod.useNightfireMedia()).toBe(ctx);
	});

	it("returns null when context unavailable", async () => {
		const mod = await import("../../src/nightfire/media/context");
		expect(mod.useNightfireMedia()).toBeNull();
	});
});
