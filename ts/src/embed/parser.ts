/**
 * Embed Parser
 *
 * Main entry point for parsing media embeds. Handles three input formats:
 * 1. Embed HTML (iframe, object, embed, script tags)
 * 2. URLs (direct video/audio URLs, share links, embed URLs)
 * 3. Raw IDs (provider-specific identifiers)
 *
 * The parser detects the provider from the URL patterns and delegates
 * to the appropriate provider for parsing.
 */

import type {
  ParsedEmbed,
  ParseEmbedResult,
  EmbedParserConfig,
  EmbedOptions,
  RenderOptions,
} from "./types.js";
import {
  isEmbedHtml,
  isUrl,
  extractUrlFromEmbed,
  extractDimensions,
  normalizeUrl,
  generateIframeHtml,
} from "./utils.js";
import {
  defaultRegistry,
  detectProviderFromUrl,
} from "./providers/index.js";
import {
  renderGenericEmbed,
  validateEmbedHtml,
  isDomainAllowed,
  type GenericParsedEmbed,
} from "./providers/generic.js";

/**
 * Parse an embed input (HTML, URL, or ID)
 *
 * @param input - The embed code, URL, or ID to parse
 * @param config - Optional parser configuration
 * @returns Parse result with success status and parsed data or error
 */
export function parseEmbed(
  input: string,
  config?: EmbedParserConfig
): ParseEmbedResult {
  const trimmed = input.trim();

  if (!trimmed) {
    return {
      success: false,
      error: "Empty input",
    };
  }

  // Step 1: Determine what kind of input we have
  let url: string | null = null;
  let dimensions: { width?: number; height?: number } = {};
  let originalEmbed: string | undefined;

  if (isEmbedHtml(trimmed)) {
    // Input is embed HTML - extract the URL from it
    url = extractUrlFromEmbed(trimmed);
    dimensions = extractDimensions(trimmed);
    originalEmbed = trimmed;

    if (!url) {
      // Couldn't extract URL, but it looks like HTML
      // Try generic provider for passthrough
      if (config?.allowGeneric !== false) {
        const genericResult = parseWithGenericFallback(
          trimmed,
          originalEmbed,
          dimensions,
          config
        );
        if (genericResult.success) {
          return genericResult;
        }
      }
      return {
        success: false,
        error: "Could not extract URL from embed code",
      };
    }
  } else if (isUrl(trimmed)) {
    // Input is a URL
    url = normalizeUrl(trimmed);
  } else {
    // Input might be a raw ID - try each provider
    return parseRawId(trimmed, config);
  }

  // Step 2: Detect provider from URL
  const providerName = detectProviderFromUrl(url);

  // Step 3: If we found a provider, use it
  if (providerName) {
    // Check if provider is allowed
    if (
      config?.allowedProviders &&
      config.allowedProviders.length > 0 &&
      !config.allowedProviders.includes(providerName)
    ) {
      return {
        success: false,
        error: `Provider "${providerName}" is not allowed`,
      };
    }

    const provider = defaultRegistry.get(providerName);
    if (provider) {
      const parsed = provider.parse(url);
      if (parsed) {
        // Add dimensions from embed HTML if available
        if (dimensions.width) parsed.width = dimensions.width;
        if (dimensions.height) parsed.height = dimensions.height;
        if (originalEmbed) parsed.originalEmbed = originalEmbed;

        return {
          success: true,
          parsed,
        };
      }
    }
  }

  // Step 4: No provider matched - try generic fallback
  if (config?.allowGeneric !== false) {
    return parseWithGenericFallback(url, originalEmbed, dimensions, config);
  }

  return {
    success: false,
    error: "Could not detect provider from URL",
  };
}

/**
 * Try to parse input as a raw ID using each provider
 */
function parseRawId(
  input: string,
  config?: EmbedParserConfig
): ParseEmbedResult {
  const providers = defaultRegistry.all();

  for (const provider of providers) {
    // Skip generic provider for raw ID matching
    if (provider.name === "generic") continue;

    // Check if provider is allowed
    if (
      config?.allowedProviders &&
      config.allowedProviders.length > 0 &&
      !config.allowedProviders.includes(provider.name)
    ) {
      continue;
    }

    const parsed = provider.parse(input);
    if (parsed) {
      return {
        success: true,
        parsed,
      };
    }
  }

  // Raw IDs can't be handled by generic provider
  return {
    success: false,
    error:
      "Could not detect provider. Please provide a full URL or embed code.",
  };
}

