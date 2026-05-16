import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import adapter_auto from "@sveltejs/adapter-auto";
import adapter_cloudflare from "@sveltejs/adapter-cloudflare";

const nix_build = process.env.NIX_BUILD;

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: nix_build
			? adapter_cloudflare({
					fallback: "index.html"
				})
			: adapter_auto({
					fallback: "index.html"
				}),
		alias: {
			"$components/*": "./src/components/*",
			$components: "./src/components/index.ts"
		}
	}
};

export default config;
