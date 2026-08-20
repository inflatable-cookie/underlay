import { describe, expect, it } from "vitest";
import { transformSummaryBlockOnLayoutChange } from "../../../src/nightfire/editor/summary-transform";
import { label } from "./fixtures";

describe("nightfire/editor/summary-transform slider transitions", () => {
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

		const result = transformSummaryBlockOnLayoutChange(current, "summary.image_slider", label);
		expect(result.block).toMatchObject({
			type: "summary.image_slider",
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
			type: "summary.image_slider",
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

	it("handles non-dropping slider and slider-to-text conversions", () => {
		const toSlider = transformSummaryBlockOnLayoutChange(
			{ type: "summary.book", data: { pages: [{ title: "A", body: "desc" }] } },
			"summary.image_slider",
			label
		);
		expect(toSlider.warning).toContain("keeps the first page's text as the slider description.");
		expect(toSlider.warning).not.toContain("discards other pages");

		const fromSliderNoImages = transformSummaryBlockOnLayoutChange(
			{ type: "summary.image_slider", data: { description: "Only text" } },
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
});
