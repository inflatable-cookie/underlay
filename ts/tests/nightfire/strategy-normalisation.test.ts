import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	normaliseNightfireValue: vi.fn(),
}));

vi.mock("../../src/nightfire/utils", () => ({
	normaliseNightfireValue: mocks.normaliseNightfireValue,
}));

import { normaliseForStrategy } from "../../src/nightfire/editor/strategy-normalisation";

describe("nightfire/editor/strategy-normalisation", () => {
	it("keeps the blocks array and reports schema mismatch", () => {
		mocks.normaliseNightfireValue.mockReturnValue({
			schema: "old",
			blocks: [{ type: "a" }, { type: "b" }],
		});

		const result = normaliseForStrategy({ schema: "old", blocks: [] } as any, "new", "single");
		expect(result.coerced).toEqual({ schema: "new", blocks: [{ type: "a" }, { type: "b" }] });
		expect(result.schemaMismatch).toBe("old");
	});

	it("does not convert a leftover v1 block field", () => {
		mocks.normaliseNightfireValue.mockReturnValue({ schema: "same", blocks: [] });
		expect(normaliseForStrategy({ schema: "same", block: { type: "a" } } as any, "same", "multi")).toEqual({
			coerced: { schema: "same", blocks: [] },
			schemaMismatch: null,
		});
	});

	it("handles empty blocks", () => {
		mocks.normaliseNightfireValue.mockReturnValue({ blocks: [] });
		expect(normaliseForStrategy(null as any, "schema-a", "single")).toEqual({
			coerced: { schema: "schema-a", blocks: [] },
			schemaMismatch: null,
		});
	});

	it("keeps multi arrays in multi mode", () => {
		mocks.normaliseNightfireValue.mockReturnValue({ blocks: [{ type: "a" }] });
		expect(normaliseForStrategy({ schema: 123 } as any, "schema-b", "multi")).toEqual({
			coerced: { schema: "schema-b", blocks: [{ type: "a" }] },
			schemaMismatch: 123,
		});
	});
});
