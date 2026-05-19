<script lang="ts" generics="TUsage extends MediaUsageListItem = MediaUsageListItem">
  import { Code, EmptyState, InlineListSection } from "@poodle/svelte";
  import type { MediaUsageListItem } from "./template.types";

  interface Props {
    usages: TUsage[];
    title?: string;
    emptyTitle?: string;
  }

  let {
    usages,
    title = "Usages",
    emptyTitle = "This media is not used anywhere yet"
  }: Props = $props();

  function codeValue(value?: string | null, fallback = "manual"): string {
    return value?.trim() || fallback;
  }
</script>

<div class="underlay-details-content">
  {#if usages.length === 0}
    <EmptyState title={emptyTitle} size="compact" />
  {:else}
    <InlineListSection {title} items={usages}>
      {#snippet item(usage)}
        <div class="underlay-media-usage-list__item-content">
          <span class="underlay-media-usage-list__dot"></span>
          <span class="underlay-media-usage-list__label-group">
            <span class="underlay-media-usage-list__label">{usage.usedByType}</span>
            <span class="underlay-media-usage-list__sublabel">
              <Code inline source={codeValue(usage.usedById)} />
              {#if usage.ownerField}
                <span class="underlay-media-usage-list__field"> · {usage.ownerField}</span>
              {/if}
            </span>
            {#if usage.usageRole || usage.locatorKind || usage.locatorKey}
              <span class="underlay-media-usage-list__sublabel">
                {#if usage.usageRole}
                  <span>{usage.usageRole}</span>
                {/if}
                {#if usage.locatorKind && usage.locatorKind !== "field"}
                  <span class="underlay-media-usage-list__field">
                    · {usage.locatorKind}{usage.locatorKey ? ":" : ""}
                  </span>
                {:else if usage.locatorKind}
                  <span class="underlay-media-usage-list__field">· {usage.locatorKind}</span>
                {/if}
                {#if usage.locatorKey}
                  <Code inline source={usage.locatorKey} />
                {/if}
              </span>
            {/if}
          </span>
        </div>
      {/snippet}
    </InlineListSection>
  {/if}
</div>

<style>
  .underlay-media-usage-list__item-content {
    display: flex;
    align-items: flex-start;
    gap: 0.625rem;
    min-width: 0;
    flex: 1;
  }

  .underlay-media-usage-list__dot {
    width: 0.375rem;
    height: 0.375rem;
    margin-top: 0.45rem;
    border-radius: 999rem;
    background: #6366f1;
    flex-shrink: 0;
  }

  .underlay-media-usage-list__label-group {
    min-width: 0;
    display: grid;
    gap: 0.125rem;
  }

  .underlay-media-usage-list__label {
    font-size: 0.9rem;
    font-weight: 500;
  }

  .underlay-media-usage-list__sublabel {
    font-size: 0.8rem;
    color: var(--underlay-color-text-muted, #9ca3af);
    display: flex;
    gap: 0.35rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .underlay-media-usage-list__field {
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
