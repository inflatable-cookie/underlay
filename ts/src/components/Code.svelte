<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLAttributes<HTMLElement>, "class"> {
    class?: string;
    children?: Snippet;
  }

  let { class: className = "", children, ...restProps }: Props = $props();
</script>

<code class="underlay-code {className}" {...restProps}>{@render children?.()}</code>

<style>
  :global(.underlay-code) {
    font-family: var(--underlay-font-mono, ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace);
    font-size: 0.85em;
    padding: 0.15em 0.4em;
    background: var(--underlay-color-surface-inset, rgba(0, 0, 0, 0.2));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    border-radius: var(--underlay-radius-sm, 0.25rem);
  }
</style>
