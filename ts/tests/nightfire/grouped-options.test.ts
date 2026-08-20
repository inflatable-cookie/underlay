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
			{ type: "acow.question.timeline", label: "Timeline", category: "InteractiveQuestion" },
			{ type: "acow.question.numeric", label: "Numeric", category: "InteractiveQuestion" },
			{ type: "acow.question.dnd.cards", label: "Drag", category: "InteractiveQuestion" },
			{ type: "acow.question.mc_true_false", label: "MC Alt", category: "InteractiveQuestion" },
			{ type: "acow.question.multiple_choice", label: "MC", category: "InteractiveQuestion" },
			{ type: "acow.question.unknown", label: "Unknown", category: "InteractiveQuestion" },
			{ type: "acow.question.placeholder", label: "Placeholder", category: "InteractiveQuestion" },
			{ type: "acow.question.hitpoint", label: "Hitpoint", category: "InteractiveQuestion" },
			{ type: "acow.question.tf_multi", label: "TF Multi", category: "InteractiveQuestion" },
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

	it("covers category and subcategory comparator tie branches", () => {
		const result = buildGroupedOptions([
			{ type: "a", label: "Zulu", category: "MiscB", subcategory: "A" },
			{ type: "b", label: "Alpha", category: "MiscA", subcategory: "A" },
			{ type: "c", label: "Gamma", category: "CustomCategory", subcategory: "z" },
			{ type: "d", label: "Beta", category: "CustomCategory", subcategory: "a" },
			{ type: "e", label: "Delta", category: "CustomCategory", subcategory: "a" },
		]);

		expect(result.map((group) => group.category)).toEqual(["CustomCategory", "MiscA", "MiscB"]);
		const custom = result[0];
		expect(custom.options.map((o) => `${o.subcategory}:${o.label}`)).toEqual([
			"a:Beta",
			"a:Delta",
			"z:Gamma",
		]);
	});

	it("formats acronym labels and preserves whitespace-only category labels", () => {
		const result = buildGroupedOptions([
			{ type: "api", label: "API Block", category: "API_tools" },
			{ type: "blank", label: "Blank Category", category: "   " },
		]);

		expect(result.find((group) => group.category === "API_tools")?.label).toBe("API Tools");
		expect(result.find((group) => group.category === "   ")?.label).toBe("   ");
	});

	it("handles case-insensitive category label ties", () => {
		const result = buildGroupedOptions([
			{ type: "one", label: "One", category: "Alpha" },
			{ type: "two", label: "Two", category: "alpha" },
		]);

		expect(result.map((group) => group.label)).toEqual(["Alpha", "Alpha"]);
	});

	it("sorts reverse category and subcategory comparisons", () => {
		const result = buildGroupedOptions([
			{ type: "z-cat", label: "Zulu Category", category: "zeta" },
			{ type: "a-cat", label: "Alpha Category", category: "alpha" },
			{ type: "sub-z", label: "Sub Z", category: "SubSort", subcategory: "z" },
			{ type: "sub-a", label: "Sub A", category: "SubSort", subcategory: "a" },
			{ type: "sub-none", label: "Sub None", category: "SubSort" },
			{ type: "none-1", label: "No Category A" },
			{ type: "none-2", label: "No Category B" },
		]);

		expect(result.map((group) => group.category)).toEqual(["alpha", "SubSort", "zeta", null]);
		const subSort = result.find((group) => group.category === "SubSort");
		expect(subSort?.options.map((o) => `${o.subcategory ?? "null"}:${o.label}`)).toEqual([
			"null:Sub None",
			"a:Sub A",
			"z:Sub Z",
		]);
	});

	it("sorts unknown and null categories lexically when priority ties", () => {
		const result = buildGroupedOptions([
			{ type: "none", label: "No Category" },
			{ type: "beta", label: "Beta", category: "beta" },
			{ type: "alpha", label: "Alpha", category: "alpha" },
		]);

		expect(result.map((group) => group.category)).toEqual(["alpha", "beta", null]);
	});

	it("covers reverse subcategory comparator branch", () => {
		const result = buildGroupedOptions([
			{ type: "s1", label: "One", category: "SortBranch", subcategory: "b" },
			{ type: "s2", label: "Two", category: "SortBranch", subcategory: "a" },
		]);

		const group = result.find((entry) => entry.category === "SortBranch");
		expect(group?.options.map((entry) => `${entry.subcategory}:${entry.label}`)).toEqual([
			"a:Two",
			"b:One",
		]);
	});
});
