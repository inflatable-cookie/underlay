import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	normaliseNightfireValue: vi.fn(),
}));

vi.mock("../../src/nightfire/utils", () => ({
	normaliseNightfireValue: mocks.normaliseNightfireValue,
}));

import { normaliseForStrategy } from "../../src/nightfire/editor/strategy-normalisation";

describe("nightfire/editor/strategy-normalisation", () => {
	it("coerces multi blocks into single mode and reports schema mismatch", () => {
		mocks.normaliseNightfireValue.mockReturnValue({ blocks: [{ type: "a" }, { type: "b" }] });

		const result = normaliseForStrategy({ schema: "old" } as any, "new", "single");
		expect(result.coerced).toEqual({ schema: "new", block: { type: "a" }, blocks: undefined });
		expect(result.schemaMismatch).toBe("old");
	});

	it("coerces single block into multi mode and clears block when both exist", () => {
		mocks.normaliseNightfireValue.mockReturnValue({ block: { type: "a" } });
		expect(normaliseForStrategy({ schema: "same" } as any, "same", "multi")).toEqual({
			coerced: { schema: "same", block: undefined, blocks: [{ type: "a" }] },
			schemaMismatch: null,
		});

		mocks.normaliseNightfireValue.mockReturnValue({ block: { type: "a" }, blocks: [{ type: "b" }] });
		expect(normaliseForStrategy({} as any, "schema-x", "multi").coerced).toEqual({
			schema: "schema-x",
			block: undefined,
			blocks: [{ type: "b" }],
		});
	});
});
