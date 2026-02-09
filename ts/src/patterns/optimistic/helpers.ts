export function withSetValue<T>(source: Set<T>, value: T): Set<T> {
  const next = new Set(source);
  next.add(value);
  return next;
}

export function withoutSetValue<T>(source: Set<T>, value: T): Set<T> {
  const next = new Set(source);
  next.delete(value);
  return next;
}

export function createNoopOperation() {
  return {
    confirm: () => {},
    rollback: () => {}
  };
}
