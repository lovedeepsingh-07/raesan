import { API_URL } from "$lib";
import { invoke, isTauri } from "@tauri-apps/api/core";

export const cancel_populate = async () => {
	if (!isTauri()) {
		const res = await fetch(`${API_URL}/cancel_populate`, { method: "GET" });
		const data = await res.text();
		alert(data);
	} else {
		invoke("cancel_populate")
			.then((output) => {
				console.log("Something went right", output);
			})
			.catch((error) => {
				if (error.kind == "NotFoundError") {
					alert("cant cancel something that is not running");
				} else {
					console.error("Something went wrong", error);
				}
			});
	}
};
