/**
 * YouTube Embed Provider
 *
 * Supports parsing and embedding YouTube videos.
 *
 * URL formats supported:
 * - https://www.youtube.com/watch?v={id}
 * - https://youtu.be/{id}
 * - https://www.youtube.com/embed/{id}
 * - https://www.youtube-nocookie.com/embed/{id}
 * - https://www.youtube.com/v/{id}
 */

import type { EmbedProvider, ParsedEmbed, EmbedOptions, EmbedMeta } from "../types.js";
import {
  normalizeUrl,
  parseQueryString,
  buildQueryString,
  timestampToSeconds,
} from "../utils.js";

const YOUTUBE_PATTERNS = [
  "youtube.com",
  "youtu.be",
  "youtube-nocookie.com",
];

/**
 * Extract YouTube video ID from a URL
 */
function extractYouTubeId(url: string): {
  id: string | null;
  queryParams: Record<string, string>;
} {
  const normalized = normalizeUrl(url);
  let id: string | null = null;
  let queryParams: Record<string, string> = {};

  try {
    const urlObj = new URL(normalized);
    const hostname = urlObj.hostname.toLowerCase();
    const pathname = urlObj.pathname;
    queryParams = parseQueryString(urlObj.search);

    // youtu.be/{id}
    if (hostname === "youtu.be" || hostname === "www.youtu.be") {
      const pathId = pathname.slice(1).split("/")[0].split("?")[0];
      if (pathId && pathId.length >= 11) {
        id = pathId;
      }
    }
    // youtube.com/watch?v={id}
    else if (queryParams.v) {
      id = queryParams.v;
    }
    // youtube.com/embed/{id} or youtube.com/v/{id}
    else if (pathname.startsWith("/embed/") || pathname.startsWith("/v/")) {
      const pathId = pathname.split("/")[2]?.split("?")[0];
      if (pathId) {
        id = pathId;
      }
    }
    // youtube.com/shorts/{id}
    else if (pathname.startsWith("/shorts/")) {
      const pathId = pathname.split("/")[2]?.split("?")[0];
      if (pathId) {
        id = pathId;
      }
    }
  } catch {
    // URL parsing failed
  }

  return { id, queryParams };
}

/**
 * Check if a URL is a YouTube URL
 */
function isYouTubeUrl(url: string): boolean {
  const lowerUrl = url.toLowerCase();
  return YOUTUBE_PATTERNS.some((pattern) => lowerUrl.includes(pattern));
}

export const youtube: EmbedProvider = {
  name: "youtube",

  parse(input: string): ParsedEmbed | null {
    const trimmed = input.trim();

    // Check if it's a YouTube URL
    if (!isYouTubeUrl(trimmed)) {
      // Could be a raw ID - YouTube IDs are typically 11 characters
      // containing letters, numbers, underscores, and hyphens
      if (/^[a-zA-Z0-9_-]{11}$/.test(trimmed)) {
        return {
          provider: "youtube",
          id: trimmed,
        };
      }
      return null;
    }

    const { id, queryParams } = extractYouTubeId(trimmed);

    if (!id) {
      return null;
    }

    const result: ParsedEmbed = {
      provider: "youtube",
      id,
      originalUrl: trimmed,
      queryParams,
    };

    // Extract start time from various formats
    const startParam = queryParams.t || queryParams.start || queryParams.time_continue;
    if (startParam) {
      const startSeconds = timestampToSeconds(startParam);
      if (startSeconds !== null) {
        result.queryParams = {
          ...result.queryParams,
          start: String(startSeconds),
        };
      }
    }

    // Extract end time
    if (queryParams.end) {
      const endSeconds = timestampToSeconds(queryParams.end);
      if (endSeconds !== null) {
        result.queryParams = {
          ...result.queryParams,
          end: String(endSeconds),
        };
      }
    }

    return result;
  },

  getEmbedUrl(id: string, options?: EmbedOptions): string {
    const params: Record<string, string | number | boolean | undefined> = {};

    // Use nocookie domain for privacy/GDPR compliance
    const domain =
      options?.consent === false
        ? "www.youtube-nocookie.com"
        : "www.youtube.com";

    if (options?.autoplay) {
      params.autoplay = 1;
    }

    if (options?.startTime !== undefined) {
      params.start = options.startTime;
    }

    if (options?.endTime !== undefined) {
      params.end = options.endTime;
    }

    if (options?.enableApi) {
      params.enablejsapi = 1;
    }

    if (options?.origin) {
      params.origin = options.origin;
    }

    // Always enable rel=0 to prevent showing related videos from other channels
    params.rel = 0;

    const queryString = buildQueryString(params);
    return `https://${domain}/embed/${id}${queryString}`;
  },

  getThumbnailUrl(
    id: string,
    size: "default" | "medium" | "high" | "max" = "high"
  ): string {
    // YouTube thumbnail URL formats:
    // default: 120x90
    // mqdefault (medium): 320x180
    // hqdefault (high): 480x360
    // sddefault: 640x480
    // maxresdefault: 1280x720 (may not exist for all videos)
    const sizeMap = {
      default: "default",
      medium: "mqdefault",
      high: "hqdefault",
      max: "maxresdefault",
    };

    return `https://img.youtube.com/vi/${id}/${sizeMap[size]}.jpg`;
  },

  async lookupMeta(
    id: string,
    fetchFn: typeof fetch = fetch
  ): Promise<EmbedMeta | null> {
    try {
      // Use oEmbed API - no API key required
      const videoUrl = `https://www.youtube.com/watch?v=${id}`;
      const oembedUrl = `https://www.youtube.com/oembed?url=${encodeURIComponent(videoUrl)}&format=json`;

      const response = await fetchFn(oembedUrl);

      if (!response.ok) {
        return null;
      }

      const data = await response.json();

      const meta: EmbedMeta = {
        title: data.title,
        authorName: data.author_name,
        authorUrl: data.author_url,
        thumbnailUrl: data.thumbnail_url,
        thumbnailWidth: data.thumbnail_width,
        thumbnailHeight: data.thumbnail_height,
      };

      // Note: YouTube oEmbed doesn't provide duration.
      // The get_video_info endpoint is blocked by CORS in browsers.
      // Duration must be fetched via a server-side proxy.

      return meta;
    } catch {
      return null;
    }
  },
};
