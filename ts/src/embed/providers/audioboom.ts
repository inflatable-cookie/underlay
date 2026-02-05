/**
 * Audioboom Embed Provider
 *
 * Supports parsing and embedding Audioboom audio content.
 *
 * URL formats supported:
 * Single audio:
 * - https://audioboom.com/posts/{id}
 * - https://audioboom.com/boos/{id}
 * - https://embeds.audioboom.com/posts/{id}/embed/v4
 *
 * Playlist:
 * - https://audioboom.com/publishing/playlist/v4?data_for_content_type={id}
 * - https://embeds.audioboom.com/publishing/playlist/v4?...
 */

import type { EmbedProvider, ParsedEmbed, EmbedOptions, EmbedMeta } from "../types.js";
import { normalizeUrl, parseQueryString, buildQueryString } from "../utils.js";

const AUDIOBOOM_PATTERNS = [
  "audioboom.com",
  "audioboo.com",
  "embeds.audioboom.com",
];

type AudioboomEmbedType = "single" | "playlist";

/**
 * Extract Audioboom ID and type from a URL
 */
function extractAudioboomId(url: string): {
  id: string | null;
  embedType: AudioboomEmbedType;
  queryParams: Record<string, string>;
} {
  const normalized = normalizeUrl(url);
  let id: string | null = null;
  let embedType: AudioboomEmbedType = "single";
  let queryParams: Record<string, string> = {};

  try {
    const urlObj = new URL(normalized);
    const pathname = urlObj.pathname;
    queryParams = parseQueryString(urlObj.search);

    // Playlist URL: /publishing/playlist/v4?data_for_content_type={id}
    if (pathname.includes("/publishing/playlist/")) {
      embedType = "playlist";
      // The playlist ID can be in various params
      id =
        queryParams.data_for_content_type ||
        queryParams.playlist_id ||
        queryParams.channel_id ||
        null;
    }
    // Single audio: /posts/{id} or /boos/{id}
    else if (pathname.startsWith("/posts/") || pathname.startsWith("/boos/")) {
      const parts = pathname.split("/");
      const rawId = parts[2]?.split("/")[0]?.split("?")[0];
      // Audioboom IDs can be numeric or slugs
      if (rawId) {
        // Extract numeric ID if it's a slug like "12345-title-here"
        const numericMatch = rawId.match(/^(\d+)/);
        id = numericMatch ? numericMatch[1] : rawId;
      }
    }
    // Embed URL: /posts/{id}/embed/v4
    else if (pathname.includes("/embed/")) {
      const parts = pathname.split("/");
      const postsIndex = parts.indexOf("posts");
      if (postsIndex !== -1 && parts[postsIndex + 1]) {
        const rawId = parts[postsIndex + 1];
        const numericMatch = rawId.match(/^(\d+)/);
        id = numericMatch ? numericMatch[1] : rawId;
      }
    }
  } catch {
    // URL parsing failed
  }

  return { id, embedType, queryParams };
}

/**
 * Check if a URL is an Audioboom URL
 */
function isAudioboomUrl(url: string): boolean {
  const lowerUrl = url.toLowerCase();
  return AUDIOBOOM_PATTERNS.some((pattern) => lowerUrl.includes(pattern));
}

export const audioboom: EmbedProvider = {
  name: "audioboom",

  parse(input: string): ParsedEmbed | null {
    const trimmed = input.trim();

    // Check if it's an Audioboom URL
    if (!isAudioboomUrl(trimmed)) {
      // Could be a raw numeric ID
      if (/^\d{5,}$/.test(trimmed)) {
        return {
          provider: "audioboom",
          id: trimmed,
          embedType: "single",
        };
      }
      return null;
    }

    const { id, embedType, queryParams } = extractAudioboomId(trimmed);

    if (!id) {
      return null;
    }

    return {
      provider: "audioboom",
      id,
      originalUrl: trimmed,
      queryParams,
      embedType,
    };
  },

  getEmbedUrl(id: string, options?: EmbedOptions): string {
    // Note: We need to know if this is a single or playlist
    // For this basic implementation, assume single unless id contains special chars
    // In practice, the embedType should be passed through somehow

    const params: Record<string, string | number | boolean | undefined> = {};

    if (options?.autoplay) {
      params.autoplay = 1;
    }

    // Single audio embed
    return `https://embeds.audioboom.com/posts/${id}/embed/v4${buildQueryString(params)}`;
  },

  // Audioboom doesn't have predictable thumbnail URLs
  getThumbnailUrl: undefined,

  async lookupMeta(
    id: string,
    fetchFn: typeof fetch = fetch
  ): Promise<EmbedMeta | null> {
    try {
      // Use oEmbed API for single posts
      const postUrl = `https://audioboom.com/posts/${id}`;
      const oembedUrl = `https://audioboom.com/publishing/oembed.json?url=${encodeURIComponent(postUrl)}`;

      const response = await fetchFn(oembedUrl);

      if (!response.ok) {
        return null;
      }

      const data = await response.json();

      const meta: EmbedMeta = {
        title: data.title,
        description: data.description,
        duration: data.duration,
        authorName: data.author_name,
        authorUrl: data.author_url,
        thumbnailUrl: data.thumbnail_url,
        thumbnailWidth: data.thumbnail_width,
        thumbnailHeight: data.thumbnail_height,
      };

      return meta;
    } catch {
      return null;
    }
  },
};

/**
 * Generate a playlist embed URL for Audioboom
 */
export function getAudioboomPlaylistUrl(
  playlistId: string,
  options?: EmbedOptions & {
    channelId?: string;
    contentType?: string;
  }
): string {
  const params: Record<string, string | number | boolean | undefined> = {
    data_for_content_type: playlistId,
  };

  if (options?.channelId) {
    params.channel_id = options.channelId;
  }

  if (options?.contentType) {
    params.content_type = options.contentType;
  }

  if (options?.autoplay) {
    params.autoplay = 1;
  }

  return `https://embeds.audioboom.com/publishing/playlist/v4${buildQueryString(params)}`;
}

/**
 * Look up playlist metadata (aggregates duration)
 */
export async function lookupPlaylistMeta(
  playlistId: string,
  fetchFn: typeof fetch = fetch
): Promise<EmbedMeta | null> {
  try {
    // Audioboom playlist API
    const apiUrl = `https://api.audioboom.com/playlists/${playlistId}`;

    const response = await fetchFn(apiUrl);

    if (!response.ok) {
      return null;
    }

    const data = await response.json();
    const playlist = data.body?.playlist;

    if (!playlist) {
      return null;
    }

    // Aggregate duration from all clips if available
    let totalDuration = 0;
    if (playlist.clips && Array.isArray(playlist.clips)) {
      for (const clip of playlist.clips) {
        if (clip.duration) {
          totalDuration += clip.duration;
        }
      }
    }

    return {
      title: playlist.title,
      description: playlist.description,
      duration: totalDuration > 0 ? totalDuration : undefined,
      thumbnailUrl: playlist.image_urls?.original || playlist.image_url,
      authorName: playlist.channel?.title,
    };
  } catch {
    return null;
  }
}
