import { describe, expect, it, vi } from "vitest";

describe("patterns/timezone.svelte.ts", () => {
	it("handles init flows, conflict resolution, and manual updates", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const mod = await import("../../src/patterns/timezone.svelte");
		const {
			timezoneStore,
			resetTimezone,
			initTimezone,
			resolveTimezoneConflict,
			setEffectiveTimezone,
		} = mod;

		resetTimezone();
		expect(timezoneStore.initialized).toBe(false);

		const onAutoFill = vi.fn();
		initTimezone({ profileTimezone: null, onAutoFill });
		expect(timezoneStore.initialized).toBe(true);
		expect(timezoneStore.hasConflict).toBe(false);
		expect(onAutoFill).toHaveBeenCalledTimes(1);

		initTimezone({ profileTimezone: null });
		expect(timezoneStore.initialized).toBe(true);

		const onConflict = vi.fn();
		initTimezone({ profileTimezone: "Europe/London", onConflict });
		expect(timezoneStore.hasConflict).toBe(true);
		expect(timezoneStore.effective).toBe("Europe/London");
		expect(onConflict).toHaveBeenCalledTimes(1);

		initTimezone({ profileTimezone: timezoneStore.browser });
		expect(timezoneStore.hasConflict).toBe(false);
		expect(timezoneStore.effective).toBe(timezoneStore.browser);

		resolveTimezoneConflict("browser");
		expect(timezoneStore.hasConflict).toBe(false);
		expect(timezoneStore.effective).toBe(timezoneStore.browser);

		initTimezone({ profileTimezone: "America/New_York" });
		resolveTimezoneConflict("profile");
		expect(timezoneStore.effective).toBe("America/New_York");

		setEffectiveTimezone("Pacific/Auckland");
		expect(timezoneStore.effective).toBe("Pacific/Auckland");
		expect(timezoneStore.profile).toBe("Pacific/Auckland");
		expect(timezoneStore.hasConflict).toBe(false);
	});

	it("formats dates and handles invalid inputs/timezones", async () => {
		vi.resetModules();
		(globalThis as any).$state = <T>(initial: T) => initial;

		const { formatInTimezone, formatDate, setEffectiveTimezone } = await import(
			"../../src/patterns/timezone.svelte"
		);

		expect(formatInTimezone("not-a-date", "UTC")).toBe("Invalid date");
		expect(formatInTimezone(new Date("2026-01-30T14:30:00Z"), "UTC")).toEqual(
			expect.any(String)
		);
		expect(formatInTimezone("2026-01-30T14:30:00Z", "Invalid/Timezone")).toEqual(
			expect.any(String)
		);

		setEffectiveTimezone("UTC");
		expect(formatDate("2026-01-30T14:30:00Z")).toEqual(expect.any(String));
	});

	it("detects browser timezone when browser environment is available", async () => {
		vi.resetModules();
		vi.doMock("esm-env", () => ({ BROWSER: true }));
		(globalThis as any).$state = <T>(initial: T) => initial;

		const originalDateTimeFormat = Intl.DateTimeFormat;
		(Intl as any).DateTimeFormat = () => ({
			resolvedOptions: () => ({ timeZone: "Europe/Paris" }),
		});

		const { detectBrowserTimezone } = await import("../../src/patterns/timezone.svelte");
		expect(detectBrowserTimezone()).toBe("Europe/Paris");

		(Intl as any).DateTimeFormat = () => {
			throw new Error("Intl unavailable");
		};
		expect(detectBrowserTimezone()).toBe("UTC");

		(Intl as any).DateTimeFormat = originalDateTimeFormat;
		vi.doUnmock("esm-env");
		vi.resetModules();
	});
});
