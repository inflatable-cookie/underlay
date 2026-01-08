export class ApiError extends Error {
  public readonly code: string;
  public readonly statusCode: number;
  public readonly details?: Record<string, string>;

  constructor(envelope: ErrorEnvelope) {
    super(envelope.message);
    this.code = envelope.code;
    this.statusCode = envelope.status_code;
    this.details = envelope.details as Record<string, string> | undefined;
  }

  isAuthError(): boolean {
    return this.code.startsWith('auth.');
  }

  isNotFound(): boolean {
    return this.code === 'resource.not_found';
  }

  isValidationError(): boolean {
    return this.code.startsWith('validation.');
  }

  getRetryAfterSeconds(): number | undefined {
    if (this.code === 'auth.rate_limited') {
      return this.details?.retry_after_seconds
        ? parseInt(this.details.retry_after_seconds, 10)
        : undefined;
    }
    return undefined;
  }
}

export type ApiResult<T> = T | ApiError;
