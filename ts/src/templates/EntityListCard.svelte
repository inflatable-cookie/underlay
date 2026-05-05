<script lang="ts">
  import {
    Icon,
    ListCard,
    ListCardCounter,
    Pill,
  } from "@poodle/svelte";
  import type { EntityListCardProps } from "./entity-list-card.types";

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
    selectionMode = false,
    selected = false,
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
    leadingShape = "circle",
    leadingFill = "tint",
    badges: badgeItems = [],
    counters: counterItems = [],
    footerText = null,
    contextMenuItems = null,
    contextMenuAriaLabel = null,
    onClick = null,
    onSelectionChange = null,
    onContextAction = null,
    leading: leadingContent,
    footer: footerContent
  }: EntityListCardProps = $props();

  let hasBadges = $derived(badgeItems.length > 0);
  let hasFooter = $derived(Boolean(footerText) || counterItems.length > 0 || Boolean(footerContent));
  let showLeading = $derived(Boolean(leadingImageUrl || leadingIcon || leadingContent));

  function handleContextAction(value: string): void {
    onContextAction?.(value);
  }

  function handleSelectedChange(event: CustomEvent<{ selected: boolean }>): void {
    onSelectionChange?.(event.detail.selected);
  }

  function handleClick(event: CustomEvent<MouseEvent>): void {
    onClick?.(event.detail);
  }
</script>

{#if showLeading}
  {#if hasFooter}
    <PoodleListCard
      {title}
      {subtitle}
      {meta}
      {href}
      {size}
      {density}
      {layout}
      {interactive}
      {disabled}
      selectable={selectionMode}
      {selected}
      {selectionIndicator}
      {showReorderHandle}
      {notLive}
      {sash}
      {sashColor}
      {accentColor}
      {ariaLabel}
      {leadingShape}
      {leadingFill}
      contextMenuItems={contextMenuItems ?? []}
      {contextMenuAriaLabel}
      onContextAction={handleContextAction}
      on:selectedChange={handleSelectedChange}
      on:click={handleClick}
    >
      <svelte:fragment slot="leading">
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
          <Icon icon={leadingIcon} size="sm" />
        {/if}
      </svelte:fragment>

      <svelte:fragment slot="badges">
        {#each badgeItems as badge (`${badge.label}:${badge.tone ?? "neutral"}`)}
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
      </svelte:fragment>

      <div slot="footer" class="underlay-entity-list-card__footer">
        {#if footerText}
          <span class="underlay-entity-list-card__footer-text">{footerText}</span>
        {/if}

        {#each counterItems as counter, index (`${index}:${counter.count}`)}
          <ListCardCounter
            icon={counter.icon}
            count={counter.count}
            tooltip={counter.tooltip ?? null}
            href={counter.href ?? null}
            onClick={counter.onClick ?? null}
          />
        {/each}

        {#if footerContent}
          {@render footerContent()}
        {/if}
      </div>

    </PoodleListCard>
  {:else}
    <PoodleListCard
      {title}
      {subtitle}
      {meta}
      {href}
      {size}
      {density}
      {layout}
      {interactive}
      {disabled}
      selectable={selectionMode}
      {selected}
      {selectionIndicator}
      {showReorderHandle}
      {notLive}
      {sash}
      {sashColor}
      {accentColor}
      {ariaLabel}
      {leadingShape}
      {leadingFill}
      contextMenuItems={contextMenuItems ?? []}
      {contextMenuAriaLabel}
      onContextAction={handleContextAction}
      on:selectedChange={handleSelectedChange}
      on:click={handleClick}
    >
      <svelte:fragment slot="leading">
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
          <Icon icon={leadingIcon} size="sm" />
        {/if}
      </svelte:fragment>

      <svelte:fragment slot="badges">
        {#each badgeItems as badge (`${badge.label}:${badge.tone ?? "neutral"}`)}
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
      </svelte:fragment>

    </PoodleListCard>
  {/if}
{:else if hasFooter}
  <PoodleListCard
    {title}
    {subtitle}
    {meta}
    {href}
    {size}
    {density}
    {layout}
    {interactive}
    {disabled}
    selectable={selectionMode}
    {selected}
    {selectionIndicator}
    {showReorderHandle}
    {notLive}
    {sash}
    {sashColor}
    {accentColor}
    {ariaLabel}
    {leadingShape}
    {leadingFill}
    contextMenuItems={contextMenuItems ?? []}
    {contextMenuAriaLabel}
    onContextAction={handleContextAction}
    on:selectedChange={handleSelectedChange}
    on:click={handleClick}
  >
    <svelte:fragment slot="badges">
      {#each badgeItems as badge (`${badge.label}:${badge.tone ?? "neutral"}`)}
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
    </svelte:fragment>

    <div slot="footer" class="underlay-entity-list-card__footer">
      {#if footerText}
        <span class="underlay-entity-list-card__footer-text">{footerText}</span>
      {/if}

      {#each counterItems as counter, index (`${index}:${counter.count}`)}
        <ListCardCounter
          icon={counter.icon}
          count={counter.count}
          tooltip={counter.tooltip ?? null}
          href={counter.href ?? null}
          onClick={counter.onClick ?? null}
        />
      {/each}

      {#if footerContent}
        {@render footerContent()}
      {/if}
    </div>

  </PoodleListCard>
{:else}
  <PoodleListCard
    {title}
    {subtitle}
    {meta}
    {href}
    {size}
    {density}
    {layout}
    {interactive}
    {disabled}
    selectable={selectionMode}
    {selected}
    {selectionIndicator}
    {showReorderHandle}
    {notLive}
    {sash}
    {sashColor}
    {accentColor}
    {ariaLabel}
    {leadingShape}
    {leadingFill}
    contextMenuItems={contextMenuItems ?? []}
    {contextMenuAriaLabel}
    onContextAction={handleContextAction}
    on:selectedChange={handleSelectedChange}
    on:click={handleClick}
  >
    <svelte:fragment slot="badges">
      {#each badgeItems as badge (`${badge.label}:${badge.tone ?? "neutral"}`)}
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
    </svelte:fragment>

  </PoodleListCard>
{/if}

<style>
  .underlay-entity-list-card__leading-image {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
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
