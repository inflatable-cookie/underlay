import { describe, expect, it } from "vitest";
import { transformSummaryBlockOnLayoutChange } from "../../src/nightfire/editor/summary-transform";

const label = (type: string) => type;

describe("nightfire/editor/summary-transform", () => {
	it("keeps block when type unchanged or missing", () => {
		expect(transformSummaryBlockOnLayoutChange(undefined, "summary.book", label)).toEqual({
			block: { type: "summary.book" },
			warning: null,
		});

		expect(
			transformSummaryBlockOnLayoutChange({ type: "summary.book", data: { pages: [] } }, "summary.book", label)
		).toEqual({
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
		expect(toPie.block).toEqual({
			type: "summary.pie",
			data: { pages: [{ title: "T1", body: "B1" }], subTitle: "Sub" },
		});

		const toBook = transformSummaryBlockOnLayoutChange({ type: "summary.steps", data: { pages: [{ title: "T2", body: "B2" }], subTitle: "X" } }, "summary.book", label);
		expect(toBook.block).toEqual({
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
		expect(result.block).toEqual({
			type: "summary.circles",
			data: { pages: [{ title: "A", body: "B" }], subTitle: null },
		});
		expect(result.warning).toContain("drops image selections");
	});

	it("maps text/image pages to slider with warning behavior", () => {
		const current = {
			type: "summary.book",
			data: {
				pages: [
					{ title: "A", body: "desc" },
					{ title: "B", body: "other" },
				],
				subTitle: "Sub",
			},
		};

		const result = transformSummaryBlockOnLayoutChange(current, "summary.imageSlider", label);
		expect(result.block).toEqual({
			type: "summary.imageSlider",
			data: {
				subTitle: "Sub",
				description: "desc",
				image1Id: null,
				image1Alt: null,
				image2Id: null,
				image2Alt: null,
				startPosition: "left",
			},
		});
		expect(result.warning).toContain("discards other pages");
	});

	it("maps slider to page layouts and warns when slider had images", () => {
		const current = {
			type: "summary.imageSlider",
			data: {
				description: "Slide body",
				image1Id: "img-1",
				image2Id: "img-2",
			},
		};

		const toImagePages = transformSummaryBlockOnLayoutChange(current, "summary.diagram", label);
		expect(toImagePages.block).toEqual({
			type: "summary.diagram",
			data: { pages: [{ title: null, body: "Slide body", image_id: null }] },
		});
		expect(toImagePages.warning).toContain("drops image selections");
	});
});
