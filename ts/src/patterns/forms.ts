/**
 * Form state management utilities for SvelteKit applications.
 *
 * Provides a reactive form state manager that handles:
 * - Loading/submitting states
 * - Field-level and global error handling
 * - Success callbacks
 * - Form reset
 * - SvelteKit enhance integration
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createFormState } from '@decodelabs/underlay/patterns';
 *   import { enhance } from '$app/forms';
 *
 *   const form = createFormState({
 *     onSuccess: () => {
 *       // Handle success
 *     }
 *   });
 * </script>
 *
 * <form method="post" use:enhance={form.enhance}>
 *   <input name="email" disabled={form.isSubmitting} />
 *   {#if form.fieldErrors.email}
 *     <span class="error">{form.fieldErrors.email}</span>
 *   {/if}
 *
 *   {#if form.error}
 *     <div class="error">{form.error}</div>
 *   {/if}
 *
 *   <button disabled={form.isSubmitting}>
 *     {form.isSubmitting ? 'Saving...' : 'Save'}
 *   </button>
 * </form>
 * ```
 */

import { writable, get, type Writable, type Readable } from "svelte/store";

// ============================================================================
// Types
// ============================================================================

/** Field-level validation errors keyed by field name */
export type FieldErrors = Record<string, string>;

/** Form state configuration options */
export interface FormStateOptions<T = unknown> {
  /** Called when form submission succeeds */
  onSuccess?: (data: T) => void | Promise<void>;

  /** Called when form submission fails */
  onError?: (error: string, fieldErrors?: FieldErrors) => void;

  /** Called before form submission starts */
  onSubmit?: () => void;

  /** Initial field errors (e.g., from server-side validation) */
  initialFieldErrors?: FieldErrors;

  /** Initial global error message */
  initialError?: string | null;

  /** Auto-reset form state after successful submission */
  resetOnSuccess?: boolean;
}

/** The reactive form state object */
export interface FormState<T = unknown> {
  /** Whether the form is currently submitting */
  readonly isSubmitting: boolean;

  /** Global error message (not field-specific) */
  readonly error: string | null;

  /** Field-level error messages */
  readonly fieldErrors: FieldErrors;

  /** Whether the form has been successfully submitted */
  readonly isSuccess: boolean;

  /** Start form submission (call at beginning of submit handler) */
  startSubmit: () => void;

  /** Complete submission with success */
  setSuccess: (data?: T) => void;

  /** Complete submission with error */
  setError: (message: string, fieldErrors?: FieldErrors) => void;

  /** Set field-level errors without global error */
  setFieldErrors: (errors: FieldErrors) => void;

  /** Clear a specific field error */
  clearFieldError: (field: string) => void;

  /** Reset form to initial state */
  reset: () => void;

  /**
   * SvelteKit enhance function wrapper.
   * Use with: `use:enhance={form.enhance}`
   */
  enhance: EnhanceFn;
}

/** SvelteKit enhance function signature */
type EnhanceFn = (
  form: HTMLFormElement
) => {
  destroy?: () => void;
};

/** SvelteKit ActionResult type (simplified) */
interface ActionResult {
  type: "success" | "failure" | "redirect" | "error";
  status?: number;
  data?: Record<string, unknown>;
  location?: string;
  error?: Error;
}

// ============================================================================
// Store-based implementation (Svelte 4 compatible, works with Svelte 5)
// ============================================================================

interface FormStateInternal {
  isSubmitting: boolean;
  error: string | null;
  fieldErrors: FieldErrors;
  isSuccess: boolean;
}

/**
 * Create a form state manager using Svelte stores.
 *
 * This is compatible with both Svelte 4 and Svelte 5.
 * For Svelte 5, you can use the returned stores with `$` syntax.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createFormState } from '@decodelabs/underlay/patterns';
 *
 *   const form = createFormState({
 *     onSuccess: () => goto('/success')
 *   });
 *
 *   // Access state via stores
 *   $: disabled = $form.state.isSubmitting;
 * </script>
 * ```
 */
