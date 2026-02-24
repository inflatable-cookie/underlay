import { describe, it, expect } from "vitest";
import {
	resolveBackButtonInfo,
	consumeBackNavigation,
	computeResolvedBackInfo
} from "../../src/patterns/navigation-back-info";

describe("navigation-back-info", () => {
	it("uses fallback when context target does not match current path", () => {
		const result = resolveBackButtonInfo(
			{ label: "Users", href: "/users", targetHref: "/users/1" },
			"Back",
			"/home",
			() => false
		);
		expect(result).toEqual({
			label: "Back",
			href: "/home",
			isContextual: false
		});
	});

	it("builds contextual back info when context is valid", () => {
		const result = resolveBackButtonInfo(
			{ label: "Users", href: "/users", targetHref: "/users/1" },
			"Back",
			"/home",
			() => true
		);
		expect(result).toEqual({
			label: "Back to Users",
			href: "/users",
			isContextual: true
		});
	});

	it("uses fallback when no context exists", () => {
		const result = resolveBackButtonInfo(null, "Back", "/home", () => true);
		expect(result).toEqual({
			label: "Back",
			href: "/home",
			isContextual: false
		});
	});

	it("consumeBackNavigation returns context href and returnTo for valid context", () => {
		const result = consumeBackNavigation(
			{ label: "Projects", href: "/projects", targetHref: "/projects/1" },
			"Back",
			"/home",
			() => true
		);
		expect(result).toEqual({
			backInfo: {
				label: "Back to Projects",
				href: "/projects",
				isContextual: true
			},
			returnTo: "/projects"
		});
	});

	it("consumeBackNavigation falls back for missing/invalid context", () => {
		expect(
			consumeBackNavigation(null, "Back", "/home", () => true)
		).toEqual({
			backInfo: { label: "Back", href: "/home", isContextual: false },
			returnTo: "/home"
		});

		expect(
			consumeBackNavigation(
				{ label: "X", href: "/x", targetHref: "/target" },
				"Back",
				"/home",
				() => false
			)
		).toEqual({
			backInfo: { label: "Back", href: "/home", isContextual: false },
			returnTo: "/home"
		});
	});

	it("computeResolvedBackInfo prioritizes contextual and optional fallback", () => {
		expect(
			computeResolvedBackInfo({ label: "Back to A", href: "/a", isContextual: true }, {
				label: "Fallback",
				href: "/fallback"
			})
		).toEqual({ label: "Back to A", href: "/a", isContextual: true });

		expect(
			computeResolvedBackInfo(
				{ label: "Back", href: "/home", isContextual: false },
				{ label: "Fallback", href: "/fallback" }
			)
		).toEqual({
			label: "Fallback",
			href: "/fallback",
			isContextual: false
		});

		expect(
			computeResolvedBackInfo({ label: "Back", href: "/home", isContextual: false })
		).toEqual({ label: "Back", href: "/home", isContextual: false });
	});
});
