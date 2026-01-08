import { createClient as createStemClient } from "@stem";
import { env } from "$env/dynamic/public";

const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createBloomClient(
  fetchFn: typeof fetch,
  authToken: string | null | undefined
) {
  return createStemClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken ?? null
  });
}
