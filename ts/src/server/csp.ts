/**
 * Content Security Policy (CSP) utilities for SvelteKit applications.
 *
 * Provides nonce-based CSP header generation with sensible defaults
 * that work well with SvelteKit's hydration model.
 *
 * @example
 * ```typescript
 * // In hooks.server.ts
 * import { createCspConfig, generateNonce, buildCspHeader, applyCspHeaders } from "@decodelabs/underlay/client";
 *
 * const cspConfig = createCspConfig({
 *   connectSrc: ["https://api.example.com"],
 *   frameSrc: ["https://www.youtube.com"]
 * });
 *
 * export const handle: Handle = async ({ event, resolve }) => {
 *   const nonce = generateNonce();
 *
 *   const response = await resolve(event, {
 *     transformPageChunk: ({ html }) => html.replace("%sveltekit.nonce%", nonce)
 *   });
 *
 *   applyCspHeaders(response, cspConfig, nonce);
 *   return response;
 * };
 * ```
 */

/// <reference types="node" />
import { randomBytes } from "crypto";
import {
  DEFAULT_SECURITY_HEADERS,
  resolveCspConfig,
  resolveSecurityHeadersConfig
} from "./csp-config";

// ============================================================================
// Types
// ============================================================================

/**
 * CSP directive sources that can be used in policies.
 */
export type CspSource = string;

/**
 * Configuration for Content Security Policy.
 *
 * All array properties are additive to sensible defaults.
 * Set to `false` to disable that directive entirely.
 *
 * Note: In development with Vite, CSP headers should be skipped entirely
 * since Vite's HMR requires inline scripts that conflict with nonce-based CSP.
 * The CSP spec ignores 'unsafe-inline' when a nonce is present.
 */
export interface CspConfig {
  /**
   * Base default-src. Defaults to ["'self'"].
   */
  defaultSrc?: CspSource[] | false;

  /**
   * Script sources. Nonce is automatically added.
   * Defaults to ["'self'"].
   */
  scriptSrc?: CspSource[] | false;

  /**
   * Style sources.
   * Defaults to ["'self'", "'unsafe-inline'"] (needed for component styles).
   */
  styleSrc?: CspSource[] | false;

  /**
   * Image sources.
   * Defaults to ["'self'", "data:", "https:"].
   */
  imgSrc?: CspSource[] | false;

  /**
   * Font sources.
   * Defaults to ["'self'"].
   */
  fontSrc?: CspSource[] | false;

  /**
   * Connect sources (fetch, XHR, WebSocket).
   * Defaults to ["'self'"]. Add your API URLs here.
   */
  connectSrc?: CspSource[] | false;

  /**
   * Frame sources (iframes).
   * Defaults to ["'self'"]. Add video embed domains here.
   */
  frameSrc?: CspSource[] | false;

  /**
   * Media sources (audio, video).
   * Defaults to ["'self'"].
   */
  mediaSrc?: CspSource[] | false;

  /**
   * Object/embed sources.
   * Defaults to ["'none'"] (blocks plugins).
   */
  objectSrc?: CspSource[] | false;

  /**
   * Form action targets.
   * Defaults to ["'self'"].
   */
  formAction?: CspSource[] | false;

  /**
   * Base URI for relative URLs.
   * Defaults to ["'self'"].
   */
  baseUri?: CspSource[] | false;

  /**
   * Frame ancestors (who can embed this page).
   * Defaults to ["'none'"] (prevents clickjacking).
   */
  frameAncestors?: CspSource[] | false;

  /**
   * Whether to use report-only mode.
   * Defaults to false (enforcing mode).
   */
  reportOnly?: boolean;

  /**
   * Report URI for CSP violations.
   * Optional - if set, violations will be reported to this endpoint.
   */
  reportUri?: string;
}

/**
 * Resolved CSP configuration with all defaults applied.
 */
