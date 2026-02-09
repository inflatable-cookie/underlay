/**
 * Internationalization helpers for date, number, and currency formatting.
 *
 * Uses the built-in Intl API for locale-aware formatting. All formatters
 * default to the browser's locale but can be configured globally or per-call.
 *
 * @example
 * ```typescript
 * import { format } from '@decodelabs/underlay/patterns';
 *
 * format.date(new Date(), 'short');      // "12 Jan 2026"
 * format.relative(yesterday);            // "yesterday"
 * format.number(1234567);                // "1,234,567"
 * format.currency(1234.56, 'GBP');       // "£1,234.56"
 * format.fileSize(1536000);              // "1.5 MB"
 * ```
 *
 * @module
 */

import {
	mergeI18nConfig,
	resolveLocale,
	resolveTimezone,
	type I18nFormatConfig
} from "./i18n/intl-context";
export {
	formatNumber,
	formatPercent,
	formatFileSize,
	formatCurrency,
	plural,
	pluralCount,
	type PluralForms
} from "./i18n/number-format";
import {
	formatNumber,
	formatPercent,
	formatFileSize,
	formatCurrency,
	plural,
	pluralCount
} from "./i18n/number-format";

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

export interface FormatConfig extends I18nFormatConfig {}

/**
 * Configure global formatting defaults.
 *
 * @example
 * ```typescript
 * configureFormat({ locale: 'en-GB', timezone: 'Europe/London' });
 * ```
 */
export function configureFormat(config: FormatConfig): void {
	mergeI18nConfig(config);
}

// ---------------------------------------------------------------------------
// Date Formatting
// ---------------------------------------------------------------------------

export type DateStyle = "short" | "medium" | "long" | "full";

const dateStyleMap: Record<DateStyle, Intl.DateTimeFormatOptions> = {
	short: { day: "numeric", month: "short", year: "numeric" },
	medium: { day: "numeric", month: "long", year: "numeric" },
	long: { weekday: "long", day: "numeric", month: "long", year: "numeric" },
	full: { weekday: "long", day: "numeric", month: "long", year: "numeric", hour: "2-digit", minute: "2-digit" }
};

export type TimeStyle = "short" | "medium" | "long";

const timeStyleMap: Record<TimeStyle, Intl.DateTimeFormatOptions> = {
	short: { hour: "2-digit", minute: "2-digit" },
	medium: { hour: "2-digit", minute: "2-digit", second: "2-digit" },
	long: { hour: "2-digit", minute: "2-digit", second: "2-digit", timeZoneName: "short" }
};

/**
 * Format a date using the specified style.
 *
 * @param date - Date to format (Date object, ISO string, or timestamp)
 * @param style - Formatting style: 'short', 'medium', 'long', or 'full'
 * @param options - Optional overrides for locale/timezone
 * @returns Formatted date string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatDate(new Date(), 'short');   // "12 Jan 2026"
 * formatDate(new Date(), 'medium');  // "12 January 2026"
 * formatDate(new Date(), 'long');    // "Sunday, 12 January 2026"
 * formatDate(new Date(), 'full');    // "Sunday, 12 January 2026, 14:30"
 * ```
 */
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

/**
 * Format a time using the specified style.
 *
 * @param date - Date to format (Date object, ISO string, or timestamp)
 * @param style - Formatting style: 'short', 'medium', or 'long'
 * @param options - Optional overrides for locale/timezone
 * @returns Formatted time string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatTime(new Date(), 'short');   // "14:30"
 * formatTime(new Date(), 'medium');  // "14:30:45"
 * formatTime(new Date(), 'long');    // "14:30:45 GMT"
 * ```
 */
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

/**
 * Format a date/time as a relative string (e.g., "2 hours ago", "in 3 days").
 *
 * Uses Intl.RelativeTimeFormat for locale-aware relative time formatting.
 *
 * @param date - Date to format (Date object, ISO string, or timestamp)
 * @param options - Optional overrides for locale and reference date
 * @returns Relative time string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatRelative(yesterday);           // "yesterday"
 * formatRelative(twoHoursAgo);         // "2 hours ago"
 * formatRelative(inThreeDays);         // "in 3 days"
 * formatRelative(lastWeek);            // "last week"
 * ```
 */
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

	// Choose the most appropriate unit
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

/**
 * Format a date and time together.
 *
 * @param date - Date to format
 * @param dateStyle - Date style
 * @param timeStyle - Time style
 * @param options - Optional overrides
 * @returns Combined date and time string
 *
 * @example
 * ```typescript
 * formatDateTime(new Date(), 'short', 'short');  // "12 Jan 2026, 14:30"
 * ```
 */
export function formatDateTime(
	date: Date | string | number | null | undefined,
	dateStyle: DateStyle = "short",
	timeStyle: TimeStyle = "short",
	options?: { locale?: string; timezone?: string }
): string {
	const d = formatDate(date, dateStyle, options);
	const t = formatTime(date, timeStyle, options);
	if (!d) return "";
	if (!t) return d;
	return `${d}, ${t}`;
}

// ---------------------------------------------------------------------------
// Convenience Object Export
// ---------------------------------------------------------------------------

/**
 * Convenience object grouping all format functions.
 *
 * @example
 * ```typescript
 * import { format } from '@decodelabs/underlay/patterns';
 *
 * format.date(new Date(), 'short');
 * format.relative(yesterday);
 * format.number(1234567);
 * format.currency(1234.56, 'GBP');
 * format.fileSize(1536000);
 * format.plural(5, { one: 'item', other: 'items' });
 * ```
 */
export const format = {
	/** Configure global formatting defaults */
	configure: configureFormat,

	/** Format a date */
	date: formatDate,

	/** Format a time */
	time: formatTime,

	/** Format a date and time together */
	dateTime: formatDateTime,

	/** Format a relative time (e.g., "2 hours ago") */
	relative: formatRelative,

	/** Format a number with locale-aware separators */
	number: formatNumber,

	/** Format a number as a percentage */
	percent: formatPercent,

	/** Format bytes as a human-readable file size */
	fileSize: formatFileSize,

	/** Format a number as currency */
	currency: formatCurrency,

	/** Select the correct plural form */
	plural,

	/** Format a count with its pluralized label */
	pluralCount
} as const;
