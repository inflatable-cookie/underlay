/**
 * Tests for slug utilities.
 */

import { describe, it, expect } from "vitest";
import {
	slugify,
	isValidSlugFormat,
	isReservedSlug,
	validateSlug,
	RESERVED_SLUGS,
} from "../../src/patterns/slugify";

// ============================================================================
// slugify
// ============================================================================

describe("slugify", () => {
	describe("basic conversion", () => {
		it("converts to lowercase", () => {
			expect(slugify("Hello World")).toBe("hello-world");
			expect(slugify("UPPERCASE")).toBe("uppercase");
		});

		it("replaces spaces with hyphens", () => {
			expect(slugify("hello world")).toBe("hello-world");
			expect(slugify("multiple   spaces")).toBe("multiple-spaces");
		});

		it("removes underscores", () => {
			// Note: underscores are removed by the non-alphanumeric filter
			// before the space-to-hyphen conversion can process them
			expect(slugify("hello_world")).toBe("helloworld");
			expect(slugify("snake_case_text")).toBe("snakecasetext");
		});

		it("trims whitespace", () => {
			expect(slugify("  hello  ")).toBe("hello");
			expect(slugify("  hello world  ")).toBe("hello-world");
		});

		it("removes leading and trailing hyphens", () => {
			expect(slugify("-hello-")).toBe("hello");
			expect(slugify("--hello--")).toBe("hello");
		});
	});

	describe("special characters", () => {
		it("removes punctuation", () => {
			expect(slugify("Hello, World!")).toBe("hello-world");
			expect(slugify("What's up?")).toBe("whats-up");
			expect(slugify("Test: 1.2.3")).toBe("test-123");
		});

		it("removes symbols", () => {
			expect(slugify("Price: $100")).toBe("price-100");
			expect(slugify("50% off")).toBe("50-off");
			expect(slugify("A & B")).toBe("a-b");
		});

		it("collapses multiple hyphens", () => {
			expect(slugify("hello---world")).toBe("hello-world");
			expect(slugify("a - b - c")).toBe("a-b-c");
		});
	});

	describe("unicode handling", () => {
		it("removes accents from characters", () => {
			expect(slugify("Über")).toBe("uber");
			expect(slugify("Café")).toBe("cafe");
			expect(slugify("naïve")).toBe("naive");
			expect(slugify("résumé")).toBe("resume");
		});

		it("handles various accented characters", () => {
			expect(slugify("àáâãäå")).toBe("aaaaaa");
			expect(slugify("èéêë")).toBe("eeee");
			expect(slugify("ñ")).toBe("n");
			expect(slugify("ç")).toBe("c");
		});

		it("removes non-Latin characters", () => {
			expect(slugify("hello 世界")).toBe("hello");
			expect(slugify("привет мир")).toBe("");
		});
	});

	describe("numbers", () => {
		it("preserves numbers", () => {
			expect(slugify("FA1 2024")).toBe("fa1-2024");
			expect(slugify("Chapter 10")).toBe("chapter-10");
			expect(slugify("123")).toBe("123");
		});

		it("handles mixed alphanumeric", () => {
			expect(slugify("v2.0.0")).toBe("v200");
			expect(slugify("test-123-abc")).toBe("test-123-abc");
		});
	});

	describe("edge cases", () => {
		it("handles empty string", () => {
			expect(slugify("")).toBe("");
		});

		it("handles whitespace only", () => {
			expect(slugify("   ")).toBe("");
		});

		it("handles special characters only", () => {
			expect(slugify("!@#$%")).toBe("");
		});

		it("handles single character", () => {
			expect(slugify("a")).toBe("a");
			expect(slugify("A")).toBe("a");
		});
	});
});

// ============================================================================
// isValidSlugFormat
// ============================================================================

