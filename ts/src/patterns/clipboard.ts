import type { ToastStore } from "./toasts";

export async function copyTextToClipboard(text: string): Promise<void> {
  const clipboard = globalThis?.navigator?.clipboard;
  if (!clipboard) {
    throw new Error("Clipboard API unavailable");
  }

  await clipboard.writeText(text);
}

export async function copyToClipboard(
  toastStore: ToastStore,
  text: string,
  successMessage: string,
  failureMessage: string = "Failed to copy"
): Promise<void> {
  try {
    await copyTextToClipboard(text);
    toastStore.push({ variant: "success", message: successMessage });
  } catch (e) {
    // eslint-disable-next-line no-console
    console.error("Failed to copy to clipboard", e);
    toastStore.push({ variant: "error", message: failureMessage });
    throw e;
  }
}
