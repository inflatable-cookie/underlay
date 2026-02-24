import { describe, expect, it } from "vitest";
import {
	buildGridColumns,
	getRenderedActionHref,
	getRenderedCellValue,
	getRenderedRowActions,
} from "../../src/components/data-table/render";

describe("components/data-table/render", () => {
	it("renders cell values and action metadata", () => {
		const row = { id: "1", user: { name: "Clay" } };
		const column = { key: "user.name" } as any;
		expect(getRenderedCellValue(row, column)).toBe("Clay");

		const actions = [
			{ label: "Edit", href: "/items/1", show: () => true },
			{ label: "Hidden", show: () => false },
		] as any;
		expect(getRenderedRowActions(row, actions)).toHaveLength(1);
		expect(getRenderedActionHref({ href: (r: typeof row) => `/items/${r.id}` } as any, row)).toBe("/items/1");
	});

	it("builds CSS grid columns from selection/columns/action settings", () => {
		const columns = [
			{ key: "title", minWidth: "140px" },
			{ key: "status", width: "180px", minWidth: "120px" },
		] as any;

		expect(buildGridColumns(true, columns, [])).toBe("40px minmax(140px, 1fr) minmax(120px, 180px)");
		expect(buildGridColumns(false, columns, [{ label: "Edit" }] as any)).toBe(
			"minmax(140px, 1fr) minmax(120px, 180px) 80px"
		);
	});
});
