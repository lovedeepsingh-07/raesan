import type { Exam, Chapter } from "$lib/models";

export const fetch_filter_metadata = async (
	svelte_fetch: typeof fetch,
	API_URL: string
): Promise<Array<Exam>> => {
	const url = `${API_URL}/api/filter_metadata`;
	const res = await svelte_fetch(url, { method: "GET" });
	if (!res.ok) {
		throw new Error(`HTTP error: ${res.status}`);
	}
	return await res.json();
};

export const fetch_chapter_data = async (
	svelte_fetch: typeof fetch,
	API_URL: string,
	chapter_id: string
): Promise<Chapter> => {
	const url = `${API_URL}/api/chapter_data/${chapter_id}`;
	const res = await svelte_fetch(url, { method: "GET" });
	if (!res.ok) {
		throw new Error(`HTTP error: ${res.status}`);
	}
	return await res.json();
};
