import { resolveLocale, resolveTimezone } from "./intl-context";

export type DateStyle = "short" | "medium" | "long" | "full";

const dateStyleMap: Record<DateStyle, Intl.DateTimeFormatOptions> = {
	short: { day: "numeric", month: "short", year: "numeric" },
	medium: { day: "numeric", month: "long", year: "numeric" },
	long: { weekday: "long", day: "numeric", month: "long", year: "numeric" },
	full: {
		weekday: "long",
		day: "numeric",
		month: "long",
		year: "numeric",
		hour: "2-digit",
		minute: "2-digit"
	}
};

export type TimeStyle = "short" | "medium" | "long";

const timeStyleMap: Record<TimeStyle, Intl.DateTimeFormatOptions> = {
	short: { hour: "2-digit", minute: "2-digit" },
	medium: { hour: "2-digit", minute: "2-digit", second: "2-digit" },
	long: { hour: "2-digit", minute: "2-digit", second: "2-digit", timeZoneName: "short" }
};

export function formatDate(
	date: Date | string | number | null | undefined,
	style: DateStyle = "short",
	options?: { locale?: string; timezone?: string }
): string {
	if (date == null) return "";

	const d = date instanceof Date ? date : new Date(date);
	if (isNaN(d.getTime())) return "";

	const locale = resolveLocale(options?.locale);
	const timezone = resolveTimezone(options?.timezone);
	const formatOptions: Intl.DateTimeFormatOptions = {
		...dateStyleMap[style],
		...(timezone && { timeZone: timezone })
	};

	return new Intl.DateTimeFormat(locale, formatOptions).format(d);
}

export function formatTime(
	date: Date | string | number | null | undefined,
	style: TimeStyle = "short",
	options?: { locale?: string; timezone?: string }
): string {
	if (date == null) return "";

	const d = date instanceof Date ? date : new Date(date);
	if (isNaN(d.getTime())) return "";

	const locale = resolveLocale(options?.locale);
	const timezone = resolveTimezone(options?.timezone);
	const formatOptions: Intl.DateTimeFormatOptions = {
		...timeStyleMap[style],
		...(timezone && { timeZone: timezone })
	};

	return new Intl.DateTimeFormat(locale, formatOptions).format(d);
}

export function formatRelative(
	date: Date | string | number | null | undefined,
	options?: { locale?: string; now?: Date }
): string {
	if (date == null) return "";

	const d = date instanceof Date ? date : new Date(date);
	if (isNaN(d.getTime())) return "";

	const now = options?.now ?? new Date();
	const locale = resolveLocale(options?.locale);

	const diffMs = d.getTime() - now.getTime();
	const diffSecs = Math.round(diffMs / 1000);
	const diffMins = Math.round(diffSecs / 60);
	const diffHours = Math.round(diffMins / 60);
	const diffDays = Math.round(diffHours / 24);
	const diffWeeks = Math.round(diffDays / 7);
	const diffMonths = Math.round(diffDays / 30);
	const diffYears = Math.round(diffDays / 365);

	const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });

	if (Math.abs(diffSecs) < 60) {
		return rtf.format(diffSecs, "second");
	} else if (Math.abs(diffMins) < 60) {
		return rtf.format(diffMins, "minute");
	} else if (Math.abs(diffHours) < 24) {
		return rtf.format(diffHours, "hour");
	} else if (Math.abs(diffDays) < 7) {
		return rtf.format(diffDays, "day");
	} else if (Math.abs(diffWeeks) < 4) {
		return rtf.format(diffWeeks, "week");
	} else if (Math.abs(diffMonths) < 12) {
		return rtf.format(diffMonths, "month");
	} else {
		return rtf.format(diffYears, "year");
	}
}
