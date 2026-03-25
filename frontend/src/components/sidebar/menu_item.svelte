<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { Button } from "$lib/components/ui/button";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";

	const sidebar = Sidebar.useSidebar();
	const is_mobile = new IsMobile();

	let { item }: { item: { title: string; url: string; icon: any } } = $props();
</script>

<Sidebar.MenuItem class="w-full">
	<Button
		onclick={() => {
			if (is_mobile.current) {
				sidebar.toggle();
			}
		}}
		href={item.url}
		variant="ghost"
		class="flex w-full max-w-[200px] items-center justify-start"
	>
		<item.icon />
		<span>{item.title}</span>
	</Button>
</Sidebar.MenuItem>

<!-- <Sidebar.MenuItem> -->
<!-- 	<Sidebar.MenuButton -->
<!-- 		onclick={() => { -->
<!-- 			if (is_mobile.current) { -->
<!-- 				sidebar.toggle(); -->
<!-- 			} -->
<!-- 		}} -->
<!-- 	> -->
<!-- 		{#snippet child({ props })} -->
<!-- 			<a href={item.url} {...props} class="bg-red-500"> -->
<!-- 				<item.icon /> -->
<!-- 				<span>{item.title}</span> -->
<!-- 			</a> -->
<!-- 		{/snippet} -->
<!-- 	</Sidebar.MenuButton> -->
<!-- </Sidebar.MenuItem> -->
