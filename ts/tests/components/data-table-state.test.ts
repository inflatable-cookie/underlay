import { describe, expect, it } from "vitest";
import {
	getCellDisplayValue,
	getColumnWidthValue,
	getNextSort,
	toggleHiddenColumn,
	updateFilters,
} from "../../src/components/data-table/state";

describe("components/data-table/state", () => {
	it("resolves display values with nested keys and formatters", () => {
		const row = { user: { name: "Clay" }, createdAt: new Date("2026-01-01T00:00:00Z") };
		expect(getCellDisplayValue(row, { key: "user.name" })).toBe("Clay");
		expect(getCellDisplayValue(row, { key: "missing.value" })).toBe("");
		expect(getCellDisplayValue(row, { key: "createdAt" })).toMatch(/\d{1,2}\/\d{1,2}\/\d{4}/);
		expect(getCellDisplayValue(row, { key: "user.name", formatter: (v) => `Name:${v as string}` })).toBe("Name:Clay");
	});

	it("computes sort/filters/hidden-columns/width", () => {
		expect(getNextSort(null, "title")).toEqual({ key: "title", direction: "asc" });
		expect(getNextSort({ key: "title", direction: "asc" }, "title")).toEqual({
			key: "title",
			direction: "desc",
		});
		expect(updateFilters({ q: "a" }, "status", "active")).toEqual({ q: "a", status: "active" });

		const hidden = toggleHiddenColumn(new Set(["a"]), "a");
		expect(hidden.has("a")).toBe(false);
		expect(toggleHiddenColumn(hidden, "b").has("b")).toBe(true);

		expect(getColumnWidthValue({ key: "x", width: "200px" })).toBe("200px");
		expect(getColumnWidthValue({ key: "x", width: "200px", minWidth: "120px" })).toBe(
			"minmax(120px, 200px)"
		);
		expect(getColumnWidthValue({ key: "x", minWidth: "90px" })).toBe("minmax(90px, 1fr)");
		expect(getColumnWidthValue({ key: "x" })).toBe("minmax(100px, 1fr)");
	});
});
