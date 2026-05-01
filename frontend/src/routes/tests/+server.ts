import { PUBLIC_API_URL } from "$env/static/public";
import { json } from "@sveltejs/kit";

export const POST = async (server_utils) => {
	const body = await server_utils.request.json();

	const response = await fetch(`${PUBLIC_API_URL}/api/create_test`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(body)
	});

	const result = await response.json();
	return json({ status: "OK", test_data: result });
};
