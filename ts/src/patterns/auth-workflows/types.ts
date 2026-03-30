export type AuthFieldErrors = Record<string, string>;

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
