import { beforeEach, describe, expect, it } from "vitest";
import { configureFormat, formatNumber } from "../../../src/patterns/i18n";
import { resolveLocale, resolveTimezone } from "../../../src/patterns/i18n/intl-context";

describe("i18n global configuration", () => {
	beforeEach(() => {
		configureFormat({ locale: "en-GB" });
	});

	it("respects configured locale", () => {
		configureFormat({ locale: "de-DE" });
		const result = formatNumber(1234.56);
		expect(result).toContain("1.234");
	});

	it("resolves locale and timezone from config and per-call override", () => {
		configureFormat({ locale: "fr-FR", timezone: "Europe/Paris" });
		expect(resolveLocale()).toBe("fr-FR");
		expect(resolveLocale("en-US")).toBe("en-US");
		expect(resolveTimezone()).toBe("Europe/Paris");
		expect(resolveTimezone("UTC")).toBe("UTC");
	});

	it("falls back to navigator locale when no config is set", () => {
		const originalNavigator = (globalThis as { navigator?: Navigator }).navigator;
		Object.defineProperty(globalThis, "navigator", {
			value: { language: "it-IT" } as Navigator,
			writable: true,
			configurable: true
		});
		configureFormat({ locale: undefined, timezone: undefined });
		expect(resolveLocale()).toBe("it-IT");
		Object.defineProperty(globalThis, "navigator", {
			value: originalNavigator,
			writable: true,
			configurable: true
		});
	});

	it("falls back to en-GB when neither config nor navigator is available", () => {
		const originalNavigator = (globalThis as { navigator?: Navigator }).navigator;
		Object.defineProperty(globalThis, "navigator", {
			value: undefined,
			writable: true,
			configurable: true
		});
		configureFormat({ locale: undefined, timezone: undefined });
		expect(resolveLocale()).toBe("en-GB");
		Object.defineProperty(globalThis, "navigator", {
			value: originalNavigator,
			writable: true,
			configurable: true
		});
	});
});
