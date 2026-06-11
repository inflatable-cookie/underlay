<script lang="ts">
  import {
    Icon,
    ListCard,
    ListCardCounter,
    Pill,
  } from "@poodle/svelte";
  import type { EntityListCardModeDisplay, EntityListCardProps } from "./entity-list-card.types";

  const PoodleListCard: any = ListCard;

  let {
    title,
    subtitle = null,
    meta = null,
    href = null,
    size = null,
    density = null,
    layout = "default",
    interactive = false,
    disabled = false,
    reorderMode = false,
    selectionMode = false,
    reorderDisplay = undefined,
    selectionDisplay = undefined,
    selected = false,
    highlighted = false,
    selectionIndicator = "checkbox",
    showReorderHandle = false,
    notLive = false,
    sash = null,
    sashColor = null,
    accentColor = null,
    ariaLabel = null,
    leadingIcon = null,
    leadingImageUrl = null,
    leadingImageAlt = null,
    leadingShape = "rounded-square",
    leadingFill = "tint",
    badges: badgeItems = [],
    counters: counterItems = [],
    footerText = null,
    contextMenuItems = null,
    contextMenuAriaLabel = null,
    contextMenuTrigger = "context",
    onClick = null,
    onSelectionChange = null,
    onContextAction = null,
    titleContent,
    leading: leadingContent,
    corner: cornerContent,
    footer: footerContent
  }: EntityListCardProps = $props();

  const defaultReorderDisplay: EntityListCardModeDisplay = {
    layout: "compact",
    size: "xs",
    showSubtitle: false,
    showMeta: false,
    showBadges: false,
    showFooter: false,
    showCounters: false
  };

  let activeDisplay = $derived.by((): EntityListCardModeDisplay | null => {
    if (reorderMode) return { ...defaultReorderDisplay, ...(reorderDisplay ?? {}) };
    if (selectionMode && selectionDisplay) return selectionDisplay;
    return null;
  });

  let resolvedLayout = $derived(activeDisplay?.layout ?? layout);
  let resolvedSize = $derived(
    activeDisplay?.size ?? (resolvedLayout === "compact" ? "sm" : size)
  );
  let resolvedDensity = $derived(
    activeDisplay?.density ?? (resolvedLayout === "compact" ? "compact" : density)
  );
  let resolvedHref = $derived(reorderMode ? null : href);
  let resolvedInteractive = $derived(reorderMode ? false : (interactive || Boolean(onClick)));
  let resolvedContextMenuItems = $derived(reorderMode || selectionMode ? [] : (contextMenuItems ?? []));
  let showLeading = $derived(Boolean(leadingImageUrl || leadingIcon || leadingContent));
  let resolvedShowReorderHandle = $derived(reorderMode || showReorderHandle);
  let resolvedSubtitle = $derived(activeDisplay?.showSubtitle === false ? null : subtitle);
  let resolvedMeta = $derived(activeDisplay?.showMeta === false ? null : meta);
  let resolvedBadgeItems = $derived(activeDisplay?.showBadges === false ? [] : badgeItems);
  let resolvedCounterItems = $derived(activeDisplay?.showCounters === false ? [] : counterItems);
  let resolvedFooterText = $derived(activeDisplay?.showFooter === false ? null : footerText);
  let resolvedFooterContent = $derived(activeDisplay?.showFooter === false ? null : footerContent);
  let hasFooter = $derived(
    Boolean(resolvedFooterText) ||
      resolvedCounterItems.length > 0 ||
      Boolean(resolvedFooterContent)
  );

  function handleContextAction(value: string): void {
    onContextAction?.(value);
  }

  function handleSelectedChange(selectedValue: boolean): void {
    onSelectionChange?.(selectedValue);
  }

  function handleClick(event: MouseEvent): void {
    onClick?.(event);
  }
</script>

