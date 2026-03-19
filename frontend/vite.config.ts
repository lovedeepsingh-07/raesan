import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
	plugins: [sveltekit(), tailwindcss()],

	clearScreen: false,
	server: {
		port: 3000,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 3001
				}
			: undefined,
		watch: {
			ignored: []
		}
	}
}));
