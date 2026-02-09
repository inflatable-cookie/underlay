import { storage } from "./storage";

export function readNavigationContextStack<T>(storageKey: string): T[] {
	return storage.session.get<T[]>(storageKey, []);
}

export function writeNavigationContextStack<T>(storageKey: string, stack: T[]): void {
	storage.session.set(storageKey, stack);
}

export function popNavigationContextStack<T>(storageKey: string): T | null {
	const stack = readNavigationContextStack<T>(storageKey);
	if (stack.length === 0) return null;

	const popped = stack.pop()!;
	writeNavigationContextStack(storageKey, stack);
	return popped;
}

export function peekNavigationContextStack<T>(storageKey: string): T | null {
	const stack = readNavigationContextStack<T>(storageKey);
	return stack.length > 0 ? stack[stack.length - 1] : null;
}

export function clearNavigationContextStack(storageKey: string): void {
	storage.session.remove(storageKey);
}