export interface ResolvedCspConfig {
  defaultSrc: CspSource[];
  scriptSrc: CspSource[];
  styleSrc: CspSource[];
  imgSrc: CspSource[];
  fontSrc: CspSource[];
  connectSrc: CspSource[];
  frameSrc: CspSource[];
  mediaSrc: CspSource[];
  objectSrc: CspSource[];
  formAction: CspSource[];
  baseUri: CspSource[];
  frameAncestors: CspSource[];
  reportOnly: boolean;
  reportUri?: string;
}

/**
 * Additional security headers that complement CSP.
 */
export interface SecurityHeadersConfig {
  /**
   * X-Content-Type-Options header.
   * Defaults to "nosniff".
   */
  contentTypeOptions?: string | false;

  /**
   * X-Frame-Options header.
   * Defaults to "DENY".
   */
  frameOptions?: string | false;

  /**
   * Referrer-Policy header.
   * Defaults to "strict-origin-when-cross-origin".
   */
  referrerPolicy?: string | false;

  /**
   * X-XSS-Protection header.
   * Defaults to false (modern browsers use CSP instead).
   */
  xssProtection?: string | false;

  /**
   * Permissions-Policy header.
   * Optional - set to configure browser feature permissions.
   */
  permissionsPolicy?: string | false;
}

// ============================================================================
// Nonce Generation
// ============================================================================

/**
 * Generate a cryptographically secure nonce for CSP.
 *
 * Uses Web Crypto API for secure random generation.
 * Returns a base64-encoded 16-byte random value.
 *
 * @returns A unique nonce string for this request
 */
export function generateNonce(): string {
  return randomBytes(16).toString("base64");
}

// ============================================================================
// Configuration
// ============================================================================

/**
 * Create a CSP configuration by merging with defaults.
 *
 * @param config - Partial configuration to merge with defaults
 * @returns Resolved configuration with all values set
 *
 * @example
 * ```typescript
 * const config = createCspConfig({
 *   connectSrc: ["https://api.example.com"],
 *   frameSrc: ["https://www.youtube.com", "https://player.vimeo.com"],
 *   reportOnly: true // Start in report-only mode
 * });
 * ```
 */
export function createCspConfig(config: CspConfig = {}): ResolvedCspConfig {
  return resolveCspConfig(config);
}

/**
 * Create security headers configuration by merging with defaults.
 *
 * @param config - Partial configuration to merge with defaults
 * @returns Resolved configuration with all values set
 */
export function createSecurityHeadersConfig(
  config: SecurityHeadersConfig = {}
): SecurityHeadersConfig {
  return resolveSecurityHeadersConfig(config);
}

// ============================================================================
// Header Building
// ============================================================================

/**
 * Build a CSP header value from configuration.
 *
 * @param config - Resolved CSP configuration
 * @param nonce - Optional nonce to include in script-src
 * @returns The CSP header value string
 *
 * @example
 * ```typescript
 * const nonce = generateNonce();
 * const headerValue = buildCspHeader(config, nonce);
 * response.headers.set("Content-Security-Policy", headerValue);
 * ```
 */
export function buildCspHeader(config: ResolvedCspConfig, nonce?: string): string {
  const directives: string[] = [];

  const addDirective = (name: string, sources: CspSource[]) => {
    if (sources.length > 0) {
      directives.push(`${name} ${sources.join(" ")}`);
    }
  };

  addDirective("default-src", config.defaultSrc);

  // Add nonce to script-src if provided
  const scriptSources = nonce
    ? [...config.scriptSrc, `'nonce-${nonce}'`]
    : config.scriptSrc;
  addDirective("script-src", scriptSources);

  // Note: Do NOT add nonce to style-src. SvelteKit component styles use inline
  // styles that rely on 'unsafe-inline'. Adding a nonce makes the browser ignore
  // 'unsafe-inline' per CSP spec, breaking all component styles.
  addDirective("style-src", config.styleSrc);

  addDirective("img-src", config.imgSrc);
  addDirective("font-src", config.fontSrc);
  addDirective("connect-src", config.connectSrc);
  addDirective("frame-src", config.frameSrc);
  addDirective("media-src", config.mediaSrc);
  addDirective("object-src", config.objectSrc);
  addDirective("form-action", config.formAction);
  addDirective("base-uri", config.baseUri);
  addDirective("frame-ancestors", config.frameAncestors);

  if (config.reportUri) {
    directives.push(`report-uri ${config.reportUri}`);
  }

  return directives.join("; ");
}

