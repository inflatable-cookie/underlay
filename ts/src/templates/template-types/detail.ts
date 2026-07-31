import type { TemplateSurface } from "./primitives";

export interface DetailMetaItemConfig {
  label: string;
  value: string | TemplateSurface;
  separator?: boolean;
  code?: boolean;
}

export interface DetailItemConfig {
  label: string;
  value: string | TemplateSurface;
  description?: string;
  emptyText?: string;
  truncateValue?: boolean;
  layout?: "inline" | "stacked";
  presentation?: "simple" | "surface";
  span?: "full" | "half" | null;
}

export interface DetailTabConfig<TItem, TData = unknown> {
  id: string;
  label: string;
  count?: number;
  content?: TemplateSurface;
  /**
   * Data-driven tab: fetched lazily on first activation. Pair with
   * `render` to display the loaded data.
   */
  dataLoader?: (item: TItem | null) => Promise<TData>;
  /** Render the loaded tab data. Required when `dataLoader` is set. */
  render?: (data: TData | null, item: TItem | null) => TemplateSurface;
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
