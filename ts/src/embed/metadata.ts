/**
 * Embed Metadata Lookup Service
 *
 * Provides async metadata lookup for embedded media using provider APIs
 * (primarily oEmbed). Handles errors gracefully and normalizes responses.
 */

import type { ParsedEmbed, EmbedMeta } from "./types.js";
import { defaultRegistry } from "./providers/index.js";

/**
 * Look up metadata for a parsed embed
 *
 * @param parsed - The parsed embed to look up
 * @param fetchFn - Optional custom fetch function (for server-side proxying)
 * @returns Metadata or null if lookup fails/unsupported
 */
export async function lookupMeta(
  parsed: ParsedEmbed,
  fetchFn: typeof fetch = fetch
): Promise<EmbedMeta | null> {
  const provider = defaultRegistry.get(parsed.provider);

  if (!provider?.lookupMeta) {
    return null;
  }

  try {
    return await provider.lookupMeta(parsed.id, fetchFn, parsed);
  } catch {
    return null;
  }
}

/**
 * Look up metadata by provider name and ID
 *
 * @param providerName - Provider name (youtube, vimeo, etc.)
 * @param id - Media ID
 * @param fetchFn - Optional custom fetch function
 * @returns Metadata or null if lookup fails/unsupported
 */
export async function lookupMetaById(
  providerName: string,
  id: string,
  fetchFn: typeof fetch = fetch
): Promise<EmbedMeta | null> {
  const provider = defaultRegistry.get(providerName);

  if (!provider?.lookupMeta) {
    return null;
  }

  try {
    return await provider.lookupMeta(id, fetchFn);
  } catch {
    return null;
  }
}

/**
 * Batch metadata lookup for multiple embeds
 *
 * @param parsedEmbeds - Array of parsed embeds
 * @param fetchFn - Optional custom fetch function
 * @returns Map of embed ID to metadata (null for failed/unsupported)
 */
export async function lookupMetaBatch(
  parsedEmbeds: ParsedEmbed[],
  fetchFn: typeof fetch = fetch
): Promise<Map<string, EmbedMeta | null>> {
  const results = new Map<string, EmbedMeta | null>();

  // Run lookups in parallel
  const promises = parsedEmbeds.map(async (parsed) => {
    const key = `${parsed.provider}:${parsed.id}`;
    const meta = await lookupMeta(parsed, fetchFn);
    results.set(key, meta);
  });

  await Promise.all(promises);

  return results;
}

/**
 * Result of a metadata lookup operation
 */
export interface MetaLookupResult {
  success: boolean;
  meta?: EmbedMeta;
  error?: string;
}

/**
 * Look up metadata with detailed result
 *
 * @param parsed - The parsed embed to look up
 * @param fetchFn - Optional custom fetch function
 * @returns Result object with success status and data/error
 */
export async function lookupMetaWithResult(
  parsed: ParsedEmbed,
  fetchFn: typeof fetch = fetch
): Promise<MetaLookupResult> {
  const provider = defaultRegistry.get(parsed.provider);

  if (!provider) {
    return {
      success: false,
      error: `Unknown provider: ${parsed.provider}`,
    };
  }

  if (!provider.lookupMeta) {
    return {
      success: false,
      error: `Provider "${parsed.provider}" does not support metadata lookup`,
    };
  }

  try {
    const meta = await provider.lookupMeta(parsed.id, fetchFn, parsed);

    if (meta) {
      return {
        success: true,
        meta,
      };
    } else {
      return {
        success: false,
        error: "Metadata lookup returned no results",
      };
    }
  } catch (err) {
    return {
      success: false,
      error: err instanceof Error ? err.message : "Metadata lookup failed",
    };
  }
}

/**
 * Cached metadata store for client-side use
 */
export function createMetaCache() {
  const cache = new Map<string, { meta: EmbedMeta; timestamp: number }>();
  const DEFAULT_TTL = 5 * 60 * 1000; // 5 minutes

  return {
    /**
     * Get cached metadata
     */
    get(provider: string, id: string): EmbedMeta | null {
      const key = `${provider}:${id}`;
      const entry = cache.get(key);

      if (!entry) return null;

      // Check if expired
      if (Date.now() - entry.timestamp > DEFAULT_TTL) {
        cache.delete(key);
        return null;
      }

      return entry.meta;
    },

    /**
     * Store metadata in cache
     */
    set(provider: string, id: string, meta: EmbedMeta): void {
      const key = `${provider}:${id}`;
      cache.set(key, { meta, timestamp: Date.now() });
    },

    /**
     * Check if metadata is cached
     */
    has(provider: string, id: string): boolean {
      return this.get(provider, id) !== null;
    },

    /**
     * Clear all cached metadata
     */
    clear(): void {
      cache.clear();
    },

    /**
     * Get or fetch metadata (with caching)
     */
    async getOrFetch(
      parsed: ParsedEmbed,
      fetchFn: typeof fetch = fetch
    ): Promise<EmbedMeta | null> {
      // Check cache first
      const cached = this.get(parsed.provider, parsed.id);
      if (cached) return cached;

      // Fetch and cache
      const meta = await lookupMeta(parsed, fetchFn);
      if (meta) {
        this.set(parsed.provider, parsed.id, meta);
      }

      return meta;
    },
  };
}

/**
 * Default metadata cache instance
 */
export const metaCache = createMetaCache();

/**
 * Format duration in seconds to human-readable string
 */
export function formatDuration(seconds: number | undefined): string | null {
  if (seconds === undefined || seconds === null) return null;

  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours}:${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  }

  return `${mins}:${secs.toString().padStart(2, "0")}`;
}

/**
 * Parse duration string (HH:MM:SS or MM:SS) to seconds
 */
export function parseDuration(duration: string): number | null {
  const parts = duration.split(":").map((p) => parseInt(p, 10));

  if (parts.some(isNaN)) return null;

  if (parts.length === 3) {
    // HH:MM:SS
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  } else if (parts.length === 2) {
    // MM:SS
    return parts[0] * 60 + parts[1];
  }

  return null;
}
