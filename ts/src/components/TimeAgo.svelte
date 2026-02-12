<script lang="ts">
  import Tooltip from "./Tooltip.svelte";

  interface Props {
    /** The date to display as relative time. Accepts ISO string or Date object. */
    date?: string | Date | null;
    /** Format for the tooltip. Defaults to full locale string. */
    tooltipFormat?: "full" | "date" | "datetime";
    /**
     * IANA timezone identifier for tooltip display (e.g., "Europe/London").
     * If not provided, uses the browser's local timezone.
     */
    timezone?: string;
    /** Custom class for the wrapper element */
    class?: string;
    /** Use abbreviated output (e.g., "5 min ago") */
    short?: boolean;
  }

  let {
    date = null,
    tooltipFormat = "datetime",
    timezone,
    class: className,
    short = false,
  }: Props = $props();

  /**
   * Calculate relative time string from a date.
   * Relative time is timezone-agnostic (always comparing UTC timestamps).
   */
  function getRelativeTime(date: Date, shortFormat: boolean): string {
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffSeconds = Math.floor(diffMs / 1000);
    const diffMinutes = Math.floor(diffSeconds / 60);
    const diffHours = Math.floor(diffMinutes / 60);
    const diffDays = Math.floor(diffHours / 24);
    const diffWeeks = Math.floor(diffDays / 7);
    const diffMonths = Math.floor(diffDays / 30);
    const diffYears = Math.floor(diffDays / 365);

    // Future dates
    if (diffMs < 0) {
      const absDiffMs = Math.abs(diffMs);
      const absDiffSeconds = Math.floor(absDiffMs / 1000);
      const absDiffMinutes = Math.floor(absDiffSeconds / 60);
      const absDiffHours = Math.floor(absDiffMinutes / 60);
      const absDiffDays = Math.floor(absDiffHours / 24);

      if (absDiffSeconds < 60) return shortFormat ? "soon" : "in a few seconds";
      if (absDiffMinutes < 60)
        return shortFormat
          ? `in ${absDiffMinutes} min`
          : `in ${absDiffMinutes} minute${absDiffMinutes === 1 ? "" : "s"}`;
      if (absDiffHours < 24)
        return shortFormat
          ? `in ${absDiffHours} h`
          : `in ${absDiffHours} hour${absDiffHours === 1 ? "" : "s"}`;
      return shortFormat
        ? `in ${absDiffDays} d`
        : `in ${absDiffDays} day${absDiffDays === 1 ? "" : "s"}`;
    }

    // Past dates
    if (diffSeconds < 10) return shortFormat ? "now" : "just now";
    if (diffSeconds < 60) return shortFormat ? `${diffSeconds}s ago` : `${diffSeconds} seconds ago`;
    if (diffMinutes === 1) return shortFormat ? "1 min ago" : "1 minute ago";
    if (diffMinutes < 60) return shortFormat ? `${diffMinutes} min ago` : `${diffMinutes} minutes ago`;
    if (diffHours === 1) return shortFormat ? "1 h ago" : "1 hour ago";
    if (diffHours < 24) return shortFormat ? `${diffHours} h ago` : `${diffHours} hours ago`;
    if (diffDays === 1) return shortFormat ? "1 d ago" : "yesterday";
    if (diffDays < 7) return shortFormat ? `${diffDays} d ago` : `${diffDays} days ago`;
    if (diffWeeks === 1) return shortFormat ? "1 wk ago" : "1 week ago";
    if (diffWeeks < 4) return shortFormat ? `${diffWeeks} wk ago` : `${diffWeeks} weeks ago`;
    if (diffMonths === 1) return shortFormat ? "1 mo ago" : "1 month ago";
    if (diffMonths < 12) return shortFormat ? `${diffMonths} mo ago` : `${diffMonths} months ago`;
    if (diffYears === 1) return shortFormat ? "1 yr ago" : "1 year ago";
    return shortFormat ? `${diffYears} yr ago` : `${diffYears} years ago`;
  }

  /**
   * Format date for tooltip display in the specified timezone.
   */
  function formatTooltip(date: Date, format: "full" | "date" | "datetime", tz?: string): string {
    const tzOption = tz ? { timeZone: tz } : {};

    try {
      switch (format) {
        case "date":
          return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            ...tzOption,
          });
        case "datetime":
          return date.toLocaleString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            ...tzOption,
          });
        case "full":
        default:
          return date.toLocaleString(undefined, {
            year: "numeric",
            month: "long",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
            timeZoneName: "short",
            ...tzOption,
          });
      }
    } catch {
      // Fallback if timezone is invalid
      return formatTooltip(date, format, undefined);
    }
  }

  function isValidDateValue(value: Date | null): value is Date {
    return value instanceof Date && !Number.isNaN(value.getTime());
  }

  const dateObj = $derived(
    typeof date === "string" ? new Date(date) : date instanceof Date ? date : null
  );
  const isValidDate = $derived(isValidDateValue(dateObj));
  const safeDate = $derived(isValidDate ? (dateObj as Date) : null);
  const relativeTime = $derived(
    safeDate ? getRelativeTime(safeDate, short) : ""
  );
  const tooltipText = $derived(
    safeDate ? formatTooltip(safeDate, tooltipFormat, timezone) : ""
  );
</script>

{#if isValidDate}
  <Tooltip content={tooltipText} inline delayDuration={300} class={className}>
    {#snippet trigger()}
      <time class="underlay-time-ago" datetime={safeDate?.toISOString() ?? ""}>{relativeTime}</time>
    {/snippet}
  </Tooltip>
{:else}
  <span class="underlay-time-ago underlay-time-ago--empty {className ?? ''}">—</span>
{/if}

<style>
  .underlay-time-ago {
    text-decoration: underline;
    text-decoration-style: dotted;
    text-underline-offset: 2px;
    text-decoration-color: var(--underlay-color-text-muted, #9ca3af);
  }

  .underlay-time-ago:hover {
    text-decoration-color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-time-ago--empty {
    text-decoration: none;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
