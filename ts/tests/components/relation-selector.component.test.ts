import { fireEvent, render, screen } from "@testing-library/svelte";
import { createRawSnippet } from "svelte";
import { describe, expect, it, vi } from "vitest";
import RelationSelector from "../../src/patterns/RelationSelector/RelationSelector.svelte";
import type {
	SearchOptions,
	SelectableRelation,
	SuggestionOptions,
} from "../../src/patterns/RelationSelector/types.js";

interface Venue extends SelectableRelation {
	capacity: number;
}

describe("RelationSelector", () => {
	it("hydrates a drill-down selection and passes full records to custom item rendering", async () => {
		const existing = {
			id: "a",
			label: "Previously selected",
			capacity: 20,
			metadata: { district: "Central" },
		};
		const suggested = {
			id: "b",
			label: "Community hall",
			capacity: 100,
			metadata: { district: "North" },
		};
		const searched = {
			id: "c",
			label: "Studio",
			capacity: 50,
			metadata: { district: "North" },
		};
		const suggestions = vi.fn(async (options?: SuggestionOptions) =>
			options?.filters?.area === "north" ? [suggested] : [existing],
		);
		const search = vi.fn(async (_query: string, _options?: SearchOptions) => ({
			items: [searched],
			total: 1,
		}));
		const onChange = vi.fn();

		render(RelationSelector, {
			label: "Venue",
			value: existing.id,
			suggestions,
			search,
			onChange,
			drillDown: {
				levels: [
					{
						key: "area",
						label: "Area",
						suggestions: async () => [{ id: "north", label: "North" }],
						search: async () => ({ items: [], total: 0 }),
					},
				],
			},
			renderItem: createRawSnippet<[Venue, boolean]>((item) => ({
				render: () =>
					`<span>${item().label} · ${item().capacity} · ${item().metadata?.district}</span>`,
			})),
		});

		await fireEvent.click(await screen.findByText(existing.label));
		await fireEvent.click(await screen.findByRole("button", { name: "North" }));
		expect(await screen.findByText("Community hall · 100 · North")).toBeDefined();
		expect(suggestions).toHaveBeenLastCalledWith({
			recentHints: undefined,
			filters: { area: "north" },
		});
		expect(search).not.toHaveBeenCalled();

		await fireEvent.input(await screen.findByPlaceholderText("Search..."), {
			target: { value: "Studio" },
		});
		await fireEvent.click(await screen.findByText("Studio · 50 · North"));
		expect(search).toHaveBeenCalledExactlyOnceWith("Studio", {
			limit: 20,
			offset: 0,
			filters: { area: "north" },
		});
		expect(onChange).toHaveBeenCalledExactlyOnceWith("c");
	});

	it("renders a suggestions failure in the picker", async () => {
		render(RelationSelector, {
			label: "Venue",
			search: async () => ({ items: [], total: 0 }),
			suggestions: async () => {
				throw new Error("Venue service unavailable");
			},
		});

		await fireEvent.click(screen.getByText("Select..."));
		expect(await screen.findByText("Could not load suggestions")).toBeDefined();
		expect(screen.getByText("Venue service unavailable")).toBeDefined();
		expect(screen.queryByText("No suggestions")).toBeNull();
	});

	it("keeps late label hydration independent of drill-down results", async () => {
		let finishHydration: (items: SelectableRelation[]) => void = () => {};
		const hydration = new Promise<SelectableRelation[]>((resolve) => {
			finishHydration = resolve;
		});
		let finishLevel: (items: SelectableRelation[]) => void = () => {};
		const level = new Promise<SelectableRelation[]>((resolve) => {
			finishLevel = resolve;
		});
		const suggestions = vi.fn(async (options?: SuggestionOptions) =>
			options?.filters?.area === "north" ? [{ id: "hall", label: "Community hall" }] : hydration,
		);
		render(RelationSelector, {
			label: "Venue",
			value: "existing",
			search: async () => ({ items: [], total: 0 }),
			suggestions,
			drillDown: {
				levels: [
					{
						key: "area",
						label: "Area",
						suggestions: () => level,
						search: async () => ({ items: [], total: 0 }),
					},
				],
			},
		});

		await fireEvent.click(screen.getByText("Select..."));
		expect(await screen.findByText("Loading items...")).toBeDefined();
		expect(screen.queryByText("No suggestions")).toBeNull();
		finishLevel([{ id: "north", label: "North" }]);
		await fireEvent.click(await screen.findByRole("button", { name: "North" }));
		expect(await screen.findByRole("button", { name: "Community hall" })).toBeDefined();
		finishHydration([{ id: "existing", label: "Previously selected" }]);
		expect(await screen.findByText("Previously selected")).toBeDefined();
		expect(screen.getByRole("button", { name: "Community hall" })).toBeDefined();
	});
});
