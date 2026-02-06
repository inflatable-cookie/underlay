/**
 * Tests for local search utilities.
 */

import { describe, it, expect } from "vitest";
import {
	createLocalSearchFns,
	type LocalSearchOptions,
} from "../../src/patterns/local-search";
import type { SelectableRelation } from "../../src/patterns/RelationSelector/types";

// ============================================================================
// Test Data
// ============================================================================

interface TestItem {
	id: string;
	name: string;
	category: string;
	description?: string;
}

const testItems: TestItem[] = [
	{ id: "1", name: "Apple", category: "fruit", description: "A red fruit" },
	{ id: "2", name: "Banana", category: "fruit", description: "A yellow fruit" },
	{ id: "3", name: "Carrot", category: "vegetable", description: "An orange vegetable" },
	{ id: "4", name: "Broccoli", category: "vegetable" },
	{ id: "5", name: "Apricot", category: "fruit", description: "A small orange fruit" },
];

const defaultOptions: LocalSearchOptions<TestItem, SelectableRelation> = {
	toSelectable: (item) => ({
		id: item.id,
		label: item.name,
		description: item.description,
	}),
	getSearchText: (item) => [item.name, item.description ?? ""],
};

// ============================================================================
// createLocalSearchFns - search
// ============================================================================

describe("createLocalSearchFns - search", () => {
	describe("basic text search", () => {
		it("finds items by name", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("Apple");

			expect(result.items).toHaveLength(1);
			expect(result.items[0].label).toBe("Apple");
			expect(result.total).toBe(1);
		});

		it("finds multiple matching items", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("Ap");

			expect(result.items).toHaveLength(2); // Apple, Apricot
			expect(result.items.map((i) => i.label)).toContain("Apple");
			expect(result.items.map((i) => i.label)).toContain("Apricot");
		});

		it("search is case-insensitive", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);

			const result1 = await search("apple");
			const result2 = await search("APPLE");
			const result3 = await search("ApPlE");

			expect(result1.items).toHaveLength(1);
			expect(result2.items).toHaveLength(1);
			expect(result3.items).toHaveLength(1);
		});

		it("searches in all text fields", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("orange");

			// Matches Carrot (description) and Apricot (description)
			expect(result.items).toHaveLength(2);
		});

		it("returns empty for no matches", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("xyz");

			expect(result.items).toHaveLength(0);
			expect(result.total).toBe(0);
		});

		it("matches partial strings", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("an");

			// Matches BanANa, OrANge (in descriptions)
			expect(result.items.length).toBeGreaterThan(0);
		});
	});

	describe("with filters", () => {
		const optionsWithFilters: LocalSearchOptions<TestItem, SelectableRelation> = {
			...defaultOptions,
			applyFilters: (items, filters) => {
				if (!filters?.category) return items;
				return items.filter((item) => item.category === filters.category);
			},
		};

		it("applies filter before text search", async () => {
			const { search } = createLocalSearchFns(() => testItems, optionsWithFilters);
			const result = await search("", { filters: { category: "fruit" } });

			expect(result.items).toHaveLength(3); // Apple, Banana, Apricot
			expect(result.items.every((i) =>
				["Apple", "Banana", "Apricot"].includes(i.label)
			)).toBe(true);
		});

		it("combines filter with text search", async () => {
			const { search } = createLocalSearchFns(() => testItems, optionsWithFilters);
			const result = await search("orange", { filters: { category: "fruit" } });

			// Only Apricot matches "orange" AND is a fruit
			expect(result.items).toHaveLength(1);
			expect(result.items[0].label).toBe("Apricot");
		});

		it("works without filters", async () => {
			const { search } = createLocalSearchFns(() => testItems, optionsWithFilters);
			const result = await search("Apple");

			expect(result.items).toHaveLength(1);
		});
	});

	describe("result mapping", () => {
		it("converts items to SelectableRelation format", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("Apple");

			expect(result.items[0]).toEqual({
				id: "1",
				label: "Apple",
				description: "A red fruit",
			});
		});

		it("handles items without optional fields", async () => {
			const { search } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await search("Broccoli");

			expect(result.items[0].id).toBe("4");
			expect(result.items[0].label).toBe("Broccoli");
			expect(result.items[0].description).toBeUndefined();
		});
	});

	describe("reactive data source", () => {
		it("uses current items from getter", async () => {
			let items = [...testItems];
			const { search } = createLocalSearchFns(() => items, defaultOptions);

			const result1 = await search("Apple");
			expect(result1.items).toHaveLength(1);

			// Add new item
			items = [...items, { id: "6", name: "Green Apple", category: "fruit" }];
			const result2 = await search("Apple");
			expect(result2.items).toHaveLength(2);
		});
	});
});

