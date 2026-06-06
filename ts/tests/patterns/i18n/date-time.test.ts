import { beforeEach, describe, expect, it, vi } from "vitest";
import {
	configureFormat,
	formatDate,
	formatDateTime,
	formatRelative,
	formatTime
} from "../../../src/patterns/i18n";

describe("i18n date and time formatting", () => {
	beforeEach(() => {
		configureFormat({ locale: "en-GB" });
	});

	describe("formatDate", () => {
		it("formats dates with short style", () => {
			const date = new Date("2026-01-12T14:30:00Z");
			const result = formatDate(date, "short", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("12 Jan 2026");
		});

		it("formats dates with medium style", () => {
			const date = new Date("2026-01-12T14:30:00Z");
			const result = formatDate(date, "medium", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("12 January 2026");
		});

		it("handles null/undefined", () => {
			expect(formatDate(null)).toBe("");
			expect(formatDate(undefined)).toBe("");
		});

		it("handles invalid dates", () => {
			expect(formatDate("not-a-date")).toBe("");
			expect(formatDate(NaN)).toBe("");
		});

		it("accepts ISO strings", () => {
			const result = formatDate("2026-01-12T14:30:00Z", "short", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("12 Jan 2026");
		});

		it("accepts timestamps", () => {
			const timestamp = new Date("2026-01-12T14:30:00Z").getTime();
			const result = formatDate(timestamp, "short", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("12 Jan 2026");
		});
	});

	describe("formatTime", () => {
		it("formats time with short style", () => {
			const date = new Date("2026-01-12T14:30:00Z");
			const result = formatTime(date, "short", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("14:30");
		});

		it("formats time with medium style", () => {
			const date = new Date("2026-01-12T14:30:45Z");
			const result = formatTime(date, "medium", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("14:30:45");
		});

		it("handles null/undefined", () => {
			expect(formatTime(null)).toBe("");
			expect(formatTime(undefined)).toBe("");
		});

		it("handles invalid date input", () => {
			expect(formatTime("not-a-date")).toBe("");
			expect(formatTime(NaN)).toBe("");
		});
	});

	describe("formatDateTime", () => {
		it("combines date and time", () => {
			const date = new Date("2026-01-12T14:30:00Z");
			const result = formatDateTime(date, "short", "short", { locale: "en-GB", timezone: "UTC" });
			expect(result).toBe("12 Jan 2026, 14:30");
		});

		it("handles null/undefined", () => {
			expect(formatDateTime(null)).toBe("");
		});

		it("returns date-only when time formatter returns empty", async () => {
			vi.resetModules();
			vi.doMock("../../../src/patterns/i18n/date-format", () => ({
				formatDate: () => "12 Jan 2026",
				formatTime: () => "",
				formatRelative: () => "now"
			}));

			const { formatDateTime: mockedFormatDateTime } = await import("../../../src/patterns/i18n");
			expect(mockedFormatDateTime(new Date("2026-01-12T14:30:00Z"))).toBe("12 Jan 2026");

			vi.doUnmock("../../../src/patterns/i18n/date-format");
			vi.resetModules();
		});
	});

	describe("formatRelative", () => {
		it("formats seconds ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T14:29:30Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("30 seconds ago");
		});

		it("formats minutes ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T14:25:00Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("5 minutes ago");
		});

		it("formats hours ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T12:30:00Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("2 hours ago");
		});

		it("formats yesterday", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-11T14:30:00Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("yesterday");
		});

		it("formats days ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-09T14:30:00Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("3 days ago");
		});

		it("formats future dates", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-14T14:30:00Z");
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("in 2 days");
		});

		it("formats weeks, months, and years ranges", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			expect(formatRelative(new Date("2025-12-29T14:30:00Z"), { locale: "en-GB", now })).toContain(
				"week"
			);
			expect(formatRelative(new Date("2025-11-13T14:30:00Z"), { locale: "en-GB", now })).toContain(
				"month"
			);
			expect(formatRelative(new Date("2024-01-13T14:30:00Z"), { locale: "en-GB", now })).toContain(
				"year"
			);
		});

		it("handles null/undefined", () => {
			expect(formatRelative(null)).toBe("");
			expect(formatRelative(undefined)).toBe("");
		});

		it("handles invalid date input and defaults now when omitted", () => {
			expect(formatRelative("not-a-date")).toBe("");
			expect(typeof formatRelative(new Date())).toBe("string");
		});
	});
});
