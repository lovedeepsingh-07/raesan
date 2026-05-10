import { env } from "$env/dynamic/private";
import { json } from "@sveltejs/kit";

export const POST = async (load_event) => {
	const body = await load_event.request.json();

	const response = await fetch(`${env.API_URL}/api/create_test`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(body)
	});

	const result = await response.json();
	return json({ status: "OK", test_data: result });
};
