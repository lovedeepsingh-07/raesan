import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			fallback: "index.html"
		}),
		alias: {
			"$components/*": "./src/components/*",
			$components: "./src/components/index.ts",
			"$sdk/*": "./src/sdk/*",
			$sdk: "./src/sdk/index.ts"
		}
	}
};

export default config;
