export type Uuid = string;

export interface ListResponse<T> {
  data: T[];
}

export interface PagedListResponse<T> {
  data: T[];
  total: number;
  hasMore: boolean;
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

export type RestoreBlockerKind = "conflict" | "parent_state" | "invalid_state";

export interface RestoreReference {
  kind: string;
  id: string;
  displayName?: string | null;
}

export interface RestoreFieldConflict {
  fieldName: string;
  candidateValue?: string | null;
  activeOccupant?: RestoreReference | null;
  resolutionHints: string[];
}

export interface RestoreBlocker {
  kind: RestoreBlockerKind;
  entity: RestoreReference;
  parent?: RestoreReference | null;
  parentState?: string | null;
  message?: string | null;
  fieldConflicts: RestoreFieldConflict[];
}

export interface RestoreBlockedResult {
  status: "blocked";
  scope: string;
  deleteBatchId: string;
  blockerKinds: string[];
  blockers: RestoreBlocker[];
}

export type RestoreReferenceFormatter = (input: {
  blocker: RestoreBlocker;
  reference: RestoreReference;
  role: "entity" | "parent" | "active_occupant";
  conflict?: RestoreFieldConflict | null;
}) => string | null;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isRestoreReference(value: unknown): value is RestoreReference {
  return (
    isRecord(value) &&
    typeof value.kind === "string" &&
    typeof value.id === "string" &&
    (value.displayName === undefined ||
      value.displayName === null ||
      typeof value.displayName === "string")
  );
}

function isRestoreFieldConflict(value: unknown): value is RestoreFieldConflict {
  return (
    isRecord(value) &&
    typeof value.fieldName === "string" &&
    (value.candidateValue === undefined ||
      value.candidateValue === null ||
      typeof value.candidateValue === "string") &&
    (value.activeOccupant === undefined ||
      value.activeOccupant === null ||
      isRestoreReference(value.activeOccupant)) &&
    Array.isArray(value.resolutionHints) &&
    value.resolutionHints.every((hint) => typeof hint === "string")
  );
}

function isRestoreBlocker(value: unknown): value is RestoreBlocker {
  return (
    isRecord(value) &&
    (value.kind === "conflict" ||
      value.kind === "parent_state" ||
      value.kind === "invalid_state") &&
    isRestoreReference(value.entity) &&
    (value.parent === undefined ||
      value.parent === null ||
      isRestoreReference(value.parent)) &&
    (value.parentState === undefined ||
      value.parentState === null ||
      typeof value.parentState === "string") &&
    (value.message === undefined ||
      value.message === null ||
      typeof value.message === "string") &&
    Array.isArray(value.fieldConflicts) &&
    value.fieldConflicts.every((conflict) => isRestoreFieldConflict(conflict))
  );
}

export function isRestoreBlockedResult(value: unknown): value is RestoreBlockedResult {
  return (
    isRecord(value) &&
    value.status === "blocked" &&
    typeof value.scope === "string" &&
    typeof value.deleteBatchId === "string" &&
    Array.isArray(value.blockerKinds) &&
    value.blockerKinds.every((kind) => typeof kind === "string") &&
    Array.isArray(value.blockers) &&
    value.blockers.every((blocker) => isRestoreBlocker(blocker))
  );
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
