import { get, writable, type Writable } from "svelte/store";
import {
  captureFormDraft,
  resolveDraftStorage,
  restoreFormDraft,
} from "./draft";
import { createFormEnhance } from "./enhance";
import type {
  FieldErrors,
  FormDraft,
  FormStateInternal,
  FormStateOptions,
  FormStateWithStore,
} from "./types";

export function createFormState<T = unknown>(
  options: FormStateOptions<T> = {},
): FormStateWithStore<T> {
  const {
    onSuccess,
    onError,
    onSubmit,
    initialFieldErrors = {},
    initialError = null,
    resetOnSuccess = false,
    autoSave,
  } = options;

  const initialState: FormStateInternal = {
    isSubmitting: false,
    error: initialError,
    fieldErrors: { ...initialFieldErrors },
    isSuccess: false,
  };

  const state: Writable<FormStateInternal> = writable({ ...initialState });
  const autoSaveStorage = resolveDraftStorage(autoSave);
  const autoSaveStorageOptions =
    autoSave === undefined
      ? undefined
      : {
          ttl: autoSave.ttl,
          expiresAt: autoSave.expiresAt,
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

    autoSaveStorage.set(
      autoSave.key,
      captureFormDraft(formEl),
      autoSaveStorageOptions,
    );
  }

  function scheduleDraftWrite(formEl: HTMLFormElement): void {
    if (!autoSaveStorage || !autoSave) {
      return;
    }

    clearDraftTimer();
    draftTimer = setTimeout(
      () => {
        writeDraft(formEl);
        draftTimer = null;
      },
      Math.max(0, autoSaveDebounce),
    );
  }

  function restoreDraft(formEl: HTMLFormElement): void {
    if (!autoSaveStorage || !autoSave || autoSave.restoreOnMount === false) {
      return;
    }

    const savedDraft = autoSaveStorage.get<FormDraft>(
      autoSave.key,
      {},
      autoSaveStorageOptions,
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
      isSuccess: false,
    }));
    onSubmit?.();
  }

  function setSuccess(data?: T): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      error: null,
      fieldErrors: {},
      isSuccess: true,
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
      setTimeout(() => reset(), 0);
    }
  }

  function setError(message: string, fieldErrors?: FieldErrors): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      error: message,
      fieldErrors: fieldErrors ?? {},
      isSuccess: false,
    }));
    onError?.(message, fieldErrors);
  }

  function setFieldErrors(errors: FieldErrors): void {
    state.update((s) => ({
      ...s,
      isSubmitting: false,
      fieldErrors: errors,
      isSuccess: false,
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

  const enhance = createFormEnhance<T>({
    autoSave,
    hasAutoSaveStorage: Boolean(autoSaveStorage),
    clearDraftTimer,
    writeDraft,
    scheduleDraftWrite,
    restoreDraft,
    startSubmit,
    setSuccess,
    setError,
  });

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
    enhance,
  };
}
