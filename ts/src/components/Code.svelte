<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import Copy from "lucide-svelte/icons/copy";
  import Check from "lucide-svelte/icons/check";
  import { copyTextToClipboard } from "../patterns/clipboard";

  interface Props extends Omit<HTMLAttributes<HTMLElement>, "class"> {
    class?: string;
    children?: Snippet;
    /** Show a one-click copy button */
    copy?: boolean;
  }

  let { class: className = "", children, copy = false, ...restProps }: Props = $props();
  let codeElement: HTMLElement | null = $state(null);
  let copied = $state(false);
  let resetTimeout: ReturnType<typeof setTimeout> | null = null;

  async function handleCopy(event: MouseEvent) {
    event.preventDefault();
    event.stopPropagation();
    const text = codeElement?.textContent?.trim() ?? "";
    if (!text) return;
    try {
      await copyTextToClipboard(text);
      copied = true;
      if (resetTimeout) clearTimeout(resetTimeout);
      resetTimeout = setTimeout(() => {
        copied = false;
        resetTimeout = null;
      }, 2000);
    } catch {
      // ignore
    }
  }
</script>

<span class="underlay-code-wrap">
  <code bind:this={codeElement} class="underlay-code {className}" {...restProps}>{@render children?.()}</code>
  {#if copy}
    <button type="button" class="underlay-code-copy" aria-label="Copy code" onclick={handleCopy}>
      {#if copied}
        <Check size={14} />
      {:else}
        <Copy size={14} />
      {/if}
    </button>
  {/if}
</span>

<style>
  .underlay-code-wrap {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
  }

  :global(.underlay-code) {
    font-family: var(--underlay-font-mono, ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace);
    font-size: 0.85em;
    padding: 0.15em 0.4em;
    background: var(--underlay-color-surface-inset, rgba(0, 0, 0, 0.2));
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
    color: var(--underlay-color-text-subtle, var(--underlay-color-text-muted, #6b7280));
    border-radius: var(--underlay-radius-sm, 0.25rem);
  }

  .underlay-code-copy {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    border-radius: 0.35rem;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #6b7280);
    cursor: pointer;
  }

  .underlay-code-copy:hover {
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-code-copy:focus-visible {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, rgba(59, 130, 246, 0.9));
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }
</style>
