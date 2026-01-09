let cached: any | null = null;
let inflight: Promise<any> | null = null;

export async function lazyLoadEasyMde(): Promise<any> {
  if (cached) {
    return cached;
  }

  if (inflight) {
    return inflight;
  }

  inflight = import("easymde").then((mod: any) => {
    cached = mod?.default ?? mod;
    return cached;
  });

  return inflight;
}

export async function prefetchEasyMde(): Promise<void> {
  const isBrowser = typeof window !== "undefined";
  if (!isBrowser) return;

  try {
    await lazyLoadEasyMde();
  } catch {
    // Best-effort prefetch.
  }
}
