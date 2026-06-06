import { beforeEach, describe, expect, it } from "vitest";
import {
	configureFormat,
	format,
	formatCurrency,
	formatDate,
	formatDateTime,
	formatFileSize,
	formatNumber,
	formatPercent,
	formatRelative,
	formatTime,
	plural,
	pluralCount
} from "../../../src/patterns/i18n";

describe("i18n pluralization and facade", () => {
	beforeEach(() => {
		configureFormat({ locale: "en-GB" });
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

		it("falls back to other when locale rule-specific form is missing", () => {
			const result = plural(2, { one: "item", other: "items" }, { locale: "ru" });
			expect(result).toBe("items");
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
});