// ============================================================================
// createLocalSearchFns - suggest
// ============================================================================

describe("createLocalSearchFns - suggest", () => {
	describe("basic suggestions", () => {
		it("returns all items when no limit", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await suggest();

			expect(result).toHaveLength(5);
		});

		it("respects maxSuggestions limit", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, {
				...defaultOptions,
				maxSuggestions: 3,
			});
			const result = await suggest();

			expect(result).toHaveLength(3);
		});

		it("returns items in original order", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await suggest();

			expect(result[0].label).toBe("Apple");
			expect(result[4].label).toBe("Apricot");
		});
	});

	describe("with filters", () => {
		const optionsWithFilters: LocalSearchOptions<TestItem, SelectableRelation> = {
			...defaultOptions,
			applyFilters: (items, filters) => {
				if (!filters?.category) return items;
				return items.filter((item) => item.category === filters.category);
			},
		};

		it("applies filter to suggestions", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, optionsWithFilters);
			const result = await suggest({ filters: { category: "vegetable" } });

			expect(result).toHaveLength(2); // Carrot, Broccoli
		});

		it("combines filter with maxSuggestions", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, {
				...optionsWithFilters,
				maxSuggestions: 2,
			});
			const result = await suggest({ filters: { category: "fruit" } });

			expect(result).toHaveLength(2); // Limited to 2 fruits
		});
	});

	describe("result mapping", () => {
		it("converts items to SelectableRelation format", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, defaultOptions);
			const result = await suggest();

			expect(result[0]).toEqual({
				id: "1",
				label: "Apple",
				description: "A red fruit",
			});
		});
	});

	describe("empty data", () => {
		it("returns empty array for empty source", async () => {
			const { suggest } = createLocalSearchFns(() => [], defaultOptions);
			const result = await suggest();

			expect(result).toEqual([]);
		});

		it("returns empty when filter excludes all", async () => {
			const { suggest } = createLocalSearchFns(() => testItems, {
				...defaultOptions,
				applyFilters: () => [], // Exclude everything
			});
			const result = await suggest();

			expect(result).toEqual([]);
		});
	});
});

// ============================================================================
// Integration tests
// ============================================================================

describe("createLocalSearchFns - integration", () => {
	it("search and suggest share the same data source", async () => {
		let items = [...testItems];
		const { search, suggest } = createLocalSearchFns(() => items, defaultOptions);

		// Initial state
		const suggest1 = await suggest();
		expect(suggest1).toHaveLength(5);

		// Modify data
		items = items.slice(0, 3);

		// Both should reflect changes
		const suggest2 = await suggest();
		expect(suggest2).toHaveLength(3);

		const searchResult = await search("Banana");
		expect(searchResult.items).toHaveLength(1);
	});

	it("can be used with complex selectable types", async () => {
		interface ExtendedSelectable extends SelectableRelation {
			category: string;
			priority: number;
		}

		const extendedOptions: LocalSearchOptions<TestItem, ExtendedSelectable> = {
			toSelectable: (item) => ({
				id: item.id,
				label: item.name,
				description: item.description,
				category: item.category,
				priority: item.category === "fruit" ? 1 : 2,
			}),
			getSearchText: (item) => [item.name, item.category],
		};

		const { search } = createLocalSearchFns(() => testItems, extendedOptions);
		const result = await search("fruit");

		expect(result.items).toHaveLength(3);
		expect(result.items[0].category).toBe("fruit");
		expect(result.items[0].priority).toBe(1);
	});
});
