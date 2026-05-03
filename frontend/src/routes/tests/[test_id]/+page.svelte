<script lang="ts">
	import { db } from "$lib/database";
	import type { RaesanTest } from "$lib/models";
	import { RaesanTestModel } from "$lib/models";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";

	let { data } = $props();

	let curr_test: RaesanTest = $state(RaesanTestModel.parse({}));
	onMount(async () => {
		try {
			curr_test = await RaesanTestModel.parse(db.test_list.get(data.test_id));
			console.log(curr_test);
		} catch {
			goto("/");
		}
	});
</script>
