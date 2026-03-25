<script lang="ts">
	import { fetch_metadata } from "$lib";
	import { Button } from "$lib/components/ui/button";

	let fetching = $state(false);
</script>

<h1 class="text-2xl">Questions Page</h1>
<Button
	onclick={async () => {
		fetching = true;
		await fetch_metadata();
		fetching = false;
	}}
	disabled={fetching}>{fetching ? "fetching..." : "Fetch"}</Button
>
