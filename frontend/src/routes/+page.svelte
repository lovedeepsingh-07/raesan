<script lang="ts">
	import { begin_populate, cancel_populate } from "$lib/populate";
	import { API_URL } from "$lib";
	import { isTauri } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { Button } from "$lib/components/ui/button";

	type StreamEvent = {
		name: string;
		data: string;
	};
	let events: StreamEvent[] = $state([]);

	if (isTauri()) {
		listen<StreamEvent>("populate_event", (event) => {
			events.push(event.payload);
		});
	} else {
		const source = new EventSource(`${API_URL}/stream_populate`);
		source.addEventListener("populate_event", (event) => {
			console.log(event);
		});
	}
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
	<Button onclick={begin_populate} class="w-fit hover:cursor-pointer" variant="secondary"
		>Populate</Button
	>
	<Button onclick={cancel_populate} class="w-fit hover:cursor-pointer" variant="secondary"
		>Cancel</Button
	>
	{#each events as event}
		<p>{event.name} - {event.data}</p>
	{/each}
</div>
