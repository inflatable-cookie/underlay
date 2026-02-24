import { describe, it, expect } from "vitest";
import { buildPushedContextStack } from "../../src/patterns/navigation-stack";
import type { NavigationContext } from "../../src/patterns/navigation-types";

function ctx(
	href: string,
	type: NavigationContext["type"],
	label = href
): NavigationContext {
	return { href, type, label };
}

describe("buildPushedContextStack", () => {
	it("appends a new context and deduplicates existing href", () => {
		const stack = [ctx("/list", "list"), ctx("/detail/1", "detail")];
		const next = buildPushedContextStack(stack, ctx("/detail/1", "detail", "Updated"), 5);

		expect(next).toEqual([ctx("/list", "list"), ctx("/detail/1", "detail", "Updated")]);
	});

	it("collapses top list->list transitions", () => {
		const stack = [ctx("/projects", "list"), ctx("/users", "list")];
		const next = buildPushedContextStack(stack, ctx("/teams", "list"), 5);

		expect(next).toEqual([ctx("/projects", "list"), ctx("/teams", "list")]);
	});

	it("does not collapse non-list transitions", () => {
		const stack = [ctx("/projects", "list"), ctx("/users/1", "detail")];
		const next = buildPushedContextStack(stack, ctx("/users/1/edit", "edit"), 5);

		expect(next).toEqual([ctx("/projects", "list"), ctx("/users/1", "detail"), ctx("/users/1/edit", "edit")]);
	});

	it("trims stack to maxDepth from the end", () => {
		const stack = [ctx("/a", "list"), ctx("/b", "detail"), ctx("/c", "edit")];
		const next = buildPushedContextStack(stack, ctx("/d", "detail"), 2);

		expect(next).toEqual([ctx("/c", "edit"), ctx("/d", "detail")]);
	});
});
