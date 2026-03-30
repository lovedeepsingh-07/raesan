<script lang="ts">
	import { IsMobile } from "$lib/hooks/is-mobile.svelte";

	const is_mobile = new IsMobile();

	let { item }: { item: { title: string; url: string; icon: any } } = $props();
</script>

<a
	onclick={() => {
		if (is_mobile.current) {
			alert("something");
		}
	}}
	href={item.url}
	variant="ghost"
	class="flex w-full max-w-[200px] items-center justify-start"
>
	<item.icon />
	<span>{item.title}</span>
</a>
