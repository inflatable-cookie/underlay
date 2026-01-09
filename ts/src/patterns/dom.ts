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
  const requestSubmit = (element as any)?.requestSubmit;

  if (typeof requestSubmit === "function") {
    requestSubmit.call(element);
  }
}
