/**
 * Media Embed Parser Types
 *
 * Core interfaces for parsing and handling media embeds from various providers
 * (YouTube, Vimeo, Audioboom, etc.) as well as generic/unknown sources.
 */

/**
 * Options for generating embed URLs
 */
export interface EmbedOptions {
  /** Auto-play the media on load */
  autoplay?: boolean;
  /** Start time in seconds */
  startTime?: number;
  /** End time in seconds (YouTube only) */
  endTime?: number;
  /** GDPR consent mode - use privacy-enhanced domains (e.g., youtube-nocookie.com) */
  consent?: boolean;
  /** Enable JavaScript API access */
  enableApi?: boolean;
  /** Origin domain for API access */
  origin?: string;
}

/**
 * Result of parsing an embed input (code, URL, or ID)
 */
export interface ParsedEmbed {
  /** Provider name (youtube, vimeo, audioboom, generic) */
  provider: string;
  /** Provider-specific media ID */
  id: string;
  /** Original URL if parsed from URL or embed code */
  originalUrl?: string;
  /** Original embed HTML if input was embed code */
  originalEmbed?: string;
  /** Width from embed code (if present) */
  width?: number;
  /** Height from embed code (if present) */
  height?: number;
  /** Query parameters extracted from URL */
  queryParams?: Record<string, string>;
  /** Embed type for providers with multiple formats (e.g., audioboom single vs playlist) */
  embedType?: string;
}

/**
 * Metadata retrieved from provider APIs (oEmbed, etc.)
 */
export interface EmbedMeta {
  /** Media title */
  title?: string;
  /** Media description */
  description?: string;
  /** Duration in seconds */
  duration?: number;
  /** Thumbnail image URL */
  thumbnailUrl?: string;
  /** Thumbnail width */
  thumbnailWidth?: number;
  /** Thumbnail height */
  thumbnailHeight?: number;
  /** Author/channel name */
  authorName?: string;
  /** Author/channel URL */
  authorUrl?: string;
  /** Upload/publish date (ISO string) */
  uploadDate?: string;
}

/**
 * Options for rendering embed HTML
 */
export interface RenderOptions extends EmbedOptions {
  /** Iframe width (number for pixels, string for other units) */
  width?: number | string;
  /** Iframe height (number for pixels, string for other units) */
  height?: number | string;
  /** Additional CSS class for the iframe */
  className?: string;
  /** Title attribute for accessibility */
  title?: string;
  /** Loading strategy */
  loading?: "eager" | "lazy";
  /** Sandbox restrictions for generic embeds */
  sandbox?: string;
}

/**
 * Provider implementation interface
 */
export interface EmbedProvider {
  /** Provider identifier */
  name: string;

  /**
   * Attempt to parse an input string (URL or ID) for this provider.
   * Returns null if the input doesn't match this provider.
   */
  parse(input: string): ParsedEmbed | null;

  /**
   * Generate an embed URL for the given media ID and options.
   */
  getEmbedUrl(id: string, options?: EmbedOptions): string;

  /**
   * Get a direct thumbnail URL (if available without API call).
   * Not all providers support this.
   */
  getThumbnailUrl?(id: string, size?: "default" | "medium" | "high" | "max"): string;

  /**
   * Look up metadata via provider API (oEmbed, etc.).
   * Returns null if lookup fails or is not supported.
   *
   * @param id - Media ID
   * @param fetchFn - Fetch function (for server-side proxying)
   * @param parsed - Full parsed embed (for providers needing extra context like embedType)
   */
  lookupMeta?(id: string, fetchFn?: typeof fetch, parsed?: ParsedEmbed): Promise<EmbedMeta | null>;
}

/**
 * Registry of available providers
 */
export interface ProviderRegistry {
  /** Get a provider by name */
  get(name: string): EmbedProvider | undefined;

  /** Get all registered providers */
  all(): EmbedProvider[];

  /** Register a new provider */
  register(provider: EmbedProvider): void;

  /** Check if a provider is registered */
  has(name: string): boolean;
}

/**
 * Result of the main parseEmbed function
 */
export interface ParseEmbedResult {
  /** Whether parsing was successful */
  success: boolean;
  /** Parsed embed data (if successful) */
  parsed?: ParsedEmbed;
  /** Error message (if unsuccessful) */
  error?: string;
}

/**
 * Configuration for the embed parser
 */
export interface EmbedParserConfig {
  /** Allowed providers (empty = all) */
  allowedProviders?: string[];
  /** Whether to allow generic/unknown embeds */
  allowGeneric?: boolean;
  /** Default embed options */
  defaultOptions?: EmbedOptions;
  /** Allowed domains for generic embeds (security) */
  allowedDomains?: string[];
}
