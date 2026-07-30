import {
  resolveActionFailureResult,
  type ActionResult,
} from "../forms-action-result";
import { resolveRedirectTo } from "../../client/route-protection";
import type { FieldErrors, FormAutoSaveOptions } from "./types";

export type CreateFormEnhanceInput<T> = {
  autoSave?: FormAutoSaveOptions;
  hasAutoSaveStorage: boolean;
  clearDraftTimer: () => void;
  writeDraft: (formEl: HTMLFormElement) => void;
  scheduleDraftWrite: (formEl: HTMLFormElement) => void;
  restoreDraft: (formEl: HTMLFormElement) => void;
  startSubmit: () => void;
  setSuccess: (data?: T) => void;
  setError: (message: string, fieldErrors?: FieldErrors) => void;
};

export function createFormEnhance<T>(
  input: CreateFormEnhanceInput<T>,
): (formEl: HTMLFormElement) => { destroy?: () => void } {
  return function enhance(formEl: HTMLFormElement): { destroy?: () => void } {
    input.restoreDraft(formEl);

    function handleDraftUpdate() {
      input.scheduleDraftWrite(formEl);
    }

    async function handleSubmit(event: SubmitEvent) {
      event.preventDefault();
      input.clearDraftTimer();
      input.writeDraft(formEl);
      input.startSubmit();

      const formData = new FormData(formEl);
      const action = formEl.action;
      const method = formEl.method?.toUpperCase() || "POST";

      try {
        const response = await fetch(action, {
          method,
          body: formData,
          headers: {
            Accept: "application/json",
          },
        });

        let result: ActionResult;
        try {
          result = await response.json();
        } catch {
          if (response.ok) {
            input.setSuccess();
          } else {
            input.setError(`Request failed with status ${response.status}`);
          }
          return;
        }

        handleActionResult(result);
      } catch (err) {
        const message =
          err instanceof Error ? err.message : "An unexpected error occurred";
        input.setError(message);
      }
    }

    function handleActionResult(result: ActionResult): void {
      switch (result.type) {
        case "success":
          input.setSuccess(result.data as T);
          break;

        case "failure": {
          const { message, fieldErrors } = resolveActionFailureResult(
            result.data,
          );
          input.setError(message, fieldErrors);
          break;
        }

        case "redirect":
          if (result.location) {
            window.location.href = resolveRedirectTo(result.location);
          }
          input.setSuccess();
          break;

        case "error":
          input.setError(
            result.error?.message ?? "An unexpected error occurred",
          );
          break;

        default:
          input.setSuccess();
      }
    }

    if (input.hasAutoSaveStorage && input.autoSave) {
      formEl.addEventListener("input", handleDraftUpdate);
      formEl.addEventListener("change", handleDraftUpdate);
    }

    formEl.addEventListener("submit", handleSubmit);

    return {
      destroy() {
        input.clearDraftTimer();
        if (input.hasAutoSaveStorage && input.autoSave) {
          formEl.removeEventListener("input", handleDraftUpdate);
          formEl.removeEventListener("change", handleDraftUpdate);
        }
        formEl.removeEventListener("submit", handleSubmit);
      },
    };
  };
}
