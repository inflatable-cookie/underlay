import type { Readable } from "svelte/store";
import type { StorageOptions, StorageWrapper } from "../storage";

export type FieldErrors = Record<string, string>;

export interface FormStateOptions<T = unknown> {
  onSuccess?: (data: T) => void | Promise<void>;
  onError?: (error: string, fieldErrors?: FieldErrors) => void;
  onSubmit?: () => void;
  initialFieldErrors?: FieldErrors;
  initialError?: string | null;
  resetOnSuccess?: boolean;
  autoSave?: FormAutoSaveOptions;
}

export interface FormAutoSaveOptions extends Pick<
  StorageOptions,
  "ttl" | "expiresAt"
> {
  key: string;
  storage?: "local" | "session" | StorageWrapper;
  debounce?: number;
  restoreOnMount?: boolean;
  clearOnSuccess?: boolean;
}

export interface FormState<T = unknown> {
  readonly isSubmitting: boolean;
  readonly error: string | null;
  readonly fieldErrors: FieldErrors;
  readonly isSuccess: boolean;
  startSubmit: () => void;
  setSuccess: (data?: T) => void;
  setError: (message: string, fieldErrors?: FieldErrors) => void;
  setFieldErrors: (errors: FieldErrors) => void;
  clearFieldError: (field: string) => void;
  reset: () => void;
  clearDraft: () => void;
  enhance: EnhanceFn;
}

export type FormStateWithStore<T = unknown> = FormState<T> & {
  state: Readable<FormStateInternal>;
};

export type EnhanceFn = (form: HTMLFormElement) => {
  destroy?: () => void;
};

export interface FormStateInternal {
  isSubmitting: boolean;
  error: string | null;
  fieldErrors: FieldErrors;
  isSuccess: boolean;
}

export type FormDraft = Record<string, DraftValue>;

export type DraftValue =
  | { kind: "single"; value: string }
  | { kind: "boolean"; checked: boolean }
  | { kind: "multi"; values: string[] };
