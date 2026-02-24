import { beforeEach, describe, expect, it, vi } from "vitest";

const stores = vi.hoisted(() => ({
	local: new Map<string, unknown>(),
	session: new Map<string, unknown>(),
}));

function makeStore(map: Map<string, unknown>) {
	return {
		get<T>(key: string, fallback: T): T {
			return (map.has(key) ? (map.get(key) as T) : fallback);
		},
		set<T>(key: string, value: T): void {
			map.set(key, value);
		},
		remove(key: string): void {
			map.delete(key);
		},
	};
}

vi.mock("../../src/patterns/storage.js", () => ({
	storage: {
		local: makeStore(stores.local),
		session: makeStore(stores.session),
	},
}));

import {
	appendSuggestionParams,
	buildSuggestionParams,
	createSelectionHistory,
	formatHintsParam,
	parseHintsParam,
} from "../../src/patterns/selection-history";

describe("patterns/selection-history", () => {
	beforeEach(() => {
		stores.local.clear();
		stores.session.clear();
		vi.useRealTimers();
	});

	it("tracks ids, deduplicates, limits, and supports remove/clear", () => {
		vi.useFakeTimers();
		vi.setSystemTime(new Date("2026-01-01T00:00:00Z"));

		const history = createSelectionHistory("levels", { maxItems: 3, storageType: "local" });
		history.track("a");
		vi.setSystemTime(new Date("2026-01-01T00:00:01Z"));
		history.track("b");
		history.track("a"); // dedupe + move to front
		history.track("c");
		history.track("d"); // trims to 3

		expect(history.getRecentIds()).toEqual(["d", "c", "a"]);
		expect(history.getRecentIds(2)).toEqual(["d", "c"]);
		expect(history.hasRecent("a")).toBe(true);
		expect(history.hasRecent("b")).toBe(false);

		history.remove("c");
		expect(history.getRecentIds()).toEqual(["d", "a"]);

		history.clear();
		expect(history.getRecentIds()).toEqual([]);
		expect(history.getStorageKey()).toBe("underlay:selection-history:levels");
	});

	it("tracks multiple ids in provided order and supports namespace/session storage", () => {
		const history = createSelectionHistory("items", {
			maxItems: 4,
			storageType: "session",
			namespace: "ns",
		});

		history.track("x");
		history.trackMultiple(["a", "b", "x", "c"]);
		expect(history.getRecentIds()).toEqual(["a", "b", "x", "c"]);

		history.trackMultiple([]);
		expect(history.getRecentIds()).toEqual(["a", "b", "x", "c"]);

		expect(history.getStorageKey()).toBe("underlay:selection-history:ns:items");
	});

	it("formats/parses hints and builds/appends suggestion params", () => {
		expect(formatHintsParam(["id1", "id2"])).toBe("id1,id2");
		expect(parseHintsParam(null)).toEqual([]);
		expect(parseHintsParam("  ")).toEqual([]);
		expect(parseHintsParam("id1,,id2, ")).toEqual(["id1", "id2"]);

		const params = buildSuggestionParams({ suggestions: true, recentHints: ["id1", "id2"] });
		expect(params.toString()).toBe("suggestions=true&recentHints=id1%2Cid2");
		expect(buildSuggestionParams({ suggestions: false, recentHints: [] }).toString()).toBe("");

		expect(
			appendSuggestionParams("/api/items", { suggestions: true, recentHints: ["id1"] })
		).toBe("/api/items?suggestions=true&recentHints=id1");
		expect(
			appendSuggestionParams("/api/items?archived=true", { suggestions: true })
		).toBe("/api/items?archived=true&suggestions=true");
		expect(appendSuggestionParams("/api/items")).toBe("/api/items");
	});
});
