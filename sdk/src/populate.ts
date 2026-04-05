import { invoke } from "@tauri-apps/api/core";

export const begin_populate = async () => {
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
};

export const cancel_populate = async () => {
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
};
