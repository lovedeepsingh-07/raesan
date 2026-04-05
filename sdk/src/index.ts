import { invoke, isTauri } from "@tauri-apps/api/core";

export const fetch_metadata = async (API_URL: string) => {
	if (isTauri()) {
		try {
			const res = await invoke("metadata");
			console.log(res);
		} catch (err) {
			console.error("Something went wrong on the native side", err);
		}
	} else {
		try {
			const res = await fetch(`${API_URL}/api/metadata`, { method: "GET" });
			const data = await res.json();
			console.log(data);
		} catch (err) {
			console.error("Something went wrong on the web side", err);
		}
	}
};
