import type { HttpClient } from "./http";
import type { Session, User } from "./auth-types";
import type { SingleResponse } from "./envelopes";

export interface PasswordAuthParams {
  email: string;
  password: string;
}

export interface TotpAuthParams {
  code: string;
}

export interface PassKeyAuthParams {
  // WebAuthn/Passkey credential response from the browser.
  credential: unknown;
}

export interface RegisterParams {
  email: string;
  password: string;
  displayName: string;
}

export interface AuthSession {
  user: User;
  session: Session;
  accessToken: string;
  refreshToken: string;
}

export interface AuthCommands {
  register(params: RegisterParams): Promise<AuthSession>;
  loginWithPassword(params: PasswordAuthParams & Partial<TotpAuthParams>): Promise<AuthSession>;
  loginWithPasskey(params: PassKeyAuthParams): Promise<AuthSession>;
  logout(): Promise<void>;
  refresh(): Promise<AuthSession>;
  session(): Promise<AuthSession>;
}

export interface AuthRoutes {
  register: string;
  loginPassword: string;
  loginPasskey: string;
  logout: string;
  refresh: string;
  session: string;
}

export function createAuthCommands(http: HttpClient, routes: AuthRoutes): AuthCommands {
  return {
    async register(params) {
      const res = await http.post<SingleResponse<AuthSession>>(routes.register, params);
      return res.data;
    },

    async loginWithPassword(params) {
      const res = await http.post<SingleResponse<AuthSession>>(routes.loginPassword, params);
      return res.data;
    },

    async loginWithPasskey(params) {
      const res = await http.post<SingleResponse<AuthSession>>(routes.loginPasskey, params);
      return res.data;
    },

    async logout() {
      await http.post<void>(routes.logout);
    },

    async refresh() {
      const res = await http.post<SingleResponse<AuthSession>>(routes.refresh);
      return res.data;
    },

    async session() {
      const res = await http.get<SingleResponse<AuthSession>>(routes.session);
      return res.data;
    },
  };
}
