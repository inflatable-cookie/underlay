<script module lang="ts">
  export type DateRangeInput = string | Date | null | undefined;
  export type DateRangeStyle = "adaptive" | "full";

  export interface DateRangeFormatOptions {
    locale?: string;
    style?: DateRangeStyle;
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

  export function formatAdaptiveDateRange(
    startInput: DateRangeInput,
    endInput: DateRangeInput,
    options: DateRangeFormatOptions = {}
  ): string | null {
    const locale = options.locale ?? "en-GB";
    const style = options.style ?? "adaptive";

    const start = parseDate(startInput);
    const end = parseDate(endInput);
    if (!start || !end) return null;

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
</script>

<script lang="ts">
  interface Props {
    startDate?: DateRangeInput;
    endDate?: DateRangeInput;
    locale?: string;
    style?: DateRangeStyle;
    emptyText?: string;
    class?: string;
  }

  let {
    startDate = null,
    endDate = null,
    locale = "en-GB",
    style = "adaptive",
    emptyText = "—",
    class: className
  }: Props = $props();

  const label = $derived(formatAdaptiveDateRange(startDate, endDate, { locale, style }));
</script>

{#if label}
  <span class={`underlay-date-range ${className ?? ""}`}>{label}</span>
{:else}
  <span class={`underlay-date-range underlay-date-range--empty ${className ?? ""}`}>{emptyText}</span>
{/if}

<style>
  .underlay-date-range {
    color: inherit;
  }

  .underlay-date-range--empty {
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
