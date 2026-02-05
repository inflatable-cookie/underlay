/**
 * Vimeo Embed Provider
 *
 * Supports parsing and embedding Vimeo videos.
 *
 * URL formats supported:
 * - https://vimeo.com/{id}
 * - https://player.vimeo.com/video/{id}
 * - https://vimeo.com/channels/{channel}/{id}
 * - https://vimeo.com/groups/{group}/videos/{id}
 * - https://vimeo.com/{id}?h={hash} (private videos with hash)
 */

import type { EmbedProvider, ParsedEmbed, EmbedOptions, EmbedMeta } from "../types.js";
import { normalizeUrl, parseQueryString, buildQueryString } from "../utils.js";

const VIMEO_PATTERNS = ["vimeo.com", "player.vimeo.com"];

/**
 * Extract Vimeo video ID from a URL
 */
function extractVimeoId(url: string): {
  id: string | null;
  hash: string | null;
  queryParams: Record<string, string>;
} {
  const normalized = normalizeUrl(url);
  let id: string | null = null;
  let hash: string | null = null;
  let queryParams: Record<string, string> = {};

  try {
    const urlObj = new URL(normalized);
    const pathname = urlObj.pathname;
    queryParams = parseQueryString(urlObj.search);

    // Check for hash in query params (private videos)
    hash = queryParams.h || null;

    // player.vimeo.com/video/{id}
    if (pathname.startsWith("/video/")) {
      const pathId = pathname.split("/")[2]?.split("?")[0];
      if (pathId && /^\d+$/.test(pathId)) {
        id = pathId;
      }
    }
    // vimeo.com/channels/{channel}/{id}
    else if (pathname.startsWith("/channels/")) {
      const parts = pathname.split("/");
      const pathId = parts[3]?.split("?")[0];
      if (pathId && /^\d+$/.test(pathId)) {
        id = pathId;
      }
    }
    // vimeo.com/groups/{group}/videos/{id}
    else if (pathname.includes("/videos/")) {
      const parts = pathname.split("/videos/");
      const pathId = parts[1]?.split("/")[0]?.split("?")[0];
      if (pathId && /^\d+$/.test(pathId)) {
        id = pathId;
      }
    }
    // vimeo.com/{id}
    else {
      const pathId = pathname.slice(1).split("/")[0]?.split("?")[0];
      if (pathId && /^\d+$/.test(pathId)) {
        id = pathId;
      }
    }
  } catch {
    // URL parsing failed
  }

  return { id, hash, queryParams };
}

/**
 * Check if a URL is a Vimeo URL
 */
function isVimeoUrl(url: string): boolean {
  const lowerUrl = url.toLowerCase();
  return VIMEO_PATTERNS.some((pattern) => lowerUrl.includes(pattern));
}

export const vimeo: EmbedProvider = {
  name: "vimeo",

  parse(input: string): ParsedEmbed | null {
    const trimmed = input.trim();

    // Check if it's a Vimeo URL
    if (!isVimeoUrl(trimmed)) {
      // Could be a raw ID - Vimeo IDs are numeric
      if (/^\d{6,}$/.test(trimmed)) {
        return {
          provider: "vimeo",
          id: trimmed,
        };
      }
      return null;
    }

    const { id, hash, queryParams } = extractVimeoId(trimmed);

    if (!id) {
      return null;
    }

    const result: ParsedEmbed = {
      provider: "vimeo",
      id,
      originalUrl: trimmed,
      queryParams,
    };

    // Store hash for private videos
    if (hash) {
      result.queryParams = {
        ...result.queryParams,
        h: hash,
      };
    }

    return result;
  },

  getEmbedUrl(id: string, options?: EmbedOptions): string {
    const params: Record<string, string | number | boolean | undefined> = {};

    if (options?.autoplay) {
      params.autoplay = 1;
    }

    // Vimeo uses #t= for start time in URL, but uses different params for embed
    // The parameter varies by context - checking docs shows it's complex
    // For iframe embeds, we can append #t={seconds}s to the URL

    if (options?.enableApi) {
      params.api = 1;
    }

    if (options?.origin) {
      params.origin = options.origin;
    }

    // Build base URL
    let url = `https://player.vimeo.com/video/${id}`;

    const queryString = buildQueryString(params);
    url += queryString;

    // Add time fragment if start time is specified
    if (options?.startTime !== undefined) {
      url += `#t=${options.startTime}s`;
    }

    return url;
  },

  // Vimeo doesn't have predictable thumbnail URLs like YouTube
  // Must use oEmbed API to get thumbnails
  getThumbnailUrl: undefined,

  async lookupMeta(
    id: string,
    fetchFn: typeof fetch = fetch
  ): Promise<EmbedMeta | null> {
    try {
      // Use oEmbed API
      const videoUrl = `https://vimeo.com/${id}`;
      const oembedUrl = `https://vimeo.com/api/oembed.json?url=${encodeURIComponent(videoUrl)}`;

      const response = await fetchFn(oembedUrl, {
        headers: {
          // Vimeo requires a Referer header for oEmbed
          Referer: "https://vimeo.com",
        },
      });

      if (!response.ok) {
        return null;
      }

      const data = await response.json();

      const meta: EmbedMeta = {
        title: data.title,
        description: data.description,
        duration: data.duration, // Vimeo oEmbed includes duration!
        authorName: data.author_name,
        authorUrl: data.author_url,
        thumbnailUrl: data.thumbnail_url,
        thumbnailWidth: data.thumbnail_width,
        thumbnailHeight: data.thumbnail_height,
        uploadDate: data.upload_date,
      };

      return meta;
    } catch {
      return null;
    }
  },
};
