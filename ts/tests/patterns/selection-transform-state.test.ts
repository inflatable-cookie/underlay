import { describe, expect, it } from "vitest";
import { buildSelectionTransformState } from "../../src/patterns/selection-transform-state";

describe("patterns/selection-transform-state.ts", () => {
	it("returns empty transform state when selection mode is off", () => {
		expect(
			buildSelectionTransformState({
				selectionMode: false,
				selectedIds: ["a"],
				buildCopyHref: (ids) => `/copy?ids=${ids.join(",")}`,
				buildMoveHref: (ids) => `/move?ids=${ids.join(",")}`,
			})
		).toEqual({
			canLaunchBatch: false,
			canLaunchCopy: false,
			copyHref: "",
			moveHref: "",
		});
	});

	it("supports single-item copy and multi-item move semantics", () => {
		expect(
			buildSelectionTransformState({
				selectionMode: true,
				selectedIds: ["a"],
				buildCopyHref: (ids) => `/copy?ids=${ids.join(",")}`,
				buildMoveHref: (ids) => `/move?ids=${ids.join(",")}`,
				requireSingleForCopy: true,
			})
		).toEqual({
			canLaunchBatch: true,
			canLaunchCopy: true,
			copyHref: "/copy?ids=a",
			moveHref: "/move?ids=a",
		});

		expect(
			buildSelectionTransformState({
				selectionMode: true,
				selectedIds: ["a", "b"],
				buildCopyHref: (ids) => `/copy?ids=${ids.join(",")}`,
				buildMoveHref: (ids) => `/move?ids=${ids.join(",")}`,
				requireSingleForCopy: true,
			})
		).toEqual({
			canLaunchBatch: true,
			canLaunchCopy: false,
			copyHref: "",
			moveHref: "/move?ids=a,b",
		});
	});
});