describe("isValidSlugFormat", () => {
	describe("valid slugs", () => {
		it("accepts lowercase letters", () => {
			expect(isValidSlugFormat("hello")).toBe(true);
			expect(isValidSlugFormat("world")).toBe(true);
		});

		it("accepts numbers", () => {
			expect(isValidSlugFormat("123")).toBe(true);
			expect(isValidSlugFormat("abc123")).toBe(true);
			expect(isValidSlugFormat("123abc")).toBe(true);
		});

		it("accepts hyphens between segments", () => {
			expect(isValidSlugFormat("hello-world")).toBe(true);
			expect(isValidSlugFormat("a-b-c")).toBe(true);
			expect(isValidSlugFormat("test-123")).toBe(true);
		});

		it("accepts minimum length (2)", () => {
			expect(isValidSlugFormat("ab")).toBe(true);
			expect(isValidSlugFormat("12")).toBe(true);
		});
	});

	describe("invalid slugs", () => {
		it("rejects uppercase letters", () => {
			expect(isValidSlugFormat("Hello")).toBe(false);
			expect(isValidSlugFormat("WORLD")).toBe(false);
		});

		it("rejects spaces", () => {
			expect(isValidSlugFormat("hello world")).toBe(false);
		});

		it("rejects special characters", () => {
			expect(isValidSlugFormat("hello_world")).toBe(false);
			expect(isValidSlugFormat("hello.world")).toBe(false);
			expect(isValidSlugFormat("hello!")).toBe(false);
		});

		it("rejects leading hyphens", () => {
			expect(isValidSlugFormat("-hello")).toBe(false);
		});

		it("rejects trailing hyphens", () => {
			expect(isValidSlugFormat("hello-")).toBe(false);
		});

		it("rejects consecutive hyphens", () => {
			expect(isValidSlugFormat("hello--world")).toBe(false);
		});

		it("rejects too short (< 2 chars)", () => {
			expect(isValidSlugFormat("a")).toBe(false);
			expect(isValidSlugFormat("")).toBe(false);
		});

		it("rejects too long (> maxLength)", () => {
			const longSlug = "a".repeat(101);
			expect(isValidSlugFormat(longSlug)).toBe(false);
		});

		it("respects custom maxLength", () => {
			expect(isValidSlugFormat("abcdef", 5)).toBe(false);
			expect(isValidSlugFormat("abcde", 5)).toBe(true);
		});
	});
});

// ============================================================================
// isReservedSlug
// ============================================================================

describe("isReservedSlug", () => {
	it("identifies reserved route slugs", () => {
		expect(isReservedSlug("new")).toBe(true);
		expect(isReservedSlug("edit")).toBe(true);
		expect(isReservedSlug("delete")).toBe(true);
		expect(isReservedSlug("admin")).toBe(true);
		expect(isReservedSlug("api")).toBe(true);
		expect(isReservedSlug("login")).toBe(true);
	});

	it("returns false for non-reserved slugs", () => {
		expect(isReservedSlug("my-content")).toBe(false);
		expect(isReservedSlug("chapter-1")).toBe(false);
		expect(isReservedSlug("introduction")).toBe(false);
	});

	it("is case-sensitive (reserved list is lowercase)", () => {
		// The function checks exact match against lowercase reserved list
		expect(isReservedSlug("NEW")).toBe(false);
		expect(isReservedSlug("Admin")).toBe(false);
	});

	it("covers all documented reserved slugs", () => {
		// Verify all exported reserved slugs are detected
		for (const slug of RESERVED_SLUGS) {
			expect(isReservedSlug(slug)).toBe(true);
		}
	});
});

// ============================================================================
// validateSlug
// ============================================================================

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
			// Single char is too short, also invalid format if uppercase
			expect(validateSlug("A")).toEqual({ valid: false, reason: "too_short" });
		});

		it("checks format before reserved", () => {
			// "NEW" is reserved but also invalid format (uppercase)
			expect(validateSlug("NEW")).toEqual({
				valid: false,
				reason: "invalid_format",
			});
		});
	});
});

// ============================================================================
// Integration: slugify + validateSlug
// ============================================================================

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
		// "New!" slugifies to "new" which is reserved
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
