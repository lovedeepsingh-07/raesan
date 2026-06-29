import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import checker from "vite-plugin-checker";

export default defineConfig(async () => ({
	plugins: [checker({ typescript: true }), sveltekit(), tailwindcss()],
	server: {
		port: 3000,
		host: true
	}
}));
