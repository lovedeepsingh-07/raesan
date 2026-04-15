import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkMath from "remark-math";
import remarkRehype from "remark-rehype";
import rehypeKatex from "rehype-katex";
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

export const render_content = async (input: string) => {
	const result = await unified()
		.use(remarkParse)
		.use(remarkMath)
		.use(remarkRehype)
		.use(rehypeKatex)
		.use(rehypeStringify)
		.process(input);

	return String(result);
};
