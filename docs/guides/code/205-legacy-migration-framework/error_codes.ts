export function withCode(code: string, message: string): string {
  return `[${code}] ${message}`;
}

export function fail(code: string, message: string): never {
  throw new Error(withCode(code, message));
}
