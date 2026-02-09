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
	type I18nFormatConfig
} from "./i18n/intl-context";
export {
	formatDate,
	formatTime,
	formatRelative,
	type DateStyle,
	type TimeStyle
} from "./i18n/date-format";
import {
	formatDate,
	formatTime,
	formatRelative,
	type DateStyle,
	type TimeStyle
} from "./i18n/date-format";
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
