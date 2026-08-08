import { describe, expect, it } from "vitest";
import {
	EntityDetailPage,
	EntityList,
	EntityListPage,
	toPagedListResult,
} from "@inflatable-cookie/underlay/templates";

describe("template package compatibility", () => {
	it("exposes retained template components and helpers", () => {
		expect(EntityListPage).toBeDefined();
		expect(EntityDetailPage).toBeDefined();
		expect(EntityList).toBeDefined();
		expect(
			toPagedListResult({
				data: ["a"],
				total: 1,
				hasMore: false,
			}),
		).toEqual({
			data: ["a"],
			total: 1,
			hasMore: false,
		});
	});
});
