import { describe, expect, it } from "vitest";
import { get } from "svelte/store";
import {
	createOptimisticCounter,
	createOptimisticList,
	createOptimisticToggle,
	createOptimisticValue,
} from "../../src/patterns/optimistic";

describe("patterns/optimistic (barrel)", () => {
	it("re-exports optimistic builders and they behave as expected", () => {
		const counter = createOptimisticCounter(1);
		counter.increment(2).confirm();
		expect(get(counter)).toBe(3);

		const toggle = createOptimisticToggle(false);
		toggle.toggle().confirm();
		expect(get(toggle)).toBe(true);

		const value = createOptimisticValue("a");
		value.set("b").confirm();
		expect(get(value)).toBe("b");

		const list = createOptimisticList([{ id: "1", label: "One" }]);
		list.add({ label: "Two" } as any).confirm({ id: "2", label: "Two" });
		expect(get(list)).toEqual([
			{ id: "1", label: "One" },
			{ id: "2", label: "Two" },
		]);
	});
});
