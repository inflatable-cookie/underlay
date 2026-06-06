export async function goto(): Promise<void> {}

export function afterNavigate(): void {}
export function beforeNavigate(): void {}
export function disableScrollHandling(): void {}
export function invalidate(): Promise<void> {
	return Promise.resolve();
}
export function invalidateAll(): Promise<void> {
	return Promise.resolve();
}
export function onNavigate(): void {}
export function preloadCode(): Promise<void> {
	return Promise.resolve();
}
export function preloadData(): Promise<void> {
	return Promise.resolve();
}
export function pushState(): void {}
export function replaceState(): void {}
