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
// Number Formatting
// ---------------------------------------------------------------------------

/**
 * Format a number with locale-aware thousands separators.
 *
 * @param value - Number to format
 * @param options - Optional overrides for locale and decimal places
 * @returns Formatted number string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatNumber(1234567);                    // "1,234,567"
 * formatNumber(1234.567, { decimals: 2 });  // "1,234.57"
 * ```
 */
export function formatNumber(
	value: number | null | undefined,
	options?: { locale?: string; decimals?: number; minDecimals?: number; maxDecimals?: number }
): string {
	if (value == null || isNaN(value)) return "";

	const locale = resolveLocale(options?.locale);
	const formatOptions: Intl.NumberFormatOptions = {};

	if (options?.decimals !== undefined) {
		formatOptions.minimumFractionDigits = options.decimals;
		formatOptions.maximumFractionDigits = options.decimals;
	} else {
		if (options?.minDecimals !== undefined) {
			formatOptions.minimumFractionDigits = options.minDecimals;
		}
		if (options?.maxDecimals !== undefined) {
			formatOptions.maximumFractionDigits = options.maxDecimals;
		}
	}

	return new Intl.NumberFormat(locale, formatOptions).format(value);
}

/**
 * Format a number as a percentage.
 *
 * @param value - Number to format (0.5 = 50%)
 * @param options - Optional overrides for locale and decimal places
 * @returns Formatted percentage string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatPercent(0.856);                    // "86%"
 * formatPercent(0.856, { decimals: 1 });   // "85.6%"
 * formatPercent(1.5);                      // "150%"
 * ```
 */
export function formatPercent(
	value: number | null | undefined,
	options?: { locale?: string; decimals?: number }
): string {
	if (value == null || isNaN(value)) return "";

	const locale = resolveLocale(options?.locale);
	const formatOptions: Intl.NumberFormatOptions = {
		style: "percent",
		minimumFractionDigits: options?.decimals ?? 0,
		maximumFractionDigits: options?.decimals ?? 0
	};

	return new Intl.NumberFormat(locale, formatOptions).format(value);
}

/**
 * Format bytes as a human-readable file size.
 *
 * @param bytes - Number of bytes
 * @param options - Optional overrides for locale and decimal places
 * @returns Formatted file size string (e.g., "1.5 MB"), or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatFileSize(1024);          // "1 KB"
 * formatFileSize(1536000);       // "1.5 MB"
 * formatFileSize(1073741824);    // "1 GB"
 * ```
 */
export function formatFileSize(bytes: number | null | undefined, options?: { locale?: string; decimals?: number }): string {
	if (bytes == null || isNaN(bytes)) return "";
	if (bytes === 0) return "0 B";

	const locale = resolveLocale(options?.locale);
	const decimals = options?.decimals ?? 1;

	const units = ["B", "KB", "MB", "GB", "TB", "PB"];
	const k = 1024;
	const i = Math.floor(Math.log(Math.abs(bytes)) / Math.log(k));
	const unitIndex = Math.min(i, units.length - 1);

	const value = bytes / Math.pow(k, unitIndex);

	// Don't show decimals for bytes
	const showDecimals = unitIndex === 0 ? 0 : decimals;

	const formatted = new Intl.NumberFormat(locale, {
		minimumFractionDigits: 0,
		maximumFractionDigits: showDecimals
	}).format(value);

	return `${formatted} ${units[unitIndex]}`;
}

// ---------------------------------------------------------------------------
// Currency Formatting
// ---------------------------------------------------------------------------

/**
 * Format a number as currency.
 *
 * @param value - Amount to format
 * @param currency - ISO 4217 currency code (e.g., 'GBP', 'USD', 'EUR')
 * @param options - Optional overrides for locale
 * @returns Formatted currency string, or empty string for invalid input
 *
 * @example
 * ```typescript
 * formatCurrency(1234.56, 'GBP');  // "£1,234.56"
 * formatCurrency(1234.56, 'USD');  // "$1,234.56"
 * formatCurrency(1234.56, 'EUR');  // "€1,234.56"
 * formatCurrency(1234, 'JPY');     // "¥1,234"
 * ```
 */
export function formatCurrency(
	value: number | null | undefined,
	currency: string,
	options?: { locale?: string; hideSymbol?: boolean }
): string {
	if (value == null || isNaN(value)) return "";

	const locale = resolveLocale(options?.locale);

	const formatOptions: Intl.NumberFormatOptions = {
		style: "currency",
		currency: currency.toUpperCase()
	};

	if (options?.hideSymbol) {
		formatOptions.currencyDisplay = "code";
	}

	return new Intl.NumberFormat(locale, formatOptions).format(value);
}

// ---------------------------------------------------------------------------
// Pluralization
// ---------------------------------------------------------------------------

export interface PluralForms {
	/** Form for count === 0 (optional, falls back to 'other') */
	zero?: string;
	/** Form for count === 1 */
	one: string;
	/** Form for count === 2 (optional, used in some languages) */
	two?: string;
	/** Form for "a few" (optional, used in some languages) */
	few?: string;
	/** Form for "many" (optional, used in some languages) */
	many?: string;
	/** Default form for other counts */
	other: string;
}

/**
 * Select the correct plural form based on count.
 *
 * Uses Intl.PluralRules for locale-aware pluralization, supporting
 * languages with complex plural rules (e.g., Arabic, Russian).
 *
 * @param count - The count to pluralize
 * @param forms - Object with plural forms
 * @param options - Optional locale override
 * @returns The selected plural form
 *
 * @example
 * ```typescript
 * plural(0, { one: 'item', other: 'items' });   // "items"
 * plural(1, { one: 'item', other: 'items' });   // "item"
 * plural(5, { one: 'item', other: 'items' });   // "items"
 *
 * // With zero form
 * plural(0, { zero: 'no items', one: 'item', other: 'items' });  // "no items"
 * ```
 */
export function plural(count: number, forms: PluralForms, options?: { locale?: string }): string {
	const locale = resolveLocale(options?.locale);
	const pr = new Intl.PluralRules(locale);
	const rule = pr.select(count);

	// Check for explicit zero form first
	if (count === 0 && forms.zero !== undefined) {
		return forms.zero;
	}

	// Use Intl.PluralRules result
	const form = forms[rule as keyof PluralForms];
	return form ?? forms.other;
}

/**
 * Format a count with its pluralized label.
 *
 * @param count - The count
 * @param forms - Object with plural forms
 * @param options - Optional locale override
 * @returns Formatted string like "5 items"
 *
 * @example
 * ```typescript
 * pluralCount(0, { one: 'item', other: 'items' });   // "0 items"
 * pluralCount(1, { one: 'item', other: 'items' });   // "1 item"
 * pluralCount(5, { one: 'item', other: 'items' });   // "5 items"
 *
 * // With zero form
 * pluralCount(0, { zero: 'no items', one: 'item', other: 'items' });  // "no items"
 * ```
 */
export function pluralCount(count: number, forms: PluralForms, options?: { locale?: string }): string {
	const form = plural(count, forms, options);

	// If using zero form that includes count info, don't prefix with number
	if (count === 0 && forms.zero !== undefined) {
		return form;
	}

	const formattedCount = formatNumber(count, { locale: options?.locale });
	return `${formattedCount} ${form}`;
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
