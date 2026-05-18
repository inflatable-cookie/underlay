<script lang="ts">
  import { Code, Dialog, IconButton } from "@poodle/svelte";

  interface Props {
    value?: unknown | null;
    title?: string;
    triggerLabel?: string;
    maxHeight?: string;
    showCloseButton?: boolean;
    closeLabel?: string;
  }

  let {
    value = null,
    title = "Metadata",
    triggerLabel = "Metadata",
    maxHeight = "min(60vh, 32rem)",
    showCloseButton = true,
    closeLabel = "Close metadata dialog"
  }: Props = $props();

  let open = $state(false);
  const source = $derived(normalizeValue(value));

  function normalizeValue(input: unknown): string | null {
    if (input === null || input === undefined) {
      return null;
    }

    if (typeof input === "string") {
      const trimmed = input.trim();
      return trimmed.length > 0 ? trimmed : null;
    }

    if (Array.isArray(input)) {
      if (input.length === 0) return null;
    } else if (typeof input === "object") {
      if (Object.keys(input as Record<string, unknown>).length === 0) {
        return null;
      }
    }

    try {
      return JSON.stringify(input, null, 2);
    } catch {
      return String(input);
    }
  }
</script>

{#if source}
  <span class="underlay-metadata-dialog-trigger">
    <IconButton
      type="button"
      icon="code"
      variant="ghost"
      size="xs"
      ariaLabel={triggerLabel}
      tooltip={triggerLabel}
      onClick={() => {
        open = true;
      }}
    />
  </span>

  <Dialog
    open={open}
    {title}
    width="lg"
    {showCloseButton}
    {closeLabel}
    onOpenChange={(nextOpen) => {
      open = nextOpen;
    }}
  >
    <Code source={source} language="json" maxHeight={maxHeight} />
  </Dialog>
{/if}

<style>
  .underlay-metadata-dialog-trigger {
    display: inline-flex;
    align-items: center;
  }

  .underlay-metadata-dialog-trigger :global(.poodle-icon-button) {
    --poodle-icon-button-text: var(--poodle-color-text-secondary);
    --poodle-icon-button-fill-hover: color-mix(
      in srgb,
      var(--poodle-color-background-panel) 70%,
      var(--poodle-color-text-primary) 8%
    );
    --poodle-icon-button-fill-active: color-mix(
      in srgb,
      var(--poodle-color-background-panel) 58%,
      var(--poodle-color-text-primary) 12%
    );
  }

  .underlay-metadata-dialog-trigger :global(.poodle-icon-button:hover) {
    --poodle-icon-button-text: var(--poodle-color-text-primary);
  }
</style>
