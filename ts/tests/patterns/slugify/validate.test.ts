import { describe, expect, it } from "vitest";
import { slugify, validateSlug } from "../../../src/patterns/slugify";

describe("validateSlug", () => {
	describe("valid slugs", () => {
		it("returns valid: true for good slugs", () => {
			expect(validateSlug("hello-world")).toEqual({ valid: true });
			expect(validateSlug("chapter-1")).toEqual({ valid: true });
			expect(validateSlug("my-awesome-content")).toEqual({ valid: true });
		});
	});

	describe("too short", () => {
		it("returns too_short for single character", () => {
			expect(validateSlug("a")).toEqual({ valid: false, reason: "too_short" });
		});

		it("returns too_short for empty string", () => {
			expect(validateSlug("")).toEqual({ valid: false, reason: "too_short" });
		});
	});

	describe("too long", () => {
		it("returns too_long for > 100 characters", () => {
			const longSlug = "a".repeat(101);
			expect(validateSlug(longSlug)).toEqual({
				valid: false,
				reason: "too_long",
			});
		});

		it("accepts exactly 100 characters", () => {
			const maxSlug = "a".repeat(100);
			expect(validateSlug(maxSlug)).toEqual({ valid: true });
		});
	});

	describe("invalid format", () => {
		it("returns invalid_format for uppercase", () => {
			expect(validateSlug("Hello")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});

		it("returns invalid_format for spaces", () => {
			expect(validateSlug("hello world")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});

		it("returns invalid_format for leading hyphen", () => {
			expect(validateSlug("-hello")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});

		it("returns invalid_format for consecutive hyphens", () => {
			expect(validateSlug("hello--world")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});
	});

	describe("reserved slugs", () => {
		it("returns reserved for route-conflicting slugs", () => {
			expect(validateSlug("new")).toEqual({ valid: false, reason: "reserved" });
			expect(validateSlug("edit")).toEqual({
				valid: false,
				reason: "reserved",
			});
			expect(validateSlug("admin")).toEqual({
				valid: false,
				reason: "reserved",
			});
		});
	});

	describe("validation order", () => {
		it("checks length before format", () => {
			expect(validateSlug("A")).toEqual({ valid: false, reason: "too_short" });
		});

		it("checks format before reserved", () => {
			expect(validateSlug("NEW")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});
	});
});

describe("slugify and validateSlug integration", () => {
	it("slugify output passes validation (usually)", () => {
		const inputs = [
			"Hello World",
			"My Blog Post",
			"Chapter 10: The End",
			"FAQ & Help",
		];

		for (const input of inputs) {
			const slug = slugify(input);
			if (slug.length >= 2) {
				const result = validateSlug(slug);
				expect(result.valid).toBe(true);
			}
		}
	});

	it("slugify may produce reserved slugs", () => {
		const slug = slugify("New!");
		expect(slug).toBe("new");
		expect(validateSlug(slug)).toEqual({ valid: false, reason: "reserved" });
	});

	it("slugify may produce too-short slugs", () => {
		const slug = slugify("A");
		expect(slug).toBe("a");
		expect(validateSlug(slug)).toEqual({ valid: false, reason: "too_short" });
	});
});