/**
 * Get the appropriate CSP header name based on configuration.
 *
 * @param config - CSP configuration
 * @returns "Content-Security-Policy-Report-Only" or "Content-Security-Policy"
 */
export function getCspHeaderName(config: ResolvedCspConfig): string {
  return config.reportOnly
    ? "Content-Security-Policy-Report-Only"
    : "Content-Security-Policy";
}

/**
 * Apply CSP and security headers to a Response object.
 *
 * @param response - The Response to add headers to
 * @param cspConfig - CSP configuration
 * @param nonce - Optional nonce for script-src
 * @param securityHeaders - Additional security headers configuration
 *
 * @example
 * ```typescript
 * const response = await resolve(event);
 * applyCspHeaders(response, cspConfig, nonce);
 * return response;
 * ```
 */
export function applyCspHeaders(
  response: Response,
  cspConfig: ResolvedCspConfig,
  nonce?: string,
  securityHeaders: SecurityHeadersConfig = DEFAULT_SECURITY_HEADERS
): void {
  // CSP header
  response.headers.set(
    getCspHeaderName(cspConfig),
    buildCspHeader(cspConfig, nonce)
  );

  // Additional security headers
  if (securityHeaders.contentTypeOptions) {
    response.headers.set("X-Content-Type-Options", securityHeaders.contentTypeOptions);
  }
  if (securityHeaders.frameOptions) {
    response.headers.set("X-Frame-Options", securityHeaders.frameOptions);
  }
  if (securityHeaders.referrerPolicy) {
    response.headers.set("Referrer-Policy", securityHeaders.referrerPolicy);
  }
  if (securityHeaders.xssProtection) {
    response.headers.set("X-XSS-Protection", securityHeaders.xssProtection);
  }
  if (securityHeaders.permissionsPolicy) {
    response.headers.set("Permissions-Policy", securityHeaders.permissionsPolicy);
  }
}

// ============================================================================
// SvelteKit Integration
// ============================================================================

/**
 * Options for the SvelteKit CSP handle wrapper.
 */
export interface CspHandleOptions {
  /**
   * CSP configuration.
   */
  csp: ResolvedCspConfig;

  /**
   * Security headers configuration.
   * Defaults to sensible security headers.
   */
  securityHeaders?: SecurityHeadersConfig;
}

/**
 * Create resolve options for SvelteKit that inject the nonce.
 *
 * Use this with the `resolve` function in your handle hook.
 *
 * @param nonce - The nonce to inject
 * @param existingOptions - Any existing resolve options to merge
 * @returns Resolve options with nonce transformation
 *
 * @example
 * ```typescript
 * const nonce = generateNonce();
 * const response = await resolve(event, createCspResolveOptions(nonce, {
 *   filterSerializedResponseHeaders: (name) => name === "content-type"
 * }));
 * ```
 */
export function createCspResolveOptions(
  nonce: string,
  existingOptions: {
    transformPageChunk?: (input: { html: string; done: boolean }) => string | Promise<string>;
    filterSerializedResponseHeaders?: (name: string, value: string) => boolean;
    preload?: (input: { type: string; path: string }) => boolean;
  } = {}
): {
  transformPageChunk: (input: { html: string; done: boolean }) => string | Promise<string>;
  filterSerializedResponseHeaders?: (name: string, value: string) => boolean;
  preload?: (input: { type: string; path: string }) => boolean;
} {
  const existingTransform = existingOptions.transformPageChunk;

  return {
    ...existingOptions,
    transformPageChunk: async ({ html, done }) => {
      // Replace the SvelteKit nonce placeholder
      let transformed = html.replace(/%sveltekit\.nonce%/g, nonce);

      // Apply any existing transform
      if (existingTransform) {
        transformed = await existingTransform({ html: transformed, done });
      }

      return transformed;
    }
  };
}
