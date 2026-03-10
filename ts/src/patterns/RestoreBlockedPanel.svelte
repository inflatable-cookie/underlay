<script lang="ts">
  import type { RestoreBlockedResult } from "../client/soft-delete";
  import Banner from "./Banner.svelte";
  import {
    type RestoreBlockedAction,
    type RestoreBlockedActionResolver,
    type RestoreReferenceFormatter,
    formatRestoreBlockedHeadline,
    formatRestoreBlockerSummary,
    formatRestoreFieldConflict,
    formatRestoreReference,
  } from "./restore-blocked";

  interface Props {
    result: RestoreBlockedResult;
    title?: string;
    getActions?: RestoreBlockedActionResolver;
    formatReference?: RestoreReferenceFormatter;
    embedded?: boolean;
  }

  let {
    result,
    title = "Restore blocked",
    getActions = undefined,
    formatReference = undefined,
    embedded = false,
  }: Props = $props();

  const headline = $derived(formatRestoreBlockedHeadline(result));

  function dedupeActions(actions: RestoreBlockedAction[]): RestoreBlockedAction[] {
    const seen = new Set<string>();
    return actions.filter((action) => {
      const key = `${action.label}::${action.href}`;
      if (seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    });
  }

  function getBlockerActions(blocker: RestoreBlockedResult["blockers"][number]): RestoreBlockedAction[] {
    if (!getActions) {
      return [];
    }

    const actions: RestoreBlockedAction[] = [];

    actions.push(
      ...(getActions({
        blocker,
        reference: blocker.entity,
        role: "entity",
      }) ?? []),
    );

    if (blocker.parent) {
      actions.push(
        ...(getActions({
          blocker,
          reference: blocker.parent,
          role: "parent",
        }) ?? []),
      );
    }

    for (const conflict of blocker.fieldConflicts) {
      if (!conflict.activeOccupant) {
        continue;
      }

      actions.push(
        ...(getActions({
          blocker,
          reference: conflict.activeOccupant,
          role: "active_occupant",
          conflict,
        }) ?? []),
      );
    }

    return dedupeActions(actions);
  }
</script>

<section class="underlay-restore-blocked-panel" aria-label={title}>
  <Banner variant="error" message={headline} />

  <div
    class="underlay-restore-blocked-panel__body"
    class:underlay-restore-blocked-panel__body--embedded={embedded}
  >
    <h2 class="underlay-restore-blocked-panel__title">{title}</h2>
    <p class="underlay-restore-blocked-panel__description">
      Resolve the blockers below, then try the restore again.
    </p>

    <ul class="underlay-restore-blocked-panel__list">
      {#each result.blockers as blocker}
        <li class="underlay-restore-blocked-panel__item">
          <p class="underlay-restore-blocked-panel__item-title">
            {formatRestoreReference(
              blocker.entity,
              formatReference
                ? (context) => formatReference({
                  blocker,
                  reference: blocker.entity,
                  role: "entity",
                  conflict: context.conflict,
                })
                : undefined,
            )}
          </p>
          <p class="underlay-restore-blocked-panel__item-summary">
            {formatRestoreBlockerSummary(blocker, formatReference)}
          </p>

          {#if blocker.fieldConflicts.length > 0}
            <ul class="underlay-restore-blocked-panel__conflicts">
              {#each blocker.fieldConflicts as conflict}
                <li>{formatRestoreFieldConflict(conflict, blocker, formatReference)}</li>
              {/each}
            </ul>
          {/if}

          {#if blocker.parent}
            <p class="underlay-restore-blocked-panel__meta">
              Depends on {formatRestoreReference(
                blocker.parent,
                formatReference
                  ? () => formatReference({
                    blocker,
                    reference: blocker.parent!,
                    role: "parent",
                  })
                  : undefined,
              )}.
            </p>
          {/if}

          {#if blocker.fieldConflicts.some((conflict) => conflict.resolutionHints.length > 0)}
            <ul class="underlay-restore-blocked-panel__hints">
              {#each blocker.fieldConflicts as conflict}
                {#each conflict.resolutionHints as hint}
                  <li>{hint}</li>
                {/each}
              {/each}
            </ul>
          {/if}

          {#each [getBlockerActions(blocker)] as actions}
            {#if actions.length > 0}
              <div class="underlay-restore-blocked-panel__actions">
                {#each actions as action}
                  <a
                    href={action.href}
                    class="underlay-button underlay-button--pill underlay-button--subtle underlay-button--sm underlay-restore-blocked-panel__action-link"
                  >
                    {action.label}
                  </a>
                {/each}
              </div>
            {/if}
          {/each}
        </li>
      {/each}
    </ul>
  </div>
</section>

<style>
  .underlay-restore-blocked-panel {
    display: grid;
    gap: 0.875rem;
  }

  .underlay-restore-blocked-panel__body {
    border: 1px solid rgba(239, 68, 68, 0.22);
    background: rgba(127, 29, 29, 0.08);
    border-radius: 0.875rem;
    padding: 1rem 1.125rem;
    display: grid;
    gap: 0.875rem;
  }

  .underlay-restore-blocked-panel__body--embedded {
    border: none;
    background: transparent;
    border-radius: 0;
    padding: 0;
  }

  .underlay-restore-blocked-panel__title {
    margin: 0;
    font-size: 1rem;
    font-weight: 700;
    color: var(--underlay-color-text-primary, #f3f4f6);
  }

  .underlay-restore-blocked-panel__description {
    margin: 0;
    color: var(--underlay-color-text-secondary, #d1d5db);
  }

  .underlay-restore-blocked-panel__list,
  .underlay-restore-blocked-panel__conflicts,
  .underlay-restore-blocked-panel__hints {
    margin: 0;
    padding-left: 1.1rem;
  }

  .underlay-restore-blocked-panel__list {
    display: grid;
    gap: 0.875rem;
  }

  .underlay-restore-blocked-panel__item {
    display: grid;
    gap: 0.375rem;
    color: var(--underlay-color-text-secondary, #d1d5db);
  }

  .underlay-restore-blocked-panel__item-title {
    margin: 0;
    font-weight: 600;
    color: var(--underlay-color-text-primary, #f3f4f6);
  }

  .underlay-restore-blocked-panel__item-summary,
  .underlay-restore-blocked-panel__meta {
    margin: 0;
  }

  .underlay-restore-blocked-panel__conflicts,
  .underlay-restore-blocked-panel__hints {
    display: grid;
    gap: 0.25rem;
  }

  .underlay-restore-blocked-panel__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .underlay-restore-blocked-panel__action-link {
    text-decoration: none;
  }
</style>
