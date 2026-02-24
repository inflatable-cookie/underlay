import { describe, expect, it } from "vitest";
import { getBlockRenderer, registerBlockRenderer } from "../../src/nightfire/render-registry";

class A {}
class B {}

describe("nightfire/render-registry", () => {
	it("returns null for missing type", () => {
		expect(getBlockRenderer("schema", undefined)).toBeNull();
	});

	it("resolves schema-specific and wildcard renderers", () => {
		registerBlockRenderer(null, "markdown", A as any);
		registerBlockRenderer("schema-1", "markdown", B as any);

		expect(getBlockRenderer("schema-1", "markdown")).toBe(B as any);
		expect(getBlockRenderer("other", "markdown")).toBe(A as any);
		expect(getBlockRenderer("schema-1", "unknown")).toBeNull();
	});
});
