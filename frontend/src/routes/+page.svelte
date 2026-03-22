<script lang="ts">
	import { begin_populate, cancel_populate } from "$lib/populate";
	import { isTauri } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { Button } from "$lib/components/ui/button";
	import { onMount } from "svelte";
	import { PUBLIC_API_URL } from "$env/static/public";

	type StreamEvent = {
		name: string;
		data: string;
	};
	let events: StreamEvent[] = $state([]);

	let is_tauri = $state(false);
	onMount(async () => {
		is_tauri = isTauri();
		if (is_tauri) {
			listen<StreamEvent>("populate_event", (event) => {
				events.push(event.payload);
			});
		}
	});

	let checking_health = $state(false);
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
	{#if is_tauri}
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
	{:else}
		<Button
			onclick={async () => {
				checking_health = true;
				try {
					const res = await fetch(`${PUBLIC_API_URL}/health`);
					const data = await res.text();
					alert(data);
				} catch (error) {
					console.error("something went wrong: ", error);
				} finally {
					checking_health = false;
				}
			}}
			class="w-fit hover:cursor-pointer"
			disabled={checking_health}
			variant="secondary">{checking_health ? "checking" : "Health"}</Button
		>
	{/if}
</div>
