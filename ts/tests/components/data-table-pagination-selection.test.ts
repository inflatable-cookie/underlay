import { describe, expect, it } from "vitest";
import {
	getNextPage,
	getPageAfterLimitChange,
	toggleRowSelection,
	toggleSelectAllRows,
} from "../../src/components/data-table/pagination-selection";

describe("components/data-table/pagination-selection", () => {
	it("calculates next page safely", () => {
		expect(getNextPage(1, 3)).toBe(1);
		expect(getNextPage(0, 3)).toBeNull();
		expect(getNextPage(4, 3)).toBeNull();
		expect(getPageAfterLimitChange()).toBe(1);
	});

	it("toggles select-all and single-row selection", () => {
		const rows = [{ id: 1 }, { id: 2 }];
		expect(toggleSelectAllRows(rows, [], false)).toEqual(rows);
		expect(toggleSelectAllRows(rows, rows, true)).toEqual([]);
		expect(toggleRowSelection([], rows[0])).toEqual([rows[0]]);
		expect(toggleRowSelection([rows[0], rows[1]], rows[0])).toEqual([rows[1]]);
	});
});
