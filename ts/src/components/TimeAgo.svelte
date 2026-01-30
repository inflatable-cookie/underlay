<script lang="ts">
  import Tooltip from "./Tooltip.svelte";

  interface Props {
    /** The date to display as relative time. Accepts ISO string or Date object. */
    date: string | Date;
    /** Format for the tooltip. Defaults to full locale string. */
    tooltipFormat?: "full" | "date" | "datetime";
    /**
     * IANA timezone identifier for tooltip display (e.g., "Europe/London").
     * If not provided, uses the browser's local timezone.
     */
    timezone?: string;
    /** Custom class for the wrapper element */
    class?: string;
  }

  let {
    date,
    tooltipFormat = "datetime",
    timezone,
    class: className,
  }: Props = $props();

  /**
   * Calculate relative time string from a date.
   * Relative time is timezone-agnostic (always comparing UTC timestamps).
   */
  function getRelativeTime(date: Date): string {
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

      if (absDiffSeconds < 60) return "in a few seconds";
      if (absDiffMinutes < 60) return `in ${absDiffMinutes} minute${absDiffMinutes === 1 ? "" : "s"}`;
      if (absDiffHours < 24) return `in ${absDiffHours} hour${absDiffHours === 1 ? "" : "s"}`;
      return `in ${absDiffDays} day${absDiffDays === 1 ? "" : "s"}`;
    }

    // Past dates
    if (diffSeconds < 10) return "just now";
    if (diffSeconds < 60) return `${diffSeconds} seconds ago`;
    if (diffMinutes === 1) return "1 minute ago";
    if (diffMinutes < 60) return `${diffMinutes} minutes ago`;
    if (diffHours === 1) return "1 hour ago";
    if (diffHours < 24) return `${diffHours} hours ago`;
    if (diffDays === 1) return "yesterday";
    if (diffDays < 7) return `${diffDays} days ago`;
    if (diffWeeks === 1) return "1 week ago";
    if (diffWeeks < 4) return `${diffWeeks} weeks ago`;
    if (diffMonths === 1) return "1 month ago";
    if (diffMonths < 12) return `${diffMonths} months ago`;
    if (diffYears === 1) return "1 year ago";
    return `${diffYears} years ago`;
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

  const dateObj = $derived(typeof date === "string" ? new Date(date) : date);
  const relativeTime = $derived(getRelativeTime(dateObj));
  const tooltipText = $derived(formatTooltip(dateObj, tooltipFormat, timezone));
</script>

<Tooltip content={tooltipText} inline delayDuration={300} class={className}>
  {#snippet trigger()}
    <time class="time-ago" datetime={dateObj.toISOString()}>{relativeTime}</time>
  {/snippet}
</Tooltip>

<style>
  .time-ago {
    text-decoration: underline;
    text-decoration-style: dotted;
    text-underline-offset: 2px;
    text-decoration-color: var(--underlay-color-text-muted, #9ca3af);
  }

  .time-ago:hover {
    text-decoration-color: var(--underlay-color-text, #e5e7eb);
  }
</style>
