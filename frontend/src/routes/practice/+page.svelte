<script lang="ts">
	import { fetch_metadata } from "$lib";

	let fetching = $state(false);
</script>

<div class="h-[1000px]">
	<h1 class="text-2xl">Questions Page</h1>
	<button
		onclick={async () => {
			fetching = true;
			await fetch_metadata();
			fetching = false;
		}}
		disabled={fetching}>{fetching ? "fetching..." : "Fetch"}</button
	>
</div>
