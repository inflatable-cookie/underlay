import type { QueryParams, SortDirection } from "../../client/query";
import type { FetchFn, PagedListResult } from "./primitives";

export interface TemplateFilterOption {
  value: string;
  label: string;
}

export interface TemplateSortField {
  key: string;
  label: string;
  defaultDirection?: SortDirection;
}

export interface FilterConfig {
  id: string;
  type: "search" | "select" | "date" | "number" | "sort";
  label: string;
  disabled?: boolean;
  options?: TemplateFilterOption[];
  loadOptions?: (context?: {
    query?: string;
    value?: string | null;
    loadKey?: string | null;
  }) => Promise<TemplateFilterOption[]>;
  loadKey?: string;
  searchable?: boolean;
  placeholder?: string;
  sortFields?: TemplateSortField[];
}

export interface ListVariantDefinition {
  id: string;
  label: string;
  description?: string;
  tone?: "default" | "info" | "success" | "warning" | "danger";
  count?: number;
  isDefault?: boolean;
}

export interface ListFilterDefinition extends FilterConfig {
  variants?: string[];
}

export interface ListCapabilities {
  defaultVariantId?: string;
  variants: ListVariantDefinition[];
  filters: ListFilterDefinition[];
}

export type EntityListDataLoader<TItem> = (
  fetch: FetchFn,
  token: string | null,
  query: QueryParams,
) => Promise<PagedListResult<TItem>>;

export type EntityListCapabilitiesLoader = (
  fetch: FetchFn,
  token: string | null,
) => Promise<ListCapabilities>;
