import type { Snippet } from "svelte";

export interface SelectOption {
  value: string;
  label: string;
}

export type FieldType = "text" | "textarea" | "select" | "number" | "checkbox" | "custom";

export interface BaseFieldConfig {
  id: string;
  type: FieldType;
  label: string;
  required?: boolean;
  placeholder?: string;
  helpText?: string;
}

export interface TextFieldConfig extends BaseFieldConfig {
  type: "text";
}

export interface TextareaFieldConfig extends BaseFieldConfig {
  type: "textarea";
  rows?: number;
}

export interface SelectFieldConfig extends BaseFieldConfig {
  type: "select";
  options: SelectOption[];
  loadOptions?: () => Promise<SelectOption[]>;
}

export interface NumberFieldConfig extends BaseFieldConfig {
  type: "number";
  min?: number;
  max?: number;
  step?: number;
}

export interface CheckboxFieldConfig extends BaseFieldConfig {
  type: "checkbox";
  checkboxLabel?: string;
}

export interface FieldRenderContext {
  value: unknown;
  onChange: (value: unknown) => void;
  error?: string;
  disabled: boolean;
}

export interface CustomFieldConfig extends BaseFieldConfig {
  type: "custom";
  render: Snippet<[FieldRenderContext]>;
}

export type FieldConfig =
  | TextFieldConfig
  | TextareaFieldConfig
  | SelectFieldConfig
  | NumberFieldConfig
  | CheckboxFieldConfig
  | CustomFieldConfig;
