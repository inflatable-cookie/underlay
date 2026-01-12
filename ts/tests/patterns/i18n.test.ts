import { describe, it, expect, beforeEach } from "vitest";
import {
	format,
	formatDate,
	formatTime,
	formatRelative,
	formatDateTime,
	formatNumber,
	formatPercent,
	formatFileSize,
	formatCurrency,
	plural,
	pluralCount,
	configureFormat
} from "../../src/patterns/i18n";

describe("i18n formatting", () => {
	// Reset config before each test
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
	});

	describe("formatRelative", () => {
		it("formats seconds ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T14:29:30Z"); // 30 seconds ago
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("30 seconds ago");
		});

		it("formats minutes ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T14:25:00Z"); // 5 minutes ago
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("5 minutes ago");
		});

		it("formats hours ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-12T12:30:00Z"); // 2 hours ago
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("2 hours ago");
		});

		it("formats yesterday", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-11T14:30:00Z"); // yesterday
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("yesterday");
		});

		it("formats days ago", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-09T14:30:00Z"); // 3 days ago
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("3 days ago");
		});

		it("formats future dates", () => {
			const now = new Date("2026-01-12T14:30:00Z");
			const date = new Date("2026-01-14T14:30:00Z"); // in 2 days
			const result = formatRelative(date, { locale: "en-GB", now });
			expect(result).toBe("in 2 days");
		});

		it("handles null/undefined", () => {
			expect(formatRelative(null)).toBe("");
			expect(formatRelative(undefined)).toBe("");
		});
	});

	describe("formatNumber", () => {
		it("formats with thousands separators", () => {
			const result = formatNumber(1234567, { locale: "en-GB" });
			expect(result).toBe("1,234,567");
		});

		it("formats with decimal places", () => {
			const result = formatNumber(1234.567, { locale: "en-GB", decimals: 2 });
			expect(result).toBe("1,234.57");
		});

		it("formats with min/max decimals", () => {
			const result = formatNumber(1234.5, { locale: "en-GB", minDecimals: 2, maxDecimals: 4 });
			expect(result).toBe("1,234.50");
		});

		it("handles null/undefined", () => {
			expect(formatNumber(null)).toBe("");
			expect(formatNumber(undefined)).toBe("");
		});

		it("handles NaN", () => {
			expect(formatNumber(NaN)).toBe("");
		});
	});

	describe("formatPercent", () => {
		it("formats as percentage", () => {
			const result = formatPercent(0.856, { locale: "en-GB" });
			expect(result).toBe("86%");
		});

		it("formats with decimals", () => {
			const result = formatPercent(0.8567, { locale: "en-GB", decimals: 1 });
			expect(result).toBe("85.7%");
		});

		it("handles values over 1", () => {
			const result = formatPercent(1.5, { locale: "en-GB" });
			expect(result).toBe("150%");
		});

		it("handles null/undefined", () => {
			expect(formatPercent(null)).toBe("");
			expect(formatPercent(undefined)).toBe("");
		});
	});

	describe("formatFileSize", () => {
		it("formats bytes", () => {
			expect(formatFileSize(500, { locale: "en-GB" })).toBe("500 B");
		});

		it("formats kilobytes", () => {
			expect(formatFileSize(1024, { locale: "en-GB" })).toBe("1 KB");
		});

		it("formats megabytes", () => {
			expect(formatFileSize(1536000, { locale: "en-GB" })).toBe("1.5 MB");
		});

		it("formats gigabytes", () => {
			expect(formatFileSize(1073741824, { locale: "en-GB" })).toBe("1 GB");
		});

		it("handles zero", () => {
			expect(formatFileSize(0)).toBe("0 B");
		});

		it("handles null/undefined", () => {
			expect(formatFileSize(null)).toBe("");
			expect(formatFileSize(undefined)).toBe("");
		});

		it("respects decimals option", () => {
			expect(formatFileSize(1500000, { locale: "en-GB", decimals: 2 })).toBe("1.43 MB");
		});
	});

	describe("formatCurrency", () => {
		it("formats GBP", () => {
			const result = formatCurrency(1234.56, "GBP", { locale: "en-GB" });
			expect(result).toBe("£1,234.56");
		});

		it("formats USD", () => {
			const result = formatCurrency(1234.56, "USD", { locale: "en-US" });
			expect(result).toBe("$1,234.56");
		});

		it("formats EUR", () => {
			const result = formatCurrency(1234.56, "EUR", { locale: "en-GB" });
			expect(result).toContain("1,234.56");
			expect(result).toContain("€");
		});

		it("handles currencies without decimals", () => {
			const result = formatCurrency(1234, "JPY", { locale: "ja-JP" });
			expect(result).toContain("1,234");
		});

		it("handles null/undefined", () => {
			expect(formatCurrency(null, "GBP")).toBe("");
			expect(formatCurrency(undefined, "USD")).toBe("");
		});
	});

	describe("plural", () => {
		it("selects one form for 1", () => {
			const result = plural(1, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("item");
		});

		it("selects other form for 0", () => {
			const result = plural(0, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("items");
		});

		it("selects other form for plural", () => {
			const result = plural(5, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("items");
		});

		it("uses zero form when provided", () => {
			const result = plural(0, { zero: "no items", one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("no items");
		});
	});

	describe("pluralCount", () => {
		it("formats count with singular", () => {
			const result = pluralCount(1, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("1 item");
		});

		it("formats count with plural", () => {
			const result = pluralCount(5, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("5 items");
		});

		it("uses zero form without count prefix", () => {
			const result = pluralCount(0, { zero: "no items", one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("no items");
		});

		it("formats large counts with separators", () => {
			const result = pluralCount(1234, { one: "item", other: "items" }, { locale: "en-GB" });
			expect(result).toBe("1,234 items");
		});
	});

	describe("format object", () => {
		it("exposes all formatters", () => {
			expect(format.date).toBe(formatDate);
			expect(format.time).toBe(formatTime);
			expect(format.dateTime).toBe(formatDateTime);
			expect(format.relative).toBe(formatRelative);
			expect(format.number).toBe(formatNumber);
			expect(format.percent).toBe(formatPercent);
			expect(format.fileSize).toBe(formatFileSize);
			expect(format.currency).toBe(formatCurrency);
			expect(format.plural).toBe(plural);
			expect(format.pluralCount).toBe(pluralCount);
			expect(format.configure).toBe(configureFormat);
		});
	});

	describe("global configuration", () => {
		it("respects configured locale", () => {
			configureFormat({ locale: "de-DE" });
			const result = formatNumber(1234.56);
			// German uses comma for decimals and period for thousands
			expect(result).toContain("1.234");
		});
	});
});