export function createFormState<T = unknown>(
  options: FormStateOptions<T> = {}
): FormState<T> & { state: Readable<FormStateInternal> } {
  const {
    onSuccess,
    onError,
    onSubmit,
    initialFieldErrors = {},
    initialError = null,
    resetOnSuccess = false
  } = options;

  const initialState: FormStateInternal = {
    isSubmitting: false,
    error: initialError,
    fieldErrors: { ...initialFieldErrors },
    isSuccess: false
  };

  const state: Writable<FormStateInternal> = writable({ ...initialState });

  function startSubmit(): void {
    state.update((s) => ({
      ...s,
      isSubmitting: true,
      error: null,
      fieldErrors: {},
      isSuccess: false
    }));
    onSubmit?.();
  }

  function setSuccess(data?: T): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      error: null,
      fieldErrors: {},
      isSuccess: true
    }));

    if (onSuccess && data !== undefined) {
      Promise.resolve(onSuccess(data)).catch(console.error);
    } else if (onSuccess) {
      Promise.resolve(onSuccess(undefined as T)).catch(console.error);
    }

    if (resetOnSuccess) {
      // Delay reset to allow success state to be observed
      setTimeout(() => reset(), 0);
    }
  }

  function setError(message: string, fieldErrors?: FieldErrors): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      error: message,
      fieldErrors: fieldErrors ?? {},
      isSuccess: false
    }));
    onError?.(message, fieldErrors);
  }

  function setFieldErrors(errors: FieldErrors): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      fieldErrors: errors,
      isSuccess: false
    }));
  }

  function clearFieldError(field: string): void {
    state.update((s) => {
      const { [field]: _, ...rest } = s.fieldErrors;
      return { ...s, fieldErrors: rest };
    });
  }

  function reset(): void {
    state.set({ ...initialState });
  }

  /**
   * SvelteKit enhance wrapper that automatically manages form state.
   *
   * This handles the common pattern of:
   * 1. Setting loading state on submit
   * 2. Handling success/failure responses
   * 3. Extracting field errors from ActionData
   */
  function enhance(formEl: HTMLFormElement): { destroy?: () => void } {
    async function handleSubmit(event: SubmitEvent) {
      event.preventDefault();
      startSubmit();

      const formData = new FormData(formEl);
      const action = formEl.action;
      const method = formEl.method?.toUpperCase() || "POST";

      try {
        const response = await fetch(action, {
          method,
          body: formData,
          headers: {
            Accept: "application/json"
          }
        });

        // Try to parse as JSON (SvelteKit action response)
        let result: ActionResult;
        try {
          result = await response.json();
        } catch {
          // Not JSON, treat as success if 2xx
          if (response.ok) {
            setSuccess();
          } else {
            setError(`Request failed with status ${response.status}`);
          }
          return;
        }

        // Handle SvelteKit action results
        handleActionResult(result);
      } catch (err) {
        const message =
          err instanceof Error ? err.message : "An unexpected error occurred";
        setError(message);
      }
    }

    function handleActionResult(result: ActionResult): void {
      switch (result.type) {
        case "success":
          setSuccess(result.data as T);
          break;

        case "failure": {
          // Extract error message and field errors from ActionData
          const data = result.data ?? {};
          const message =
            typeof data.error === "string"
              ? data.error
              : typeof data.message === "string"
                ? data.message
                : "Validation failed";

          const fieldErrors: FieldErrors = {};
          if (
            data.fieldErrors &&
            typeof data.fieldErrors === "object" &&
            !Array.isArray(data.fieldErrors)
          ) {
            for (const [key, value] of Object.entries(
              data.fieldErrors as Record<string, unknown>
            )) {
              if (typeof value === "string") {
                fieldErrors[key] = value;
              }
            }
          }

          setError(message, fieldErrors);
          break;
        }

        case "redirect":
          // Let SvelteKit handle the redirect
          if (result.location) {
            window.location.href = result.location;
          }
          setSuccess();
          break;

        case "error":
          setError(result.error?.message ?? "An unexpected error occurred");
          break;

        default:
          // Unknown result type, treat as success if no error indicators
          setSuccess();
      }
    }

    formEl.addEventListener("submit", handleSubmit);

    return {
      destroy() {
        formEl.removeEventListener("submit", handleSubmit);
      }
    };
  }

  // Create a proxy that exposes current state values as getters
  // This allows `form.isSubmitting` syntax while keeping reactivity
  return {
    get isSubmitting() {
      return get(state).isSubmitting;
    },
    get error() {
      return get(state).error;
    },
    get fieldErrors() {
      return get(state).fieldErrors;
    },
    get isSuccess() {
      return get(state).isSuccess;
    },
    state,
    startSubmit,
    setSuccess,
    setError,
    setFieldErrors,
    clearFieldError,
    reset,
    enhance
  };
}

// ============================================================================
// Helper functions
// ============================================================================

/**
 * Check if an action result has field errors.
 *
 * Useful for extracting field errors from SvelteKit ActionData.
 */
export function hasFieldErrors(data: unknown): data is { fieldErrors: FieldErrors } {
  return (
    typeof data === "object" &&
    data !== null &&
    "fieldErrors" in data &&
    typeof (data as Record<string, unknown>).fieldErrors === "object"
  );
}

/**
 * Extract an error message from various error formats.
 */
export function extractErrorMessage(
  error: unknown,
  fallback: string = "An error occurred"
): string {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;

  if (typeof error === "object" && error !== null) {
    const obj = error as Record<string, unknown>;
    if (typeof obj.message === "string") return obj.message;
    if (typeof obj.error === "string") return obj.error;
  }

  return fallback;
}

/**
 * Merge field errors from multiple sources.
 */
export function mergeFieldErrors(...sources: (FieldErrors | undefined | null)[]): FieldErrors {
  const result: FieldErrors = {};
  for (const source of sources) {
    if (source) {
      Object.assign(result, source);
    }
  }
  return result;
}
