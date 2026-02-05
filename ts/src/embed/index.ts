/**
 * Underlay Embed Parser
 *
 * A TypeScript library for parsing and handling media embeds from various
 * providers (YouTube, Vimeo, Audioboom, etc.).
 *
 * Features:
 * - Parse embed HTML, URLs, and raw IDs
 * - Detect provider from URL patterns
 * - Generate normalized embed URLs
 * - Look up metadata via oEmbed APIs
 * - Generate iframe HTML with proper attributes
 * - Generic fallback for unsupported platforms
 *
 * @example
 * ```typescript
 * import { parseEmbed, getEmbedUrl, lookupMeta } from '@decodelabs/underlay/embed';
 *
 * // Parse a YouTube URL
 * const result = parseEmbed('https://www.youtube.com/watch?v=dQw4w9WgXcQ');
 * if (result.success) {
 *   console.log(result.parsed.provider); // 'youtube'
 *   console.log(result.parsed.id);       // 'dQw4w9WgXcQ'
 *
 *   // Get embed URL
 *   const embedUrl = getEmbedUrl(result.parsed, { consent: false });
 *
 *   // Look up metadata
 *   const meta = await lookupMeta(result.parsed);
 *   if (meta) {
 *     console.log(meta.title, meta.duration);
 *   }
 * }
 * ```
 */

// Types
export type {
  EmbedProvider,
  ParsedEmbed,
  EmbedMeta,
  EmbedOptions,
  RenderOptions,
  ProviderRegistry,
  ParseEmbedResult,
  EmbedParserConfig,
} from "./types.js";

// Parser functions
export {
  parseEmbed,
  getEmbedUrl,
  renderEmbed,
  getThumbnailUrl,
  supportsThumbnailUrl,
  supportsMetadataLookup,
} from "./parser.js";

// Metadata functions
export {
  lookupMeta,
  lookupMetaById,
  lookupMetaBatch,
  lookupMetaWithResult,
  createMetaCache,
  metaCache,
  formatDuration,
  parseDuration,
  type MetaLookupResult,
} from "./metadata.js";

// Provider registry
export {
  defaultRegistry,
  createProviderRegistry,
  detectProviderFromUrl,
  URL_PATTERNS,
  PROVIDER_ACCENTS,
  getProviderAccent,
} from "./providers/index.js";

// Individual providers (for direct access if needed)
export { youtube } from "./providers/youtube.js";
export { vimeo } from "./providers/vimeo.js";
export { audioboom, getAudioboomPlaylistUrl, lookupPlaylistMeta } from "./providers/audioboom.js";
export {
  generic,
  renderGenericEmbed,
  isDomainAllowed,
  validateEmbedHtml,
  type GenericParsedEmbed,
} from "./providers/generic.js";

// Utility functions
export {
  extractUrlFromEmbed,
  extractAttribute,
  extractDimensions,
  decodeHtmlEntities,
  isEmbedHtml,
  isUrl,
  normalizeUrl,
  parseQueryString,
  buildQueryString,
  getHostname,
  hostnameMatches,
  isSafeEmbedUrl,
  generateIframeHtml,
  escapeHtmlAttribute,
  secondsToTimestamp,
  timestampToSeconds,
} from "./utils.js";

// Server proxy utilities
export {
  CORS_BLOCKED_PROVIDERS,
  requiresServerProxy,
  createProxyAwareLookup,
  proxyResponseToMeta,
  type EmbedMetaProxyRequest,
  type EmbedMetaProxyResponse,
  type ProxyLookupConfig,
} from "./proxy.js";