/**
 * Parse using generic fallback provider
 */
function parseWithGenericFallback(
  input: string,
  originalEmbed: string | undefined,
  dimensions: { width?: number; height?: number },
  config?: EmbedParserConfig
): ParseEmbedResult {
  const genericProvider = defaultRegistry.get("generic");
  if (!genericProvider) {
    return {
      success: false,
      error: "Generic provider not available",
    };
  }

  // For URLs, check domain allowlist
  if (isUrl(input) && config?.allowedDomains?.length) {
    if (!isDomainAllowed(input, config.allowedDomains)) {
      return {
        success: false,
        error: "Domain not in allowed list",
      };
    }
  }

  // For embed HTML, validate safety
  if (originalEmbed) {
    const validation = validateEmbedHtml(originalEmbed);
    if (!validation.valid) {
      return {
        success: false,
        error: validation.reason || "Invalid embed HTML",
      };
    }
  }

  const parsed = genericProvider.parse(
    originalEmbed || input
  ) as GenericParsedEmbed | null;

  if (parsed) {
    if (dimensions.width) parsed.width = dimensions.width;
    if (dimensions.height) parsed.height = dimensions.height;
    if (originalEmbed && !parsed.originalEmbed) {
      parsed.originalEmbed = originalEmbed;
    }

    return {
      success: true,
      parsed,
    };
  }

  return {
    success: false,
    error: "Could not parse embed",
  };
}

/**
 * Generate an embed URL for a parsed embed
 */
export function getEmbedUrl(
  parsed: ParsedEmbed,
  options?: EmbedOptions
): string {
  const provider = defaultRegistry.get(parsed.provider);
  if (!provider) {
    return "";
  }

  // Merge query params from parsed result with options
  const mergedOptions: EmbedOptions = { ...options };

  if (parsed.queryParams) {
    if (parsed.queryParams.start && !mergedOptions.startTime) {
      mergedOptions.startTime = parseInt(parsed.queryParams.start, 10);
    }
    if (parsed.queryParams.end && !mergedOptions.endTime) {
      mergedOptions.endTime = parseInt(parsed.queryParams.end, 10);
    }
  }

  return provider.getEmbedUrl(parsed.id, mergedOptions);
}

/**
 * Generate iframe HTML for a parsed embed
 */
export function renderEmbed(
  parsed: ParsedEmbed,
  options?: RenderOptions
): string {
  // For generic passthrough embeds, return original HTML
  if (parsed.provider === "generic") {
    const genericParsed = parsed as GenericParsedEmbed;
    if (genericParsed.isPassthrough && genericParsed.embedHtml) {
      return genericParsed.embedHtml;
    }
    return renderGenericEmbed(genericParsed, options);
  }

  // Get embed URL from provider
  const embedUrl = getEmbedUrl(parsed, options);
  if (!embedUrl) {
    return "";
  }

  // Generate iframe
  return generateIframeHtml(embedUrl, {
    width: options?.width ?? parsed.width ?? 560,
    height: options?.height ?? parsed.height ?? 315,
    title: options?.title ?? `${parsed.provider} embed`,
    allowFullscreen: true,
    allow: "autoplay; encrypted-media; fullscreen; picture-in-picture",
    loading: options?.loading ?? "lazy",
    className: options?.className,
  });
}

/**
 * Get a thumbnail URL for a parsed embed (if available)
 */
export function getThumbnailUrl(
  parsed: ParsedEmbed,
  size?: "default" | "medium" | "high" | "max"
): string | null {
  const provider = defaultRegistry.get(parsed.provider);
  if (!provider?.getThumbnailUrl) {
    return null;
  }
  return provider.getThumbnailUrl(parsed.id, size);
}

/**
 * Check if a provider supports direct thumbnail URLs
 */
export function supportsThumbnailUrl(providerName: string): boolean {
  const provider = defaultRegistry.get(providerName);
  return !!provider?.getThumbnailUrl;
}

/**
 * Check if a provider supports metadata lookup
 */
export function supportsMetadataLookup(providerName: string): boolean {
  const provider = defaultRegistry.get(providerName);
  return !!provider?.lookupMeta;
}
