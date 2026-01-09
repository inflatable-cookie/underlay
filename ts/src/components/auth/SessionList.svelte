<script lang="ts">
  import type { SessionListItem } from "./types";

  import Card from "../Card.svelte";
  import ConfirmAction from "../ConfirmAction.svelte";

  export let sessions: SessionListItem[] = [];
  export let currentSessionId: string | null = null;

  export let title: string = "Sessions";
  export let emptyMessage: string = "No active sessions.";

  export let onRevoke: ((sessionId: string) => void | Promise<void>) | null = null;

  function formatDate(iso: string) {
    const d = new Date(iso);
    if (Number.isNaN(d.valueOf())) return iso;
    return d.toLocaleString();
  }
</script>

<Card class="underlay-session-list">
  <header class="underlay-session-list__header">
    <h3 class="underlay-session-list__title">{title}</h3>
  </header>

  {#if sessions.length === 0}
    <p class="underlay-session-list__empty">{emptyMessage}</p>
  {:else}
    <ul class="underlay-session-list__items">
      {#each sessions as s (s.id)}
        <li class="underlay-session-list__item">
          <div class="underlay-session-list__meta">
            <div class="underlay-session-list__row">
              <span class="underlay-session-list__label">Last used</span>
              <span class="underlay-session-list__value">{formatDate(s.lastUsedAt)}</span>
            </div>

            <div class="underlay-session-list__row">
              <span class="underlay-session-list__label">Created</span>
              <span class="underlay-session-list__value">{formatDate(s.createdAt)}</span>
            </div>

            {#if s.ipAddress}
              <div class="underlay-session-list__row">
                <span class="underlay-session-list__label">IP</span>
                <span class="underlay-session-list__value">{s.ipAddress}</span>
              </div>
            {/if}

            {#if s.userAgent}
              <div class="underlay-session-list__row">
                <span class="underlay-session-list__label">Device</span>
                <span class="underlay-session-list__value">{s.userAgent}</span>
              </div>
            {/if}

            {#if currentSessionId && s.id === currentSessionId}
              <div class="underlay-session-list__badge">Current</div>
            {/if}
          </div>

          <div class="underlay-session-list__actions">
            <ConfirmAction
              triggerLabel="Revoke"
              triggerVariant="danger"
              title="Revoke session"
              description="This will sign out this device."
              confirmLabel="Revoke"
              onConfirm={() => onRevoke?.(s.id)}
            />
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</Card>

<style>
  .underlay-session-list__header {
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-session-list__title {
    margin: 0;
    font-size: var(--underlay-font-size-lg, 1.05rem);
  }

  .underlay-session-list__empty {
    margin: 0;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }

  .underlay-session-list__items {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-session-list__item {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: var(--underlay-space-3, 0.75rem);
    padding: var(--underlay-space-3, 0.75rem);
    border-radius: var(--underlay-radius-md, 0.75rem);
    border: 1px solid
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background: var(--underlay-color-surface-muted, rgba(255, 255, 255, 0.02));
  }

  .underlay-session-list__row {
    display: grid;
    grid-template-columns: 6rem 1fr;
    gap: var(--underlay-space-2, 0.5rem);
    font-size: var(--underlay-font-size-sm, 0.9rem);
  }

  .underlay-session-list__label {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }

  .underlay-session-list__value {
    color: var(--underlay-color-text, inherit);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-session-list__badge {
    display: inline-flex;
    margin-top: var(--underlay-space-2, 0.5rem);
    padding: 0.2em 0.45em;
    border-radius: 999px;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    border: 1px solid
      var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }

  .underlay-session-list__actions {
    display: flex;
    align-items: start;
  }

  @media (max-width: 36rem) {
    .underlay-session-list__item {
      grid-template-columns: 1fr;
    }

    .underlay-session-list__row {
      grid-template-columns: 1fr;
    }
  }
</style>
