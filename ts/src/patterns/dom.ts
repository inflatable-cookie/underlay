export function requestSubmitById(elementId: string): void {
  const doc = globalThis?.document;
  if (!doc) return;

  const element = doc.getElementById(elementId);
  const requestSubmit = (element as any)?.requestSubmit;

  if (typeof requestSubmit === "function") {
    requestSubmit.call(element);
  }
}
