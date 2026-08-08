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
			'@inflatable-cookie/underlay': resolve(__dirname, './ts/src'),
			'@inflatable-cookie/underlay/client': resolve(__dirname, './ts/src/client'),
			'@inflatable-cookie/underlay/patterns': resolve(__dirname, './ts/src/patterns'),
			'$app/navigation': resolve(__dirname, './ts/tests/setup/sveltekit-navigation.mock.ts'),
			'$app/state': resolve(__dirname, './ts/tests/setup/sveltekit-state.mock.ts'),
			'$app/environment': resolve(__dirname, './ts/tests/setup/sveltekit-environment.mock.ts')
		}
	}
});
