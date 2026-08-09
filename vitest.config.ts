import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
	test: {
		globals: true,
		environment: 'node',
		include: ['ts/tests/**/*.test.ts'],
		exclude: ['node_modules', 'dist', 'ts/tests/**/*.component.test.ts'],
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
					statements: 65,
					branches: 68,
					functions: 67,
					lines: 65
				}
			}
		},
	resolve: {
		alias: {
			'@inflatable-cookie/underlay': resolve(import.meta.dirname, './ts/src'),
			'@inflatable-cookie/underlay/client': resolve(import.meta.dirname, './ts/src/client'),
			'@inflatable-cookie/underlay/patterns': resolve(import.meta.dirname, './ts/src/patterns'),
			'$app/navigation': resolve(import.meta.dirname, './ts/tests/setup/sveltekit-navigation.mock.ts'),
			'$app/state': resolve(import.meta.dirname, './ts/tests/setup/sveltekit-state.mock.ts'),
			'$app/environment': resolve(import.meta.dirname, './ts/tests/setup/sveltekit-environment.mock.ts')
		}
	}
});
