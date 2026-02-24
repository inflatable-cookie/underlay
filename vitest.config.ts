import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
	test: {
		globals: true,
		environment: 'node',
		include: ['ts/tests/**/*.test.ts'],
		exclude: ['node_modules', 'dist'],
			coverage: {
				provider: 'v8',
				reporter: ['text', 'json', 'html'],
				include: ['ts/src/**/*.ts'],
				exclude: [
					'ts/src/**/*.d.ts',
					'ts/src/**/index.ts', // Barrel exports
					'ts/src/tools/**' // CLI tools
				],
				thresholds: {
					statements: 58,
					branches: 59,
					functions: 61,
					lines: 58
				}
			}
		},
	resolve: {
		alias: {
			'@decodelabs/underlay': resolve(__dirname, './ts/src'),
			'@decodelabs/underlay/client': resolve(__dirname, './ts/src/client'),
			'@decodelabs/underlay/patterns': resolve(__dirname, './ts/src/patterns'),
			'@decodelabs/underlay/components': resolve(__dirname, './ts/src/components')
		}
	}
});
