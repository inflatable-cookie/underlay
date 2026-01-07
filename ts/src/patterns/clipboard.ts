export async function copyTextToClipboard(text: string): Promise<void> {
  const clipboard = globalThis?.navigator?.clipboard;
  if (!clipboard) {
    throw new Error("Clipboard API unavailable");
  }

  await clipboard.writeText(text);
}
