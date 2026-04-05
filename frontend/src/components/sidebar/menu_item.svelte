<script lang="ts">
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";
	import { getContext } from "svelte";

	const is_mobile = new IsMobile();
	const sidebar = getContext("sidebar");

	let { item }: { item: { title: string; url: string; icon: any } } = $props();
</script>

<a
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
</a>
