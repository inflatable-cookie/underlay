/**
 * Base64URL encoding/decoding utilities for WebAuthn.
 *
 * These functions convert between Base64URL strings (used in WebAuthn JSON)
 * and ArrayBuffers (used by the WebAuthn API).
 */

/**
 * Convert a Base64URL-encoded string to an ArrayBuffer.
 *
 * @param value - Base64URL encoded string
 * @returns ArrayBuffer containing the decoded bytes
 */
export function base64urlToArrayBuffer(value: string): ArrayBuffer {
  // Convert Base64URL to standard Base64
  const base64 = value
    .replace(/-/g, "+")
    .replace(/_/g, "/")
    .padEnd(Math.ceil(value.length / 4) * 4, "=");

  const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
  return bytes.buffer;
}

/**
 * Convert an ArrayBuffer to a Base64URL-encoded string.
 *
 * @param buffer - ArrayBuffer to encode
 * @returns Base64URL encoded string (no padding)
 */
export function arrayBufferToBase64url(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = "";
  for (const b of bytes) {
    binary += String.fromCharCode(b);
  }

  // Convert to Base64 and then to Base64URL (no padding)
  return btoa(binary)
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/g, "");
}
