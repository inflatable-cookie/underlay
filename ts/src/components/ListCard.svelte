<script lang="ts">
  import ListCardBody from "./list-card/ListCardBody.svelte";
  import ListCardCompactContent from "./list-card/ListCardCompactContent.svelte";
  import ListCardContainer from "./list-card/ListCardContainer.svelte";
  import ListCardMediaSlot from "./list-card/ListCardMediaSlot.svelte";
  import type { Snippet } from "svelte";

  type ListCardVariant = "default" | "compact";

  interface Props {
    href?: string | null;
    title: string;
    /** Optional content to render after the title (e.g., badges, tags) */
    titleSuffix?: Snippet;
    subtitle?: string | null;
    ariaLabel?: string | null;
    accent?: string | null;
    /** Visual variant - 'compact' shows small icon + title only for reorder mode */
    variant?: ListCardVariant;
    /** When false, card displays with reduced opacity and dashed border to indicate draft/hidden status */
    isLive?: boolean;
    /** Show drag handle for reorder mode (only visible in compact variant) */
    showDragHandle?: boolean;
    /** Whether this card is selected (enables selection mode when provided) */
    selected?: boolean;
    /** Callback when selection changes - providing this enables selection mode */
    onSelectionChange?: (selected: boolean) => void;
    media?: Snippet;
    trailing?: Snippet;
    /** Renders the actions menu. When provided, the media area becomes a custom trigger containing the icon + dots.
     * The snippet receives `trigger` (the media content to render) and `align` (recommended dropdown alignment). */
    actions?: Snippet<[{ trigger: Snippet; align: "start" | "center" | "end" }]>;
    /** Where to place the actions trigger. Defaults to "media". */
    actionsPlacement?: "media" | "media-overlay" | "trailing";
    children?: Snippet;
    onclick?: ((event: MouseEvent) => void) | null;
  }

  let {
    href = null,
    title,
    titleSuffix,
    subtitle = null,
    ariaLabel = null,
    accent = null,
    variant = "default",
    isLive = true,
    showDragHandle = false,
    selected = false,
    onSelectionChange,
    media,
    trailing,
    actions,
    actionsPlacement = "media",
    children,
    onclick = null
  }: Props = $props();

  let hasActions = $derived(Boolean(actions));
  let isSelectionMode = $derived(Boolean(onSelectionChange));
  let style = $derived(accent ? `--underlay-list-card-accent: ${accent};` : undefined);
  let isCompact = $derived(variant === "compact");
  let cardClass = $derived([
    "underlay-list-card",
    !isLive && "underlay-list-card--draft",
    isCompact && "underlay-list-card--compact",
    isSelectionMode && "underlay-list-card--selectable"
  ].filter(Boolean).join(" "));

  function handleSelectionToggle(e: Event) {
    e.stopPropagation();
    onSelectionChange?.(!selected);
  }

  function handleCardClick(e: MouseEvent) {
    // In selection mode, clicking anywhere on the card toggles selection
    if (isSelectionMode) {
      e.preventDefault();
      onSelectionChange?.(!selected);
    }
  }
</script>

{#snippet fullContent()}
  <ListCardMediaSlot
    {media}
    {actions}
    {actionsPlacement}
    {isSelectionMode}
    {selected}
    onSelectionToggle={handleSelectionToggle}
  />
  <ListCardBody
    {title}
    {titleSuffix}
    {subtitle}
    {trailing}
    {actions}
    {actionsPlacement}
    {children}
  />
{/snippet}

{#snippet compactContent()}
  <ListCardCompactContent {title} {showDragHandle} {media} />
{/snippet}

<ListCardContainer
  {href}
  {title}
  {ariaLabel}
  {isLive}
  {isCompact}
  {isSelectionMode}
  {selected}
  {cardClass}
  {style}
  {onclick}
  onCardClick={handleCardClick}
  fullContent={fullContent}
  compactContent={compactContent}
/>
