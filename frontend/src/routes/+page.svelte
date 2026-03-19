<script lang="ts">
	import { invoke, isTauri } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { Button } from "$lib/components/ui/button";

	type StreamEvent = {
		name: string;
		data: string;
	};

	let events: StreamEvent[] = $state([]);

	if (isTauri()) {
		listen<StreamEvent>("some_event", (event) => {
			events.push(event.payload);
		});
	}

	const handle_listen_click = () => {
		if (!isTauri()) {
			alert("listening without tauri backend");
			return;
		}
		invoke("populate_database", { input_data: "This is from Svelte" })
			.then((output) => {
				console.log("Something went right", output);
			})
			.catch((error) => {
				if (error.kind == "AlreadyRunningError") {
					alert(error.message);
				} else {
					console.error("Something went wrong", error);
				}
			});
	};
	const handle_cancel_click = () => {
		if (!isTauri()) {
			alert("canceling without tauri backend");
			return;
		}
		invoke("cancel_populate")
			.then((output) => {
				console.log("Something went right", output);
			})
			.catch((error) => {
				if (error.kind == "NotFoundError") {
					alert("cant cancel something that is not running");
				} else {
					console.error("Something went wrong", error);
				}
			});
	};
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
	<Button onclick={handle_listen_click} class="w-fit hover:cursor-pointer" variant="secondary"
		>Populate</Button
	>
	<Button onclick={handle_cancel_click} class="w-fit hover:cursor-pointer" variant="secondary"
		>Cancel</Button
	>
	{#each events as event}
		<p>{event.name} - {event.data}</p>
	{/each}
</div>
