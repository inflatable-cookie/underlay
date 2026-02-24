import { describe, expect, it } from "vitest";
import {
	createNoopOperation,
	withSetValue,
	withoutSetValue,
} from "../../src/patterns/optimistic/helpers";

describe("patterns/optimistic/helpers", () => {
	it("adds and removes values immutably", () => {
		const start = new Set(["a"]);
		const withValue = withSetValue(start, "b");
		const withoutValue = withoutSetValue(withValue, "a");

		expect(Array.from(start)).toEqual(["a"]);
		expect(Array.from(withValue)).toEqual(["a", "b"]);
		expect(Array.from(withoutValue)).toEqual(["b"]);
	});

	it("returns safe noop confirm/rollback operations", () => {
		const op = createNoopOperation();
		expect(() => op.confirm()).not.toThrow();
		expect(() => op.rollback()).not.toThrow();
	});
});
