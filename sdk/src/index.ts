import { invoke, isTauri } from "@tauri-apps/api/core";

export const fetch_filter_metadata = async (server_utils?, API_URL: string) => {
	if (isTauri()) {
		const res = await invoke("fetch_filter_metadata");
		return res;
	} else {
		const res = server_utils
			? await server_utils.fetch(`${API_URL}/api/filter_metadata`, { method: "GET" })
			: await fetch(`${API_URL}/api/filter_metadata`, { method: "GET" });
		if (!res.ok) {
			throw new Error(`HTTP error: ${res.status}`);
		}
		return await res.json();
	}
};
