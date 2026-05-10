import { env } from "$env/dynamic/private";
import { fetch_chapter_data } from "$lib/server";

export const load = async (load_event) => {
	const { chapter_id } = load_event.params;
	return {
		chapter_id,
		chapter_data: fetch_chapter_data(load_event.fetch, env.API_URL, chapter_id)
	};
};
