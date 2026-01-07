import { writable, type Readable } from "svelte/store";

export type ToastVariant = "info" | "success" | "error";

export type Toast = {
  id: string;
  variant: ToastVariant;
  title?: string;
  message: string;
  createdAtMs: number;
};

export type ToastInput = {
  id?: string;
  variant?: ToastVariant;
  title?: string;
  message: string;
};

export type ToastStore = {
  toasts: Readable<Toast[]>;
  push: (toast: ToastInput) => string;
  dismiss: (id: string) => void;
  clear: () => void;
};

export const UNDERLAY_TOASTS_CONTEXT_KEY = Symbol("underlayToasts");

function generateToastId() {
  // Avoid crypto/randomUUID to keep SSR safe.
  return `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
}

export function createToastStore(): ToastStore {
  const toasts = writable<Toast[]>([]);

  function push(input: ToastInput): string {
    const id = input.id ?? generateToastId();
    const next: Toast = {
      id,
      variant: input.variant ?? "info",
      title: input.title,
      message: input.message,
      createdAtMs: Date.now()
    };

    toasts.update((existing) => [...existing, next]);
    return id;
  }

  function dismiss(id: string) {
    toasts.update((existing) => existing.filter((toast) => toast.id !== id));
  }

  function clear() {
    toasts.set([]);
  }

  return { toasts, push, dismiss, clear };
}
