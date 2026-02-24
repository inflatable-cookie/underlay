import { describe, expect, it } from "vitest";
import { FIELD_A11Y_CONTEXT_KEY, mergeAriaDescribedBy } from "../../src/components/field/a11y-context";

describe("components/field/a11y-context", () => {
	it("exports stable context key", () => {
		expect(FIELD_A11Y_CONTEXT_KEY).toBe("underlayFieldA11y");
	});

	it("merges, trims, deduplicates aria-describedby ids", () => {
		expect(mergeAriaDescribedBy(undefined, null, "")).toBeUndefined();
		expect(mergeAriaDescribedBy("a b", " b c ", "a", "d")).toBe("a b c d");
	});
});
