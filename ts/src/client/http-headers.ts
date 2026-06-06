export function hasHeader(
  headers: Record<string, string>,
  name: string,
): boolean {
  const target = name.toLowerCase();
  return Object.keys(headers).some((k) => k.toLowerCase() === target);
}

export function setHeaderIfMissing(
  headers: Record<string, string>,
  name: string,
  value: string,
): void {
  if (hasHeader(headers, name)) return;
  headers[name] = value;
}

export function headersToRecord(
  headers: Headers | undefined,
): Record<string, string> {
  const result: Record<string, string> = {};
  if (!headers || typeof headers.forEach !== "function") return result;
  headers.forEach((value, key) => {
    result[key] = value;
  });
  return result;
}
