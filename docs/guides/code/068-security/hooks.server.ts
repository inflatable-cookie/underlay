/**
 * Example hooks.server.ts with CSP and security headers.
 * 
 * This demonstrates the recommended security setup for a SvelteKit
 * application using Underlay.
 */

import type { Handle } from "@sveltejs/kit";
import {
  createCookieTokenStore,
  isPublicPath,
  createLoginRedirect
} from "@decodelabs/underlay/client";
import {
  createCspConfig,
  generateNonce,
  applyCspHeaders,
  createCspResolveOptions
} from "@decodelabs/underlay/server";
import { env } from "$env/dynamic/public";

// =============================================================================
// Configuration
// =============================================================================

// Paths that don't require authentication
const PUBLIC_PATHS = [
  "/login",
  "/register",
  "/forgot-password",
  "/reset-password"
];

// Cookie configuration
const ACCESS_TOKEN_COOKIE = "access_token";
const REFRESH_TOKEN_COOKIE = "refresh_token";

// Determine production mode from API URL
const isProduction = env.PUBLIC_API_BASE_URL?.startsWith("https://") ?? false;

// =============================================================================
// Content Security Policy
// =============================================================================

// Configure CSP once at module load
const cspConfig = createCspConfig({
  // Allow connections to your API
  connectSrc: [env.PUBLIC_API_BASE_URL ?? "http://127.0.0.1:3000"],
  
  // Allow video embeds from trusted sources
  frameSrc: [
    "https://www.youtube.com",
    "https://www.youtube-nocookie.com",
    "https://player.vimeo.com"
  ],
  
  // Start in report-only mode during development
  // Change to false once you've verified no violations
  reportOnly: !isProduction,
  
  // Optional: Send violation reports to an endpoint
  // reportUri: "/api/csp-report"
});

// =============================================================================
// Request Handler
// =============================================================================

export const handle: Handle = async ({ event, resolve }) => {
  // Create token store for cookie management
  const tokenStore = createCookieTokenStore(event.cookies, {
    accessTokenName: ACCESS_TOKEN_COOKIE,
    refreshTokenName: REFRESH_TOKEN_COOKIE,
    secure: isProduction,
    sameSite: "lax"
  });
  
  // Check authentication for protected routes
  if (!isPublicPath(event.url.pathname, PUBLIC_PATHS)) {
    const accessToken = tokenStore.getAccessToken();
    
    if (!accessToken) {
      // Store intended destination and redirect to login
      return createLoginRedirect(event.url, "/login");
    }
    
    // Make token available to load functions
    event.locals.accessToken = accessToken;
  }
  
  // Generate a unique nonce for this request
  const nonce = generateNonce();
  
  // Store nonce for use in templates if needed
  event.locals.cspNonce = nonce;
  
  // Resolve with nonce injection for script tags
  const response = await resolve(event, createCspResolveOptions(nonce, {
    // Allow content-type header to be serialized for universal load functions
    filterSerializedResponseHeaders: (name: string) => {
      return name === "content-type";
    }
  }));
  
  // Apply CSP and security headers
  applyCspHeaders(response, cspConfig, nonce);
  
  return response;
};
