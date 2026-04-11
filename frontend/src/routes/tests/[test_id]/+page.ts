export const load = async (server_utils) => {
	const { test_id } = server_utils.params;
	return {
		test_id
	};
};
