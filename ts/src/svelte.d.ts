declare module "*.svelte" {
  const component: any;
  export default component;

  // Named exports used by some components via `<script module>`.
  // These are declared broadly so `tsc` can typecheck our TS entrypoints
  // that re-export them; `svelte-check` remains the source of truth.

  export interface ValidationResult {
    valid: boolean;
    message?: string;
    suggestion?: string;
  }

  export interface DataTableColumn<T = unknown> {
    key: string;
    label: string;
    width?: string;
    sortable?: boolean;
    filterable?: boolean;
    filterType?: "text" | "select" | "date";
    filterOptions?: Array<{ value: string; label: string } | string>;
    formatter?: (value: unknown, row: T) => string;
    align?: "left" | "center" | "right";
    hideOnMobile?: boolean;
    hideable?: boolean;
  }

  export interface DataTableAction<T = unknown> {
    label: string;
    icon?: string;
    href?: string | ((row: T) => string);
    onClick?: (row: T) => void;
    variant?: "default" | "danger" | "primary";
    confirm?: string;
    show?: (row: T) => boolean;
  }

  export interface DataTablePagination {
    page: number;
    limit: number;
    total: number;
  }

  export interface DataTableSort {
    key: string;
    direction: "asc" | "desc";
  }

  export type DataTableFilters = Record<string, string>;

  export const DEFAULT_LIMIT_OPTIONS: number[];

  export function exportToCsv<T extends Record<string, unknown>>(
    data: T[],
    columns: DataTableColumn<T>[],
    filename?: string
  ): void;

  export interface FileUploadItem {
    file: File;
    id: string;
    progress: number;
    status: "pending" | "uploading" | "complete" | "error";
    error?: string;
    previewUrl?: string;
    originalFile?: File;
  }

  export interface ImageCompressionOptions {
    maxWidth?: number;
    maxHeight?: number;
    quality?: number;
    format?: "image/jpeg" | "image/png" | "image/webp";
  }

  export const DEFAULT_COMPRESSION: ImageCompressionOptions;

  export function compressImage(
    file: File,
    options?: ImageCompressionOptions
  ): Promise<File>;

  export interface PaginationState {
    page: number;
    limit: number;
    total: number;
  }
}

declare module "*.css" {
  const css: string;
  export default css;
}
