import { API_URL } from "$lib";
import { invoke, isTauri } from "@tauri-apps/api/core";

export const begin_populate = async () => {
	if (!isTauri()) {
		const res = await fetch(`${API_URL}/begin_populate`, { method: "GET" });
		const data = await res.text();
		alert(data);
		return;
	} else {
		invoke("populate_database", { input_data: "This is from Svelte" })
			.then((output) => {
				console.log("Something went right", output);
			})
			.catch((error) => {
				if (error.kind == "AlreadyRunningError") {
					alert(error.message);
				} else {
					console.error("Something went wrong", error);
				}
			});
	}
};
