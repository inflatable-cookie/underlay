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
