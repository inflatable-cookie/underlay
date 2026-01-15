<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    /** Optional accent color - when provided, pill uses color-mix for bg/border/text */
    accent?: string;
    /** Additional CSS classes */
    className?: string;
    children: Snippet;
  }

  let { accent, className, children }: Props = $props();
</script>

<span
  class="underlay-pill {className ?? ''}"
  class:underlay-pill--accent={accent}
  style:--pill-accent={accent}
>
  {@render children()}
</span>

<style>
  .underlay-pill {
    display: inline-flex;
    align-self: flex-start;
    width: fit-content;
    font-size: 0.625em;
    font-weight: 650;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    border-radius: 999px;
    padding: 0.25em 0.7em;
    white-space: nowrap;

    /* Neutral (default) colors */
    color: rgba(148, 163, 184, 0.9);
    background: rgba(148, 163, 184, 0.14);
    border: 1px solid rgba(148, 163, 184, 0.22);
  }

  .underlay-pill--accent {
    background: color-mix(in srgb, var(--pill-accent) 18%, rgba(148, 163, 184, 0.08));
    border: 1px solid color-mix(in srgb, var(--pill-accent) 30%, rgba(148, 163, 184, 0.12));
    color: color-mix(in srgb, var(--pill-accent) 88%, white);
  }
</style>
