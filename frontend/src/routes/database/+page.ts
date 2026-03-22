import { error } from "@sveltejs/kit";
import { PUBLIC_APP_PLATFORM } from "$env/static/public";
import { env } from "$env/dynamic/public";

export const load = () => {
	if (env.PUBLIC_APP_PLATFORM == "web") {
		error(404, "Not found");
	}
};
