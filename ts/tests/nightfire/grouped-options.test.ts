import { describe, expect, it } from "vitest";
import { buildGroupedOptions } from "../../src/nightfire/editor/grouped-options";

describe("nightfire/editor/grouped-options", () => {
	it("groups by category with priority and formatted labels", () => {
		const result = buildGroupedOptions([
			{ type: "x", label: "Other A" },
			{ type: "t", label: "Text B", category: "Text" },
			{ type: "m", label: "Media A", category: "Media" },
			{ type: "i", label: "IQ", category: "InteractiveQuestion" },
		]);

		expect(result.map((g) => g.label)).toEqual(["Text", "Media", "Interactive Question", "Other"]);
		expect(result[3].category).toBeNull();
	});

	it("infers and sorts interactive question subcategories by priority then label", () => {
		const result = buildGroupedOptions([
			{ type: "acow.question.timeline@1", label: "Timeline", category: "InteractiveQuestion" },
			{ type: "acow.question.numeric@1", label: "Numeric", category: "InteractiveQuestion" },
			{ type: "acow.question.dnd.cards@1", label: "Drag", category: "InteractiveQuestion" },
			{ type: "acow.question.mcTrueFalse@1", label: "MC Alt", category: "InteractiveQuestion" },
			{ type: "acow.question.multipleChoice@1", label: "MC", category: "InteractiveQuestion" },
			{ type: "acow.question.unknown@1", label: "Unknown", category: "InteractiveQuestion" },
			{ type: "acow.question.placeholder@1", label: "Placeholder", category: "InteractiveQuestion" },
			{ type: "acow.question.hitpoint@1", label: "Hitpoint", category: "InteractiveQuestion" },
			{ type: "acow.question.tfMulti@1", label: "TF Multi", category: "InteractiveQuestion" },
		]);

		const interactive = result.find((g) => g.category === "InteractiveQuestion");
		expect(interactive).toBeDefined();
		expect(interactive?.options.map((o) => `${o.subcategory}:${o.label}`)).toEqual([
			"MultipleChoice:MC",
			"MultipleChoice:MC Alt",
			"MultipleChoice:TF Multi",
			"Input:Numeric",
			"Input:Placeholder",
			"Interactive:Hitpoint",
			"Interactive:Timeline",
			"DragAndDrop:Drag",
			"Other:Unknown",
		]);
	});

	it("preserves explicit subcategory for non-interactive categories", () => {
		const result = buildGroupedOptions([
			{ type: "x", label: "Alpha", category: "CustomCategory", subcategory: "B" },
			{ type: "y", label: "Beta", category: "CustomCategory", subcategory: "A" },
		]);

		expect(result[0].label).toBe("Custom Category");
		expect(result[0].options.map((o) => o.subcategory)).toEqual(["A", "B"]);
	});
});
