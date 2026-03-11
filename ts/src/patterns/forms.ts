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
import {
  resolveActionFailureResult,
  type ActionResult
} from "./forms-action-result";
import { storage, type StorageOptions, type StorageWrapper } from "./storage";

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

  /** Optional draft persistence for form values */
  autoSave?: FormAutoSaveOptions;
}

/** Draft persistence configuration for `createFormState` */
export interface FormAutoSaveOptions
  extends Pick<StorageOptions, "ttl" | "expiresAt"> {
  /** Storage key used for the saved draft */
  key: string;

  /** Draft storage location (`session` by default) */
  storage?: "local" | "session" | StorageWrapper;

  /** Debounce delay for writes in milliseconds (`600` by default) */
  debounce?: number;

  /** Whether to restore the draft when `enhance` attaches (`true` by default) */
  restoreOnMount?: boolean;

  /** Whether to clear the draft after `setSuccess()` (`true` by default) */
  clearOnSuccess?: boolean;
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

  /** Remove the persisted draft when auto-save is enabled */
  clearDraft: () => void;

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

// ============================================================================
// Store-based implementation (Svelte 4 compatible, works with Svelte 5)
// ============================================================================

interface FormStateInternal {
  isSubmitting: boolean;
  error: string | null;
  fieldErrors: FieldErrors;
  isSuccess: boolean;
}

type DraftValue =
  | { kind: "single"; value: string }
  | { kind: "boolean"; checked: boolean }
  | { kind: "multi"; values: string[] };

type FormDraft = Record<string, DraftValue>;

interface DraftControlBase {
  name: string;
  type?: string;
  value?: string;
  checked?: boolean;
  disabled?: boolean;
  multiple?: boolean;
  tagName?: string;
  options?: ArrayLike<{ value: string; selected: boolean }>;
  dispatchEvent?: (event: Event) => boolean;
}

type DraftControl = DraftControlBase & {
  type?: string;
  value: string;
};

function isDraftControl(control: unknown): control is DraftControl {
  if (typeof control !== "object" || control === null) {
    return false;
  }

  const entry = control as DraftControlBase;
  const tagName = getControlTagName(entry);
  return (
    typeof entry.name === "string" &&
    (tagName === "input" || tagName === "select" || tagName === "textarea")
  );
}

function getControlTagName(control: DraftControlBase): string {
  return typeof control.tagName === "string" ? control.tagName.toLowerCase() : "";
}

function getControlType(control: DraftControlBase): string {
  return typeof control.type === "string" ? control.type.toLowerCase() : "";
}

function isFileInput(control: DraftControlBase): boolean {
  return getControlTagName(control) === "input" && getControlType(control) === "file";
}

function isCheckboxControl(control: DraftControlBase): boolean {
  return getControlTagName(control) === "input" && getControlType(control) === "checkbox";
}

function isRadioControl(control: DraftControlBase): boolean {
  return getControlTagName(control) === "input" && getControlType(control) === "radio";
}

function isMultiSelectControl(control: DraftControlBase): boolean {
  return getControlTagName(control) === "select" && control.multiple === true;
}

function createDraftEventsFor(control: DraftControlBase): Event[] {
  const type = getControlType(control);
  if (type === "checkbox" || type === "radio" || getControlTagName(control) === "select") {
    return [new Event("change", { bubbles: true })];
  }

  return [
    new Event("input", { bubbles: true }),
    new Event("change", { bubbles: true })
  ];
}

function dispatchDraftEvents(control: DraftControlBase): void {
  if (!control.dispatchEvent) {
    return;
  }

  for (const event of createDraftEventsFor(control)) {
    control.dispatchEvent(event);
  }
}

function collectDraftControls(formEl: HTMLFormElement): Map<string, DraftControl[]> {
  const controls = new Map<string, DraftControl[]>();

  for (const entry of Array.from(formEl.elements ?? [])) {
    if (!isDraftControl(entry)) {
      continue;
    }

    if (!entry.name || isFileInput(entry)) {
      continue;
    }

    const group = controls.get(entry.name) ?? [];
    group.push(entry);
    controls.set(entry.name, group);
  }

  return controls;
}

function captureFormDraft(formEl: HTMLFormElement): FormDraft {
  const controlsByName = collectDraftControls(formEl);
  const draft: FormDraft = {};

  for (const [name, controls] of controlsByName.entries()) {
    const first = controls[0];

    if (isCheckboxControl(first)) {
      if (controls.length === 1) {
        draft[name] = {
          kind: "boolean",
          checked: Boolean(first.checked)
        };
        continue;
      }

      draft[name] = {
        kind: "multi",
        values: controls.filter((control) => control.checked).map((control) => control.value)
      };
      continue;
    }

    if (isRadioControl(first)) {
      const selected = controls.find((control) => control.checked);
      if (selected) {
        draft[name] = {
          kind: "single",
          value: selected.value
        };
      }
      continue;
    }

    if (isMultiSelectControl(first)) {
      draft[name] = {
        kind: "multi",
        values: Array.from(first.options ?? [])
          .filter((option) => option.selected)
          .map((option) => option.value)
      };
      continue;
    }

    draft[name] = {
      kind: "single",
      value: first.value ?? ""
    };
  }

  return draft;
}

function restoreFormDraft(formEl: HTMLFormElement, draft: FormDraft): void {
  const controlsByName = collectDraftControls(formEl);

  for (const [name, entry] of Object.entries(draft)) {
    const controls = controlsByName.get(name);
    if (!controls || controls.length === 0) {
      continue;
    }

    const first = controls[0];

    switch (entry.kind) {
      case "boolean": {
        if (!isCheckboxControl(first) || controls.length !== 1) {
          break;
        }

        if (Boolean(first.checked) !== entry.checked) {
          first.checked = entry.checked;
          dispatchDraftEvents(first);
        }
        break;
      }

      case "multi": {
        if (isMultiSelectControl(first)) {
          const nextValues = new Set(entry.values);
          let changed = false;

          for (const option of Array.from(first.options ?? [])) {
            const shouldSelect = nextValues.has(option.value);
            if (option.selected !== shouldSelect) {
              option.selected = shouldSelect;
              changed = true;
            }
          }

          if (changed) {
            dispatchDraftEvents(first);
          }
          break;
        }

        if (isCheckboxControl(first)) {
          const nextValues = new Set(entry.values);
          let changed = false;

          for (const control of controls) {
            const shouldCheck = nextValues.has(control.value);
            if (Boolean(control.checked) !== shouldCheck) {
              control.checked = shouldCheck;
              changed = true;
              dispatchDraftEvents(control);
            }
          }

          if (changed) {
            dispatchDraftEvents(first);
          }
        }
        break;
      }

      case "single": {
        if (isRadioControl(first)) {
          for (const control of controls) {
            const shouldCheck = control.value === entry.value;
            if (Boolean(control.checked) !== shouldCheck) {
              control.checked = shouldCheck;
              dispatchDraftEvents(control);
            }
          }
          break;
        }

        if ((first.value ?? "") !== entry.value) {
          first.value = entry.value;
          dispatchDraftEvents(first);
        }
        break;
      }
    }
  }
}

function resolveDraftStorage(
  autoSave?: FormAutoSaveOptions
): StorageWrapper | null {
  if (!autoSave) {
    return null;
  }

  if (!autoSave.storage || autoSave.storage === "session") {
    return storage.session;
  }

  if (autoSave.storage === "local") {
    return storage.local;
  }

  return autoSave.storage;
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
    resetOnSuccess = false,
    autoSave
  } = options;

  const initialState: FormStateInternal = {
    isSubmitting: false,
    error: initialError,
    fieldErrors: { ...initialFieldErrors },
    isSuccess: false
  };

  const state: Writable<FormStateInternal> = writable({ ...initialState });
  const autoSaveStorage = resolveDraftStorage(autoSave);
  const autoSaveStorageOptions =
    autoSave === undefined
      ? undefined
      : {
          ttl: autoSave.ttl,
          expiresAt: autoSave.expiresAt
        };
  const autoSaveDebounce = autoSave?.debounce ?? 600;
  let draftTimer: ReturnType<typeof setTimeout> | null = null;

  function clearDraftTimer(): void {
    if (draftTimer !== null) {
      clearTimeout(draftTimer);
      draftTimer = null;
    }
  }

  function clearDraft(): void {
    clearDraftTimer();
    if (autoSaveStorage && autoSave) {
      autoSaveStorage.remove(autoSave.key);
    }
  }

  function writeDraft(formEl: HTMLFormElement): void {
    if (!autoSaveStorage || !autoSave) {
      return;
    }

    autoSaveStorage.set(autoSave.key, captureFormDraft(formEl), autoSaveStorageOptions);
  }

  function scheduleDraftWrite(formEl: HTMLFormElement): void {
    if (!autoSaveStorage || !autoSave) {
      return;
    }

    clearDraftTimer();
    draftTimer = setTimeout(() => {
      writeDraft(formEl);
      draftTimer = null;
    }, Math.max(0, autoSaveDebounce));
  }

  function restoreDraft(formEl: HTMLFormElement): void {
    if (!autoSaveStorage || !autoSave || autoSave.restoreOnMount === false) {
      return;
    }

    const savedDraft = autoSaveStorage.get<FormDraft>(
      autoSave.key,
      {},
      autoSaveStorageOptions
    );

    if (Object.keys(savedDraft).length === 0) {
      return;
    }

    restoreFormDraft(formEl, savedDraft);
  }

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

    if (autoSave?.clearOnSuccess !== false) {
      clearDraft();
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
    restoreDraft(formEl);

    function handleDraftUpdate() {
      scheduleDraftWrite(formEl);
    }

    async function handleSubmit(event: SubmitEvent) {
      event.preventDefault();
      clearDraftTimer();
      writeDraft(formEl);
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
          const { message, fieldErrors } = resolveActionFailureResult(result.data);
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

    if (autoSaveStorage && autoSave) {
      formEl.addEventListener("input", handleDraftUpdate);
      formEl.addEventListener("change", handleDraftUpdate);
    }

    formEl.addEventListener("submit", handleSubmit);

    return {
      destroy() {
        clearDraftTimer();
        if (autoSaveStorage && autoSave) {
          formEl.removeEventListener("input", handleDraftUpdate);
          formEl.removeEventListener("change", handleDraftUpdate);
        }
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
    clearDraft,
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
