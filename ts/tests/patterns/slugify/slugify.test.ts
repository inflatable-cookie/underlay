import { describe, expect, it } from "vitest";
import { slugify } from "../../../src/patterns/slugify";

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
