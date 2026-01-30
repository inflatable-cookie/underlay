export type AuthFieldErrors = Record<string, string>;

export type LoginPayload = {
  email: string;
  password: string;
  code?: string;
};

export type RegisterPayload = {
  email: string;
  password: string;
  passwordConfirm: string;
  /** Optional display name. Identity/personalization should live in account.user_profile. */
  displayName?: string;
};

export type RecoveryPayload = {
  email: string;
};

export type TotpSetupPayload = {
  code: string;
};

export type PassKeyStartPayload = {
  source: "button";
};

export type SessionListItem = {
  id: string;
  createdAt: string;
  lastUsedAt: string;
  ipAddress?: string | null;
  userAgent?: string | null;
  status?: string;
};
