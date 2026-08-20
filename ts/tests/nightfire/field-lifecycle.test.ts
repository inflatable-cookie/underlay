import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	createDefaultBlock: vi.fn(),
	writePreparedNightfireToFormData: vi.fn(),
}));

vi.mock("../../src/nightfire/editor/block-list", () => ({
	createDefaultBlock: mocks.createDefaultBlock,
}));

vi.mock("../../src/nightfire/validation", () => ({
	writePreparedNightfireToFormData: mocks.writePreparedNightfireToFormData,
}));

import { createPrepareWriter, createRequiredInitialValue } from "../../src/nightfire/editor/field-lifecycle";

describe("nightfire/editor/field-lifecycle", () => {
	it("creates required initial values as a blocks array for both modes", () => {
		mocks.createDefaultBlock.mockReturnValue({ type: "markdown", data: {} });

		expect(createRequiredInitialValue("schema-1", "single", "markdown")).toEqual({
			schema: "schema-1",
			blocks: [{ type: "markdown", data: {} }],
		});

		expect(createRequiredInitialValue("schema-2", "multi", "markdown")).toEqual({
			schema: "schema-2",
			blocks: [{ type: "markdown", data: {} }],
		});
	});

	it("creates prepare writer closure that writes to FormData", () => {
		const getValue = vi.fn(() => ({ schema: "s", blocks: [{ type: "markdown", data: {} }] }));
		const getName = vi.fn(() => "content");
		const formData = new FormData();

		const writer = createPrepareWriter(getValue as any, getName);
		writer(formData);

		expect(mocks.writePreparedNightfireToFormData).toHaveBeenCalledWith(
			formData,
			"content",
			{ schema: "s", blocks: [{ type: "markdown", data: {} }] }
		);
	});
});
