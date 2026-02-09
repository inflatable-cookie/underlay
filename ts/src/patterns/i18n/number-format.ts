import { resolveLocale } from "./intl-context";

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
	const showDecimals = unitIndex === 0 ? 0 : decimals;

	const formatted = new Intl.NumberFormat(locale, {
		minimumFractionDigits: 0,
		maximumFractionDigits: showDecimals
	}).format(value);

	return `${formatted} ${units[unitIndex]}`;
}

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

export interface PluralForms {
	zero?: string;
	one: string;
	two?: string;
	few?: string;
	many?: string;
	other: string;
}

export function plural(count: number, forms: PluralForms, options?: { locale?: string }): string {
	const locale = resolveLocale(options?.locale);
	const pr = new Intl.PluralRules(locale);
	const rule = pr.select(count);

	if (count === 0 && forms.zero !== undefined) {
		return forms.zero;
	}

	const form = forms[rule as keyof PluralForms];
	return form ?? forms.other;
}

export function pluralCount(count: number, forms: PluralForms, options?: { locale?: string }): string {
	const form = plural(count, forms, options);

	if (count === 0 && forms.zero !== undefined) {
		return form;
	}

	const formattedCount = formatNumber(count, { locale: options?.locale });
	return `${formattedCount} ${form}`;
}
