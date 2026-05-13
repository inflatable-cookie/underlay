import { describe, it, expect } from "vitest";
import {
	asSingleBlockValue,
	asMultiBlockValue,
	replaceBlockAtIndex,
	changeSingleBlockType,
	changeBlockType,
	addBlock,
	insertBlockAfter,
	removeBlock,
	moveBlock
} from "../../src/nightfire/editor/value-updates";

describe("nightfire value-updates", () => {
	it("builds single-block values", () => {
		const block = { type: "markdown", data: { text: "Hello" } };
		expect(asSingleBlockValue("schema@1", block)).toMatchObject({
			schema: "schema@1",
			block: {
				type: "markdown",
				data: { text: "Hello" },
				version: "initial",
				hash: ""
			},
			blocks: undefined
		});
	});

	it("builds multi-block values", () => {
		const blocks = [{ type: "markdown" }, { type: "image" }];
		expect(asMultiBlockValue("schema@1", blocks)).toMatchObject({
			schema: "schema@1",
			block: undefined,
			blocks: [
				{ type: "markdown", data: {}, version: "initial", hash: "" },
				{ type: "image", data: {}, version: "initial", hash: "" }
			]
		});
	});

	it("replaces a block by index without mutating source", () => {
		const original = [{ type: "a" }, { type: "b" }];
		const next = replaceBlockAtIndex(original, 1, { type: "c" });
		expect(next).toMatchObject([{ type: "a" }, { type: "c", data: {}, version: "initial", hash: "" }]);
		expect(next).not.toBe(original);
		expect(original).toEqual([{ type: "a" }, { type: "b" }]);
	});

	it("changes single-block type for non-summary schema", () => {
		const result = changeSingleBlockType("acow:content/article@1", null, "custom", (type) => type);
		expect(result.warning).toBeNull();
		expect(result.block).toEqual({ type: "custom" });
	});

	it("transforms summary layouts and surfaces warning text when data can be dropped", () => {
		const current = {
			type: "summary.book",
			data: {
				pages: [
					{ title: "One", body: "First page body" },
					{ title: "Two", body: "Second page body", image_id: "img-2" }
				]
			}
		};
		const result = changeSingleBlockType(
			"acow:content/summary@1",
			current,
			"summary.imageSlider",
			(type) => type
		);
		expect((result.block as { type: string }).type).toBe("summary.imageSlider");
		expect(result.warning).toContain("Changing layout");
	});

	it("changes generic block type and normalises non-object input", () => {
		expect(changeBlockType({ foo: "bar" }, "new.type")).toMatchObject({
			type: "new.type",
			data: {},
			version: "initial",
			hash: ""
		});
		expect(changeBlockType("not-an-object", "markdown")).toMatchObject({
			type: "markdown",
			data: {},
			version: "initial",
			hash: ""
		});
	});

	it("adds, removes, and moves blocks", () => {
		const added = addBlock([], "markdown");
		expect(added).toHaveLength(1);
		expect(added[0]).toMatchObject({
			type: "markdown",
			version: "initial",
			hash: "",
			data: {}
		});
		expect((added[0] as { id?: string }).id).toMatch(/^nf_/);

		const inserted = insertBlockAfter([{ type: "a" }, { type: "b" }], 0, "markdown");
		expect(inserted).toHaveLength(3);
		expect(inserted[0]).toMatchObject({ type: "a" });
		expect(inserted[1]).toMatchObject({
			type: "markdown",
			version: "initial",
			hash: "",
			data: {}
		});
		expect((inserted[1] as { id?: string }).id).toMatch(/^nf_/);
		expect(inserted[2]).toMatchObject({ type: "b" });

		const removed = removeBlock([{ type: "a" }, { type: "b" }], 0);
		expect(removed).toEqual([{ type: "b" }]);

		const moved = moveBlock([{ type: "a" }, { type: "b" }, { type: "c" }], 0, 2);
		expect(moved).toEqual([{ type: "b" }, { type: "c" }, { type: "a" }]);
	});

	it("returns null when block move indices are invalid", () => {
		const blocks = [{ type: "a" }, { type: "b" }];
		expect(moveBlock(blocks, -1, 1)).toBeNull();
		expect(moveBlock(blocks, 0, 2)).toBeNull();
		expect(moveBlock(blocks, 1, 1)).toBeNull();
	});
});
