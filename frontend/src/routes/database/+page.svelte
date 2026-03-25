<script lang="ts">
	import { begin_populate, cancel_populate } from "$lib/populate";
	import { listen } from "@tauri-apps/api/event";
	import { Button } from "$lib/components/ui/button";
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
	<Button
		onclick={() => {
			alert("Shit works");
		}}
		class="w-fit hover:cursor-pointer"
		variant="secondary">Test</Button
	>
	<Button
		onclick={async () => {
			await begin_populate();
		}}
		class="w-fit hover:cursor-pointer"
		variant="secondary">Populate</Button
	>
	<Button
		onclick={async () => {
			await cancel_populate();
		}}
		class="w-fit hover:cursor-pointer"
		variant="secondary">Cancel</Button
	>
	{#each events as event}
		<p>{event.name} - {event.data}</p>
	{/each}
</div>
