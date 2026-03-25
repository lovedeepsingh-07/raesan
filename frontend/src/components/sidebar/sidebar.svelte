<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import MenuItem from "./menu_item.svelte";
	import { Button } from "$lib/components/ui/button";
	import { Navbar } from "$components";
	import { House, Notebook, Database } from "@lucide/svelte";
	import { isTauri } from "@tauri-apps/api/core";

	const items = [
		{
			title: "Home",
			url: "/",
			icon: House
		},
		{
			title: "Practice Questions",
			url: "/practice",
			icon: Notebook
		}
	];

	if (isTauri()) {
		items.push({
			title: "Database",
			url: "/database",
			icon: Database
		});
	}

	let { children } = $props();
</script>

{#snippet SidebarGroup()}
	<Sidebar.Group>
		<Sidebar.GroupContent>
			<Sidebar.Menu class="gap-[10px]">
				{#each items as item (item.title)}
					<MenuItem {item} />
				{/each}
			</Sidebar.Menu>
		</Sidebar.GroupContent>
	</Sidebar.Group>
{/snippet}

<Sidebar.Provider>
	<Sidebar.Root class="max-w-[60%]">
		<Sidebar.Header />
		<Sidebar.Content>{@render SidebarGroup()}</Sidebar.Content>
		<Sidebar.Footer />
	</Sidebar.Root>
	<main class="w-full">
		<Navbar />
		{@render children()}
	</main>
</Sidebar.Provider>
