let idCounter = 0;

export function createStableId(prefix: string): string {
  // Deterministic across SSR + hydration, as long as component construction order matches.
  idCounter += 1;
  return `${prefix}-${idCounter}`;
}

export function requestSubmitById(elementId: string): void {
  const doc = globalThis?.document;
  if (!doc) return;

  const element = doc.getElementById(elementId);
  if (!element) return;

  // requestSubmit is only declared on HTMLFormElement, but callers legitimately
  // pass any element that implements it. Narrow through the `in` operator so
  // the element keeps its type instead of being erased to `any`.
  if (!("requestSubmit" in element)) return;

  const { requestSubmit } = element;

  if (typeof requestSubmit === "function") {
    requestSubmit.call(element);
  }
}

/**
 * Submit a form with a specific intent value.
 *
 * This is useful for forms that use intent-based actions (e.g., "save", "delete", "save-close").
 * It sets a hidden input's value before submitting the form programmatically.
 *
 * @param intent - The intent value to set (e.g., "delete", "save", "save-close")
 * @param formSelector - CSS selector for the form (default: "form")
 * @param intentFieldName - Name of the hidden intent input field (default: "intent")
 *
 * @example
 * ```typescript
 * // In a delete button handler
 * function handleDelete() {
 *   submitFormWithIntent("delete");
 * }
 *
 * // With custom form selector
 * function handleArchive() {
 *   submitFormWithIntent("archive", "#main-form");
 * }
 * ```
 */
export function submitFormWithIntent(
  intent: string,
  formSelector = "form",
  intentFieldName = "intent"
): void {
  const doc = globalThis?.document;
  if (!doc) return;

  const formElement = doc.querySelector(formSelector) as HTMLFormElement | null;
  if (!formElement) return;

  const intentInput = formElement.querySelector(
    `input[name="${intentFieldName}"]`
  ) as HTMLInputElement | null;

  if (intentInput) {
    intentInput.value = intent;
  }

  formElement.requestSubmit();
}
