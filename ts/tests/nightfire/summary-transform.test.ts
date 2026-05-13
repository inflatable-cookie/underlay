import { describe, expect, it } from "vitest";
import { transformSummaryBlockOnLayoutChange } from "../../src/nightfire/editor/summary-transform";

const label = (type: string) => type;

describe("nightfire/editor/summary-transform", () => {
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
		expect(result.block).toMatchObject({
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
		expect(toImagePages.block).toMatchObject({
			type: "summary.diagram",
			data: { pages: [{ title: null, body: "Slide body", image_id: null }] },
		});
		expect(toImagePages.warning).toContain("drops image selections");
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

	it("handles non-dropping slider and slider-to-text conversions", () => {
		const toSlider = transformSummaryBlockOnLayoutChange(
			{ type: "summary.book", data: { pages: [{ title: "A", body: "desc" }] } },
			"summary.imageSlider",
			label
		);
		expect(toSlider.warning).toContain("keeps the first page's text as the slider description.");
		expect(toSlider.warning).not.toContain("discards other pages");

		const fromSliderNoImages = transformSummaryBlockOnLayoutChange(
			{ type: "summary.imageSlider", data: { description: "Only text" } },
			"summary.steps",
			label
		);
		expect(fromSliderNoImages.block).toMatchObject({
			type: "summary.steps",
			data: { pages: [{ title: null, body: "Only text" }] },
		});
		expect(fromSliderNoImages.warning).toContain("keeps the description as the first page's body.");
		expect(fromSliderNoImages.warning).not.toContain("drops image selections");
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

	it("normalises malformed text/slider inputs to nullable fields", () => {
		const toTextPages = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.book",
				data: {
					pages: [{ title: 42 as any, body: "" }, { title: "Valid", body: false as any }],
					subTitle: "",
				},
			},
			"summary.pie",
			label
		);
		expect(toTextPages.block).toMatchObject({
			type: "summary.pie",
			data: {
				pages: [
					{ title: null, body: null },
					{ title: "Valid", body: null },
				],
				subTitle: null,
			},
		});

		const fromSlider = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.imageSlider",
				data: {
					description: "",
					image1Id: "   ",
					image2Id: "img-2",
				},
			},
			"summary.book",
			label
		);
		expect(fromSlider.block).toMatchObject({
			type: "summary.book",
			data: { pages: [{ title: null, body: null }] },
		});
		expect(fromSlider.warning).toContain("drops image selections");
	});

	it("handles non-array pages and second-image-only slider state", () => {
		const fromBrokenImagePages = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.diagram",
				data: { pages: "not-an-array" as any, subTitle: 123 as any },
			},
			"summary.steps",
			label
		);
		expect(fromBrokenImagePages.block).toMatchObject({
			type: "summary.steps",
			data: { pages: [], subTitle: null },
		});
		expect(fromBrokenImagePages.warning).toBeNull();

		const toSliderWithNonStringBody = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.book",
				data: { pages: [{ body: 0 as any }], subTitle: 42 as any },
			},
			"summary.imageSlider",
			label
		);
		expect(toSliderWithNonStringBody.block).toMatchObject({
			type: "summary.imageSlider",
			data: {
				subTitle: null,
				description: null,
				image1Id: null,
				image1Alt: null,
				image2Id: null,
				image2Alt: null,
				startPosition: "left",
			},
		});

		const fromSliderSecondImageOnly = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.imageSlider",
				data: {
					description: "Body",
					image1Id: "",
					image2Id: "img-2",
				},
			},
			"summary.steps",
			label
		);
		expect(fromSliderSecondImageOnly.warning).toContain("drops image selections");
	});

	it("handles malformed data payloads for text and slider transitions", () => {
		const fromTextWithNonObjectData = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.book",
				data: "bad-data" as any,
			},
			"summary.pie",
			label
		);
		expect(fromTextWithNonObjectData.block).toMatchObject({
			type: "summary.pie",
			data: {
				pages: [],
				subTitle: null,
			},
		});
		expect(fromTextWithNonObjectData.warning).toBeNull();

		const toSliderWithMissingPages = transformSummaryBlockOnLayoutChange(
			{
				type: "summary.book",
				data: { pages: "not-array" as any, subTitle: "kept" },
			},
			"summary.imageSlider",
			label
		);
		expect(toSliderWithMissingPages.block).toMatchObject({
			type: "summary.imageSlider",
			data: {
				subTitle: "kept",
				description: null,
				image1Id: null,
				image1Alt: null,
				image2Id: null,
				image2Alt: null,
				startPosition: "left",
			},
		});
		expect(toSliderWithMissingPages.warning).toContain(
			"keeps the first page's text as the slider description."
		);
	});
});
