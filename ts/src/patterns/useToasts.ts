import { getContext } from "svelte";

import {
  UNDERLAY_TOASTS_CONTEXT_KEY,
  type ToastStore
} from "./toasts";

export function useToasts(): ToastStore {
  return getContext<ToastStore>(UNDERLAY_TOASTS_CONTEXT_KEY);
}
