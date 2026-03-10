<script lang="ts">
  import type { Snippet } from "svelte";
  import Button from "../components/Button.svelte";

  interface Props {
    title: string;
    description: string;
    onClose?: (() => void) | null;
    closeLabel?: string;
    meta?: Snippet;
    controls?: Snippet;
    message?: string | null;
    body?: Snippet;
    actions?: Snippet;
  }

  let {
    title,
    description,
    onClose = null,
    closeLabel = "Close",
    meta = undefined,
    controls = undefined,
    message = null,
    body = undefined,
    actions = undefined,
  }: Props = $props();
</script>

<section class="underlay-restore-resolution-shell" aria-label={title}>
  <div class="underlay-restore-resolution-shell__header">
    <div>
      <h2 class="underlay-restore-resolution-shell__title">{title}</h2>
      <p class="underlay-restore-resolution-shell__description">{description}</p>
    </div>
    {#if onClose}
      <Button variant="subtle" onclick={onClose}>{closeLabel}</Button>
    {/if}
  </div>

  {#if meta}
    <div class="underlay-restore-resolution-shell__meta">
      {@render meta()}
    </div>
  {/if}

  {#if controls}
    <div class="underlay-restore-resolution-shell__controls">
      {@render controls()}
    </div>
  {/if}

  {#if message}
    <p class="underlay-restore-resolution-shell__message">{message}</p>
  {/if}

  {#if body}
    <div class="underlay-restore-resolution-shell__body">
      {@render body()}
    </div>
  {/if}

  {#if actions}
    <div class="underlay-restore-resolution-shell__actions">
      {@render actions()}
    </div>
  {/if}
</section>

<style>
  .underlay-restore-resolution-shell {
    display: grid;
    gap: 1rem;
  }

  .underlay-restore-resolution-shell__header {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    align-items: start;
  }

  .underlay-restore-resolution-shell__title,
  .underlay-restore-resolution-shell__description,
  .underlay-restore-resolution-shell__message {
    margin: 0;
  }

  .underlay-restore-resolution-shell__title {
    font-size: 1rem;
    font-weight: 700;
    color: var(--underlay-color-text-primary, #f8fafc);
  }

  .underlay-restore-resolution-shell__description,
  .underlay-restore-resolution-shell__meta,
  .underlay-restore-resolution-shell__message {
    color: var(--underlay-color-text-secondary, #cbd5e1);
  }

  .underlay-restore-resolution-shell__meta,
  .underlay-restore-resolution-shell__controls,
  .underlay-restore-resolution-shell__body {
    display: grid;
    gap: 0.5rem;
  }

  .underlay-restore-resolution-shell__actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }
</style>
