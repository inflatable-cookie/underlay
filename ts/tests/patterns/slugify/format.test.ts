import { describe, expect, it } from "vitest";
import {
	isReservedSlug,
	isValidSlugFormat,
	RESERVED_SLUGS,
} from "../../../src/patterns/slugify";

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
		expect(isReservedSlug("NEW")).toBe(false);
		expect(isReservedSlug("Admin")).toBe(false);
	});

	it("covers all documented reserved slugs", () => {
		for (const slug of RESERVED_SLUGS) {
			expect(isReservedSlug(slug)).toBe(true);
		}
	});
});
