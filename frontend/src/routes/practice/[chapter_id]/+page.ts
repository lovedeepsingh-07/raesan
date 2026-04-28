import { PUBLIC_API_URL } from "$env/static/public";
import { fetch_chapter_data } from "$sdk";

export const load = async (server_utils) => {
	const { chapter_id } = server_utils.params;
	return {
		chapter_id,
		chapter_data: fetch_chapter_data(server_utils, PUBLIC_API_URL, chapter_id)
	};
};
