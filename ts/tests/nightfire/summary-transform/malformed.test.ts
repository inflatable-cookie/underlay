import { describe, expect, it } from "vitest";
import { transformSummaryBlockOnLayoutChange } from "../../../src/nightfire/editor/summary-transform";
import { label } from "./fixtures";

describe("nightfire/editor/summary-transform malformed input", () => {
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
