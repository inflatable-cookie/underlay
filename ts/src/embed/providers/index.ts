/**
 * Embed Provider Registry
 *
 * Manages registration and lookup of embed providers.
 */

import type { EmbedProvider, ProviderRegistry } from "../types.js";
import { youtube } from "./youtube.js";
import { vimeo } from "./vimeo.js";
import { audioboom } from "./audioboom.js";
import { generic } from "./generic.js";

/**
 * URL patterns for provider detection
 */
export const URL_PATTERNS: Record<string, string[]> = {
  youtube: ["youtube.com", "youtu.be", "youtube-nocookie.com"],
  vimeo: ["vimeo.com", "player.vimeo.com"],
  audioboom: ["audioboom.com", "audioboo.com", "embeds.audioboom.com"],
};

/**
 * Brand accent colors for providers
 */
export const PROVIDER_ACCENTS: Record<string, string> = {
  youtube: "#ff0000",
  vimeo: "#1ab7ea",
  audioboom: "#f5a623",
};

/**
 * Get the brand accent color for a provider
 * @param provider - Provider name (case-insensitive)
 * @param fallback - Fallback color if provider not found (default: gray)
 */
export function getProviderAccent(
  provider: string | null | undefined,
  fallback = "#94a3b8"
): string {
  if (!provider) return fallback;
  return PROVIDER_ACCENTS[provider.toLowerCase()] ?? fallback;
}

/**
 * Create a provider registry with default providers
 */
export function createProviderRegistry(): ProviderRegistry {
  const providers = new Map<string, EmbedProvider>();

  const registry: ProviderRegistry = {
    get(name: string): EmbedProvider | undefined {
      return providers.get(name.toLowerCase());
    },

    all(): EmbedProvider[] {
      return Array.from(providers.values());
    },

    register(provider: EmbedProvider): void {
      providers.set(provider.name.toLowerCase(), provider);
    },

    has(name: string): boolean {
      return providers.has(name.toLowerCase());
    },
  };

  // Register default providers
  registry.register(youtube);
  registry.register(vimeo);
  registry.register(audioboom);
  registry.register(generic);

  return registry;
}

/**
 * Detect provider from a URL
 */
export function detectProviderFromUrl(url: string): string | null {
  const lowerUrl = url.toLowerCase();

  for (const [provider, patterns] of Object.entries(URL_PATTERNS)) {
    for (const pattern of patterns) {
      if (lowerUrl.includes(pattern.toLowerCase())) {
        return provider;
      }
    }
  }

  return null;
}

/**
 * Default provider registry instance
 */
export const defaultRegistry = createProviderRegistry();

// Re-export individual providers for direct access
export { youtube } from "./youtube.js";
export { vimeo } from "./vimeo.js";
export { audioboom } from "./audioboom.js";
export { generic } from "./generic.js";
