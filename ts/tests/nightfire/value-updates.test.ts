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
		expect(asSingleBlockValue("schema@1", block)).toEqual({
			schema: "schema@1",
			block,
			blocks: undefined
		});
	});

	it("builds multi-block values", () => {
		const blocks = [{ type: "markdown" }, { type: "image" }];
		expect(asMultiBlockValue("schema@1", blocks)).toEqual({
			schema: "schema@1",
			block: undefined,
			blocks
		});
	});

	it("replaces a block by index without mutating source", () => {
		const original = [{ type: "a" }, { type: "b" }];
		const next = replaceBlockAtIndex(original, 1, { type: "c" });
		expect(next).toEqual([{ type: "a" }, { type: "c" }]);
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
		expect(changeBlockType({ foo: "bar" }, "new.type")).toEqual({
			foo: "bar",
			type: "new.type"
		});
		expect(changeBlockType("not-an-object", "markdown")).toEqual({
			type: "markdown"
		});
	});

	it("adds, removes, and moves blocks", () => {
		const added = addBlock([], "markdown");
		expect(added).toEqual([
			{
				type: "markdown",
				version: "initial",
				hash: "",
				data: {}
			}
		]);

		const inserted = insertBlockAfter([{ type: "a" }, { type: "b" }], 0, "markdown");
		expect(inserted).toEqual([
			{ type: "a" },
			{
				type: "markdown",
				version: "initial",
				hash: "",
				data: {}
			},
			{ type: "b" }
		]);

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
