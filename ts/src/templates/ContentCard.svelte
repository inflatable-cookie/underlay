<script lang="ts">
  import { Card } from "@poodle/svelte";

  import { NightfireRenderer } from "../nightfire/renderer";
  import { isEmptyNightfire, normaliseNightfireValue } from "../nightfire/utils";

  interface Props {
    title?: string;
    value?: unknown;
    emptyMessage?: string;
    markdown?: boolean;
    maxHeight?: number;
    overflowBehavior?: "scroll" | "hidden" | "visible";
    schema?: string;
    class?: string;
  }

  let {
    title = "",
    value = null,
    emptyMessage = "No content.",
    markdown = false,
    maxHeight = 0,
    overflowBehavior = "scroll",
    schema = "underlay.content-card",
    class: className = ""
  }: Props = $props();

  const normalisedValue = $derived(
    normaliseNightfireValue(value, schema, markdown ? ["markdown"] : null)
  );
  const hasValue = $derived(!isEmptyNightfire(normalisedValue));
  const contentStyle = $derived(
    maxHeight && maxHeight > 0
      ? `max-height:${maxHeight}px;overflow:${overflowBehavior === "scroll" ? "auto" : overflowBehavior};`
      : undefined
  );
</script>

<div class={`underlay-content-card ${className}`.trim()}>
  <Card>
    {#if title}
      <header class="underlay-content-card__header">
        <h3>{title}</h3>
      </header>
    {/if}

    <div class="underlay-content-card__body" style={contentStyle}>
      {#if hasValue}
        <NightfireRenderer value={normalisedValue} />
      {:else}
        <p class="underlay-content-card__empty">{emptyMessage}</p>
      {/if}
    </div>
  </Card>
</div>

<style>
  .underlay-content-card {
    display: grid;
    gap: 0.75rem;
  }

  .underlay-content-card__header h3 {
    margin: 0;
    font-size: 0.9375rem;
    line-height: 1.35;
  }

  .underlay-content-card__body {
    min-width: 0;
  }

  .underlay-content-card__body :global(> :first-child),
  .underlay-content-card__body :global(> [data-nightfire-block]:first-child > :first-child),
  .underlay-content-card__body :global(> :first-child p:first-of-type) {
    margin-top: 0;
  }

  .underlay-content-card__body :global(> :last-child),
  .underlay-content-card__body :global(> [data-nightfire-block]:last-child > :last-child),
  .underlay-content-card__body :global(> :last-child p:last-of-type) {
    margin-bottom: 0;
  }

  .underlay-content-card__empty {
    margin: 0;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.75));
    font-size: 0.875rem;
    line-height: 1.5;
  }
</style>
