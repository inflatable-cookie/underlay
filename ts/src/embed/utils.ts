/**
 * Embed Parser Utilities
 *
 * Helper functions for URL parsing, HTML extraction, and embed validation.
 */

/**
 * Extract src URL from embed HTML (iframe, object, embed, script tags)
 */
export function extractUrlFromEmbed(html: string): string | null {
  // Decode HTML entities first
  const decoded = decodeHtmlEntities(html);

  // Try iframe src
  const iframeSrc = extractAttribute(decoded, "iframe", "src");
  if (iframeSrc) return iframeSrc;

  // Try embed src
  const embedSrc = extractAttribute(decoded, "embed", "src");
  if (embedSrc) return embedSrc;

  // Try object data
  const objectData = extractAttribute(decoded, "object", "data");
  if (objectData) return objectData;

  // Try script src (some providers use script embeds)
  const scriptSrc = extractAttribute(decoded, "script", "src");
  if (scriptSrc) return scriptSrc;

  return null;
}

/**
 * Extract an attribute value from an HTML tag
 */
export function extractAttribute(
  html: string,
  tagName: string,
  attrName: string
): string | null {
  // Match the tag with flexible attribute ordering
  const tagPattern = new RegExp(
    `<${tagName}[^>]*\\s${attrName}\\s*=\\s*["']([^"']+)["'][^>]*>`,
    "i"
  );
  const match = html.match(tagPattern);
  return match?.[1] ?? null;
}

/**
 * Extract width and height from embed HTML
 */
