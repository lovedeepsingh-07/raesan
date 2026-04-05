<script lang="ts">
	import MenuItem from "./menu_item.svelte";
	import { Navbar } from "$components";
	import { House, Notebook, Database } from "@lucide/svelte";
	import { isTauri } from "@tauri-apps/api/core";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";
	import { setContext } from "svelte";
	import { fly, fade } from "svelte/transition";

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

	const is_mobile = new IsMobile();
	const sidebar = $state({
		open: !is_mobile.current,
		width_tcss: "w-[300px]",
		width_num: 300
	});
	const is_sidebar_open = () => {
		return sidebar.open;
	};
	const toggle_sidebar = () => {
		sidebar.open = !sidebar.open;
	};
	setContext("sidebar", {
		get open() {
			return is_sidebar_open();
		},
		toggle: toggle_sidebar
	});
	$effect(() => {
		sidebar.open = !is_mobile.current;
	});

	let { children } = $props();
</script>

<div class="flex h-screen overflow-hidden">
	{#if is_sidebar_open() && is_mobile.current}
		<button
			transition:fade={{ duration: 200 }}
			onclick={toggle_sidebar}
			class="fixed inset-0 z-[99] bg-black/40"
			aria-label="sidebar-backdrop"
		></button>
	{/if}
	{#if sidebar.open}
		<div
			transition:fly={{ x: -sidebar.width_num, duration: sidebar.width_num, opacity: 1 }}
			class={`z-[100] ${sidebar.width_tcss} bg-sidebar ${is_mobile.current ? "fixed top-0 bottom-0 left-0" : ""}`}
		>
			{#if is_mobile.current}
				<button onclick={toggle_sidebar}>close</button>
			{/if}
			{#each items as item (item.title)}
				<MenuItem {item} />
			{/each}
		</div>
	{/if}
	<div class="w-full overflow-y-auto">
		<Navbar />
		<main>
			{@render children()}
		</main>
	</div>
</div>
