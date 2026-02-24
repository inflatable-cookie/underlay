import { describe, it, expect, vi, beforeEach } from "vitest";

describe("useToasts", () => {
	beforeEach(() => {
		vi.resetModules();
	});

	it("returns toast store from Svelte context key", async () => {
		const mockStore = {
			toasts: { subscribe: vi.fn() },
			push: vi.fn(),
			dismiss: vi.fn(),
			clear: vi.fn()
		};
		const getContext = vi.fn(() => mockStore);

		vi.doMock("svelte", () => ({
			getContext
		}));

		const { UNDERLAY_TOASTS_CONTEXT_KEY } = await import("../../src/patterns/toasts");
		const { useToasts } = await import("../../src/patterns/useToasts");
		const result = useToasts();

		expect(getContext).toHaveBeenCalledWith(UNDERLAY_TOASTS_CONTEXT_KEY);
		expect(result).toBe(mockStore);
	});
});
