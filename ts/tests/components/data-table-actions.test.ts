import { describe, expect, it } from "vitest";
import { getRowActionHref, getVisibleRowActions } from "../../src/components/data-table/actions";

describe("components/data-table/actions", () => {
	it("filters visible row actions based on show predicate", () => {
		const row = { id: 1, locked: false };
		const actions = [
			{ href: "/edit" },
			{ show: (r: typeof row) => r.locked },
			{ show: (r: typeof row) => !r.locked },
		];

		expect(getVisibleRowActions(row, actions)).toHaveLength(2);
		expect(getVisibleRowActions(row, (r) => actions.filter((_, i) => i !== (r.id - 1)))).toHaveLength(1);
	});

	it("resolves row action href from literal and function values", () => {
		const row = { id: "abc" };
		expect(getRowActionHref({ href: "/items/abc" }, row)).toBe("/items/abc");
		expect(getRowActionHref({ href: (r: typeof row) => `/items/${r.id}` }, row)).toBe("/items/abc");
		expect(getRowActionHref({}, row)).toBeUndefined();
	});
});
