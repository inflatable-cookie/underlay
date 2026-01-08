import type { ErrorEnvelope, SingleResponse, ListResponse } from '@decodelabs/underlay';
import { ApiError } from './errors.js';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export interface HttpClientConfig {
  baseUrl?: string;
  defaultHeaders?: Record<string, string>;
  authTokenGetter?: () => string | null;
}

export class HttpClient {
  private baseUrl: string;
  private defaultHeaders: HeadersInit;
  private authTokenGetter?: () => string | null;

  constructor(config: HttpClientConfig = {}) {
    this.baseUrl = config.baseUrl || API_URL;
    this.defaultHeaders = {
      'Content-Type': 'application/json',
      ...config.defaultHeaders,
    };
    this.authTokenGetter = config.authTokenGetter;
  }

  setAuthTokenGetter(getter: () => string | null): void {
    this.authTokenGetter = getter;
  }

  private getAuthToken(): string | null {
    if (this.authTokenGetter) {
      return this.authTokenGetter();
    }
    return null;
  }

  private buildHeaders(headers?: HeadersInit): HeadersInit {
    const authToken = this.getAuthToken();
    const authHeader = authToken ? { Authorization: `Bearer ${authToken}` } : {};

    return {
      ...this.defaultHeaders,
      ...authHeader,
      ...headers,
    };
  }

  private async request<T>(
    method: string,
    path: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const headers = this.buildHeaders(options.headers);

    const response = await fetch(url, {
      method,
      headers,
      credentials: 'include',
      body: options.body ? JSON.stringify(options.body) : undefined,
    });

    if (!response.ok) {
      const error: ErrorEnvelope = await response.json();
      throw new ApiError(error);
    }

    const text = await response.text();
    if (!text) {
      return {} as T;
    }

    return JSON.parse(text);
  }

  get<T>(path: string): Promise<T> {
    return this.request<T>('GET', path);
  }

  post<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('POST', path, { body });
  }

  put<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PUT', path, { body });
  }

  patch<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PATCH', path, { body });
  }

  delete<T>(path: string): Promise<T> {
    return this.request<T>('DELETE', path);
  }
}

export const http = new HttpClient();