{#snippet leadingVisual()}
  {#if leadingContent}
    {@render leadingContent()}
  {:else if leadingImageUrl}
    <img
      class="underlay-entity-list-card__leading-image"
      src={leadingImageUrl}
      alt={leadingImageAlt ?? ""}
      loading="lazy"
    />
  {:else if leadingIcon}
    <Icon icon={leadingIcon} />
  {/if}
{/snippet}

{#snippet badgeContent()}
  {#each resolvedBadgeItems as badge (`${badge.label}:${badge.tone ?? "neutral"}`)}
    <Pill
      tone={badge.tone ?? "neutral"}
      appearance={badge.appearance ?? "subtle"}
      size={badge.size ?? "sm"}
      accent={badge.accent ?? null}
      muted={badge.muted ?? false}
      ariaLabel={badge.ariaLabel ?? undefined}
    >
      {badge.label}
    </Pill>
  {/each}
{/snippet}

{#snippet footerBlock()}
  <div class="underlay-entity-list-card__footer">
    {#if resolvedFooterText}
      <span class="underlay-entity-list-card__footer-text">{resolvedFooterText}</span>
    {/if}

    {#each resolvedCounterItems as counter, index (`${index}:${counter.count}`)}
      <ListCardCounter
        icon={counter.icon}
        count={counter.count}
        tooltip={counter.tooltip ?? null}
        href={counter.href ?? null}
        onClick={counter.onClick ?? null}
      />
    {/each}

    {#if resolvedFooterContent}
      {@render resolvedFooterContent()}
    {/if}
  </div>
{/snippet}

{#if showLeading}
  {#if hasFooter}
    <PoodleListCard
      {title}
      subtitle={resolvedSubtitle}
      meta={resolvedMeta}
      href={resolvedHref}
      size={resolvedSize}
      density={resolvedDensity}
      layout={resolvedLayout}
      interactive={resolvedInteractive}
      {disabled}
      selectable={selectionMode}
      {selected}
      {highlighted}
      {selectionIndicator}
      showReorderHandle={resolvedShowReorderHandle}
      {notLive}
      {sash}
      {sashColor}
      {accentColor}
      {ariaLabel}
      {leadingShape}
      {leadingFill}
      contextMenuItems={resolvedContextMenuItems}
      {contextMenuAriaLabel}
      {contextMenuTrigger}
      onContextAction={handleContextAction}
      onSelectedChange={handleSelectedChange}
      onClick={handleClick}
      {titleContent}
      leading={leadingVisual}
      badges={resolvedBadgeItems.length > 0 ? badgeContent : undefined}
      corner={cornerContent}
      footer={footerBlock}
    />
  {:else}
    <PoodleListCard
      {title}
      subtitle={resolvedSubtitle}
      meta={resolvedMeta}
      href={resolvedHref}
      size={resolvedSize}
      density={resolvedDensity}
      layout={resolvedLayout}
      interactive={resolvedInteractive}
      {disabled}
      selectable={selectionMode}
      {selected}
      {highlighted}
      {selectionIndicator}
      showReorderHandle={resolvedShowReorderHandle}
      {notLive}
      {sash}
      {sashColor}
      {accentColor}
      {ariaLabel}
      {leadingShape}
      {leadingFill}
      contextMenuItems={resolvedContextMenuItems}
      {contextMenuAriaLabel}
      {contextMenuTrigger}
      onContextAction={handleContextAction}
      onSelectedChange={handleSelectedChange}
      onClick={handleClick}
      {titleContent}
      leading={leadingVisual}
      badges={resolvedBadgeItems.length > 0 ? badgeContent : undefined}
      corner={cornerContent}
    />
  {/if}
{:else if hasFooter}
  <PoodleListCard
    {title}
    subtitle={resolvedSubtitle}
    meta={resolvedMeta}
    href={resolvedHref}
    size={resolvedSize}
    density={resolvedDensity}
    layout={resolvedLayout}
    interactive={resolvedInteractive}
    {disabled}
    selectable={selectionMode}
    {selected}
    {highlighted}
    {selectionIndicator}
    showReorderHandle={resolvedShowReorderHandle}
    {notLive}
    {sash}
    {sashColor}
    {accentColor}
    {ariaLabel}
    {leadingShape}
    {leadingFill}
    contextMenuItems={resolvedContextMenuItems}
    {contextMenuAriaLabel}
    onContextAction={handleContextAction}
    onSelectedChange={handleSelectedChange}
    onClick={handleClick}
    {titleContent}
    badges={resolvedBadgeItems.length > 0 ? badgeContent : undefined}
    corner={cornerContent}
    footer={footerBlock}
  />
{:else}
  <PoodleListCard
    {title}
    subtitle={resolvedSubtitle}
    meta={resolvedMeta}
    href={resolvedHref}
    size={resolvedSize}
    density={resolvedDensity}
    layout={resolvedLayout}
    interactive={resolvedInteractive}
    {disabled}
    selectable={selectionMode}
    {selected}
    {highlighted}
    {selectionIndicator}
    showReorderHandle={resolvedShowReorderHandle}
    {notLive}
    {sash}
    {sashColor}
    {accentColor}
    {ariaLabel}
    {leadingShape}
    {leadingFill}
    contextMenuItems={resolvedContextMenuItems}
    {contextMenuAriaLabel}
    onContextAction={handleContextAction}
    onSelectedChange={handleSelectedChange}
    onClick={handleClick}
    {titleContent}
    badges={resolvedBadgeItems.length > 0 ? badgeContent : undefined}
    corner={cornerContent}
  />
{/if}

<style>
  .underlay-entity-list-card__leading-image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  :global(.poodle-list-card) {
    min-width: 0;
  }

  .underlay-entity-list-card__footer-text {
    min-width: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .underlay-entity-list-card__footer {
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-md);
    min-width: 0;
  }
</style>
