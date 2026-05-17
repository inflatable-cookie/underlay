let nextSelectionScopeId = 0;
let activeSelectionScopeId: string | null = null;
const listeners = new Map<string, () => void>();

export function createListSelectionScopeId(prefix = "entity-list"): string {
  nextSelectionScopeId += 1;
  return `${prefix}-${nextSelectionScopeId}`;
}

export function registerListSelectionScope(id: string, deactivate: () => void): () => void {
  listeners.set(id, deactivate);

  return () => {
    listeners.delete(id);
    releaseListSelectionScope(id);
  };
}

export function claimListSelectionScope(id: string): void {
  activeSelectionScopeId = id;
  for (const [listenerId, deactivate] of listeners) {
    if (listenerId !== id) {
      deactivate();
    }
  }
}

export function releaseListSelectionScope(id: string): void {
  if (activeSelectionScopeId === id) {
    activeSelectionScopeId = null;
  }
}
