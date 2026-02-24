import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	getSchemaDefinition: vi.fn(),
}));

vi.mock("../../src/nightfire/editor-registry", () => ({
	getSchemaDefinition: mocks.getSchemaDefinition,
}));

import { resolveSchemaDefinition } from "../../src/nightfire/editor/schema-resolution";

describe("nightfire/editor/schema-resolution", () => {
	it("prefers requested schema when definition exists", () => {
		mocks.getSchemaDefinition.mockImplementation((id: string) =>
			id === "requested"
				? { schema: "requested", mode: "multi", defaultType: "section" }
				: null
		);

		expect(resolveSchemaDefinition("requested", "fallback")).toEqual({
			editorSchema: "requested",
			registryDef: { schema: "requested", mode: "multi", defaultType: "section" },
		});
	});

	it("falls back to fallback schema or synthetic default", () => {
		mocks.getSchemaDefinition.mockImplementation((id: string) =>
			id === "fallback"
				? { schema: "fallback", mode: "single", defaultType: "markdown" }
				: null
		);

		expect(resolveSchemaDefinition("missing", "fallback")).toEqual({
			editorSchema: "fallback",
			registryDef: { schema: "fallback", mode: "single", defaultType: "markdown" },
		});

		mocks.getSchemaDefinition.mockReturnValue(null);
		expect(resolveSchemaDefinition("missing", "also-missing")).toEqual({
			editorSchema: "missing",
			registryDef: { schema: "missing", mode: "single", defaultType: "markdown" },
		});
	});
});
