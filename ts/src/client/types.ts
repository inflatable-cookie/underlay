export type Uuid = string;

export interface ListResponse<T> {
  data: T[];
}

export interface SingleResponse<T> {
  data: T;
}

export interface ErrorBody {
  code: string;
  message: string;
  fieldErrors?: Record<string, string>;
}

export interface ErrorEnvelope {
  error: ErrorBody;
}

export type AuthErrorCode =
  | "auth.user_not_found"
  | "auth.email_already_exists"
  | "auth.registration_disabled"
  | "auth.account_suspended"
  | "auth.account_deleted"
  | "auth.wrong_credentials"
  | "auth.wrong_password"
  | "auth.password_weak"
  | "auth.password_compromised"
  | "auth.password_change_requires_current"
  | "auth.password_same_as_current"
  | "auth.2fa_required"
  | "auth.2fa_invalid"
  | "auth.2fa_not_set_up"
  | "auth.backup_code_invalid"
  | "auth.backup_codes_exhausted"
  | "auth.session_expired"
  | "auth.session_revoked"
  | "auth.token_invalid"
  | "auth.token_malformed"
  | "auth.token_not_yet_valid"
  | "auth.token_fingerprint_mismatch"
  | "auth.passkey_registration_failed"
  | "auth.passkey_authentication_failed"
  | "auth.passkey_credential_not_found"
  | "auth.passkey_counter_regression"
  | "auth.oauth_error"
  | "auth.oauth_already_connected"
  | "auth.oauth_not_connected"
  | "auth.oauth_token_refresh_failed"
  | "auth.oauth_access_denied"
  | "auth.rate_limited"
  | "auth.unauthorized"
  | "auth.forbidden"
  | "auth.bad_request"
  | "auth.internal"
  | (string & {});

export interface AuthError {
  code: AuthErrorCode;
  message: string;
  fieldErrors?: Record<string, string>;
}

export type UserStatus = "active" | "suspended" | "deleted";

export interface User {
  id: Uuid;
  email: string;
  displayName: string;
  status: UserStatus;
  createdAt: string;
  updatedAt: string;
}

export type CredentialType = "password" | "totp" | "passkey" | "oauth_google";

export type CredentialMetadata =
  | {
      type: "password";
      algorithm: string;
      memoryKb: number;
      iterations: number;
      parallelism: number;
    }
  | {
      type: "totp";
      issuer: string;
      algorithm: string;
      digits: number;
      period: number;
    }
  | {
      type: "passkey";
      credentialId: string;
      transports: string[];
      lastCounter: number;
    }
  | {
      type: "oauthGoogle";
      googleUserId: string;
      scopes: string[];
    };

export interface Credential {
  id: Uuid;
  userId: Uuid;
  credentialType: CredentialType;
  secretEncrypted: string;
  metadata: CredentialMetadata;
  verified: boolean;
  createdAt: string;
  updatedAt: string;
  lastUsedAt: string | null;
}

export type SessionStatus = "active" | "revoked" | "expired";

export interface Session {
  id: Uuid;
  userId: Uuid;
  accessTokenFingerprint: string;
  refreshTokenFingerprint: string;
  accessTokenExpiresAt: string;
  refreshTokenExpiresAt: string;
  createdAt: string;
  lastUsedAt: string;
  ipAddress: string | null;
  userAgent: string | null;
  status: SessionStatus;
  revocationReason: string | null;
  revokedAt: string | null;
}
