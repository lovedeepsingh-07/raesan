import { PUBLIC_API_URL } from "$env/static/public";
import { fetch_filter_metadata } from "@raesan/sdk";

export const load = async (server_utils) => {
	return {
		filter_metadata: fetch_filter_metadata(server_utils, PUBLIC_API_URL)
	};
};
