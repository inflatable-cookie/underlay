import { describe, expect, it } from "vitest";
import {
	getHideableColumns,
	getTotalPages,
	getVisibleColumns,
	isAllSelected,
	isSomeSelected,
} from "../../src/components/data-table/view";

describe("components/data-table/view", () => {
	it("filters visible and hideable columns", () => {
		const columns = [
			{ key: "id" },
			{ key: "secret", hideable: false },
			{ key: "name", hideable: true },
		] as any[];

		expect(getVisibleColumns(columns, new Set(["secret"]))).toEqual([
			{ key: "id" },
			{ key: "name", hideable: true },
		]);
		expect(getHideableColumns(columns)).toEqual([
			{ key: "id" },
			{ key: "name", hideable: true },
		]);
	});

	it("computes pagination and selection status", () => {
		expect(getTotalPages(null)).toBe(1);
		expect(getTotalPages({ total: 120, limit: 50 } as any)).toBe(3);

		expect(isAllSelected(3, 3)).toBe(true);
		expect(isAllSelected(0, 0)).toBe(false);
		expect(isSomeSelected(3, 2)).toBe(true);
		expect(isSomeSelected(3, 3)).toBe(false);
		expect(isSomeSelected(3, 0)).toBe(false);
	});
});
