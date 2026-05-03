import { API_URL } from "$env/static/private";
import { fetch_chapter_data } from "$lib/server";

export const load = async (load_event) => {
	const { chapter_id } = load_event.params;
	return {
		chapter_id,
		chapter_data: fetch_chapter_data(load_event.fetch, API_URL, chapter_id)
	};
};
