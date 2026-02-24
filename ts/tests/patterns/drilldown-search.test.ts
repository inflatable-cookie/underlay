import { describe, expect, it } from "vitest";
import { createLocalDrillDownSearchFns } from "../../src/patterns/drilldown-search";

type Item = {
	id: string;
	label: string;
	description: string;
	moduleId: string;
};

const items: Item[] = [
	{ id: "1", label: "Intro", description: "Welcome module", moduleId: "m1" },
	{ id: "2", label: "Install", description: "Setup guide", moduleId: "m1" },
	{ id: "3", label: "Advanced", description: "Deep dive", moduleId: "m2" },
];

describe("patterns/drilldown-search", () => {
	it("searches case-insensitively and returns totals", async () => {
		const { search } = createLocalDrillDownSearchFns(
			() => items,
			{
				toItem: (item) => ({ id: item.id, label: item.label }),
				getSearchText: (item) => [item.label, item.description],
			}
		);

		await expect(search("intro", {})).resolves.toEqual({
			items: [{ id: "1", label: "Intro" }],
			total: 1,
		});

		await expect(search("GUIDE", {})).resolves.toEqual({
			items: [{ id: "2", label: "Install" }],
			total: 1,
		});
	});

	it("applies context filtering to search and suggestions", async () => {
		const { search, suggest } = createLocalDrillDownSearchFns(
			() => items,
			{
				toItem: (item) => ({ id: item.id, label: item.label }),
				getSearchText: (item) => [item.label, item.description],
				applyContext: (source, context) => {
					if (!context.module) return source;
					return source.filter((item) => item.moduleId === context.module);
				},
				maxSuggestions: 1,
			}
		);

		await expect(search("", { module: "m1" })).resolves.toEqual({
			items: [
				{ id: "1", label: "Intro" },
				{ id: "2", label: "Install" },
			],
			total: 2,
		});

		await expect(suggest({ module: "m1" })).resolves.toEqual([
			{ id: "1", label: "Intro" },
		]);
	});
});
