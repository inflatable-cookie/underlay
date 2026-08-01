import { getContext, setContext } from "svelte";

const FORM_FIELD_ERRORS_KEY = "underlay-form-field-errors";

export type FormFieldErrors = {
  readonly errors: Record<string, string> | null;
  fieldError(name: string): string | undefined;
};

/**
 * Set by form-page templates (EntityFormPage) so nested form content can
 * bind field-level errors without prop drilling.
 */
export function setFormFieldErrors(errors: Record<string, string> | null): void {
  setContext(FORM_FIELD_ERRORS_KEY, errors);
}

/**
 * Read field errors from the enclosing form page. Returns a resolver:
 * `error={fieldError("title")}`.
 */
export function useFormFieldErrors(): (name: string) => string | undefined {
  const errors = getContext<Record<string, string> | null | undefined>(
    FORM_FIELD_ERRORS_KEY,
  );
  return (name: string) => errors?.[name];
}
