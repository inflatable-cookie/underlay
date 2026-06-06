import { getAuthConfig } from "./auth";

export interface AuthFetchHandlerOptions {
  getToken?: () => string | null;
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;
}

export interface AuthFetchHandlers {
  getToken: () => string | null;
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;
}

export function resolveAuthFetchHandlers(
  caller: string,
  options: AuthFetchHandlerOptions,
): AuthFetchHandlers {
  const globalConfig = getAuthConfig();
  const getToken = options.getToken ?? globalConfig?.getToken;
  const onRefresh = options.onRefresh ?? globalConfig?.onRefresh;

  if (!getToken) {
    throw new Error(
      `${caller}: getToken is required. Either pass it in options or call configureAuth() at app startup.`,
    );
  }

  return { getToken, onRefresh };
}

export function shouldSkipAuthReadyFetch(
  fetched: boolean,
  authLoading: boolean,
  currentUser: unknown,
): boolean {
  return fetched || authLoading || !currentUser;
}

export function errorFromUnknown(value: unknown, fallbackMessage: string): Error {
  return value instanceof Error ? value : new Error(fallbackMessage);
}

export function isStatusError(value: unknown, status: number): boolean {
  return (
    value !== null &&
    typeof value === "object" &&
    "status" in value &&
    (value as { status: unknown }).status === status
  );
}

export interface RunAuthenticatedFetchOptions<T> extends AuthFetchHandlers {
  fetcher: (token: string) => Promise<T>;
  onSuccess: (result: T) => void;
  onError?: (error: Error) => void;
  fallbackErrorMessage?: string;
  refreshFailureMessage?: string;
  preserveRefreshFailureError?: boolean;
}

export async function runAuthenticatedFetch<T>(
  options: RunAuthenticatedFetchOptions<T>,
): Promise<boolean> {
  const token = options.getToken();
  if (!token) {
    return false;
  }

  const fallbackErrorMessage = options.fallbackErrorMessage ?? "Failed to load data";
  const refreshFailureMessage = options.refreshFailureMessage ?? "Session expired";

  const emitError = (error: Error) => {
    options.onError?.(error);
  };

  try {
    const result = await options.fetcher(token);
    options.onSuccess(result);
    return true;
  } catch (error) {
    if (!isStatusError(error, 401) || !options.onRefresh) {
      emitError(errorFromUnknown(error, fallbackErrorMessage));
      return true;
    }

    let refreshedToken: string | null = null;
    try {
      refreshedToken = await options.onRefresh(fetch);
    } catch (refreshError) {
      emitError(errorFromUnknown(refreshError, refreshFailureMessage));
      return true;
    }

    if (!refreshedToken) {
      emitError(
        options.preserveRefreshFailureError
          ? errorFromUnknown(error, refreshFailureMessage)
          : new Error(refreshFailureMessage),
      );
      return true;
    }

    try {
      const result = await options.fetcher(refreshedToken);
      options.onSuccess(result);
      return true;
    } catch (retryError) {
      emitError(errorFromUnknown(retryError, fallbackErrorMessage));
      return true;
    }
  }
}
