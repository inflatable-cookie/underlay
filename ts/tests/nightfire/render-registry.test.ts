import { describe, expect, it, vi } from "vitest";
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
		expect(getBlockRenderer(undefined, "markdown")).toBe(A as any);
		expect(getBlockRenderer("schema-1", "unknown")).toBeNull();
	});

	it("loads default render registrations", async () => {
		const loaded = { markupRender: false };
		vi.resetModules();
		vi.doMock("../../src/nightfire/markup/render", () => {
			loaded.markupRender = true;
			return {};
		});
		await import("../../src/nightfire/render-registrations");
		expect(loaded).toEqual({ markupRender: true });
	});
});
