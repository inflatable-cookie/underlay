import { describe, expect, it } from "vitest";
import { transformSummaryBlockOnLayoutChange } from "../../../src/nightfire/editor/summary-transform";
import { label } from "./fixtures";

describe("nightfire/editor/summary-transform page layouts", () => {
	it("keeps block when type unchanged or missing", () => {
		expect(transformSummaryBlockOnLayoutChange(undefined, "summary.book", label)).toMatchObject({
			block: { type: "summary.book", data: {} },
			warning: null,
		});

		expect(
			transformSummaryBlockOnLayoutChange({ type: "summary.book", data: { pages: [] } }, "summary.book", label)
		).toMatchObject({
			block: { type: "summary.book", data: { pages: [] } },
			warning: null,
		});
	});

	it("transforms text layouts while preserving title/body and subtitle rules", () => {
		const current = {
			type: "summary.book",
			data: { pages: [{ title: "T1", body: "B1", ignored: true }], subTitle: "Sub" },
		};

		const toPie = transformSummaryBlockOnLayoutChange(current, "summary.pie", label);
		expect(toPie.warning).toBeNull();
		expect(toPie.block).toMatchObject({
			type: "summary.pie",
			data: { pages: [{ title: "T1", body: "B1" }], subTitle: "Sub" },
		});

		const toBook = transformSummaryBlockOnLayoutChange({ type: "summary.steps", data: { pages: [{ title: "T2", body: "B2" }], subTitle: "X" } }, "summary.book", label);
		expect(toBook.block).toMatchObject({
			type: "summary.book",
			data: { pages: [{ title: "T2", body: "B2" }] },
		});
	});

	it("warns when converting image pages to text pages with existing image selections", () => {
		const current = {
			type: "summary.diagram",
			data: { pages: [{ title: "A", body: "B", image_id: "img-1" }] },
		};

		const result = transformSummaryBlockOnLayoutChange(current, "summary.circles", label);
		expect(result.block).toMatchObject({
			type: "summary.circles",
			data: { pages: [{ title: "A", body: "B" }], subTitle: null },
		});
		expect(result.warning).toContain("drops image selections");
	});

	it("removes subtitle when converting image pages to non-subtitled text layouts", () => {
		const result = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.diagram",
				data: { pages: [{ title: "A", body: "B", image_id: null }], subTitle: "keep?" },
			},
			"summary.book",
			label
		);

		expect(result.block).toMatchObject({
			type: "summary.book",
			data: { pages: [{ title: "A", body: "B" }] },
		});
		expect(result.warning).toBeNull();
	});

	it("supports image-page to image-page and text-page to image-page conversions", () => {
		const fromImagePages = transformSummaryBlockOnLayoutChange(
			{ type: "summary.diagram", data: { pages: [{ title: "T", body: "B", image_id: "img-1" }] } },
			"summary.slideshow",
			label
		);
		expect(fromImagePages.block).toMatchObject({
			type: "summary.slideshow",
			data: { pages: [{ title: "T", body: "B", image_id: "img-1" }] },
		});
		expect(fromImagePages.warning).toBeNull();

		const fromTextPages = transformSummaryBlockOnLayoutChange(
			{ type: "summary.book", data: { pages: [{ title: "T", body: "B" }] } },
			"summary.diagram",
			label
		);
		expect(fromTextPages.block).toMatchObject({
			type: "summary.diagram",
			data: { pages: [{ title: "T", body: "B", image_id: null }] },
		});
		expect(fromTextPages.warning).toBeNull();
	});

	it("falls back to a plain type swap for unrelated layouts", () => {
		const result = transformSummaryBlockOnLayoutChange(
			{ type: "summary.unknown", data: { something: true } } as any,
			"summary.diagram",
			label
		);
		expect(result).toMatchObject({
			block: { type: "summary.diagram", data: { something: true } },
			warning: null,
		});
	});

	it("preserves non-empty subtitle on image-to-text subtitle layouts", () => {
		const result = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.diagram",
				data: { pages: [{ title: "A", body: "B", image_id: null }], subTitle: "Kept subtitle" },
			},
			"summary.steps",
			label
		);

		expect(result.block).toMatchObject({
			type: "summary.steps",
			data: { pages: [{ title: "A", body: "B" }], subTitle: "Kept subtitle" },
		});
		expect(result.warning).toBeNull();
	});
});
