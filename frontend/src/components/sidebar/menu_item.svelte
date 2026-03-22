<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";

	const sidebar = Sidebar.useSidebar();
	const is_mobile = new IsMobile();

	let { item }: { item: { title: string; url: string; icon: any } } = $props();
</script>

<Sidebar.MenuItem>
	<Sidebar.MenuButton
		onclick={() => {
			if (is_mobile.current) {
				sidebar.toggle();
			}
		}}
	>
		{#snippet child({ props })}
			<a href={item.url} {...props}>
				<item.icon />
				<span>{item.title}</span>
			</a>
		{/snippet}
	</Sidebar.MenuButton>
</Sidebar.MenuItem>
