import { describe, expect, it } from "vitest";
import {
	appendSuggestionParams,
	buildSuggestionParams,
	formatHintsParam,
	parseHintsParam,
} from "@decodelabs/underlay/client/suggestions";

describe("client/suggestions", () => {
	it("formats and parses recent hint params", () => {
		expect(formatHintsParam(["id1", "id2"])).toBe("id1,id2");
		expect(parseHintsParam(null)).toEqual([]);
		expect(parseHintsParam(undefined)).toEqual([]);
		expect(parseHintsParam("  ")).toEqual([]);
		expect(parseHintsParam("id1,, id2,")).toEqual(["id1", "id2"]);
	});

	it("builds suggestion request params from bounded options", () => {
		const params = buildSuggestionParams({
			suggestions: true,
			recentHints: ["a", "b"],
			query: "  alpha  ",
			limit: 12.8,
		});

		expect(params.toString()).toBe(
			"suggestions=true&recentHints=a%2Cb&query=alpha&limit=12",
		);
		expect(
			buildSuggestionParams({
				suggestions: false,
				recentHints: [],
				query: " ",
				limit: Number.NaN,
			}).toString(),
		).toBe("");
		expect(buildSuggestionParams({ limit: -5 }).toString()).toBe("limit=1");
	});

	it("appends and merges params without duplicating keys", () => {
		expect(
			appendSuggestionParams("/api/items", {
				suggestions: true,
				recentHints: ["id1"],
				query: "beta",
				limit: 5,
			}),
		).toBe("/api/items?suggestions=true&recentHints=id1&query=beta&limit=5");
		expect(
			appendSuggestionParams("/api/items?archived=true", { suggestions: true }),
		).toBe("/api/items?archived=true&suggestions=true");
		expect(
			appendSuggestionParams("/api/items?suggestions=false", {
				suggestions: true,
			}),
		).toBe("/api/items?suggestions=true");
		expect(appendSuggestionParams("/api/items")).toBe("/api/items");
	});
});
