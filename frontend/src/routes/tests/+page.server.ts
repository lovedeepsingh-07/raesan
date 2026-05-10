import { env } from "$env/dynamic/private";
import { fetch_filter_metadata } from "$lib/server";

export const load = async (load_event) => {
	return {
		filter_metadata: fetch_filter_metadata(load_event.fetch, env.API_URL)
	};
};
