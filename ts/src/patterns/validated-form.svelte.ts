import { z } from "zod";

export type ValidatedFormErrors = Record<string, string>;

export interface ValidatedFormOptions<TSchema extends z.ZodObject> {
  schema: TSchema;
  initialValues: z.input<TSchema>;
  onSubmit: (values: z.output<TSchema>) => Promise<void> | void;
  onError?: (error: Error) => void;
  validateOnChange?: boolean;
}

export interface ValidatedFormResult<TSchema extends z.ZodObject> {
  readonly values: z.input<TSchema>;
  readonly errors: ValidatedFormErrors;
  readonly isValid: boolean;
  readonly isSubmitting: boolean;
  readonly submitError: string | null;
  setField: <K extends keyof z.input<TSchema>>(field: K, value: z.input<TSchema>[K]) => void;
  setValues: (values: Partial<z.input<TSchema>>) => void;
  clearError: (field: string) => void;
  reset: () => void;
  validate: () => boolean;
  submit: () => Promise<boolean>;
}

function toFieldErrors(error: z.ZodError): ValidatedFormErrors {
  const next: ValidatedFormErrors = {};

  for (const issue of error.issues) {
    const key = issue.path.length > 0 ? issue.path.join(".") : "_form";
    if (!(key in next)) {
      next[key] = issue.message;
    }
  }

  return next;
}

export function useValidatedForm<TSchema extends z.ZodObject>(
  options: ValidatedFormOptions<TSchema>,
): ValidatedFormResult<TSchema> {
  const initialValues = structuredClone(options.initialValues);
  let values = $state<z.input<TSchema>>(structuredClone(initialValues));
  let errors = $state<ValidatedFormErrors>({});
  let isSubmitting = $state(false);
  let submitError = $state<string | null>(null);

  function runValidation(): z.ZodSafeParseResult<z.output<TSchema>> {
    const result = options.schema.safeParse(values);
    errors = result.success ? {} : toFieldErrors(result.error);
    return result;
  }

  function setField<K extends keyof z.input<TSchema>>(
    field: K,
    value: z.input<TSchema>[K],
  ): void {
    values = { ...values, [field]: value };
    submitError = null;
    if (options.validateOnChange) {
      runValidation();
    }
  }

  function setValues(nextValues: Partial<z.input<TSchema>>): void {
    values = { ...values, ...nextValues };
    submitError = null;
    if (options.validateOnChange) {
      runValidation();
    }
  }

  function clearError(field: string): void {
    if (!(field in errors)) {
      return;
    }

    const { [field]: _removed, ...rest } = errors;
    errors = rest;
  }

  function reset(): void {
    values = structuredClone(initialValues);
    errors = {};
    isSubmitting = false;
    submitError = null;
  }

  function validate(): boolean {
    return runValidation().success;
  }

  async function submit(): Promise<boolean> {
    submitError = null;
    const result = runValidation();
    if (!result.success) {
      return false;
    }

    isSubmitting = true;
    try {
      await options.onSubmit(result.data);
      return true;
    } catch (cause) {
      const error =
        cause instanceof Error ? cause : new Error("Form submission failed");
      submitError = error.message;
      options.onError?.(error);
      return false;
    } finally {
      isSubmitting = false;
    }
  }

  return {
    get values() {
      return values;
    },
    get errors() {
      return errors;
    },
    get isValid() {
      return Object.keys(errors).length === 0;
    },
    get isSubmitting() {
      return isSubmitting;
    },
    get submitError() {
      return submitError;
    },
    setField,
    setValues,
    clearError,
    reset,
    validate,
    submit,
  };
}
