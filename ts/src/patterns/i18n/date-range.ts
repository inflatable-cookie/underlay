export type DateRangeInput = string | Date | null | undefined;
export type DateRangeStyle = "adaptive" | "full";

export interface DateRangeFormatOptions {
  locale?: string;
  style?: DateRangeStyle;
  hideDays?: boolean;
}

function parseDate(input: DateRangeInput): Date | null {
  if (!input) return null;
  const date = input instanceof Date ? input : new Date(input);
  return Number.isNaN(date.getTime()) ? null : date;
}

function getOrdinalSuffix(day: number): string {
  if (day >= 11 && day <= 13) return "th";
  switch (day % 10) {
    case 1:
      return "st";
    case 2:
      return "nd";
    case 3:
      return "rd";
    default:
      return "th";
  }
}

export function formatDateWithOrdinal(input: DateRangeInput, locale = "en-GB"): string | null {
  const date = parseDate(input);
  if (!date) return null;

  const day = date.getDate();
  const month = date.toLocaleDateString(locale, { month: "short" });
  const year = date.getFullYear();
  return `${day}${getOrdinalSuffix(day)} ${month} ${year}`;
}

function formatDayMonth(input: Date, locale: string): string {
  const day = input.getDate();
  const month = input.toLocaleDateString(locale, { month: "short" });
  return `${day}${getOrdinalSuffix(day)} ${month}`;
}

function formatMonthYear(date: Date, locale: string): string {
  return date.toLocaleDateString(locale, { month: "long", year: "numeric" });
}

export function formatAdaptiveDateRange(
  startInput: DateRangeInput,
  endInput: DateRangeInput,
  options: DateRangeFormatOptions = {}
): string | null {
  const locale = options.locale ?? "en-GB";
  const style = options.style ?? "adaptive";
  const hideDays = options.hideDays ?? false;

  const start = parseDate(startInput);
  const end = parseDate(endInput);
  if (!start || !end) return null;

  if (hideDays) {
    const startStr = formatMonthYear(start, locale);
    const endStr = formatMonthYear(end, locale);
    if (startStr === endStr) return startStr;
    if (start.getFullYear() === end.getFullYear()) {
      const startMonth = start.toLocaleDateString(locale, { month: "long" });
      const endMonth = end.toLocaleDateString(locale, { month: "long" });
      return `${startMonth} – ${endMonth} ${start.getFullYear()}`;
    }
    return `${startStr} – ${endStr}`;
  }

  if (style === "full") {
    const startFull = formatDateWithOrdinal(start, locale);
    const endFull = formatDateWithOrdinal(end, locale);
    return startFull && endFull ? `${startFull} to ${endFull}` : null;
  }

  const sameYear = start.getFullYear() === end.getFullYear();
  const sameMonth = sameYear && start.getMonth() === end.getMonth();

  if (sameMonth) {
    const startDay = start.getDate();
    const endDay = end.getDate();
    const month = start.toLocaleDateString(locale, { month: "short" });
    const year = start.getFullYear();
    return `${startDay}${getOrdinalSuffix(startDay)} to ${endDay}${getOrdinalSuffix(endDay)} ${month} ${year}`;
  }

  if (sameYear) {
    return `${formatDayMonth(start, locale)} to ${formatDayMonth(end, locale)} ${start.getFullYear()}`;
  }

  const startFull = formatDateWithOrdinal(start, locale);
  const endFull = formatDateWithOrdinal(end, locale);
  return startFull && endFull ? `${startFull} to ${endFull}` : null;
}
