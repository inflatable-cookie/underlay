import type { Snippet } from "svelte";
import type { TemplateSurface } from "./primitives";

export interface DetailMetaItemConfig {
  label: string;
  value: string | TemplateSurface;
  separator?: boolean;
  code?: boolean;
}

export interface DetailItemConfig {
  label: string;
  value: string | Snippet;
  description?: string;
  emptyText?: string;
  truncateValue?: boolean;
  layout?: "inline" | "stacked";
  presentation?: "simple" | "surface";
  span?: "full" | "half" | null;
}

export interface DetailTabConfig<TItem> {
  id: string;
  label: string;
  count?: number;
  content?: TemplateSurface;
  separator?: boolean;
}

export interface DetailActionConfirm {
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
}

export interface DetailActionConfig {
  label: string;
  tone?: "default" | "danger" | "warning";
  handler: () => void;
  confirm?: boolean | DetailActionConfirm;
}
