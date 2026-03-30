declare module "*.svelte" {
  import type { Component } from "svelte";

  const component: Component<Record<string, unknown>>;
  export default component;

  // Named exports used by some components via `<script module>`.
  // These are declared broadly so `tsc` can typecheck our TS entrypoints
  // that re-export them; `svelte-check` remains the source of truth.

  export type DateRangeInput = string | Date | null | undefined;
  export type DateRangeStyle = "adaptive" | "full";
  export interface DateRangeFormatOptions {
    locale?: string;
    style?: DateRangeStyle;
    hideDays?: boolean;
  }
  export function formatDateWithOrdinal(
    input: DateRangeInput,
    locale?: string
  ): string | null;
  export function formatAdaptiveDateRange(
    startInput: DateRangeInput,
    endInput: DateRangeInput,
    options?: DateRangeFormatOptions
  ): string | null;

  export interface RangeSliderOption {
    value: string;
    label?: string;
    tone?: "default" | "primary" | "success" | "warning" | "danger";
    color?: string;
  }

  export interface LogEntry {
    id: string;
    occurredAt: string;
    actor?: LogActor | null;
    action: string;
    resourceType: string;
    resourceId: string;
    resourceLabel?: string;
    details?: Record<string, unknown>;
  }

  export interface LogActor {
    id: string;
    email?: string;
    name?: string;
  }

  export interface LogFilter {
    field: string;
    label: string;
    type: "select" | "date";
    options?: { value: string; label: string }[];
    placeholder?: string;
  }

  export type LogActionType =
    | "create"
    | "update"
    | "delete"
    | "restore"
    | "upload"
    | "login"
    | "logout"
    | "security"
    | "other";

  export type StatVariant =
    | "default"
    | "success"
    | "warning"
    | "danger"
    | "info";

  export type TabsVariant = "pills" | "boxed" | "underline" | "plain";
  export type TabsSize = "default" | "sm";

  export interface SchemaMismatchInfo {
    actualSchema: string | null;
    expectedSchema: string;
  }
}

declare module "*.css" {
  const css: string;
  export default css;
}
