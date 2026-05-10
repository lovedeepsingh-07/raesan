import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(async () => ({
	plugins: [sveltekit(), tailwindcss()],
	server: {
		port: 3000,
		host: true
	}
}));
