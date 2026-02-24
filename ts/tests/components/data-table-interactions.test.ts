import { describe, expect, it, vi } from "vitest";
import {
	applySelectAll,
	applySelectRow,
	emitLimitChange,
	emitNextPage,
	getNextFiltersState,
	getNextSortState,
	runRowAction,
} from "../../src/components/data-table/interactions";

describe("components/data-table/interactions", () => {
	it("computes sort/filter state updates", () => {
		expect(getNextSortState(null, { key: "title" } as any)).toEqual({
			key: "title",
			direction: "asc",
		});
		expect(getNextSortState({ key: "title", direction: "asc" } as any, { key: "title" } as any)).toEqual({
			key: "title",
			direction: "desc",
		});

		expect(getNextFiltersState({ q: "old" }, "q", "new")).toEqual({ q: "new" });
	});

	it("emits page/limit changes only when valid", () => {
		const onPage = vi.fn();
		emitNextPage(2, 3, onPage);
		expect(onPage).toHaveBeenCalledWith(2);
		emitNextPage(4, 3, onPage);
		expect(onPage).toHaveBeenCalledTimes(1);

		const onLimit = vi.fn();
		emitLimitChange(50, onLimit, onPage);
		expect(onLimit).toHaveBeenCalledWith(50);
		expect(onPage).toHaveBeenCalledWith(1);
	});

	it("applies select-all and single-row selection", () => {
		const rows = [{ id: 1 }, { id: 2 }];
		const onSelect = vi.fn();

		expect(applySelectAll(rows, [], false, onSelect)).toEqual(rows);
		expect(onSelect).toHaveBeenCalledWith(rows);

		expect(applySelectAll(rows, rows, true)).toEqual([]);
		expect(applySelectRow([], rows[0])).toEqual([rows[0]]);
		expect(applySelectRow([rows[0]], rows[0])).toEqual([]);
	});

	it("runs row actions with guards and callbacks", () => {
		const row = { id: 1 };
		const onClick = vi.fn();
		const onAction = vi.fn();
		const confirmFn = vi.fn().mockReturnValue(true);

		runRowAction({ label: "Edit", onClick, confirm: "Continue?" } as any, row, onAction, confirmFn);
		expect(confirmFn).toHaveBeenCalledWith("Continue?");
		expect(onClick).toHaveBeenCalledWith(row);
		expect(onAction).toHaveBeenCalledWith({ action: "Edit", row });

		runRowAction({ label: "Disabled", disabled: true } as any, row, onAction, confirmFn);
		runRowAction({ label: "Separator", separator: true } as any, row, onAction, confirmFn);
		expect(onAction).toHaveBeenCalledTimes(1);

		const denied = vi.fn().mockReturnValue(false);
		runRowAction({ label: "Delete", confirm: "Sure?" } as any, row, onAction, denied);
		expect(onAction).toHaveBeenCalledTimes(1);
	});
});
