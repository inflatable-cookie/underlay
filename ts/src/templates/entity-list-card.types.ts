import type {
  ControlDensity,
  ControlSize,
  IconProp,
  MenuItem,
  PillAppearance,
  PillSize,
  PillTone
} from "@poodle/svelte";

export interface EntityListCardBadge {
  label: string;
  tone?: PillTone;
  appearance?: PillAppearance;
  size?: PillSize;
  accent?: string | null;
  muted?: boolean;
  ariaLabel?: string | null;
}

export interface EntityListCardCounter {
  icon: IconProp;
  count: number;
  tooltip?: string | null;
  href?: string | null;
  onClick?: ((event: MouseEvent) => void) | null;
}

export interface EntityListCardModeDisplay {
  layout?: "default" | "compact";
  size?: ControlSize | null;
  density?: ControlDensity | null;
  showSubtitle?: boolean;
  showMeta?: boolean;
  showBadges?: boolean;
  showFooter?: boolean;
  showCounters?: boolean;
}

export type EntityListCardMenuTrigger = "context" | "leading";

export interface EntityListCardProps {
  title: string;
  subtitle?: string | null;
  meta?: string | null;
  href?: string | null;
  size?: ControlSize | null;
  density?: ControlDensity | null;
  layout?: "default" | "compact";
  interactive?: boolean;
  disabled?: boolean;
  reorderMode?: boolean;
  selectionMode?: boolean;
  reorderDisplay?: EntityListCardModeDisplay;
  selectionDisplay?: EntityListCardModeDisplay;
  selected?: boolean;
  selectionIndicator?: "none" | "checkbox";
  showReorderHandle?: boolean;
  notLive?: boolean;
  sash?: string | null;
  sashColor?: string | null;
  accentColor?: string | null;
  ariaLabel?: string | null;
  leadingIcon?: IconProp | null;
  leadingImageUrl?: string | null;
  leadingImageAlt?: string | null;
  leadingShape?: "circle" | "rounded-square";
  leadingFill?: "tint" | "solid";
  badges?: EntityListCardBadge[];
  counters?: EntityListCardCounter[];
  footerText?: string | null;
  contextMenuItems?: MenuItem[] | null;
  contextMenuAriaLabel?: string | null;
  contextMenuTrigger?: EntityListCardMenuTrigger;
  onClick?: ((event: MouseEvent) => void) | null;
  onSelectionChange?: ((selected: boolean) => void) | null;
  onContextAction?: ((value: string) => void) | null;
  leading?: any;
  footer?: any;
}
