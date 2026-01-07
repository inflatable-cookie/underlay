let cached: any | null = null;

export async function lazyLoadEasyMde(): Promise<any> {
  if (cached) {
    return cached;
  }

  const mod: any = await import("easymde");
  cached = mod?.default ?? mod;
  return cached;
}
