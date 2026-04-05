<script lang="ts">
	import { begin_populate, cancel_populate } from "@raesan/sdk/populate";
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";
	import { PUBLIC_API_URL } from "$env/static/public";

	type StreamEvent = {
		name: string;
		data: string;
	};
	let events: StreamEvent[] = $state([]);

	onMount(async () => {
		listen<StreamEvent>("populate_event", (event) => {
			events.push(event.payload);
		});
	});
</script>

<div class="mt-[20px] h-[1000px]">
	<h1 class="text-2xl">Hello from Svelte!</h1>
	<button
		onclick={() => {
			alert("Shit works");
		}}
		class="w-fit hover:cursor-pointer">Test</button
	>
	<button
		onclick={async () => {
			await begin_populate();
		}}
		class="w-fit hover:cursor-pointer">Populate</button
	>
	<button
		onclick={async () => {
			await cancel_populate();
		}}
		class="w-fit hover:cursor-pointer">Cancel</button
	>
	{#each events as event}
		<p>{event.name} - {event.data}</p>
	{/each}
</div>