export function extractDimensions(html: string): {
  width?: number;
  height?: number;
} {
  const decoded = decodeHtmlEntities(html);
  const result: { width?: number; height?: number } = {};

  // Try to extract from any tag that might have dimensions
  const widthMatch = decoded.match(/\bwidth\s*=\s*["']?(\d+)/i);
  const heightMatch = decoded.match(/\bheight\s*=\s*["']?(\d+)/i);

  if (widthMatch) {
    result.width = parseInt(widthMatch[1], 10);
  }
  if (heightMatch) {
    result.height = parseInt(heightMatch[1], 10);
  }

  return result;
}

/**
 * Decode common HTML entities
 */
export function decodeHtmlEntities(html: string): string {
  return html
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&#x27;/g, "'")
    .replace(/&#x2F;/g, "/");
}

/**
 * Check if a string looks like embed HTML
 */
export function isEmbedHtml(input: string): boolean {
  const trimmed = input.trim();
  // Check for common embed tags
  return (
    /<iframe\s/i.test(trimmed) ||
    /<embed\s/i.test(trimmed) ||
    /<object\s/i.test(trimmed) ||
    /<script\s[^>]*src\s*=/i.test(trimmed)
  );
}

/**
 * Check if a string looks like a URL
 */
export function isUrl(input: string): boolean {
  const trimmed = input.trim();
  return (
    trimmed.startsWith("http://") ||
    trimmed.startsWith("https://") ||
    trimmed.startsWith("//")
  );
}

/**
 * Normalize a URL (handle protocol-relative, ensure https)
 */
export function normalizeUrl(url: string): string {
  let normalized = url.trim();

  // Handle protocol-relative URLs
  if (normalized.startsWith("//")) {
    normalized = "https:" + normalized;
  }

  // Upgrade http to https
  if (normalized.startsWith("http://")) {
    normalized = "https://" + normalized.slice(7);
  }

  return normalized;
}

/**
 * Parse query string into key-value pairs
 */
export function parseQueryString(queryString: string): Record<string, string> {
  const params: Record<string, string> = {};

  if (!queryString) return params;

  // Remove leading ? if present
  const qs = queryString.startsWith("?") ? queryString.slice(1) : queryString;

  for (const pair of qs.split("&")) {
    const [key, value] = pair.split("=");
    if (key) {
      params[decodeURIComponent(key)] = value
        ? decodeURIComponent(value)
        : "";
    }
  }

  return params;
}

/**
 * Build a query string from key-value pairs
 */
export function buildQueryString(params: Record<string, string | number | boolean | undefined>): string {
  const pairs: string[] = [];

  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined && value !== null && value !== "") {
      pairs.push(
        `${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`
      );
    }
  }

  return pairs.length > 0 ? "?" + pairs.join("&") : "";
}

/**
 * Extract the hostname from a URL
 */
export function getHostname(url: string): string | null {
  try {
    const normalized = normalizeUrl(url);
    const urlObj = new URL(normalized);
    return urlObj.hostname.toLowerCase();
  } catch {
    return null;
  }
}

/**
 * Check if a URL's hostname matches any of the given patterns
 */
export function hostnameMatches(url: string, patterns: string[]): boolean {
  const hostname = getHostname(url);
  if (!hostname) return false;

  return patterns.some((pattern) => {
    const lowerPattern = pattern.toLowerCase();
    // Check for exact match or subdomain match
    return (
      hostname === lowerPattern || hostname.endsWith("." + lowerPattern)
    );
  });
}

/**
 * Validate that a URL is safe to embed (no javascript:, data:, etc.)
 */
export function isSafeEmbedUrl(url: string): boolean {
  const trimmed = url.trim().toLowerCase();

  // Block dangerous protocols
  if (
    trimmed.startsWith("javascript:") ||
    trimmed.startsWith("data:") ||
    trimmed.startsWith("vbscript:") ||
    trimmed.startsWith("file:")
  ) {
    return false;
  }

  // Must be http, https, or protocol-relative
  return (
    trimmed.startsWith("http://") ||
    trimmed.startsWith("https://") ||
    trimmed.startsWith("//")
  );
}

/**
 * Generate a basic iframe HTML string
 */
export function generateIframeHtml(
  src: string,
  options: {
    width?: number | string;
    height?: number | string;
    title?: string;
    allowFullscreen?: boolean;
    allow?: string;
    loading?: "eager" | "lazy";
    sandbox?: string;
    className?: string;
  } = {}
): string {
  const {
    width = 560,
    height = 315,
    title = "Embedded content",
    allowFullscreen = true,
    allow = "autoplay; encrypted-media; fullscreen",
    loading = "lazy",
    sandbox,
    className,
  } = options;

  const attrs: string[] = [
    `src="${escapeHtmlAttribute(src)}"`,
    `width="${width}"`,
    `height="${height}"`,
    `title="${escapeHtmlAttribute(title)}"`,
    'frameborder="0"',
    `loading="${loading}"`,
  ];

  if (allow) {
    attrs.push(`allow="${escapeHtmlAttribute(allow)}"`);
  }

  if (allowFullscreen) {
    attrs.push("allowfullscreen");
  }

  if (sandbox) {
    attrs.push(`sandbox="${escapeHtmlAttribute(sandbox)}"`);
  }

  if (className) {
    attrs.push(`class="${escapeHtmlAttribute(className)}"`);
  }

  return `<iframe ${attrs.join(" ")}></iframe>`;
}

/**
 * Escape a string for use in an HTML attribute
 */
export function escapeHtmlAttribute(str: string): string {
  return str
    .replace(/&/g, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

/**
 * Convert seconds to YouTube-style timestamp (e.g., 1h2m3s or 2m3s or 45s)
 */
export function secondsToTimestamp(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;

  let result = "";
  if (h > 0) result += `${h}h`;
  if (m > 0) result += `${m}m`;
  if (s > 0 || result === "") result += `${s}s`;

  return result;
}

/**
 * Parse a timestamp string (1h2m3s, 2:30, etc.) to seconds
 */
export function timestampToSeconds(timestamp: string): number | null {
  // Try YouTube format: 1h2m3s
  const ytMatch = timestamp.match(
    /^(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?$/i
  );
  if (ytMatch && (ytMatch[1] || ytMatch[2] || ytMatch[3])) {
    const h = parseInt(ytMatch[1] || "0", 10);
    const m = parseInt(ytMatch[2] || "0", 10);
    const s = parseInt(ytMatch[3] || "0", 10);
    return h * 3600 + m * 60 + s;
  }

  // Try colon format: 1:02:03 or 2:30
  const colonMatch = timestamp.match(/^(\d+):(\d{2})(?::(\d{2}))?$/);
  if (colonMatch) {
    if (colonMatch[3]) {
      // h:mm:ss
      return (
        parseInt(colonMatch[1], 10) * 3600 +
        parseInt(colonMatch[2], 10) * 60 +
        parseInt(colonMatch[3], 10)
      );
    } else {
      // m:ss
      return (
        parseInt(colonMatch[1], 10) * 60 + parseInt(colonMatch[2], 10)
      );
    }
  }

  // Try plain seconds
  const plainMatch = timestamp.match(/^(\d+)$/);
  if (plainMatch) {
    return parseInt(plainMatch[1], 10);
  }

  return null;
}
