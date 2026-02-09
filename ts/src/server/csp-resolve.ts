type ResolveOptions = {
	transformPageChunk?: (input: { html: string; done: boolean }) => string | Promise<string>;
	filterSerializedResponseHeaders?: (name: string, value: string) => boolean;
	preload?: (input: { type: string; path: string }) => boolean;
};

export function createCspResolveOptions(
	nonce: string,
	existingOptions: ResolveOptions = {}
): {
	transformPageChunk: (input: { html: string; done: boolean }) => string | Promise<string>;
	filterSerializedResponseHeaders?: (name: string, value: string) => boolean;
	preload?: (input: { type: string; path: string }) => boolean;
} {
	const existingTransform = existingOptions.transformPageChunk;

	return {
		...existingOptions,
		transformPageChunk: async ({ html, done }) => {
			let transformed = html.replace(/%sveltekit\.nonce%/g, nonce);

			if (existingTransform) {
				transformed = await existingTransform({ html: transformed, done });
			}

			return transformed;
		}
	};
}
