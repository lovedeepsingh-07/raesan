export const load = async (load_event) => {
	const { test_id } = load_event.params;
	return {
		test_id
	};
};
