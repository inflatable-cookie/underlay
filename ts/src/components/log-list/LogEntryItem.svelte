<script lang="ts">
  import { Pill } from "@poodle/svelte-primitives";
  import { formatRelative } from "../../patterns/i18n";
  import User from "lucide-svelte/icons/user";
  import Plus from "lucide-svelte/icons/plus";
  import Pencil from "lucide-svelte/icons/pencil";
  import Trash2 from "lucide-svelte/icons/trash-2";
  import RotateCcw from "lucide-svelte/icons/rotate-ccw";
  import Upload from "lucide-svelte/icons/upload";
  import LogIn from "lucide-svelte/icons/log-in";
  import LogOut from "lucide-svelte/icons/log-out";
  import Shield from "lucide-svelte/icons/shield";
  import type { Snippet } from "svelte";
  import type { LogActionType, LogActor, LogEntry } from "../LogList.svelte";

  type BadgeVariant = "default" | "success" | "warning" | "danger" | "info" | "muted";

  interface Props {
    entry: LogEntry;
    actionType: LogActionType;
    actionVariant: BadgeVariant;
    actionLabel: string;
    resourceTypeLabel: string;
    actorName: string;
    resourceHref: string | null;
    actorHref?: string;
    actionIcon?: Snippet<[LogActionType]>;
    entryDetails?: Snippet<[LogEntry]>;
  }

  let {
    entry,
    actionType,
    actionVariant,
    actionLabel,
    resourceTypeLabel,
    actorName,
    resourceHref,
    actorHref,
    actionIcon,
    entryDetails
  }: Props = $props();

  function mapActionTone(variant: BadgeVariant): "neutral" | "success" | "danger" {
    if (variant === "success") return "success";
    if (variant === "danger") return "danger";
    return "neutral";
  }
</script>

<li class="underlay-log-entry">
  <div class="underlay-log-entry__icon underlay-log-entry__icon--{actionType}">
    {#if actionIcon}
      {@render actionIcon(actionType)}
    {:else if actionType === "create"}
      <Plus size={14} />
    {:else if actionType === "update"}
      <Pencil size={14} />
    {:else if actionType === "delete"}
      <Trash2 size={14} />
    {:else if actionType === "restore"}
      <RotateCcw size={14} />
    {:else if actionType === "upload"}
      <Upload size={14} />
    {:else if actionType === "login"}
      <LogIn size={14} />
    {:else if actionType === "logout"}
      <LogOut size={14} />
    {:else if actionType === "security"}
      <Shield size={14} />
    {:else}
      <User size={14} />
    {/if}
  </div>

  <div class="underlay-log-entry__body">
    <div class="underlay-log-entry__main">
      <span class="underlay-log-entry__actor">
        {#if entry.actor}
          {#if actorHref}
            <a href={actorHref} class="underlay-log-entry__actor-link">{actorName}</a>
          {:else}
            {actorName}
          {/if}
        {:else}
          System
        {/if}
      </span>
      <Pill tone={mapActionTone(actionVariant)} appearance="badge" size="sm">
        {actionLabel}
      </Pill>
      {#if resourceHref}
        <a href={resourceHref} class="underlay-log-entry__resource-link">
          {resourceTypeLabel}
          {#if entry.resourceLabel}
            <span class="underlay-log-entry__resource-label">"{entry.resourceLabel}"</span>
          {/if}
        </a>
      {:else}
        <span class="underlay-log-entry__resource-type">
          {resourceTypeLabel}
        </span>
        {#if entry.resourceLabel}
          <span class="underlay-log-entry__resource-label">"{entry.resourceLabel}"</span>
        {/if}
      {/if}
    </div>

    {#if entryDetails}
      <div class="underlay-log-entry__details">
        {@render entryDetails(entry)}
      </div>
    {/if}

    <time class="underlay-log-entry__time" datetime={entry.occurredAt}>
      {formatRelative(entry.occurredAt)}
    </time>
  </div>
</li>

<style>
  .underlay-log-entry {
    display: flex;
    gap: 0.75rem;
    padding: 0.875rem 1rem;
    border-bottom: 1px solid var(--underlay-color-border-subtle, #334155);
  }

  .underlay-log-entry:last-child {
    border-bottom: none;
  }

  .underlay-log-entry:hover {
    background: var(--underlay-color-surface-hover, rgba(255, 255, 255, 0.02));
  }

  .underlay-log-entry__icon {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    background: var(--underlay-color-surface-raised, #283548);
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-log-entry__icon--create {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .underlay-log-entry__icon--update {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .underlay-log-entry__icon--delete {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }

  .underlay-log-entry__icon--restore {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .underlay-log-entry__icon--upload {
    background: rgba(168, 85, 247, 0.15);
    color: #a855f7;
  }

  .underlay-log-entry__icon--login,
  .underlay-log-entry__icon--logout {
    background: rgba(148, 163, 184, 0.15);
    color: #94a3b8;
  }

  .underlay-log-entry__icon--security {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
  }

  .underlay-log-entry__body {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .underlay-log-entry__main {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
    line-height: 1.4;
  }

  .underlay-log-entry__actor {
    font-weight: 500;
    color: var(--underlay-color-text, #f1f5f9);
  }

  .underlay-log-entry__actor-link {
    color: inherit;
    text-decoration: none;
  }

  .underlay-log-entry__actor-link:hover {
    text-decoration: underline;
    color: var(--underlay-color-primary, #3b82f6);
  }

  .underlay-log-entry__resource-type {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-log-entry__resource-link {
    color: var(--underlay-color-text-muted, #94a3b8);
    text-decoration: none;
  }

  .underlay-log-entry__resource-link:hover {
    text-decoration: underline;
    color: var(--underlay-color-primary, #3b82f6);
  }

  .underlay-log-entry__resource-link .underlay-log-entry__resource-label {
    color: inherit;
  }

  .underlay-log-entry__resource-label {
    color: var(--underlay-color-text-secondary, #cbd5e1);
    font-style: italic;
  }

  .underlay-log-entry__details {
    font-size: 0.8125rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-log-entry__time {
    font-size: 0.75rem;
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
