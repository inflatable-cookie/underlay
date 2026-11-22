import { describe, expect, it } from "vitest";
import { toPagedListResult } from "../../src/templates/paged-list";

describe("toPagedListResult", () => {
	it("maps the public client paged list shape onto the template loader shape", () => {
		const result = toPagedListResult({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: true,
		});

		expect(result).toEqual({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: true,
		});
	});

	it("accepts legacy-compatible page-shaped inputs without hasMore", () => {
		const result = toPagedListResult({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
		});

		expect(result).toEqual({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: undefined,
		});
	});

	it("falls back to wire has_more when hasMore is absent", () => {
		const result = toPagedListResult({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			has_more: true,
		});

		expect(result).toEqual({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: true,
		});
	});

	it("prefers explicit hasMore over wire has_more when both are present", () => {
		const result = toPagedListResult({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: false,
			has_more: true,
		});

		expect(result).toEqual({
			data: [{ id: "p1", title: "Project One" }],
			total: 42,
			hasMore: false,
		});
	});
});
