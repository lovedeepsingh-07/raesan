import katex from "katex";
import { unified } from "unified";
import rehypeParse from "rehype-parse";
import rehypeStringify from "rehype-stringify";
import { invoke, isTauri } from "@tauri-apps/api/core";

export const fetch_filter_metadata = async (server_utils?, API_URL: string) => {
	if (isTauri()) {
		const res = await invoke("filter_metadata");
		return res;
	} else {
		const url = `${API_URL}/api/filter_metadata`;
		const res = server_utils
			? await server_utils.fetch(url, { method: "GET" })
			: await fetch(url, { method: "GET" });
		if (!res.ok) {
			throw new Error(`HTTP error: ${res.status}`);
		}
		return await res.json();
	}
};

export const fetch_chapter_data = async (server_utils?, API_URL: string, chapter_id: string) => {
	if (isTauri()) {
		const res = await invoke("chapter_data", {
			chapter_id
		});
		return res;
	} else {
		const url = `${API_URL}/api/chapter_data/${chapter_id}`;
		const res = server_utils
			? await server_utils.fetch(url, { method: "GET" })
			: await fetch(url, { method: "GET" });
		if (!res.ok) {
			throw new Error(`HTTP error: ${res.status}`);
		}
		return await res.json();
	}
};

export const render_math = (input: string) => {
	const with_math = input
		.replace(/\$\$([\s\S]+?)\$\$/g, (_, math) => {
			// const cleaned = math.replace(/<br\s*\/?>/gi, " \\\\ ");
			return katex.renderToString(math.trim(), { displayMode: false, throwOnError: false });
		})
		.replace(/\$([^\$]+?)\$/g, (_, math) => {
			// const cleaned = math.replace(/<br\s*\/?>/gi, " \\\\ ");
			return katex.renderToString(math.trim(), { displayMode: false, throwOnError: false });
		});

	const result = unified()
		.use(rehypeParse, { fragment: true })
		.use(rehypeStringify)
		.processSync(with_math);

	const output = String(result);
	return output;
};
