import { getContext, setContext } from "svelte";

const FORM_FIELD_ERRORS_KEY = "underlay-form-field-errors";

export type FormFieldErrors = {
  readonly errors: Record<string, string> | null;
  fieldError(name: string): string | undefined;
};

export type FormFieldErrorsSource =
  | Record<string, string>
  | null
  | (() => Record<string, string> | null);

/**
 * Set by form-page templates (EntityFormPage) during component initialization
 * so nested form content can bind field-level errors reactively without prop
 * drilling. Pass a getter `() => fieldErrors` so updates after submit stay live.
 */
export function setFormFieldErrors(source: FormFieldErrorsSource): void {
  const getter = typeof source === "function" ? source : () => source;
  setContext(FORM_FIELD_ERRORS_KEY, getter);
}

/**
 * Read field errors from the enclosing form page. Returns a resolver:
 * `error={fieldError("title")}`.
 */
export function useFormFieldErrors(): (name: string) => string | undefined {
  const getter = getContext<(() => Record<string, string> | null) | undefined>(
    FORM_FIELD_ERRORS_KEY,
  );
  return (name: string) => getter?.()?.[name];
}
