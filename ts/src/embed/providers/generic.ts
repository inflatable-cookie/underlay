/**
 * Generic/Fallback Embed Provider
 *
 * Handles embeds from unsupported platforms by:
 * 1. Using embed HTML as-is if valid
 * 2. Wrapping URLs in an iframe
 *
 * This provider is used when no specific provider is detected.
 */

import type { EmbedProvider, ParsedEmbed, EmbedOptions, RenderOptions } from "../types.js";
import {
  isUrl,
  isSafeEmbedUrl,
  generateIframeHtml,
  normalizeUrl,
  getHostname,
} from "../utils.js";

/**
 * Extended parsed embed for generic provider
 */
export interface GenericParsedEmbed extends ParsedEmbed {
  provider: "generic";
  /** Original embed HTML if input was valid embed code */
  embedHtml?: string;
  /** URL for iframe fallback */
  fallbackUrl?: string;
  /** Whether this is passthrough embed HTML */
  isPassthrough?: boolean;
}

export const generic: EmbedProvider = {
  name: "generic",

  parse(input: string): GenericParsedEmbed | null {
    const trimmed = input.trim();

    // Empty input
    if (!trimmed) {
      return null;
    }

    // If it's a URL, validate and create iframe wrapper
    if (isUrl(trimmed)) {
      if (!isSafeEmbedUrl(trimmed)) {
        return null; // Unsafe URL (javascript:, data:, etc.)
      }

      const normalized = normalizeUrl(trimmed);
      const hostname = getHostname(normalized);

      return {
        provider: "generic",
        id: hostname || "unknown",
        originalUrl: normalized,
        fallbackUrl: normalized,
      };
    }

    // If it looks like a raw ID (no URL, no HTML), we can't handle it
    // because we don't know which platform it belongs to
    if (!trimmed.includes("<") && !trimmed.includes(">")) {
      return null;
    }

    // Otherwise, assume it's embed HTML - pass through as-is
    // We could try to extract the src, but for generic embeds,
    // preserving the original HTML is safer
    return {
      provider: "generic",
      id: "embedded",
      originalEmbed: trimmed,
      embedHtml: trimmed,
      isPassthrough: true,
    };
  },

  getEmbedUrl(id: string, options?: EmbedOptions): string {
    // For generic embeds, the "id" might actually be a URL
    // This is a bit of a hack, but works for the fallback case
    if (id.startsWith("http://") || id.startsWith("https://")) {
      return id;
    }

    // If it's just an ID with no context, we can't generate a URL
    return "";
  },

  // Generic provider can't generate thumbnails
  getThumbnailUrl: undefined,

  // Generic provider can't look up metadata
  lookupMeta: undefined,
};

/**
 * Render a generic embed to HTML
 *
 * If the parsed embed has `embedHtml`, returns it as-is (passthrough).
 * If it has `fallbackUrl`, generates an iframe wrapper.
 */
export function renderGenericEmbed(
  parsed: GenericParsedEmbed,
  options?: RenderOptions
): string {
  // If we have original embed HTML, use it as-is
  if (parsed.embedHtml && parsed.isPassthrough) {
    return parsed.embedHtml;
  }

  // If we have a fallback URL, generate an iframe
  if (parsed.fallbackUrl) {
    return generateIframeHtml(parsed.fallbackUrl, {
      width: options?.width ?? 560,
      height: options?.height ?? 315,
      title: options?.title ?? `Embedded content from ${parsed.id}`,
      allowFullscreen: true,
      loading: options?.loading ?? "lazy",
      // For generic embeds, consider using sandbox for security
      // But this may break some embeds, so make it optional
      sandbox: options?.sandbox,
      className: options?.className,
    });
  }

  return "";
}

/**
 * Check if a domain is in an allowed list
 */
export function isDomainAllowed(
  url: string,
  allowedDomains: string[]
): boolean {
  if (allowedDomains.length === 0) {
    return true; // No restrictions
  }

  const hostname = getHostname(url);
  if (!hostname) {
    return false;
  }

  return allowedDomains.some((domain) => {
    const lowerDomain = domain.toLowerCase();
    return (
      hostname === lowerDomain ||
      hostname.endsWith("." + lowerDomain)
    );
  });
}

/**
 * Validate embed HTML for basic safety
 *
 * This is a basic check - it doesn't prevent all attacks but catches
 * obvious issues.
 */
export function validateEmbedHtml(html: string): {
  valid: boolean;
  reason?: string;
} {
  const lower = html.toLowerCase();

  // Check for script tags (except in src attribute context)
  if (/<script[^>]*>[\s\S]*<\/script>/i.test(html)) {
    // Allow script tags only if they have a src attribute and no inline code
    const scriptMatch = html.match(/<script([^>]*)>([\s\S]*?)<\/script>/gi);
    if (scriptMatch) {
      for (const script of scriptMatch) {
        // If there's content between script tags, it's inline code
        const contentMatch = script.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
        if (contentMatch && contentMatch[1].trim()) {
          return {
            valid: false,
            reason: "Inline script content not allowed",
          };
        }
      }
    }
  }

  // Check for dangerous event handlers
  const eventHandlers = [
    "onload",
    "onerror",
    "onclick",
    "onmouseover",
    "onfocus",
    "onblur",
    "onsubmit",
    "onkeydown",
    "onkeyup",
    "onkeypress",
  ];

  for (const handler of eventHandlers) {
    if (lower.includes(handler + "=")) {
      return {
        valid: false,
        reason: `Event handler "${handler}" not allowed`,
      };
    }
  }

  // Check for javascript: URLs
  if (lower.includes("javascript:")) {
    return {
      valid: false,
      reason: "JavaScript URLs not allowed",
    };
  }

  // Check for data: URLs in src attributes
  if (/src\s*=\s*["']?data:/i.test(html)) {
    return {
      valid: false,
      reason: "Data URLs in src not allowed",
    };
  }

  return { valid: true };
}
