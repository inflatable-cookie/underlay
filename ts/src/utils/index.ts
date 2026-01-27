/**
 * Utility functions for Underlay.
 */

// Base64URL encoding/decoding
export {
  base64urlToArrayBuffer,
  arrayBufferToBase64url,
} from "./base64url.js";

// WebAuthn credential helpers
export {
  toPublicKeyRequestOptions,
  toPublicKeyCreationOptions,
  assertionToJson,
  credentialCreationToJson,
} from "./webauthn.js";
