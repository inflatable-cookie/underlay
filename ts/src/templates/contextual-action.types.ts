import type { TemplateSurface } from "./template.types";

export type ContextActionResultMode = "client_prefill" | "backend_mutation" | "suggestion_review";

export type ContextActionRunState = "idle" | "validating" | "running" | "succeeded" | "failed";

export interface ContextActionModelOption {
  alias: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface ContextActionFieldOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export type ContextActionFieldType = "text" | "textarea" | "select" | "number" | "checkbox";

export interface ContextActionInputField {
  id: string;
  label: string;
  type: ContextActionFieldType;
  description?: string;
  placeholder?: string;
  required?: boolean;
  options?: ContextActionFieldOption[];
  defaultValue?: string | number | boolean | null;
  min?: number;
  max?: number;
  step?: number;
  rows?: number;
}

export interface ContextActionDefinition {
  id: string;
  name: string;
  description: string;
  routeMatcher?: string;
  resultMode: ContextActionResultMode;
  defaultModelAlias?: string;
  modelOptions?: ContextActionModelOption[];
  fields?: ContextActionInputField[];
  form?: ContextActionDialogForm;
  submitLabel?: string;
}

export interface ContextActionSubmitDetail {
  action: ContextActionDefinition;
  values: Record<string, unknown>;
  selectedModelAlias?: string;
}

export interface ContextActionDialogFormContext {
  action: ContextActionDefinition;
  values: Record<string, unknown>;
  selectedModelAlias?: string;
  setValue: (fieldId: string, value: unknown) => void;
  submit: () => void;
  cancel: () => void;
}

export type ContextActionDialogForm = TemplateSurface;
