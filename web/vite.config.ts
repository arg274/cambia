import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import Icons from 'unplugin-icons/vite'
import { FileSystemIconLoader } from 'unplugin-icons/loaders'

export default defineConfig({
	plugins: [
		sveltekit(),
		purgeCss(),
		Icons({
			compiler: 'svelte',
			customCollections: {
				'cambia': FileSystemIconLoader('static/icons'),
			}
		}),
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
