<script lang="ts">
  import { onDestroy } from "svelte";
  import { IconButton, TextLink } from "@poodle/svelte";

  import { copyTextToClipboard, copyToClipboard, useToasts } from "../runtime/feedback";
  import type { ToastStore } from "../patterns/toasts";

  interface Props {
    href?: string | null;
    name: string;
    entityId: string;
    secondary?: string | null;
    linkAriaLabel?: string | null;
    target?: string | null;
    rel?: string | null;
    copyLabel?: string;
    copySuccessMessage?: string;
    copyFailureMessage?: string;
    class?: string;
  }

  let {
    href = null,
    name,
    entityId,
    secondary = null,
    linkAriaLabel = null,
    target = null,
    rel = null,
    copyLabel = "Copy ID",
    copySuccessMessage = "Copied ID",
    copyFailureMessage = "Failed to copy ID",
    class: className = ""
  }: Props = $props();

  const toastStore = useToasts() as ToastStore | undefined;
  let copied = $state(false);
  let copyTimer: ReturnType<typeof setTimeout> | null = null;

  const displayName = $derived(name.trim() || "—");
  const canCopy = $derived(entityId.trim().length > 0);

  onDestroy(() => {
    if (copyTimer) {
      clearTimeout(copyTimer);
    }
  });

  function markCopied(): void {
    copied = true;

    if (copyTimer) {
      clearTimeout(copyTimer);
    }

    copyTimer = setTimeout(() => {
      copied = false;
      copyTimer = null;
    }, 1500);
  }

  async function handleCopy(event: MouseEvent): Promise<void> {
    event.preventDefault();
    event.stopPropagation();

    if (!canCopy) {
      return;
    }

    try {
      if (toastStore) {
        await copyToClipboard(toastStore, entityId, copySuccessMessage, copyFailureMessage);
      } else {
        await copyTextToClipboard(entityId);
      }

      markCopied();
    } catch {
      // copyToClipboard already emits the failure toast when a toast store exists.
    }
  }
</script>

<span class={`underlay-relation-link ${className}`.trim()}>
  <span class="underlay-relation-link__line">
    {#if href}
      <TextLink
        {href}
        {target}
        {rel}
        ariaLabel={linkAriaLabel}
        className="underlay-relation-link__anchor"
      >
        {displayName}
      </TextLink>
    {:else}
      <span class="underlay-relation-link__name">{displayName}</span>
    {/if}

    {#if canCopy}
      <IconButton
        icon={copied ? "check" : "copy"}
        ariaLabel={copied ? "Copied ID" : copyLabel}
        tooltip={copied ? "Copied ID" : copyLabel}
        size="xs"
        sizeRole="chrome"
        onClick={handleCopy}
      />
    {/if}
  </span>

  {#if secondary?.trim()}
    <span class="underlay-relation-link__secondary">{secondary}</span>
  {/if}
</span>

<style>
  .underlay-relation-link {
    display: inline-grid;
    gap: 0.125rem;
    max-width: 100%;
    min-width: 0;
    vertical-align: top;
  }

  .underlay-relation-link__line {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    max-width: 100%;
    min-width: 0;
  }

  .underlay-relation-link__anchor,
  .underlay-relation-link__name {
    min-width: 0;
    overflow-wrap: anywhere;
  }

  .underlay-relation-link__secondary {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.75));
    font-size: 0.875em;
    line-height: 1.35;
    overflow-wrap: anywhere;
  }
</style>
