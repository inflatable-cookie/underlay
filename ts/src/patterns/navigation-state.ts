import { storage } from "./storage";

const DEFAULT_STATE_STORAGE_KEY = "underlay:nav-state";

type PageStateStore = Record<string, Record<string, unknown>>;

function getPageStateStore(): PageStateStore {
  return storage.session.get<PageStateStore>(DEFAULT_STATE_STORAGE_KEY, {});
}

function savePageStateStore(store: PageStateStore): void {
  storage.session.set(DEFAULT_STATE_STORAGE_KEY, store);
}

export function storePageState(pathname: string, state: Record<string, unknown>): void {
  const store = getPageStateStore();
  store[pathname] = state;
  savePageStateStore(store);
}

export function retrievePageState<T extends Record<string, unknown>>(
  pathname: string
): T | null {
  const store = getPageStateStore();
  return (store[pathname] as T) ?? null;
}

export function consumePageState<T extends Record<string, unknown>>(
  pathname?: string
): T | null {
  const targetPath = pathname ?? (typeof window !== "undefined" ? window.location.pathname : null);
  if (!targetPath) return null;

  const store = getPageStateStore();
  const state = store[targetPath] as T | undefined;

  if (state) {
    delete store[targetPath];
    savePageStateStore(store);
  }

  return state ?? null;
}

export function clearPageStates(): void {
  storage.session.remove(DEFAULT_STATE_STORAGE_KEY);
}
