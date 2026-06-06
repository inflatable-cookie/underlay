import { beforeEach, describe, expect, it } from "vitest";
import {
	configureFormat,
	formatCurrency,
	formatFileSize,
	formatNumber,
	formatPercent
} from "../../../src/patterns/i18n";

describe("i18n numeric formatting", () => {
	beforeEach(() => {
		configureFormat({ locale: "en-GB" });
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

		it("uses code display when hideSymbol is enabled", () => {
			const result = formatCurrency(1234.56, "usd", {
				locale: "en-US",
				hideSymbol: true,
			});
			expect(result).toContain("USD");
		});

		it("handles null/undefined", () => {
			expect(formatCurrency(null, "GBP")).toBe("");
			expect(formatCurrency(undefined, "USD")).toBe("");
		});
	});
});
