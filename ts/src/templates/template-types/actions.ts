import type { QueryParams } from "../../client/query";
import type { ReorderableItem } from "../../patterns/reorder-controller.svelte";
import type { FetchFn, TemplateSurface } from "./primitives";

export interface BatchDialogContext {
  ids: string[];
  onSubmit: (values: Record<string, unknown>) => void;
  onCancel: () => void;
}

export interface BatchDialogConfig {
  title: string;
  content: TemplateSurface;
}

export interface InlineListDialogContext {
  close: () => void;
  refetch: () => Promise<void>;
}

export interface InlineListDialogConfig {
  title: string;
  description?: string;
  width?: "sm" | "md" | "lg" | "xl" | "full";
  content: TemplateSurface;
}

export interface InlineListItemActionConfig<TItem> {
  label: string;
  handler: (item: TItem) => void | Promise<void>;
  disabled?: boolean;
  destructive?: boolean;
  separator?: boolean;
}

export interface InlineListItemDeleteConfig<TItem> {
  title: string;
  description: string;
  confirmLabel: string;
  entityLabel?: (item: TItem) => string | null;
  handler: (item: TItem) => void | Promise<void>;
}

export interface ReorderActionState {
  active: boolean;
  available: boolean;
  dirty: boolean;
  saving: boolean;
  enter: () => void | Promise<void>;
  save: () => void | Promise<void>;
  cancel: () => void;
}

export interface BatchActionConfirm {
  title: string;
  description: string | ((count: number) => string);
  confirmLabel?: string;
  cancelLabel?: string;
}

export interface BatchActionConfig {
  id: string;
  label: string;
  tone?: "default" | "danger" | "warning";
  icon?: string;
  disabled?: boolean | ((ids: string[]) => boolean);
  confirm?: boolean | BatchActionConfirm;
  dialog?: BatchDialogConfig;
  handler: (ids: string[], values?: Record<string, unknown>) => Promise<void>;
}

export interface InlineReorderConfig {
  enabled: boolean;
  handler: (orderedIds: string[]) => Promise<void>;
  strategy?: "inline";
  successMessage?: string;
}

export interface LoadedReorderConfig<TItem> {
  enabled: boolean;
  strategy: "loaded";
  loadItems: (
    fetch: FetchFn,
    token: string | null,
    query: QueryParams,
  ) => Promise<{ items: TItem[]; error?: string }>;
  mapItems?: (items: TItem[]) => Array<TItem & ReorderableItem>;
  handler: (orderedIds: string[]) => Promise<void>;
  successMessage?: string;
}

export interface CustomReorderConfig<TItem> {
  enabled: boolean;
  strategy: "custom";
}

export type ReorderConfig<TItem = unknown> =
  | InlineReorderConfig
  | LoadedReorderConfig<TItem>
  | CustomReorderConfig<TItem>;

export interface ReorderErrorResult {
  message: string;
  highlightedIds?: string[];
}
