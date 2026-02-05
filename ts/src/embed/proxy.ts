/**
 * Server-side Embed Metadata Proxy
 *
 * Utilities for looking up embed metadata via a server proxy to bypass
 * CORS restrictions for providers that block browser-side API calls.
 */

import type { ParsedEmbed, EmbedMeta } from "./types.js";

/**
 * Request to look up embed metadata via server proxy.
 */
export interface EmbedMetaProxyRequest {
  /** Provider name: "audioboom", "youtube", "vimeo" */
  provider: string;
  /** Media ID */
  id: string;
  /** Embed type for providers with multiple formats (e.g., "single" or "playlist") */
  embedType?: string;
}

/**
 * Response from server-side embed metadata lookup.
 */
export interface EmbedMetaProxyResponse {
  /** Whether lookup was successful */
  success: boolean;
  /** Media title */
  title?: string | null;
  /** Media description */
  description?: string | null;
  /** Duration in seconds */
  duration?: number | null;
  /** Thumbnail URL */
  thumbnailUrl?: string | null;
  /** Author/channel name */
  authorName?: string | null;
  /** Error message if lookup failed */
  error?: string | null;
}

/**
 * Providers that require server-side proxy for metadata lookup due to CORS.
 */
export const CORS_BLOCKED_PROVIDERS = ["audioboom"] as const;

/**
 * Check if a provider requires server-side proxy for metadata lookup.
 */
export function requiresServerProxy(provider: string): boolean {
  return CORS_BLOCKED_PROVIDERS.includes(provider as (typeof CORS_BLOCKED_PROVIDERS)[number]);
}

/**
 * Configuration for creating a proxy-aware metadata lookup function.
 */
export interface ProxyLookupConfig {
  /**
   * Function to call the server proxy endpoint.
   * Should POST to your server's embed metadata endpoint.
   */
  proxyFn: (request: EmbedMetaProxyRequest) => Promise<EmbedMetaProxyResponse>;

  /**
   * Optional: Override which providers use the proxy.
   * Defaults to CORS_BLOCKED_PROVIDERS.
   */
  proxyProviders?: string[];
}

/**
 * Convert a proxy response to the standard EmbedMeta format.
 */
export function proxyResponseToMeta(response: EmbedMetaProxyResponse): EmbedMeta | null {
  if (!response.success) {
    return null;
  }

  return {
    title: response.title ?? undefined,
    description: response.description ?? undefined,
    duration: response.duration ?? undefined,
    thumbnailUrl: response.thumbnailUrl ?? undefined,
    authorName: response.authorName ?? undefined,
  };
}

/**
 * Create a proxy-aware metadata lookup function.
 *
 * This wraps the standard lookupMeta to automatically route requests
 * through your server proxy for CORS-blocked providers.
 *
 * @example
 * ```typescript
 * import { createProxyAwareLookup, type EmbedMetaProxyRequest } from "@decodelabs/underlay/embed";
 *
 * const lookupMeta = createProxyAwareLookup({
 *   proxyFn: async (req) => {
 *     const response = await fetch("/api/embed/metadata", {
 *       method: "POST",
 *       headers: { "Content-Type": "application/json" },
 *       body: JSON.stringify(req),
 *     });
 *     return response.json();
 *   },
 * });
 *
 * // Now use lookupMeta - it will automatically use the proxy for Audioboom
 * const meta = await lookupMeta(parsedEmbed);
 * ```
 */
export function createProxyAwareLookup(
  config: ProxyLookupConfig,
  baseLookupMeta: (parsed: ParsedEmbed, fetchFn?: typeof fetch) => Promise<EmbedMeta | null>
): (parsed: ParsedEmbed, fetchFn?: typeof fetch) => Promise<EmbedMeta | null> {
  const proxyProviders = config.proxyProviders ?? [...CORS_BLOCKED_PROVIDERS];

  return async (parsed: ParsedEmbed, fetchFn?: typeof fetch): Promise<EmbedMeta | null> => {
    // Use proxy for CORS-blocked providers
    if (proxyProviders.includes(parsed.provider)) {
      const request: EmbedMetaProxyRequest = {
        provider: parsed.provider,
        id: parsed.id,
        embedType: parsed.embedType,
      };

      try {
        const response = await config.proxyFn(request);
        return proxyResponseToMeta(response);
      } catch {
        return null;
      }
    }

    // Use direct lookup for other providers
    return baseLookupMeta(parsed, fetchFn);
  };
}
